# Q1 Cryptographic Domain Registry V1

Version: 0.1.2

Registry identifier: `q1-domain-registry-v1`

Status: Normative — Approved 2026-07-24

Authority: ADR-0003

## 1. Rule

SHA-256 is the Q1 V1 consensus hash. No raw consensus object may be hashed.
Every hash and signed message SHALL use one registered, versioned domain.
Changing a domain identifier or framing rule is a protocol change.

## 2. Candidate Domain Frame

For payload byte string `P`, define:

```text
Q1DomainFrameV1(domain_id, P) =
    0x51 0x31 0x44 0x53       # ASCII "Q1DS"
 || 0x00 0x01                 # registry version, uint16 big-endian
 || domain_id                 # uint16 big-endian
 || len(P)                    # uint64 big-endian byte length
 || P
```

`len(P)` SHALL be encoded exactly as an unsigned 64-bit big-endian integer.
Concatenation SHALL occur only after the payload has been unambiguously
constructed. Consensus object payloads SHALL be their canonical
`q1-dcbor-v1` bytes unless a registry row explicitly defines another payload.

```text
Q1HashV1(domain_id, P) = SHA-256(Q1DomainFrameV1(domain_id, P))
```

Signatures use the same frame as specified by the Ed25519 profile, but sign
the frame bytes directly; they do not silently replace the message with its
SHA-256 digest.

Chain/network identity SHALL be a required field in every replay-sensitive
canonical payload. It is not duplicated in this domain header. An address HRP
binds its network separately. This avoids per-chain domain allocation and the
risk that duplicate header/payload chain values disagree.

Alternatives considered:

- variable human-readable labels are easier to inspect but introduce length,
  case, normalization, and registration questions;
- an unversioned numeric ID is compact but weakly bound to Q1 and one framing
  grammar;
- the recommended fixed `Q1DS` prefix, version, and fixed-width ID is compact,
  self-delimiting, and append-only.

## 3. Registry

Domain IDs are never reused.

| ID | Symbol | Purpose and payload |
|---:|---|---|
| `0x0001` | `TRANSACTION_SIGNING` | Canonical unsigned/signing transaction form |
| `0x0002` | `TRANSACTION_ID` | Canonical complete transaction form |
| `0x0003` | `BLOCK_HEADER_SIGNING` | Canonical unsigned block header |
| `0x0004` | `BLOCK_ID` | Canonical complete signed block header |
| `0x0005` | `PROPOSAL_SIGNING` | Canonical unsigned block proposal |
| `0x0006` | `ATTESTATION_SIGNING` | Canonical unsigned attestation |
| `0x0007` | `FINALIZATION_CERTIFICATE` | Canonical finalization certificate |
| `0x0008` | `PARTICIPANT_RECORD` | Canonical participant registry record |
| `0x0009` | `MERKLE_LEAF` | Tree/profile ID followed by the canonical leaf bytes |
| `0x000a` | `MERKLE_INTERNAL` | Tree/profile ID followed by left and right child digests |
| `0x000b` | `DELAY_CHALLENGE` | Canonical delay-challenge input |
| `0x000c` | `DELAY_OUTPUT` | Canonical delay output/proof commitment |
| `0x000d` | `GENESIS` | Canonical genesis object |
| `0x000e` | `SNAPSHOT` | Canonical snapshot manifest |
| `0x000f` | `ADDRESS_PAYLOAD` | Algorithm-specific public account material used by the address profile |
| `0x0010` | `CHAIN_ID` | Canonical three-field ChainIdentityPreimageV1 approved by SCHEMA-COMMON-001 |
| `0x0011` | `LOCALNET_PROPOSAL_ID` | Complete signed LOCALNET_V0 proposal; no public-schema activation |
| `0x0013` | `PARTICIPANT_ID` | Canonical three-field ParticipantIdentityBodyV1 approved by SCHEMA-PART-002 |
| `0x0014` | `DELAY_EVIDENCE` | Complete canonical DelayEvidenceV1; NONE activated only for LOCALNET v0 |
| `0x0015` | `LOCALNET_STATE` | Complete canonical LOCALNET_V0 state snapshot; no partial tree proofs |
| `0xfffe` | `CONFORMANCE_TEST` | Non-consensus profile-vector research only |

The exact eight-byte prefix before the payload length for each protocol entry
is:

| Domain | Exact prefix hex |
|---|---|
| `TRANSACTION_SIGNING` | `5131445300010001` |
| `TRANSACTION_ID` | `5131445300010002` |
| `BLOCK_HEADER_SIGNING` | `5131445300010003` |
| `BLOCK_ID` | `5131445300010004` |
| `PROPOSAL_SIGNING` | `5131445300010005` |
| `ATTESTATION_SIGNING` | `5131445300010006` |
| `FINALIZATION_CERTIFICATE` | `5131445300010007` |
| `PARTICIPANT_RECORD` | `5131445300010008` |
| `MERKLE_LEAF` | `5131445300010009` |
| `MERKLE_INTERNAL` | `513144530001000a` |
| `DELAY_CHALLENGE` | `513144530001000b` |
| `DELAY_OUTPUT` | `513144530001000c` |
| `GENESIS` | `513144530001000d` |
| `SNAPSHOT` | `513144530001000e` |
| `ADDRESS_PAYLOAD` | `513144530001000f` |
| `CHAIN_ID` | `5131445300010010` |
| `LOCALNET_PROPOSAL_ID` | `5131445300010011` |
| `PARTICIPANT_ID` | `5131445300010013` |
| `DELAY_EVIDENCE` | `5131445300010014` |
| `LOCALNET_STATE` | `5131445300010015` |

Every registry row outputs either a 32-byte SHA-256 digest through `Q1HashV1`
or, for a signing row, the exact frame bytes supplied to pure Ed25519.

## 4. Tree Payloads

Merkle construction is not selected by this registry. A future tree profile
must allocate a stable tree/profile identifier and define unambiguous payloads.
At minimum:

```text
leaf payload     = tree_profile_id:u16be || leaf_bytes
internal payload = tree_profile_id:u16be || left_digest:32 || right_digest:32
```

No implementation may infer a Merkle tree algorithm from these two domain
rows.

## 5. Algorithm and Output Representation

- SHA-256 output is exactly 32 bytes.
- Hex is documentation/debug representation only and SHALL be lowercase.
- Stored or transmitted digests use raw 32-byte strings in canonical objects.
- The registry version and, where ambiguity is possible, algorithm/profile
  identifiers SHALL travel in the containing versioned object or genesis
  configuration.
- Domain framing is not a MAC and SHALL NOT be used as one.

## 6. Framing Vectors

For domain `TRANSACTION_ID` (`0x0002`) and empty payload:

```text
frame =
51314453000100020000000000000000

SHA-256(frame) =
0b368d1bc8790bd7aa6bbf27d608e25d
a78de7758440a70935433b08cdd6e7cf
```

For payload bytes `00ff`, the frame is:

```text
5131445300010002000000000000000200ff
```

Its SHA-256 output is:

```text
381c8a7503b38197292f36a2a3ecc18e
7f0155fba733cc71cbcdc0bd94fc461c
```

Both vectors were reproduced with Node.js `crypto` and Python `hashlib` on
2026-07-24. Cross-language reproduction in the selected Rust adapter remains
required before M1 implementation.

## 7. Rejection and Review Rules

Reject or fail closed on:

- unknown registry version or domain ID;
- wrong fixed-width fields;
- payload length mismatch or trailing bytes;
- a domain inconsistent with the object/signature role;
- raw SHA-256 of a consensus object;
- string-concatenated or delimiter-based substitutes for this binary frame.

Code review and tests SHALL make the domain ID visible at every
consensus-critical hash/sign call site.

Collision/ambiguity tests SHALL prove that changing the magic, version, domain
ID, payload length, payload byte, or payload order changes the frame and
SHA-256 output; truncation, length mismatch, unknown domains, and raw
unframed hashes are rejected.

## 8. Registration Procedure

- `0x0001..0x7fff` may be assigned only through explicit human approval and a
  registry/profile update with vectors.
- `0x8000..0xfffd` is reserved and rejected in V1.
- `0xfffe` is permanently non-consensus conformance research.
- `0xffff` is invalid.
- Assignments are append-only and never reused, including after deprecation.
- A new framing grammar uses a new registry version; it does not reinterpret
  V1 entries.

## 9. Approval Record

Decision rows: PREM1-DOM-001 through PREM1-DOM-010 in
`docs/28_PRE_M1_NORMATIVE_PARAMETER_DECISION_PACKAGE.md`.

All rows were approved by Yousef Bahrami on 2026-07-24. Merkle algorithms and
future object schemas remain separate decisions.

The append-only registrations `CHAIN_ID=0x0010` and
`PARTICIPANT_ID=0x0013` implement the assignments approved in DEC-Q1-027
Sessions 1 and 2, under the 2026-10-01 human instruction to implement
already-approved independent schemas. The framing registry version remains
one. These entries authorize identity derivation only; no public network,
genesis schema, or participant admission policy is implied. Fixed identity
vectors are verified by `crates/q1-protocol-types/tests/identity.rs` and the
M1.2 reference-vector tooling.

DELAY_EVIDENCE=0x0014 was approved on 2026-10-01, conditional on absence of
a collision. The complete registry and repository assignments were checked
before registration: 0x0014 was unassigned (at that time 0x0011/0x0012 were proposals,
0x0013 is ParticipantId, and 0xfffe is research-only). No existing assignment
changed. NONE activation is restricted to the explicit LOCALNET v0 profile;
registration does not activate NONE on private-testnet, research, or production.

On 2026-10-02 the explicit human authorization for minimal LOCALNET_V0
compound schemas and full state commitments activated LOCALNET_PROPOSAL_ID
(0x0011) and LOCALNET_STATE (0x0015). The complete registry and repository
assignments were checked: 0x0011 appeared only as an unapproved ProposalId
candidate in docs/32; 0x0015 appeared only in the local state proposal. Neither
was assigned to another domain. This registers the local payloads above;
it does not approve a future generic proposal/state schema. IDs are never reused.
0x0012 remains unregistered. Frozen local vectors cover both framed hashes;
the whole-u16 registry scan rejects aliases and tests 20 consensus assignments.
