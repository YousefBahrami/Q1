Q1 Glossary

01_GLOSSARY.md

Project: Q1 Experimental Distributed Ledger
Glossary Version: 0.1.0
Status: Canonical Terminology Reference
Classification: Normative

⸻

1. Purpose

This glossary defines the official meaning of technical terms used throughout the Q1 specification.

Whenever a term defined here appears in another document, this glossary SHALL be considered the authoritative reference.

No implementation, presentation, article, or discussion may silently redefine a glossary term.

⸻

2. Terminology Rules

Unless explicitly stated otherwise:

* each term has one canonical meaning;
* capitalized terms refer to these definitions;
* undefined terms have no protocol authority;
* implementation-specific names do not override glossary definitions.

⸻

3. Core Concepts

Account

A protocol object that holds state such as balance, nonce, and metadata.

⸻

Address

A human-usable representation of a public-key derived identifier.

⸻

AI Observer

A non-consensus component that monitors the network, analyzes telemetry, detects anomalies, and produces reports without participating in consensus.

⸻

Archive Node

A node that retains complete historical ledger information.

⸻

Attestation

A signed statement by an authorized committee member regarding a proposed block.

⸻

Balance

The quantity of protocol units currently assigned to an account.

⸻

Amount

A semantically distinct unsigned 128-bit monetary value. Its canonical
protocol encoding is exactly one definite 16-byte CBOR byte string containing
the big-endian, left-zero-padded unsigned value. An Amount is not a Balance,
Fee, Reward, Supply, raw integer, or arbitrary byte string.

⸻

Block

An ordered collection of validated transactions together with protocol metadata.

⸻

Block Height

The sequential position of a block within the ledger.

⸻

Bootstrap Node

A node used only to help peers discover one another.

It has no special consensus authority.

⸻

Canonical Serialization

The unique byte representation of protocol data.

For Q1 V1 consensus objects, this means the restricted deterministic CBOR
profile selected by ADR-0002, not generic CBOR.

⸻

Cryptographic Domain

A unique, versioned protocol context that prevents bytes belonging to one
object or signature role from being interpreted as another cryptographic use.

⸻

Golden Vector

A language-neutral example that fixes the exact semantic fields, canonical
bytes, domain frame, hash, signing payload, signature, and expected rejection
behavior for one approved protocol-object schema. A candidate or illustrative
vector is not a Golden Vector until its schema is normatively approved.

⸻

Network Class

An append-only numeric classification used in ChainId derivation to distinguish
localnet, private-testnet, research, and reserved future-public network
categories. Network Class is not a ChainId, GenesisId, human network name, or
address HRP. The future-public value remains reserved and inactive.

⸻

Participant Identity Body

The canonical three-field V1 identity preimage containing schema version,
nullable producer public key, and nullable validator public key. At least one
role key is present and two present keys are distinct. It excludes activation,
node transport identity, roles, weight, and metadata.

⸻

Participant ID

A typed bytes32 identity derived under the approved `PARTICIPANT_ID` domain
from the canonical Participant Identity Body. It identifies role-key identity,
not activation scheduling or the complete participant record. Domain
`0x0013` is approved pending normative registration.

⸻

Participant Record

The six-field V1 registry record containing schema version, Participant ID,
nullable producer and validator keys, activation height, and nullable
deactivation height. Roles, activity, and V1 unit role weight are derived;
node transport identity is excluded.

⸻

Participant Record Hash

A typed hash under the `PARTICIPANT_RECORD` domain over the complete canonical
ParticipantRecordV1. It commits to activation scheduling and is not
interchangeable with Participant ID.

⸻

Block ID

A distinct semantic bytes32 identifier derived under `BLOCK_ID` from canonical
SignedBlockHeaderV1, including signature algorithm and producer signature. It
is never serialized inside either header form and is not interchangeable with
a generic hash.

⸻

Block Header Body

The approved eleven-field BlockHeaderBodyV1 canonical array: schema version,
protocol version, ChainId, Height, RoundNumber, ParentReferenceV1, producer
ParticipantId, TransactionRoot, ParticipantRoot, StateRoot, and
DelayEvidenceHash. Every field is required and the V1 order is permanent.
Schema approval does not authorize implementation or activation.

⸻

Parent Reference

The exact three-field ParentReferenceV1 record containing schema version,
ParentKind, and a bytes32 parent ID. At signed-block height 1 the kind is
GENESIS and the ID is the governing GenesisId. Above height 1 the kind is
BLOCK and the ID is the BlockId at the immediately preceding height. Null and
zero-sentinel parents are invalid.

⸻

Parent Kind

The append-only `u16` discriminator inside ParentReferenceV1. V1 assigns
`0x0001` to GENESIS and `0x0002` to BLOCK. It determines whether the wire
bytes32 is semantically GenesisId or BlockId.

⸻

Round Number

A distinct unsigned 32-bit semantic value for consensus context
`(Height, RoundNumber)`, canonically encoded as the shortest CBOR unsigned
integer. The active primitive is an independent `RoundNumber(u32)` with
`RoundNumber::ZERO`; it has no Slot field, conversion, or shared
representation. It begins at zero and does not authorize timing or
round-transition logic.

⸻

Delay Evidence Hash

A distinct semantic bytes32 commitment to the complete canonical
DelayEvidenceV1 under the future `DELAY_EVIDENCE` domain. It is not a
GenericHash and does not reuse `DELAY_OUTPUT`. Production remains impossible
until the new domain is normatively registered and conformance vectors exist.

⸻

Delay Evidence

The approved five-field base record containing schema version, DelayEngine,
difficulty, definite-length output bytes, and definite-length proof bytes.
Engine-specific structures, bounds, semantics, and verification remain
unapproved. Engine NONE is `[1, 0, 0, h'', h'']` only for a network profile
that explicitly permits it.

⸻

Transfer Body

The approved nine-field V1 canonical body for ordinary value transfer:
schema version, ChainId, sender public key, recipient binary AddressEnvelope,
Amount, FeeLimit, Nonce, and inclusive validity-window heights. It contains no
transaction-type discriminant, sender address, memo, signature, or self ID.

⸻

Signed Transfer

The three-field V1 envelope containing TransferBodyV1, signature algorithm,
and signature. The signature covers the domain-framed canonical body and is
included in TransferId.

⸻

Transfer ID

The typed result of hashing canonical SignedTransferV1 under the existing
`TRANSACTION_ID` domain. It includes the signature and is not serialized
inside the signed transfer.

⸻

Merkle Sequence Profile

The approved shared binary indexed-sequence commitment construction using the
existing `MERKLE_LEAF` and `MERKLE_INTERNAL` domains plus an append-only
two-byte tree profile ID. Leaves commit to profile, zero-based index, canonical
item length, and exact canonical item bytes. The Merkle layer never sorts or
normalizes its input.

⸻

Odd-Node Promotion

The approved rule for an unpaired hash at a non-root Merkle level: carry it
unchanged to the next level. It is never duplicated. A one-item tree therefore
has its leaf hash as its root.

⸻

Typed Empty Root

The zero-item commitment derived under `MERKLE_LEAF` from exactly
`tree_profile_id:u16be || item_count:u64be`, where item count is zero. It is
not an indexed leaf and remains semantically distinct for each profile.

⸻

Transaction Root

The distinct semantic root type for the exact block-body sequence of canonical
SignedTransferV1 values. Profile ID is `0x0001`. Its construction is approved
for future implementation only after explicit authorization. It is a required
eventual BlockHeaderBodyV1 member; its count derives from the canonical
BlockBodyV1 transfer-array length.

⸻

Participant Root

The distinct semantic root type for canonical ParticipantRecordV1 values
already ordered by ascending raw ParticipantId. Profile ID is `0x0003`.
Duplicate or unsorted ParticipantIds are rejected before Merkle construction.
It is a required eventual Header member and represents the active set
governing height H from finalized pre-H state. Implementation still requires
explicit authorization.

⸻

Block Body

The exact two-field BlockBodyV1 canonical object containing schema version and
the ordered SignedTransferV1 array. It contains no receipt, evidence, metadata,
reserved section, ID, generic hash, or generic body commitment.

⸻

Transaction Count

A distinct semantic `u32` derived from both the BlockBodyV1 transfer-array
length and TransactionRoot leaf count. It is not serialized as a Header or
BlockBody field. Zero is valid.

⸻

Participant Set

The exact three-field ParticipantSetV1 canonical object containing schema
version, reference Height, and a non-empty sorted ParticipantRecordV1 array.
For reference height H it is derived from finalized state before executing H
and governs validation of signed block H.

⸻

Participant Count

A distinct semantic `u32` derived from both the ParticipantSetV1 record-array
length and ParticipantRoot leaf count. It is not serialized as a Header or
ParticipantSet field. Zero is invalid.

⸻

Receipt Root

A distinct bytes32 semantic root type whose profile remains reserved inactive.
It is omitted from BlockHeaderBodyV1 and has no active constructor or validity
claim.

⸻

State Root

A distinct bytes32 semantic type required in the eventual
BlockHeaderBodyV1. Only field shape is approved: construction profile
`0x0005` remains inactive, and valid Header instantiation is blocked until the
complete state model is approved.

⸻

Protocol Object Schema

The normative definition of one versioned protocol object, including every
field, fixed position, type and width, presence rule, collection ordering,
resource limit, canonical encoding, signing form, complete form, and
cryptographic domain. A conceptual field list is not a Protocol Object Schema.

⸻

Domain Separation

The mandatory construction that binds a cryptographic operation to one
registered Cryptographic Domain.

⸻

Certificate

A cryptographically verifiable collection of signatures proving that required protocol conditions have been satisfied.

⸻

Challenge

The protocol-defined input used to produce a Delay Proof.

⸻

Committee

The deterministic subset of participants responsible for validating a proposed block during one consensus round.

⸻

Consensus

The protocol process through which honest nodes agree on the next valid ledger state.

⸻

Delay Engine

The subsystem responsible for producing and verifying sequential delay proofs.

⸻

Delay Proof

Evidence that the required sequential computation has been completed.

⸻

Determinism

The property that identical inputs always produce identical outputs.

⸻

Deterministic CBOR Profile

The strict Q1 subset of RFC 8949 that defines accepted types, unique encodings,
schema behavior, rejection rules, and decoder resource limits for
consensus-critical objects.

⸻

Equivocation

The act of producing multiple conflicting protocol messages that should have been unique.

⸻

Explorer

A read-only service that exposes ledger information to users.

⸻

Fee

The protocol-defined cost paid to submit a transaction.

⸻

Finality

The protocol condition under which a block is considered irreversible according to the current protocol rules.

⸻

Finalization Certificate

The collection of protocol evidence proving finality.

⸻

Full Node

A node that independently verifies protocol rules.

⸻

Genesis

The initial protocol state from which every node begins.

⸻

Governance

The documented process by which protocol rules may change in the future.

Governance is distinct from consensus.

⸻

HDD Module

An experimental subsystem that measures or evaluates HDD-related research without automatically becoming part of consensus.

⸻

Honest Node

A node following the protocol specification correctly.

⸻

Ledger

The ordered and verifiable record of protocol state.

⸻

Localnet

A private development network operating entirely under local engineering control.

⸻

Bech32m Envelope

The Q1-defined human-readable address container using the Bech32m alphabet and
checksum. It does not inherit Bitcoin witness-address semantics.

⸻

Mempool

The temporary collection of valid transactions awaiting inclusion in a block.

⸻

Milestone

A formally defined engineering checkpoint in the project roadmap.

⸻

Node

A software instance participating in the Q1 network.

⸻

Nonce

A monotonic account value preventing transaction replay.

⸻

Observer

A component that monitors protocol behavior without changing protocol state.

⸻

Peer

A node connected through the peer-to-peer network.

⸻

Private Testnet

A distributed experimental network with controlled participation.

⸻

Producer

The participant selected to construct the next candidate block.

⸻

Proposal

A candidate block presented to the committee.

⸻

Protocol

The complete set of rules defining valid Q1 behavior.

⸻

Q1

The name of the experimental distributed-ledger project described by this specification set.

⸻

QIP

Q1 Improvement Proposal.

The formal mechanism used to propose governance changes.

⸻

Quorum

The minimum required committee participation necessary for protocol progress.

⸻

Receipt

Protocol evidence describing the outcome of transaction execution.

⸻

Reward

Protocol-defined compensation distributed according to deterministic rules.

⸻

Roadmap

The evidence-based sequence of project milestones and decision gates.

⸻

Round

The historical composite `Round { slot, number }` primitive is deprecated,
legacy, and compatibility-only. It is not the approved active consensus round
primitive; new primitive code uses RoundNumber. Its historical codec remains
temporarily unchanged, and removal requires a later explicit Gate.

⸻

Slot

The historical `Slot(u64)` primitive is a deprecated legacy non-consensus data
type. It has no approved scheduling, timing, leader-selection, or runtime role
and must not substitute for Height or RoundNumber. Its historical codec remains
temporarily unchanged, and removal requires a later explicit Gate.

⸻

Safe Mode

The protocol state entered when critical invariants can no longer be trusted.

⸻

Sequential Computation

A computation intentionally designed to resist efficient parallel execution.

⸻

Signature

A cryptographic proof that a protocol message originated from the holder of the corresponding private key.

⸻

Simulator

An isolated environment used to evaluate protocol behavior without affecting live ledger state.

⸻

Snapshot

A compact representation of ledger state at a specific finalized height.

⸻

State Root

The typed eventual commitment to complete protocol state. In V1 only its
required bytes32 field shape is approved; construction and activation remain
blocked by the state-model decision.

⸻

Supply

The total quantity of protocol units currently existing according to protocol rules.

⸻

Sybil Attack

An attempt to gain disproportionate influence by creating multiple identities.

⸻

Telemetry

Operational measurements produced by nodes.

Telemetry is not consensus evidence.

⸻

Test Unit

A non-production protocol unit used exclusively for experimentation.

It has no implied financial value.

⸻

Transaction

A signed protocol instruction requesting a state transition.

⸻

Transaction Root

The typed profile-`0x0001` commitment to the exact ordered canonical
SignedTransferV1 sequence in BlockBodyV1.

⸻

Validator

A committee participant responsible for verifying candidate blocks.

⸻

Verifiable Delay Function (VDF)

A cryptographic construction that requires sequential computation while allowing efficient verification.

⸻

Wallet

Software used to generate, store, protect, and use cryptographic keys.

⸻

4. Reserved Terms

The following names are reserved for protocol use:

Account
Address
AI Observer
Archive Node
Attestation
Balance
Block
Certificate
Committee
Consensus
Delay Engine
Delay Proof
Explorer
Finality
Genesis
Governance
Ledger
Mempool
Node
Producer
Protocol
Q1
QIP
Reward
Safe Mode
Snapshot
State Root
Telemetry
Transaction
Validator
Wallet

⸻

5. Future Additions

New terminology SHALL:

* include one canonical definition;
* identify related documents;
* avoid ambiguity with existing terms;
* preserve backward compatibility whenever practical.

⸻

6. Final Principle

A distributed system cannot remain deterministic if its language is ambiguous.

Therefore, before nodes agree on blocks, developers must agree on words.

This glossary exists so that every engineer, reviewer, researcher, auditor, and future implementation speaks the same technical language before writing a single line of consensus-critical code.
