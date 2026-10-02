# DEC-Q1-027 Session 5A — Block ID and Parent Review

Status: **DOCUMENTATION REVIEW ONLY — NO DECISION RECORDED**

Scope: SCHEMA-BLOCK-001, SCHEMA-BLOCK-002, parent-block representation, and
BlockId definition

Review Date: 2026-07-25

## 1. Exact Normative Alternatives

### 1.1 SCHEMA-BLOCK-001 — Header and Signed Form

#### Alternative A — Separate body and three-field signed envelope

```text
BlockHeaderBodyV1 = [
    schema_version,
    ...approved header fields
]

SignedBlockHeaderV1 = [
    body,
    signature_algorithm,
    producer_signature
]
```

The body is a fixed-position canonical array. The signed envelope has exact
length three. The signature payload is:

```text
Q1DomainFrameV1(
    BLOCK_HEADER_SIGNING,
    CanonicalCBOR(BlockHeaderBodyV1)
)
```

#### Alternative B — Signature fields inside BlockHeaderBodyV1

The body contains its own signature algorithm and signature and is serialized
as one array.

#### Alternative C — Unsigned canonical header with signature stored outside

BlockHeaderBodyV1 is canonical, but its signature is carried in an external
container that is not part of the canonical signed-header object.

### 1.2 Parent-Block Representation

#### Alternative P1 — Explicit null at the first signed block

```text
parent_block_id_or_null: BlockId | null
```

Normative invariant:

```text
height == 0  iff parent_block_id_or_null is null
height > 0   iff parent_block_id_or_null is a BlockId
```

This alternative assumes the first signed block has height zero. If genesis
uses another height model, the boundary invariant must be revised explicitly
before approval.

#### Alternative P2 — All-zero BlockId sentinel

```text
parent_block_id: BlockId
```

Normative invariant:

```text
height == 0  iff parent_block_id == 32 zero bytes
height > 0   iff parent_block_id != 32 zero bytes
```

The all-zero value is reserved permanently and can never identify a block.

#### Alternative P3 — Tagged genesis-or-block parent

```text
parent: [
    parent_kind: u16,
    parent_id: bytes32
]

parent_kind = 1 => GENESIS_ID
parent_kind = 2 => BLOCK_ID
```

The first signed block references GenesisId; every later block references
BlockId. The two identifier types remain semantically distinct.

#### Alternative P4 — Separate optional GenesisId and BlockId slots

```text
genesis_parent_id_or_null: GenesisId | null
parent_block_id_or_null: BlockId | null
```

Exactly one field is non-null: GenesisId at the first signed block and BlockId
afterward.

### 1.3 SCHEMA-BLOCK-002 — BlockId Definition

#### Alternative I1 — Complete signed-envelope identity

```text
BlockId =
    Q1HashV1(
        BLOCK_ID,
        CanonicalCBOR(SignedBlockHeaderV1)
    )
```

BlockId is a distinct bytes32 semantic type. It includes signature algorithm
and producer signature and is never serialized inside either header form.

#### Alternative I2 — Body-only identity

```text
BlockId =
    Q1HashV1(
        BLOCK_ID,
        CanonicalCBOR(BlockHeaderBodyV1)
    )
```

Different valid signatures over the same body produce the same BlockId.

#### Alternative I3 — Undomained digest

```text
BlockId = SHA-256(CanonicalCBOR(SignedBlockHeaderV1))
```

#### Alternative I4 — Partial-envelope identity

BlockId hashes the body and signature but omits the serialized signature
algorithm or another envelope field.

## 2. Advantages and Disadvantages

| Alternative | Advantages | Disadvantages |
|---|---|---|
| A | matches approved body/envelope separation; exact signing bytes; reusable verification model | header identity changes when signature changes |
| B | one array | circular or special-case signing rules; conflicts with approved envelope separation |
| C | small canonical body | canonical signed object is no longer self-contained; conflicts with the registered BLOCK_ID payload description |
| P1 | no fake BlockId; explicit absence uses the approved null rule; compact | field is a union; depends on a fixed first-block height convention |
| P2 | fixed bytes32 field and simple parsing | magic value inhabits BlockId space; APIs can mistake sentinel for an identifier |
| P3 | first block is cryptographically linked to GenesisId; semantic types remain explicit | larger nested form; requires parent-kind registry and exact genesis linkage rules |
| P4 | strongest type separation without a tag registry | two fields and cross-field exclusivity; permanent header overhead |
| I1 | matches approved complete-envelope object-ID rule and existing BLOCK_ID domain | signature malleability must be prevented by the strict signature profile |
| I2 | identity stable across signatures | contradicts the approved rule that official signed-object IDs include signatures |
| I3 | superficially simple | violates mandatory domain separation and permits cross-context ambiguity |
| I4 | potentially smaller conceptual preimage | creates a second envelope interpretation and ambiguous algorithm binding |

## 3. Long-Term Compatibility Implications

- Alternative A preserves one common signed-object model. Changing later to B
  or C would change canonical bytes, signing payloads, and every BlockId.
- P1 permanently encodes the first-block boundary as absence. Later addition
  of an explicit GenesisId link would require a new header schema version.
- P2 permanently reserves one bytes32 value and requires every API, database,
  and verifier to preserve sentinel semantics.
- P3 supports explicit genesis linkage and future identifier algorithms
  through a versioned parent kind, but its tag meanings become append-only
  protocol law.
- P4 preserves direct type separation but fixes two parent slots into every
  future compatible V1 header.
- I1 makes the exact signature algorithm and signature bytes part of durable
  block identity. Signature or envelope changes require a new schema version.
- I2 would permit signature replacement without changing identity, but moving
  later to the already-approved complete-envelope model would rewrite the
  identity contract.
- Public APIs should expose `BlockId`, not a generic hash. Reusing a generic
  `BlockHash` API risks accidental interchange even when both are bytes32.

## 4. Consensus Risks

- accepting more than one parent-boundary representation creates multiple
  canonical headers for the same logical first block;
- treating zero bytes as both sentinel and ordinary BlockId creates ambiguous
  ancestry and inconsistent database keys;
- using an untagged GenesisId in a BlockId field violates semantic type
  separation and can create cross-object reference confusion;
- failing to bind the first-block height invariant permits null or sentinel
  parents at later heights;
- body-only or partial-envelope IDs allow nodes to disagree about whether two
  differently signed headers are the same block;
- undomained hashing permits the same canonical bytes to be interpreted in
  another hashing context;
- serializing BlockId inside its own preimage creates a circular or
  normalization-dependent identifier;
- accepting unknown signature algorithms before version-specific parsing can
  produce divergent BlockIds and verification results;
- changing any selected representation after deployment breaks stored parent
  links and historical identity.

## 5. Recommendation Without Implementation

Approve Alternative A for SCHEMA-BLOCK-001:

```text
SignedBlockHeaderV1 = [
    BlockHeaderBodyV1,
    signature_algorithm,
    producer_signature
]
```

Approve Alternative I1 for SCHEMA-BLOCK-002:

```text
BlockId =
    Q1HashV1(
        BLOCK_ID,
        CanonicalCBOR(SignedBlockHeaderV1)
    )
```

Define BlockId as a distinct bytes32 semantic type and prohibit its
self-serialization.

For parent representation, prefer P1 if the human decision confirms that the
first signed block is height zero:

```text
parent_block_id_or_null: BlockId | null
```

with null valid if and only if `height == 0`. This avoids inventing a fake
BlockId and uses the already-approved explicit-null rule.

If the first signed block must cryptographically reference a separate
GenesisId, select P3 instead. Do not place an untagged GenesisId into a BlockId
field and do not silently reinterpret zero bytes.
