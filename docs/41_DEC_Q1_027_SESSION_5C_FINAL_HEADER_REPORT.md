# DEC-Q1-027 Session 5C Final Header Report

Status: Human decision recorded; implementation and activation blocked

Decision owner: Yousef Bahrami

Review date: 2026-07-25

## Scope and Gate

Session 5C reviewed only the final BlockHeaderV1 fields and exclusions,
protocol and round representation, and the DelayEvidence commitment boundary.
The gate outcome is:

- Schema: **APPROVED**
- Implementation: **BLOCKED**
- Runtime activation: **BLOCKED**

No source code, crate, struct, parser, codec, validator, delay algorithm,
Merkle/state implementation, vector, test, or runtime behavior was created.

## Reviewed Decisions

The human review approved:

- required `protocol_version: u16`, with V1 value `1`, shortest canonical CBOR,
  and rejection of zero, unknown, and known-inactive versions;
- required `round_number: RoundNumber(u32)`, with zero valid and consensus
  context `(Height, RoundNumber)`;
- SCHEMA-BLOCK-003's exact exclusion set;
- SCHEMA-BLOCK-006 with a mandatory pre-implementation M1.1 correction;
- complete omission of CandidateIndex;
- one required typed DelayEvidenceHash Header commitment;
- the five-field DelayEvidenceV1 base shape;
- a future separate DELAY_EVIDENCE domain, pending normative registration;
- the conditional canonical NONE evidence representation.

No activation, governance, scheduler, timeout, selection, delay verification,
state transition, economic, or execution behavior was approved.

## Final BlockHeaderBodyV1

The canonical form is an exact array of length 11:

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
```

All fields are required. V1 has no optional, unknown, or trailing fields. The
zero-based order above is permanent.

## Signed Envelope and Identity

The previously approved forms remain unchanged:

```text
SignedBlockHeaderV1 = [
    body: BlockHeaderBodyV1,
    signature_algorithm: SignatureAlgorithm,
    producer_signature: Ed25519Signature
]

BlockHeaderSigningPayload =
    Q1DomainFrameV1(
        BLOCK_HEADER_SIGNING,
        CanonicalCBOR(BlockHeaderBodyV1)
    )

BlockId =
    Q1HashV1(
        BLOCK_ID,
        CanonicalCBOR(SignedBlockHeaderV1)
    )
```

The signature is outside its own signing payload and inside BlockId. No
self-derived BlockId is serialized.

## Exclusions

Header V1 contains no timestamp or wall-clock field, fork-choice data,
CandidateIndex or selection proof, full delay proof, ReceiptRoot, computed fee
total, reward, issuance, treasury value, economic_rules_version, generic
block_body_commitment, self BlockId, ProposalId, FinalizationCertificateId,
parent_height, transaction_count, or participant_count. None receives a null,
sentinel, reserved position, or placeholder.

These exclusions remove derivable or redundant values, operational metadata,
unapproved economics, objects owned elsewhere, and consensus behavior that
must not become Header identity accidentally.

## DelayEvidenceV1

The approved base canonical object has exact outer length 5:

```text
DelayEvidenceV1 = [
    schema_version: u16 = 1,
    engine_id: DelayEngine,
    difficulty: u64,
    output: bytes,
    proof: bytes
]
```

Output and proof are definite-length byte strings. Missing, unknown, or
trailing fields are rejected. Engine-specific byte structures, limits,
semantics, and verification remain unapproved.

The Header uses the distinct semantic type:

```text
DelayEvidenceHash =
    Q1HashV1(
        DELAY_EVIDENCE,
        CanonicalCBOR(DelayEvidenceV1)
    )
```

The existing DELAY_OUTPUT domain is not reinterpreted. DELAY_EVIDENCE has no
numeric assignment from Session 5C and requires a separate normative registry
decision. Until registration and vectors, no valid DelayEvidenceHash may be
produced.

The exact NONE evidence is:

```text
[1, 0, 0, h'', h'']
```

Engine ID and difficulty are zero; output and proof are definite empty byte
strings. It follows the normal hash path and is valid only where an approved
Genesis/network profile explicitly permits NONE.

## Implementation and Activation Blockers

At minimum, implementation and valid instantiation require:

1. a separately authorized M1.1 correction introducing RoundNumber, limiting
   Slot to legacy/non-consensus use or deprecating it, and deprecating the
   composite Round for consensus;
2. normative registration of the approved ChainId and ParticipantId domains;
3. normative assignment and registration of DELAY_EVIDENCE;
4. exact engine-specific DelayEvidence profiles and resource bounds;
5. applicable network-profile authorization for NONE;
6. the complete state-model decision and StateRoot activation;
7. independent cross-language golden and rejection vectors;
8. explicit implementation authorization.

Schema approval is not evidence that any blocker is satisfied.

## Changed Documentation

Session 5C updated the decision package, README, project status, glossary,
architecture, ledger, consensus, delay-engine boundary, test plan,
traceability, open decisions, and changelog, and created this report. The
normative domain registry was intentionally not changed.

## Recommendation for Session 6

Do not begin Session 6 automatically. Human review should first confirm the
next bounded decision topic. If the next work depends on BlockHeader identity,
the safest prerequisite sequence is the narrow M1.1 RoundNumber correction,
the three pending domain-registration decisions, and the state/delay profile
gates. This is a decision-order recommendation, not implementation authority.
