# Q1 resource evidence v1 laboratory

RESEARCH ONLY. No consensus integration, reward, issuance, producer selection
or proof of physical HDD. Uses Python standard library. The trusted harness
assigns unpredictable bytes and pins their root before a fresh verifier nonce.
It measures access to selected committed data; neither disk type nor unique
ownership is authenticated. See the [design](../../docs/research/Q1_MINING_DELAY_EXPERIMENT_V1_DESIGN.md).

From repository root:

```sh
python3 -m unittest discover -s research/mining_delay_v1 -p 'test_*.py'
python3 research/mining_delay_v1/run.py --output /tmp/q1-v1-new-run
python3 research/mining_delay_v1/replay.py docs/reports/research/mining-delay-v1-2026-10-05
```

Use a nonexistent output directory. Live runs use fresh OS randomness, so roots
and outcomes need not match another run. Recorded positive proofs replay exactly;
replay does not prove freshness. Dataset creation writes 21 MiB per suite, then
reads existing temporary files. No cache flushing, device writes, purchases or
long-running process. Cold access and physical HDD remain unmeasured.

The verifier enforces a predeclared one-second functional response window;
this is not an honest-path latency calibration or unavoidable delay proof.
Every issued challenge produces one success/failure record; no retry exclusion.
Complete recovery runs before and after each control. Proof parsing is bounded,
canonical and context-bound. A trusted single-process harness controls reader
capabilities; it is not a hostile-process/OS boundary or secure memory erasure.
The deleted-file reader receives no corpus bytes; allocator remnants and
previous public sample bytes are not evidence of inaccessible physical memory.

The simulator uses 72 leaf hashes from the actual 1 MiB committed corpus:
64 assigned to one simulated owner, eight to small owners. Presentation
commitments hash those same atomic leaf IDs under one/many keys, partitions
and duplicates. A trusted allocation oracle identifies aliases. Unit credit
is rounded before key grouping. Zero split gain under that oracle is not
permissionless Sybil resistance; an attacker manufacturing new oracle-approved
unit identities is outside the proven result. There is no economic reward.
