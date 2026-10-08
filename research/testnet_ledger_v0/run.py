"""Six-process loopback TESTNET-only crash experiment; finite and self-cleaning."""
import argparse
import copy
import hashlib
import json
import os
from pathlib import Path
import secrets
import socket
import struct
import subprocess
import sys
import tempfile
import time
os.environ.setdefault('Q1_TESTNET_LAB_CHAIN',secrets.token_hex(32))
import node as lab
from node import (PROFILE,CHAIN,MAX_FRAME,Rejected,authentic,body,canonical,check_cert,
                 hid,rpc,signed,value,receive)

class Harness:
    def __init__(self,root):
        self.root=Path(root); self.processes={}; self.logs={}; self.seq=0
        sockets=[]
        for _ in range(6):
            s=socket.socket(); s.bind(('127.0.0.1',0)); sockets.append(s)
        self.ports=[s.getsockname()[1] for s in sockets]
        for s in sockets: s.close()
    def request(self,who,kind,**kw):
        self.seq+=1
        return signed(who,body(kind,seq=10000+self.seq,**kw))
    def call(self,node,kind,**kw):
        response=rpc(self.ports[node],self.request(5,kind,**kw))
        need=response['signer']==node
        if not need: raise AssertionError('response signer')
        return authentic(response)
    def start(self,i):
        self.logs[i]=(self.root/f'{i}.log').open('ab')
        self.processes[i]=subprocess.Popen([sys.executable,str(Path(lab.__file__)),str(self.root/str(i)),str(i),','.join(map(str,self.ports))],stdout=self.logs[i],stderr=self.logs[i])
        deadline=time.monotonic()+10
        while time.monotonic()<deadline:
            if self.processes[i].poll() is not None: raise RuntimeError(f'worker {i} exited')
            try: self.call(i,'status'); return
            except OSError: time.sleep(.02)
        raise RuntimeError('startup timeout')
    def stop(self,i):
        p=self.processes.pop(i,None)
        if p:
            if p.poll() is None: p.kill()
            p.wait(timeout=5)
        if i in self.logs: self.logs.pop(i).close()
    def close(self):
        for i in list(self.processes): self.stop(i)
    def roots(self):
        return {str(i):self.call(i,'status')['ledger']['state_root'] for i in self.processes if self.processes[i].poll() is None}
    def sync(self,i,history): return self.call(i,'sync',history=history)


def rejection(h,node,req,expected):
    try: rpc(h.ports[node],req)
    except Rejected as e:
        assert str(e)==expected,(str(e),expected)
        return str(e)
    raise AssertionError('malicious message accepted')


def run(output):
    output.mkdir(parents=True,exist_ok=False)
    with tempfile.TemporaryDirectory(prefix='q1-ledger-failover-') as tmp:
        h=Harness(tmp); evidence={}
        def crash(who,ballot,phase):
            try: h.call(who,'run',ballot=ballot,payload=10,crash=phase)
            except (OSError,Rejected): pass
            h.processes[who].wait(timeout=10)
            assert h.processes[who].returncode==70+phase
            h.stop(who)
        def advance(who,ballot,amount=10):
            r=h.call(who,'run',ballot=ballot,payload=amount,crash=0)
            assert len(set(h.roots().values()))==1
            return r
        try:
            for i in range(6): h.start(i)
            # Deterministic controller rule: observed exit => next ballot,
            # next eligible producer. This is not a distributed failure detector.
            crash(3,0,1)
            r=advance(4,1); evidence['before_proposal']=r['ledger']
            h.start(3); h.sync(3,h.call(4,'status')['history'])
            crash(4,0,2)
            r=advance(3,1); evidence['after_proposal_before_votes']=r['ledger']
            h.start(4); h.sync(4,h.call(3,'status')['history'])
            h.stop(2); h.stop(5)
            crash(3,0,3)
            chosen=json.loads((Path(tmp)/'0/journal.json').read_text())['accepted']['value']
            r=advance(4,1,99)
            assert r['certificate']['value']==chosen
            assert len(r['certificate']['votes'])==2
            evidence['majority_before_publication']={'adopted':hid(chosen),'ledger':r['ledger'],'votes':2}
            evidence['voter_and_producer_crash']=True
            history=h.call(4,'status')['history']
            h.stop(4); h.start(4)
            assert h.call(4,'status')['ledger']==r['ledger']
            evidence['replacement_restart']=True
            for i in (2,3,5): h.start(i); h.sync(i,history)
            evidence['stale_producer_return']=rejection(h,0,h.request(3,'prepare',height=3,ballot=0),'STALE_OR_FUTURE_HEIGHT')
            # Make and reserve two separately valid signed transfers for height4.
            # A conflicting one must never earn a second reservation in ballot0.
            promises=[rpc(h.ports[i],h.request(4,'prepare',height=4,ballot=0)) for i in (0,1)]
            a=lab.backend('propose',history=history,who=4,ballot=0,amount=10)
            b=lab.backend('propose',history=history,who=4,ballot=0,amount=99)
            rpc(h.ports[0],h.request(4,'accept',height=4,ballot=0,value=a,promises=promises))
            h.stop(0); h.start(0)
            evidence['conflict_after_voter_restart']=rejection(h,0,h.request(4,'accept',height=4,ballot=0,value=b,promises=promises),'EQUIVOCATION')
            # Finish via the other producer; forged conflicting fixture votes
            # below exercise learner defense, not Byzantine quorum safety.
            h.stop(4)
            r=advance(3,1,99); assert r['certificate']['value']==a
            history=h.call(3,'status')['history']
            h.start(4); h.sync(4,history)
            req=h.request(5,'status'); rpc(h.ports[0],req)
            h.stop(0); h.start(0)
            evidence['network_replay_after_failover']=rejection(h,0,req,'REPLAYED_MESSAGE')
            # Even an authenticated conflicting finalized history is rejected.
            bad=copy.deepcopy(history)
            bad[-1]=dict(height=4,ballot=0,value=b,votes=[signed(i,body('vote',height=4,ballot=0,value_id=hid(b))) for i in (0,1)])
            evidence['conflicting_finality']=rejection(h,5,h.request(5,'sync',history=bad),'CONFLICTING_FINALITY')
            roots=h.roots(); assert len(roots)==6 and len(set(roots.values()))==1
            ledger=h.call(3,'status')['ledger']
            assert (ledger['height'],ledger['sender'],ledger['recipient'],ledger['nonce'],ledger['reward_pool'],ledger['total_supply'])==(4,'956','40',4,'4','1000')
            for i in range(6): h.stop(i)
            for i in range(6): h.start(i)
            assert h.roots()==roots
            report=dict(profile=PROFILE,chain=CHAIN,processes=6,finalized_height=4,
                final_state_roots=roots,ledger=ledger,history=history,tests=evidence,
                all_restart_from_disk=True,
                assumptions=['honest authorized voters','two reachable voters','fixed fixture membership','controller observes process exits and increments ballot','loopback, not multi-host'],
                not_claimed=['Byzantine voter tolerance','permissionless membership','production failure detector','Mainnet','resource-based rewards'],
                source_sha256={str(p.relative_to(lab.ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in [*sorted(Path(__file__).parent.glob('*.py')),lab.ROOT/'crates/q1-testnet/src/main.rs',lab.ROOT/'crates/q1-localnet/src/state.rs',lab.ROOT/'crates/q1-localnet/src/ledger.rs']})
            (output/'acceptance.json').write_bytes(canonical(report))
            (output/'SHA256SUMS').write_text(hashlib.sha256(canonical(report)).hexdigest()+'  acceptance.json\n')
            print(json.dumps({k:v for k,v in report.items() if k not in ('history','source_sha256')},indent=2))
        except BaseException:
            for log in Path(tmp).glob('*.log'):
                print(log.name,log.read_text()[-1500:],file=sys.stderr)
            raise
        finally: h.close()

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--output',type=Path);p.add_argument('--replay',type=Path);a=p.parse_args()
    if a.replay:
        r=json.loads(a.replay.read_text());lab.CHAIN=r['chain']
        state=lab.backend('status',history=r['history'])
        assert state==r['ledger'] and set(r['final_state_roots'].values())=={state['state_root']}
        print('Actual Q1 ledger certificate replay and StateRoot verified.')
    elif a.output: run(a.output)
    else: p.error('--output or --replay required')
