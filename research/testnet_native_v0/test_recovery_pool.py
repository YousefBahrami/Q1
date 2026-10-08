"""Deterministic journal barriers and real TLS recovery/session fault tests."""
from concurrent.futures import ThreadPoolExecutor
import json
from pathlib import Path
import socket
import ssl
import struct
import subprocess
import tempfile
import time
import unittest
from lab import Harness,NATIVE,backend,canonical,write
import test_transport as transport
from test_transport import encode,decode


class RecoveryPoolTests(unittest.TestCase):
    def setUp(self):
        self.tmp=tempfile.TemporaryDirectory(prefix='qnp-',dir='/tmp');self.h=Harness(Path(self.tmp.name))
        for i in range(6):
            (self.h.root/str(i)/'barriers').mkdir(mode=0o700);self.h.start(i)
        self.c=json.loads(self.h.config(0).read_bytes())
    def tearDown(self):
        # Release test-only barriers even on assertion failure, before draining.
        for p in self.h.root.glob('*/barriers/*.reached'):p.with_suffix('.release').touch()
        self.h.close();self.tmp.cleanup()
    send=transport.NativeTests.send
    receive=transport.NativeTests.receive
    hello=transport.NativeTests.hello
    def ready(self,target=0,who=5):
        ctx=ssl.create_default_context(ssl.Purpose.SERVER_AUTH,cafile=str(self.h.root/'ca.crt'))
        ctx.minimum_version=ctx.maximum_version=ssl.TLSVersion.TLSv1_3
        ctx.set_alpn_protocols(['q1-testnet-native-v0']);ctx.load_cert_chain(str(self.h.root/f'q1-{who}.crt'),str(self.h.root/f'q1-{who}.pem'))
        s=ctx.wrap_socket(socket.socket(),server_hostname=f'q1-{target}.test');s.settimeout(8)
        port=int(self.c['peers'][target]['address'].split(':')[1])
        try:
            s.connect(('127.0.0.1',port));self.send(s,self.hello(who));self.assertEqual(self.receive(s),self.hello(target))
        except BaseException:s.close();raise
        return s
    def pending(self,who,target,request):
        p=self.h.root/str(who)/'transport'/f'send-{target}.json'
        seq=json.loads(p.read_bytes())['seq']+1 if p.exists() else 1
        write(p,canonical(dict(chain=self.c['chain'],genesis=self.c['genesis'],seq=seq,pending=request)))
        op={'status':0,'prepare':1,'accept':2,'sync':3,'run':4}[request['body']['kind']]
        return [1,1,*self.hello()[3:5],bytes.fromhex(self.c['peers'][who]['id']),bytes.fromhex(self.c['peers'][target]['id']),seq,op,canonical(request)]
    def cli(self,who,target,request=None,recover=False):
        return subprocess.run([str(NATIVE),'recover' if recover else 'request',str(self.h.config(who)),str(target)],input=b'' if recover else canonical(request),capture_output=True,timeout=40)
    def success(self,p):
        self.assertEqual(p.returncode,0,p.stderr.decode());return json.loads(p.stdout)
    def arm(self,target,stage):
        p=self.h.root/str(target)/'barriers'/stage;p.with_suffix('.arm').touch();return p
    def reached(self,p):
        end=time.monotonic()+10
        while not p.with_suffix('.reached').exists():
            self.assertLess(time.monotonic(),end,'explicit barrier not reached');time.sleep(.005)
    def release(self,p):p.with_suffix('.release').touch()
    def ledger(self,who=0):
        state=json.loads((self.h.root/str(who)/'ledger/journal.json').read_bytes())['state']
        return backend(self.h.root,who,'status',history=state['history'])
    def test_disconnect_before_remote_receive(self):
        req=self.h.signed(5,'status');f=self.pending(5,0,req)
        with self.ready() as s:
            raw=encode(f);s.sendall(struct.pack('!I',len(raw))+raw[:2])
        result=self.success(self.cli(5,0,recover=True));self.assertEqual(result['body']['kind'],'status')
        self.assertEqual(self.ledger()['height'],0)
    def test_receive_before_dispatch_stops_observably(self):
        req=self.h.signed(5,'status');f=self.pending(5,0,req);barrier=self.arm(0,'before_status_dispatch')
        with self.ready() as s:self.send(s,f);self.reached(barrier)
        self.h.stop(0,crash=True);self.h.start(0)
        p=self.cli(5,0,recover=True);self.assertNotEqual(p.returncode,0)
        self.assertIn(b'STOP_RECONCILIATION_REQUIRED',p.stderr)
        stored=json.loads((self.h.root/'5/transport/send-0.json').read_bytes())
        self.assertEqual(stored['pending'],req);self.assertEqual(self.ledger()['reward_pool'],'0')
    def test_after_response_commit_sender_and_receiver_restart(self):
        req=self.h.signed(5,'status');f=self.pending(5,0,req);barrier=self.arm(0,'after_status_commit')
        with self.ready() as s:self.send(s,f);self.reached(barrier)
        self.h.stop(0,crash=True);self.h.stop(5,crash=True)
        self.h.start(0);self.h.start(5)
        result=self.success(self.cli(5,0,recover=True));self.assertEqual(result['body']['kind'],'status')
        self.assertIsNone(json.loads((self.h.root/'5/transport/send-0.json').read_bytes())['pending'])
    def test_after_vote_retains_same_signed_vote(self):
        promises=[self.success(self.cli(3,i,self.h.signed(3,'prepare',height=1,ballot=0))) for i in (0,1)]
        value=backend(self.h.root,3,'propose',history=[],who=3,ballot=0,amount=10)
        req=self.h.signed(3,'accept',height=1,ballot=0,value=value,promises=promises)
        f=self.pending(3,0,req);barrier=self.arm(0,'after_accept_commit')
        with self.ready(who=3) as s:self.send(s,f);self.reached(barrier)
        self.h.stop(0,crash=True);self.h.start(0)
        result=self.success(self.cli(3,0,recover=True));self.assertEqual(result['body']['kind'],'vote')
        f[1]=3
        with self.ready(who=3) as s:self.send(s,f);r=self.receive(s)
        self.assertEqual(r[7],0);self.assertEqual(json.loads(r[8]),result)
        self.assertEqual(self.ledger()['reward_pool'],'0')
    def test_after_finalization_no_second_fee(self):
        req=self.h.signed(5,'run',ballot=0,payload=10,crash=0)
        f=self.pending(5,3,req);barrier=self.arm(3,'after_run_commit')
        with self.ready(3) as s:self.send(s,f);self.reached(barrier)
        self.h.stop(3,crash=True);self.h.start(3)
        result=self.success(self.cli(5,3,recover=True));self.assertEqual(result['body']['ledger']['height'],1)
        self.assertEqual(self.success(self.cli(5,3,recover=True)),result)
        f[1]=3
        with self.ready(3) as s:self.send(s,f);r=self.receive(s)
        self.assertEqual(json.loads(r[8]),result)
        roots=self.h.roots();self.assertEqual(len(set(roots.values())),1)
        self.assertEqual((self.ledger()['sender'],self.ledger()['reward_pool'],self.ledger()['total_supply']),('989','1','1000'))
    def test_pool_reuse_reconnect_simultaneous_directions(self):
        for _ in range(3):self.h.call(0,'status')
        log=(self.h.root/'5/process.log').read_text()
        self.assertEqual(sum('session_open' in l and 'peer 0' in l for l in log.splitlines()),1)
        requests=[(5,0,self.h.signed(5,'status')),(0,5,self.h.signed(0,'status'))]
        with ThreadPoolExecutor(2) as pool:
            results=list(pool.map(lambda x:self.cli(*x),requests))
        for r in results:self.success(r)
        self.h.stop(0,crash=True);self.h.start(0)
        self.h.call(0,'status')
        log=(self.h.root/'5/process.log').read_text();self.assertEqual(sum('session_open' in l and 'peer 0' in l for l in log.splitlines()),2)
    def test_pool_race_backpressure(self):
        barrier=self.arm(0,'after_status_commit');req=self.h.signed(5,'status')
        with ThreadPoolExecutor(2) as pool:
            first=pool.submit(self.cli,5,0,req);self.reached(barrier)
            second=self.cli(5,0,self.h.signed(5,'status'))
            self.assertNotEqual(second.returncode,0);self.assertIn(b'PEER_BACKPRESSURE',second.stderr)
            self.release(barrier);self.success(first.result(timeout=20))
        self.assertEqual(self.ledger()['height'],0)
    def test_preauth_cap_preserves_existing_authenticated_session(self):
        with self.ready(who=4) as healthy:
            port=int(self.c['peers'][0]['address'].split(':')[1])
            sockets=[socket.create_connection(('127.0.0.1',port),timeout=1) for _ in range(3)]
            try:
                sockets[-1].settimeout(1);self.assertEqual(sockets[-1].recv(1),b'')
                f=self.pending(4,0,self.h.signed(4,'status'));self.send(healthy,f);self.assertEqual(self.receive(healthy)[7],0)
                started=time.monotonic();sockets[0].settimeout(7)
                self.assertEqual(sockets[0].recv(1),b'');self.assertLess(time.monotonic()-started,6.5)
            finally:
                for s in sockets:s.close()
    def test_peer_cap_and_frame_rate_preserve_other_peer(self):
        one=self.ready();two=self.ready()
        try:
            with self.assertRaises((OSError,EOFError)):self.ready()
            with self.ready(who=4) as other:
                self.send(other,self.pending(4,0,self.h.signed(4,'status')));self.assertEqual(self.receive(other)[7],0)
            f=self.pending(5,0,self.h.signed(5,'status'))
            # A duplicate floods one stable session; no expensive ledger reexecution.
            rejected=False
            for _ in range(80):
                try:self.send(one,f);self.receive(one)
                except (OSError,EOFError):rejected=True;break
            self.assertTrue(rejected)
        finally:one.close();two.close()
        self.assertEqual(self.ledger()['height'],0)

    def test_idle_session_timeout(self):
        with self.ready() as s:
            s.settimeout(35);started=time.monotonic()
            with self.assertRaises((OSError,EOFError)):self.receive(s)
            elapsed=time.monotonic()-started
            self.assertGreaterEqual(elapsed,29);self.assertLess(elapsed,34)
        self.assertEqual(self.h.call(0,'status')['ledger']['height'],0)

if __name__=='__main__':unittest.main(verbosity=2)
