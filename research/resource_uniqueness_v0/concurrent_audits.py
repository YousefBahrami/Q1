"""Concurrent access counterexamples and conditional reward math; no issuance."""
from concurrent.futures import ThreadPoolExecutor
from fractions import Fraction
import hashlib
import json
import multiprocessing
from pathlib import Path
import secrets
import tempfile
import threading
import time
import zlib
from experiment import (CHUNK, COUNT, ROOT, FileReader, RemoteReader, Verifier,
                        Rejected, canonical, content, helper, manifest, proof, tree)

MODELS = ('shared_storage', 'duplicate_commitments', 'remote_outsourcing',
          'resource_leasing', 'reconstruction', 'partial_retention')


def run(output):
    output.mkdir(parents=True, exist_ok=False)
    results = []
    with tempfile.TemporaryDirectory(prefix='q1-concurrent-resource-') as tmp:
        path = Path(tmp)/'fixture'
        corpus = b''.join(content(i) for i in range(COUNT))
        path.write_bytes(corpus)
        levels = tree([content(i) for i in range(COUNT)])
        packed = zlib.compress(corpus, 9)
        partial = {i:content(i) for i in range(COUNT//2)}
        local = FileReader(path)
        ctx = multiprocessing.get_context('spawn')
        parent, child = ctx.Pipe(duplex=False)
        process = ctx.Process(target=helper, args=(str(path), child))
        process.start(); child.close()
        try:
            if not parent.poll(10):
                raise RuntimeError('helper not ready')
            remote = RemoteReader(parent.recv())
            shared = threading.Lock()
            for model in MODELS:
                for count in (1, 2, 4, 8):
                    barrier = threading.Barrier(count)
                    reads = [0]
                    remote_before = remote.calls

                    def attempt(index):
                        label = f'claim-{index}'
                        m = manifest(hashlib.sha256(label.encode()).hexdigest(),
                                     levels[-1][0].hex(), COUNT, secrets.token_hex(32))
                        verifier = Verifier(m); verifier.commit(m)
                        q = verifier.issue(8)
                        barrier.wait(timeout=5)
                        begin = time.monotonic_ns()

                        def read(i):
                            # One shared simulated service queue. Not measured hardware IO.
                            with shared:
                                time.sleep(.002)
                                reads[0] += 1
                                if model == 'remote_outsourcing': return remote(i)
                                if model == 'partial_retention':
                                    return partial[i] if i in partial else remote(i)
                                if model == 'reconstruction':
                                    return zlib.decompress(packed)[i*CHUNK:(i+1)*CHUNK]
                                return local(i)
                        raw = proof(m,q,levels,read)
                        try:
                            digest=verifier.finish(raw);accepted=True;reason='accepted'
                        except Rejected as error:
                            digest=None;accepted=False;reason=str(error)
                        return dict(label=label,manifest=m,challenge=q,proof_hash=digest,
                                    accepted=accepted,reason=reason,response_ns=time.monotonic_ns()-begin)
                    with ThreadPoolExecutor(max_workers=count) as pool:
                        claims=list(pool.map(attempt, range(count)))
                    accepted=sum(c['accepted'] for c in claims)
                    results.append(dict(model=model,identities=count,claims=claims,
                        accepted=accepted,chunk_reads=reads[0],remote_reads=remote.calls-remote_before,
                        per_identity_units=accepted,
                        one_externally_authorized_job_units=int(accepted>0),
                        self_issued_job_units=accepted,
                        hypothetical_lottery_share_against_eight_other_tickets=str(Fraction(accepted,8+accepted))))

            # The partial holder cannot use the helper in this control. A challenge
            # sampling every chunk deterministically includes missing retained data.
            m=manifest('11'*32,levels[-1][0].hex(),COUNT,'22'*32)
            v=Verifier(m);v.commit(m);q=v.issue(COUNT)
            try:proof(m,q,levels,lambda i:partial.get(i,b''))
            except Rejected as error:missing=str(error)
            else:raise AssertionError('missing retained data accepted')
            record=dict(profile='Q1_RESOURCE_UNIQUENESS_CONCURRENT_V0_RESEARCH_ONLY',
                results=results,logical_fixture_bytes=len(corpus),compressed_payload_bytes=len(packed),
                partial_retained_payload_bytes=sum(map(len,partial.values())),
                partial_without_helper_control=missing,
                topology='one host; separate loopback helper process; not a WAN experiment',
                reward_model='dimensionless eligibility/receipt thought experiment; no selected economics',
                limits=['same backing fixture, increasing audit work; no zero-cost claim',
                        'synthetic 2ms shared queue, not hardware capacity or saturation',
                        'claim labels are not authenticated resource owners',
                        'authorized job is an assumed external one-use receipt, not implemented permissionless demand',
                        'no physical resource oracle, monetary issuance or consensus weight'],
                source_sha256={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()
                    for p in (Path(__file__),ROOT/'research/resource_uniqueness_v0/experiment.py',ROOT/'research/mining_delay_v1/lab.py')})
            raw=canonical(record);(output/'results.json').write_bytes(raw)
            (output/'SHA256SUMS').write_text(hashlib.sha256(raw).hexdigest()+'  results.json\n')
            print(json.dumps(dict(cohorts=len(results),attempts=sum(len(r['claims']) for r in results),
                accepted=sum(r['accepted'] for r in results),partial_without_helper=missing)))
        finally:
            parent.close()
            if process.is_alive():process.terminate()
            process.join(timeout=5)
            if process.is_alive():process.kill();process.join()


def replay(directory):
    raw=(directory/'results.json').read_bytes()
    digest,name=(directory/'SHA256SUMS').read_text().split()
    assert name=='results.json' and hashlib.sha256(raw).hexdigest()==digest
    record=json.loads(raw)
    for name,digest in record['source_sha256'].items():
        assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==digest
    levels=tree([content(i) for i in range(COUNT)])
    count=0
    for row in record['results']:
        accepted=sum(c['accepted'] for c in row['claims'])
        assert accepted==row['accepted']==row['per_identity_units']==row['self_issued_job_units']
        assert row['one_externally_authorized_job_units']==int(accepted>0)
        assert row['hypothetical_lottery_share_against_eight_other_tickets']==str(Fraction(accepted,8+accepted))
        for claim in row['claims']:
            if claim['accepted']:
                from lab import verify
                raw=proof(claim['manifest'],claim['challenge'],levels,content)
                assert verify(raw,claim['manifest'],claim['challenge'])==claim['proof_hash']
                count+=1
    print(f'{count} canonical proofs replayed; timing and resource ownership not replayed')


if __name__ == '__main__':
    import argparse
    p=argparse.ArgumentParser();p.add_argument('--output',type=Path);p.add_argument('--replay',type=Path)
    args=p.parse_args()
    if bool(args.output)==bool(args.replay):p.error('choose exactly one of --output / --replay')
    if args.output:run(args.output)
    else:replay(args.replay)
