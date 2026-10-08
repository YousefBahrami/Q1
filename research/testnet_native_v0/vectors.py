"""Independent Python reconstruction of frozen native framing, no signing claim."""
import sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT/'scripts'))
from localnet_acceptance import encode,decode
rows=dict(line.split('\t') for line in (ROOT/'vectors/testnet-native/v0/approved.tsv').read_text().splitlines())
expected={
 'hello':[1,0,b'TESTNET_NATIVE_TRANSPORT_V0',bytes(range(32)),bytes([17])*32,bytes([34])*32],
 'request_status':[1,1,bytes(range(32)),bytes([17])*32,bytes([34])*32,bytes([51])*32,1,0,b'{"body":{"kind":"status"},"signer":5}'],
 'response_seen':[1,2,bytes(range(32)),bytes([17])*32,bytes([51])*32,bytes([34])*32,1,1,b'{"error":"ALREADY_SEEN"}']}
assert rows=={k:encode(v).hex() for k,v in expected.items()}
for k,v in rows.items():assert decode(bytes.fromhex(v))==expected[k]
print('PASS: 3 native CBOR schema vectors independently reconstructed; synthetic payloads, not valid ledger signatures')
