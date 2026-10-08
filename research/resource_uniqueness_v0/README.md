# Resource uniqueness v0

A fixed 128 KiB useful static-help fixture, nine attacker capability models and
1/2/4/8 claimant labels. No larger-dataset benchmark, consensus change or issuance.

```sh
python3 -m unittest discover -s research/resource_uniqueness_v0 -p 'test_*.py'
python3 research/resource_uniqueness_v0/experiment.py --output /tmp/q1-unique-new
python3 research/resource_uniqueness_v0/replay.py /tmp/q1-unique-new
```

Uses standard-library Python and the unchanged v1/v2 proof verifier. A separate
helper process serves chunks over loopback TCP; this is actual process/TCP
outsourcing, not a measured remote machine. Claimed keys are distinct labels in
the resource manifest, not a newly implemented identity signature scheme. The
existing proof verifier does not authenticate physical ownership or enforce an
exclusive capacity reservation. Scoring takes visible claim metadata, not a
trusted resource-to-device map. The shared authorized-job score is an explicitly
conditional thought experiment: job authorization/demand is assumed, not built.
Self-issued jobs are tested separately and reproduce identity inflation.

Three controls distinguish honest access from ownership: compression/dedup reduce
stored payload without invalidating content; unavailable data is rejected; a
response delayed beyond the existing one-second window is rejected. Sampling is
sequential, not concurrent overload testing. Allocator/Merkle/index overhead and
all other process memory must not be mistaken for measured disk capacity.
No hostile-process isolation, continuous retention or physical device attestation.

Frozen evidence records each manifest/challenge/hash and scores. Replay constructs
and verifies all accepted proof bytes from the public fixture; it does not verify
historical latency, remote location or the claimed physical setup. New runs use
fresh challenges and may have different helper counts/timings. No provider/customer
records are used. See [findings](../../docs/research/Q1_RESOURCE_UNIQUENESS_V0.md).

Concurrent follow-up is in `concurrent_audits.py`: six models, 1/2/4/8 labels,
shared simulated service queue and unchanged proof verification. See
[concurrent findings](../../docs/research/Q1_RESOURCE_UNIQUENESS_CONCURRENT_V0.md).
The fixed source/checksum/proof replay runs in `scripts/check_all.py`; elapsed
performance and physical resource uniqueness are not established by replay.
