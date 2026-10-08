# Q1 resource uniqueness v0 — counterexamples and accounting candidates

2026-10-06. Research, not monetary policy. Locked strategy unchanged: useful
resources/services, future service assurance/escrow and machine/AI settlement;
deterministic consensus independent of AI judgment. Resource wealth does not
automatically grant authority. **Permissionless economic-resource uniqueness is
not solved.** No hardware identity, stake or KYC admission rule is introduced.

[Experiment](../../research/resource_uniqueness_v0/README.md),
[frozen measurements](../reports/research/resource-uniqueness-v0-2026-10-06/results.json),
[checksum](../reports/research/resource-uniqueness-v0-2026-10-06/SHA256SUMS).
The prior [v2 findings](Q1_USEFUL_RESOURCE_V2_RESULTS.md) remain valid.

## What was tested

Use one fixed 128 KiB immutable help-page fixture, not a larger random blob. Nine
capability classes, each at 1/2/4/8 claimant labels, produce 135 sampled access
responses. The existing v1/v2 verifier accepts all 135 in the recorded run. Missing
response and a deliberately late response are rejected. Manifests/challenges are
recorded; replay regenerates and verifies the 135 accepted canonical proofs.
Claimant labels represent separate keys/accounts in the accounting attack model;
the resource proof itself has no identity-signature or ownership check. Adding a
signature would bind the claim to a key but would not prevent sharing its backing
bytes. This experiment does not claim to have implemented such signatures.

A separate process actually serves requested chunks over loopback TCP. It is a
helper-access counterexample, not evidence of WAN latency or independent physical
hosts. Sequential challenges do not measure saturation or concurrent availability.
A single file and explicitly shared readers model one backing resource; its disk
serial, device count and physical placement are not attested. Capability separation
is declared by the harness, not enforced against a hostile OS/prover.

The logical fixture contains four distinct 4096-byte pages repeated across 32
positions. Its deduplicated payload is 16384 bytes plus 32 references; its compressed
payload is 468 bytes plus decompressor/code. Merkle/index and runtime memory are
additional, not zero. Both readers answer valid challenges. These are counterexamples
to equating logical content size with occupied physical capacity, not a performance
claim. Layout aliases use deterministic permutations of the same page multiset;
eight accepted roots can be generated without eight independent data holdings.

## Attack models and current verifier outcomes

Every row below accepted all 1+2+4+8 tested claims. Rejection would occur if the
sampled bytes/path are wrong, missing, late or out of manifest/challenge context.
None of those checks asserts physical exclusivity or economic independence.

| Attack | What attacker owns / accesses | What attacker claims | Why v2 accepts; experiment boundary |
|---|---|---|---|
| 1. Same data, many keys | One backing content copy, multiple labels | One full contribution per identity | Each fresh proof is valid; no cross-identity reward rule exists |
| 2. One device, many participants | Same file through many readers | Independently contributed capacity | Indexed content verification sees bytes, not device occupancy; one-file model, no device attestation |
| 3. Remote outsourcing | Access to helper-held content, little local data | Locally retained capacity | Helper returns bytes within window; actual child process + loopback TCP, WAN case remains untested |
| 4. Resource leasing | Permission to read one lessor's copy | Owned or exclusively reserved capacity | Read access suffices; permission/lease metadata is simulated, no contract or exclusivity enforcement |
| 5. Different commitments, equivalent data | Same page multiset plus permutations | Distinct resource for every different root | Indexed roots differ while underlying bytes are shared; all eight layout roots pass |
| 6. Physical deduplication | Content-addressed pool of four pages + index | Full logical size per stored object/identity | Repeated bytes can be reconstructed at each position; an explicit dedup store models filesystem dedup, not a specific device measurement |
| 7. Compression/reconstruction | 468-byte compressed payload + software | 128 KiB reserved capacity per claimant | Lossless reconstruction yields correct bytes inside deadline; source/program storage is additional |
| 8. Partial + on-demand | Half local pages, helper for missing pages | Full local retention | Missing samples fetched on demand; success requires helper availability and sufficient latency budget |
| 9. Colluding pool | Shared helper/backend among all labels | Independent capacity and fault diversity | Separate claims are answerable from one pool; ownership and correlated failure remain unobservable |

## Detection, cost and side effects

Cost judgments are qualitative unless measured in the JSON. There are no assumed
prices, token rewards or proofs that these attacks are economically unprofitable.

| Attack | Detection possibility | Attacker cost | Verification cost | False positives | Centralization risk |
|---|---|---|---|---|---|
| 1 | Exact-root duplicate index within a specified accounting scope | Additional keys/proofs, little extra storage | Hash lookup plus ordinary proof checks | Legitimate replicas can be wrongly excluded | Global root registrar / censorship |
| 2 | Concurrent service audits may reveal overload, not hardware count | Aggregate IO/CPU demand; cache can amortize reads | Concurrent bandwidth/timing evidence | Slow honest shared hosts | Favors high-end storage and nearby auditors |
| 3 | Fresh deadlines can reject helpers too slow | Network round trips, egress, helper payment if any | Fresh challenges/deadline measurement | Honest distant providers | Datacenter/low-latency advantage |
| 4 | Disclosed time/scope registry can detect duplicate leases inside that registry | Rental/access, contractual obligations; no assumed bond | Lease authorization/history + service checks | Legitimate nonexclusive leases | Large lessors, registry and dispute authority |
| 5 | Canonical content normalization or overlap checks for supported encodings | Permutation/index, re-encoding or salting | Chunk indexing/normalization, potentially linear | Legitimate encoded/reordered datasets | Format gatekeepers and index operators |
| 6 | Distinguish logical service bytes from physical reservation; cannot infer physical size from proof alone | Unique chunks + references/lookup | Ordinary verification plus explicit reservation protocol if adopted | Efficient honest dedup penalized | Favors prescribed hardware/encoding implementations |
| 7 | Recognize known reconstructibility; cannot generally prove incompressibility of useful public data | Compressed state + decompression CPU/code | Normal proofs; assessing all compressors is infeasible | Useful public/compressible content excluded | Artificial formats and specialized hardware favored |
| 8 | Full retrieval/concurrent fresh audits reveal failures when helper cannot keep up | Retained subset + fetch latency/egress | More samples/full recovery and timing | Honest transient network failures | High-bandwidth peers advantaged |
| 9 | Independently observed service failures correlate claims; correlation does not prove common ownership | Backend pool and orchestration | Simultaneous service/availability records | Shared ISP/power affects unrelated peers | Surveillance or central scheduler temptation |

The helper's request/byte counts and each proof's response/verification timings
are in the frozen record. They measure local work, not adversarial WAN prices or
physical capacity. A registry detects claims only within its own agreement scope.
No trusted resource-to-owner map is supplied to scoring. Ground truth is used to
construct attacks and explain their shared inputs, never as an admission oracle.

## Resource-level accounting counterexamples

Scores are dimensionless illustrative units per successful claim/job, not a
selected reward formula. One underlying controlled input and the same nominal
workload are held constant while splitting from one to eight labels.

| Candidate unit | One→eight keys, same root | Alias roots from same data | What remains necessary |
|---|---|---|---|
| Unique data commitment | 1→1 | 1→8 | Common content scope/normalization; counts content identity, not physical replicas |
| Challengeable region, caller-labelled | 1→8 | 1→8 | Demonstrate non-overlap/exclusivity; a region identifier cannot establish it |
| Claimed capacity commitment | 1→8 | 1→8 | Verifiable reservation and anti-oversubscription; logical length alone fails |
| Time-bounded self-issued lease | 1→8 | 1→8 | Authorized lease scope and conflicting-lease rule; leasing need not imply ownership |
| Resource epoch = root + epoch | 1→1 in same epoch | 1→8 | Root-alias problem remains; define time coverage and cross-epoch accounting |
| Availability per identity/epoch | 1→8 | 1→8 | Availability must attach to service/resource obligation, not number of keys |
| Useful service delivery, one authorized unique job | 1→1 | 1→1 for same job | Conditional on genuine independent demand and one-use job authorization; NOT implemented |

A separate self-issued-service-job score rises 1→8. That is the direct counterexample
to calling any self-reported request “useful work.” The authorized-job variant assumes
one customer-authorized job shared across claimants; it is not an authentication or
permissionless demand solution. It uses a job scope, not a physical-resource oracle,
but who may create/reward jobs and how collusive demand is handled remain decisions.
Counterexamples at 1/2/4/8 keys are experimental, not a proof of expected economic
reward invariance for arbitrary workloads, attackers, hardware or future prices.

## Best current candidates — research recommendations, no protocol selection

First investigate **verified fulfillment of independently requested useful jobs**:
canonical job ID, terms, result/content commitment, beneficiary authorization,
replay protection and independently checkable fulfillment. This best aligns the
observable contribution with delivered value. Colluding customers, subsidized
fake demand, retries and fair treatment of parallel providers must be addressed.
Service payment still comes from its own contract/customer; any protocol resource
subsidy would require an independently approved budget/rule. Neither grants votes.

Second investigate **availability over a defined resource-service epoch**, with
explicit required replicas, locality/failure diversity claims and repeated probes.
This can measure useful service continuity without pretending to identify a disk.
Replica reward allocation, common-backend risk and normalization remain unresolved.

Treat **capacity reservation / bounded lease** as a further candidate requiring
concurrent challenge/load and anti-oversubscription evidence. Root dedup alone is a
useful accounting filter but not a uniqueness solution. Avoid arbitrary mining
blobs unless their added scarcity is separately justified against useful capacity.

Disk serials, vendor IDs, TPM identity and OS hardware IDs are **not defaults**.
They could be an optional explicitly permissioned attestation experiment, but are
spoofable or vendor-dependent at different layers, expose linkability, restrict
hardware participation and still need rules for virtualization, shared devices and
leases. No such identifier was collected or fed to consensus in this experiment.

Next measurable step: confirmed independent hosts, concurrent correlated challenges,
remote-helper delay/egress, partial recovery under interrupted helper access, and
job/epoch accounting counterexamples including legitimate replicas. Preserve honest
false-rejection measurements. No monetary issuance or selected reward percentage.
