"""Adversarial resource accounting, with no physical-resource oracle in scoring."""
import hashlib
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import json
import multiprocessing
from pathlib import Path
import random
import secrets
import statistics
import sys
import tempfile
import time
import urllib.request
import zlib

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'research/mining_delay_v1'))
from lab import CHUNK, FileReader, Rejected, Verifier, canonical, manifest, proof, tree

PROFILE = 'Q1_RESOURCE_UNIQUENESS_V0_RESEARCH_ONLY'
COUNT = 32
ATTACKS = ('multiple_keys', 'one_device', 'remote_outsourcing', 'leasing',
           'distinct_commitments', 'deduplicated_storage', 'compressed_reconstructed',
           'partial_on_demand', 'colluding_pool')


def content(index):
    # Four distinct useful static help-page fixtures repeated in an archive.
    # Deliberately compressible public data; this is not a capacity benchmark.
    page = canonical({'page': index % 4, 'format': 'Q1_PUBLIC_HELP_FIXTURE',
                      'text': 'Technical evaluation only. ' * 32})
    return page.ljust(CHUNK, b' ')


def helper(path, ready):
    reader = FileReader(path)
    class Handler(BaseHTTPRequestHandler):
        def do_GET(self):
            if not self.path.startswith('/chunk/') or not self.path[7:].isdigit():
                self.send_error(400); return
            i = int(self.path[7:])
            if not 0 <= i < COUNT:
                self.send_error(400); return
            data = reader(i)
            self.send_response(200); self.send_header('Content-Length', str(len(data)))
            self.end_headers(); self.wfile.write(data)
        def log_message(self, *args): pass
    with ThreadingHTTPServer(('127.0.0.1', 0), Handler) as server:
        ready.send(server.server_port); ready.close(); server.serve_forever()


class RemoteReader:
    def __init__(self, port): self.port, self.calls, self.bytes = port, 0, 0
    def __call__(self, i):
        with urllib.request.urlopen(f'http://127.0.0.1:{self.port}/chunk/{i}', timeout=2) as r:
            raw = r.read(CHUNK + 1)
        self.calls += 1; self.bytes += len(raw)
        return raw


def score(claims):
    """Units only, never coins. All identifiers below are visible claim metadata.

    The unique-job variant relies on a separately authorized job identifier;
    untrusted self-created jobs are deliberately evaluated as a counterexample.
    """
    ok = [c for c in claims if c['accepted']]
    return {
        'per_identity': len({c['key'] for c in ok}),
        'unique_data_commitment': len({c['root'] for c in ok}),
        'claimed_region': len({(c['root'], c['region']) for c in ok}),
        'claimed_capacity': sum(c['capacity_units'] for c in ok),
        'self_issued_lease': len({c['lease'] for c in ok}),
        'resource_epoch': len({(c['root'], c['epoch']) for c in ok}),
        'availability_per_key_epoch': len({(c['key'], c['epoch']) for c in ok}),
        'authorized_service_job': len({c['authorized_job'] for c in ok}),
        'self_issued_service_job': len({c['self_job'] for c in ok}),
    }


def attempt(reader, levels, key, session):
    m = manifest(hashlib.sha256(key.encode()).hexdigest(), levels[-1][0].hex(), COUNT, session)
    v = Verifier(m); v.commit(m); q = v.issue(8); start = time.monotonic_ns()
    try: raw = proof(m, q, levels, reader)
    except Rejected: raw = None
    built = time.monotonic_ns()
    try: proof_hash = v.finish(raw); accepted, reason = True, 'accepted'
    except Rejected as e: proof_hash = None; accepted, reason = False, str(e)
    end = time.monotonic_ns()
    return dict(key=key, manifest=m, root=m['root'], accepted=accepted, reason=reason,
                challenge=q, proof_hash=proof_hash, proof_bytes=len(raw) if raw else 0,
                response_ns=end-start, verification_ns=end-built,
                region=key+'-claimed-region', capacity_units=1, lease=key+'-lease',
                epoch=1, authorized_job='one-customer-authorized-job', self_job=key+'-job')


def run(output):
    output.mkdir(parents=True, exist_ok=False)
    records = []
    with tempfile.TemporaryDirectory(prefix='q1-uniqueness-') as tmp:
        path = Path(tmp)/'shared-data'; path.write_bytes(b''.join(content(i) for i in range(COUNT)))
        file_reader = FileReader(path)
        ctx = multiprocessing.get_context('spawn'); parent, child = ctx.Pipe(duplex=False)
        process = ctx.Process(target=helper, args=(str(path), child)); process.start(); child.close()
        try:
            if not parent.poll(10): raise RuntimeError('helper startup timeout')
            remote = RemoteReader(parent.recv()); parent.close()
            compressed = zlib.compress(path.read_bytes(), level=9)
            unique_chunks = {hashlib.sha256(content(i)).hexdigest():content(i) for i in range(COUNT)}
            chunk_ids = [hashlib.sha256(content(i)).hexdigest() for i in range(COUNT)]
            local_half = {i: content(i) for i in range(COUNT//2)}
            for attack in ATTACKS:
                for keys in (1, 2, 4, 8):
                    claims = []; remote_before = remote.calls
                    for k in range(keys):
                        order = list(range(COUNT))
                        if attack == 'distinct_commitments': random.Random(k + 9100).shuffle(order)
                        levels = tree([content(i) for i in order])
                        def reader(i):
                            j = order[i]
                            if attack in ('remote_outsourcing', 'colluding_pool'): return remote(j)
                            if attack == 'partial_on_demand': return local_half[j] if j in local_half else remote(j)
                            if attack == 'deduplicated_storage': return unique_chunks[chunk_ids[j]]
                            if attack == 'compressed_reconstructed': return zlib.decompress(compressed)[j*CHUNK:(j+1)*CHUNK]
                            # Leasing is access to the same file; no legal contract,
                            # ownership or exclusive reservation is inferred.
                            return file_reader(j)
                        claims.append(attempt(reader, levels, f'fixture-key-{k}', secrets.token_hex(32)))
                    records.append(dict(attack=attack, identities=keys, claims=claims,
                                        units=score(claims), helper_chunk_requests=remote.calls-remote_before,
                                        median_response_ms=statistics.median(c['response_ns'] for c in claims)/1e6,
                                        median_verify_ms=statistics.median(c['verification_ns'] for c in claims)/1e6))
            levels=tree([content(i) for i in range(COUNT)])
            missing=attempt(lambda i:b'',levels,'missing','33'*32)
            delayed=[False]
            def slow(i):
                if not delayed[0]:
                    delayed[0]=True;time.sleep(1.05)
                return content(i)
            late=attempt(slow,levels,'late','44'*32)
            report = dict(profile=PROFILE, measurements=records, negative_controls=[missing,late],
                fixture_logical_bytes=COUNT*CHUNK, unique_chunk_payload_bytes=sum(map(len, unique_chunks.values())),
                compressed_payload_bytes=len(compressed), chunk_index_entries=len(chunk_ids),
                helper_requests=remote.calls, helper_response_bytes=remote.bytes,
                helper_location='separate process on same host over loopback TCP; NOT a remote-host result',
                scoring='dimensionless reward-unit counterexamples; no issuance or economic prices',
                ground_truth='controlled one shared fixture; not used by score() and not a permissionless oracle',
                limits=['sequential challenges, not concurrency saturation', 'declared reader capabilities, no hostile OS erasure',
                        'one filesystem file is not proof of one physical device', 'availability is sampled, not continuous',
                        'keys are synthetic labels; no identity signatures in the v1 resource proof'],
                source_sha256={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()
                    for p in [Path(__file__),ROOT/'research/mining_delay_v1/lab.py']})
            raw = canonical(report); (output/'results.json').write_bytes(raw)
            (output/'SHA256SUMS').write_text(hashlib.sha256(raw).hexdigest()+'  results.json\n')
            print(json.dumps({k:v for k,v in report.items() if k not in ('measurements','source_sha256')},indent=2))
        finally:
            parent.close()
            if process.is_alive(): process.terminate()
            process.join(timeout=5)
            if process.is_alive(): process.kill(); process.join()


if __name__ == '__main__':
    import argparse
    p=argparse.ArgumentParser();p.add_argument('--output', type=Path, required=True)
    run(p.parse_args().output)
