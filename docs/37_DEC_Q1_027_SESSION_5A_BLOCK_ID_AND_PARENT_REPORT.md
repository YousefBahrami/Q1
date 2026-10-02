# DEC-Q1-027 Session 5A Block ID and Parent Report

Report Version: 0.1.0

Review Date: 2026-07-25

Decision Owner: Yousef Bahrami

Gate Outcome: **APPROVED WITH RECORDED PARENT REVISION**

Implementation Authorization: **NONE**

Session 5B Authorization: **NONE**

## 1. Scope

Session 5A reviewed and recorded only:

- SCHEMA-BLOCK-001 signed-header form;
- SCHEMA-BLOCK-002 BlockId definition;
- GenesisManifest and signed-block height boundary;
- ParentReferenceV1 and ParentKind;
- the related rejected V1 alternatives.

The complete BlockHeaderBodyV1 field table and array length remain
unapproved. No other Session 5 topic was reviewed or decided.

## 2. Alternatives Reviewed

| Area | Alternatives |
|---|---|
| signed-header form | A: separate canonical Body and signed envelope; B: signature inside Body; C: signature outside canonical signed object |
| BlockId | I1: complete signed envelope; I2: Body only; I3: undomained digest; I4: partial envelope |
| parent boundary | P1: nullable BlockId; P2: all-zero sentinel; P3: tagged GenesisId-or-BlockId; P4: separate nullable slots |

## 3. Final Human Decisions

- SCHEMA-BLOCK-001: **APPROVED — ALTERNATIVE A**.
- SCHEMA-BLOCK-002: **APPROVED — ALTERNATIVE I1**.
- Parent representation: **APPROVED — P3 WITH THE RECORDED HEIGHT-ONE
  BOUNDARY**.
- Alternatives B, C, I2, I3, I4, P1, P2, and P4 are rejected for V1.

## 4. SignedBlockHeaderV1

```text
BlockHeaderBodyV1 = [
    schema_version,
    ...fields approved in later Session 5 decisions
]
```

```text
SignedBlockHeaderV1 = [
    body: BlockHeaderBodyV1,
    signature_algorithm: SignatureAlgorithm,
    producer_signature: Ed25519Signature
]
```

SignedBlockHeaderV1 has exact array length three. Unknown, missing, or trailing
fields are rejected. The algorithm must be supported by the applicable schema
and protocol version.

```text
BlockHeaderSigningPayload =
    Q1DomainFrameV1(
        BLOCK_HEADER_SIGNING,
        CanonicalCBOR(BlockHeaderBodyV1)
    )
```

The producer signature is excluded from its own signing payload. Algorithm and
signature appear only in the signed envelope.

## 5. BlockId

```text
BlockId =
    Q1HashV1(
        BLOCK_ID,
        CanonicalCBOR(SignedBlockHeaderV1)
    )
```

BlockId is a distinct semantic bytes32 type. Its preimage includes signature
algorithm and producer signature. It is not serialized inside
BlockHeaderBodyV1 or SignedBlockHeaderV1. Typed public APIs expose BlockId
rather than an interchangeable generic hash. Strict deterministic Ed25519
semantics remain required.

## 6. GenesisManifest and Height Boundary

GenesisManifest is an independent protocol object and is not
SignedBlockHeaderV1.

- the first SignedBlockHeaderV1 has height 1;
- height 0 is invalid for SignedBlockHeaderV1;
- the first signed block explicitly references the governing GenesisId.

## 7. ParentReferenceV1

```text
ParentReferenceV1 = [
    schema_version: u16 = 1,
    parent_kind: ParentKind,
    parent_id: bytes32
]
```

Exact array length is three.

| Value | ParentKind | Semantic interpretation of `parent_id` |
|---:|---|---|
| `0x0001` | `GENESIS` | GenesisId |
| `0x0002` | `BLOCK` | BlockId |

Version zero, unknown versions, and unknown kinds are rejected. ParentKind
assignments are append-only and never reused. ParentReferenceV1 must occupy one
fixed field in the later approved BlockHeaderBodyV1; Session 5A does not fix
that field's final index.

## 8. Structural and Historical Invariants

Approved boundary:

```text
height == 1
iff parent_kind == GENESIS
and parent_id == the GenesisId governing this chain
```

```text
height > 1
iff parent_kind == BLOCK
and parent_id == the BlockId at height - 1
```

Structural parsing validates the exact ParentReferenceV1 schema, version,
kind, and bytes32 width.

Historical-chain validation verifies the governing GenesisId or referenced
BlockId and its exact height relationship.

The following are invalid:

- signed-block height 0;
- GENESIS parent above height 1;
- BLOCK parent at height 1;
- null or all-zero sentinel parent;
- wrong-chain GenesisId;
- BlockId from any height other than height minus one;
- untagged GenesisId stored as BlockId.

## 9. Rejected Alternatives

- B: signature embedded in Body;
- C: signature outside the canonical signed object;
- I2: body-only BlockId;
- I3: undomained or raw SHA-256 digest;
- I4: partial-envelope BlockId;
- P1: nullable BlockId parent;
- P2: all-zero BlockId sentinel;
- P4: separate nullable GenesisId and BlockId slots.

They are rejected because they introduce semantic type confusion, sentinel
ambiguity, permanent redundant fields, divergence from the approved
signed-object identity rule, or loss of domain separation.

## 10. Dependencies Carried into Session 5B

- the complete BlockHeaderBodyV1 field membership, order, and length remain
  undecided;
- the final fixed index of ParentReferenceV1 remains undecided;
- the required narrow M1.1 RoundNumber correction remains separately gated;
- all other Session 5 decision rows remain blank;
- implementation requires separate explicit authorization.

## 11. Files Changed

- `docs/32_DEC_Q1_027_SCHEMA_DECISION_PACKAGE.md`;
- `docs/37_DEC_Q1_027_SESSION_5A_BLOCK_ID_AND_PARENT_REPORT.md`;
- `OPEN_DECISIONS.md`;
- `CHANGELOG.md`;
- `PROJECT.md`;
- `README.md`;
- `docs/01_GLOSSARY.md`;
- `docs/03_ARCHITECTURE.md`;
- `docs/04_LEDGER_AND_TRANSACTIONS.md`;
- `docs/05_CONSENSUS.md`;
- `docs/22_REQUIREMENTS_TRACEABILITY_SKELETON.md`.

The earlier Session 5 and Session 5A review briefs remain non-normative
engineering-review artifacts.

## 12. No-Implementation Confirmation

No source code, crate, struct, codec, parser, validator, vector, test, runtime
behavior, or protocol implementation was created or changed.

Session 5B was not started.
