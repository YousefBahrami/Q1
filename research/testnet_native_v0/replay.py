"""Replay frozen native acceptance using only its public ledger registry."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
from lab import BACKEND, canonical, write


def replay(directory):
    raw=(directory/'acceptance.json').read_bytes()
    recorded=dict(line.split('  ',1)[::-1] for line in (directory/'SHA256SUMS').read_text().splitlines())
    assert hashlib.sha256(raw).hexdigest()==recorded['acceptance.json']
    r=json.loads(raw);assert r['physical_hosts']==1 and r['public_ledger']['seeds']=={}
    with tempfile.TemporaryDirectory(prefix='qnr-',dir='/tmp') as tmp:
        key=Path(tmp)/'public.json';write(key,canonical(r['public_ledger']))
        env=os.environ.copy();env.update(Q1_TESTNET_KEYS_FILE=str(key),Q1_TESTNET_NATIVE_REQUIRED='1')
        p=subprocess.run([str(BACKEND)],input=canonical(dict(op='status',chain=r['chain_nonce'],history=r['history'])),env=env,capture_output=True,check=True)
        assert json.loads(p.stdout)==r['ledger']
        assert set(r['final_state_roots'].values())=={r['ledger']['state_root']}
    print('PASS: frozen native evidence checksum and actual signed ledger replay with public keys only')

if __name__=='__main__':replay(Path(sys.argv[1]))
