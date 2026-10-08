"""Reconstruct canonical accepted proof bytes; timings remain historical observations."""
import hashlib
import json
from pathlib import Path
import random
import sys
import experiment as e


def replay(directory):
    directory=Path(directory)
    digest,name=(directory/'SHA256SUMS').read_text().split()
    raw=(directory/name).read_bytes();assert hashlib.sha256(raw).hexdigest()==digest
    r=json.loads(raw);count=0
    for row in r['measurements']:
        for k,c in enumerate(row['claims']):
            order=list(range(e.COUNT))
            if row['attack']=='distinct_commitments':random.Random(k+9100).shuffle(order)
            levels=e.tree([e.content(i) for i in order])
            assert c['root']==levels[-1][0].hex()
            if c['accepted']:
                raw=e.proof(c['manifest'],c['challenge'],levels,lambda i:e.content(order[i]))
                from lab import verify
                assert verify(raw,c['manifest'],c['challenge'])==c['proof_hash']
                count+=1
        assert e.score(row['claims'])==row['units']
    print(f'{count} accepted resource proofs reconstructed and verified; timing/physical ownership not proved by replay.')

if __name__=='__main__':replay(sys.argv[1])
