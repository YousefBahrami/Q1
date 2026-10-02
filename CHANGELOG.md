# Q1 Changelog

All notable specification, architecture, and implementation changes are
recorded here. Changes to protocol semantics require an approved decision or
ADR and must identify affected requirement IDs.

## Identity/privacy and early-revenue preparation — 2026-10-03

- Audited public commit identities, files, release metadata and downloaded assets;
  found only the existing GitHub noreply identity in baseline Git metadata.
- Added canonical project/fork guidance without changing Apache-2.0 or asserting
  registered trademark rights; documented future commit privacy commands.
- Compared nine revenue paths and expanded the explicit monetary-distribution
  approval boundary. No token sale, payment channel or hosted service launched.

## Post-release documentation review — 2026-10-02

- Corrected public-source/CI status and project attribution; protocol behavior unchanged.
- Added differentiation audit, proposed PUBLIC TESTNET v0 gates and pre-offer boundary.
- Public testnet implementation remains subject to explicit human approval.
- The published tag and original release assets remain unchanged.

## v0.1.0-localnet.1 — public source release, 2026-10-02

- Accepted LOCALNET v0 protocol milestone 0c177fd; canonical vectors unchanged.
- Approved Apache-2.0 under DEC-Q1-015; included LICENSE and package metadata.
- Added foreground four-node supervisor, local wallet creation, signing/submission
  and status commands with executable end-to-end reproduction checks.
- Reworked README, contribution/security guidance and release notes for new users.
- Prepared deterministic source-only archive/checksums with personal context,
  original Git history, keys, node data and generated output excluded.
- Audited release hygiene and documented build-cache size. Original private history
  was preserved privately; a clean public repository was initialized from the snapshot.
- Published the tag and experimental release after remote CI passed; anonymous
  fresh-clone tests and public asset checksum verification subsequently passed.

The entries below are dated implementation history. Their then-current permission,
license and publication boundaries are superseded by later explicit decisions.

## Unreleased — LOCALNET v0 four-node execution (2026-10-02)

- Implemented explicitly authorized LOCALNET_V0 full canonical state, minimal
  genesis/proposal/attestation/certificate and approved eleven-field HeaderBody.
- Registered collision-checked local proposal/state domains; added frozen
  reference vectors and independent Python reconstruction/hash verification.
- Added strict whole-block validation, atomic durable archives, authenticated
  replay, exclusive locking and persistent per-voter equivocation detection.
- Added loopback node daemon, explicit fixture keys and real four-process
  acceptance including 2-of-3 progress with a voter offline, restart/catch-up,
  equal StateRoot, insufficient quorum and post-restart continuation.
- Wired acceptance into the existing local/CI check runner. Public deployment,
  final Mainnet protocol, producer rotation and license decision were outside
  that local milestone; its report predates the later successful publication CI.

## Unreleased — Phase 0.1 normalization

- Added initial specification consistency review and M0 planning documents.
- Approved document-aware requirement namespaces and applied the first
  identifier migration.
- Normalized canonical paths to `YOS/` and `docs/adr/`.
- Approved the exact genesis-allocation invariant.
- Registered the private-testnet permissioned participant model.
- Added Proposed ADR-0001 through ADR-0005.
- Added M0 security-document drafts.
- Expanded the Q1 decision register and index.

## Unreleased — Phase 0.2 ADR evaluation

- Expanded ADR-0001 through ADR-0005 with decision drivers, credible
  alternatives, comparative matrices, impacts, risks, recommendations,
  evidence, and limitations.
- Added substantive TypeScript/Node.js and hybrid-architecture analysis.
- Prepared all five ADRs as READY_FOR_DECISION without accepting them.
- Added the Phase 0.2 ADR evaluation report.

## Unreleased — Pre-M1 determinism and toolchain readiness

- Recorded human acceptance of ADR-0001 through ADR-0005.
- Selected a Rust consensus-critical core with a strictly bounded
  TypeScript/Node.js application and developer layer.
- Added proposed normative V1 profiles for restricted deterministic CBOR,
  cryptographic domain separation, Ed25519, and Q1 Bech32m addresses.
- Added initial canonical, rejection, domain, signature, and address vectors;
  remaining independent Rust reproduction is explicitly tracked.
- Installed and validated stable Rust 1.97.1 with Cargo, rustfmt, and Clippy
  using an isolated non-Q1 smoke project.
- Updated project state, open decisions, glossary, traceability, and affected
  architecture, ledger, wallet, security, test, and build specifications.
- Added the pre-M1 readiness report; M1 remains blocked pending gate review.

## Unreleased — Pre-M1 bounded normative finalization

- Recorded acceptance of the pre-M1 readiness report and the human gate
  decision `PRE-M1: REPEAT — BOUNDED FINALIZATION CYCLE`.
- Added the normative parameter decision package with stable PREM1 decision
  identifiers and blank human-approval fields.
- Added isolated pinned conformance artifacts under
  `research/pre_m1_conformance/`; no Q1 runtime module was created.
- Reproduced the Q1 Ed25519 vector independently with ed25519-dalek 3.0.0 and
  Node/OpenSSL, including mutated-message and mutated-signature rejection.
- Reproduced CBOR acceptance/rejection, domain hashes, and Bech32m address
  vectors across Rust, Node, and Python research implementations.
- Expanded the four profile drafts with exact candidate parameters, vectors,
  alternatives, security/compatibility impacts, and remaining human choices.
- M1 remains blocked pending explicit human parameter decisions.

## Unreleased — M1.1 protocol primitives

- Recorded `PRE-M1: COMPLETE` and limited authorization for M1.1.
- Approved all PREM1 parameter rows and made the four protocol profiles
  normative.
- Added Q1-LTX-024: `Amount(u128)` is encoded only as a definite 16-byte
  big-endian CBOR byte string.
- Added the pinned Rust workspace and `crates/q1-primitives`.
- Implemented typed Amount, Height, Slot, Round, hash, Ed25519 key/signature,
  and address primitives.
- Implemented restricted deterministic CBOR, domain framing, SHA-256, strict
  Ed25519, structured errors, and minimal reusable traits.
- Added module unit tests, published vectors, malformed/boundary tests, and
  property tests without adding ledger or runtime behavior.

## Unreleased — M1.2 schema decision gate

- Recorded bounded authorization for protocol object schemas and golden
  vectors.
- Audited all eight required objects against the approved CBOR, domain,
  signature, address, ledger, consensus, genesis, and snapshot specifications.
- Confirmed that every object still depends on at least one unresolved
  consensus-critical field or encoding decision.
- Added DEC-Q1-027 and the M1.2 blocker report; no protocol schema crate,
  canonical object bytes, or golden vectors were created.
- Added the authorized DEC-Q1-027 human-decision package with explicit common
  alternatives, eight proposed zero-based schema tables, Merkle/economic/delay
  sub-decisions, security and compatibility consequences, blank approval
  fields, and a grouped approval recommendation.
- Recorded DEC-Q1-027 Session 1 approval for SCHEMA-COMMON-001–019 and
  SCHEMA-GATE-001–003, including the revised three-field ChainId preimage,
  network-class registry, conditional enum activation, format-ceiling limits,
  complete-envelope ObjectId rule, and revised approval order.
- Recorded `CHAIN_ID = 0x0010` as approved pending normative domain-registry
  registration. No schema implementation or Session 2 work was authorized.
- Recorded DEC-Q1-027 Session 2 approval for SCHEMA-COMMON-020–022 and
  SCHEMA-PART-001–005.
- Approved distinct reusable semantic wire types, fixed-width FeeLimit, and
  `RoundNumber(u32)` while preserving the required narrow M1.1 primitive
  correction as unauthorized follow-up work.
- Replaced the proposed ten-field participant shape with the approved
  six-field ParticipantRecordV1 and three-field ParticipantIdentityBodyV1.
- Removed roles, eligibility, weight, and node transport identity from the V1
  wire record because they are derived or belong to another subsystem.
- Recorded `PARTICIPANT_ID = 0x0013` as approved pending normative
  domain-registry registration. No implementation or Session 3 work was
  authorized.
- Recorded DEC-Q1-027 Session 3 approval for SCHEMA-TX-001–006 and
  SCHEMA-ECON-001.
- Replaced the proposed generic twelve-field transaction body with the
  approved nine-field transfer-only TransferBodyV1 and three-field
  SignedTransferV1.
- Removed transaction_type, redundant sender_address, and memo_hash from V1
  consensus bytes; defined TransferId over the complete signed envelope.
- Froze FeeLimit wire shape without approving fee, replay, expiration,
  mempool, ledger, wallet, or execution behavior. No implementation or
  Session 4 work was authorized.
- Recorded DEC-Q1-027 Session 4 approval for SCHEMA-MERKLE-001–005.
- Approved the shared binary indexed-sequence profile with typed empty roots,
  profile-bound leaves/internal nodes, and odd-node promotion; rejected the
  earlier duplicate-last recommendation.
- Registered semantic profile IDs 1–5 in the decision package, with
  TransactionRoot and ParticipantRoot future-only and ReceiptRoot,
  SnapshotChunkRoot, and StateRoot inactive pending their dependencies.
- Recorded owning-schema item-count binding and future independent
  conformance requirements without creating Merkle code or vectors.
- Recorded DEC-Q1-027 Session 5A approval of SCHEMA-BLOCK-001/002: the
  separate canonical header body, exact three-field signed envelope, and
  complete-envelope BlockId rule.
- Approved ParentReferenceV1 and its GENESIS/BLOCK tag registry, with the first
  signed block at height 1 linked to GenesisId and later blocks linked to the
  immediately preceding BlockId.
- Rejected null and zero-sentinel parents plus body-only, partial-envelope,
  undomained, and raw hash identifiers. The full header field table,
  implementation, and Session 5B remain unauthorized.
- Recorded DEC-Q1-027 Session 5B root membership, canonical ownership, and
  count-binding decisions.
- Approved minimal two-field BlockBodyV1 and three-field ParticipantSetV1;
  TransactionCount and ParticipantCount remain distinct derived `u32` values
  and are not serialized.
- Approved required eventual TransactionRoot, pre-H ParticipantRoot, and
  shape-only StateRoot membership; recorded the producer membership
  structural-versus-historical validation split.
- Revised and approved SCHEMA-BLOCK-004/005 to omit ReceiptRoot and generic
  block-body commitment. StateRoot construction remains inactive and blocks
  Header implementation and valid instantiation.
- Recorded DEC-Q1-027 Session 5C approval of SCHEMA-BLOCK-003/006 and the
  exact eleven-field BlockHeaderBodyV1.
- Fixed required protocol_version and RoundNumber fields, the complete V1
  exclusion set, and a typed DelayEvidenceHash commitment to the approved
  five-field DelayEvidenceV1 base shape.
- Preserved DELAY_OUTPUT unchanged; recorded the separate DELAY_EVIDENCE
  domain as pending normative registration and the conditional NONE evidence
  form `[1, 0, 0, h'', h'']`.
- Recorded schema approval separately from blocked implementation and runtime
  activation; no implementation, vectors, Session 6 work, commit, or push was
  authorized.
- Preserved the prohibition on ledger execution, consensus logic, networking,
  wallet, delay, mining, and runtime work.
- Recorded DEC-Q1-M1.1 Gate 1 human decisions RN-G1-001–010 for the
  RoundNumber primitive correction.
- Approved standalone `RoundNumber(u32)` with zero initial value and complete
  Slot independence; classified historical Round and Slot as bounded
  deprecated legacy types.
- Recorded that implementation has not begun, the primitive mismatch remains,
  legacy removal requires a later Gate, and Session 6 remains unauthorized.
- Implemented the authorized Gate 2 standalone `RoundNumber(u32)` primitive
  with `ZERO`, numeric ordering, and standalone shortest canonical CBOR.
- Deprecated historical Round as legacy compatibility-only and Slot as legacy
  non-consensus data while retaining both historical codecs unchanged.
- Added normative construction, boundary, encoding, malformed-input,
  ordering, zero, Slot-independence, and property coverage. No protocol object
  or runtime behavior was migrated.


## Unreleased — executable M1.2 subsets and LOCALNET v0 policy

- Implemented approved identity, participant, transfer, parent, body/set and
  typed sequence-root schemas with independent reference vectors.
- Registered previously approved chain/participant domains and the newly
  approved collision-checked local delay-evidence domain.
- Added local-only atomic ledger accounting with fee=1 and conserved supply,
  fixed-membership two-of-three tally and explicitly local NONE verification.
- Added shared local/CI verification; reduced CBOR allocation from untrusted
  array counts. StateRoot and remaining compound/runtime decisions stay open.
