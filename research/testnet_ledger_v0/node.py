"""TESTNET_FAILOVER_V0 coordination with actual Rust Q1 ledger execution.

TEST ONLY — NEVER USE FOR VALUE-BEARING NETWORKS. Fixture signing seeds are public.
Six finite loopback workers exercise coordination independently of LOCALNET.
"""
from __future__ import annotations
import copy
import fcntl
import hashlib
import json
import os
from pathlib import Path
import socket
import struct
import subprocess

PROFILE = 'TESTNET_FAILOVER_V0'
CHAIN = os.environ.get('Q1_TESTNET_LAB_CHAIN', hashlib.sha256(PROFILE.encode()).hexdigest())
VOTERS = (0, 1, 2)
PRODUCERS = (3, 4)
MAX_FRAME = 65536
ROOT = Path(__file__).resolve().parents[2]
TARGET = Path(os.environ.get('CARGO_TARGET_DIR', ROOT/'target'))
CRYPTO = TARGET/'debug/q1-testnet'

class Rejected(ValueError): pass

def need(ok, reason):
    if not ok: raise Rejected(reason)

def canonical(obj):
    return json.dumps(obj,sort_keys=True,separators=(',', ':'),ensure_ascii=True,allow_nan=False).encode()

def hid(obj): return hashlib.sha256(canonical(obj)).hexdigest()

def fields(obj,names): need(type(obj) is dict and set(obj)==set(names.split()),'SCHEMA')

def integer(x,low=0,high=2**32-1):
    need(type(x) is int and low<=x<=high,'INTEGER')
    return x

def decode(raw):
    need(len(raw)<=MAX_FRAME,'OVERSIZED_MESSAGE')
    def unique(items):
        d={}
        for k,v in items:
            need(k not in d,'DUPLICATE_FIELD'); d[k]=v
        return d
    try:
        obj=json.loads(raw,object_pairs_hook=unique)
        def bounded(v,depth=0):
            need(depth<=16,'DEPTH')
            if type(v) in (dict,list):
                need(len(v)<=128,'COLLECTION')
                for x in (v.values() if type(v) is dict else v): bounded(x,depth+1)
            else: need(type(v) in (int,str,type(None)),'TYPE')
        bounded(obj)
        need(canonical(obj)==raw,'NONCANONICAL')
        return obj
    except (UnicodeError,json.JSONDecodeError,RecursionError,OverflowError):
        raise Rejected('ENCODING') from None

def backend(op, **kwargs):
    req=dict(op=op,chain=CHAIN,**kwargs)
    result=subprocess.run([str(CRYPTO)],input=canonical(req),capture_output=True)
    need(result.returncode==0,'Q1_LEDGER: '+result.stderr.decode().strip())
    return json.loads(result.stdout)

def signed(who,body):
    return backend('sign',who=who,body=body)

def authentic(envelope):
    return backend('verify',envelope=envelope)

def body(kind,**kwargs): return dict(profile=PROFILE,chain=CHAIN,kind=kind,**kwargs)

def leader(height,ballot): return PRODUCERS[(height-1+ballot)%2]

def value(origin,height,ballot,parent,payload):
    need(origin==leader(height,ballot),'PRODUCER')
    return signed(origin,body('value',height=height,origin_ballot=ballot,parent=parent,payload=payload))

def check_value(v,height,parent):
    b=authentic(v)
    fields(b,'profile chain kind height origin_ballot parent payload')
    need(b['kind']=='value' and b['height']==height and b['parent']==parent,'VALUE_CONTEXT')
    integer(b['origin_ballot']); integer(b['height'],1)
    need(v['signer']==leader(height,b['origin_ballot']),'PRODUCER')
    need(type(b['payload']) is str and b['payload'].isascii() and len(b['payload'])<=4096,'PAYLOAD')

def check_votes(votes,v,height,ballot):
    need(type(votes) is list and 2<=len(votes)<=3,'MALFORMED_CERTIFICATE')
    seen=set()
    for vote in votes:
        fields(vote,'signer body signature')
        need(vote.get('signer') in VOTERS,'UNAUTHORIZED_VOTER')
        need(vote['signer'] not in seen,'DUPLICATE_VOTE'); seen.add(vote['signer'])
        b=authentic(vote)
        fields(b,'profile chain kind height ballot value_id')
        integer(b['height'],1); integer(b['ballot'])
        need(b['kind']=='vote' and b['height']==height and b['ballot']==ballot,'VOTE_CONTEXT')
        need(b['value_id']==hid(v),'DIFFERENT_PROPOSAL')

def check_cert(cert,height,parent):
    fields(cert,'value height ballot votes')
    integer(cert['height'],1); integer(cert['ballot'])
    need(cert['height']==height,'HEIGHT')
    check_value(cert['value'],height,parent)
    need(cert['value']['body']['origin_ballot']<=cert['ballot'],'ORIGIN_BALLOT')
    check_votes(cert['votes'],cert['value'],height,cert['ballot'])

def chosen_from(promises,height,ballot,parent):
    need(type(promises) is list and 2<=len(promises)<=3,'PREPARE_QUORUM')
    seen=set(); accepted=[]
    for p in promises:
        fields(p,'signer body signature')
        need(p.get('signer') in VOTERS and p['signer'] not in seen,'PREPARE_VOTER')
        seen.add(p['signer']); b=authentic(p)
        fields(b,'profile chain kind height ballot accepted')
        need(b['kind']=='promise' and b['height']==height and b['ballot']==ballot,'PREPARE_CONTEXT')
        if b['accepted'] is not None:
            a=b['accepted']; fields(a,'ballot value')
            integer(a['ballot']); need(a['ballot']<=ballot,'ACCEPTED_BALLOT')
            check_value(a['value'],height,parent); accepted.append(a)
    if not accepted: return None
    latest=max(a['ballot'] for a in accepted)
    candidates=[a['value'] for a in accepted if a['ballot']==latest]
    need(all(v==candidates[0] for v in candidates),'EQUIVOCATION')
    return candidates[0]

def recv_exact(sock,n):
    chunks=[]
    while n:
        b=sock.recv(n)
        need(bool(b),'TRUNCATED'); chunks.append(b); n-=len(b)
    return b''.join(chunks)

def receive(sock):
    n=struct.unpack('!I',recv_exact(sock,4))[0]
    need(n<=MAX_FRAME,'OVERSIZED_MESSAGE')
    return decode(recv_exact(sock,n))

def send(sock,obj):
    raw=canonical(obj); need(len(raw)<=MAX_FRAME,'OVERSIZED_MESSAGE')
    sock.sendall(struct.pack('!I',len(raw))+raw)

def rpc(port,request):
    with socket.create_connection(('127.0.0.1',port),timeout=2) as sock:
        sock.settimeout(10); send(sock,request); result=receive(sock)
    need('error' not in result,result.get('error','REMOTE'))
    return result

class Worker:
    storage_limit = MAX_FRAME
    def decode_state(self, raw): return decode(raw)
    def encode_state(self): return canonical(self.state)
    def commit_response(self, request, response): self.persist()
    def __init__(self,directory,who,ports):
        self.directory=Path(directory); self.directory.mkdir(parents=True,exist_ok=True)
        self.lock=(self.directory/'lock').open('a+b')
        try: fcntl.flock(self.lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
        except BaseException:
            self.lock.close(); raise
        self.path=self.directory/'journal.json'; self.who=who; self.ports=ports; self.poisoned=False
        try: self.load()
        except BaseException:
            self.lock.close(); raise
    def load(self):
        who=self.who
        if self.path.exists():
            with self.path.open('rb') as f: self.state=self.decode_state(f.read(self.storage_limit+1))
            fields(self.state,'profile who history promised accepted seen outseq state_root')
            need(self.state['profile']==PROFILE and self.state['who']==who,'STORE_PROFILE')
            parent='0'*64
            for h,c in enumerate(self.state['history'],1):
                check_cert(c,h,parent); parent=hid(c['value'])
            integer(self.state['outseq']); integer(self.state['promised'],-1)
            need(type(self.state['seen']) is dict,'STORE_SEEN')
            for seq in self.state['seen'].values(): integer(seq,1)
            state=backend('status',history=self.state['history'])
            need(state['state_root']==self.state['state_root'],'PERSISTED_STATE_ROOT')
            if self.state['accepted'] is not None:
                a=self.state['accepted']; fields(a,'ballot value')
                integer(a['ballot']); need(a['ballot']<=self.state['promised'],'STORE_BALLOT')
                check_value(a['value'],self.height,self.tip)
                backend('validate',history=self.state['history'],value=a['value'])
        else:
            self.state=dict(profile=PROFILE,who=who,history=[],promised=-1,accepted=None,seen={},outseq=0,state_root=backend('status',history=[])['state_root'])
            self.persist()
    @property
    def height(self): return len(self.state['history'])+1
    @property
    def tip(self): return hid(self.state['history'][-1]['value']) if self.state['history'] else '0'*64
    def close(self): self.lock.close()
    def persist(self):
        need(not self.poisoned,'POISONED')
        raw=self.encode_state(); need(len(raw)<=self.storage_limit,'ARCHIVE_LIMIT')
        pending=self.directory/'pending.json'
        try:
            with pending.open('wb') as f: f.write(raw); f.flush(); os.fsync(f.fileno())
            os.replace(pending,self.path)
            fd=os.open(self.directory,os.O_RDONLY)
            try: os.fsync(fd)
            finally: os.close(fd)
        except OSError:
            self.poisoned=True
            raise
    def request(self,target,kind,**kwargs):
        self.state['outseq']+=1; self.persist()
        req=signed(self.who,body(kind,seq=self.state['outseq'],**kwargs))
        return rpc(self.ports[target],req)
    def apply(self,request):
        need(not self.poisoned,'POISONED')
        b=authentic(request); who=request['signer']; kind=b['kind']
        integer(b.get('seq'),1)
        need(b['seq']>self.state['seen'].get(str(who),0),'REPLAYED_MESSAGE')
        previous=copy.deepcopy(self.state)
        try:
            if kind=='status':
                fields(b,'profile chain kind seq')
                answer=body('status',height=self.height,tip=self.tip,history=self.state['history'],ledger=backend('status',history=self.state['history']))
            elif kind=='run':
                fields(b,'profile chain kind seq ballot payload crash')
                need(self.who in PRODUCERS and who==5,'CONTROL')
                answer=self.coordinate(b['ballot'],b['payload'],b['crash'])
            elif kind in ('prepare','accept'):
                need(self.who in VOTERS,'NOT_VOTER')
                integer(b['height'],1); integer(b['ballot'])
                need(b['height']==self.height,'STALE_OR_FUTURE_HEIGHT')
                need(who==leader(b['height'],b['ballot']),'UNAUTHORIZED_PRODUCER')
                need(b['ballot']>=self.state['promised'],'STALE_BALLOT')
                if kind=='prepare':
                    fields(b,'profile chain kind seq height ballot')
                    self.state['promised']=b['ballot']
                    answer=body('promise',height=self.height,ballot=b['ballot'],accepted=self.state['accepted'])
                else:
                    fields(b,'profile chain kind seq height ballot value promises')
                    check_value(b['value'],self.height,self.tip)
                    backend('validate',history=self.state['history'],value=b['value'])
                    prior=chosen_from(b['promises'],self.height,b['ballot'],self.tip)
                    need(prior is None or prior==b['value'],'ADOPTION_REQUIRED')
                    if prior is None:
                        need(b['value']['body']['origin_ballot']==b['ballot'],'FRESH_VALUE_BALLOT')
                    a=self.state['accepted']
                    if a and a['ballot']==b['ballot']:
                        need(a['value']==b['value'],'EQUIVOCATION')
                    self.state['promised']=b['ballot']
                    self.state['accepted']=dict(ballot=b['ballot'],value=b['value'])
                    answer=body('vote',height=self.height,ballot=b['ballot'],value_id=hid(b['value']))
            elif kind=='sync':
                fields(b,'profile chain kind seq history')
                need(type(b['history']) is list and len(b['history'])<=16,'SYNC_LIMIT')
                backend('status',history=b['history'])
                for h,c in enumerate(b['history'],1):
                    parent='0'*64 if h==1 else hid(b['history'][h-2]['value'])
                    check_cert(c,h,parent)
                    if h<self.height:
                        need(c['value']==self.state['history'][h-1]['value'],'CONFLICTING_FINALITY')
                    else:
                        need(h==self.height,'SYNC_GAP')
                        self.state['history'].append(c)
                        self.state['promised']=-1; self.state['accepted']=None
                self.state['state_root']=backend('status',history=self.state['history'])['state_root']
                answer=body('synced',height=self.height,tip=self.tip,state_root=self.state['state_root'])
            else: raise Rejected('OPERATION')
            self.state['seen'][str(who)]=b['seq']
            response=signed(self.who,answer)
            self.commit_response(request,response)  # durable before delivering any reply
            return response
        except Exception:
            if not self.poisoned and kind!='run': self.state=previous
            raise
    def coordinate(self,ballot,payload,crash):
        integer(ballot); integer(crash,0,3)
        need(self.who==leader(self.height,ballot),'WRONG_LEADER')
        if crash==1: os._exit(71)
        promises=[]
        for voter in VOTERS:
            try: promises.append(self.request(voter,'prepare',height=self.height,ballot=ballot))
            except (OSError,Rejected): pass
        prior=chosen_from(promises,self.height,ballot,self.tip)
        v=prior or backend('propose',history=self.state['history'],who=self.who,ballot=ballot,amount=payload)
        backend('validate',history=self.state['history'],value=v)
        if crash==2: os._exit(72)
        votes=[]
        for voter in VOTERS:
            try: votes.append(self.request(voter,'accept',height=self.height,ballot=ballot,value=v,promises=promises))
            except (OSError,Rejected): pass
        check_votes(votes,v,self.height,ballot)
        if crash==3: os._exit(73)  # chosen but certificate never announced
        cert=dict(value=v,height=self.height,ballot=ballot,votes=votes)
        check_cert(cert,self.height,self.tip)
        next_state=backend('status',history=self.state['history']+[cert])
        self.state['state_root']=next_state['state_root']
        self.state['history'].append(cert); self.state['accepted']=None; self.state['promised']=-1
        self.persist()
        for peer in range(6):
            if peer!=self.who:
                try: self.request(peer,'sync',history=self.state['history'])
                except (OSError,Rejected): pass
        return body('result',height=self.height,tip=self.tip,adopted=int(prior is not None),certificate=cert,ledger=next_state)


def server(directory,who,ports):
    worker=Worker(directory,who,ports)
    with socket.socket() as server:
        server.setsockopt(socket.SOL_SOCKET,socket.SO_REUSEADDR,1)
        server.bind(('127.0.0.1',ports[who])); server.listen(8)
        while True:
            connection,_=server.accept()
            with connection:
                connection.settimeout(2)
                try: send(connection,worker.apply(receive(connection)))
                except (Rejected,KeyError,TypeError,ValueError,AttributeError,IndexError) as e:
                    try: send(connection,dict(error=str(e) if isinstance(e,Rejected) else 'SCHEMA'))
                    except OSError: pass
                except (OSError,subprocess.SubprocessError):
                    if worker.poisoned: raise

if __name__=='__main__':
    import sys
    server(sys.argv[1],int(sys.argv[2]),[int(x) for x in sys.argv[3].split(',')])
