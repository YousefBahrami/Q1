# DEC-Q1-027 Session 5C — Final Header Engineering Brief

Status: **ENGINEERING REVIEW ONLY — NO HUMAN DECISION RECORDED**

Review Date: 2026-07-25

Implementation Authorization: **NONE**

## 1. Inherited Decisions

- BlockHeaderBodyV1 is a canonical Body inside the exact three-field
  SignedBlockHeaderV1 envelope.
- BlockId hashes canonical SignedBlockHeaderV1, including algorithm and
  producer signature.
- signed-block height starts at 1; ParentReferenceV1 contains the governing
  GenesisId at height 1 and the immediately preceding BlockId above height 1.
- schema version, ChainId, Height, ParentReferenceV1, producer_id,
  TransactionRoot, ParticipantRoot, and StateRoot are approved eventual Header
  members; final indexes and length are not approved.
- TransactionRoot commits to canonical BlockBodyV1 transfer order.
- ParticipantRoot commits to the active participant set governing H from
  finalized pre-H state.
- ReceiptRoot and generic block-body commitment are omitted.
- StateRoot is required in shape, but its construction is inactive and blocks
  BlockHeaderV1 implementation and valid instantiation.
- schema version and protocol version are distinct concepts.
- consensus round context is `(Height, RoundNumber)`; RoundNumber is a distinct
  `u32`, starts at zero unless later changed, and Slot cannot substitute for
  either component.
- DelayEngine `NONE=0` is conditionally permitted only by an applicable
  approved network/genesis profile.

## 2. Protocol-Version Alternatives

| Alternative | Advantages | Disadvantages |
|---|---|---|
| A. required `protocol_version:u16` in Header | explicit historical rule set; directly BlockId-bound; clear rejection of inactive versions; schema/protocol separation preserved | repeats a value constrained by chain configuration; future activation rules still need a separate decision |
| B. derive protocol rules from `schema_version` | fewer fields | conflates encoding and behavior; violates the approved separation rule |
| C. derive only from GenesisManifest | strong genesis binding | Header cannot state later behavioral version; historical verification depends on external lookup |
| D. place protocol version outside Header | smaller Header | protocol behavior is not BlockId-bound and may differ across validators |

### Recommendation

Select A. Require:

```text
protocol_version: u16 = 1
```

Use shortest canonical CBOR unsigned encoding. Structural validation rejects
zero and unknown values; historical/configuration validation rejects known but
inactive values. This review does not define activation or governance.

## 3. RoundNumber

RoundNumber belongs in Header identity because two attempts at the same height
and parent but different rounds are different consensus contexts. Omitting it
would collapse distinct signed attempts into one Body shape and weaken
equivocation/evidence interpretation.

`(height, round_number)` is unique only inside the same chain and validated
parent context; it is not a globally unique identifier by itself.

Recommended field:

```text
round_number: RoundNumber(u32)
```

- required;
- shortest canonical CBOR unsigned encoding;
- round zero valid;
- included in signing payload and BlockId;
- structural validation enforces `u32`;
- consensus validation interprets round under separately approved rules.

Keeping round only in another container would leave BlockId insensitive to
the consensus attempt that produced the Header.

### Required Narrow M1.1 Correction

Before any schema implementation:

1. introduce distinct `RoundNumber(u32)`;
2. retain `Slot(u64)` only as a legacy/non-consensus pure-data type and prohibit
   its use as Height or RoundNumber;
3. deprecate composite `Round { slot, number }` for consensus use;
4. direct all future consensus schemas to `(Height, RoundNumber)`;
5. preserve existing names long enough for repository migration, then consider
   removal only through a separately approved breaking primitive revision;
6. add no clock, timing, scheduler, increment-transition, or selection logic.

No code change is authorized by this review.

## 4. CandidateIndex Alternatives

| Alternative | Advantages | Disadvantages |
|---|---|---|
| A. include `candidate_index:u16` in Header | selection position is BlockId-bound | meaning is unapproved; otherwise identical Headers receive different IDs; couples Header V1 to one selection model |
| B. include only with selection evidence outside Header | keeps evidence close to its derivation | Header does not independently state selection index |
| C. omit until selection model approval | follows inactive/derivable-field rule; preserves Header identity across future election changes | later need may require another object or Header version |
| D. derive from deterministic selection proof | avoids duplicated value | proof format and derivation are unapproved |

### Recommendation

Select C for BlockHeaderV1. CandidateIndex has no active approved semantics and
may be redundant with producer, height, round, or future evidence. Do not make
leader-election changes alter BlockId accidentally.

## 5. Delay-Field Architecture

| Alternative | Header fields | Advantages | Disadvantages |
|---|---|---|---|
| A | engine ID, difficulty, output commitment | direct configuration visibility; stable fixed fields | duplicates evidence; algorithm-specific difficulty becomes Header law |
| B | only typed DelayEvidence commitment | complete evidence substitution changes BlockId; minimal fixed Header; engine agility | evidence must be available; requires unambiguous evidence domain |
| C | engine ID plus commitment; difficulty from configuration | engine visible and difficulty not duplicated | engine still duplicates evidence; configuration binding remains external |
| D | no delay fields; evidence elsewhere | smallest Header | BlockId does not bind the evidence used to justify the block |

### Recommendation

Select B:

```text
delay_evidence_commitment: DelayEvidenceHash(bytes32)
```

The commitment must cover the complete canonical DelayEvidenceV1. Header does
not duplicate engine ID or difficulty. Evidence availability and
configuration validation remain external dependencies.

This supports replaceable engines without changing Header field layout or
placing proof bytes in Header. A different evidence object necessarily changes
BlockId.

## 6. DelayEvidenceV1 and Domain Question

Candidate review form:

```text
DelayEvidenceV1 = [
    schema_version: u16 = 1,
    engine_id: DelayEngine,
    difficulty: u64,
    output: bytes,
    proof: bytes
]
```

Exact outer length is five. Engine-specific output/proof bounds and semantics
remain undecided.

Duplicating engine ID or difficulty in Header creates two sources requiring
equality. Hashing the complete evidence avoids substitution of any field.

The existing `DELAY_OUTPUT` registry description says “canonical delay
output/proof commitment,” while an earlier payload description says
“canonical delay output.” This is not precise enough to conclude that the
domain already means canonical complete DelayEvidenceV1.

### Recommendation

Do not silently reinterpret `DELAY_OUTPUT`. Prefer a separately approved
`DELAY_EVIDENCE` domain for:

```text
Q1HashV1(
    DELAY_EVIDENCE,
    CanonicalCBOR(DelayEvidenceV1)
)
```

An alternative is an explicit human-approved clarification proving that
`DELAY_OUTPUT` was intended for this exact five-field payload. Either path
requires a separate domain/profile update and vectors. This review does not
allocate or edit a domain.

## 7. DelayEngine NONE

| Alternative | Advantages | Disadvantages |
|---|---|---|
| A. full evidence `[1, NONE, 0, empty bytes, empty bytes]` | one canonical evidence object; same Header shape and commitment path | still requires profile authorization and evidence-domain decision |
| B. fixed typed Header constant | compact | special commitment path; evidence substitution model differs by engine |
| C. nullable fields | explicit absence | multiple Header states and future null transition |
| D. no delay fields under another Header version | no false evidence claim | splits Header versions solely by network profile |

### Recommendation

Select A:

```text
DelayEvidenceV1_NONE = [
    1,
    0,
    0,
    h'',
    h''
]
```

Both output and proof are definite zero-length CBOR byte strings. The Header
contains the normal typed commitment to this complete object. NONE is valid
only when the governing approved genesis/network profile permits it and is
rejected wherever delay is mandatory. No network obtains NONE permission from
this encoding alone.

## 8. SCHEMA-BLOCK-003 Exclusion Proposal

| Excluded value | Reason |
|---|---|
| timestamp and wall-clock time | operational observation; nondeterministic |
| fork-choice data | consensus behavior, not Header identity |
| candidate index and selection proof | inactive selection semantics; evidence belongs elsewhere |
| full delay proof | potentially large/engine-specific; committed evidence belongs outside Header |
| ReceiptRoot | inactive and explicitly omitted |
| computed fee totals | derived economic result |
| reward, issuance, and treasury values | inactive economic behavior |
| `economic_rules_version` | recommended derivation from required protocol version; independent registry not approved |
| generic `block_body_commitment` | redundant with typed TransactionRoot and explicitly omitted |
| BlockId self-field | self-derived and circular |
| other object IDs for packaging/finality | assigned to other objects, not Header identity |
| parent height | derived as Header height minus one under ParentReference rules |
| transaction count | derived from BlockBodyV1 array and TransactionRoot leaves |
| participant count | derived from ParticipantSetV1 array and ParticipantRoot leaves |

### Recommendation

Approve this exact exclusion set for V1. Do not reserve null or placeholder
positions for excluded values.

## 9. SCHEMA-BLOCK-006 Proposal

Recommended final status:

```text
RoundNumber field membership: REQUIRED
wire type: distinct u32, shortest canonical CBOR
round zero: valid
signing/BlockId inclusion: yes
M1.1 correction: required before implementation
implementation status: blocked until correction
```

BlockHeader documentation can be finalized before the narrow primitive
correction because the normative target type and encoding are known. All code,
vectors, and valid runtime instantiation remain blocked until the correction
and the existing StateRoot/domain blockers are closed.

## 10. Candidate Final Field Membership

This is a review candidate, not a normative schema:

| Field | Semantic type | Presence | Encoding/width | Structural validation | Historical/state validation | Sign | BlockId | Blocker |
|---|---|---|---|---|---|---|---|---|
| `schema_version` | schema version | required | `u16=1`, shortest CBOR | exact supported version | none | yes | yes | none |
| `protocol_version` | protocol version | required | `u16=1`, shortest CBOR | nonzero/known | active for chain/height | yes | yes | human decision |
| `chain_id` | ChainId | required | bytes32 | exact type/length | active chain match | yes | yes | pending domain registration already tracked |
| `height` | Height | required | `u64`, shortest CBOR | height >= 1 | chain position | yes | yes | none |
| `round_number` | RoundNumber | required | `u32`, shortest CBOR | bounded; zero valid | round validity | yes | yes | M1.1 correction |
| `parent_reference` | ParentReferenceV1 | required | exact three-field array | version/kind/bytes32 | genesis or immediate-parent relation | yes | yes | none |
| `producer_id` | ParticipantId | required | bytes32 | exact type/length | active producer in pre-H set | yes | yes | historical validation implementation |
| `transaction_root` | TransactionRoot | required | bytes32 | exact semantic type | matches BlockBodyV1 | yes | yes | implementation authorization |
| `participant_root` | ParticipantRoot | required | bytes32 | exact semantic type | matches pre-H ParticipantSetV1 | yes | yes | implementation authorization |
| `state_root` | StateRoot | required | bytes32 | typed shape only | complete state reconstruction | yes | yes | state model/profile inactive |
| `delay_evidence_commitment` | DelayEvidenceHash | required | bytes32 | exact semantic type | evidence/config/profile validation | yes | yes | domain, evidence schema, bounds, vectors |

Recommended exclusions mean there is no optional field in this candidate.

## 11. Canonical Ordering Alternatives

| Strategy | Assessment |
|---|---|
| A. identity/context, parent/producer, commitments, delay | clear conceptual flow; close to validation order |
| B. versions first, identifiers, coordinates, commitments | strongest audit grouping and consistent version-first schemas |
| C. fixed bytes before integers | performance-oriented and conceptually fragmented |
| D. preserve original proposed order | minimizes editorial movement but retains groupings from now-rejected fields |

### Recommendation

Use a merged B/A strategy: versions first, chain and consensus coordinates,
parent/producer identity, then typed commitments.

Candidate exact zero-based order:

| Index | Field |
|---:|---|
| 0 | `schema_version` |
| 1 | `protocol_version` |
| 2 | `chain_id` |
| 3 | `height` |
| 4 | `round_number` |
| 5 | `parent_reference` |
| 6 | `producer_id` |
| 7 | `transaction_root` |
| 8 | `participant_root` |
| 9 | `state_root` |
| 10 | `delay_evidence_commitment` |

Candidate final array length: **11**.

This order favors auditability and conceptual grouping rather than
serialization micro-optimization. Once human-approved, it must not be
reordered within V1.

## 12. Remaining Blockers

- human approval of SCHEMA-BLOCK-003 and SCHEMA-BLOCK-006;
- human approval of protocol_version, RoundNumber, CandidateIndex omission,
  delay architecture, exclusions, final membership/order, and length;
- narrow M1.1 RoundNumber primitive correction;
- exact DelayEvidenceV1 output/proof types and resource bounds;
- explicit delay-evidence domain decision and registry/profile update;
- exact NONE profile authorization in governing genesis/network configuration;
- complete StateRoot model, profile activation, and independent vectors;
- implementation authorization for already approved roots and owning objects;
- operational resource limits;
- schema and cross-language vectors after every blocker is resolved.

## 13. No-Decision and No-Implementation Confirmation

This brief records alternatives and recommendations only. It does not complete
any Human Approval field, finalize BlockHeaderBodyV1, approve
DelayEvidenceV1, allocate a domain, or authorize NONE for any network.

No source code, crate, struct, codec, parser, validator, delay algorithm,
proof, Merkle tree, vector, Header implementation, consensus behavior, ledger
logic, state logic, selection logic, networking, or runtime behavior was
created or changed.
