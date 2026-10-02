# DEC-Q1-027 Session 4 Merkle Report

Report Version: 0.1.0

Review Date: 2026-07-24

Decision Owner: Yousef Bahrami

Gate Outcome: **APPROVED WITH RECORDED CONDITIONS AND REVISIONS**

Implementation Authorization: **NONE**

Session 5 Authorization: **NONE**

## 1. Scope

Session 4 reviewed and recorded only SCHEMA-MERKLE-001 through
SCHEMA-MERKLE-005 and their documentation dependencies.

It did not authorize Merkle trees, builders, proofs, parsers, codecs, golden
vectors, Rust structs, runtime validation, BlockHeader, ledger state, receipts,
snapshot logic, consensus behavior, or Session 5.

## 2. Reviewed Decision Record

| Decision ID | Original proposal | Final human status | Revised normative value | Conditions |
|---|---|---|---|---|
| SCHEMA-MERKLE-001 | Shared binary indexed sequence with typed/kinded leaves | Approved with revision | shared binary indexed sequence; exact profile/index/length/item leaf payload; existing leaf/internal domains | no Merkle sorting or normalization; cross-profile reuse invalid |
| SCHEMA-MERKLE-002 | Profile IDs 1–4 and reserved StateRoot 5 | Approved | append-only root registry with distinct semantic root types | reservation does not activate a profile |
| SCHEMA-MERKLE-003 | Duplicate-last odd handling and typed empty root | Approved with revision | odd hash promoted unchanged; typed empty payload binds profile and zero count | duplicate-last rejected; owning schemas bind item count |
| SCHEMA-MERKLE-004 | StateRoot separate from sequence profile | Approved | profile 5 reserved inactive | requires complete state-model decision |
| SCHEMA-MERKLE-005 | Separate receipt/chunk leaf approval | Approved with condition | TransactionRoot and ParticipantRoot future-only; ReceiptRoot, SnapshotChunkRoot, and StateRoot inactive | explicit implementation authorization and per-profile dependencies required |

## 3. Exact Framing

For the item at zero-based index `i`:

```text
leaf_payload =
    tree_profile_id:u16be
 || index:u64be
 || item_length:u64be
 || canonical_item_bytes

leaf_hash =
    Q1HashV1(
        MERKLE_LEAF,
        leaf_payload
    )
```

```text
internal_payload =
    tree_profile_id:u16be
 || left_hash:bytes32
 || right_hash:bytes32

internal_hash =
    Q1HashV1(
        MERKLE_INTERNAL,
        internal_payload
    )
```

```text
empty_payload =
    tree_profile_id:u16be
 || item_count:u64be

item_count = 0

empty_root =
    Q1HashV1(
        MERKLE_LEAF,
        empty_payload
    )
```

Profile ID is two unsigned big-endian bytes. Index and item length are each
eight unsigned big-endian bytes. Item length must equal the exact canonical
item byte length. Child hashes are exactly 32 bytes and left/right position is
normative.

## 4. Tree Construction

At every non-root level, complete adjacent pairs are hashed from left to right.
An unpaired final hash is promoted unchanged. It is never duplicated.

- zero items use the typed empty construction, not an indexed leaf;
- one item has its leaf hash as root;
- two or more items repeat pairwise hashing and odd-node promotion until one
  root remains.

Duplicate-last was rejected because it creates a synthetic second child not
present in the committed sequence. Promotion preserves the actual level while
the indexed leaves and owning-schema count binding retain sequence context.

## 5. Profile Activation

| ID | Root type | Leaf object | Status |
|---:|---|---|---|
| `0x0001` | TransactionRoot | CanonicalCBOR(SignedTransferV1), exact block-body order | approved for future implementation after explicit authorization and transaction-count binding |
| `0x0002` | ReceiptRoot | not selected | reserved inactive pending Receipt schema |
| `0x0003` | ParticipantRoot | CanonicalCBOR(ParticipantRecordV1), ascending raw ParticipantId | approved for future implementation after explicit authorization and record-count binding |
| `0x0004` | SnapshotChunkRoot | not selected | reserved inactive pending canonical chunk descriptor |
| `0x0005` | StateRoot | not applicable to shared sequence profile | reserved inactive pending separate state model |

Semantic root types remain distinct despite their common bytes32
representation. Generic MerkleHash APIs must not permit accidental
interchange.

## 6. Ordering and Duplicate Rules

TransactionRoot preserves the exact block-body order. It does not sort,
deduplicate, or reject equal values; later block validation decides duplicate
TransferId policy. Index binding distinguishes equal canonical values at
different positions.

ParticipantRoot input must already be sorted by ascending raw ParticipantId.
Duplicate IDs and unsorted input are rejected, and canonical Merkle processing
must not repair either condition. Canonical record bytes are leaves;
ParticipantRecordHash is not substituted.

## 7. Count-Binding Dependency

The Merkle root alone does not introduce a generic count field. Every owning
canonical schema must either serialize the exact item count or derive it
unambiguously from another canonical committed object. That mechanism must be
approved before implementation. No hidden count assumption is permitted.

## 8. Dependencies Carried into Session 5

- BlockBody or another approved canonical commitment must bind
  `transaction_count`;
- the owning participant-set schema must bind exact record count;
- BlockHeader fields and root activation remain undecided;
- Receipt and SnapshotChunk leaf schemas remain undecided;
- StateRoot requires a separate state-model decision;
- object-specific resource limits remain required;
- implementation and conformance vectors require separate authorization.

## 9. Files Changed

- `docs/32_DEC_Q1_027_SCHEMA_DECISION_PACKAGE.md`;
- `docs/36_DEC_Q1_027_SESSION_4_MERKLE_REPORT.md`;
- `docs/01_GLOSSARY.md`;
- `docs/04_LEDGER_AND_TRANSACTIONS.md`;
- `docs/14_TEST_PLAN.md`;
- `docs/22_REQUIREMENTS_TRACEABILITY_SKELETON.md`;
- `OPEN_DECISIONS.md`;
- `CHANGELOG.md`;
- `PROJECT.md`;
- `README.md`.

Previously uncommitted M1.2 decision-gate documentation remains part of the
same pending working-tree change set.

## 10. No-Implementation Confirmation

No code, crate, Merkle tree, builder, proof, parser, codec, Rust struct,
runtime validator, golden vector, BlockHeader, ledger state, receipt logic,
snapshot logic, or consensus behavior was created or changed.

The existing cryptographic domain assignments were not changed and no new
Merkle root domain was allocated.

## 11. Recommendation for Session 5

Wait for explicit Session 5 authorization. Under the approved gate order,
Session 5 may review BlockHeader only after its remaining root/count and
dependency questions are made explicit. Do not begin BlockHeader review or
implementation automatically.
