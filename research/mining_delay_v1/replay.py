"""Replay frozen positive transcripts; does not recreate live unpredictability."""
import hashlib
import json
from pathlib import Path
import sys
from lab import verify

def replay(directory):
    for line in (directory/'SHA256SUMS').read_text().splitlines():
        expected,name=line.split('  ')
        if name not in ('fixtures.json','results.json'): raise ValueError('unexpected artifact')
        assert hashlib.sha256((directory/name).read_bytes()).hexdigest()==expected
    fixtures=json.loads((directory/'fixtures.json').read_bytes())
    for fixture in fixtures:
        assert verify(bytes.fromhex(fixture['proof']),fixture['manifest'],fixture['challenge'])==fixture['proof_hash']
    result=json.loads((directory/'results.json').read_bytes())
    for row in result['results']:
        assert row['attempts']==row['passed']+row['failed']==len(row['transcripts'])
        assert row['passed']==sum(t['outcome']=='accepted' for t in row['transcripts'])
    print(f'{len(fixtures)} pinned positive proofs replayed; trial counts/checksums verified; no freshness/security inference.')

if __name__=='__main__': replay(Path(sys.argv[1]))
