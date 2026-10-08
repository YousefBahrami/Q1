# Useful resource v2 — measured results and permissionless limits

Date: 2026-10-05. PUBLIC technical report. No issuance, marketplace or authority
change. [Runner](../../research/useful_resource_v2/README.md),
[frozen evidence](../reports/research/useful-resource-v2-2026-10-05/results.json),
[checksum](../reports/research/useful-resource-v2-2026-10-05/SHA256SUMS).

## Results

Ten fresh challenges for each of five controls across three workloads: 150 trials.
Each workload: retained 10/10; reconstructed 10/10; duplicate identity using one
file 10/10; half retention 0/10; deleted reader 0/10. Full recovery succeeds for
retained/reconstructed/duplicate and fails for partial/deleted. These measured
partial failures are not a universal no-cheating guarantee: sampling can miss
missing chunks. v1's partial-retention counterexample remains relevant.

| Workload | Useful bytes | Padded bytes | Merkle hash bytes | Retained median response / verification ms | Reconstructed median response ms |
|---|---:|---:|---:|---:|---:|
| Static request objects | 343937 | 524288 | 8160 | 5.729 / 3.201 | 124.917 |
| Chunked synthetic dataset | 483541 | 524288 | 8160 | 5.003 / 2.972 | 167.946 |
| License + public vector archive | 30720 | 32768 | 480 | 2.754 / 1.418 | 6.052 |

Commitment is an indexed SHA-256 Merkle root bound to the v1 manifest. Challenges
are fresh unpredictable nonces issued after commitment, with deterministic sample
indices and one-use verification; responses contain indexed chunks and paths.
Sixteen indices are sampled for objects/dataset; all eight archive chunks are
requested. Padding overhead is 180351, 40747 and 2048 bytes respectively. Hash
storage excludes runtime objects, original generator/source and filesystem
metadata. Proof byte counts and every individual timing are in the evidence.
Response includes proof construction plus verification. No cold cache, physical
HDD, concurrent customer throughput, network or hostile-process measurements.

**Finding:** the bytes have verifiable application/reproduction utility and can
be recovered. The proof demonstrates access within this experiment's window.
It cannot distinguish stored data from fast reconstruction or remote retrieval.
Useful public content is naturally copyable: refusing valid reconstruction would
require a separately justified service/resource rule. No evidence yet supports
paying per physical disk, identity or presumed exclusive capacity.

## Resource Market alignment

These immutable resources can be future service offers, conditional on approved
retrieval and fulfillment rules. Minimal metadata is recorded for each workload:
resource ID (original content hash), original size, manifest/root/chunk layout,
finite availability window, provider identity, service terms and proof history.
Versioned terms should also identify retrieval endpoint, encoding, permitted
reuse, retention period, failure handling and measurement policy before a real
service. Provider key is an attribution claim, not ownership proof. No marketplace,
customer billing, escrow, SLA or dispute process is implemented.

Keep three independent ledgers of meaning: resource/availability reward (future
protocol decision); customer service payment (fulfilled customer request); and
consensus authority (approved membership/selection). Neither content bytes nor
service income grants votes. The locked Q1 useful-resource thesis remains intact.

## Permissionless Sybil analysis — no resource oracle

Costs below are qualitative, not measured economic deterrence. There is no
trusted mapping from roots or keys to physical disks. Equal roots identify equal
committed bytes, not independent capacity. Unknown peer ownership stays unknown.

| Candidate | Prevents / detects | Does NOT prevent | Attacker cost | Verification cost | Centralization risk |
|---|---|---|---|---|---|
| Exact commitment index | Repeated identical root in one declared reward scope | Reordered, encoded, salted or disjoint claims on shared storage; cross-registry double claims | Hash/re-encode or choose another root | Root lookup + agreed registry state | Global index operators and censorship; content dedup may penalize useful redundancy |
| Simultaneous correlated challenge windows | May expose an overloaded shared reader through missed deadlines | RAM/cache, sufficiently fast shared storage, distributed helpers; no ownership inference | Peak bandwidth/IO and scheduling | Concurrent fresh audits, clocks and bandwidth | Favors nearby high-bandwidth operators; auditor scheduling power |
| Overlap-aware content/chunk accounting | Identical chunks under agreed canonical encoding can avoid duplicate *content* credit | Compression, recoding, undeclared overlap or legitimate replicas; physical uniqueness | Transform data or use unique content | Chunk index or overlap proofs, potentially linear metadata | Large indexing services; information leakage and canonical-format gatekeeping |
| Shared-storage outsourcing audits | Fresh retrieval probes detect unavailable service | Many identities sharing a competent backend | Rent storage/egress; amortize same object | Repeated probes and retained evidence | Hyperscaler economies; location/latency bias |
| Many keys on one resource: reward by verified service event | Stops paying merely for creating keys if each unique fulfilled job is counted once | Fake demand/colluding customers, shared backend, fabricated uniqueness | Creating/funding apparent demand; serving aggregate load | Unique-job authorization, replay prevention and fulfillment accounting | Customer/platform gatekeepers; rules not yet specified |
| Remote proof outsourcing | Tight fresh challenges can reject helpers too slow for declared service window | Fast remote helpers or prepositioned replicas | Network/compute/egress | Nonce issuance, deadline checks, proof verification | Nearby datacenters advantaged; not a distance proof |
| Resource leasing | Explicit service leases clarify who owes availability; avoid overlapping paid leases in one contract scope | Undisclosed subleases and duplicate contracts elsewhere | Rental/collateral if separately approved; no collateral assumed here | Lease history + service proofs + dispute rules | Large lessors and adjudicators; legal enforcement dependency |

The v2 exact-root registry flags the second claim for all three workloads while
its proofs still pass. The other proposals are analysis, not tested mechanisms.
Do not turn these conditional costs into a claim that duplicating identities is
unprofitable. No KYC or stake default is introduced. A credible next experiment
uses independent prover/verifier hosts, simultaneous challenges, one backend
serving many keys, remote helpers and distinct content encodings; report false
rejections of legitimate redundancy alongside cheating successes.

## Validation and unresolved work

Three workload tests passed. The first run failed because the archive factory
referenced a nonexistent NOTICE; it now archives existing licensed public files
only. Full-system verification is recorded in the phase report. v0 and v1 are
retained unchanged. No reward, service payment or mining authority was activated.
