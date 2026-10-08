# Q1 resource evidence experiment v1 — design

Date: 2026-10-05. Original design, subsequently approved for limited implementation.
[Measured v1 results](Q1_MINING_DELAY_EXPERIMENT_V1_RESULTS.md) now supersede
its earlier not-implemented status; unexecuted future experiments stay open. The human
requested redesign after the negative v0 results. This document proposes a
bounded laboratory experiment, not a new consensus primitive, domain number,
Mainnet identity, issuance rule or production mining claim.

## Accepted negative result

[Experiment v0](Q1_MINING_DELAY_EXPERIMENT_V0.md) did **not** prove durable
storage, physical HDD possession, HDD-bound work, Sybil resistance or
economically fair mining. It did demonstrate canonical commitment/evidence
construction, deterministic verification and reproducibility of that laboratory
path. Deleting the file and regenerating chunks from its public seed still
produced valid evidence. Retained tree metadata was much smaller than the data.
Identity splitting materially improved several simulated reward shares.
These are design failures, not reasons to repeat the same workload on a bigger
drive. Preserve the v0 source, measurements and checksums as historical evidence.

## Four different claims

| Claim | What v1 would test | What it would not establish |
|---|---|---|
| Data possession | Fresh selected bytes authenticate against an earlier commitment | Ownership of a device, continuous retention or completeness from one sample |
| Available storage | Repeated successful reads and bounded complete retrieval of an assigned corpus | Independent physical copies, reserved capacity or availability between audits |
| Time / delay | Verifier-observed response duration, including network and scheduling | An unavoidable sequential delay, trusted prover clock or physical seek time |
| Participant uniqueness | Key ownership and duplicate transcript/resource accounting | One human, one business, one disk or one independent operator |

The existing public-seed dataset cannot meet the new possession hypothesis.
The v1 assumption is an independently assigned, high-entropy corpus whose bytes
cannot be reconstructed from public information. Its hash commitment is binding
under the hash assumption; binding alone supplies no availability guarantee.

## Candidate experiment and explicit trust boundary

1. An independent laboratory corpus provider generates random bytes using the
   operating system's cryptographic random source. No public generation seed,
   compressible fixture or prover-selected corpus is eligible for a live trial.
   Deliver a distinct corpus for each allocation and record a unique unit ID,
   byte length, chunk width and assignment. The provider is trusted not to
   collude, reuse assignments or leak reconstruction state. This centralized
   laboratory assumption is **not** a permissionless admission solution.
2. The provider independently computes the assigned corpus's reference root.
   The prover builds an indexed Merkle commitment; the verifier accepts it only
   if it matches that assigned root and dimensions. Otherwise a prover could
   substitute compressible data. Record and acknowledge the complete manifest
   before releasing any challenge. A root change requires a new provider
   assignment; it cannot repair an outstanding failed challenge.
3. An independent verifier then generates a fresh 256-bit random nonce and
   derives distinct chunk indices by rejection sampling a domain-separated
   hash stream. Sampling is uniform **without replacement**. Include session,
   manifest digest, unit, epoch and challenge counter in the stream input.
   Publish the exact stream framing and vectors before implementation. No
   Q1 protocol domain is allocated by this design.
4. The prover returns selected chunk bytes and indexed authentication paths.
   The verifier checks the pinned manifest, complete index set, lengths, path
   directions/depth, root and canonical encoding. It cannot regenerate the
   expected bytes from a public seed as v0 did; authentication establishes
   membership, not honest corpus generation.
5. Measure response arrival using the verifier's monotonic clock. Freeze the
   deadline policy after honest calibration and before adversarial trials.
   Count timeout, disconnect and missing responses as failures, not omitted
   observations. Repeated fresh challenges continue over a declared window.
6. Perform a bounded **complete retrieval and root recomputation** before and
   after the window. Archive the full transcript and independent reference
   reconstruction after the run. An audit copy must be inaccessible to the
   prover during trials, except in an explicitly labeled outsourcing attack.

Two processes on one host provide functional separation only; they do not
establish a hostile prover/verifier boundary. Live runs need independent
verifier control. Replaying a published transcript demonstrates determinism;
it cannot demonstrate that its original nonce was unpredictable. Fixed public
fixtures must be labeled replay fixtures and never counted as fresh challenges.

## Probability boundary — do not overclaim the core requirement

If **none** of the unpredictable corpus is available to the adversary, including
RAM, backups and remote helpers, public root/tree metadata should not supply
the challenged bytes. This remains a cryptographic assumption to review, not
an experimentally proven lower bound on all attackers.

If only some bytes are missing, sampling cannot promise near-zero acceptance.
For N chunks, m unavailable chunks and k distinct sampled chunks, a static
adversary that answers exactly its retained chunks succeeds with probability
`C(N-m,k) / C(N,k)`. With t independent fresh challenges and the same missing
set, the probability is that quantity to power t. Adaptation, leakage and
remote retrieval invalidate that simplified model.

Analytical values, calculated using exact integer combinations; **not v1
measurements** (N=4096, k=64):

| Missing chunks | One challenge passes | Ten independent challenges all pass |
|---|---:|---:|
| 1 | 0.984375 | 0.854291 |
| 41, approximately 1% | 0.522630 | 0.00152035 |
| 410, approximately 10% | 0.00110781 | 2.78392e-30 |
| 2048, 50% | 3.28815e-20 | 1.47744e-195 |

Consequently, the literal requirement that **any** unavailable committed data
must make acceptance negligible is not satisfied by 64 samples. Complete
retrieval detects missing data at the retrieval instant; neither test proves
continuous storage. Report `sample audit passed` separately from `complete
retrieval passed`. A stronger retrievability claim needs an approved formal
construction, adversary model and review before reward or authority use.

[Provable Data Possession](https://eprint.iacr.org/2007/202) studies probabilistic
checks of stored data using sampled blocks. [Compact Proofs of
Retrievability](https://eprint.iacr.org/2008/073) studies schemes with extraction
guarantees. These are comparison references: Q1's proposed Merkle experiment
does not inherit their security proofs, implement their schemes or establish
an extractor. Neither reference turns Q1's evidence into a hardware proof.

## Canonical transcript and bounded verifier design

Proposed lab-only v1 encoding: UTF-8 JSON restricted to ASCII field names and
string values, unsigned bounded integers, arrays and objects; no floats,
booleans as integers, nulls, duplicate keys or unknown fields. Serialize sorted
object keys with no whitespace and one specified trailing-newline policy;
reject bytes that do not equal canonical reserialization. Hash the canonical
bytes with separate length-framed SHA-256 labels for manifest, leaf, tree node,
challenge and transcript. Use lowercase fixed-width hex for hashes/nonces and
chunk bytes. This is separate from approved Q1 CBOR/domain registries.

Required schema groups to freeze with vectors before coding:

- Manifest: version/purpose, session/allocation/unit IDs, dataset length,
  chunk width/count, Merkle root and assigned prover/verifier key fingerprints.
  Leaves bind index, length and bytes; power-of-two dataset sizes avoid an
  unspecified odd-leaf rule. Key fingerprints identify experiment endpoints,
  never unique people. Authenticated assignment/challenge delivery must be
  specified for a multi-host trial; local files alone are not authentication.
- Challenge: manifest digest, epoch/counter, nonce, sorted distinct indices,
  sampling version and deadline-policy ID. The verifier derives indices itself.
- Response: challenge digest, exact selected chunks and paths in index order.
  A replay for another allocation/session/epoch/counter must fail. Repeated
  copies of one response never create additional credit.
- Result: accept/reject reason and verifier receipt timing in a separate
  telemetry record. Wall-clock timestamps and host/process IDs are not proof
  validity or consensus inputs.

Keep the existing safe scale: 1/4/16 MiB datasets, 4 KiB chunks, 64 indices,
ten fresh rounds per mode, at most 2 MiB per sample proof and bounded parser
depth/collection sizes. Complete retrieval streams one chunk at a time and
has a separate 16 MiB total cap. Create each dataset once per suite (21 MiB
combined), reuse read-only across trials, and record actual write totals.
No unbounded nonce grinding, disk fill or continuous write benchmark.

State progression: assigned → committed → challenged → accepted/rejected or
timed out. Each challenge has one terminal result; late responses remain late.
Store all issued challenges and failures. No adaptive exclusion of bad runs,
silent retries, response-only denominators or prover-selected nonce/deadline.

## Cache, device and substitution matrix

| Mode | Planned procedure | Interpretation |
|---|---|---|
| Cold access attempt | Record cache controls, first-access conditions and physical-read counters where available | A fresh file or first read after writing is not proof of a cold disk; label cold status unverified if counters/control are unavailable |
| Warm/cache | Repeat reads with cache intentionally warm; report logical versus physical bytes | Measures cached path, not media speed |
| RAM-resident bytes | Keep actual corpus in RAM, delete disk copy in the experiment directory, answer fresh challenges | Expected to pass possession; shows the mechanism does not require a disk |
| RAM/public-seed reconstruction | Retain only v0 seed/tree and attempt v0-style reconstruction against the v1 root | Must fail for a genuinely independent v1 corpus; a leaked private corpus-generation seed is a failed assumption |
| SSD | Record declared device metadata, read policy and counters | Host metadata is not remotely attested device identity |
| HDD | Same bounded reads only when an existing drive is available | NOT RUN until hardware exists; no purchase and no claim HDD is required |
| Root/tree only | Remove corpus from every prover-accessible source, keep metadata | Fresh challenge must fail; accidental audit-copy access invalidates the trial |
| Partial retention | Retain 25/50/90/99% and all but one chunk; log retained indices | Compare against sampling model and full retrieval; do not demand impossible sampling guarantees |
| Remote helper / aliases | Outsource reads and reuse storage under many keys concurrently | Passing exposes locality/uniqueness limitations; it is not evidence of separate capacity |

Do not drop system-wide caches, bypass device safety, fill free space or use
privileged/destructive IO. Per-file cache hints may not control controller
caches. If physical IO cannot be measured, report it unavailable. Report setup,
proof, verification, RTT, timeouts, CPU, peak memory, retained data/metadata,
proof size and logical/physical reads separately; energy and wear are unknown
unless actually measured. Network deadlines can disadvantage remote honest
participants; faster hardware remains an advantage, not a protocol violation.

## Acceptance and next implementation gate

Future tests must include commitment-after-challenge rejection, nonce replay,
wrong context/root/index/length/path, duplicate fields, oversized/deep input,
timeouts, missing responses, partial storage, metadata-only generation,
outsourcing and cross-key duplicate accounting. Publish honest and adversarial
results, including failures, rather than a single throughput ranking.

Implement only after review of corpus trust, exact transcript/authentication,
deadline calibration and what constitutes a successful **laboratory** result.
Even successful v1 evidence cannot close the [Sybil
blocker](Q1_SYBIL_AND_RESOURCE_IDENTITY_ANALYSIS.md). The research process
remains optional telemetry outside consensus: no producer selection, vote,
real reward, monetary issuance or Mainnet claim. No v1 test ran in the original
design-only checkpoint. The subsequent approved execution and its limits are
recorded in [v1 results](Q1_MINING_DELAY_EXPERIMENT_V1_RESULTS.md).
