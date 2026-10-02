# DEC-Q1-027 Session 5 BlockHeader Decision Brief

Status: **DOCUMENTATION REVIEW ONLY — NO SESSION 5 DECISIONS RECORDED**

Review Scope: SCHEMA-BLOCK-001 through SCHEMA-BLOCK-006

Review Date: 2026-07-25

Implementation Authorization: **NONE**

## 1. Approved Dependencies Inherited from Sessions 1–4

- canonical objects use fixed-position arrays, explicit versions, exact field
  counts, strict types, and no trailing fields;
- `ChainId` is a distinct bytes32 value; `Height` is `u64`;
  `RoundNumber` is a distinct `u32` using shortest canonical CBOR;
- consensus round context is `(Height, RoundNumber)`; Slot cannot substitute
  for Height;
- typed hashes and semantic Merkle roots cannot be interchanged;
- `ParticipantId` is bytes32 derived only from the approved role-key identity
  body; node transport identity is excluded;
- signed envelopes separate canonical Body, signature algorithm, and
  signature; a signature is excluded from its own signing payload;
- an official signed-object ID hashes the complete signed envelope, includes
  the signature, and is never serialized into its own preimage;
- TransactionRoot profile `0x0001` commits to canonical SignedTransferV1
  values in exact block-body order;
- ParticipantRoot profile `0x0003` commits to canonical ParticipantRecordV1
  values ordered by ascending raw ParticipantId;
- Merkle odd nodes are promoted unchanged; typed empty roots and profile-bound
  leaf/internal framing are fixed;
- every owning schema must bind its exact Merkle item count;
- ReceiptRoot `0x0002` and StateRoot `0x0005` remain reserved inactive.

## 2. Remaining Normative Choices

| Decision | Choice still required |
|---|---|
| SCHEMA-BLOCK-001 | final body field set, exact array length/order, and whether every proposed field belongs in V1 |
| SCHEMA-BLOCK-002 | BlockId naming/type and exact application of the approved complete-envelope ID rule to SignedBlockHeaderV1 |
| SCHEMA-BLOCK-003 | confirmation that timestamp, fork-choice data, computed economic values, and full delay proof are absent; exact treatment of any compact delay commitment |
| SCHEMA-BLOCK-004 | omit/null/include ReceiptRoot while its profile is inactive; no receipt leaf schema currently exists |
| SCHEMA-BLOCK-005 | retain or remove `block_body_commitment`; if retained, its body schema, domain, algorithm, and relation to TransactionRoot must be fixed |
| SCHEMA-BLOCK-006 | complete the required M1.1 RoundNumber correction before schema implementation; decide no timing or transition behavior here |

Additional choices required by the proposed field set:

- parent linkage and the genesis boundary representation;
- whether `candidate_index` is header identity or proposal/selection evidence;
- which participant set ParticipantRoot commits to and how record count is
  bound;
- direct `transaction_count` versus derivation from an approved canonical
  BlockBody;
- StateRoot construction and activation under a separate state-model decision;
- delay-engine fields, `NONE` representation, difficulty interpretation, and
  output-commitment schema;
- whether `economic_rules_version` is independent or selected by
  `protocol_version`.

## 3. Fields Fixed Now

Only the following header-level facts are fixed. They do not approve the
proposed 17-field body as a whole.

| Field/form | Fixed fact |
|---|---|
| `schema_version` | required at body index 0; semantic `u16`; V1 is `1` |
| signed envelope | if approved as a signed object, form is `[Body, signature_algorithm, signature]` |
| signing form | canonical Body is domain-framed; signature is not inside its own payload |
| object ID form | complete signed envelope is hashed; self ID is absent |

The following candidate fields have fixed semantic types/encodings if human
review retains them, but their inclusion and positions are not yet approved:

| Candidate field | Fixed inherited type/encoding |
|---|---|
| `chain_id` | distinct ChainId, bytes32 |
| `height` | distinct Height, `u64` |
| `round_number` | distinct RoundNumber, `u32`, shortest CBOR |
| `parent_block_id` | typed block hash, bytes32 |
| `producer_id` | distinct ParticipantId, bytes32 |
| `candidate_index` | distinct CandidateIndex, `u16`, shortest CBOR |
| `transaction_root` | distinct TransactionRoot, bytes32, profile `0x0001` |
| `participant_root` | distinct ParticipantRoot, bytes32, profile `0x0003` |
| `receipt_root` | distinct ReceiptRoot, bytes32, profile `0x0002`, inactive |
| `state_root` | distinct StateRoot, bytes32, profile `0x0005`, inactive |

## 4. Fields Still Blocked

| Candidate field | Blocking reason |
|---|---|
| `protocol_version` | presence is not approved for this schema; relation to independent economic version remains open |
| `parent_block_id` | genesis/first-successor linkage and zero/sentinel policy are not fixed |
| `candidate_index` | selection semantics and whether the value belongs in BlockId are not fixed |
| `transaction_root` | construction is fixed, but owning BlockBody and exact transaction-count binding are not |
| `receipt_root` | profile inactive; Receipt schema, ordering, correspondence, status/error form, and activation are absent |
| `state_root` | profile inactive; state keys, values, order, duplicates, empty state, updates, proofs, and persistence are absent |
| `participant_root` | construction is fixed, but owning participant-set definition, reference height, and record-count binding are not |
| `delay_engine_id` | conditional `NONE` exists, but applicable profile and header inclusion are not fixed |
| `delay_difficulty` | engine-specific meaning and configuration binding are not fixed |
| `delay_output_commitment` | committed object, exact bytes/type, and evidence relationship are not fixed |
| `economic_rules_version` | independent field versus derivation from protocol version is undecided |
| `block_body_commitment` | presence, committed body bytes, domain, algorithm, and overlap with typed roots are undecided |

Consequently, the proposed body length `17`, every position after index 0,
and the complete SignedBlockHeaderV1 schema remain unapproved.

## 5. Hidden Coupling

| Area | Coupling |
|---|---|
| Transfer | TransactionRoot commits to exact canonical SignedTransferV1 order. Any BlockBody filtering, reordering, duplicate policy, or count representation changes the root/header contract. |
| Merkle | TransactionRoot and ParticipantRoot need owning-schema count binding. ReceiptRoot and StateRoot cannot be activated by placing bytes32 fields in a header. Root types must remain non-interchangeable. |
| ParticipantRecord | `producer_id` resolves role keys through ParticipantRecordV1. ParticipantRoot depends on the exact participant-set snapshot, activation-height rule, sorted raw ParticipantId order, and count. |
| Future FinalizationCertificate | Certificate and attestations are expected to reference a stable BlockId. Changing header membership, signature inclusion, roots, or BlockId derivation later would change the object that validators attest and certificates finalize. |

## 6. Consensus Risks

- ambiguous field membership or order creates different canonical header bytes,
  signatures, and BlockIds across clients;
- activating inactive roots with placeholder bytes creates false commitments
  that cannot be independently reconstructed;
- omitting Merkle count binding permits distinct owning-object interpretations
  around the same root;
- confusing ParticipantRoot reference height can make producer eligibility and
  validator-set interpretation circular or fork-dependent;
- putting selection evidence such as candidate index into BlockId without a
  stable selection model can make equivalent blocks have different identities;
- redundant body and transaction commitments can disagree unless their exact
  relationship is normative;
- an underspecified parent/genesis sentinel can create incompatible chain
  linkage at the boundary;
- delay or economic fields without approved interpretation can make header
  validity depend on unstated consensus behavior;
- changing BlockId after attestation/certificate schemas are built invalidates
  signatures, evidence references, and finality interoperability;
- using Slot/composite Round instead of `(Height, RoundNumber)` would violate
  the approved round context and create cross-client identity divergence.
