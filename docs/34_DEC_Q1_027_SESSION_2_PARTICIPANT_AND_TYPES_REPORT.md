# DEC-Q1-027 Session 2 Participant and Types Report

Report Version: 0.1.0

Review Date: 2026-07-24

Decision Owner: Yousef Bahrami

Gate Outcome: **APPROVED WITH RECORDED CONDITIONS AND REVISIONS**

Implementation Authorization: **NONE**

Session 3 Authorization: **NONE**

## 1. Scope

Session 2 reviewed and recorded only:

- SCHEMA-COMMON-020 through SCHEMA-COMMON-022;
- SCHEMA-PART-001 through SCHEMA-PART-005.

It did not authorize source code, crates, codecs, parsers, golden vectors,
ParticipantRecord structs, the required M1.1 primitive correction, networking
identity, ledger or consensus behavior, wallet logic, runtime services, or
Session 3.

## 2. Reviewed Decision Record

| Decision ID | Original proposal | Final human status | Revised normative value | Conditions | Dependencies carried forward |
|---|---|---|---|---|---|
| SCHEMA-COMMON-020 | Reusable fixed-width/type table | Approved with condition | ChainId/ParticipantId/typed hashes bytes32; AddressEnvelope bytes36; Ed25519 key/signature bytes32/64; Amount/FeeLimit fixed bytes16; Weight/Nonce/Height/ByteCount u64; RoundNumber u32; CandidateIndex u16 | semantic types remain distinct; no implicit conversions; public APIs preserve boundaries; Weight excluded from ParticipantRecordV1 | future implementation authorization |
| SCHEMA-COMMON-021 | Distinct FeeLimit with Amount wire width | Approved | `FeeLimit(u128)` is only `0x50 || to_be_bytes()`: fixed bytes16, unsigned, big-endian, left-zero-padded | checked arithmetic; no float or Amount conversion; no fee behavior approved | future type implementation authorization |
| SCHEMA-COMMON-022 | RoundNumber u32 and M1.1 mismatch review | Approved with required M1.1 correction | consensus context is `(Height, RoundNumber)`; RoundNumber starts at 0 and uses shortest CBOR uint | Slot cannot substitute for Height; no timing/transition behavior | separately authorized narrow M1.1 correction before M1.2 schema implementation |
| SCHEMA-PART-001 | Multi-role record with distinct role keys and serialized roles | Approved with revision | independent nullable producer/validator slots; roles derived from key presence; no roles array | at least one key; two present keys distinct | strict Ed25519 profile |
| SCHEMA-PART-002 | ParticipantId over producer, validator, and optional node identity | Approved with revision | hash only canonical three-field ParticipantIdentityBodyV1 under `PARTICIPANT_ID=0x0013` | activation and record fields excluded; domain pending normative registration | domain registration |
| SCHEMA-PART-003 | Choose required, optional, or excluded node key | Approved — Option C | node transport identity excluded from ParticipantRecordV1 | authenticated P2P remains possible in a networking-specific schema; NODE_IDENTITY role inactive | later networking schema |
| SCHEMA-PART-004 | Serialized roles and equal weight | Approved with revision | producer and validator active role weight is protocol-defined as 1; no roles or weight field | non-unit/asymmetric weighting needs separate decision and protocol/schema version | later weighting decision, if any |
| SCHEMA-PART-005 | Activation/deactivation representation | Approved with condition | required activation Height and nullable deactivation Height; activity derived by half-open interval | deactivation strictly greater than activation; genesis-active height is 0; no eligibility flag | future registry implementation authorization |

## 3. Approved Canonical Shapes

```text
ParticipantIdentityBodyV1 = [
    schema_version: u16 = 1,
    producer_public_key_or_null: Ed25519PublicKey | null,
    validator_public_key_or_null: Ed25519PublicKey | null
]
```

```text
ParticipantRecordV1 = [
    schema_version: u16 = 1,
    participant_id: ParticipantId,
    producer_public_key_or_null: Ed25519PublicKey | null,
    validator_public_key_or_null: Ed25519PublicKey | null,
    activation_height: Height,
    deactivation_height_or_null: Height | null
]
```

`participant_id` must be recomputed from the canonical identity body. Missing
role keys use canonical CBOR null. Unknown or trailing fields are rejected.

## 4. Derived Fields Removed

- `roles` is derived from the presence of each role-specific key.
- `eligible` is replaced by the deterministic activation interval.
- `weight` is fixed by V1 protocol rule at one for each active role.
- `node_identity_public_key` belongs to authenticated networking rather than
  consensus participant identity.

Removing these values prevents contradictory representations of the same
participant and keeps transport identity outside consensus identity.

## 5. Identity, Record Hash, and Activity

```text
ParticipantId =
    Q1HashV1(
        PARTICIPANT_ID,
        CanonicalCBOR(ParticipantIdentityBodyV1)
    )
```

```text
ParticipantRecordHash =
    Q1HashV1(
        PARTICIPANT_RECORD,
        CanonicalCBOR(ParticipantRecordV1)
    )
```

ParticipantId identifies the role-key body. ParticipantRecordHash also commits
to activation scheduling. They are not interchangeable and neither
self-derived hash is serialized into the bytes used to derive itself.

A participant is active at height `H` exactly when:

```text
activation_height <= H
and
(deactivation_height is null or H < deactivation_height)
```

## 6. Round and Slot Follow-up

Before any M1.2 schema implementation, separately authorized narrow M1.1 work
must:

1. introduce the distinct `RoundNumber(u32)` primitive;
2. deprecate or reclassify Slot/composite Round for consensus use;
3. preserve `(Height, RoundNumber)` as the consensus round context.

Session 2 did not authorize this correction or any consensus timing,
scheduling, or round-transition rule.

## 7. Domain Registration Status

`PARTICIPANT_ID = 0x0013` is approved pending normative registration. The
normative domain registry was not edited. No implementation may assume the
assignment is active before the separate registration gate is satisfied.

## 8. Dependencies Carried into Session 3

- normative registration of approved domains `0x0010` and `0x0013`;
- separately authorized M1.1 RoundNumber correction;
- object-specific resource limits below the format ceiling;
- TransactionEnvelope field, signing, identity, and rejection decisions;
- an explicit Session 3 authorization.

No TransactionEnvelope decision row was reviewed in Session 2.

## 9. Files Changed

- `docs/32_DEC_Q1_027_SCHEMA_DECISION_PACKAGE.md`;
- `docs/34_DEC_Q1_027_SESSION_2_PARTICIPANT_AND_TYPES_REPORT.md`;
- `OPEN_DECISIONS.md`;
- `CHANGELOG.md`;
- `PROJECT.md`;
- `docs/01_GLOSSARY.md`;
- `docs/05_CONSENSUS.md`;
- `docs/22_REQUIREMENTS_TRACEABILITY_SKELETON.md`.

Previously uncommitted M1.2 decision-gate documentation remains part of the
same pending working-tree change set.

## 10. No-Implementation Confirmation

No source code, crate, codec, parser, validator, struct, golden vector, Merkle
logic, networking identity implementation, ledger behavior, consensus
behavior, wallet logic, or runtime service was created or changed.

The normative cryptographic domain registry was not modified.

## 11. Recommendation for Session 3

Wait for explicit Session 3 authorization. If authorized under the approved
gate order, review only TransactionEnvelope and its dependencies. Do not begin
Merkle or later object groups automatically.
