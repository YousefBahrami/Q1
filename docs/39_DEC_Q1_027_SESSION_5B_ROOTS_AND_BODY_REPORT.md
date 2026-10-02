# DEC-Q1-027 Session 5B Roots and Body Report

Report Version: 0.1.0

Review Date: 2026-07-25

Decision Owner: Yousef Bahrami

Gate Outcome: **APPROVED WITH RECORDED CONDITIONS AND IMPLEMENTATION
BLOCKERS**

Implementation Authorization: **NONE**

Session 5C Authorization: **NONE**

## 1. Scope

Session 5B recorded decisions for:

- TransactionRoot and ParticipantRoot Header membership;
- canonical BlockBodyV1 and ParticipantSetV1;
- derived TransactionCount and ParticipantCount;
- producer-membership validation;
- ReceiptRoot omission;
- StateRoot field shape and hard implementation blocker;
- generic block-body commitment omission;
- semantic root and count boundaries.

It did not finalize BlockHeaderBodyV1 field indexes, order, or array length.
No implementation, parser, codec, tree, proof, test, or vector was authorized.

## 2. Alternatives Reviewed and Final Decisions

| Topic | Alternatives reviewed | Final human decision |
|---|---|---|
| transaction commitment | direct count/root; root with body-derived count; body commitment; root plus generic commitment | required TransactionRoot; count derived from canonical BlockBodyV1 |
| canonical body | minimal transfer body; serialized count; external array; reserved multi-section body | exact minimal two-field BlockBodyV1 |
| participant commitment | direct count/root; root with ParticipantSet-derived count; set ID; committee root | required ParticipantRoot over active pre-H ParticipantSetV1 |
| ReceiptRoot | omit; required empty; nullable; opaque | omit entirely; SCHEMA-BLOCK-004 revised and approved |
| StateRoot | omit; required typed shape; nullable; generic | required typed bytes32 field shape; construction inactive and Header implementation blocked |
| generic body commitment | omit; retain beside root; replace root; multi-section body | omit entirely; SCHEMA-BLOCK-005 revised and approved |

## 3. BlockBodyV1

```text
BlockBodyV1 = [
    schema_version: u16 = 1,
    transfers: array<SignedTransferV1>
]
```

Exact outer array length is two. Transfers are an ordered protocol sequence
and their order is normative. Missing, unknown, or trailing fields are
rejected.

V1 contains no receipt, evidence, metadata, reserved section, or future
placeholder. No BlockBodyId, separate body hash domain, or generic body
commitment is approved. Future non-transfer content requires a new schema
version or another explicitly approved schema.

## 4. TransactionRoot and TransactionCount

TransactionRoot is a required eventual BlockHeaderBodyV1 member with final
index deferred. It:

- is a distinct bytes32 semantic type;
- uses profile `0x0001`;
- hashes exact CanonicalCBOR(SignedTransferV1) leaves;
- preserves exact BlockBodyV1 order;
- neither sorts nor deduplicates;
- distinguishes equal canonical transfers at different indexes.

Duplicate TransferId policy remains a future validation decision.

```text
TransactionCount =
    len(BlockBodyV1.transfers) =
    TransactionRoot leaf count
```

TransactionCount is a distinct semantic `u32` for APIs, validation,
diagnostics, and limits. It is not serialized in Header or BlockBody. Zero is
valid and maps to the typed empty TransactionRoot. The value must fit `u32`;
no runtime limit is approved.

## 5. ParticipantSetV1

```text
ParticipantSetV1 = [
    schema_version: u16 = 1,
    reference_height: Height,
    participants: array<ParticipantRecordV1>
]
```

Exact outer array length is three. `reference_height=H` means the set governs
validation of signed block H and derives from finalized state before executing
H.

The participants array:

- is non-empty;
- contains records active at H;
- is sorted by ascending raw ParticipantId;
- rejects duplicate ParticipantIds;
- adds no role, weight, transport identity, metadata, stake, HDD, AI,
  reputation, or committee-selection field.

ParticipantRoot is computed only from this array. Reference height supplies
the owning object's temporal context. No ParticipantSetId or new domain is
approved.

## 6. ParticipantRoot and ParticipantCount

ParticipantRoot is a required eventual BlockHeaderBodyV1 member with final
index deferred. It:

- is a distinct bytes32 semantic type;
- uses profile `0x0003`;
- commits to exact CanonicalCBOR(ParticipantRecordV1) leaves;
- uses ascending raw ParticipantId order;
- represents the active set governing validation at H from finalized pre-H
  state;
- is not the set produced by H and does not automatically govern H+1.

```text
ParticipantCount =
    len(ParticipantSetV1.participants) =
    ParticipantRoot leaf count
```

ParticipantCount is a distinct semantic `u32` and is not serialized. Zero is
invalid. It is not implicitly convertible to TransactionCount or a generic
integer. Runtime limits require a separate decision.

## 7. Producer-Membership Validation Split

Approved invariant:

```text
producer_id identifies an active PRODUCER
inside ParticipantSetV1 for Header.height,
whose participants are committed by ParticipantRoot
```

Structural validation checks canonical Header and ParticipantSet forms plus
typed producer_id and ParticipantRoot.

Historical/state validation later verifies:

1. ParticipantSet reference height equals Header height;
2. reconstructed ParticipantRoot equals the Header field;
3. producer_id exists in the set;
4. the record has an active producer role key at H;
5. the activation interval includes H;
6. the producer public key resolves from that ParticipantRecordV1;
7. SignedBlockHeaderV1 verifies with that key.

This decision does not define selection, election, committee, quorum, or
finality behavior.

## 8. ReceiptRoot Decision

SCHEMA-BLOCK-004 is revised and approved: ReceiptRoot is omitted entirely from
BlockHeaderBodyV1.

V1 contains no required empty root, nullable root, opaque bytes32, or reserved
receipt field. The profile remains inactive because receipt schema,
transaction correspondence, ordering, and empty semantics do not exist.
Receipt-enabled headers require a new Header schema version after their
dependencies and vectors are approved.

## 9. StateRoot Blocker

StateRoot is a required eventual BlockHeaderBodyV1 member with final index
deferred. Approval is limited to its distinct bytes32 field shape.

| Layer | Status |
|---|---|
| field shape | approved |
| construction algorithm | not approved |
| profile `0x0005` | reserved inactive |
| runtime activation | blocked |
| BlockHeaderV1 implementation and valid instantiation | blocked until state-model approval |

Arbitrary bytes32 values are not valid StateRoots. No active constructor or
validity claim is permitted. The separate state decision must define canonical
keys/values, ordering, duplicates, empty state, updates, inclusion and
non-inclusion, proofs, persistence, and independent vectors.

Nullable, absent, and generic opaque state commitments are rejected for V1.

## 10. Generic Block-Body Commitment

SCHEMA-BLOCK-005 is revised and approved:
`block_body_commitment` is omitted entirely.

Signed transfers are the only approved V1 body content, and TransactionRoot
already commits to their exact ordered sequence. A second generic commitment
would be redundant or ambiguous and could introduce disagreement states. No
new body domain is allocated.

## 11. Semantic Boundaries

Approved documentation-level types:

| Type | Representation and state |
|---|---|
| TransactionCount | derived `u32`, not serialized |
| ParticipantCount | derived `u32`, not serialized |
| TransactionRoot | bytes32, profile `0x0001` |
| ParticipantRoot | bytes32, profile `0x0003` |
| ReceiptRoot | bytes32 type reserved inactive; absent from Header V1 |
| StateRoot | required bytes32 field shape; construction inactive |

No implicit conversions are permitted among roots, counts, or generic
representations. Generic MerkleHash cannot bypass these semantic boundaries.
Inactive roots expose no active constructor or validity claim.

## 12. Approved Header Membership to Date

Approved eventual BlockHeaderBodyV1 members, without final indexes/order or
array length:

```text
schema_version
chain_id
height
parent_reference
producer_id
transaction_root
participant_root
state_root
```

The already approved signed form remains:

```text
SignedBlockHeaderV1 = [
    BlockHeaderBodyV1,
    signature_algorithm,
    producer_signature
]
```

The complete Header schema is not finalized.

## 13. Dependencies Carried into Session 5C

- remaining Header-field membership decisions;
- final field order and array length;
- required narrow M1.1 RoundNumber correction;
- duplicate TransferId validation policy;
- complete StateRoot state-model decision and vectors;
- operational count and resource limits;
- implementation and conformance authorization.

Session 5C is not automatically authorized.

## 14. Files Changed

- `docs/32_DEC_Q1_027_SCHEMA_DECISION_PACKAGE.md`;
- `docs/39_DEC_Q1_027_SESSION_5B_ROOTS_AND_BODY_REPORT.md`;
- `OPEN_DECISIONS.md`;
- `CHANGELOG.md`;
- `PROJECT.md`;
- `README.md`;
- `docs/01_GLOSSARY.md`;
- `docs/03_ARCHITECTURE.md`;
- `docs/04_LEDGER_AND_TRANSACTIONS.md`;
- `docs/05_CONSENSUS.md`;
- `docs/14_TEST_PLAN.md`;
- `docs/22_REQUIREMENTS_TRACEABILITY_SKELETON.md`.

The Session 5B engineering brief remains a non-normative review artifact.

## 15. No-Implementation Confirmation

No source code, crate, schema implementation, Merkle implementation, parser,
codec, validator, test, vector, ledger/state logic, receipt logic, consensus
behavior, or runtime behavior was created or changed.

The cryptographic domain registry was not edited. Session 5C was not started.
