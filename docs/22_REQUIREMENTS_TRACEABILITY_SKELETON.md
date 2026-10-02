Q1 Requirements Traceability Skeleton

Version: 0.2.0
Status: Draft — identifier migration applied; semantic review pending

⸻

1. Purpose

This file defines the minimum traceability record. It is not yet the
authoritative matrix. The structural identifier collision has been removed,
but the migration and unnamed normative clauses still require human review.

⸻

2. Required Fields

| Field | Meaning |
|---|---|
| Requirement ID | Globally unique stable identifier |
| Canonical owner | One normative document and section |
| Requirement text hash | Hash after the canonical text is frozen |
| Classification | protocol, security, implementation, test, research, operations |
| Consensus critical | yes/no |
| Decision dependencies | OPEN_DECISIONS IDs |
| ADR dependencies | Approved ADR IDs |
| Module/interface | Planned implementation owner |
| Verification | test, analysis, simulation, review, inspection |
| Test IDs | Automated/manual evidence identifiers |
| Milestone | Earliest authorized milestone |
| Status | draft, blocked, approved, implemented, verified, retired |
| Notes | Limitations and supersession links |

⸻

3. ID Migration Record

The complete applied table is:

`docs/reports/consistency/REQUIREMENT_ID_MIGRATION.md`

Applied migration statistics:

* pre-existing definitions mapped: 389;
* definitions renamed: 238;
* duplicate definition identifiers after migration: 0;
* new post-migration system requirements: Q1-SYS-232 and Q1-SYS-233.

Historical old identifiers remain searchable in the migration record.

⸻

4. Traceability Rows

| Requirement ID | Canonical owner | Class | Consensus critical | Decisions | ADRs | Module | Verification | Tests | Milestone | Status |
|---|---|---|---|---|---|---|---|---|---|---|
| Q1-LTX-010 | docs/04, address derivation | protocol | yes | DEC-Q1-003 | ADR-0003, ADR-0004, ADR-0005 | research/pre_m1_conformance | three-language vectors + strict rejection | PREM1-ADR-001–012 | Pre-M1 | human parameter decision pending |
| Q1-LTX-014 | docs/04, canonical bytes | protocol | yes | DEC-Q1-002 | ADR-0002 | research/pre_m1_conformance | 10 canonical + 14 rejection vectors in Rust/Node | PREM1-CBOR-001–020 | Pre-M1 | human parameter decision pending |
| Q1-LTX-015 | docs/04, ambiguous encoding rejection | security | yes | DEC-Q1-002 | ADR-0002 | protocol/serialization | malformed corpus + fuzzing | PRE-M1-CBOR-REJECT-V1 | Pre-M1/M1 | proposed profile |
| Q1-LTX-016 | docs/04, serialization format | protocol | yes | DEC-Q1-002 | ADR-0002 | protocol/serialization | profile review + conformance | PRE-M1-CBOR-V1 | Pre-M1 | decided; profile review pending |
| Q1-LTX-024 | docs/04, fixed-width Amount | protocol | yes | DEC-Q1-002 | ADR-0002 | crates/q1-primitives/amount | canonical/rejection/property tests | M1-PRIM-AMOUNT-001 | M1.1 | approved; implementation under review |
| M1.2 schema gate | docs/31, eight protocol objects | protocol | yes | DEC-Q1-027 and object-specific dependencies | ADR-0002–0005 | crates/q1-protocol-types (blocked) | approved schema tables + cross-language golden vectors | not created | M1.2 | blocked pending normative decisions |
| DEC-Q1-027 Session 1 | docs/32–33, common schema rules | protocol | yes | DEC-Q1-027 | ADR-0002–0005 | no implementation authorized | human decision-record inspection | SCHEMA-COMMON-001–019; SCHEMA-GATE-001–003 | M1.2 decision gate | approved with recorded conditions; ChainId domain pending registration |
| DEC-Q1-027 Session 2 | docs/32, docs/34, reusable types and ParticipantRecord | protocol | yes | DEC-Q1-027 | ADR-0002–0005 | no implementation authorized | human decision-record inspection | SCHEMA-COMMON-020–022; SCHEMA-PART-001–005 | M1.2 decision gate | approved with conditions; domains pending registration; M1.1 RoundNumber correction pending |
| DEC-Q1-027 Session 3 | docs/32, docs/35, TransferBodyV1 and SignedTransferV1 | protocol | yes | DEC-Q1-027 | ADR-0002–0005 | no implementation authorized | human decision-record inspection | SCHEMA-TX-001–006; SCHEMA-ECON-001 | M1.2 decision gate | approved with revisions; transfer schema fixed; behavior and implementation unauthorized |
| DEC-Q1-027 Session 4 | docs/32, docs/36, Merkle indexed-sequence profile | protocol | yes | DEC-Q1-027, DEC-Q1-009 | ADR-0002–0005 | no implementation authorized | human decision-record inspection; future cross-language vectors | SCHEMA-MERKLE-001–005 | M1.2 decision gate | approved with conditions; TransactionRoot/ParticipantRoot future-only; other profiles inactive |
| DEC-Q1-027 Session 5A | docs/32, Session 5A report, signed header identity and parent | protocol | yes | DEC-Q1-027 | ADR-0002–0005 | no implementation authorized | human decision-record inspection | SCHEMA-BLOCK-001–002; ParentReferenceV1 | M1.2 decision gate | envelope, BlockId, parent tag, and height boundary approved; full body table pending |
| DEC-Q1-027 Session 5B | docs/32, docs/39, roots, canonical body, and count binding | protocol | yes | DEC-Q1-027, DEC-Q1-009 | ADR-0002–0005 | no implementation authorized | human decision-record inspection; future conformance | SCHEMA-BLOCK-004–005; BlockBodyV1; ParticipantSetV1 | M1.2 decision gate | root membership/body/count semantics approved; StateRoot hard blocker; final Header deferred to Session 5C |
| DEC-Q1-027 Session 5C | docs/32, docs/41, final Header and delay commitment | protocol | yes | DEC-Q1-027 | ADR-0002–0005 | no implementation authorized | human decision-record inspection; future three-language conformance | SCHEMA-BLOCK-003; SCHEMA-BLOCK-006; BlockHeaderBodyV1; DelayEvidenceV1 | M1.2 decision gate | 11-field schema approved; implementation and activation blocked |
| DEC-Q1-M1.1 Gates 1–3 | docs/43–44, RoundNumber primitive correction and closure | protocol | yes | DEC-Q1-027; RN-G1-001–010 | ADR-0002 | crates/q1-primitives/round | unit, boundary, malformed, canonical-vector, ordering, independence, and property tests | RoundNumber; legacy Round; legacy Slot | M1.1 correction gate | Gate 1 decisions closed; Gate 2 implementation committed as `e85f8a53a42ea2e40696a2e99b8bc554f9072429`; Gate 3 code, test, scope, and documentation conformance passed; DEC-Q1-M1.1 closed by final human decision; protocol objects, runtime, Session 6, and legacy removal unauthorized |
| Q1-SYS-176 | docs/02, unauthorized balance creation | security | yes | DEC-Q1-006 | ADR-0012 candidate | ledger/supply | invariant/property tests | TBD | M2 | draft |
| Q1-SYS-232 | docs/02, exact genesis supply | economics | yes | DEC-Q1-006 | ADR-0012 candidate | genesis/ledger | schema + invariant tests | Q1-TST-LED-0001 | M0/M2 | approved requirement |
| Q1-SYS-233 | docs/02, private registry | consensus | yes | DEC-Q1-004 | ADR-0010 candidate | genesis/consensus | schema + selection tests | Q1-TST-CON-0001 | M0/M6 | approved requirement |
| Q1-SYS-157 | docs/02, AI outside consensus | architecture | yes | none known | ADR-0011 candidate | ai_observer boundary | integration isolation test | TBD | M10 | draft |
| Q1-SYS-071 | docs/02, HDD is an experimental plugin | architecture | yes | DEC-Q1-010 | ADR-0011 candidate | hdd_lab adapter | configuration equivalence test | TBD | M8 | draft |

⸻

5. Traceability Gate

The baseline may become authoritative only when:

1. every normative ID is unique;
2. every row has one canonical owner;
3. every consensus-critical row has a verification method;
4. all decision and ADR dependencies are linked;
5. superseded identifiers remain searchable;
6. automated validation detects duplicates and broken document references.

Current limitation: many normative MUST/SHALL statements are section-scoped
rather than individually registered. They must be inventoried without changing
their semantics before full traceability can be claimed.


## 2026-10-01 implementation update

Earlier session rows above preserve their historical gate state. Current
implementation authority is PROJECT.md and the bounded local profile.

| Approved basis | Implementation | Verification | Remaining boundary |
|---|---|---|---|
| COMMON-001–022, PART-001–005 | q1-protocol-types chain/types/participant | identity tests, independent TSV vectors | registry provenance is runtime validation |
| TX-001–006, ECON-001 | q1-protocol-types transfer/address | transfer and golden tests, Node Ed25519 | structural decode is not execution |
| MERKLE-001–005, Session 5B owners | q1-protocol-types merkle/block_body | independent 0/1/2/3/5-leaf tests | state/receipt/snapshot profiles inactive |
| Session 5A parent reference | q1-protocol-types parent | exact bytes and contextual parent tests | authenticated history supplied by caller |
| LOCALNET v0 user decisions | q1-localnet ledger/quorum; protocol-types delay | accounting/property/policy/delay tests | no certificates, StateRoot or four-node runtime |

## 2026-10-02 LOCALNET_V0 implementation update

The latest human instruction authorizes local compound schemas and full state;
it does not close generic schema/protocol decisions in historical rows above.

| Approved basis | Implementation | Verification | Remaining boundary |
|---|---|---|---|
| Full state commitment, sorted complete consensus state | q1-localnet state/genesis | state tests, 13 frozen vectors, independent Python initial-state/hash reproduction | no partial proofs or final Mainnet tree |
| Session 5A–5C header + authorized local compound schemas | q1-localnet block | block tests: roots, parent, signatures, mixed/duplicate/unknown votes, 2-of-3, rollback | general consensus fault model remains open |
| Atomic persistence and restart | q1-localnet store | recovery tests: replay/tampering/write failure/locking/reservations | full archive rewrite, bounded local storage |
| Fixed producer/three voters and approved offline acceptance | q1-node; scripts/localnet_acceptance.py | real four-process kill/restart/catch-up, same root, 1-of-3 rejection, durable anti-equivocation | loopback only, no leader rotation or public deployment |
