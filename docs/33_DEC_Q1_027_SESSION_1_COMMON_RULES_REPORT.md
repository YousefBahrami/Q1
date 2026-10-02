# DEC-Q1-027 Session 1 Common Rules Report

Report Version: 0.1.0

Review Date: 2026-07-24

Decision Owner: Yousef Bahrami

Gate Outcome: **APPROVED WITH RECORDED CONDITIONS AND REVISIONS**

Implementation Authorization: **NONE**

Session 2 Authorization: **NONE**

## 1. Scope

Session 1 reviewed and recorded only:

- SCHEMA-COMMON-001 through SCHEMA-COMMON-019;
- SCHEMA-GATE-001 through SCHEMA-GATE-003.

It did not review or approve:

- SCHEMA-COMMON-020 through SCHEMA-COMMON-022;
- ParticipantRecord, TransactionEnvelope, BlockHeader, BlockProposal,
  Attestation, FinalizationCertificate, GenesisManifest, or SnapshotManifest
  schema rows;
- Merkle, economic, delay, remaining domain, or implementation rows;
- source code, crates, codecs, parsers, golden vectors, or runtime behavior.

## 2. Reviewed Decision Record

| Decision ID | Original recommendation | Final human status | Revised normative value | Conditions | Dependencies carried forward |
|---|---|---|---|---|---|
| SCHEMA-COMMON-001 | Fixed bytes32 hash-derived ChainId from `[version, nonce]` under proposed `0x0010` | Approved with revision | Preimage is `[version:u16=1, network_class:u16, creation_nonce:bytes32]`; `CHAIN_ID=0x0010` | Future-public class reserved inactive; HRP/class related only by config validation | normative domain-registry registration |
| SCHEMA-COMMON-002 | Public CSPRNG nonce bytes32 | Approved | exactly 32 public CSPRNG bytes, generated once and included in genesis package | reuse prohibited; tooling warns/rejects known reuse where detectable | genesis tooling |
| SCHEMA-COMMON-003 | Separate network name, ChainId, GenesisId, and HRP | Approved | also separate Network Class; exact API forms recorded | no equality or implicit derivation unless separately specified | API schemas |
| SCHEMA-COMMON-004 | semantic `u16` version at index 0 | Approved | every top-level and normative nested record begins with V1=`1` | shortest CBOR unsigned encoding | per-object versions |
| SCHEMA-COMMON-005 | reject zero/unknown versions and never reuse | Approved | field order/type/presence/meaning changes require a new version | reject before version-specific parsing | parser design |
| SCHEMA-COMMON-006 | schema and protocol versions distinct | Approved | encoding version and behavioral version cannot substitute | both fields only where schema requires | protocol-version registry |
| SCHEMA-COMMON-007 | append-only numeric enum registry | Approved | semantic `u16`, except AddressType `u8`; zero invalid unless `NONE` | unknown rejected; text excluded | later enum assignments |
| SCHEMA-COMMON-008 | transaction type values 1–4 | Approved with condition | TRANSFER active; genesis/reward reserved for distinct schemas; penalty reserved inactive | reserved values rejected by TransactionEnvelopeV1; no execution authorized | Transaction schema sessions |
| SCHEMA-COMMON-009 | sorted unique ParticipantRole array | Approved with condition | producer/validator active; node identity reserved | NODE_IDENTITY inactive pending SCHEMA-PART-003 | ParticipantRecord decision |
| SCHEMA-COMMON-010 | initial hash/attestation/snapshot/compression/delay enums | Approved with conditions | listed values assigned; future values inactive | Delay NONE only under approved profile and cannot bypass mandatory delay | genesis/network profile; delay decision |
| SCHEMA-COMMON-011 | omit ProposalType V1 | Approved | schema version identifies only block-proposal interpretation | new proposal class requires new version/envelope | BlockProposal decision |
| SCHEMA-COMMON-012 | fixed-position explicit null | Approved | only CBOR `0xf6` represents absence | omitted/shortened alternatives prohibited | optional fields per schema |
| SCHEMA-COMMON-013 | strict fixed field count/type | Approved | missing/trailing/null-required/wrong-type rejected | semantic change requires new schema version | decoder design |
| SCHEMA-COMMON-014 | ascending raw-byte sorted sets/maps | Approved with condition | normative encoder/decoder reject unsorted and duplicate keys | non-normative sorting helper allowed but cannot change normative behavior | per-collection sort keys |
| SCHEMA-COMMON-015 | proposed collection classes and ceilings | Approved with condition | 65,535 is only absolute format ceiling; ParticipantRole V1 max 3 | lower runtime/untrusted limits required; no allocation from ceiling alone | object-specific resource limits |
| SCHEMA-COMMON-016 | distinct Body/signing/envelope/complete/ID forms | Approved | signature excluded from its own domain-framed canonical Body | envelope remains `[Body, algorithm, signature]` | signed object schemas |
| SCHEMA-COMMON-017 | ObjectId hashes complete signed envelope | Approved with condition | signature is part of official signed-object ID | future BodyHash is non-official and needs separate domain/approval | ID-domain rows |
| SCHEMA-COMMON-018 | no duplicate signer key | Approved | key appears once in Body or resolves from approved registry | envelope remains body/algorithm/signature only | object verification model |
| SCHEMA-COMMON-019 | no serialized self ID | Approved | IDs only in external indexes/APIs/references/metadata | zero-substitution prohibited | every object schema |
| SCHEMA-GATE-001 | grouped approval | Approved | do not approve all eight atomically | group boundaries remain gates | later sessions |
| SCHEMA-GATE-002 | original A–E group order | Revised and approved | Common; reusable/participant; transaction; Merkle; header; attestation; proposal; certificate; genesis; snapshot/delay | Attestation follows stable BlockId | Session 2 and later gates |
| SCHEMA-GATE-003 | separate Merkle decision before header | Approved | approve Merkle before commitment-bearing objects | no Merkle implementation authorized | separate Merkle package |

## 3. Revised ChainId Rule

Approved preimage:

```text
ChainIdentityPreimageV1 = [
    schema_version: u16 = 1,
    network_class: u16,
    creation_nonce: bytes32
]
```

Approved active/reserved registry:

| Value | Network Class | State |
|---:|---|---|
| `0x0001` | `LOCALNET` | active |
| `0x0002` | `PRIVATE_TESTNET` | active |
| `0x0003` | `RESEARCH` | active |
| `0x0004` | `RESERVED_FUTURE_PUBLIC` | reserved inactive |

Derivation:

```text
ChainId = SHA-256(
    Q1DomainFrameV1(
        CHAIN_ID,
        CanonicalCBOR(ChainIdentityPreimageV1)
    )
)
```

`CHAIN_ID = 0x0010` is approved pending normative registration. The approved
domain registry was not edited in Session 1 because this authorization was
limited to recording decisions and explicitly preserved a possible separate
registration gate.

## 4. Conditions Carried Forward

1. Lower object-specific and operational resource limits remain required.
2. `NODE_IDENTITY` remains inactive pending SCHEMA-PART-003.
3. Reserved TransactionType values remain invalid in TransactionEnvelopeV1.
4. DelayEngine `NONE` requires an applicable approved profile.
5. A BodyHash is not an official ObjectId and cannot be invented.
6. `CHAIN_ID=0x0010` must be registered normatively before implementation.
7. BlockId must stabilize before Attestation approval or implementation.
8. A separate Merkle decision must precede BlockHeader.
9. All object-schema approval fields remain blank.

## 5. Files Changed

- `docs/32_DEC_Q1_027_SCHEMA_DECISION_PACKAGE.md`;
- `docs/33_DEC_Q1_027_SESSION_1_COMMON_RULES_REPORT.md`;
- `OPEN_DECISIONS.md`;
- `CHANGELOG.md`;
- `PROJECT.md`;
- `docs/22_REQUIREMENTS_TRACEABILITY_SKELETON.md`;
- `docs/01_GLOSSARY.md`.

The previously uncommitted M1.2 review files and status updates remain part of
the same pending documentation change set.

## 6. No-Implementation Confirmation

No source code, crate, codec, parser, schema implementation, golden vector,
Merkle logic, ledger behavior, consensus behavior, network behavior, wallet
logic, or runtime service was created or changed.

The normative cryptographic domain registry was not modified.

## 7. Recommendation for Session 2

Wait for explicit Session 2 authorization.

When authorized, Session 2 should follow the revised gate and review:

1. SCHEMA-COMMON-020 — reusable fixed-width/type table;
2. SCHEMA-COMMON-021 — distinct FeeLimit type;
3. SCHEMA-COMMON-022 — RoundNumber and the M1.1 Round mismatch;
4. SCHEMA-PART-001 through SCHEMA-PART-005 — ParticipantRecord.

Do not review TransactionEnvelope until the reusable-type and participant
decisions required by the approved order are complete.
