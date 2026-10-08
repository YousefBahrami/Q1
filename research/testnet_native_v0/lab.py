"""Private TESTNET ONLY credential provisioning and native process harness.

No keys are fixtures. Runtime directories are private and never release assets.
Uses OpenSSL 3, Rust TLS adapter, and the unchanged TESTNET ledger coordinator.
"""
import hashlib
import json
import os
from pathlib import Path
import secrets
import socket
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
TARGET = Path(os.environ.get('CARGO_TARGET_DIR', ROOT/'target')).resolve()
NATIVE = TARGET/'debug/q1-native-transport'
BACKEND = TARGET/'debug/q1-testnet'
PROFILE = 'TESTNET_NATIVE_TRANSPORT_V0'


def canonical(v):
    return json.dumps(v, sort_keys=True, separators=(',', ':'), ensure_ascii=True).encode()


def write(path, data):
    path.write_bytes(data)
    path.chmod(0o600)


def openssl(*args, data=None):
    return subprocess.run(['openssl', *map(str, args)], input=data, capture_output=True, check=True).stdout


def certificate(root, name, ca=True):
    """Fresh P-256 TLS key; CA-signed with both server/client EKUs."""
    key = root/f'{name}.pem'
    openssl('genpkey', '-algorithm', 'EC', '-pkeyopt', 'ec_paramgen_curve:P-256', '-out', key)
    key.chmod(0o600)
    if not ca:
        openssl('req', '-new', '-x509', '-key', key, '-out', root/f'{name}.crt', '-days', '7', '-subj', '/CN=TESTNET ONLY CA', '-addext', 'basicConstraints=critical,CA:TRUE', '-addext', 'keyUsage=critical,keyCertSign,cRLSign')
    else:
        openssl('req', '-new', '-key', key, '-out', root/f'{name}.csr', '-subj', f'/CN=TESTNET ONLY {name}')
        ext = root/f'{name}.ext'
        write(ext, f'basicConstraints=critical,CA:FALSE\nkeyUsage=critical,digitalSignature\nextendedKeyUsage=serverAuth,clientAuth\nsubjectAltName=DNS:{name}.test\n'.encode())
        openssl('x509', '-req', '-in', root/f'{name}.csr', '-CA', root/'ca.crt', '-CAkey', root/'ca.pem', '-set_serial', str(secrets.randbits(120)+1), '-days', '2', '-extfile', ext, '-out', root/f'{name}.crt')
    openssl('x509', '-in', root/f'{name}.crt', '-outform', 'DER', '-out', root/f'{name}.der')
    openssl('pkcs8', '-topk8', '-nocrypt', '-in', key, '-outform', 'DER', '-out', root/f'{name}.key')
    (root/f'{name}.key').chmod(0o600)
    public = openssl('pkey', '-in', key, '-pubout', '-outform', 'DER')
    return hashlib.sha256(b'TESTNET_NATIVE_TRANSPORT_V0_PEER\0'+public).hexdigest()


def backend(root, credential_role, op, **kw):
    env = os.environ.copy()
    env.update(Q1_TESTNET_KEYS_FILE=str(root/f'{credential_role}/ledger-keys.json'), Q1_TESTNET_NATIVE_REQUIRED='1')
    result = subprocess.run([str(BACKEND)], input=canonical(dict(op=op, chain=(root/'nonce').read_text(), **kw)), env=env, capture_output=True)
    if result.returncode:
        raise RuntimeError(result.stderr.decode().strip())
    return json.loads(result.stdout)


def call_native(config, peer, request, verify):
    """Reconcile a durable pending identity before accepting any new operation."""
    args=[str(NATIVE),'request',str(config),str(peer)]
    p=subprocess.run(args,input=canonical(request),capture_output=True,timeout=60)
    if not p.returncode:return p
    different=b'PENDING_REQUIRES_RECONCILIATION' in p.stderr
    same=b'ALREADY_SEEN_RECONCILE' in p.stderr or b'APPLICATION_UNCERTAIN' in p.stderr
    if not (different or same):return p
    recovered=subprocess.run([str(NATIVE),'recover',str(config),str(peer)],input=b'',capture_output=True,timeout=60)
    if recovered.returncode:return recovered
    outcome=json.loads(recovered.stdout)
    if 'error' not in outcome:verify(outcome)
    print(json.dumps(dict(event='request_reconciled',peer=peer,subsequent_operation=different)),file=sys.stderr,flush=True)
    if not different:return recovered
    return subprocess.run(args,input=canonical(request),capture_output=True,timeout=60)


def provision(root):
    root = Path(root)
    root.mkdir(mode=0o700, parents=True, exist_ok=True)
    root.chmod(0o700)
    if (root/'nonce').exists():
        raise ValueError('Refusing to replace an existing chain or credentials')
    write(root/'nonce', secrets.token_hex(32).encode())
    seeds = {str(i):secrets.token_bytes(32) for i in [0,1,2,3,4,5,201,202]}
    public = {}
    for i, seed in seeds.items():
        der = openssl('pkey', '-inform', 'DER', '-pubout', '-outform', 'DER', data=bytes.fromhex('302e020100300506032b657004220420')+seed)
        assert der[:12].hex() == '302a300506032b6570032100'
        public[i] = der[12:].hex()
    for i in range(6):
        d = root/str(i); d.mkdir(mode=0o700)
        (d/'transport').mkdir(mode=0o700)
        own = [str(i), '201'] if i in (3,4) else [str(i)]
        write(d/'ledger-keys.json', canonical(dict(profile=PROFILE, warning='TESTNET ONLY', public=public, seeds={k:seeds[k].hex() for k in own})))
    # No ledger signing secrets in this public replay registry.
    write(root/'public-ledger.json', canonical(dict(profile=PROFILE, warning='TESTNET ONLY', public=public, seeds={})))
    certificate(root, 'ca', ca=False)
    sockets = [socket.socket() for _ in range(6)]
    try:
        for s in sockets: s.bind(('127.0.0.1', 0))
        ports = [s.getsockname()[1] for s in sockets]
    finally:
        for s in sockets: s.close()
    peers = [dict(id=certificate(root, f'q1-{i}'), role=i, address=f'127.0.0.1:{ports[i]}', name=f'q1-{i}.test') for i in range(6)]
    state = backend(root, 5, 'status', history=[])
    for i in range(6):
        d = root/str(i)
        config = dict(profile=PROFILE, warning='TESTNET ONLY', who=i, chain=state['chain_id'], genesis=state['genesis_id'], ca=str(root/'ca.der'), certificate=str(root/f'q1-{i}.der'), key=str(root/f'q1-{i}.key'), state=str(d/'transport'), worker=str(d/'worker.sock'), peers=peers)
        write(d/'config.json', canonical(config))
    return state


class Harness:
    def __init__(self, root):
        self.root = Path(root); self.workers = {}; self.servers = {}; self.logs = {}; self.seq = 100000
        self.initial = provision(self.root)
    def config(self, i): return self.root/str(i)/'config.json'
    def signed(self, who, kind, **kw):
        self.seq += 1
        return backend(self.root, who, 'sign', who=who, body=dict(profile='TESTNET_FAILOVER_V0', chain=(self.root/'nonce').read_text(), kind=kind, seq=self.seq, **kw))
    def rpc(self, who, target, request):
        p = call_native(self.config(who),target,request,lambda result:backend(self.root,who,'verify',envelope=result))
        if p.returncode: raise RuntimeError(p.stderr.decode().strip())
        result = json.loads(p.stdout)
        if 'error' in result: raise RuntimeError(result['error'])
        assert result['signer'] == target
        return backend(self.root, who, 'verify', envelope=result)
    def call(self, target, kind, **kw): return self.rpc(5, target, self.signed(5, kind, **kw))
    def start(self, i):
        d = self.root/str(i)
        (d/'transport/stop').unlink(missing_ok=True)
        (d/'worker.sock').unlink(missing_ok=True)
        log = (d/'process.log').open('ab'); self.logs[i] = log
        env = os.environ.copy()
        if (d/'barriers').is_dir():env['Q1_NATIVE_TEST_BARRIERS']=str(d/'barriers')
        env.update(Q1_TESTNET_KEYS_FILE=str(d/'ledger-keys.json'), Q1_TESTNET_NATIVE_REQUIRED='1', Q1_TESTNET_LAB_CHAIN=(self.root/'nonce').read_text())
        self.workers[i] = subprocess.Popen([sys.executable, str(Path(__file__).with_name('worker.py')), str(self.config(i))], env=env, stdout=log, stderr=log)
        deadline = time.monotonic()+10
        while not (d/'worker.sock').exists():
            if self.workers[i].poll() is not None or time.monotonic()>deadline: raise RuntimeError(f'worker {i} startup failed')
            time.sleep(.02)
        self.servers[i] = subprocess.Popen([str(NATIVE), 'serve', str(self.config(i))], stdout=log, stderr=log)
        config = json.loads(self.config(i).read_bytes())
        port = int(config['peers'][i]['address'].split(':')[1])
        while True:
            if self.servers[i].poll() is not None: raise RuntimeError(f'transport {i} exited')
            try:
                with socket.create_connection(('127.0.0.1', port), timeout=.1): break
            except OSError:
                if time.monotonic()>deadline: raise
                time.sleep(.02)
    def stop(self, i, crash=False):
        server = self.servers.pop(i, None)
        if server:
            if server.poll() is None:
                if crash: server.kill()
                else: subprocess.run([str(NATIVE), 'stop', str(self.config(i))], check=True, capture_output=True)
            server.wait(timeout=45)
        worker = self.workers.pop(i, None)
        if worker:
            if worker.poll() is None:
                if crash: worker.kill()
                else: worker.terminate()
            worker.wait(timeout=5)
        log = self.logs.pop(i, None)
        if log: log.close()
    def close(self):
        for i in list(self.workers): self.stop(i)
    def status(self, i):
        # Controller's own learner ledger is local operator inspection.
        if i == 5:
            history = json.loads((self.root/'5/ledger/journal.json').read_bytes())['state']['history']
            return dict(history=history, ledger=backend(self.root, 5, 'status', history=history))
        return self.call(i, 'status')
    def roots(self): return {str(i):self.status(i)['ledger']['state_root'] for i in self.workers}
