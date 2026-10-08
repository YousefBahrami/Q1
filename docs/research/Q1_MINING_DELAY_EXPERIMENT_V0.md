# Q1 mining / delay experiment v0

Date: 2026-10-04. Status: isolated prototype and measured research; no Mainnet
rule, LOCALNET modification, real reward, public-network deployment or security
approval. Human authority retains HDD for testing without making it Mainnet's
security resource. [Historical recovery](Q1_MINING_AND_ISSUANCE_RECOVERY.md)
and [HDD draft](../07_HDD_LAB_MODULE.md) are the design basis.

## Accepted negative result — 2026-10-05 design checkpoint

Experiment v0 did **not** prove durable storage, physical HDD possession,
HDD-bound work, Sybil resistance or economically fair mining. It did demonstrate
canonical commitment/evidence construction, deterministic verification,
reproducibility and weaknesses requiring redesign before consensus/reward use.
Public-seed reconstruction after file deletion and reward gains from identity
splitting are protocol-design findings, not minor performance issues.

The historical measurements and source/checksum artifacts below are unchanged.
Their earlier proposed hardware follow-up is superseded by
[v1 design first](Q1_MINING_DELAY_EXPERIMENT_V1_DESIGN.md) and the
[Sybil analysis](Q1_SYBIL_AND_RESOURCE_IDENTITY_ANALYSIS.md), not a larger-HDD
rerun. No v1 measurements or production mining result are claimed.

## Recovered HDD role

The draft combines participant-bound dataset commitments, challenge-selected
data access, read/transform/commit work and optional input to the delay engine.
Telemetry-only is the default recommendation; contribution-based eligibility or
rewards require later explicit activation. It does not establish a capacity
proof, proof of continuous storage, mechanical-device proof or trusted elapsed
time. Capacity, sustained storage, latency and resource cost were hypotheses to
measure, not properties already obtained by the Merkle proof.

The implemented prototype follows that design's random-read workload and
membership-proof strategy. HDD remains in the research program. Finding a cheap
substitution falsifies a claim about this construction, not every possible use
of a hard drive; no replacement consensus mechanism is invented here.

## Three hypotheses and falsifiable boundaries

| Question | H1 — HDD as storage commitment | H2 — HDD as delay/resource evidence | H3 — HDD as miner eligibility/selection input |
|---|---|---|---|
| Hypothesis | Root/challenge evidence can demonstrate a meaningful retained storage contribution | Challenge response can impose measurable unavoidable resource/delay cost | Verifiable resource contribution could support bounded fair participation |
| Resource consumed/locked | File bytes at setup and read bandwidth; root alone locks no capacity over time | CPU, bytes accessed and observed latency; no time lock or mandatory mechanical seek established | Assumed contribution units plus operating cost; identities are not scarce resources |
| Evidence | Dataset manifest/root, selected chunks, paths and response root | Same context-bound transcript; time/IO are separate telemetry | Valid contribution record would be an experimental input only; current code gives no eligibility/vote/selection rights |
| Verification | Recompute selected deterministic chunks, membership, selection and response commitment from pinned manifest/context | Same checks; no verifier can infer genuine elapsed time or disk type from this transcript | Simulate allocations using declared accepted units/owner mappings; admission/selection proof is not implemented |
| Attack cost | Generate tree once, retain small hash metadata, regenerate requested chunks; costs measured below | Precompute/cache data/tree, use RAM/SSD or outsource; no universal economic cost lower bound | Split an operator into identities; per-identity caps/equality can be evaded; identity cost assumed zero in attack scenario |
| Precomputation | Dataset and tree intentionally public/deterministic; full precomputation possible | Fixed benchmark challenges also precomputable; unpredictable live challenges would prevent only advance response selection, not dataset precomputation | Reuse/copy/regenerate resources across identities is an open threat; record uniqueness is not economic uniqueness |
| SSD/GPU/ASIC advantage | SSD/RAM substitution succeeds; GPU/ASIC advantage not measured | Faster memory/compute may beat HDD; no proven fairness or sequential-work bound | Hardware weight can concentrate power if later adopted; capped/sqrt weights need credible operator identity to resist splitting |
| Sybil reduction | None demonstrated | None demonstrated | None demonstrated; simulation explicitly exposes cap/equality evasion |
| Ordinary-hardware feasibility | 1/4/16 MiB dataset experiment runs on current equipment; not a realistic capacity contest | Low-cost microexperiment works; formal VDF and physical-HDD benchmarks absent | Simulation is inexpensive; secure public admission is a separate unsolved cost |
| Metrics | Bytes retained, metadata fraction, sampled coverage, proof size, regeneration cost, stale/corrupt rejection | Setup/proof/verify wall and CPU time, logical/physical IO distinction, cache mode, energy/wear when measurable | Top-owner share, small-owner reward, Gini/HHI, identity splitting, supply/epoch and hardware scaling |
| Current finding | Data-access consistency works; permanent full storage is not established | Substitutions verify; no HDD or elapsed-time security claim supported | Simulation only; no justified authority grant |
| Next test | Repeat on an available HDD with declared cache/setup policy; measure data deletion/regeneration across fresh challenges | Compare HDD/SSD/retained metadata under the same workload; separate verifier overhead from IO, then obtain construction review | Sweep hardware/identity costs, rejection rates and sharing; no consensus weighting until a separately approved design exists |

No hypothesis passes a Mainnet security gate on these results. Retain HDD as an
optional experiment and the negative observations as evidence. A later design
would need to demonstrate unavoidable scarce cost under the cheapest known
substitution, verifier soundness/budget and Sybil/fairness assumptions; merely
increasing dataset size would not repair the demonstrated public-seed attack.

## Implementation and isolation

The [module](../../research/mining_delay_v0/README.md) uses Python's standard
library, dedicated temporary files, strict size limits, lab-specific hash framing
and reproducible evidence. Its exact encoding/expansion choices are documented
as laboratory details, not additions to the normative domain registry.

There is no import into the Rust workspace or production execution path. LOCALNET
still accepts only its scoped NONE witness, fixed roles and zero issuance/payout.
The regular check script adds the small independent unit tests; it does not run
hardware benchmarks or activate the module in a node. No shared consensus keys,
automatic mining service or hidden reward state exists.

## Measured results

Evidence: [raw results](../reports/research/mining-delay-v0-2026-10-04/results.json),
[sample evidence](../reports/research/mining-delay-v0-2026-10-04/evidence.json),
[manifest](../reports/research/mining-delay-v0-2026-10-04/manifest.json),
[expected context](../reports/research/mining-delay-v0-2026-10-04/context.json) and
[checksums](../reports/research/mining-delay-v0-2026-10-04/SHA256SUMS).

Host observation: macOS Darwin 21.6.0, x86_64, Python 3.13.3, 16 GiB RAM;
read-only OS disk inspection reported APFS / solid-state / PCI-Express. This is
untrusted host metadata, not device attestation. No physical-HDD, GPU, ASIC,
remote-storage, cold-cache, energy or wear measurement was performed. About
40.9 GiB free scratch capacity was observed before the run. No device identifiers,
serials, personal paths or names are in the report.

Three trials per size/mode, 64 samples of 4096 bytes (256 KiB logical bytes per
proof); dataset generation happens before the timed proof. Times below are
median milliseconds including proof serialization; setup time is separate.

| Dataset | Setup ms | File proof ms | RAM proof ms | Regenerated proof ms | File-mode verify ms | Evidence bytes | Retained Merkle digest bytes |
|---|---:|---:|---:|---:|---:|---:|---:|
| Small, 1 MiB | 16.167 | 7.510 | 7.596 | 8.256 | 9.981 | 565969 | 16352 |
| Medium, 4 MiB | 62.523 | 6.731 | 6.625 | 7.862 | 9.710 | 574572 | 65504 |
| Large, 16 MiB | 273.543 | 9.398 | 8.378 | 10.651 | 13.002 | 583196 | 262112 |

All 27 transcripts verified. For each size, file/RAM/regeneration produced
identical canonical evidence. The dataset file was deleted before regeneration;
only public seed, identity and Merkle hash metadata were required by that mode.
At 16 MiB, raw digest retention is about 1.56% of the original dataset, plus
Python bookkeeping. The attack still pays initial generation/tree cost and
per-query recomputation; it does not need continued retention of the full file.
This demonstrates a time/storage tradeoff, not zero-cost proof forgery.

Proof sampling covered 55/64/63 distinct chunks for the three sizes; repeated
indices are explicitly permitted. Membership checking remains sampled dataset
coverage. Results do not prove full capacity or sustained physical storage.

Peak process RSS was approximately 46.52 MiB. Exactly 21 MiB of dataset bytes
were generated in the recorded run; files were removed. RAM preload and initial
tree computation have costs outside per-proof measurements. OS block counters
are in raw results; physical IO bytes are deliberately null. Buffered reads
immediately after writes can be served from cache. Differences of a millisecond
over three trials cannot establish hardware fairness, disk speed or throughput.
Here verification even costs more than proof generation in several cases; no
cheap-verification VDF claim follows.

## Economic simulation — artificial units only

The simulator compares equal, proportional, capped and diminishing (square-root)
weights, with both a fixed epoch budget and an open issuance-per-weight schedule:
4 models × 4 resource/identity scenarios × 2 schedules = **32 scenarios**.
Parameters and exact rounding are documented in the module README and raw report.
No measured MiB/time is converted into validated economic contribution.

For eight one-unit operators plus one 64-unit operator, fixed budget results:

| Model | Large owner's reward share | Each small owner's share | Large owner after splitting its 64 units into 64 identities |
|---|---:|---:|---:|
| Equal per accepted identity-record | 11.11% | 11.11% | 88.89% |
| Proportional | 88.89% | 1.39% | 88.89% |
| Cap at 4 units per identity | 33.33% | 8.33% | 88.89% |
| Square-root weight | 50.00% | 6.25% | 88.89% |

Small participation remains nonzero in these chosen scenarios; that does not
prove it covers electricity/hardware/time costs. Larger hardware can dominate
proportional rewards, and arbitrary identity splitting defeats per-identity
redistribution. Simulation aggregates by the known true owner; real Q1 currently
has no verified mapping from keys to economic operators. Gini and HHI for all
scenarios are retained in the report.

Fixed budget creates 7200 artificial units per logical epoch and reaches 820000
after 100 epochs from 100000 regardless of weight model. In the unsplit 64-unit
scenario, open-schedule issuance is 900/7200/1200/1600 per epoch for equal/
proportional/capped/sqrt respectively, reaching 190000/820000/220000/260000.
After identity splitting, all four open models create 7200 per epoch in that
scenario. Supply growth therefore depends on both reward weighting **and** the
separate issuance cap; a redistribution cap alone does not cap emissions.
Epochs are not blocks, seconds or a proposed Q1 emission schedule.

No price, monetary viability, founder percentage, supply cap, treasury share or
security budget is chosen. The experiment strengthens the reason to keep service
cash separate from simulated rewards and possible future coin holdings.

## Security boundary and next evidence

Current LOCALNET safety comes from scoped role authorization, signatures,
canonical validation, deterministic execution, certificates and durable vote
reservations under its local assumptions. These do not establish a general
Byzantine public-network claim. The experimental miner supplies no vote, mint,
eligibility score, producer preference or finality override.

For three voters and quorum two, two quorums can share only one voter. If that
voter equivocates, signature counting alone cannot exclude conflicting
certificates. Role count does not choose a public fault model; public quorum,
locking, round change, transport, admission and resource caps remain separate
decisions. [Testnet role/deployment refinement](Q1_PUBLIC_TESTNET_V0_PLAN.md)
preserves this boundary and does not expose LOCALNET ports.

Next useful evidence: an existing HDD volume available for bounded experiments;
additional challenge seeds/retention conditions and raw-device-independent IO
measurement; and simulation sensitivity to real costs/identity assumptions.
No purchase is authorized. Any larger workload, destructive write test, change
in consensus authority or monetary issuance requires a different explicit scope.

## Verification record

Completion review: 2026-10-05; measurements and source research are dated above.

- `python3 -m unittest discover -s research/mining_delay_v0 -p 'test_*.py' -v`:
  13 tests passed, including corrupted/forged proofs, wrong context, size/schema
  bounds, no overwrite, regeneration substitution and reward conservation/Sybil
  comparisons. These are behavioral tests, not a cryptographic security proof.
- A second complete run in a fresh temporary directory reproduced every size's
  manifest/context/evidence hashes and all simulation results. Recorded sample
  files matched byte-for-byte; timings intentionally differed. Two benchmark
  runs together generated 42 MiB, plus small temporary unit-test datasets.
- Separate-process verification of the saved fixture, all artifact checksums
  and all recorded source hashes passed. The verifier uses the same code,
  so independent implementation/cryptographic review remains outstanding.
- `python3 scripts/check_all.py` passed: Rust format, Clippy, workspace tests,
  build/Rustdoc, historical conformance, 23 protocol vectors, 13 frozen local
  vectors, actual four-process LOCALNET acceptance, README wallet/CLI flows,
  the 13 new lab tests and blocking documentation checks. Historical document
  advisories remain; no remote CI was executed by this checkpoint.
- LOCALNET again ended at height 4, sender 956, recipient 40, pool 4, supply
  1000 and StateRoot
  `b7ec7d47f4cf37d029e74bab02d8bc2aab191f5a5733f92ac4310d919183b087`.
  No changes exist in consensus crates, genesis, vectors, Cargo dependencies,
  LICENSE or NOTICE. Test harnesses cleaned up their owned processes.

Untested: physical HDD, SSD-versus-HDD comparison, GPU/ASIC, cold cache, external
network observation, mining eligibility, paid economics, public testnet, Mainnet
and monetary distribution. No successfully checked transcript or simulation
reclassifies any of these as complete.
