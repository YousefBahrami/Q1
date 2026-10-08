# Q1 resource evidence v1 — measured results

Date: 2026-10-05. PUBLIC technical research. Limited implementation authorized
by the human after design review. No consensus, issuance, economic reward or
producer-selection integration. [Implementation](../../research/mining_delay_v1/README.md),
[design](Q1_MINING_DELAY_EXPERIMENT_V1_DESIGN.md) and
[machine-readable evidence](../reports/research/mining-delay-v1-2026-10-05/results.json).

## What changed and what the measurement means

Independent OS-random corpus bytes replace v0's publicly regenerable data.
The trusted verifier pins the assigned root before accepting the prover's
commitment. Only then does it issue a fresh random nonce and derive 64 distinct
indices. Canonical indexed Merkle proofs authenticate responses; a predeclared
one-second verifier-monotonic window rejects late completion. Repeated trials
and complete recovery before/after test availability separately.

These are capabilities within one trusted process, not mutually hostile OS
processes. In particular, deleting a file is not proof that no memory/backup
exists. RAM-retained bytes explicitly pass. The deleted-file control removes
the file and intentional full-corpus references, retains tree hashes and uses
a reader with no corpus access; memory sanitization was not measured. Positive
sample fixtures retain only disclosed selected chunks for replay. No external
storage helper is used or excluded by a cryptographic locality proof.

Each cell has only ten trials. Zero successes among the tested substitutes
is evidence against those specific attacks under these conditions, **not** a
negligible-probability proof against all adversaries. Response-time measurement
includes proof construction and verification overhead; it does not measure
mechanical access time, sequential work or physical disk ownership. The fixed
window is a functional bound, not a calibrated cross-network deadline.

## Observations

| MiB | Control | Valid responses | Median end-to-end ms | Full recovery before / after |
|---|---|---:|---:|---|
| 1 | file_present | 10/10 | 18.731 | True / True |
| 1 | warm_cache | 10/10 | 26.042 | True / True |
| 1 | ram_cached | 10/10 | 21.351 | True / True |
| 1 | partial_99pct | 3/10 | 0.630 | False / False |
| 1 | wrong_data | 0/10 | 12.571 | False / False |
| 1 | generated_substitute | 0/10 | 15.683 | False / False |
| 1 | file_deleted | 0/10 | 0.512 | False / False |
| 4 | file_present | 10/10 | 28.111 | True / True |
| 4 | warm_cache | 10/10 | 27.131 | True / True |
| 4 | ram_cached | 10/10 | 19.594 | True / True |
| 4 | partial_99pct | 3/10 | 0.638 | False / False |
| 4 | wrong_data | 0/10 | 12.829 | False / False |
| 4 | generated_substitute | 0/10 | 11.587 | False / False |
| 4 | file_deleted | 0/10 | 0.343 | False / False |
| 16 | file_present | 10/10 | 20.288 | True / True |
| 16 | warm_cache | 10/10 | 26.285 | True / True |
| 16 | ram_cached | 10/10 | 17.721 | True / True |
| 16 | partial_99pct | 4/10 | 0.588 | False / False |
| 16 | wrong_data | 0/10 | 11.435 | False / False |
| 16 | generated_substitute | 0/10 | 18.357 | False / False |
| 16 | file_deleted | 0/10 | 0.745 | False / False |

Committed file, warm reads and retained RAM passed all 30 trials per control
across the three sizes. Deleted-file access, public-seed generated substitutes
and intentionally wrong bytes passed zero of 30 each. Partial retention removed
`floor(chunk_count/100)` chunks: this is approximately 99% retention, not exactly
99% at every size. Sampling sometimes passed while both complete recovery
checks failed. That is an expected limitation, not a success to hide.

Cold access: **NOT MEASURED**; no reliable physical-read/cache-eviction evidence.
HDD-specific comparison: **NOT MEASURED**. Warm/file/RAM are separately labeled;
no speed ranking establishes a device type. The suite wrote 21 MiB of temporary
corpus data, then read it; no raw-device access, global cache flush, disk fill
or equipment purchase. Earlier development runs are separate runs, not included
in this suite's write count. Ten observations do not establish performance tails.

## Reproducibility and negative tests

The result records all 210 issued challenges, outcomes, durations and successful
proof hashes. Three complete positive proof fixtures (one per size) are retained
for deterministic independent replay against pinned manifests and challenges.
It does not retain all response bodies. Fresh runs choose new bytes/nonces and
can have different roots and partial-retention outcomes. Replaying fixed
fixtures does not prove freshness. SHA256SUMS covers both artifacts; source
hashes identify the Python files present during the measured run. The replay
utility was added afterwards and is tested separately.

Nine unit tests exercise commitment ordering, assignment substitution, replay,
challenge context, wrong/missing/partial data, path/index/length tampering,
canonical/parser limits, timeout, missing responses and Sybil accounting.
Reconstruction from a public seed no longer supplies the assigned bytes.
Future adversarial work still includes isolated verifier/prover hosts, online
outsourcing, adaptive partial storage and calibrated bandwidth contention.

## Quantitative Sybil result

The same 64 atomic chunk leaf hashes from the real committed corpus are
represented by one, four or 64 keys, split presentation commitments and repeated
identical commitments. Eight other atomic units belong to eight small simulated
owners. The allocation oracle maps every presentation back to its original
units before attribution, with integer rounding per stable unit. All 15
scenario/budget combinations (budgets 7, 71, 7200) produced **0% extra credit**
for the large owner relative to one key. At budget 7200 it receives 6400 in every
representation. A naive identity-share comparator rises from 1/9 to 64/72:
11.11% to 88.89%, or an eightfold share / 700% relative gain.

This closes the tested accounting loophole under a **trusted assignment
oracle**. It does not establish permissionless uniqueness of physical resources,
independent owners or storage replicas. These are simulated credits, never Q1
coins. The [Sybil analysis](Q1_SYBIL_AND_RESOURCE_IDENTITY_ANALYSIS.md) remains
applicable: root count cannot establish resource count.

## Alignment with the locked Q1 economic thesis

The [approved thesis](../strategy/Q1_LOCKED_ECONOMIC_THESIS.md) is preserved.
Current random audit data provides research information, not useful customer
storage or demonstrated market demand. Paying for artificial data alone would
not satisfy the desired useful-resource/service incentive loop.

Existing storage capacity could later serve the Resource Market, but occupied
space must not be simultaneously promised to incompatible jobs. A useful-data
follow-up needs authorized customer data or encrypted test objects, retention
and repair obligations, privacy-safe sampling, measurable delivery and dispute
rules. Service rewards and any future mining subsidy remain distinct.

A future provider may earn for verified useful service without acquiring more
consensus/governance votes. This code implements that separation by granting
**no authority at all**. The evidence format might later inform service
commitments/bonds, but escrow, slashing, dispute attribution and settlement are
not implemented or justified by sampling alone. AI may assist matching or job
scheduling; deterministic verification alone decides validity. This experiment
supports correctness/research and future participant tooling, not a sale or a
claim that all Mainnet gates are closed.
