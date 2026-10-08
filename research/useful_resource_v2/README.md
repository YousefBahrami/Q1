# Useful resource experiment v2

Research only, outside consensus, monetary issuance and service payment. Reuses
v1's committed indexed Merkle proof and fresh challenge verifier. No added Python
dependencies. From repository root:

```sh
python3 -m unittest discover -s research/useful_resource_v2 -p 'test_*.py'
python3 research/useful_resource_v2/run.py --output /tmp/q1-useful-resource-new
```

Use a new output directory. Three redistributable, non-sensitive workloads:
original static request objects; synthetic chunked CSV; deterministic archive of
the Apache license and already-public LOCALNET protocol vectors. No private
records, model weights or unlicensed third-party dataset. No NOTICE is present
in this source tree; none is invented or removed. Utility is checked by actual
parsing/recovery of the original bytes. This is application-test utility, not
proven customer demand or a running commercial storage service.

Commit all indexed 4096-byte chunks before issuing fresh challenges. Original
byte length and content SHA-256 remain metadata; zero padding is reported
separately and never sold as useful bytes. A finite ten-round audit uses at most
16 sampled chunks and the v1 one-second response window; the archive has eight
chunks, so its challenge is a full audit. CPU timing is local monotonic wall
time. Merkle levels remain in memory; file reads may be OS-cached. Tree byte
count excludes Python object/allocator overhead. Do not infer HDD throughput.

For every workload the runner tests present, half-retained, independently
reconstructed, second-identity/same-file and deleted-file readers, including
full recovery checks. Reconstruction is deliberately allowed to use public
fixtures/code. A root-indexed claim registry detects an exact repeat, but two
identities still generate valid responses from one backing resource. This is
not a trusted physical resource map and does not solve permissionless uniqueness.

The trusted single-process runner constrains reader capabilities, not a hostile
operating system. It retains commitment hashes; there is no assertion that a
malicious remote prover erased every copy. Timings are observations, not
universal thresholds or guaranteed performance. Results include fresh challenge
records, response hashes/lengths, latency/verification timing, recovery outcomes,
metadata and source hashes. Regeneration reruns the experiment with new entropy;
it does not reproduce timings or proof bytes exactly.

See [results and Sybil analysis](../../docs/research/Q1_USEFUL_RESOURCE_V2_RESULTS.md).
