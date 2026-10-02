OPEN_DECISIONS.md

Framework: YOS (Yousef Operating System)
Version: 1.0 (Draft)
Status: Active

Current execution status: `PROJECT.md`. Dated session records below preserve
history; later explicit decisions supersede their old permission boundaries.


2026-10-02 LOCALNET_V0 ONLY update: the human approved full-state commitment,
fixed producer/three voters/round zero, minimal local compound schemas and
atomic recovery; these are now implemented with a four-process acceptance
harness. They remove the local blockers previously recorded for state and
selection. They do not close DEC-Q1-005/009/013/014/019/020/021/022/023 for the
general protocol. Exact local schemas, domain allocations and limits are in
`docs/protocol/Q1_LOCALNET_V0.md`. No further human choice is needed for the
completed local acceptance target. DEC-Q1-015 was subsequently approved as
Apache-2.0 during release preparation. Separate human authorization subsequently
approved publication; v0.1.0-localnet.1 is now a public source release.
The [post-release plan](docs/research/Q1_PUBLIC_TESTNET_V0_PLAN.md) proposes
public-testnet gates only; all general protocol decisions above remain open.

⸻

Purpose

This document records every significant decision that has not yet been finalized.

⸻

Q1 Decision Index

| Decision ID | Title | Category | Priority | Status | Blocking Milestone | Related Documents |
|---|---|---|---|---|---|---|
| DEC-Q1-001 | Requirement identifier namespace and migration | Documentation | Critical | DECIDED | Phase 0.2 | docs/22; migration report |
| DEC-Q1-002 | Canonical serialization | Consensus | Critical | DECIDED | Pre-M1 profiles | docs/04, 05, 06, 16; ADR-0002; CBOR profile |
| DEC-Q1-003 | Cryptographic suite | Security | Critical | DECIDED | Pre-M1 profiles | docs/04, 09, 12; ADR-0003–0005; crypto/address profiles |
| DEC-Q1-004 | Private-testnet participant registry and admission | Consensus | Critical | DECIDED | M6 | docs/05, 08, 12 |
| DEC-Q1-005 | Producer and committee selection | Consensus | Critical | RESEARCH_REQUIRED | M6 | docs/05, 12, 15 |
| DEC-Q1-006 | Genesis supply invariant | Economics | Critical | DECIDED | M0 schema/M2 | docs/02, 04, 10, 14 |
| DEC-Q1-007 | Fee, issuance, reward, burn, and remainder rules | Economics | Critical | RESEARCH_REQUIRED | M2/M9 | docs/04, 10 |
| DEC-Q1-008 | Safe-mode exit and recovery authority | Security | Critical | UNDER_DISCUSSION | M6 | docs/05, 12, 19; incident response |
| DEC-Q1-009 | State, transaction, and receipt commitments | Architecture | High | OPEN | M2 | docs/04 |
| DEC-Q1-010 | Delay and HDD role | Research | High | RESEARCH_REQUIRED | M7/M8 | docs/06, 07, 20 |
| DEC-Q1-011 | Canonical repository and ADR paths | Engineering | High | DECIDED | M0 | PROJECT; docs/17, 23 |
| DEC-Q1-012 | Primary implementation language | Engineering | Critical | DECIDED | Pre-M1 toolchain | docs/17, 20; ADR-0001 |
| DEC-Q1-013 | Storage and atomic recovery | Architecture | High | RESEARCH_REQUIRED | M3 | docs/03, 08, 12 |
| DEC-Q1-014 | Network transport and handshake | Security | High | RESEARCH_REQUIRED | M5 | docs/08, 12 |
| DEC-Q1-015 | Source-code license | Legal | High | DECIDED | M0 | docs/17, 20 |
| DEC-Q1-016 | Initial integer remainder destination | Economics | High | DECIDED | M9 | docs/10 |
| DEC-Q1-017 | Initial private-testnet fee burn | Economics | High | DECIDED | M9 | docs/10 |
| DEC-Q1-018 | Selection seed and randomness | Consensus | Critical | RESEARCH_REQUIRED | M6 | docs/05, 12; ADR candidate |
| DEC-Q1-019 | Round-change protocol | Consensus | Critical | OPEN | M6 | docs/05 |
| DEC-Q1-020 | Fork choice and conflicting certificates | Consensus | Critical | UNDER_DISCUSSION | M6 | docs/05, 08, 12 |
| DEC-Q1-021 | Minimal private-testnet fee formula | Economics | Critical | UNDER_DISCUSSION | M2/M9 | docs/04, 10 |
| DEC-Q1-022 | Initial private-testnet issuance | Economics | High | UNDER_DISCUSSION | M9 | docs/10 |
| DEC-Q1-023 | Initial reward allocation | Economics | High | OPEN | M9 | docs/10 |
| DEC-Q1-024 | Public participant admission | Consensus | Critical | RESEARCH_REQUIRED | Public testnet | docs/05, 12, 20 |
| DEC-Q1-025 | Formal VDF path | Research | Critical | RESEARCH_REQUIRED | M7/public testnet | docs/06, 12 |
| DEC-Q1-026 | Public governance model | Governance | High | RESEARCH_REQUIRED | Public testnet | docs/19, 20 |

An open decision is not a bug.

It is not an omission.

It is a consciously identified question whose answer is expected to influence future work.

Keeping open decisions visible is a fundamental engineering practice within YOS.

⸻

Why This File Exists

Large projects rarely fail because they lack intelligence.

They fail because assumptions quietly become decisions.

This document prevents that.

Whenever uncertainty exists, it should be recorded here rather than silently resolved through implementation.

⸻

Decision Categories

Every entry should belong to one of the following categories:

Architecture
Engineering
Security
Consensus
Economics
Governance
User Experience
Infrastructure
Documentation
Research
Legal
Deployment
Other

⸻

Decision Status

Each decision shall have exactly one status.

OPEN
UNDER_DISCUSSION
RESEARCH_REQUIRED
BLOCKED
READY_FOR_DECISION
DECIDED
REJECTED
REMOVED

⸻

Decision Priority

Priority helps determine implementation order.

Critical
High
Medium
Low

⸻

Required Fields

Every decision should include:

Decision ID
Title
Category
Priority
Status
Owner
Created
Last Updated
Related Documents
Background
Current Options
Risks
Recommended Next Step

⸻

Decision Template

## DEC-0001
Title:
Category:
Priority:
Status:
Owner:
Created:
Last Updated:
Related Documents:
Background:
Current Options:
Option A
Option B
Option C
Risks:
Open Questions:
Recommended Next Step:

⸻

Decision Rules

A decision remains OPEN until an explicit human decision is recorded.

Neither existing code nor AI-generated suggestions close an open decision.

Only documented approval changes its status.

⸻

Responsibilities

Human

* makes final decisions;
* approves architectural direction;
* accepts trade-offs.

⸻

AI

* identifies missing decisions;
* proposes alternatives;
* compares consequences;
* highlights contradictions;
* updates documentation after approval.

AI must never silently resolve an open decision.

⸻

Engineering Policy

Whenever implementation depends on an unresolved decision, the AI should:

1. identify the dependency;
2. reference the corresponding Decision ID;
3. explain why implementation is affected;
4. recommend the smallest safe path forward.

⸻

Temporary Assumptions

Sometimes implementation must continue before a decision is finalized.

In such cases:

* assumptions must be documented;
* assumptions must reference the corresponding Decision ID;
* assumptions must be reversible.

Temporary assumptions never become permanent decisions automatically.

⸻

Decision Lifecycle

OPEN
↓
UNDER_DISCUSSION
↓
RESEARCH_REQUIRED
↓
READY_FOR_DECISION
↓
DECIDED
↓
ARCHIVED

Rejected decisions should remain documented for historical context.

⸻

Examples

DEC-0001

Title:

Primary implementation language

Status:

OPEN

Possible options:

* Rust
* Go
* Zig
* C++

⸻

DEC-0002

Title:

Canonical serialization format

Status:

UNDER_DISCUSSION

⸻

DEC-0003

Title:

Public governance model

Status:

RESEARCH_REQUIRED

⸻

Repository Rule

Every repository managed under YOS should maintain its own OPEN_DECISIONS.md.

Project-specific decisions remain inside the project.

YOS decisions remain inside the YOS repository.

Open decisions should never be mixed across unrelated projects.

⸻

Review Frequency

This file should be reviewed:

* before each milestone;
* before major architectural changes;
* before public releases;
* before introducing irreversible decisions.

Resolved decisions should be archived, not deleted.

⸻

Final Principle

An unanswered question is not a weakness.

An undocumented answer is.

Good engineering does not eliminate uncertainty.

It makes uncertainty visible until evidence allows it to disappear.

⸻

Q1 Active Decision Register

The entries below are project decisions discovered during the 2026-07-24 specification consistency review. They supersede the illustrative examples above as the active Q1 register. No entry is approved merely because an option is recommended.

## DEC-Q1-001
Title: Requirement identifier namespace and migration
Category: Documentation
Priority: Critical
Status: DECIDED
Owner: Yousef Bahrami
Created: 2026-07-24
Last Updated: 2026-07-24
Related Documents: docs/02_SYSTEM_REQUIREMENTS.md; component specifications; docs/22_REQUIREMENTS_TRACEABILITY_SKELETON.md
Background: Identifiers such as Q1-CON-001, Q1-DLY-001, Q1-HDD-001, Q1-NET-001, Q1-WAL-001, Q1-AI-001, Q1-SEC-001, Q1-API-001, and Q1-TST-001 have different definitions in multiple normative documents.
Current Options:
Option A: Reserve each prefix for one canonical document and renumber component requirements.
Option B: Add document-layer prefixes, for example SYS-CON and CNS.
Option C: Make 02_SYSTEM_REQUIREMENTS the sole requirements catalog and give component rules separate specification-clause IDs.
Risks: Broken traceability, ambiguous tests, unstable audit references.
Open Questions: Which document owns canonical requirement text? Must existing IDs remain as searchable aliases?
Recommended Next Step: Approve a namespace policy and generate a complete mechanical migration table before changing IDs.
Decision: Use globally unique document-aware domains. Existing definitions were mapped independently; superseded identifiers remain traceable in the migration report.
Approval Date: 2026-07-24

## DEC-Q1-002
Title: Canonical serialization
Category: Consensus
Priority: Critical
Status: DECIDED
Owner: Yousef Bahrami
Created: 2026-07-24
Last Updated: 2026-07-24
Related Documents: docs/04_LEDGER_AND_TRANSACTIONS.md; docs/05_CONSENSUS.md; docs/06_DELAY_ENGINE.md; docs/16_API_SPECIFICATION.md
Background: Consensus hashes and signatures require one byte representation.
Current Options:
Option A: A reviewed deterministic binary encoding with a restricted profile.
Option B: A purpose-built fixed canonical encoding.
Option C: Another format justified by test vectors and independent implementation feasibility.
Risks: Consensus splits, malleability, incompatible clients, unsafe parser complexity.
Open Questions: Unknown-field behavior, map prohibition/order, integer bounds, versioning, canonical rejection.
Decision: Use the restricted deterministic CBOR profile based on RFC 8949 in
`docs/protocol/Q1_DETERMINISTIC_CBOR_PROFILE_V1.md`. The profile requires
pre-M1 human review; generic CBOR is not sufficient. All parameters were
approved on 2026-07-24. Amount remains `u128` and has one fixed-width encoding:
CBOR byte string length 16 with an unsigned big-endian, left-zero-padded
payload; CBOR integer alternatives are prohibited.
Approval Date: 2026-07-24

## DEC-Q1-003
Title: Cryptographic suite
Category: Security
Priority: Critical
Status: DECIDED
Owner: Yousef Bahrami
Created: 2026-07-24
Last Updated: 2026-07-24
Related Documents: docs/04_LEDGER_AND_TRANSACTIONS.md; docs/06_DELAY_ENGINE.md; docs/09_WALLET.md; docs/12_SECURITY_MODEL.md
Background: Q1 requires an explicit hash, signature, key, and address suite.
Current Options:
Option A: Select one conservative suite from reviewed libraries.
Option B: Separate transaction/consensus signatures only if a demonstrated requirement justifies complexity.
Risks: Key compromise, malleability, domain confusion, ecosystem and FFI risk.
Open Questions: Algorithms, encodings, subgroup/canonical checks, domain separators, migration.
Decision: SHA-256 with versioned domain separation; pure strict-profile
Ed25519; and a Q1-defined Bech32m address envelope, as recorded in ADR-0003
through ADR-0005. Their proposed V1 profiles require pre-M1 human review.
Approval Date: 2026-07-24

## DEC-Q1-004
Title: Private-testnet participant registry and admission
Category: Consensus
Priority: Critical
Status: DECIDED
Owner: Yousef Bahrami
Created: 2026-07-24
Last Updated: 2026-07-24
Related Documents: docs/05_CONSENSUS.md; docs/08_NODE_AND_NETWORKING.md; docs/12_SECURITY_MODEL.md
Background: Deterministic selection requires a common participant set, but admission, updates, operator identity, and epoch activation are incomplete.
Current Options:
Option A: Static genesis registry for local/private milestones.
Option B: Signed, versioned registry updates under explicitly limited testnet authority.
Risks: Hidden authority, divergent participant sets, Sybil capture, key-role confusion.
Open Questions: Update authorization, activation height, removals, operator-to-node mapping, key rotation.
Recommended Next Step: Draft ADR-0010 limited to local/private test environments; do not imply public permissionlessness.
Decision: Q1 v0.1 private testnet uses a genesis-defined `PERMISSIONED_PRIVATE_TESTNET_REGISTRY` with equal producer eligibility and equal validator weights. This does not resolve public admission or Sybil resistance.
Approval Date: 2026-07-24

## DEC-Q1-005
Title: Producer and committee selection rules
Category: Consensus
Priority: Critical
Status: RESEARCH_REQUIRED
Owner: Yousef Bahrami
Created: 2026-07-24
Last Updated: 2026-07-24
Related Documents: docs/05_CONSENSUS.md; docs/12_SECURITY_MODEL.md; docs/15_SIMULATION_PLAN.md
Background: The required deterministic process is described conceptually but exact producer and committee algorithms remain open. Randomness and round change are tracked separately in DEC-Q1-018 and DEC-Q1-019.
Current Options:
Option A: A deliberately permissioned deterministic localnet placeholder, clearly isolated and non-production.
Option B: Wait for the public-admission and randomness research model.
Risks: Safety failure, biased selection, equivocation, non-termination, misleading decentralization claims.
Open Questions: weighting, seed, VRF, exclusions, locking, timeout evidence, round-change certificate, fork choice.
Recommended Next Step: Specify and review only a reversible local/private-test placeholder before M6; simulate alternatives first.

## DEC-Q1-006
Title: Genesis supply invariant
Category: Economics
Priority: Critical
Status: DECIDED
Owner: Yousef Bahrami
Created: 2026-07-24
Last Updated: 2026-07-24
Related Documents: docs/04_LEDGER_AND_TRANSACTIONS.md; docs/10_TOKENOMICS.md
Background: One document permits allocations less than configured genesis supply; another requires exact equality.
Current Options:
Option A: Allocations must equal genesis supply.
Option B: Allocations may be smaller, with explicit accounting and custody status for the remainder.
Risks: Supply invariant ambiguity, hidden reserve, different genesis state roots.
Open Questions: Meaning of unallocated supply and whether it is issued or circulating.
Recommended Next Step: Prefer the simpler equality invariant unless a documented reserve use case requires otherwise; human approval required.
Decision: The sum of all explicit allocations must equal declared genesis supply. Any reserve must be an explicit allocation to a defined address.
Approval Date: 2026-07-24

## DEC-Q1-007
Title: Fee, issuance, reward, burn, and remainder rules
Category: Economics
Priority: Critical
Status: RESEARCH_REQUIRED
Owner: Yousef Bahrami
Created: 2026-07-24
Last Updated: 2026-07-24
Related Documents: docs/04_LEDGER_AND_TRANSACTIONS.md; docs/10_TOKENOMICS.md
Background: Accounting invariants are specified, but the formulas and destinations required to execute them are not.
Narrowed By: DEC-Q1-016 resolves the initial remainder destination. DEC-Q1-017 resolves initial fee burn. DEC-Q1-021 through DEC-Q1-023 track the remaining initial fee, issuance, and reward decisions.
Current Options:
Option A: Minimal fixed private-testnet placeholders isolated behind versioned economic rules.
Option B: Delay implementation until simulation selects candidate rules.
Risks: Unauthorized issuance, nondeterministic remainder, spam, plutocracy, misleading economic behavior.
Open Questions: minimum fee, congestion rule, settlement, issuance, role shares, remainder, burn, maturity, penalties.
Recommended Next Step: Define explicit non-production placeholders only for the milestone that needs them and link every placeholder to this decision.

## DEC-Q1-008
Title: Safe-mode exit and exceptional recovery authority
Category: Security
Priority: Critical
Status: UNDER_DISCUSSION
Owner: Yousef Bahrami
Created: 2026-07-24
Last Updated: 2026-07-24
Related Documents: docs/05_CONSENSUS.md; docs/12_SECURITY_MODEL.md; docs/19_GOVERNANCE.md
Background: Entry on conflicting finality is required; exit and recovery authority remain unresolved.
Current Options:
Option A: Manual private-testnet reset with an explicit exceptional record.
Option B: Evidence-based recovery protocol approved through future governance.
Risks: Hidden sovereignty, unsafe automatic recovery, permanent halt, history rewrite.
Open Questions: who approves exit, what evidence is required, whether any chain is resumed, client coordination.
Recommended Next Step: Specify private-testnet behavior before adversarial testing; do not generalize it to public networks.
Approved Principle: Entry may be automatic. Exit requires an explicit auditable recovery package and may not be a simple administrative disable switch. The exact package and authorization remain open.

## DEC-Q1-009
Title: State, transaction, and receipt commitments
Category: Architecture
Priority: High
Status: OPEN
Owner: Yousef Bahrami
Created: 2026-07-24
Last Updated: 2026-07-24
Related Documents: docs/04_LEDGER_AND_TRANSACTIONS.md
Background: Roots are required. DEC-Q1-027 Session 4 approved the shared V1
indexed-sequence construction, typed empty roots, odd-node promotion, and
profile registry. StateRoot structure, proof formats, ReceiptRoot activation,
and snapshot-chunk activation remain outside that approval.
Current Options:
Option A: Simple deterministic ordered commitments for the prototype.
Option B: Proof-capable authenticated trees from M2.
Risks: Incompatible state roots, expensive migration, absent light-client proofs.
Open Questions: state keys/order, proof needs, pruning, Receipt schema and
activation, SnapshotChunk descriptor and activation, and owning-schema count
binding.
Approved Session 4 Constraint: TransactionRoot uses canonical
SignedTransferV1 in block-body order. ParticipantRoot uses canonical
ParticipantRecordV1 ordered by ascending raw ParticipantId. ReceiptRoot,
SnapshotChunkRoot, and StateRoot remain inactive.
Recommended Next Step: Complete owning-schema count binding and the separate
StateRoot, Receipt, and SnapshotChunk decisions before their implementation.

## DEC-Q1-010
Title: Delay and HDD role in the executable prototype
Category: Research
Priority: High
Status: RESEARCH_REQUIRED
Owner: Yousef Bahrami
Created: 2026-07-24
Last Updated: 2026-07-24
Related Documents: docs/06_DELAY_ENGINE.md; docs/07_HDD_LAB_MODULE.md; docs/20_ROADMAP.md
Background: Sequential hashing is allowed only as a labeled non-production placeholder; formal VDF and HDD effects remain open.
Current Options:
Option A: Mock plus sequential-hash delay for local/private research; HDD telemetry-only.
Option B: Wait for formal VDF selection before delay integration.
Risks: misleading security claims, hardware dominance, timing failure, cache/telemetry deception.
Open Questions: iterations, adjustment, proof limits, window integration, formal construction, HDD workload and influence.
Recommended Next Step: Keep HDD TELEMETRY_ONLY and require consensus-equivalence tests with both optional modules disabled.

## DEC-Q1-011
Title: Canonical repository and ADR paths
Category: Engineering
Priority: High
Status: DECIDED
Owner: Yousef Bahrami
Created: 2026-07-24
Last Updated: 2026-07-24
Related Documents: PROJECT.md; docs/03_ARCHITECTURE.md; docs/17_CODEX_BUILD_INSTRUCTIONS.md; ADR/README.md
Background: Current paths use YOS_v1.0 and ADR/, while project/build documents refer to YOS/ and docs/adr/.
Current Options:
Option A: Adopt docs/adr/ and keep YOS_v1.0 as a versioned vendor directory with corrected references.
Option B: Adopt another explicitly documented canonical layout.
Risks: broken links, duplicate ADR stores, agents reading stale instructions.
Open Questions: treatment of Plain text scratch files and whether YOS is copied or referenced.
Recommended Next Step: Approve canonical paths before M0 scaffolding; migrate without deleting scratch files until reviewed.
Decision: Canonical paths are `YOS/`, `docs/adr/`, `docs/build/`, `docs/security/`, and categorized directories under `docs/reports/`.
Approval Date: 2026-07-24

## DEC-Q1-012
Title: Primary implementation language
Category: Engineering
Priority: Critical
Status: DECIDED
Owner: Yousef Bahrami
Created: 2026-07-24
Last Updated: 2026-07-24
Related Documents: docs/17_CODEX_BUILD_INSTRUCTIONS.md; docs/20_ROADMAP.md
Background: M0 toolchain, package structure, CI, cryptographic ecosystem,
storage, and networking depend on this decision.
Current Options:
Option A: Rust.
Option B: Go.
Option C: Another justified systems language.
Risks: ecosystem gaps, unsafe FFI, nondeterminism, maintenance burden, slow iteration.
Open Questions: team expertise, supported platforms, independent-client strategy, reproducible builds.
Decision: Use a Rust consensus-critical core with a strictly bounded
TypeScript/Node.js application and developer-facing layer. TypeScript must not
redefine consensus rules; shared behavior requires a controlled Rust boundary
or normative cross-language conformance vectors.
Approval Date: 2026-07-24

## DEC-Q1-013
Title: Storage engine and atomic recovery model
Category: Architecture
Priority: High
Status: RESEARCH_REQUIRED
Owner: Yousef Bahrami
Created: 2026-07-24
Last Updated: 2026-07-24
Related Documents: docs/03_ARCHITECTURE.md; docs/08_NODE_AND_NETWORKING.md; docs/12_SECURITY_MODEL.md
Background: Finalized state, blocks, certificates, signing history, and crash recovery require atomic and durable semantics.
Current Options:
Option A: One embedded transactional database behind repository interfaces.
Option B: Separate stores with an explicit write-ahead/recovery protocol.
Risks: finalized-state corruption, double signing after restart, partial commits, unrecoverable migrations.
Open Questions: database, transaction scope, fsync policy, schema versioning, snapshot consistency.
Recommended Next Step: Define required durability semantics before comparing engines in ADR-0006.

## DEC-Q1-014
Title: Network transport and authenticated handshake
Category: Security
Priority: High
Status: RESEARCH_REQUIRED
Owner: Yousef Bahrami
Created: 2026-07-24
Last Updated: 2026-07-24
Related Documents: docs/08_NODE_AND_NETWORKING.md; docs/12_SECURITY_MODEL.md
Background: Transport, encryption, session agreement, discovery, message signing, and identity mapping are open.
Current Options:
Option A: A mature encrypted transport with application-level signed consensus messages.
Option B: Another transport behind a stable interface with equivalent authentication properties.
Risks: impersonation, replay, eclipse, resource exhaustion, identity/key reuse.
Open Questions: QUIC, handshake, node rotation, discovery, required object signatures, rate limits.
Recommended Next Step: Threat-model the handshake and peer lifecycle before ADR-0007.

## DEC-Q1-015
Title: Source-code license
Category: Legal
Priority: High
Status: DECIDED
Owner: Yousef Bahrami
Created: 2026-07-24
Last Updated: 2026-10-02
Related Documents: docs/17_CODEX_BUILD_INSTRUCTIONS.md; docs/20_ROADMAP.md
Background: M0 requires a LICENSE or an explicit decision placeholder; independent implementation is a stated governance principle.
Options considered before approval:
Option A: Permissive open-source license.
Option B: Copyleft open-source license.
Option C: Keep all rights reserved temporarily with explicit research terms.
Risks: contribution ambiguity, incompatible dependencies, barriers to independent clients, unintended commercialization rights.
Remaining scope: contribution process and trademark policy are not selected by a software license.
Next Step: Maintain the approved license and factual attribution in the public source repository.

Decision (2026-10-02): Apache-2.0 explicitly approved by the owner during
LOCALNET public release candidate preparation. LICENSE contains the official
text; LICENSE_DECISION.md records the comparison and superseded placeholder.
The license choice alone did not authorize publication. A subsequent explicit
human instruction authorized the completed public snapshot release, separately
from any public-network launch.

## DEC-Q1-016
Title: Initial integer remainder destination
Category: Economics
Priority: High
Status: DECIDED
Owner: Yousef Bahrami
Created: 2026-07-24
Last Updated: 2026-07-24
Related Documents: docs/10_TOKENOMICS.md
Background: Integer reward allocation can leave a remainder.
Decision: For the initial private-testnet economic model, every integer division remainder goes to the Protocol Treasury. Funding sources must equal distributed rewards plus treasury remainder plus any explicitly enabled burn.
Risks: Treasury accumulation must remain visible and exactly accounted.
Recommended Next Step: Add deterministic examples and conservation tests before economic implementation.

## DEC-Q1-017
Title: Initial private-testnet fee burn
Category: Economics
Priority: High
Status: DECIDED
Owner: Yousef Bahrami
Created: 2026-07-24
Last Updated: 2026-07-24
Related Documents: docs/10_TOKENOMICS.md
Background: Fee burn was optional and unresolved.
Decision: Fee burn is disabled in the simplest initial private-testnet configuration. Collected fees go to the security reward pool unless a later approved scenario changes allocation.
Risks: This temporary model is not permanent monetary policy.
Recommended Next Step: Preserve burn as a disabled, versioned test scenario only.

## DEC-Q1-018
Title: Selection seed and randomness construction
Category: Consensus
Priority: Critical
Status: RESEARCH_REQUIRED
Owner: Yousef Bahrami
Created: 2026-07-24
Last Updated: 2026-07-24
Related Documents: docs/05_CONSENSUS.md; docs/12_SECURITY_MODEL.md; docs/15_SIMULATION_PLAN.md
Background: Selection must be deterministic and derived only from finalized, publicly verifiable protocol context.
Current Options: Previous finalized block hash; finalized delay output; domain-separated finalized values; commit-reveal; future VRF or beacon.
Approved Constraints: No local, wall-clock, AI, external-unverified, mempool, device, or market-price randomness.
Risks: Grinding, withholding, transaction influence, bias, predictability, liveness loss.
Recommended Next Step: Prepare a focused decision memo or ADR candidate; do not select construction yet.

## DEC-Q1-019
Title: Round-change protocol
Category: Consensus
Priority: Critical
Status: OPEN
Owner: Yousef Bahrami
Created: 2026-07-24
Last Updated: 2026-07-24
Related Documents: docs/05_CONSENSUS.md
Background: Rounds and fallback exist conceptually, but transitions and evidence are incomplete.
Open Questions: Start, producer window, timeout observation/evidence/certificate, advancement, late proposals, old-round attestations, restart, partitions, conflicts, maximum round.
Risks: Divergent rounds, stale votes, non-termination, unsafe fallback.
Recommended Next Step: Create a state-transition table and dedicated decision document before implementation.

## DEC-Q1-020
Title: Fork choice and conflicting certificates
Category: Consensus
Priority: Critical
Status: UNDER_DISCUSSION
Owner: Yousef Bahrami
Created: 2026-07-24
Last Updated: 2026-07-24
Related Documents: docs/05_CONSENSUS.md; docs/08_NODE_AND_NETWORKING.md; docs/12_SECURITY_MODEL.md
Background: Q1 follows certificate-backed finalized history, not proof-of-work longest-chain selection.
Approved Principle: Follow valid finalized history supported by verified certificates. If two conflicting valid finalization certificates exist, enter Safe Mode and do not choose silently.
Open Questions: Competing unfinalized proposals, stale branch retention, sync ties, checkpoints, recovery.
Risks: Silent history choice, inconsistent synchronization, hidden recovery authority.
Recommended Next Step: Prepare a focused record distinguishing all five documented history cases.

## DEC-Q1-021
Title: Minimal private-testnet fee formula
Category: Economics
Priority: Critical
Status: UNDER_DISCUSSION
Owner: Yousef Bahrami
Created: 2026-07-24
Last Updated: 2026-07-24
Related Documents: docs/04_LEDGER_AND_TRANSACTIONS.md; docs/10_TOKENOMICS.md
Background: A deterministic proposal is authorized, not approved.
Candidate: `BaseFee + FeePerByte × SerializedTransactionSize + CongestionFee`, bounded by positive MinimumFee and SenderFeeLimit.
Approved Constraints: Value-sensitive fee zero; negative fee prohibited; anonymous subsidy disabled; market-price input prohibited; congestion uses finalized block utilization only.
Open Questions: Window, thresholds, multipliers, and adjustment bounds.
Recommended Next Step: Produce deterministic examples and a decision record before implementation.

## DEC-Q1-022
Title: Initial private-testnet issuance
Category: Economics
Priority: High
Status: UNDER_DISCUSSION
Owner: Yousef Bahrami
Created: 2026-07-24
Last Updated: 2026-07-24
Related Documents: docs/10_TOKENOMICS.md
Background: A fixed configurable issuance per finalized block may be proposed for test planning.
Approved Constraints: Testnet-only; no permanence or scarcity claim; issuance only for finalized blocks; no issuance for AI output or HDD telemetry alone.
Open Questions: Exact configured values and activation/version rules.
Recommended Next Step: Create deterministic examples without presenting them as permanent policy.

## DEC-Q1-023
Title: Initial reward allocation
Category: Economics
Priority: High
Status: OPEN
Owner: Yousef Bahrami
Created: 2026-07-24
Last Updated: 2026-07-24
Related Documents: docs/10_TOKENOMICS.md
Background: Initial roles are producer, delay executor, eligible validators, and protocol treasury. HDD, AI, and relay rewards default to zero.
Open Questions: Weights, duplicate roles, validator division, eligibility, maturity, equivocation treatment, exact conservation.
Risks: Double rewards, unauthorized value, nondeterministic division.
Recommended Next Step: Prepare an integer-weight decision with examples. Keep 35/25/30/10 explicitly illustrative.

## DEC-Q1-024
Title: Public participant admission
Category: Consensus
Priority: Critical
Status: RESEARCH_REQUIRED
Owner: Yousef Bahrami
Created: 2026-07-24
Last Updated: 2026-07-24
Related Documents: docs/05_CONSENSUS.md; docs/12_SECURITY_MODEL.md; docs/20_ROADMAP.md
Background: The permissioned v0.1 registry does not solve public admission or Sybil resistance.
Risks: Identity farming, capture, hidden permissioning, plutocracy.
Recommended Next Step: Keep outside private-testnet implementation and evaluate through simulation and independent review.

## DEC-Q1-025
Title: Formal VDF path
Category: Research
Priority: Critical
Status: RESEARCH_REQUIRED
Owner: Yousef Bahrami
Created: 2026-07-24
Last Updated: 2026-07-24
Related Documents: docs/06_DELAY_ENGINE.md; docs/12_SECURITY_MODEL.md
Background: Sequential hashing is a labeled non-production placeholder, not a formal VDF.
Open Questions: Construction, setup assumptions, proof/verification cost, light-client needs, migration.
Risks: False security claims, hardware dominance, unverifiable delay.
Recommended Next Step: Seek formal cryptographic review before any public-network role.

## DEC-Q1-026
Title: Public governance model
Category: Governance
Priority: High
Status: RESEARCH_REQUIRED
Owner: Yousef Bahrami
Created: 2026-07-24
Last Updated: 2026-07-24
Related Documents: docs/19_GOVERNANCE.md; docs/20_ROADMAP.md
Background: Initial project governance does not decide future public-network governance.
Risks: Founder, treasury, validator, wealth, or emergency-authority capture.
Recommended Next Step: Keep governance automation out of M0–M1 and evaluate before public-testnet authorization.

## DEC-Q1-027
Title: M1.2 protocol object schema package
Category: Protocol
Priority: Critical
Status: OPEN
Owner: Yousef Bahrami
Created: 2026-07-24
Last Updated: 2026-07-24
Related Documents: docs/02_SYSTEM_REQUIREMENTS.md; docs/04_LEDGER_AND_TRANSACTIONS.md; docs/05_CONSENSUS.md; docs/08_NODE_AND_NETWORKING.md; docs/10_TOKENOMICS.md; docs/31_M1_2_PROTOCOL_OBJECT_SCHEMAS_REPORT.md
Background: M1.2 requires exact canonical schemas and golden vectors for eight
protocol objects. Existing documents provide conceptual field lists and
approved cryptographic domains, but do not approve the complete field order,
field encodings, enum values, optional-field representation, collection
ordering, signing forms, and object-ID inputs required for deterministic
implementation.
Approved Constraints: Restricted deterministic CBOR; fixed-position arrays
unless separately approved; explicit schema versions; chain identity inside
replay-sensitive payloads; registered domains only; strict rejection; no
ledger execution or consensus logic.
Open Questions: Exact ChainId byte representation and length; exact schemas
for GenesisManifest, ParticipantRecord, TransactionEnvelope, BlockHeader,
BlockProposal, Attestation, FinalizationCertificate, and SnapshotManifest;
transaction discriminants and optional memo/public-key rules; whether
transaction IDs and block IDs include signatures; receipt/delay/HDD field
presence; participant role/status encodings; certificate attestation ordering
and duplicate policy; snapshot chunk-manifest form; genesis allocation and
configuration ordering; every signed-versus-complete field set.
Dependencies: DEC-Q1-004, DEC-Q1-005, DEC-Q1-006, DEC-Q1-009, DEC-Q1-010,
DEC-Q1-018, DEC-Q1-019, DEC-Q1-021, DEC-Q1-022, and DEC-Q1-023.
Risks: Cross-client divergence, ambiguous signatures, replay, unstable object
IDs, circular self-hash fields, non-canonical collections, and accidental
implementation of unresolved consensus or economics.
Recommended Next Step: Produce a human decision package with one field table
per object, including array position, type/width, required/absent semantics,
enum values, ordering, size limits, unsigned form, complete form, and domain
usage. Approve the package before creating `crates/q1-protocol-types` or
`vectors/protocol_objects/v1/`.
Decision Package: `docs/32_DEC_Q1_027_SCHEMA_DECISION_PACKAGE.md`
Preparation Gate: AUTHORIZED on 2026-07-24. This does not approve any proposed
schema row or authorize implementation.
Session 1 Record: APPROVED WITH RECORDED CONDITIONS AND REVISIONS on
2026-07-24. Approved rows are SCHEMA-COMMON-001 through SCHEMA-COMMON-019 and
SCHEMA-GATE-001 through SCHEMA-GATE-003. `CHAIN_ID = 0x0010` is approved
pending normative registry registration. No object schema, reusable type,
Merkle rule, codec, vector, or implementation was approved in Session 1.
Session 2 Record: APPROVED WITH RECORDED CONDITIONS AND REVISIONS on
2026-07-24. Approved rows are SCHEMA-COMMON-020 through SCHEMA-COMMON-022 and
SCHEMA-PART-001 through SCHEMA-PART-005. ParticipantRecordV1 is the approved
six-field record; its three-field identity body contains only nullable
producer and validator role keys. Roles, activity, unit V1 weight, and
ParticipantId are derived. Node transport identity is excluded.
`PARTICIPANT_ID = 0x0013` is approved pending normative registry registration.
A narrow M1.1 correction must introduce `RoundNumber(u32)` and deprecate or
reclassify Slot/composite Round for consensus use before schema
implementation. That correction, all implementation, TransactionEnvelope,
and Session 3 remained unauthorized at the close of Session 2.
Session 3 Record: APPROVED WITH RECORDED REVISIONS on 2026-07-24. Approved
rows are SCHEMA-TX-001 through SCHEMA-TX-006 and SCHEMA-ECON-001.
TransferBodyV1 is an exact nine-field transfer-only body and SignedTransferV1
is its three-field signature envelope. Sender identity has only one consensus
source: the serialized public key. `transaction_type`, `sender_address`, and
`memo_hash` are absent with no reserved positions. TransferId hashes the
complete signed envelope under the unchanged transaction domain. FeeLimit
shape is frozen, but no fee, replay, expiration, ledger, wallet, mempool, or
execution behavior is approved. Implementation and Session 4 remain
unauthorized.
Session 4 Record: APPROVED WITH RECORDED CONDITIONS AND REVISIONS on
2026-07-24. Approved rows are SCHEMA-MERKLE-001 through
SCHEMA-MERKLE-005. The shared binary indexed-sequence profile uses the
existing MERKLE_LEAF and MERKLE_INTERNAL domains, typed empty roots, and
odd-node promotion; duplicate-last was rejected. TransactionRoot and
ParticipantRoot are approved only for future implementation after an explicit
implementation gate and owning-schema count binding. ReceiptRoot,
SnapshotChunkRoot, and StateRoot remain reserved inactive under their recorded
dependencies. No Merkle implementation, proof, codec, parser, vector,
BlockHeader work, or Session 5 work is authorized.
Session 5A Record: APPROVED WITH RECORDED PARENT REVISION on 2026-07-25.
SCHEMA-BLOCK-001 approved the separate BlockHeaderBodyV1 and exact
three-field SignedBlockHeaderV1 envelope without approving the body's complete
field table or length. SCHEMA-BLOCK-002 approved BlockId over the canonical
complete signed envelope under BLOCK_ID. ParentReferenceV1 is an exact
three-field tagged record with GENESIS=`0x0001` and BLOCK=`0x0002`.
GenesisManifest is independent; the first signed block is height 1 and
references its governing GenesisId, while every later signed block references
the immediately preceding BlockId. Null, zero-sentinel, untagged, body-only,
partial-envelope, and undomained alternatives are rejected. No implementation
or Session 5B work is authorized.
Session 5B Record: APPROVED WITH RECORDED CONDITIONS AND IMPLEMENTATION
BLOCKERS on 2026-07-25. Required eventual Header members now include
TransactionRoot, ParticipantRoot, and StateRoot; final indexes remain open.
BlockBodyV1 is the exact two-field version/ordered-transfer object.
ParticipantSetV1 is the exact three-field version/reference-height/sorted-record
object for the active pre-H set. TransactionCount and ParticipantCount are
distinct derived `u32` semantics and are not serialized. Producer membership
and key resolution are historical/state validation. SCHEMA-BLOCK-004/005 were
revised and approved to omit ReceiptRoot and generic body commitment.
StateRoot field shape is approved, but construction profile `0x0005` remains
inactive and blocks BlockHeaderV1 implementation and valid instantiation. No
implementation or Session 5C work is authorized.
Session 5C Record: APPROVED WITH IMPLEMENTATION AND ACTIVATION BLOCKERS on
2026-07-25. SCHEMA-BLOCK-003/006 and the exact eleven-field
BlockHeaderBodyV1 are approved. The required protocol_version is semantic
`u16=1`; round_number is `RoundNumber(u32)` with zero valid. CandidateIndex
and all other recorded exclusions are omitted without placeholders. Header
index 10 is a typed DelayEvidenceHash commitment to the complete five-field
DelayEvidenceV1. The existing DELAY_OUTPUT domain is not reused;
DELAY_EVIDENCE remains pending separate normative registration. NONE has
canonical evidence `[1, 0, 0, h'', h'']` only where a network profile permits
it. Schema is approved, but implementation and activation remain blocked by
the M1.1 RoundNumber correction, pending domain registrations, delay profiles
and bounds, network policy, state model/StateRoot activation, independent
vectors, and explicit authorization. No implementation or Session 6 work is
authorized.

## DEC-Q1-M1.1 — RoundNumber Primitive Correction

Status: CLOSED — GATES 1–3 COMPLETE

Decision owner: Yousef Bahrami

Decision date: 2026-07-25

Decision record:
`docs/43_DEC_Q1_M1_1_ROUNDNUMBER_GATE_1_HUMAN_DECISIONS.md`

Closure record:
`docs/44_DEC_Q1_M1_1_ROUNDNUMBER_GATE_3_CLOSURE.md`

Approved decisions: RN-G1-001 through RN-G1-010.

RoundNumber is the distinct nominal `u32` active round primitive, begins at
zero, uses shortest canonical CBOR unsigned encoding, and has no dependency on
Slot. Historical Round is deprecated legacy compatibility-only. Historical
Slot is a deprecated legacy non-consensus data type. Compatibility is bounded;
legacy removal requires a later explicit human Gate.

Gate 2 implemented and committed the separately authorized primitive-only
correction in
`e85f8a53a42ea2e40696a2e99b8bc554f9072429`. RoundNumber is now an
independent `u32` newtype with `ZERO`, standalone shortest canonical CBOR
unsigned encoding, and no Slot dependency. The Gate 3 review found code, test,
scope, and documentation conformance. Gate 3 and the DEC-Q1-M1.1 primitive
correction are closed by final human decision. Historical Round and Slot
remain deprecated with their codecs unchanged. Legacy removal, protocol-object
work, runtime behavior, Session 6 work, and any further implementation remain
unauthorized.


## LOCALNET v0 bounded decisions — 2026-10-01

Human owner: Yousef Bahrami. Scope: LOCALNET v0 ONLY.

Approved: fixed actual fee=1 with FeeLimit ceiling; zero issuance/burn/reward
distribution; explicit reward_pool; exact supply conservation; no fee or state
change on rejection; fixed three-voter membership with quorum=2 and producer
excluded; no membership change on disconnect; NONE witness only for LOCALNET;
DELAY_EVIDENCE=0x0014 after collision check (free, now registered).

Current implementation, remaining questions and precise boundaries:
`docs/protocol/Q1_LOCALNET_V0.md` and `PROJECT.md`.

DEC-Q1-021 through DEC-Q1-023 remain open for general protocol economics.
The general quorum contradiction and final delay mechanism remain open.
StateRoot, selection and unapproved compound schemas have not been decided
by these three local decisions. No mainnet or public deployment is authorized.
