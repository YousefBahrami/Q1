# Resource uniqueness — concurrent audits v0

2026-10-06. Experimental counterexamples, no selected issuance/reward formula.
Extends [uniqueness v0](Q1_RESOURCE_UNIQUENESS_V0.md) without changing its verifier
or frozen evidence. The same underlying economic resource must not gain additional
expected reward merely by appearing under more identities: **still unsolved**.

## Experiment and observations

Use the same 128 KiB public help fixture, eight challenged chunks per claim and
unchanged one-second verifier window. Release 1/2/4/8 claimant tasks together behind
a barrier. Their reads share one lock/queue with a synthetic 2ms service delay.
This is a controlled workload, not measured device throughput or a saturation test.
Labels are synthetic identifiers, not authenticated hardware owners.

| Model | Actual capability exercised | Accepted across cohorts |
|---|---|---|
| Shared storage | Multiple concurrent claims read one backing file | 1, 2, 4, 8 |
| Duplicate commitments | Repeated identical content root under different labels/sessions | 1, 2, 4, 8 |
| Remote proof outsourcing | A separate helper process supplies chunks over loopback TCP | 1, 2, 4, 8 |
| Resource leasing | Several claims access the same file; no exclusive lease enforcement or legal contract assumed | 1, 2, 4, 8 |
| Reconstruction | Reconstruct requested chunks from the compressed fixture | 1, 2, 4, 8 |
| Partial retention | Half retained; missing chunks fetched from that helper | 1, 2, 4, 8 |

24 cohorts, 90 responses accepted. The deterministic control requests all chunks
from the partial holder with helper unavailable: MISSING_DATA rejection. Compression
and retained-payload sizes are recorded; neither includes executable/index/process
overhead or establishes physical storage consumption.

The helper is on the **same host**, not a WAN measurement. Concurrency does not
change that fact. Each new claimant performs more audit work (8 reads); the data
backing remains shared, but no zero-cost or fixed-total-work comparison is claimed.
This run stays below the synthetic service bottleneck. Saturation could reduce
successful responses and also reject honest shared/remote providers; it cannot by
itself prove independent economic ownership.

## Expected-reward counterexample, conditional on a hypothetical rule

Assume eight honest eligible tickets and an illustrative one-winner uniform
lottery over all accepted identity tickets. The shared-resource claimant's chance
is `k/(8+k)`: k=1 → 1/9; k=2 → 1/5; k=4 → 1/3; k=8 → 1/2.
This is exact arithmetic under the stated artificial rule, not Q1's selected
economics, observed monetary reward or a universal economic impossibility proof.
Accepted audits can therefore coexist with increasing expected reward for splitting
identities under a naive eligibility rule.

Counting one independently authorized, one-use customer job would yield one credit
for that job despite labels. That assumes a trustworthy authorization/demand source
and a shared accounting scope; neither is supplied by these proofs. Self-issued job
labels return the 1/2/4/8 inflation. Multiple genuine customer jobs can deserve more
payment even on one efficient resource; suppressing them solely by content root
would penalize useful service. Service payment, any future protocol subsidy and
consensus authority remain distinct.

## Reproduction and next research

```sh
python3 research/resource_uniqueness_v0/concurrent_audits.py --output /tmp/q1-concurrent-new
python3 research/resource_uniqueness_v0/concurrent_audits.py --replay docs/reports/research/resource-concurrent-v0-2026-10-06
```

[Frozen report](../reports/research/resource-concurrent-v0-2026-10-06/results.json)
and [checksum](../reports/research/resource-concurrent-v0-2026-10-06/SHA256SUMS)
record all challenges, manifests and accepted-proof hashes. Replay checks source
hashes, canonical proof bytes and the illustrative accounting arithmetic, not
historical timing, physical retention or remote location. It is included in the
local full-check command. The new tool uses only the standard library.

Next: a bounded job/epoch simulator with adversarial self-demand, colluding
beneficiaries, legitimate independent jobs, retries and duplicate claims. Compare
fixed-budget subsidies with actual customer-funded service payment and explicitly
account for marginal work. Do not select a monetary policy or percentages from
these results. No storage-quantity voting weight, hardware serial/TPM default,
mining reward, real issuance or permissionless resource oracle is introduced.
