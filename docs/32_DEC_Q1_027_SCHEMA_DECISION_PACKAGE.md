# DEC-Q1-027 — M1.2 Normative Schema Decision Package

Package Version: 0.1.0

Date: 2026-07-24

Status: Documentation Approved through Sessions 1–5C

Implementation: APPROVED INDEPENDENT SUBSETS IMPLEMENTED under the 2026-10-01
human continuation instruction; see PROJECT.md for current status.

Runtime Activation: LOCALNET ONLY where explicitly decided in
`docs/protocol/Q1_LOCALNET_V0.md`; unresolved rules remain blocked.

Current local overlay (2026-10-02): the human explicitly authorized minimal
LOCALNET_V0 Genesis/Proposal/Attestation/Certificate byte layouts and full-state
commitment while preserving approved reusable fields. See
`docs/protocol/Q1_LOCALNET_V0.md` for the implemented local schemas. Blank
approval rows below remain unapproved for the general protocol. LOCALNET-only
0x0011 and 0x0015 registrations supersede the historical registration pause
for those local payloads; 0x0012 remains unregistered.

Authority: DEC-Q1-027

Gate: DEC-Q1-027 — HUMAN SCHEMA DECISION

## 1. Executive Summary

This package proposes byte-level schema rules for the eight M1.2 protocol
objects. It is a decision document, not a protocol implementation.

Only rows with a completed Human Approval field are approved. Blank fields
remain non-normative and are intentionally reserved for later sessions.

The package recommends:

- fixed 32-byte hash-derived `ChainId`;
- semantic `u16` schema versions and enum registries encoded as shortest CBOR
  unsigned integers;
- fixed-position arrays and explicit `null` for absent optional values;
- deterministic collection ordering with duplicate rejection;
- separate body and signed-envelope schemas;
- IDs derived from complete canonical envelopes where an approved ID domain
  exists;
- no serialized self-ID fields;
- embedded, individually verifiable Ed25519 attestations in V1 certificates;
- a common binary Merkle profile for sequence commitments, while deferring
  `StateRoot` until the state-key/value model is approved;
- delay commitments in the header and full replaceable evidence outside the
  header.

Additional normative domain assignments and registrations remain required
before implementation. This package does not itself register or activate them
in the normative domain registry.

## 2. Decision Status Legend

| Label | Meaning |
|---|---|
| Approved foundation | Already fixed by an accepted architecture decision or profile |
| Proposed | Recommended here; requires human approval |
| Alternative | Preserved credible choice |
| Blocked dependency | Cannot become normative until another row is decided |
| Out of scope | Must not enter M1.2 schema behavior |

## 3. Common Schema Decisions

### 3.1 ChainId

#### Options

| Option | Canonical form | Strengths | Risks |
|---|---|---|---|
| A. Fixed 32 bytes | CBOR bytes, exactly 32 | 256-bit namespace, fixed parsing, simple APIs, future-proof | not human-readable |
| B. Fixed 16 bytes | CBOR bytes, exactly 16 | compact, UUID-like | smaller namespace and format/derivation ambiguity |
| C. Variable bytes | definite CBOR bytes with limit | flexible | multiple lengths, validation and comparison complexity |
| D. Human-readable string | CBOR text | readable | normalization, case, length, renaming, collision policy |
| E. Hash-derived fixed ID | fixed bytes from canonical preimage | deterministic, collision-resistant, creation rule auditable | needs a dedicated domain and preimage schema |

#### Recommendation

Use Option E with a 32-byte result:

```text
ChainIdentityPreimageV1 = [
    schema_version: u16 = 1,
    network_class: u16,
    creation_nonce: bytes32
]

ChainId =
    SHA-256(Q1DomainFrameV1(CHAIN_ID, CanonicalCBOR(
        ChainIdentityPreimageV1
    )))
```

`creation_nonce` must be generated once from an approved CSPRNG when a network
genesis package is created. It is public genesis input, not a secret. Reusing
one nonce for two network instances is prohibited.

Approved network-class registry:

| Value | Symbol | Activation |
|---:|---|---|
| `0x0001` | `LOCALNET` | active |
| `0x0002` | `PRIVATE_TESTNET` | active |
| `0x0003` | `RESEARCH` | active |
| `0x0004` | `RESERVED_FUTURE_PUBLIC` | reserved inactive |

`RESERVED_FUTURE_PUBLIC` does not authorize a public network or mainnet.
`network_class` and address HRP must agree under approved configuration
validation, but neither value is derived from the other.

Proposed new domain:

```text
CHAIN_ID = 0x0010
```

The domain assignment and revised preimage were approved in Session 1. The
assignment is **approved pending normative registry registration**; Session 1
does not edit or activate the normative domain registry.

#### Separation of Concepts

| Concept | Proposed representation | Authority |
|---|---|---|
| Human network name | printable ASCII UI metadata, 1–64 bytes | descriptive only |
| ChainId | fixed bytes32 derived above | replay and network-instance binding |
| Genesis hash | `GENESIS` domain hash of canonical GenesisManifest | exact genesis configuration identity |
| Address HRP | `q1l`, `q1p`, or `q1r` | address network class/display binding |

No equality or derivation between these four concepts is implied.

API representation recommendation:

- `ChainId`: exactly 64 lowercase hexadecimal characters;
- human network name: ordinary JSON string;
- genesis hash: typed 64-character lowercase hash;
- HRP: registered lowercase string.

| Decision Item | Recommendation | Human Approval |
|---|---|---|
| SCHEMA-COMMON-001 | Hash-derived fixed bytes32 ChainId using domain `0x0010` and revised three-field preimage | Approved with revision — 2026-07-24; pending registry registration |
| SCHEMA-COMMON-002 | CSPRNG public `creation_nonce: bytes32`; reuse prohibited | Approved — 2026-07-24 |
| SCHEMA-COMMON-003 | Lowercase hex API form; network name/genesis hash/HRP/network class remain distinct | Approved — 2026-07-24 |

### 3.2 Schema Versioning

Recommendation:

- every top-level object and reusable nested record starts at index `0` with
  `schema_version`;
- semantic type is `u16`;
- V1 value is `1`;
- CBOR uses the shortest major-type-0 encoding, therefore V1 is byte `0x01`;
- byte order matters only when a `u16` is embedded in a raw framing preimage;
  ordinary CBOR unsigned integers follow the approved CBOR profile;
- version `0` is permanently invalid;
- unknown versions are rejected before reading version-specific fields;
- a version number is never reused for a different field table;
- `schema_version` identifies one object encoding; `protocol_version`
  identifies protocol rules and is a separate `u16`.

| Decision Item | Recommendation | Human Approval |
|---|---|---|
| SCHEMA-COMMON-004 | Every schema begins with semantic `u16` version; V1 is `1` | Approved — 2026-07-24 |
| SCHEMA-COMMON-005 | Unknown/zero versions rejected; version meanings never reused | Approved — 2026-07-24 |
| SCHEMA-COMMON-006 | Schema version and protocol version remain distinct fields | Approved — 2026-07-24 |

### 3.3 Enum Discriminants

Common recommendation:

- semantic type `u16`, except already-approved AddressType `u8`;
- CBOR shortest unsigned integer;
- `0` means invalid/unassigned unless explicitly listed as `NONE`;
- values are append-only and never reused;
- unknown values are rejected;
- new assignments require a versioned registry change and human approval;
- text names never enter consensus bytes.

#### Proposed Registry

| Enum | Value | Symbol | Status |
|---|---:|---|---|
| TransactionType | `0x0001` | `TRANSFER` | semantic class; no TransferBodyV1 discriminant |
| TransactionType | `0x0002` | `GENESIS_ALLOCATION` | reserved for distinct schema |
| TransactionType | `0x0003` | `PROTOCOL_REWARD` | reserved for distinct schema |
| TransactionType | `0x0004` | `PROTOCOL_PENALTY` | reserved inactive |
| ParticipantRole | `0x0001` | `PRODUCER` | active |
| ParticipantRole | `0x0002` | `VALIDATOR` | active |
| ParticipantRole | `0x0003` | `NODE_IDENTITY` | reserved inactive; excluded by SCHEMA-PART-003 |
| AddressType | `0x01` | `SINGLE_ED25519` | approved |
| SignatureAlgorithm | `0x0001` | `ED25519_Q1_V1` | approved |
| HashAlgorithm | `0x0001` | `SHA256_Q1_DOMAIN_V1` | proposed identifier for containing schemas |
| AttestationType | `0x0001` | `VALID_BLOCK` | approved assignment |
| AttestationType | `0x0002`–`0x00ff` | future signed evidence | reserved |
| SnapshotType | `0x0001` | `FULL_STATE` | approved assignment |
| SnapshotType | `0x0002` | `INCREMENTAL` | reserved inactive |
| CompressionAlgorithm | `0x0000` | `NONE` | approved assignment |
| CompressionAlgorithm | `0x0001`–`0x00ff` | future profiles | reserved |
| DelayEngine | `0x0000` | `NONE` | conditionally permitted by network/genesis profile |
| DelayEngine | `0x0001`–`0x00ff` | approved future engines | reserved |

No `ProposalType` field is recommended for V1: only block proposals are in
scope and the schema version already identifies their interpretation.

Where a later schema uses a participant-role set outside ParticipantRecordV1,
it is a sorted non-empty array of unique `ParticipantRole` values, not a
bitmask. ParticipantRecordV1 derives roles from key presence and serializes no
role array. `NODE_IDENTITY` remains inactive.

Reserved TransactionType values cannot be encoded in TransferBodyV1, require
distinct future schemas or protocol versions, and do not authorize execution.
`DelayEngine::NONE` may be used only when the
applicable approved genesis/network profile permits it; it cannot bypass a
mandatory delay requirement. Incremental snapshots and future compression or
delay assignments remain inactive.

| Decision Item | Recommendation | Human Approval |
|---|---|---|
| SCHEMA-COMMON-007 | Append-only `u16` enum registry, zero invalid by default | Approved — 2026-07-24 |
| SCHEMA-COMMON-008 | TransactionType assignments and activation states above | Approved with condition — 2026-07-24 |
| SCHEMA-COMMON-009 | ParticipantRole assignments and sorted-array representation | Approved with condition — 2026-07-24 |
| SCHEMA-COMMON-010 | HashAlgorithm, AttestationType, SnapshotType, CompressionAlgorithm, and DelayEngine assignments | Approved with conditions — 2026-07-24 |
| SCHEMA-COMMON-011 | Omit ProposalType from V1 | Approved — 2026-07-24 |

### 3.4 Optional Fields

#### Options

| Option | Determinism | Evolution | Complexity |
|---|---|---|---|
| A. Fixed-position explicit null | one representation; positions stable | version required when semantics change | low |
| B. Presence boolean + value | unique if strictly checked | positions stable | redundant and more rejection states |
| C. New version for every presence change | unique | clear but version-heavy | medium |
| D. Tagged union | expressive | tags are prohibited by Q1 CBOR | incompatible |

Recommendation: Option A.

- every optional field retains its fixed array index;
- absence is exactly CBOR `null` (`0xf6`);
- omission shortens the array and is rejected;
- a present value must have the exact field type;
- `null` is invalid for required fields;
- unknown trailing fields are rejected;
- changing an optional field's meaning or type requires a new schema version.

| Decision Item | Recommendation | Human Approval |
|---|---|---|
| SCHEMA-COMMON-012 | Explicit `null` at fixed position for every optional field | Approved — 2026-07-24 |
| SCHEMA-COMMON-013 | Missing/trailing fields rejected; semantic change requires new schema version | Approved — 2026-07-24 |

### 3.5 Collection Ordering and Limits

Four collection classes are proposed:

| Class | Meaning | Encoding rule |
|---|---|---|
| Protocol sequence | order changes meaning | preserve submitted/protocol order |
| Sorted set | membership only | ascending bytewise canonical sort key |
| Sorted map-like sequence | unique keyed records | ascending bytewise canonical key |
| Operational sequence | non-consensus local order | excluded from canonical objects |

General rules:

- sorting compares raw fixed-width key bytes lexicographically, unsigned;
- sorting is ascending;
- equal sort keys are rejected, never stabilized by a secondary insertion
  order;
- encoders must reject unsorted input rather than silently normalize it;
- decoders reject unsorted or duplicate entries;
- canonical sort is stable by definition because every key is unique;
- every canonical array count is at most `65_535`, matching the approved CBOR
  profile; schemas may approve lower limits later.

The `65_535` value is only an absolute schema-format ceiling. It is not a
block, network, operational-acceptance, or safe-allocation limit. Lower
object-specific and operational limits remain mandatory before runtime or
untrusted-input acceptance. Decoders must not allocate solely from the format
ceiling.

A non-normative helper may prepare sorted collections, but the normative
encoder and decoder reject unsorted input and never normalize it.

Proposed object-specific rules:

| Collection | Class | Sort/order key | Duplicate policy | Proposed max |
|---|---|---|---|---:|
| Genesis allocations | sorted map-like | 36-byte binary address envelope | duplicate address rejected | 65,535 |
| Participant records | sorted map-like | ParticipantId bytes32 | duplicate ID or role key rejected | 65,535 |
| Participant-role sets outside ParticipantRecordV1 | sorted set | enum numeric value | duplicate rejected | 3 |
| Certificate attestations | sorted map-like | validator ParticipantId bytes32 | duplicate signer rejected | 65,535 |
| Snapshot chunks | protocol sequence | explicit chunk index ascending from zero | missing/duplicate index rejected | 65,535 |
| Block transactions | protocol sequence | block-body order | duplicate transaction ID policy is separate validation | 65,535 |

| Decision Item | Recommendation | Human Approval |
|---|---|---|
| SCHEMA-COMMON-014 | Ascending raw-byte sort for set/map-like collections; reject unsorted input | Approved with condition — 2026-07-24 |
| SCHEMA-COMMON-015 | Collection classes/keys/duplicates; 65,535 only a format ceiling | Approved with condition — 2026-07-24 |

### 3.6 Signed Form, Complete Form, and IDs

General construction:

```text
Body                 = canonical fixed-position array
SigningPayload       = Q1DomainFrameV1(signing_domain, CanonicalCBOR(Body))
SignedEnvelope       = [Body, signature_algorithm, signature]
CompleteSerialized   = CanonicalCBOR(SignedEnvelope)
ObjectIdInput        = CompleteSerialized
ObjectId             = Q1HashV1(id_domain, ObjectIdInput)
```

Signer public keys are included in `Body` only when the schema explicitly
requires self-contained verification. They are never duplicated in the
envelope. A signature is excluded from its signing payload and included in
the object ID when the registry defines the ID over the complete signed form.

The complete-object hash and object ID are the same operation unless a schema
explicitly defines otherwise.

| Decision Item | Recommendation | Human Approval |
|---|---|---|
| SCHEMA-COMMON-016 | Separate Body and SignedEnvelope; sign only canonical Body frame | Approved — 2026-07-24 |
| SCHEMA-COMMON-017 | Official ObjectId hashes complete signed envelope; later BodyHash requires separate domain/approval | Approved with condition — 2026-07-24 |
| SCHEMA-COMMON-018 | Never duplicate signer key between Body and envelope | Approved — 2026-07-24 |

### 3.7 Self-Hash Exclusion

#### Options

| Option | Risk |
|---|---|
| A. ID never serialized inside identified object | no circularity; simplest |
| B. Serialize zero ID during hashing | normalization rule and two representations |
| C. Separate Body and Envelope types | useful for signatures, but still circular if ID inserted |

Recommendation: combine A and C.

No object contains the ID/hash derived from its complete bytes. IDs are
computed values carried by indexes, APIs, references from other objects, or
outer transport records. Zero-substitution hashing is prohibited.

| Decision Item | Recommendation | Human Approval |
|---|---|---|
| SCHEMA-COMMON-019 | Self IDs never serialized; zero-substitution prohibited | Approved — 2026-07-24 |

## 4. Reusable Proposed Types

The following reusable semantic wire types are approved:

| Type | Canonical CBOR |
|---|---|
| ChainId | bytes, exactly 32 |
| ParticipantId | bytes, exactly 32 |
| AddressEnvelope | bytes, exactly 36 |
| PublicKeyEd25519 | bytes, exactly 32 |
| SignatureEd25519 | bytes, exactly 64 |
| Typed hashes | bytes, exactly 32 |
| Amount | bytes, exactly 16, approved Amount rule |
| FeeLimit | distinct semantic type; same fixed 16-byte unsigned encoding as Amount |
| Weight | unsigned `u64`, shortest CBOR |
| Nonce | unsigned `u64`, shortest CBOR |
| Height | unsigned `u64`, shortest CBOR |
| RoundNumber | unsigned `u32`, shortest CBOR |
| CandidateIndex | unsigned `u16`, shortest CBOR |
| ByteCount | unsigned `u64`, shortest CBOR |

Every row remains a distinct semantic type even where its wire representation
matches another row. Implicit conversion between semantic types is prohibited.
A generic Hash256 abstraction must not permit typed hashes to be interchanged
accidentally. Public APIs must preserve the same semantic boundaries.

`FeeLimit(u128)` uses only a definite CBOR byte string of exactly 16 bytes:

```text
0x50 || FeeLimit::to_be_bytes()
```

It is unsigned, big-endian, and left zero-padded. Arithmetic is checked; no
floating-point conversion or implicit conversion to or from `Amount` is
permitted. This fixes only the field shape and does not approve any fee
formula, minimum, congestion, settlement, or charging behavior.

`Weight` is an approved reusable type but is not serialized in
ParticipantRecordV1.

The existing M1.1 `Round { slot, number }` does not match the consensus
context `(Height, RoundNumber)`. `RoundNumber` is a distinct `u32` semantic
type, begins at zero unless a later consensus decision changes that rule, and
uses shortest canonical CBOR unsigned encoding. `Slot` must not substitute for
`Height`. A narrow M1.1 correction must introduce `RoundNumber` and deprecate
or reclassify `Slot` and composite `Round` for consensus use before M1.2
schema implementation. Session 2 does not authorize that correction or any
round-transition or timing behavior.

| Decision Item | Recommendation | Human Approval |
|---|---|---|
| SCHEMA-COMMON-020 | Approve reusable fixed-width/type table | Approved with condition — 2026-07-24 |
| SCHEMA-COMMON-021 | Add distinct FeeLimit type with Amount wire width only after approval | Approved — 2026-07-24 |
| SCHEMA-COMMON-022 | Use `RoundNumber:u32`; review M1.1 Slot/Round mismatch separately | Approved with required M1.1 correction — 2026-07-24 |

## 5. Proposed Object Schemas

All indexes below are zero-based. “Sign” and “ID” mean inclusion in the
canonical body used by the indicated operation. No table is normative before
its decision rows are approved.

### 5.1 GenesisManifestV1

Proposed canonical object: fixed array length `9`.

Genesis ID is not serialized. It is:

```text
GenesisId = Q1HashV1(GENESIS, CanonicalCBOR(GenesisManifestV1))
```

| # | Field | Normative type | Width/limit | Presence | Validation | Sign | ID | Open dependency |
|---:|---|---|---|---|---|---|---|---|
| 0 | schema_version | u16 | V1=`1` | required | supported version | n/a | yes | common version |
| 1 | protocol_version | u16 | proposed V1=`1` | required | enabled protocol | n/a | yes | protocol registry |
| 2 | chain_id | ChainId | 32 bytes | required | matches approved derivation | n/a | yes | SCHEMA-COMMON-001 |
| 3 | declared_genesis_supply | Amount | 16 bytes | required | exact sum target | n/a | yes | approved invariant |
| 4 | allocations | array AllocationV1 | 0–65,535 | required | sorted/unique; sum exact | n/a | yes | SCHEMA-GEN-002 |
| 5 | participants | array ParticipantRecordV1 | 1–65,535 | required | sorted/unique | n/a | yes | Participant schema |
| 6 | protocol_parameters_hash | GenericHash | 32 bytes | required | approved external manifest hash | n/a | yes | parameter-manifest schema/domain |
| 7 | economic_rules_version | u16 | nonzero | required | registered version | n/a | yes | economics version registry |
| 8 | delay_configuration_hash | GenericHash | 32 bytes | required | approved external config hash | n/a | yes | delay config schema/domain |

Genesis timestamp and human network name recommendation: exclude both. Store
them in a non-authoritative deployment descriptor together with the ChainId
and GenesisId. Renaming a network or correcting observational time must not
change GenesisId.

AllocationV1 proposed fixed array length `3`:

| # | Field | Type | Width | Rule |
|---:|---|---|---|---|
| 0 | schema_version | u16 | V1=`1` | required |
| 1 | recipient | AddressEnvelope | 36 bytes | address compatible with network class |
| 2 | amount | Amount | 16 bytes | may be zero only if explicitly approved; recommendation: positive |

Allocation purpose/vesting alternatives:

- A: include `purpose_code` and `vesting_rule` now;
- B: exclude both from V1 because neither changes the initial unencumbered
  balance and lock execution is not approved;
- C: commit an external allocation-policy hash.

Recommendation: Option B for executable simplicity. Human-readable purpose is
published beside the genesis package but excluded from consensus bytes. If
locked genesis funds are required, create a new allocation schema version
after lock semantics are approved.

Direct embedding is recommended for allocations and participants. Roots would
add a Merkle dependency without improving independently verifiable small
private-testnet genesis packages.

Security consequences:

- exact sum prevents hidden supply;
- direct entries expose all allocations/participants;
- excluding timestamps prevents observational metadata changing chain identity;
- external parameter hashes require their referenced manifests to be
  available and normatively encoded before genesis implementation.

| Decision Item | Recommendation | Human Approval |
|---|---|---|
| SCHEMA-GEN-001 | Approve nine-field GenesisManifestV1 and GENESIS hash rule | |
| SCHEMA-GEN-002 | Approve three-field AllocationV1, sorted by address, exact-sum validation | |
| SCHEMA-GEN-003 | Embed allocations/participants directly; exclude timestamp and network name | |
| SCHEMA-GEN-004 | Exclude purpose/vesting from V1; require new version for locked allocations | |
| SCHEMA-GEN-005 | Approve external protocol/delay config commitments or replace with exact embedded schemas | |

### 5.2 ParticipantRecordV1

Approved identity body: fixed array length `3`.

Participant ID derivation:

```text
ParticipantIdentityBodyV1 = [
    schema_version: u16 = 1,
    producer_public_key_or_null: Ed25519PublicKey | null,
    validator_public_key_or_null: Ed25519PublicKey | null
]

ParticipantId =
    Q1HashV1(PARTICIPANT_ID, CanonicalCBOR(
        ParticipantIdentityBodyV1
    ))
```

Domain `PARTICIPANT_ID = 0x0013` is approved pending normative registration.
Changing either role key produces a new ParticipantId. ParticipantId identifies
only the role-key identity body, not its activation schedule or complete
participant record.

Approved record: fixed array length `6`.

The record hash remains:

```text
ParticipantRecordHash =
    Q1HashV1(PARTICIPANT_RECORD, CanonicalCBOR(ParticipantRecordV1))
```

| # | Field | Normative type | Width/limit | Presence | Validation | Sign | ID | Open dependency |
|---:|---|---|---|---|---|---|---|---|
| 0 | schema_version | u16 | V1=`1` | required | supported | n/a | yes | common |
| 1 | participant_id | ParticipantId | 32 bytes | required | recompute from identity body | n/a | yes | new domain |
| 2 | producer_public_key_or_null | Ed25519 key/null | 32 bytes | optional | explicit null or strict key | n/a | yes | approved crypto |
| 3 | validator_public_key_or_null | Ed25519 key/null | 32 bytes | optional | explicit null or strict key | n/a | yes | approved crypto |
| 4 | activation_height | Height | u64 | required | genesis-active value is `0` | n/a | yes | approved activity rule |
| 5 | deactivation_height_or_null | Height/null | u64 | optional | null or strictly greater than activation | n/a | yes | approved activity rule |

At least one role key must be present. If both are present they must be
distinct and each present key must satisfy the strict Ed25519 public-key
profile. The record's key slots must exactly match the identity preimage and
`participant_id` must be recomputed from its canonical bytes.

Roles are derived from key presence: producer key means `PRODUCER`; validator
key means `VALIDATOR`. A participant may have either or both. No roles,
eligible, weight, or node-identity field is serialized. Active producer and
validator role weights are each protocol-defined as `1` in V1.

A participant is active at height `H` exactly when:

```text
activation_height <= H
and
(deactivation_height is null or H < deactivation_height)
```

Reputation, stake, HDD, AI, cooldown, penalty, free-form metadata, and
public-admission fields are excluded. Node transport identity belongs to
network configuration or a future network-specific schema; the reserved
`NODE_IDENTITY` role remains inactive.

| Decision Item | Recommendation | Human Approval |
|---|---|---|
| SCHEMA-PART-001 | Approve multi-role ParticipantRecord with distinct role keys | Approved with revision — 2026-07-24 |
| SCHEMA-PART-002 | Approve ParticipantId derivation and new domain `0x0013` | Approved with revision — 2026-07-24; pending registry registration |
| SCHEMA-PART-003 | Select node-key option A, B, or C; recommendation C | Approved — Option C — 2026-07-24 |
| SCHEMA-PART-004 | Equal V1 weight=`1`, sorted roles, no reputation/stake/HDD/AI fields | Approved with revision — 2026-07-24 |
| SCHEMA-PART-005 | Approve activation/deactivation representation | Approved with condition — 2026-07-24 |

### 5.3 SignedTransferV1

`TransferBodyV1` is an approved fixed array length `9`:

| # | Field | Normative type | Width/limit | Presence | Validation | Sign | ID | Approval state |
|---:|---|---|---|---|---|---|---|---|
| 0 | schema_version | u16 | V1=`1` | required | supported | yes | yes | approved |
| 1 | chain_id | ChainId | 32 bytes | required | exact active chain | yes | yes | approved common |
| 2 | sender_public_key | Ed25519 key | 32 bytes | required | strict; derives sender address | yes | yes | approved key rules |
| 3 | recipient_address | AddressEnvelope | 36 bytes | required | valid binary envelope | yes | yes | approved |
| 4 | amount | Amount | 16 bytes | required | structural Amount rule | yes | yes | approved encoding |
| 5 | fee_limit | FeeLimit | 16 bytes | required | field shape only | yes | yes | approved type |
| 6 | nonce | Nonce | u64 shortest CBOR | required | structural only | yes | yes | approved |
| 7 | valid_from_height | Height | u64 | required | `<= valid_until` | yes | yes | approved |
| 8 | valid_until_height | Height | u64 | required | `>= valid_from` | yes | yes | approved |

`SignedTransferV1` is an approved fixed array length `3`:

| # | Field | Type | Rule |
|---:|---|---|---|
| 0 | body | TransferBodyV1 | exact 9-field array |
| 1 | signature_algorithm | SignatureAlgorithm | only `0x0001` |
| 2 | signature | Ed25519 signature | exactly 64 bytes |

Operations:

```text
TransferSigningPayload =
    Q1DomainFrameV1(TRANSACTION_SIGNING, CanonicalCBOR(TransferBodyV1))

TransferId =
    Q1HashV1(TRANSACTION_ID, CanonicalCBOR(SignedTransferV1))
```

The signature is excluded from its own signing payload and included in
TransferId. No self-derived ID is serialized. Existing transaction domains are
unchanged.

Only the sender public key is serialized; its account/address is
deterministically derived using the approved address profile. Consensus bytes
store the recipient only as the binary AddressEnvelope, never a Bech32m HRP
string. `transaction_type`, `sender_address`, and `memo_hash` have no V1 field
or reserved/null position.

TransferBodyV1 represents only ordinary value transfer. Genesis allocation,
protocol reward, protocol penalty, and other reserved transaction classes
require separate schemas or later protocol versions.

Both validity heights and Nonce are signed and included in TransferId.
`valid_from_height <= valid_until_height`; no infinite-lifetime sentinel is
approved. Expiration, replay, account-state, fee calculation, charging,
settlement, congestion, minimum, burn, issuance, treasury, refund, and
execution behavior remain outside Session 3.

| Decision Item | Recommendation | Human Approval |
|---|---|---|
| SCHEMA-TX-001 | Approve 12-field TransactionBody and three-field SignedTransaction | Approved with revision — 2026-07-24; renamed 9-field TransferBodyV1 and SignedTransferV1 |
| SCHEMA-TX-002 | Include sender address and public key; require derivation match | Approved with revision — 2026-07-24; sender key only |
| SCHEMA-TX-003 | TransactionId includes complete signature-bearing envelope | Approved — 2026-07-24; named TransferId |
| SCHEMA-TX-004 | FeeLimit is distinct fixed-16-byte semantic type; no fee behavior | Approved — 2026-07-24 |
| SCHEMA-TX-005 | Memo position exists but must be null in V1 | Rejected — 2026-07-24; field removed |
| SCHEMA-TX-006 | V1 body is transfer-only; reserve other type IDs for distinct future schemas | Approved with revision — 2026-07-24; no transaction_type field |

### 5.4 BlockHeaderV1

Sessions 5A–5C approved the canonical body, signed envelope, complete field
table, and exact array lengths:

```text
BlockHeaderBodyV1 = [
    schema_version: u16 = 1,
    protocol_version: u16 = 1,
    chain_id: ChainId,
    height: Height,
    round_number: RoundNumber,
    parent_reference: ParentReferenceV1,
    producer_id: ParticipantId,
    transaction_root: TransactionRoot,
    participant_root: ParticipantRoot,
    state_root: StateRoot,
    delay_evidence_commitment: DelayEvidenceHash
]

SignedBlockHeaderV1 = [
    body: BlockHeaderBodyV1,
    signature_algorithm: SignatureAlgorithm,
    producer_signature: Ed25519Signature
]
```

The signed envelope has exact array length `3`. Unknown, missing, or trailing
envelope fields are rejected. The signature algorithm must be supported by the
applicable schema and protocol version. The producer signature is excluded
from its own signing payload.

Approved parent record:

```text
ParentReferenceV1 = [
    schema_version: u16 = 1,
    parent_kind: ParentKind,
    parent_id: bytes32
]
```

Exact length is `3`. ParentKind is append-only:

| Value | Symbol | `parent_id` semantic type |
|---:|---|---|
| `0x0001` | `GENESIS` | GenesisId |
| `0x0002` | `BLOCK` | BlockId |

Version zero, unknown versions, unknown kinds, null, all-zero sentinels, and
untagged GenesisId values in BlockId fields are rejected. ParentReferenceV1
appears as the fixed field at index 5 in BlockHeaderBodyV1.

GenesisManifest is independent and is not SignedBlockHeaderV1. The first
signed block has height `1`; height `0` is invalid:

```text
height == 1
iff parent_kind == GENESIS
and parent_id == the governing GenesisId

height > 1
iff parent_kind == BLOCK
and parent_id == the BlockId at height - 1
```

Structural parsing checks the parent record. Historical-chain validation checks
the governing identity and exact height relationship.

Session 5B approved the root members and Session 5C fixed their indexes and
the final body length:

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

TransactionRoot is required, uses profile `0x0001`, and commits to exact
CanonicalCBOR(SignedTransferV1) leaves in canonical BlockBody order.
ParticipantRoot is required, uses profile `0x0003`, and commits to the active
participant set governing validation at height H, derived from finalized
pre-H state. StateRoot is required only in field shape; construction profile
`0x0005` remains inactive and BlockHeaderV1 implementation/valid instantiation
is blocked until state-model approval.

ReceiptRoot and generic `block_body_commitment` are omitted entirely from V1.
No null, empty, opaque, or reserved field replaces either one.

Approved minimal canonical body:

```text
BlockBodyV1 = [
    schema_version: u16 = 1,
    transfers: array<SignedTransferV1>
]
```

Exact outer length is `2`. Transfer order is normative. Unknown, missing, or
trailing fields are rejected. No receipts, evidence, metadata, reserved
sections, future placeholders, BlockBodyId, body domain, or generic body
commitment are approved.

`TransactionCount` is a distinct semantic `u32` used by APIs, validation,
diagnostics, and limits but is not serialized in Header or BlockBody:

```text
TransactionCount =
    len(BlockBodyV1.transfers) =
    TransactionRoot leaf count
```

Zero is valid and uses the typed empty TransactionRoot. The value must fit
`u32`; no runtime limit is implied.

Approved canonical participant-set owner:

```text
ParticipantSetV1 = [
    schema_version: u16 = 1,
    reference_height: Height,
    participants: array<ParticipantRecordV1>
]
```

Exact outer length is `3`. `reference_height=H` means the set governs
validation of signed block H and derives from finalized state before executing
H. Records are active at H, non-empty, sorted by ascending raw ParticipantId,
and duplicate IDs are rejected. ParticipantRoot is computed from only the
participants array. No ParticipantSetId or new domain is approved.

`ParticipantCount` is a distinct semantic `u32`, not serialized in Header or
ParticipantSet:

```text
ParticipantCount =
    len(ParticipantSetV1.participants) =
    ParticipantRoot leaf count
```

Zero is invalid. TransactionCount and ParticipantCount are not interchangeable
with each other or generic integers.

Approved producer-membership invariant:

```text
producer_id identifies an active PRODUCER
inside ParticipantSetV1 for Header.height,
whose participants are committed by ParticipantRoot
```

Structural validation checks canonical typed forms. Historical/state validation
must check matching reference height, reconstructed root, membership, active
producer role and interval, role-key resolution, and producer signature.

The approved BlockHeaderBodyV1 has exact outer array length `11`. Every field
is required, no optional or trailing field exists, and this order is permanent
for V1:

| # | Field | Normative type | Width | Presence | Validation | Sign | ID | Open dependency |
|---:|---|---|---|---|---|---|---|---|
| 0 | schema_version | u16 | V1=`1` | required | supported | yes | yes | common |
| 1 | protocol_version | u16 | V1=`1` | required | enabled | yes | yes | registry |
| 2 | chain_id | ChainId | 32 bytes | required | active chain | yes | yes | common |
| 3 | height | Height | u64 | required | structural | yes | yes | approved primitive |
| 4 | round_number | RoundNumber(u32) | shortest CBOR | required | zero valid | yes | yes | M1.1 correction before implementation |
| 5 | parent_reference | ParentReferenceV1 | exact nested record | required | approved parent/height rules | yes | yes | Session 5A form |
| 6 | producer_id | ParticipantId | 32 bytes | required | referenced participant | yes | yes | Participant schema |
| 7 | transaction_root | TransactionRoot | 32 bytes | required | approved profile; implementation gated | yes | yes | canonical BlockBodyV1 |
| 8 | participant_root | ParticipantRoot | 32 bytes | required | pre-H active-set commitment | yes | yes | canonical ParticipantSetV1 |
| 9 | state_root | StateRoot | 32 bytes | required | construction/activation blocked | yes | yes | separate state-model gate |
| 10 | delay_evidence_commitment | DelayEvidenceHash | 32 bytes | required | full evidence commitment | yes | yes | domain registration and evidence profiles |

Operations:

```text
BlockHeaderSigningPayload =
    Q1DomainFrameV1(BLOCK_HEADER_SIGNING, CanonicalCBOR(BlockHeaderBodyV1))

BlockId =
    Q1HashV1(BLOCK_ID, CanonicalCBOR(SignedBlockHeaderV1))
```

BlockId is a distinct semantic bytes32 type. Signature algorithm and producer
signature are included in its preimage. BlockId is not serialized in either
header form. Body-only, partial-envelope, undomained, and raw SHA-256
identifiers are rejected; typed public APIs must not expose BlockId as an
interchangeable generic hash. Strict deterministic Ed25519 semantics remain
required.

`protocol_version` is a semantic `u16`; V1 is `1` encoded as the shortest CBOR
unsigned integer. Zero and unknown versions are rejected. Historical or
configuration validation also rejects known but inactive versions. Protocol
version is distinct from schema version; Session 5C approves no activation or
governance mechanism.

`round_number` is the distinct `RoundNumber(u32)` semantic type. Zero is valid.
Consensus context is `(Height, RoundNumber)` and Slot never substitutes for
either value. Implementation remains blocked until a separately authorized
M1.1 correction introduces RoundNumber, makes Slot legacy/non-consensus or
deprecated, and deprecates the composite `Round { slot, number }` for
consensus use.

No timestamp, wall-clock value, fork-choice data, candidate index or selection
proof, full delay proof, ReceiptRoot, computed economic value,
economic_rules_version, generic body commitment, self BlockId, ProposalId,
FinalizationCertificateId, parent_height, transaction_count, or
participant_count appears in the header. No null, reserved position, sentinel,
or placeholder replaces an excluded value.

Session 5B resolved the root/body alternatives: ParticipantRoot is the required
pre-H active-set commitment, ReceiptRoot is absent, and generic body
commitment is absent. Whether a later validation stage uses a derived committee
remains a separate decision and does not change ParticipantRoot's approved
meaning.

Schema status is **APPROVED**. Implementation and runtime activation are
**BLOCKED** by the dependencies recorded below and do not follow from schema
approval.

| Decision Item | Recommendation | Human Approval |
|---|---|---|
| SCHEMA-BLOCK-001 | Approve separate canonical body and three-field signed envelope | Approved — Alternative A — 2026-07-25; final 11-field body completed in Session 5C |
| SCHEMA-BLOCK-002 | BlockId hashes complete signed header; never serialized within it | Approved — Alternative I1 — 2026-07-25 |
| SCHEMA-BLOCK-003 | No timestamp/fork-choice/economic values/full delay proof | Approved — 2026-07-25; exact exclusion set recorded |
| SCHEMA-BLOCK-004 | Required ReceiptRoot with approved empty root | Revised and approved — 2026-07-25; ReceiptRoot omitted |
| SCHEMA-BLOCK-005 | Select body-commitment retention; recommendation retain only for non-transaction evidence | Revised and approved — 2026-07-25; generic commitment omitted |
| SCHEMA-BLOCK-006 | Resolve RoundNumber versus M1.1 Round before implementation | Approved with implementation blocker — 2026-07-25 |

### 5.5 BlockProposalV1

Recommended separation:

- signed block header identifies the proposed block;
- proposal body carries round-specific evidence and the block body;
- proposal signature proves the producer sent that exact package;
- ProposalId is distinct from BlockId because evidence/body packaging may
  differ while the signed header remains the same.

`BlockProposalBodyV1` proposed fixed array length `9`:

| # | Field | Normative type | Width/limit | Presence | Validation | Sign | ID | Open dependency |
|---:|---|---|---|---|---|---|---|---|
| 0 | schema_version | u16 | V1=`1` | required | supported | yes | yes | common |
| 1 | chain_id | ChainId | 32 bytes | required | active chain | yes | yes | common |
| 2 | height | Height | u64 | required | equals header | yes | yes | common |
| 3 | round_number | u32 | shortest CBOR | required | equals header | yes | yes | Round review |
| 4 | producer_id | ParticipantId | 32 bytes | required | equals header | yes | yes | participant |
| 5 | signed_block_header | SignedBlockHeaderV1 | exact schema | required | BlockId recomputable | yes | yes | header |
| 6 | block_body | canonical bytes | approved max | required | commitment matches header | yes | yes | body schema |
| 7 | producer_selection_proof | canonical bytes | approved max | required | opaque until selection profile | yes | yes | DEC-Q1-005/018 |
| 8 | delay_evidence | DelayEvidenceV1 | approved max | required | commitment matches header | yes | yes | delay decision |

`SignedBlockProposalV1` fixed array length `3`:

| # | Field | Type |
|---:|---|---|
| 0 | body | BlockProposalBodyV1 |
| 1 | signature_algorithm | SignatureAlgorithm |
| 2 | producer_signature | Ed25519 signature |

Operations:

```text
ProposalSigningPayload =
    Q1DomainFrameV1(PROPOSAL_SIGNING, CanonicalCBOR(BlockProposalBodyV1))

ProposalId =
    Q1HashV1(PROPOSAL_ID, CanonicalCBOR(SignedBlockProposalV1))
```

Proposed new domain `PROPOSAL_ID = 0x0011`.

Duplicate proposal detection uses ProposalId. Equivocation detection compares
producer, chain, height, round, and differing BlockId; it is consensus behavior
and is not implemented here.

Embedding the body is recommended for V1 so one proposal is independently
verifiable. A body-reference-only alternative reduces propagation size but
requires availability/network semantics outside M1.2.

| Decision Item | Recommendation | Human Approval |
|---|---|---|
| SCHEMA-PROP-001 | Approve nine-field proposal body and three-field envelope | |
| SCHEMA-PROP-002 | Distinct ProposalId with proposed domain `0x0011` | |
| SCHEMA-PROP-003 | Embed complete body/evidence in V1 rather than network references | |
| SCHEMA-PROP-004 | Keep selection proof opaque but canonically typed only after its profile exists | |

### 5.6 AttestationV1

`AttestationBodyV1` proposed fixed array length `7`:

| # | Field | Normative type | Width | Presence | Validation | Sign | ID | Open dependency |
|---:|---|---|---|---|---|---|---|---|
| 0 | schema_version | u16 | V1=`1` | required | supported | yes | yes | common |
| 1 | chain_id | ChainId | 32 bytes | required | active chain | yes | yes | common |
| 2 | height | Height | u64 | required | structural | yes | yes | primitive |
| 3 | round_number | u32 | shortest CBOR | required | structural | yes | yes | Round review |
| 4 | block_id | BlockHash | 32 bytes | required | referenced header | yes | yes | header |
| 5 | validator_id | ParticipantId | 32 bytes | required | registry reference | yes | yes | participant |
| 6 | attestation_type | AttestationType | u16 | required | V1 `VALID_BLOCK` | yes | yes | enum |

`SignedAttestationV1` fixed array length `3`:

| # | Field | Type |
|---:|---|---|
| 0 | body | AttestationBodyV1 |
| 1 | signature_algorithm | SignatureAlgorithm |
| 2 | validator_signature | Ed25519 signature |

Operations:

```text
AttestationSigningPayload =
    Q1DomainFrameV1(ATTESTATION_SIGNING, CanonicalCBOR(AttestationBodyV1))

AttestationId =
    Q1HashV1(ATTESTATION_ID, CanonicalCBOR(SignedAttestationV1))
```

Proposed new domain `ATTESTATION_ID = 0x0012`.

The validator public key is not embedded. It is resolved from the committed
ParticipantRecord and role-checked. This avoids duplicate key material and
requires the relevant participant registry to be available.

ProposalId is excluded; the attestation approves the BlockId, which remains
stable independent of proposal packaging. Observational timestamp and
committee-membership proof are excluded. The certificate supplies committee
context; membership verification is behavior outside this schema.

| Decision Item | Recommendation | Human Approval |
|---|---|---|
| SCHEMA-ATT-001 | Approve seven-field body and three-field signed envelope | |
| SCHEMA-ATT-002 | Attest BlockId, not ProposalId | |
| SCHEMA-ATT-003 | Resolve validator key from ParticipantRecord; do not embed it | |
| SCHEMA-ATT-004 | Approve AttestationId domain `0x0012` | |
| SCHEMA-ATT-005 | Exclude timestamps and membership proofs from V1 attestation | |

### 5.7 FinalizationCertificateV1

#### Evidence Container Alternatives

| Option | Determinism | Evidence completeness | Complexity | V1 assessment |
|---|---|---|---|---|
| A. Sorted complete attestations | straightforward | self-contained signatures/bodies | largest | recommended |
| B. Signer IDs + signatures | compact | reconstructs shared body only if identical | medium | viable but specialized |
| C. Bitmap + signatures | committee-order dependent | needs external ordered committee | higher | defer |
| D. Aggregated signature | compact | new algorithm/proof rules | prohibited for V1 | reject |

Recommendation: Option A.

`FinalizationCertificateV1` proposed fixed array length `10`:

| # | Field | Normative type | Width/limit | Presence | Validation | Sign | ID | Open dependency |
|---:|---|---|---|---|---|---|---|---|
| 0 | schema_version | u16 | V1=`1` | required | supported | n/a | yes | common |
| 1 | chain_id | ChainId | 32 bytes | required | active chain | n/a | yes | common |
| 2 | height | Height | u64 | required | common to evidence | n/a | yes | primitive |
| 3 | round_number | u32 | shortest CBOR | required | common to evidence | n/a | yes | Round review |
| 4 | block_id | BlockHash | 32 bytes | required | common to evidence | n/a | yes | header |
| 5 | committee_root | MerkleHash | 32 bytes | required | approved profile | n/a | yes | committee/Merkle |
| 6 | total_committee_weight | u64 | shortest CBOR | required | claimed; recomputed by verifier | n/a | yes | committee rules |
| 7 | required_approving_weight | u64 | shortest CBOR | required | claimed; recomputed | n/a | yes | threshold rule |
| 8 | approving_weight | u64 | shortest CBOR | required | claimed; recomputed | n/a | yes | weight rules |
| 9 | attestations | sorted array SignedAttestationV1 | 1–65,535 | required | validator ID ascending, unique | n/a | yes | attestation |

Certificate ID:

```text
FinalizationCertificateId =
    Q1HashV1(FINALIZATION_CERTIFICATE,
             CanonicalCBOR(FinalizationCertificateV1))
```

The certificate is hashed, not independently signed. Its evidence consists of
the embedded individual signatures. Certificate ID is not serialized.

Including total/required/approving weights makes the claimed rule inspectable,
but values are never trusted: a verifier derives committee membership/weights
from the committee commitment and checks them. The threshold formula itself
remains a consensus decision.

Alternative: omit all three weight fields and derive them. Recommendation:
retain them for explicit evidence/version auditability, subject to exact
threshold approval.

| Decision Item | Recommendation | Human Approval |
|---|---|---|
| SCHEMA-CERT-001 | Option A: sorted complete Ed25519 attestations | |
| SCHEMA-CERT-002 | Sort by validator ParticipantId; reject duplicates/unsorted input | |
| SCHEMA-CERT-003 | Approve 10-field certificate and three explicit weight claims | |
| SCHEMA-CERT-004 | Hash under existing certificate domain; no certificate signature/self-ID | |
| SCHEMA-CERT-005 | Reject aggregation/bitmap designs for V1 | |

### 5.8 SnapshotManifestV1

Consensus-bound manifest recommendation: fixed array length `11`.

| # | Field | Normative type | Width | Presence | Validation | Sign | ID | Open dependency |
|---:|---|---|---|---|---|---|---|---|
| 0 | schema_version | u16 | V1=`1` | required | supported | n/a | yes | common |
| 1 | chain_id | ChainId | 32 bytes | required | active chain | n/a | yes | common |
| 2 | snapshot_type | SnapshotType | u16 | required | V1 full-state | n/a | yes | enum |
| 3 | finalized_height | Height | u64 | required | referenced finality | n/a | yes | primitive |
| 4 | finalized_block_id | BlockHash | 32 bytes | required | finalized reference | n/a | yes | header |
| 5 | finalization_certificate_id | GenericHash | 32 bytes | required | certificate reference | n/a | yes | certificate |
| 6 | state_root | StateHash | 32 bytes | required | reconstruction target | n/a | yes | state model |
| 7 | snapshot_format_version | u16 | V1=`1` | required | content profile | n/a | yes | format profile |
| 8 | chunk_count | u32 | 1–65,535 | required | exact manifest count | n/a | yes | chunk profile |
| 9 | chunk_root | MerkleHash | 32 bytes | required | snapshot chunk profile | n/a | yes | Merkle |
| 10 | total_uncompressed_size | u64 | shortest CBOR | required | exact reconstructed size | n/a | yes | resource limit |

Snapshot ID:

```text
SnapshotManifestId =
    Q1HashV1(SNAPSHOT, CanonicalCBOR(SnapshotManifestV1))
```

Compression is excluded from the consensus-bound manifest recommendation:
compression is a transport/storage representation and must not change the
snapshot identity. Each chunk commitment is over canonical uncompressed
chunk bytes. A separate operational descriptor may contain:

```text
[descriptor_version, snapshot_id, compression_algorithm,
 compressed_size, publisher_id_or_null, signature_algorithm_or_null,
 publisher_signature_or_null, created_at_or_null]
```

That descriptor is non-authoritative and is not hashed under `SNAPSHOT`.
Publisher signatures authenticate distribution, not consensus state, and
need a separate approved domain before implementation.

Chunk list alternatives:

- A: embed every `[index, hash, uncompressed_size]` entry;
- B: store only chunk count/root and distribute a separate committed chunk
  manifest;
- C: store a flat chunk-hash list.

Recommendation: B for bounded top-level size, but the separate chunk manifest
requires its own exact schema and domain/profile approval.

| Decision Item | Recommendation | Human Approval |
|---|---|---|
| SCHEMA-SNAP-001 | Approve 11-field consensus SnapshotManifest | |
| SCHEMA-SNAP-002 | Exclude compression/publisher/time metadata from SnapshotId | |
| SCHEMA-SNAP-003 | Bind finalized certificate ID and state root | |
| SCHEMA-SNAP-004 | Option B: separate committed chunk manifest | |
| SCHEMA-SNAP-005 | No publisher signature until a separate signing domain is approved | |

## 6. Merkle and Root Sub-Decision

### 6.1 Alternatives

| Choice | Alternative A | Alternative B | Recommendation |
|---|---|---|---|
| Arity | binary | higher arity | binary |
| Odd node | duplicate last | promote unchanged | promote unchanged (approved) |
| Empty root | fixed constant | domain hash of typed empty marker | typed empty marker |
| Leaves | raw item bytes | profile + index + length + canonical bytes | typed/indexed (approved) |
| Profiles | one shared | per-root construction | shared algorithm, distinct profile IDs |

### 6.2 Approved Common Binary Indexed-Sequence Profile

Approved append-only tree profile registry:

| ID | Semantic root type | Activation |
|---:|---|---|
| `0x0001` | TransactionRoot | approved for future implementation after explicit authorization |
| `0x0002` | ReceiptRoot | reserved inactive pending Receipt schema |
| `0x0003` | ParticipantRoot | approved for future implementation after explicit authorization |
| `0x0004` | SnapshotChunkRoot | reserved inactive pending canonical chunk descriptor |
| `0x0005` | StateRoot | reserved inactive pending separate state-model decision |

Profile IDs are never reused. Each root remains a distinct semantic type even
though all are represented by 32 bytes. Generic MerkleHash APIs must not allow
accidental root interchange. Reserving a profile does not activate it.

For each item at zero-based index `i`:

```text
leaf_payload =
    tree_profile_id:u16be
 || index:u64be
 || item_length:u64be
 || canonical_item_bytes

leaf_hash = Q1HashV1(MERKLE_LEAF, leaf_payload)

internal_payload =
    tree_profile_id:u16be
 || left_hash:bytes32
 || right_hash:bytes32

internal_hash = Q1HashV1(MERKLE_INTERNAL, internal_payload)

empty_payload =
    tree_profile_id:u16be
 || item_count:u64be

item_count = 0

empty_root = Q1HashV1(MERKLE_LEAF, empty_payload)
```

Rules:

- profile ID is exactly two unsigned big-endian bytes;
- index and item length are each exactly eight unsigned big-endian bytes;
- item length equals the exact canonical-item byte length;
- no normalization or Merkle-layer sorting is permitted;
- internal left/right positions are normative and child hashes are bytes32;
- a node and both children use the owning profile ID;
- cross-profile node reuse is invalid;
- zero items use the typed empty construction above, not an indexed leaf;
- one item has `root = leaf_hash` with no unary node;
- for two or more items, complete adjacent pairs are hashed left-to-right;
- an unpaired hash at any non-root level is promoted unchanged;
- the odd hash is never duplicated.

The root alone does not create an independent generic item-count field. Every
owning canonical schema must either serialize the exact count or derive it
unambiguously from another canonical committed object. The mechanism requires
per-schema approval before implementation.

Root classification:

| Root | Sequence/order | Approved leaf/status | Duplicate rule | Activation |
|---|---|---|---|---|
| TransactionRoot | exact block-body sequence | CanonicalCBOR(SignedTransferV1) | Merkle does not remove duplicates; later block rule decides TransferId duplicates | future implementation requires explicit authorization and bound transaction count |
| ReceiptRoot | not yet fixed | no leaf schema | requires later correspondence/order decision | reserved inactive |
| ParticipantRoot | ascending ParticipantId raw bytes | CanonicalCBOR(ParticipantRecordV1) | duplicate ParticipantId and unsorted input rejected before Merkle; encoder never sorts | future implementation requires explicit authorization and bound record count |
| SnapshotChunkRoot | not yet fixed | no leaf schema | requires canonical descriptor and continuity rules | reserved inactive |
| StateRoot | not applicable to this sequence profile | no state schema | requires separate state-key/value model | reserved inactive |

`StateRoot` cannot use the sequence profile until canonical state keys,
values, ordering, duplicate rejection, inclusion/non-inclusion and empty-state
semantics, update/proof models, and persistence assumptions are approved.

TransactionRoot preserves block-body order and never sorts or removes
transfers. Equal canonical values at different indexes produce different
leaves. The owning schema must bind `transaction_count`.

ParticipantRoot takes records already sorted by ascending raw ParticipantId,
rejects duplicates and unsorted input, and hashes canonical record bytes
rather than ParticipantRecordHash. The owning participant-set schema must bind
the exact record count.

ReceiptRoot and SnapshotChunkRoot have no approved leaf schema. Receipt
activation requires an exact receipt schema, transaction correspondence,
ordering, empty sequence, status/error encoding, emitted-data commitments, and
resource limits. Snapshot activation requires an exact chunk descriptor with
index, canonical uncompressed hash and size, continuity, total-count relation,
and duplicate/missing-index rejection.

Merkle leaf index is a raw big-endian `u64` in the leaf preimage. The general
65,535-item schema ceiling remains only an absolute format ceiling for current
owning-schema proposals; it is not a runtime allocation default, safe network
limit, block transaction limit, certificate limit, or genesis participant
limit. Operational and object-specific limits require separate approval and
will normally be lower. Implementations must not preallocate from an
attacker-provided item count alone.

Security consequences:

- typed profiles prevent cross-root reinterpretation;
- indexed leaves distinguish equal values at different positions;
- explicit empty roots avoid implementation constants;
- raw canonical bytes and length fields prevent concatenation ambiguity;
- odd-node promotion avoids introducing synthetic duplicate children;
- owning-schema count binding prevents hidden count assumptions;
- StateRoot remains blocked rather than forcing ledger-model decisions.

| Decision Item | Recommendation | Human Approval |
|---|---|---|
| SCHEMA-MERKLE-001 | Common binary indexed-sequence profile using existing leaf/internal domains | Approved with revision — 2026-07-24 |
| SCHEMA-MERKLE-002 | Approve profile IDs 1–4; reserve inactive StateRoot ID 5 | Approved — 2026-07-24 |
| SCHEMA-MERKLE-003 | Duplicate-last odd handling and typed empty-root construction | Approved with revision — 2026-07-24; duplicate-last rejected, odd-node promotion approved |
| SCHEMA-MERKLE-004 | Keep StateRoot in a separate state-model decision | Approved — 2026-07-24 |
| SCHEMA-MERKLE-005 | Approve receipt/chunk leaf schemas separately before activating their roots | Approved with condition — 2026-07-24 |

## 7. Fee and Economics Dependencies

Field shape can be separated from behavior:

| Field | Shape needed now | Behavior deferred |
|---|---|---|
| Transaction fee_limit | distinct fixed 16-byte unsigned type | fee formula, minimum, congestion, settlement |
| Genesis declared supply | Amount fixed 16 bytes | future issuance |
| Genesis allocations | recipient + Amount | account initialization execution |
| Block economic_rules_version | u16 registry reference | fee/reward formulas and distribution |
| Certificate weights | u64 evidence claims | quorum/threshold computation |

Recommendation:

- approve `FeeLimit` wire shape without approving fee execution;
- include no actual fee charged, fee total, reward amount, issuance amount, or
  treasury remainder in M1.2 objects;
- Session 5C removed `economic_rules_version` from Header V1. No independent
  economics selector, formula, or activation rule is approved.

| Decision Item | Recommendation | Human Approval |
|---|---|---|
| SCHEMA-ECON-001 | Freeze FeeLimit shape; defer all fee behavior | Approved — 2026-07-24 |
| SCHEMA-ECON-002 | Exclude computed fee/reward/issuance values from M1.2 schemas | |
| SCHEMA-ECON-003 | Decide whether economic_rules_version is independent or selected by protocol_version | |

## 8. Delay Dependency

### 8.1 Options

| Option | Header size | Replaceability | Verification packaging |
|---|---|---|---|
| A. Engine, difficulty, output, proof in header | unbounded/large | coupled | self-contained header |
| B. Engine, difficulty, output commitment in header; proof in proposal/body | fixed | strong | rejected for Header V1 |
| C. Separate DelayEvidence object referenced by ID | fixed | strongest | approved; registration/availability blocked |

Session 5C approved Option C: a required typed commitment to a separate
DelayEvidence object. The Header does not duplicate engine ID, difficulty, or
the full proof.

Approved base `DelayEvidenceV1` fixed array length `5`:

| # | Field | Type | Rule |
|---:|---|---|---|
| 0 | schema_version | u16 | V1=`1` |
| 1 | engine_id | DelayEngine | registered engine value |
| 2 | difficulty | u64 | shortest CBOR unsigned integer |
| 3 | output | bytes | engine-profile exact limit |
| 4 | proof | bytes | engine-profile exact limit |

Output and proof are definite-length byte strings. Missing, unknown, or
trailing fields are rejected. Engine-specific structures, limits, semantics,
and verification remain unapproved. The Header stores:

```text
delay_evidence_commitment: DelayEvidenceHash

DelayEvidenceHash =
    Q1HashV1(DELAY_EVIDENCE, CanonicalCBOR(DelayEvidenceV1))
```

The existing `DELAY_OUTPUT` domain MUST NOT be reinterpreted. `DELAY_EVIDENCE`
is approved in purpose but remains pending a separate normative domain
registration and numeric assignment. Until registration and independent
vectors, no valid DelayEvidenceHash can be produced and BlockHeaderV1
implementation remains blocked.

Engine `NONE` has the exact canonical form `[1, 0, 0, h'', h'']`: engine ID
zero, difficulty zero, and two definite zero-length byte strings. It uses the
normal DelayEvidenceHash path, not a constant, null, or absent commitment.
NONE is valid only for a Genesis/network profile that explicitly permits it
and is rejected where delay is mandatory.

No delay calculation or verification algorithm is defined here.

| Decision Item | Recommendation | Human Approval |
|---|---|---|
| SCHEMA-DELAY-001 | Option C: typed full-evidence commitment in Header; evidence external | Session 5C decision recorded; implementation blocked |
| SCHEMA-DELAY-002 | Approve five-field DelayEvidenceV1 base shape | Session 5C base shape approved; engine profiles remain unapproved |
| SCHEMA-DELAY-003 | Allocate new DELAY_EVIDENCE domain; do not reuse DELAY_OUTPUT | Approved pending separate normative registration — 2026-07-25 |
| SCHEMA-DELAY-004 | Approve exact `NONE` evidence representation | Approved conditionally by network profile — 2026-07-25 |

## 9. New Domain Assignments Requiring Review

Proposed append-only assignments:

| ID | Symbol | Payload | Output |
|---:|---|---|---|
| `0x0010` | `CHAIN_ID` | approved revised canonical ChainIdentityPreimageV1 | SHA-256; pending normative registration |
| `0x0011` | `PROPOSAL_ID` | canonical complete SignedBlockProposalV1 | SHA-256 |
| `0x0012` | `ATTESTATION_ID` | canonical complete SignedAttestationV1 | SHA-256 |
| `0x0013` | `PARTICIPANT_ID` | canonical ParticipantIdentityBodyV1 | SHA-256 |

Session 1 approved `CHAIN_ID = 0x0010`, and Session 2 approved
`PARTICIPANT_ID = 0x0013`; both remain pending normative registry registration.
IDs `0x0011` and `0x0012` remain proposals. No implementation or registry edit
is authorized by this table.

| Decision Item | Recommendation | Human Approval |
|---|---|---|
| SCHEMA-DOMAIN-001 | Register proposed IDs `0x0011`–`0x0012`; `0x0010` and `0x0013` are approved pending registration | |

## 10. Security Consequences

Positive consequences if approved:

- fixed schemas eliminate field-order and optional-value ambiguity;
- chain binding and role-specific domains reduce replay/cross-role attacks;
- body/envelope separation removes circular signing;
- self-ID exclusion removes normalization tricks;
- sorted evidence makes certificate bytes deterministic;
- full attestations preserve individual Ed25519 verification;
- typed roots prevent commitment reuse;
- operational snapshot metadata cannot alter consensus identity.

Residual risks:

- new domain assignments require independent cross-language vectors;
- ChainId nonce generation must be auditable and never silently reused;
- body embedding can create large proposals/certificates and requires limits;
- external genesis parameter commitments are not independently useful until
  their manifests are specified;
- certificate weight fields may be misread as trusted unless APIs clearly
  require recomputation;
- StateRoot remains blocked by the state model;
- delay and selection proof byte schemas remain dependencies;
- M1.1 Round and M1.2 RoundNumber require reconciliation.

No recommendation authorizes mainnet, aggregation, economic execution,
committee selection, finality, Merkle implementation, or delay execution.

## 11. Compatibility Consequences

- every accepted field table becomes consensus-critical and must be versioned
  for future changes;
- fixed bytes32 IDs are simple across Rust, Node, Python, Go, and WebAssembly;
- shortest CBOR unsigned integers are language-neutral but semantic bounds
  must be enforced explicitly;
- explicit null is compatible with the approved CBOR profile;
- embedding complete evidence favors independent verification over bandwidth;
- API JSON must use lowercase hex for fixed IDs/hashes and decimal strings for
  monetary values;
- unknown versions/enums are rejected rather than ignored, so upgrades require
  explicit activation;
- old object IDs remain stable because version meanings and domain IDs are
  never reused.

## 12. Unresolved Alternatives and Blocking Items

The following remain blocked or require later human decisions:

1. Lower object-specific resource limits below the approved format ceiling.
2. RESOLVED: approved domains `0x0010` and `0x0013` are registered.
3. Genesis external parameter/delay commitments and their source schemas.
4. Allocation purpose/vesting exclusion.
5. ReceiptRoot leaf schema and activation.
6. RESOLVED: standalone RoundNumber correction closed in docs/44.
7. Proposal selection-proof and body-packaging semantics.
8. Attestation ID domain necessity.
9. Certificate weight claims versus fully derived fields.
10. Snapshot separate chunk-manifest schema/domain.
11. StateRoot state-model decision.
12. General economic activation remains open; LOCALNET v0 fee policy is
    separately approved, without restoring the omitted header field.
13. DelayEvidence engine profiles/resource bounds, normative domain
    registration, and network-specific authorization of `NONE`.
14. Exact object/collection byte-size ceilings.

Until selected, the affected schema remains implementation-blocked.

## 13. Proposed Approval Groups

Approving all eight together is not recommended. Grouped approval reduces
coupling and permits useful conformance work without silently deciding
consensus algorithms.

### Group A — Common Rules

- common rules and enum registry;
- ChainId, subject to normative domain registration.

Sessions 1 and 2 approved this group through SCHEMA-COMMON-001–022, subject
to their recorded conditions and the required M1.1 RoundNumber correction.

### Group B — Reusable Types and ParticipantRecord

- reusable types;
- ParticipantRecord;
- node-identity choice;
- RoundNumber reconciliation.

Session 2 approved this group through SCHEMA-PART-001–005. Implementation
remains unauthorized. `PARTICIPANT_ID=0x0013` remains pending normative
registration.

### Group C — SignedTransfer

- TransferBodyV1 and SignedTransferV1;
- FeeLimit shape;
- transfer-only type policy;
- memo policy.

Session 3 approved this group through SCHEMA-TX-001–006 and approved
SCHEMA-ECON-001. Implementation and all transfer behavior remain
unauthorized.

### Group D — Separate Merkle Decision

- sequence Merkle profile;
- profile IDs, empty root, odd-node behavior, and leaf schemas.

Session 4 approved SCHEMA-MERKLE-001–005 with odd-node promotion,
typed-empty roots, the append-only profile registry, and conditional profile
activation. No Merkle implementation or vectors are authorized.

### Group E — BlockHeader

- BlockHeader and BlockId;
- root fields and delay/body commitments.

Session 5A approved SCHEMA-BLOCK-001's separate body/envelope form,
SCHEMA-BLOCK-002's complete-envelope BlockId, ParentReferenceV1, ParentKind,
and the height-one GenesisId boundary.

Session 5B approved required TransactionRoot, ParticipantRoot, and StateRoot
membership; minimal BlockBodyV1; canonical ParticipantSetV1; derived semantic
counts; the pre-H producer-membership invariant; and revised
SCHEMA-BLOCK-004/005 omissions. Final field indexes, complete membership, and
array length were deferred to Session 5C.

Session 5C approved the exact eleven-field BlockHeaderBodyV1, required
protocol_version and RoundNumber, complete exclusion set, typed
DelayEvidenceHash commitment, five-field DelayEvidenceV1 base shape, and
conditional NONE form. SCHEMA-BLOCK-003 and SCHEMA-BLOCK-006 are approved.
Implementation and runtime activation remain blocked; Session 6 is not
automatically authorized.

### Group F — Attestation

- Attestation body/envelope and AttestationId.

Dependency: stable BlockId from Group E.

### Group G — BlockProposal

- proposal body/envelope and ProposalId.

### Group H — FinalizationCertificate

- evidence container, ordering, weights, and CertificateId.

### Group I — GenesisManifest

- allocations, participants, and parameter commitments.

### Group J — SnapshotManifest and DelayEvidence

- snapshot/chunk structures;
- delay evidence and domain usage.

Dependencies flow in the order above.

Session 1 approved the ordered gate sequence above. Sessions 2–4 recorded
Groups B–D respectively. Sessions 5A–5C completed Group E's documentation
schema while leaving implementation and activation blocked. Session 6 is not
automatically authorized.

| Decision Item | Recommendation | Human Approval |
|---|---|---|
| SCHEMA-GATE-001 | Use grouped approval rather than approve all eight atomically | Approved — 2026-07-24 |
| SCHEMA-GATE-002 | Use revised order Common; reusable/participant; transaction; Merkle; header; attestation; proposal; certificate; genesis; snapshot/delay | Revised and approved — 2026-07-24 |
| SCHEMA-GATE-003 | Separate Merkle decision before BlockHeader and compound commitment-bearing objects | Approved — 2026-07-24 |

## 14. Proposed Implementation Order After Approval

No implementation begins from this package alone. After relevant approvals:

1. register approved common rules and obtain any required implementation gate;
2. register the approved reusable types and ParticipantRecord decisions and
   complete the separately authorized M1.1 RoundNumber correction;
3. register the approved TransferBodyV1 and SignedTransferV1 decisions;
4. register the approved Merkle profile decision and satisfy each owning
   schema's count-binding and activation dependencies;
5. approve BlockHeader and stabilize BlockId;
6. approve Attestation;
7. approve BlockProposal;
8. approve FinalizationCertificate;
9. approve GenesisManifest;
10. approve SnapshotManifest and DelayEvidence;
11. implement only the groups separately authorized after those approvals;
12. run Rust plus independent Node/Python canonical and rejection vectors at
    every object gate.

No ledger or consensus execution is implied.

## 15. Human Decision Record

Decision owner: Yousef Bahrami

Review date: 2026-07-25 (latest review; Sessions 1 through 5C)

Latest gate outcome: DEC-Q1-027 SESSION 5C — APPROVED WITH IMPLEMENTATION AND
ACTIVATION BLOCKERS

Allowed outcomes:

- `APPROVE ALL SCHEMA RULES`
- `APPROVE COMMON RULES AND SELECTED OBJECTS`
- `REVISE SPECIFIC OBJECT SCHEMAS`
- `CREATE SEPARATE MERKLE DECISION`
- `REPEAT M1.2 REVIEW`

Global approval notes: Session 1 reviewed SCHEMA-COMMON-001 through
SCHEMA-COMMON-019 and SCHEMA-GATE-001 through SCHEMA-GATE-003. Session 2
reviewed SCHEMA-COMMON-020 through SCHEMA-COMMON-022 and SCHEMA-PART-001
through SCHEMA-PART-005. Session 3 reviewed SCHEMA-TX-001 through
SCHEMA-TX-006 and SCHEMA-ECON-001. Session 4 reviewed SCHEMA-MERKLE-001
through SCHEMA-MERKLE-005. Session 5A reviewed SCHEMA-BLOCK-001 and
SCHEMA-BLOCK-002 and approved the related parent/height boundary. Session 5B
reviewed SCHEMA-BLOCK-004 and SCHEMA-BLOCK-005 and approved the related
body/root/count decisions. Session 5C reviewed SCHEMA-BLOCK-003 and
SCHEMA-BLOCK-006, finalized the eleven-field Header, and recorded the delay
commitment architecture. All later object decision rows remain blank and
non-normative.

Implementation authorization, if any: NONE

Next milestone authorization: **NONE**

## 16. No-Code Confirmation

This package creates no Rust structs, codecs, parsers, validators, golden
vectors, crates, Merkle implementation, delay implementation, fee/reward
behavior, consensus logic, networking, wallet, or runtime service.

Sessions 1 through 5C are recorded. Session 5C approves the documentation
schema but no implementation, valid instantiation, runtime activation, or
later object schema.
