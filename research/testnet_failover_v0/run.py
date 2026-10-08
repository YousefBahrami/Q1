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
import lab
from lab import (PROFILE,CHAIN,MAX_FRAME,Rejected,authentic,body,canonical,check_cert,
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
        return {str(i):self.call(i,'status')['tip'] for i in self.processes if self.processes[i].poll() is None}
    def sync(self,i,history): return self.call(i,'sync',history=history)


def rejection(h,node,req,expected):
    try: rpc(h.ports[node],req)
    except Rejected as e:
        assert str(e)==expected,(str(e),expected)
        return str(e)
    raise AssertionError('malicious message accepted')


def run(output):
    output.mkdir(parents=True,exist_ok=False)
    with tempfile.TemporaryDirectory(prefix='q1-failover-lab-') as tmp:
        h=Harness(tmp)
        try:
            for i in range(6): h.start(i)
            first=h.call(3,'run',ballot=0,payload='synthetic-job-1',crash=0)
            assert len(set(h.roots().values()))==1
            h.stop(2); h.stop(5)
            # Active producer at height two is key 4. It exits only AFTER
            # validating a majority's acceptance, BEFORE announcing a cert.
            try: h.call(4,'run',ballot=0,payload='unfinished-job-2',crash=1)
            except (OSError,Rejected): pass
            h.processes[4].wait(timeout=10)
            assert h.processes[4].returncode==73
            journal=json.loads((Path(tmp)/'0/journal.json').read_text())
            chosen=journal['accepted']['value']
            assert chosen['body']['payload']=='unfinished-job-2'
            # Deterministic trigger: failed active process, advance coordination
            # round by one; no voter membership change or expired reservation.
            h.stop(4)
            recovered=h.call(3,'run',ballot=1,payload='must-not-replace-chosen-value',crash=0)
            assert recovered['adopted']==1
            assert recovered['certificate']['value']==chosen
            assert len(recovered['certificate']['votes'])==2
            for i in (2,4,5): h.start(i)
            history=h.call(3,'status')['history']
            for i in (2,4,5): h.sync(i,history)
            assert len(set(h.roots().values()))==1
            h.call(3,'run',ballot=0,payload='synthetic-job-3',crash=0)
            before=h.roots(); h.stop(1); h.stop(2)
            try: h.call(4,'run',ballot=0,payload='synthetic-job-4',crash=0)
            except Rejected as e: assert str(e)=='PREPARE_QUORUM'
            else: raise AssertionError('one voter finalized')
            assert all(root==before['0'] for root in h.roots().values())
            h.start(1); h.start(2)
            h.call(4,'run',ballot=0,payload='synthetic-job-4',crash=0)
            history=h.call(4,'status')['history']
            # Hostile sync must not mutate a node's already verified history.
            detected={}
            variants={}
            c=history[0]
            variants['duplicate_vote']=dict(c,votes=[c['votes'][0],c['votes'][0]])
            variants['unauthorized_voter']=dict(c,votes=[c['votes'][0],signed(5,c['votes'][1]['body'])])
            variants['invalid_signature']=dict(c,votes=[c['votes'][0],dict(c['votes'][1],signature='00'*64)])
            variants['different_proposal']=dict(c,votes=[c['votes'][0],signed(1,dict(c['votes'][1]['body'],value_id='0'*64))])
            variants['malformed_certificate']={}
            codes=['DUPLICATE_VOTE','UNAUTHORIZED_VOTER','INVALID_SIGNATURE','DIFFERENT_PROPOSAL','SCHEMA']
            for (name,cert),code in zip(variants.items(),codes):
                detected[name]=rejection(h,5,h.request(5,'sync',history=[cert]),code)
            for name,height in [('stale_height',4),('future_height',6)]:
                detected[name]=rejection(h,0,h.request(3,'prepare',height=height,ballot=0),'STALE_OR_FUTURE_HEIGHT')
            req=h.request(5,'status'); rpc(h.ports[0],req)
            detected['replayed_message']=rejection(h,0,req,'REPLAYED_MESSAGE')
            with socket.create_connection(('127.0.0.1',h.ports[0]),timeout=2) as s:
                s.sendall(struct.pack('!I',MAX_FRAME+1))
                response=receive(s)
                assert response['error']=='OVERSIZED_MESSAGE'
                detected['oversized_message']=response['error']
            promises=[rpc(h.ports[i],h.request(3,'prepare',height=5,ballot=0)) for i in (0,1)]
            v=value(3,5,0,hid(history[-1]['value']),'reserved-A')
            rpc(h.ports[0],h.request(3,'accept',height=5,ballot=0,value=v,promises=promises))
            conflict=value(3,5,0,hid(history[-1]['value']),'conflicting-B')
            detected['equivocation']=rejection(h,0,h.request(3,'accept',height=5,ballot=0,value=conflict,promises=promises),'EQUIVOCATION')
            roots=h.roots(); assert len(set(roots.values()))==1
            for i in range(6): h.stop(i)
            for i in range(6): h.start(i)
            assert h.roots()==roots
            detected['equivocation_after_restart']=rejection(h,0,h.request(3,'accept',height=5,ballot=0,value=conflict,promises=promises),'EQUIVOCATION')
            report=dict(profile=PROFILE,chain=CHAIN,processes=6,finalized_height=4,
                        final_research_log_roots=roots,history=history,
                        chosen_before_crash=hid(chosen),adopted_after_crash=hid(recovered['certificate']['value']),
                        failover_votes=len(recovered['certificate']['votes']),
                        producer_crash_exit=73,one_voter_and_ordinary_offline=True,
                        single_voter_cannot_finalize=True,restart_catchup=True,all_restart_from_disk=True,
                        detection=detected,
                        not_claimed=['Q1 ledger integration','Byzantine voter tolerance','public deployment','independent host failures','timer-based production failover'],
                        source_sha256={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(Path(__file__).parent.glob('*.py'))})
            (output/'acceptance.json').write_bytes(canonical(report))
            (output/'SHA256SUMS').write_text(hashlib.sha256(canonical(report)).hexdigest()+'  acceptance.json\n')
            print(json.dumps({k:v for k,v in report.items() if k not in ('history','source_sha256')},indent=2))
        finally: h.close()

if __name__=='__main__':
    parser=argparse.ArgumentParser(); parser.add_argument('--output',type=Path); parser.add_argument('--replay',type=Path)
    args=parser.parse_args()
    if args.replay:
        result=json.loads(args.replay.read_text()); lab.CHAIN=result['chain']
        parent='0'*64
        for height,cert in enumerate(result['history'],1):
            check_cert(cert,height,parent); parent=hid(cert['value'])
        assert set(result['final_research_log_roots'].values())=={parent}
        print('Authenticated research-log replay passed; not a Q1 StateRoot.')
    else:
        if args.output is None: parser.error('--output or --replay required')
        run(args.output)
