"""Actual six-role, ONE HOST native TLS acceptance. Never multi-host evidence."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile
from lab import Harness, ROOT, BACKEND, canonical


def run(output):
    output.mkdir(parents=True, exist_ok=False)
    with tempfile.TemporaryDirectory(prefix='qn-', dir='/tmp') as tmp:
        h = Harness(Path(tmp)); tests = {}
        try:
            for i in range(6): h.start(i)
            assert len(set(h.roots().values()))==1
            r = h.call(3, 'run', ballot=0, payload=10, crash=0)
            tests['fresh_genesis_signed_transfer'] = r['ledger']['height']==1
            # Height 2 producer 4 unavailable; authorized producer 3 uses ballot 1.
            h.stop(4, crash=True)
            r = h.call(3, 'run', ballot=1, payload=10, crash=0)
            tests['producer_failover'] = r['ledger']['height']==2
            h.start(4); h.call(4, 'sync', history=h.status(3)['history'])
            # Stop both adapter/worker of a voter: remaining two still certify.
            h.stop(2, crash=True)
            r = h.call(3, 'run', ballot=0, payload=10, crash=0)
            tests['two_voter_progress'] = len(r['certificate']['votes'])==2
            h.start(2)
            tests['stale_voter_before_sync'] = h.status(2)['ledger']['height']==2
            h.call(2, 'sync', history=h.status(3)['history'])
            tests['voter_restart_catchup'] = len(set(h.roots().values()))==1
            # Native endpoint disconnect/reconnect; retained identity and journals.
            h.stop(0, crash=True)
            r = h.call(4, 'run', ballot=0, payload=10, crash=0)
            h.start(0); h.call(0, 'sync', history=h.status(4)['history'])
            tests['disconnect_reconnect_sync'] = len(set(h.roots().values()))==1
            r = h.call(3, 'run', ballot=0, payload=10, crash=0)
            roots = h.roots(); assert len(roots)==6 and len(set(roots.values()))==1
            ledger = r['ledger']
            assert (ledger['height'],ledger['sender'],ledger['recipient'],ledger['nonce'],ledger['reward_pool'],ledger['total_supply'])==(5,'945','50',5,'5','1000')
            tests['baseline_economics_equivalent'] = True
            history = h.status(3)['history']
            for i in range(6): h.stop(i)
            for i in range(6): h.start(i)
            tests['full_process_restart_identical_roots'] = h.roots()==roots
            assert all(tests.values()), tests
            public = json.loads((h.root/'public-ledger.json').read_bytes())
            import os
            env = os.environ.copy(); env.update(Q1_TESTNET_KEYS_FILE=str(h.root/'public-ledger.json'),Q1_TESTNET_NATIVE_REQUIRED='1')
            replay = subprocess.run([str(BACKEND)],input=canonical(dict(op='status',chain=(h.root/'nonce').read_text(),history=history)),env=env,capture_output=True,check=True)
            assert json.loads(replay.stdout)==ledger
            tests['public_keys_only_ledger_replay'] = True
            report = dict(profile='TESTNET_NATIVE_TRANSPORT_V0',warning='TESTNET ONLY',physical_hosts=1,roles=6,transport='direct loopback TCP / TLS 1.3 mTLS; no SSH',chain_nonce=(h.root/'nonce').read_text(),public_ledger=public,history=history,ledger=ledger,final_state_roots=roots,tests=tests,native_two_host='BLOCKED: no approved private route',public_testnet='BLOCKED',source_sha256={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in [*sorted(Path(__file__).parent.glob('*.py')),*sorted((ROOT/'crates/q1-native-transport/src').glob('*.rs')),ROOT/'crates/q1-testnet/src/main.rs',ROOT/'research/testnet_ledger_v0/node.py']})
            raw = canonical(report); (output/'acceptance.json').write_bytes(raw)
            (output/'SHA256SUMS').write_text(hashlib.sha256(raw).hexdigest()+'  acceptance.json\n')
            print(json.dumps(dict(tests=tests, ledger=ledger, physical_hosts=1), indent=2))
        except BaseException:
            for p in Path(tmp).glob('*/process.log'): print(p.parent.name,p.read_text()[-2000:])
            raise
        finally: h.close()

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--output',type=Path,required=True)
    run(p.parse_args().output)
