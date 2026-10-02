# DEC-Q1-027 Session 5B — Roots and Body Decision Brief

Status: **ENGINEERING REVIEW ONLY — NO HUMAN DECISION RECORDED**

Review Date: 2026-07-25

Implementation Authorization: **NONE**

## 1. Inherited Approved Dependencies

- SignedBlockHeaderV1 is an exact three-field envelope over a canonical
  BlockHeaderBodyV1; BlockId hashes the complete signed envelope.
- GenesisManifest is independent. Signed-block height starts at 1 and
  ParentReferenceV1 binds either the governing GenesisId or the immediately
  preceding BlockId.
- TransferBodyV1 and SignedTransferV1 are fixed. TransactionRoot leaf bytes
  are `CanonicalCBOR(SignedTransferV1)` in exact block-body order.
- ParticipantRecordV1 is fixed. ParticipantRoot leaf bytes are
  `CanonicalCBOR(ParticipantRecordV1)` ordered by ascending raw ParticipantId.
- The common indexed-sequence Merkle profile commits profile ID, zero-based
  leaf index, canonical item length, and exact canonical item bytes.
- Odd hashes are promoted unchanged. Empty roots are typed. The Merkle layer
  never sorts, normalizes, or removes duplicates.
- An owning canonical schema must bind exact item count either directly or by
  unambiguous derivation from another canonical committed object.
- TransactionRoot and ParticipantRoot are distinct future-implementation
  profiles. ReceiptRoot and StateRoot remain inactive.
- Facts deterministically derivable from canonical fields should not be
  serialized again, and inactive V1 semantics should not receive reserved
  fields.
- The complete BlockHeaderBodyV1 membership, order, and length remain
  undecided.

## 2. Transaction Commitment Alternatives

| Alternative | Count binding | Advantages | Disadvantages |
|---|---|---|---|
| A. `transaction_count` + TransactionRoot in Header | direct Header count; must equal leaf count | Header independently states count; simple light-client bounds; count is inside BlockId | duplicates a count derivable from canonical transfer array; creates two values requiring equality; freezes count placement |
| B. TransactionRoot only; count from canonical BlockBody | exact transfer-array length | one source of truth; follows derived-fact rule; no extra Header field | Header alone does not reveal count; verifier needs canonical body or authenticated metadata |
| C. BlockBodyId/commitment only | defined by body schema | can commit all body bytes and future sections | loses the approved typed TransactionRoot interface; needs a body identity/domain decision; weaker transfer-specific proof ergonomics |
| D. TransactionRoot + generic body commitment | root count mechanism plus body commitment | could bind non-transfer sections independently | redundant when transfers are the only approved content; disagreement between commitments is possible; generic commitment semantics are undefined |

The approved Merkle rule does not require a direct Header count. It permits
count derivation from an exact canonical committed owning object. That path is
deterministic only if the canonical BlockBody transfer sequence is uniquely
available and its array length is the sole transaction count.

Duplicate SignedTransferV1 values remain separate leaves because indexes are
committed. Merkle construction neither rejects nor removes them. Duplicate
TransferId policy remains a separate validation dependency.

### Recommendation

Prefer B with a normative minimal BlockBodyV1. Derive TransactionCount from
the exact transfer-array length and include only TransactionRoot in the
Header. Do not serialize the same count in both body and Header.

This favors one source of truth over Header-only count visibility. If human
review instead requires a self-describing Header for light clients, select A
explicitly and treat equality with body length as a mandatory invariant.

## 3. Canonical BlockBody Alternatives

| Alternative | Count source | Root reconstruction | Separate body ID/domain | Compatibility |
|---|---|---|---|---|
| A. `[schema_version, transfers]` | derived from array length | hash each canonical SignedTransferV1 at its array index | unnecessary for approved V1 content | future non-transfer sections require BlockBodyV2 |
| B. `[schema_version, transaction_count, transfers]` | serialized and also derivable | same as A plus equality check | unnecessary | permanently retains a redundant count |
| C. no canonical body; exact transfer array carried elsewhere | array length | possible only if container semantics and exact bytes are fixed elsewhere | none | couples root reconstruction to a later container; weak independent availability contract |
| D. extensible multi-section body with reserved positions | section dependent | section-specific | likely needs additional commitment semantics | reserves inactive meanings and risks ambiguous empty sections |

Only signed transfers are approved V1 body content. Receipts and other future
evidence do not justify reserved V1 positions.

### Recommendation

Prefer A:

```text
BlockBodyV1 = [
    schema_version,
    transfers
]
```

This is a candidate shape for human review, not an approved schema. The
transaction count is the exact array length. TransactionRoot reconstructs
from those items in order. No separate BlockBodyId, hash domain, or generic
body commitment adds information for the currently approved V1 content.
Future sections should require BlockBodyV2 rather than reserved V1 fields.

## 4. TransactionCount

Candidate semantic type:

```text
TransactionCount: u32
canonical encoding: shortest CBOR unsigned integer
```

The semantic value equals the exact number of TransactionRoot leaves and the
exact length of BlockBodyV1's transfer array. Zero is valid and maps to the
typed empty TransactionRoot. The general format ceiling and any lower runtime
limit are separate from the `u32` type ceiling; this review authorizes no
operational limit.

| Placement | Consequence |
|---|---|
| Header only | direct BlockId-bound count, but body still supplies a derived length |
| BlockBody only | serialized count duplicates array length |
| both | three consistency surfaces: Header, body field, and array length |
| neither as a field | semantic count derives once from the canonical array |

### Recommendation

Approve TransactionCount later as a distinct API/validation semantic type, but
do not serialize it in V1 Header or BlockBody when the canonical transfer
array is available. Its value is derived, bounded to `u32`, and must equal the
TransactionRoot leaf count.

## 5. Participant Commitment Alternatives

| Alternative | Count binding | Advantages | Disadvantages and circularity |
|---|---|---|---|
| A. `participant_count` + ParticipantRoot in Header | direct | Header states both values | count duplicates canonical set length; set reference time still required |
| B. ParticipantRoot only; count from canonical ParticipantSet | set-record array length | one count source; exact records support membership/key resolution | requires an approved ParticipantSet object and availability |
| C. ParticipantSetId/commitment instead of direct root | object-defined | may bind reference height and records together | needs a new object identity/domain; loses direct approved-root interface |
| D. CommitteeRoot instead of ParticipantRoot | committee size | potentially smaller validation set | changes meaning from participant registry to selected subset; selection rules are not in this review |

Possible temporal meanings:

1. active set governing validation at height H, derived from finalized state
   before executing H;
2. set produced after H;
3. set governing H+1;
4. another separately versioned snapshot.

Meaning 1 avoids self-reference: header validity and producer membership are
checked against already finalized pre-H state. Meaning 2 is circular because
the block's validity would depend on a set produced by executing the block
being validated. Meaning 3 introduces an activation transition commitment
whose derivation is not yet defined.

### Recommendation

Prefer B. ParticipantRoot should represent the active participant set
governing validation at height H, deterministically derived from finalized
state before executing H. Count should derive from the canonical
ParticipantSet record array. This temporal rule requires explicit human
approval and must not be inferred from this review.

## 6. ParticipantCount

Candidate semantic type:

```text
ParticipantCount: u32
canonical encoding: shortest CBOR unsigned integer
```

It equals both the exact canonical ParticipantSet record-array length and the
ParticipantRoot leaf count. Zero participants is invalid under the reviewed
active-set model. The `u32` type ceiling is not an allocation instruction or
operational limit; lower private-testnet and runtime limits require separate
approval.

### Recommendation

Approve ParticipantCount later as a distinct API/validation semantic type, but
derive it from the canonical ParticipantSet rather than serializing it in both
Header and set. Do not permit implicit interchange with TransactionCount.

## 7. Producer Membership Invariant

Candidate exact rule:

```text
producer_id identifies an active PRODUCER record
in the participant set committed by ParticipantRoot
for signed-block height H
```

- structural validation checks typed fields and canonical object forms;
- historical/state validation reconstructs the active pre-H set, verifies its
  root, resolves producer_id to ParticipantRecordV1, applies
  `activation_height <= H` and
  `(deactivation_height is null or H < deactivation_height)`, and obtains the
  producer public key from that record;
- the producer signature is verified with the resolved producer role key;
- this participant-set commitment alone does not decide whether the same set
  governs validator evidence or whether a separate committee commitment is
  required; those are later decisions.

### Recommendation

Adopt the invariant above only with the pre-H finalized-state meaning. Treat
membership and key resolution as historical/state validation, not structural
parsing.

## 8. ReceiptRoot Alternatives

| Alternative | Advantages | Disadvantages |
|---|---|---|
| A. omit from BlockHeaderV1 | no meaningless field; matches inactive profile and absent leaf schema | receipt-enabled headers require a new schema version |
| B. required typed empty ReceiptRoot | fixes placement | falsely implies an active, reconstructable receipt commitment |
| C. nullable ReceiptRoot | explicit absence | reserves inactive V1 semantics and creates future null/non-null transition rules |
| D. opaque bytes32 placeholder | fixed width | no verifiable meaning; can be mistaken for security commitment |

### Recommendation

Choose A. ReceiptRoot should be absent from BlockHeaderV1. A future
receipt-enabled header must use a new schema version after receipt object,
ordering, correspondence, empty-sequence, and activation rules are approved.

## 9. StateRoot Alternatives

| Alternative | Advantages | Disadvantages |
|---|---|---|
| A. omit until state model approval | no false commitment; current schema can be documented independently | ledger-capable activation requires a new header version and changes BlockId shape |
| B. required typed StateRoot field, construction blocked | reserves a security-essential typed position and future BlockId stability | no valid value can be produced until state model approval; header remains non-implementable |
| C. nullable StateRoot | permits pre-state-model objects | creates signed headers with no state commitment and ambiguous activation transition |
| D. generic opaque state commitment | superficially future-flexible | erases semantic and algorithm boundaries; implies unverifiable security |

A ledger-capable finalized block should not operate without a deterministic
state commitment. Field shape, construction algorithm, and runtime activation
are separate gates:

1. field shape may be documented as required typed bytes32;
2. construction remains blocked on canonical state model;
3. runtime activation remains forbidden until construction and vectors are
   approved.

### Recommendation

Prefer B for long-term BlockId stability, with a hard consequence:
BlockHeaderV1 cannot be implemented, instantiated as valid, or activated until
the separate StateRoot construction is approved. Do not activate profile
`0x0005`, accept arbitrary bytes, or use a generic commitment.

If the project requires an implementable pre-state-model header, select A and
accept that ledger-capable operation must use BlockHeaderV2.

## 10. Block-Body Commitment Alternatives

| Alternative | Advantages | Disadvantages |
|---|---|---|
| A. omit generic `block_body_commitment` | no redundancy; TransactionRoot has exact approved semantics | future non-transfer content requires a new body/header version |
| B. retain beside TransactionRoot | could cover whole body bytes | duplicates transfer commitment; undefined disagreement handling; needs exact algorithm/domain |
| C. replace TransactionRoot | one general commitment | loses typed transfer-root semantics and proof profile |
| D. commit to canonical multi-section body | extensible | invents inactive sections and commitment semantics |

No approved V1 body content exists beyond signed transfers.

### Recommendation

Choose A. Omit generic block_body_commitment. TransactionRoot is sufficient
for the candidate minimal transfer-only BlockBodyV1. Do not invent future
sections to justify a V1 field.

## 11. Typed Root and Count Boundaries

| Type | Representation | Reviewed status |
|---|---|---|
| TransactionCount | semantic `u32`, shortest CBOR if serialized | recommended as derived API/validation type, not a V1 field |
| ParticipantCount | semantic `u32`, shortest CBOR if serialized | recommended as derived API/validation type, not a V1 field |
| TransactionRoot | distinct bytes32 | approved profile; future implementation requires authorization |
| ParticipantRoot | distinct bytes32 | approved profile; future implementation requires authorization |
| ReceiptRoot | distinct bytes32 | reserved inactive; not constructible as active |
| StateRoot | distinct bytes32 | reserved inactive; not constructible as active |

Implicit conversions between count types or root types must be prohibited.
Generic MerkleHash must not bypass semantic boundaries. Inactive root types
may exist as type names but must not expose active constructors or validation
claims.

## 12. Determinism, Security, and Compatibility Consequences

### Determinism

- one canonical transfer array produces one derived TransactionCount and one
  ordered TransactionRoot;
- one canonical pre-H participant set produces one ParticipantCount and one
  sorted ParticipantRoot;
- serializing counts twice introduces rejectable disagreement states;
- generic commitments without exact canonical inputs create client-specific
  interpretations.

### Security

- a post-H ParticipantRoot creates circular producer authorization;
- an inactive or opaque root field creates a false security signal;
- root-type interchange can validate the wrong committed object class;
- unchecked attacker-provided counts can drive unsafe allocation;
- redundant commitments can be selectively matched by different clients;
- omission of StateRoot prevents safe ledger activation until a later header
  version or state decision supplies a real commitment.

### Compatibility

- omitting ReceiptRoot intentionally requires a new header schema for receipts;
- including required StateRoot shape preserves its position and BlockId shape
  but blocks all V1 header activation until construction exists;
- minimal BlockBodyV1 forces future non-transfer sections into BlockBodyV2;
- derived counts keep V1 compact but require canonical owning objects for
  independent reconstruction;
- adding any omitted Header field later requires a new BlockHeader schema
  version and changes BlockId.

## 13. Hidden Dependencies and Circularity Risks

- TransactionRoot depends on exact BlockBodyV1 availability and transfer order.
- TransactionCount depends on the owning array and must fit `u32`.
- ParticipantRoot depends on a canonical ParticipantSet definition, reference
  height, ordering, and availability.
- Producer verification depends on resolving ParticipantRecordV1 from the
  pre-H committed set.
- StateRoot field activation depends on a separate complete state model.
- ReceiptRoot depends on an approved receipt object and correspondence model.
- Header-alone light-client verification is weaker when counts are derived
  from external canonical objects.
- Any body identity or generic body commitment would require separate
  canonical-input and domain decisions.

## 14. Candidate Reduced Header Subset

The following is a review candidate, not a normative schema or final field
order:

```text
BlockHeaderBodyV1 candidate fields justified so far:

schema_version
chain_id
height
parent_reference
producer_id
transaction_root
participant_root
state_root
```

Interpretation under this brief's recommendations:

- TransactionRoot count derives from candidate minimal BlockBodyV1.
- ParticipantRoot count derives from a canonical pre-H ParticipantSet.
- StateRoot is required in shape but blocks implementation and activation
  until its construction is approved.
- No TransactionCount, ParticipantCount, ReceiptRoot, or generic
  block_body_commitment field is included.

This subset does not approve membership, indexes, array length, or any omitted
Header topic.

## 15. Topics Deferred to Session 5C or Later

- human decisions on every recommendation in this brief;
- exact BlockHeaderBodyV1 membership, positions, and array length;
- canonical BlockBodyV1 approval;
- canonical ParticipantSet object and reference-height rule;
- TransactionCount and ParticipantCount type approval;
- duplicate TransferId validation;
- Receipt object and future receipt-enabled header version;
- complete StateRoot state model and activation;
- operational count and resource limits;
- all explicitly excluded Session 5B topics;
- implementation, vectors, and runtime behavior.
