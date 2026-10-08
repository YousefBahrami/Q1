"""TLS black-box negative matrix with actual native adapter and ledger worker."""
import copy
import json
import os
from pathlib import Path
import socket
import ssl
import struct
import subprocess
import sys
import tempfile
import time
import unittest
from lab import Harness, ROOT, NATIVE, BACKEND, canonical, certificate, write
sys.path.insert(0, str(ROOT/'scripts'))
from localnet_acceptance import encode, decode


class NativeTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.tmp = tempfile.TemporaryDirectory(prefix='qnt-', dir='/tmp')
        cls.h = Harness(Path(cls.tmp.name))
        cls.h.start(0)
        cls.c = json.loads(cls.h.config(0).read_bytes())
        cls.port = int(cls.c['peers'][0]['address'].split(':')[1])
    @classmethod
    def tearDownClass(cls):
        cls.h.close(); cls.tmp.cleanup()
    def connect(self, name='q1-5'):
        context=ssl.create_default_context(ssl.Purpose.SERVER_AUTH,cafile=str(self.h.root/'ca.crt'))
        context.minimum_version=context.maximum_version=ssl.TLSVersion.TLSv1_3
        context.set_alpn_protocols(['q1-testnet-native-v0'])
        context.load_cert_chain(str(self.h.root/f'{name}.crt'),str(self.h.root/f'{name}.pem'))
        s=context.wrap_socket(socket.socket(),server_hostname='q1-0.test');s.settimeout(8)
        try: s.connect(('127.0.0.1',self.port))
        except BaseException: s.close(); raise
        return s
    def hello(self, who=5):
        return [1,0,b'TESTNET_NATIVE_TRANSPORT_V0',bytes.fromhex(self.c['chain']),bytes.fromhex(self.c['genesis']),bytes.fromhex(self.c['peers'][who]['id'])]
    def send(self,s,v):
        b=encode(v);s.sendall(struct.pack('!I',len(b))+b)
    def receive(self,s):
        def exact(n):
            out=b''
            while len(out)<n:
                b=s.recv(n-len(out))
                if not b:raise EOFError('closed')
                out+=b
            return out
        n=struct.unpack('!I',exact(4))[0];self.assertLessEqual(n,65536)
        return decode(exact(n))
    def ready(self, who=5):
        s=self.connect(f'q1-{who}')
        try: self.send(s,self.hello(who));self.assertEqual(self.receive(s),self.hello(0))
        except BaseException: s.close();raise
        return s
    def frame(self, seq, request, operation=0, who=5):
        return [1,1,*self.hello()[3:5],bytes.fromhex(self.c['peers'][who]['id']),bytes.fromhex(self.c['peers'][0]['id']),seq,operation,canonical(request)]
    def rejected(self, f):
        with self.assertRaises((ssl.SSLError,ConnectionError,EOFError,OSError)): f()
    def test_01_unknown_peer(self):
        certificate(self.h.root,'unknown')
        def attempt():
            with self.connect('unknown') as s:self.send(s,self.hello());self.receive(s)
        self.rejected(attempt)
    def test_02_invalid_certificate(self):
        certificate(self.h.root,'untrusted',ca=False)
        def attempt():
            with self.connect('untrusted') as s:self.send(s,self.hello());self.receive(s)
        self.rejected(attempt)
    def test_03_wrong_network(self):
        def attempt():
            with self.connect() as s:
                hello=self.hello();hello[3]=bytes(32);self.send(s,hello);self.receive(s)
        self.rejected(attempt)
    def test_04_duplicate_and_05_replay(self):
        f=self.frame(1,self.h.signed(5,'status'))
        with self.ready() as s:self.send(s,f);self.assertEqual(self.receive(s)[7],0)
        path=self.h.root/'0/ledger/journal.json';before=path.read_bytes()
        with self.ready() as s:self.send(s,f);self.assertEqual(self.receive(s)[7],1)
        altered=copy.deepcopy(f);altered[8]=canonical(self.h.signed(5,'status'))
        with self.ready() as s:self.send(s,altered);self.assertEqual(self.receive(s)[7],2)
        self.assertEqual(path.read_bytes(),before)
        self.__class__.replay_frame=f
    def test_06_oversized(self):
        def attempt():
            with self.ready() as s:s.sendall(struct.pack('!I',65537));self.receive(s)
        self.rejected(attempt)
    def test_07_malformed(self):
        def attempt():
            with self.ready() as s:s.sendall(b'\0\0\0\2\x18\x01');self.receive(s)
        self.rejected(attempt)
    def test_08_truncated_and_13_disconnect_midframe(self):
        for data in [b'\0\0',b'\0\0\0\x10\x89\x01']:
            with self.ready() as s:s.sendall(data)
        time.sleep(.1)
        self.assertIsNone(self.h.servers[0].poll())
    def test_09_stale_height(self):
        with self.ready(3) as s:
            self.send(s,self.frame(1,self.h.signed(3,'prepare',height=99,ballot=0),1,3))
            r=self.receive(s);self.assertEqual(json.loads(r[8])['error'],'STALE_OR_FUTURE_HEIGHT')
    def test_10_invalid_signature(self):
        req=self.h.signed(5,'status');req['signature']='00'*64
        with self.ready() as s:
            self.send(s,self.frame(2,req));r=self.receive(s)
            self.assertIn('error',json.loads(r[8]))
    def test_11_reconnect_storm(self):
        connections=[]
        try:
            for _ in range(20):connections.append(socket.create_connection(('127.0.0.1',self.port),timeout=1))
            time.sleep(.2)
            self.assertIsNone(self.h.servers[0].poll())
        finally:
            for s in connections:s.close()
        time.sleep(.2)
        with self.ready() as s:
            self.send(s,self.frame(3,self.h.signed(5,'status')));self.assertEqual(self.receive(s)[7],0)
    def test_12_delayed_peer(self):
        with self.ready() as s:
            b=encode(self.frame(4,self.h.signed(5,'status')))
            s.sendall(struct.pack('!I',len(b))+b[:2]);time.sleep(.2);s.sendall(b[2:])
            self.assertEqual(self.receive(s)[7],0)
    def test_14_restart_replay_retained(self):
        journal=self.h.root/'0/ledger/journal.json';before=journal.read_bytes()
        self.h.stop(0,crash=True);self.h.start(0)
        with self.ready() as s:self.send(s,self.replay_frame);self.assertEqual(self.receive(s)[7],1)
        self.assertEqual(journal.read_bytes(),before)
    def test_15_public_fixture_signature_rejected(self):
        req=dict(op='sign',chain=(self.h.root/'nonce').read_text(),who=5,body=dict(profile='TESTNET_FAILOVER_V0',chain=(self.h.root/'nonce').read_text(),kind='status',seq=999999))
        env=os.environ.copy();env.pop('Q1_TESTNET_KEYS_FILE',None);env.pop('Q1_TESTNET_NATIVE_REQUIRED',None)
        p=subprocess.run([str(BACKEND)],input=canonical(req),env=env,capture_output=True,check=True)
        with self.ready() as s:
            self.send(s,self.frame(5,json.loads(p.stdout)));self.assertIn('error',json.loads(self.receive(s)[8]))
    def test_16_native_key_required(self):
        env=os.environ.copy();env.pop('Q1_TESTNET_KEYS_FILE',None);env['Q1_TESTNET_NATIVE_REQUIRED']='1'
        p=subprocess.run([str(BACKEND)],input=b'{}',env=env,capture_output=True)
        self.assertNotEqual(p.returncode,0);self.assertIn(b'native credentials required',p.stderr)
    def test_17_public_bind_rejected(self):
        c=copy.deepcopy(self.c);c['peers'][0]['address']='8.8.8.8:9000'
        p=self.h.root/'bad-config.json';write(p,canonical(c))
        result=subprocess.run([str(NATIVE),'serve',str(p)],capture_output=True,timeout=5)
        self.assertNotEqual(result.returncode,0);self.assertIn(b'PRIVATE_ENDPOINT_REQUIRED',result.stderr)
    def test_18_final_state_unchanged(self):
        history=json.loads((self.h.root/'0/ledger/journal.json').read_bytes())['state']['history']
        self.assertEqual(history,[])
        self.assertEqual(json.loads((self.h.root/'0/ledger/journal.json').read_bytes())['state']['state_root'],self.h.initial['state_root'])

    def test_19_reservation_before_failed_dispatch(self):
        worker=self.h.workers[0];worker.terminate();worker.wait(timeout=5)
        f=self.frame(6,self.h.signed(5,'status'))
        with self.ready() as s:self.send(s,f);self.assertEqual(self.receive(s)[7],3)
        self.h.stop(0);self.h.start(0)
        before=(self.h.root/'0/ledger/journal.json').read_bytes()
        with self.ready() as s:self.send(s,f);self.assertEqual(self.receive(s)[7],1)
        self.assertEqual((self.h.root/'0/ledger/journal.json').read_bytes(),before)
    def test_20_wrong_version_and_role(self):
        for variant in ('version','signer'):
            def attempt():
                with self.ready() as s:
                    f=self.frame(7,self.h.signed(5,'status'))
                    if variant=='version':f[0]=2
                    else:
                        req=json.loads(f[8]);req['signer']=4;f[8]=canonical(req)
                    self.send(s,f);self.receive(s)
            self.rejected(attempt)
    def test_21_absolute_slow_frame_timeout(self):
        with self.ready() as s:
            s.sendall(b'\0\0\0\x10\x89')
            started=time.monotonic()
            self.rejected(lambda:self.receive(s))
            self.assertLess(time.monotonic()-started,7)
    def test_22_retry_pending_survives_restart(self):
        # Induce a recorded-but-undispatched operator RPC with controller 5.
        # Use another receiver role to avoid the raw test driver's counters.
        self.h.start(1)
        worker=self.h.workers[1];worker.terminate();worker.wait(timeout=5)
        req=self.h.signed(5,'status')
        args=[str(NATIVE),'request',str(self.h.config(5)),'1']
        first=subprocess.run(args,input=canonical(req),capture_output=True,timeout=20)
        self.assertNotEqual(first.returncode,0)
        path=self.h.root/'5/transport/send-1.json';before=path.read_bytes()
        self.assertEqual(json.loads(before)['pending'],req)
        self.h.stop(1);self.h.start(1)
        second=subprocess.run(args,input=canonical(req),capture_output=True,timeout=20)
        self.assertNotEqual(second.returncode,0);self.assertIn(b'ALREADY_SEEN_RECONCILE',second.stderr)
        self.assertEqual(path.read_bytes(),before)
        other=subprocess.run(args,input=canonical(self.h.signed(5,'status')),capture_output=True,timeout=10)
        self.assertNotEqual(other.returncode,0);self.assertIn(b'PENDING_REQUIRES_RECONCILIATION',other.stderr)
        self.assertEqual(path.read_bytes(),before)

if __name__=='__main__':unittest.main(verbosity=2)
