"""Bounded live trials followed by deterministic transcript replay.

Independent roles in one trusted harness, NOT an isolated hostile-prover test.
No cache flushing, hardware wear benchmark, network or consensus API.
"""
import argparse
import hashlib
import json
import secrets
import statistics
import tempfile
import time
from collections import Counter
from pathlib import Path
from lab import (CHUNK, PROFILE, FileReader, Rejected, Verifier, canonical, digest,
                 full_recovery, manifest, proof, tree, verify)
from sybil import simulate


def run(output):
    output.mkdir(parents=True, exist_ok=False)
    rows, fixtures = [], []
    with tempfile.TemporaryDirectory(prefix='q1-access-v1-') as tmp:
        for mib in (1, 4, 16):
            path = Path(tmp)/f'{mib}.bin'
            # Provider chooses unpredictable bytes; no regeneration seed retained.
            data = secrets.token_bytes(mib * 1024 * 1024)
            with path.open('xb') as f: f.write(data)
            chunks = [data[i:i+CHUNK] for i in range(0,len(data),CHUNK)]
            levels = tree(chunks)
            m = manifest(secrets.token_hex(32),levels[-1][0].hex(),len(chunks),secrets.token_hex(32))
            if mib == 1: sybil_leaves = [x.hex() for x in levels[0][:72]]
            del data
            # Retained RAM copy is available ONLY to named RAM/partial controls.
            # Same-process separation is functional, not a security sandbox.
            readers = [('file_present',FileReader(path)),('warm_cache',FileReader(path)),
                       ('ram_cached',chunks.__getitem__)]
            missing = set(range(len(chunks)//100))
            readers += [('partial_99pct',lambda i: b'' if i in missing else chunks[i]),
                        ('wrong_data',lambda i: b'\0'*CHUNK),
                        ('generated_substitute',lambda i: hashlib.shake_256(b'public-seed'+i.to_bytes(8,'big')).digest(CHUNK))]
            for mode, reader in readers:
                rows.append(trial(mib,mode,m,levels,reader,fixtures))
            path.unlink()
            del readers, reader, chunks
            rows.append(trial(mib,'file_deleted',m,levels,FileReader(path),fixtures))
    result = dict(profile=PROFILE, rounds_per_mode=10, results=rows,
                  not_run={'cold_access':'No verified cache eviction/physical-read instrumentation; first read after write is not cold.',
                           'physical_hdd':'No confirmed HDD used. Media type is not authenticated.'},
                  total_dataset_bytes_written=21*1024*1024,
                  deadline_ns=1_000_000_000,
                  deadline_policy='Fixed generous 1s functional window chosen before trials; not a hardware or calibrated network performance threshold.',
                  isolation='Trusted single-process harness with separate reader capabilities; not malicious OS isolation or proof of all data deletion.',
                  sybil=simulate(sybil_leaves),
                  source_sha256={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(Path(__file__).parent.glob('*.py'))})
    (output/'results.json').write_bytes(canonical(result))
    (output/'fixtures.json').write_bytes(canonical(fixtures))
    # Replay every retained positive proof against its pinned manifest/challenge.
    for f in fixtures:
        assert verify(bytes.fromhex(f['proof']), f['manifest'], f['challenge']) == f['proof_hash']
    sums = ''.join(f'{hashlib.sha256(p.read_bytes()).hexdigest()}  {p.name}\n' for p in sorted(output.glob('*.json')))
    (output/'SHA256SUMS').write_text(sums)
    print(json.dumps(result,indent=2))


def trial(mib,mode,m,levels,reader,fixtures):
    v = Verifier(m)
    v.commit(m)
    reasons, timings, passed = Counter(), [], 0
    recovery = []
    transcripts = []
    for _ in range(2):
        try: full_recovery(m,reader); recovery.append(True)
        except Rejected: recovery.append(False)
        if len(recovery) == 2: break
        for _ in range(10):
            q = v.issue()
            start = time.monotonic_ns()
            try:
                raw = proof(m,q,levels,reader)
            except Rejected:
                raw = None
            proof_hash = None
            outcome = "accepted"
            try:
                proof_hash = v.finish(raw)
                passed += 1
                if mode == 'file_present' and not any(f['mib']==mib for f in fixtures):
                    fixtures.append(dict(mib=mib,manifest=m,challenge=q,proof=raw.hex(),proof_hash=proof_hash))
            except Rejected as error:
                reasons[str(error)] += 1
                outcome = str(error)
            timings.append(time.monotonic_ns()-start)
            transcripts.append(dict(challenge=q,outcome=outcome,proof_hash=proof_hash))
    return dict(mib=mib,mode=mode,attempts=10,passed=passed,failed=10-passed,
                success_rate=passed/10, failure_reasons=dict(reasons),
                median_ms=statistics.median(timings)/1e6,timings_ns=timings,transcripts=transcripts,
                full_recovery_before=recovery[0],full_recovery_after=recovery[1])

if __name__ == '__main__':
    parser=argparse.ArgumentParser()
    parser.add_argument('--output',type=Path,required=True)
    run(parser.parse_args().output)
