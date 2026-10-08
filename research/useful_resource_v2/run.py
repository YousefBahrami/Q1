"""Useful immutable content availability; never a physical-capacity proof."""
import argparse
import hashlib
import io
import json
from pathlib import Path
import secrets
import statistics
import sys
import tarfile
import tempfile
import time
ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT/'research/mining_delay_v1'))
from lab import CHUNK, FileReader, Rejected, Verifier, canonical, full_recovery, manifest, proof, tree


def object_data():
    # Original, redistributable application test objects, not customer records.
    return canonical({'format':'Q1_OBJECT_FIXTURE_V2','objects':[
        {'id':i,'type':'resource-request','operation':'retrieve','object':f'example/{i:06}.txt',
         'content':f'Q1 non-sensitive static service fixture {i}. No monetary rights.'}
        for i in range(2048)]})


def dataset_data():
    # Useful deterministic test dataset for readers, paging and aggregation.
    return ('row,job,bytes,status\n'+''.join(f'{i},job-{i//8},{(i%97+1)*4096},complete\n' for i in range(16384))).encode()


def archive_data():
    out=io.BytesIO()
    with tarfile.open(fileobj=out,mode='w') as tar:
        for name in ('LICENSE','vectors/localnet/v0/approved.tsv'):
            data=(ROOT/name).read_bytes()
            info=tarfile.TarInfo(name); info.size=len(data); info.mtime=0; info.mode=0o644
            tar.addfile(info,io.BytesIO(data))
    return out.getvalue()


def pad(data):
    chunks=max(1,(len(data)+CHUNK-1)//CHUNK)
    count=1<<(chunks-1).bit_length()
    return data.ljust(count*CHUNK,b'\0')


def utility(name,data):
    if name=='objects': return len(json.loads(data)['objects'])==2048
    if name=='dataset': return len(data.decode().splitlines())==16385
    with tarfile.open(fileobj=io.BytesIO(data)) as tar:
        return tar.extractfile('vectors/localnet/v0/approved.tsv').read()==(ROOT/'vectors/localnet/v0/approved.tsv').read_bytes()


def run(output):
    output.mkdir(parents=True,exist_ok=False)
    report={'profile':'Q1_USEFUL_RESOURCE_V2_RESEARCH_ONLY','workloads':[],
            'claim':'Availability of useful static bytes; not reserved storage, hardware ownership or permissionless uniqueness.',
            'license':'Original synthetic fixtures under repository Apache-2.0; archive contains repository license and already-public Q1 vectors only.'}
    with tempfile.TemporaryDirectory(prefix='q1-useful-v2-') as tmp:
        for name,factory in [('objects',object_data),('dataset',dataset_data),('archive',archive_data)]:
            original=factory(); packed=pad(original); count=len(packed)//CHUNK
            chunks=[packed[i:i+CHUNK] for i in range(0,len(packed),CHUNK)]
            levels=tree(chunks); path=Path(tmp)/name; path.write_bytes(packed)
            m=manifest(secrets.token_hex(32),levels[-1][0].hex(),count,secrets.token_hex(32))
            samples=min(16,count)
            item={'name':name,'useful_bytes':len(original),'padded_bytes':len(packed),
                  'padding_bytes':len(packed)-len(original),'tree_hash_bytes':sum(map(len,levels))*32,
                  'samples':samples,'utility_check':utility(name,original),'controls':[],
                  'metadata':{'resource_identifier':hashlib.sha256(original).hexdigest(),
                              'size':len(original),'commitment':m,'provider_identity':'fixture-provider-1',
                              'availability_window':'ten fresh challenges per control during this finite run',
                              'service_terms':'Research retrieval only; no SLA, payment, mining reward or voting rights.',
                              'proof_history':'controls[].trials'}}
            # Readers have declared capabilities. This is not hostile OS isolation.
            missing=set(range(max(1,count//2)))
            controls=[('present',FileReader(path)),
                      ('partial_half',lambda i:b'' if i in missing else chunks[i]),
                      ('reconstructed',lambda i:pad(factory())[i*CHUNK:(i+1)*CHUNK]),
                      ('duplicate_identity',FileReader(path))]
            for mode,reader in controls:
                item['controls'].append(trial(mode,m,levels,reader,samples))
            del controls,reader,chunks,packed,original
            path.unlink()
            item['controls'].append(trial('deleted',m,levels,FileReader(path),samples))
            claims={}
            duplicate_claims=[]
            for identity in ('fixture-provider-1','fixture-provider-2'):
                root=m['root']
                duplicate_claims.append({'provider':identity,'duplicate_root':root in claims})
                claims.setdefault(root,identity)
            item['duplicate_claim_registry']=duplicate_claims
            item['duplicate_commitment_detected']=duplicate_claims[1]['duplicate_root']
            # Same-content roots detect the exact duplicate, but the duplicate
            # identity still passes every fresh proof on the same backing file.
            report['workloads'].append(item)
    report['source_sha256']={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in [Path(__file__),ROOT/'research/mining_delay_v1/lab.py',ROOT/'vectors/localnet/v0/approved.tsv']}
    (output/'results.json').write_bytes(canonical(report))
    (output/'SHA256SUMS').write_text(hashlib.sha256(canonical(report)).hexdigest()+'  results.json\n')
    print(json.dumps(report,indent=2))


def trial(mode,m,levels,reader,samples):
    v=Verifier(m); v.commit(m); trials=[]
    for _ in range(10):
        q=v.issue(samples); start=time.monotonic_ns(); raw=None
        try: raw=proof(m,q,levels,reader)
        except Rejected: pass
        built=time.monotonic_ns(); outcome='accepted'; fingerprint=None
        try: fingerprint=v.finish(raw)
        except Rejected as e: outcome=str(e)
        end=time.monotonic_ns()
        trials.append(dict(challenge=q,outcome=outcome,proof_hash=fingerprint,
                           proof_ns=built-start,verification_ns=end-built,latency_ns=end-start,
                           proof_bytes=len(raw) if raw else 0))
    try: full_recovery(m,reader); recovered=True
    except Rejected: recovered=False
    return dict(mode=mode,attempts=10,passed=sum(t['outcome']=='accepted' for t in trials),
                full_recovery=recovered,median_latency_ms=statistics.median(t['latency_ns'] for t in trials)/1e6,
                median_verify_ms=statistics.median(t['verification_ns'] for t in trials)/1e6,trials=trials)

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--output',type=Path,required=True);run(p.parse_args().output)
