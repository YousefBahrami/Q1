Q1 Consensus Specification

05_CONSENSUS.md

Project: Q1 Experimental Distributed Ledger
Protocol Version: 0.1
Document Version: 0.1.0
Status: Draft for Engineering Review
Classification: Experimental — Not for Production or Financial Use

⸻

1. Purpose

This document defines the consensus model for Q1 v0.1.

It specifies:

* how consensus participants are identified;
* how block-production rounds begin;
* how producer candidates are selected;
* how validator committees are selected;
* how delay challenges are generated;
* how candidate blocks are proposed;
* how validators attest to blocks;
* how quorum is calculated;
* how blocks become finalized;
* how producer fallback works;
* how conflicting proposals are handled;
* how network partitions affect safety and liveness;
* how equivocation is detected;
* when nodes enter safe mode;
* and how consensus behavior is tested.

The objective of Q1 v0.1 is not to claim a final production-grade consensus mechanism.

The objective is to implement a deterministic, measurable, modular consensus laboratory capable of revealing whether the proposed combination of:

* limited producer selection;
* sequential delay;
* temporary validator committees;
* supermajority finalization;
* multi-role participation;
* and optional HDD contribution

can produce a secure and understandable distributed ledger.

⸻

2. Consensus Philosophy

Q1 consensus is based on five principles.

2.1 Work must be limited

The network SHOULD avoid forcing every participant to perform expensive work for every block.

A limited number of producer candidates SHALL be selected for each round.

⸻

2.2 Delay must be verifiable

A producer MUST complete the configured delay process before proposing a block.

The delay result MUST be independently verifiable.

⸻

2.3 Production and validation are distinct

The block producer proposes a block.

The validator committee independently verifies and attests to it.

A producer MUST NOT finalize its own proposal merely by producing it.

⸻

2.4 Safety has priority over liveness

If Q1 cannot safely determine one valid finalized block, it SHOULD pause finalization rather than accept conflicting finalized states.

⸻

2.5 No single authority defines truth

Block validity MUST derive from deterministic protocol rules and signed distributed attestations.

AI, HDD telemetry, operators, explorers, and central servers MUST NOT independently determine finality.

⸻

3. Consensus Model Summary

Q1 v0.1 SHALL initially use a committee-based, round-driven consensus model.

Each block height proceeds through one or more rounds.

For each round:

1. the network derives a common round seed;
2. an ordered list of producer candidates is selected;
3. a validator committee is selected;
4. the first eligible producer performs the delay process;
5. the producer constructs and broadcasts a candidate block;
6. committee members independently verify the block;
7. valid committee members sign attestations;
8. a block receiving the required threshold obtains a finalization certificate;
9. the finalized block becomes the unique canonical block for that height;
10. if the producer fails, a fallback producer becomes eligible;
11. if safety is threatened, nodes stop finalizing and enter safe mode.

⸻

4. Consensus Roles

4.1 Consensus participant

A consensus participant is a registered or otherwise protocol-recognized node eligible for one or more consensus roles.

A participant MUST possess:

* a node identity;
* an active protocol version;
* valid consensus-role keys;
* current eligibility status;
* sufficient synchronization;
* no active disqualification under protocol rules.

⸻

4.2 Producer candidate

A producer candidate is a participant selected for an ordered fallback position in a specific block height and round.

A candidate MAY become the active producer only during its assigned production window.

⸻

4.3 Active producer

The active producer is the currently authorized candidate permitted to publish the primary proposal for a production window.

⸻

4.4 Validator

A validator is a selected committee member authorized to verify and attest to a candidate block for a particular height and round.

⸻

4.5 Observer

An observer follows consensus, verifies messages, and records evidence but does not necessarily sign proposals or attestations.

⸻

4.6 Full node

A full node independently verifies consensus objects and maintains finalized state.

A full node MAY be a producer, validator, observer, or combination of roles.

⸻

5. Consensus Objects

Q1 consensus SHALL define the following canonical objects:

ConsensusContext
RoundContext
ProducerCandidateList
ProducerEligibilityProof
CommitteeDefinition
CommitteeMembershipProof
DelayChallenge
DelayOutput
DelayProof
BlockProposal
ValidatorAttestation
EquivocationEvidence
FallbackCertificate
FinalizationCertificate
SafeModeNotice

Every consensus object MUST be:

* canonically serialized;
* versioned;
* bound to one chain;
* bound to one block height;
* bound to one round;
* independently verifiable;
* signed where required.

⸻

6. Consensus Context

For each block height, nodes SHALL construct a deterministic consensus context.

ConsensusContext {
    protocol_version
    chain_id
    block_height
    parent_block_hash
    parent_finalization_certificate_hash
    epoch_id
    participant_set_root
    consensus_parameters_hash
}

All honest nodes using the same finalized parent and participant set MUST derive the same consensus context.

⸻

7. Block Height and Round

Q1-CON-001 — Height

block_height identifies the next ledger position after the current finalized block.

GenesisManifest is an independent protocol object, not a signed block header.
The first SignedBlockHeaderV1 has height 1; height 0 is invalid for a signed
block header. At height 1, ParentReferenceV1 must have kind GENESIS and contain
the governing GenesisId. Above height 1 it must have kind BLOCK and contain
the BlockId at height minus one.

Any `parent_block_hash` name retained in conceptual consensus pseudocode is
shorthand for the validated ParentReferenceV1 identity and is not the
canonical wire field. Null, zero-sentinel, wrong-chain genesis, and
non-immediate block parents are invalid.

The approved V1 wire Header is the exact eleven-field BlockHeaderBodyV1
recorded in DEC-Q1-027 Session 5C. Its round field is
`RoundNumber(u32)` and the consensus context is `(Height, RoundNumber)`;
Slot and CandidateIndex are not Header substitutes. The Header carries only
the typed DelayEvidenceHash commitment, not engine ID, difficulty, or proof.
These schema decisions define no scheduler, transition, selection, delay, or
activation behavior.

If the latest finalized block has height H, the next consensus process targets:

H + 1

⸻

Q1-CON-002 — Round

Each block height begins at:

round = 0

If the network fails to finalize a block during that round, it MAY advance to:

round = 1
round = 2
...

A higher round does not change the target block height.

⸻

Q1-CON-003 — Round uniqueness

A round is uniquely identified by:

RoundID =
    chain_id
    || block_height
    || round_number
    || parent_block_hash

⸻

8. Epochs

Q1 MAY group multiple block heights into epochs.

An epoch may be used to update:

* eligible participant sets;
* reputation inputs;
* reward accounting;
* committee-selection data;
* protocol parameters;
* experimental diversity metrics.

Recommended initial experimental epoch length:

100 blocks

The exact value SHALL be configurable in genesis.

Consensus participants MUST use the participant set finalized for the relevant epoch.

⸻

9. Participant Registry

Q1 v0.1 MUST define a deterministic participant registry.

For the Q1 v0.1 private testnet, the registry model is:

PERMISSIONED_PRIVATE_TESTNET_REGISTRY

It MUST be defined by genesis and MAY contain producer and validator public
role keys plus activation and optional deactivation heights. Roles, activity,
and equal V1 role weight are derived rather than serialized.

This model is temporary and private-testnet only. It MUST NOT be represented as
permissionless, as public Sybil resistance, or as a decision for future public
participant admission.

The approved ParticipantRecordV1 wire schema is:

ParticipantRecord {
    schema_version
    participant_id
    producer_public_key_or_null
    validator_public_key_or_null
    activation_height
    deactivation_height_optional
}

At least one role key MUST be present; two present role keys MUST be distinct.
Producer and validator roles are derived from their respective key presence.
Activity at height H is derived as `activation_height <= H` and either no
deactivation height exists or `H < deactivation_height`. Each active V1 role
has protocol-defined weight 1. Node transport identity, roles, eligibility,
weight, participation score, cooldown, and penalty state are not serialized
in ParticipantRecordV1.

The precise admission model remains experimental. This schema decision does
not approve participant selection or admission behavior.

ParticipantSetV1 at reference height H is the active participant set governing
validation of signed block H, derived from finalized state before executing H.
Its records are non-empty, sorted by ascending raw ParticipantId, and committed
by the Header's ParticipantRoot.

Structural parsing checks canonical typed forms only. Historical/state
validation must reconstruct the pre-H set, require matching reference height
and ParticipantRoot, resolve producer_id to an active PRODUCER record at H,
resolve the producer role public key, and verify the SignedBlockHeaderV1
producer signature. This invariant does not define selection, committee,
quorum, or finality behavior.

For early private testnets, the participant registry MAY be defined in genesis or protocol configuration.

Public permissionless admission is not required for the first milestone.

⸻

10. Consensus Randomness

Producer and committee selection require a common unpredictable seed.

Q1-CON-004 — Round seed

The round seed SHOULD be derived from finalized data unavailable before the previous block was finalized.

Conceptually:

RoundSeed =
    HASH(
        DOMAIN_Q1_ROUND_SEED
        || chain_id
        || block_height
        || round_number
        || parent_block_hash
        || parent_finalization_certificate_hash
        || previous_delay_output
    )

⸻

Q1-CON-005 — No producer-controlled seed

A candidate producer MUST NOT be able to freely choose the seed.

⸻

Q1-CON-006 — Deterministic derivation

All honest nodes MUST derive the same seed from the same consensus context.

⸻

Q1-CON-007 — Bias analysis

The implementation MUST record whether any field contributing to randomness can be manipulated by:

* the previous producer;
* validators;
* committee aggregators;
* network timing;
* transaction selection;
* HDD output.

Any manipulable input MUST be documented.

⸻

11. Producer Selection

Q1-CON-008 — Ordered candidate list

For every round, the protocol MUST select an ordered list of producer candidates.

Recommended initial candidate count:

3

Example:

Candidate 0 → primary producer
Candidate 1 → first fallback
Candidate 2 → second fallback

⸻

Q1-CON-009 — Deterministic selection

Producer selection MUST be reproducible by every honest full node.

⸻

Q1-CON-010 — Selection interface

The selection algorithm MUST implement:

select_candidates(
    round_seed,
    participant_registry,
    consensus_parameters
) -> OrderedCandidateList

⸻

Q1-CON-011 — Eligibility filter

A participant MUST be excluded if:

* inactive;
* unsynchronized beyond allowed tolerance;
* using an unsupported protocol version;
* under active penalty;
* lacking a valid producer key;
* lacking required role capability;
* inside a mandatory cooldown;
* otherwise invalid under finalized protocol state.

⸻

Q1-CON-012 — Initial weighted lottery

The initial implementation MAY use a deterministic weighted lottery.

Conceptual participant weight:

Weight_i =
    BaseEligibility_i
    × AvailabilityFactor_i
    × ParticipationFactor_i
    × CooldownFactor_i
    × AntiConcentrationFactor_i
    × ExperimentalContributionFactor_i

The exact formula SHALL be defined in later simulation work.

⸻

Q1-CON-013 — No unlimited wealth control

Selection weight MUST NOT be based solely on linear ownership of the native asset.

⸻

Q1-CON-014 — Cooldown

A recently selected producer SHOULD receive a temporary weight reduction.

Conceptual example:

CooldownFactor =
    min(
        1,
        blocks_since_last_production / cooldown_target
    )

The exact formula is experimental.

⸻

Q1-CON-015 — Duplicate candidate prevention

A participant MUST NOT appear more than once in the same candidate list.

⸻

12. Producer Eligibility Proof

Each candidate block MUST contain evidence that its producer was eligible.

In deterministic private testnet mode, this proof MAY consist of:

* the producer candidate index;
* the participant registry root;
* the round seed;
* the selection-algorithm version;
* sufficient data to recompute selection.

Future versions MAY use compact cryptographic proofs.

⸻

13. Validator Committee Selection

Q1-CON-016 — Temporary committee

Each round MUST select a temporary validator committee.

⸻

Q1-CON-017 — Deterministic selection

Every honest node MUST derive the same committee from:

* round seed;
* eligible validator registry;
* committee-selection rules;
* consensus parameters.

⸻

Q1-CON-018 — Initial committee size

Recommended values:

Local prototype: 3
Private testnet: 5
Extended private testnet: 7

The committee size MUST be odd where practical.

⸻

Q1-CON-019 — Producer exclusion

The active producer SHOULD NOT serve as a voting validator for its own proposal.

If the producer is present in the general validator registry, it MUST be excluded or assigned zero attestation weight for that proposal.

⸻

Q1-CON-020 — Candidate independence

Fallback producer candidates SHOULD also be excluded from the validator committee for the same round where practical.

This reduces conflicts of interest.

⸻

Q1-CON-021 — Committee diversity

The selector MAY include experimental diversity modifiers using:

* distinct operators;
* network ranges;
* autonomous systems;
* geographic regions;
* device categories;
* implementation versions.

These signals MUST NOT be treated as perfect proof of independence.

⸻

14. Validator Weight

Q1 v0.1 MAY initially use equal validator weight within a committee.

Recommended first implementation:

each selected validator = 1 vote

This keeps the first consensus model understandable.

Later experiments MAY introduce bounded weights.

No validator SHOULD receive unbounded influence from wealth, hardware, or seniority.

⸻

15. Quorum Threshold

Q1-CON-022 — Supermajority

A candidate block requires at least two-thirds of eligible committee weight to finalize.

Let:

W_total = total eligible committee weight
W_yes = valid positive attestation weight

Finalization requires:

3 × W_yes ≥ 2 × W_total

Integer arithmetic MUST be used.

⸻

Q1-CON-023 — Threshold examples

For equal-weight committees:

3 validators → 2 required
5 validators → 4 required
7 validators → 5 required
10 validators → 7 required

⸻

Q1-CON-024 — No rounding ambiguity

The implementation MUST use one explicit integer threshold formula.

Recommended:

threshold =
    floor((2 × W_total) / 3) + 1

⸻

16. Delay Challenge

The active producer MUST execute a delay challenge bound to the current round.

DelayChallenge {
    challenge_version
    chain_id
    block_height
    round_number
    producer_id
    candidate_index
    parent_block_hash
    round_seed
    delay_difficulty
    optional_hdd_commitment_seed
}

The challenge MUST prevent:

* reuse across heights;
* reuse across rounds;
* reuse by another producer;
* precomputation before parent finalization;
* replay from another network.

⸻

17. Production Windows

Each candidate receives a defined production window.

Conceptual schedule:

Candidate 0:
    starts at round opening
    expires after WindowDuration
Candidate 1:
    starts after Candidate 0 timeout
    expires after next WindowDuration
Candidate 2:
    starts after Candidate 1 timeout

⸻

Q1-CON-025 — Deterministic windows

Production windows MUST be derived from consensus parameters and round position.

⸻

Q1-CON-026 — Window duration

The window SHOULD include sufficient time for:

* delay execution;
* HDD experiment, if enabled;
* block construction;
* proposal propagation.

Recommended initial experimental value:

20 seconds per candidate

This value is not final.

⸻

Q1-CON-027 — Early fallback prohibition

A fallback candidate MUST NOT publish a valid primary proposal before its assigned window begins.

⸻

Q1-CON-028 — Late proposal handling

A proposal arriving after its producer window expires SHOULD be rejected for that round.

⸻

18. Candidate Block Construction

The active producer SHALL:

1. derive the round context;
2. verify its own eligibility;
3. derive the delay challenge;
4. execute the delay engine;
5. optionally execute the HDD module;
6. select executable mempool transactions;
7. execute temporary state transition;
8. calculate transaction, receipt, and state roots;
9. calculate fees and rewards;
10. include producer-selection evidence;
11. include delay output and proof;
12. include optional HDD commitment;
13. sign the canonical block proposal;
14. broadcast the proposal.

⸻

19. Block Proposal

A block proposal MUST contain:

BlockProposal {
    protocol_version
    chain_id
    block_height
    round_number
    parent_block_hash
    producer_id
    candidate_index
    producer_selection_proof
    delay_challenge
    delay_output
    delay_proof
    hdd_commitment_optional
    block_body
    block_hash
    producer_signature
}

⸻

20. Proposal Validation

A validator MUST verify at least:

1. supported protocol version;
2. correct chain ID;
3. correct target height;
4. correct round;
5. correct parent finalized block;
6. correct producer candidate;
7. active producer window;
8. producer signature;
9. producer eligibility;
10. delay challenge;
11. delay output;
12. delay proof;
13. applicable HDD rules;
14. block structure;
15. transaction validity;
16. deterministic state transition;
17. transaction root;
18. receipt root;
19. state root;
20. fee calculation;
21. reward calculation;
22. block-size limits;
23. absence of conflicting finalized state.

A validator MUST NOT attest before completing required validation.

⸻

21. Validator Attestation

A positive attestation declares that a validator independently accepts one specific proposal.

ValidatorAttestation {
    attestation_version
    chain_id
    block_height
    round_number
    block_hash
    validator_id
    committee_membership_proof
    validation_result
    attestation_timestamp_observational
    validator_signature
}

For positive votes:

validation_result = VALID

Future versions MAY define signed rejection messages, but they are not required for finalization.

⸻

22. Attestation Signing Domain

The validator signature MUST cover:

DOMAIN_Q1_ATTESTATION
|| chain_id
|| block_height
|| round_number
|| block_hash
|| validator_id
|| validation_result

Changing the block hash, round, or height MUST invalidate the attestation.

⸻

23. One Vote per Height and Round

Q1-CON-029 — No double attestation

A validator MUST NOT sign positive attestations for two different block hashes at the same:

* chain ID;
* block height;
* round number.

Doing so constitutes equivocation.

⸻

Q1-CON-030 — Persistent signing protection

A validator SHOULD persist its signed-attestation history before broadcasting an attestation.

After restart, it MUST detect whether it already signed a conflicting proposal.

⸻

24. Attestation Aggregation

Attestations MAY be collected by:

* the block producer;
* validators;
* any full node;
* a dedicated non-authoritative aggregator.

The aggregator has no special trust.

Every node MUST verify individual signatures and committee membership before accepting an aggregate.

⸻

25. Finalization Certificate

A finalization certificate proves that the required committee threshold attested to one block.

FinalizationCertificate {
    certificate_version
    chain_id
    block_height
    round_number
    block_hash
    parent_block_hash
    committee_root
    total_committee_weight
    approving_weight
    threshold
    attestations
    certificate_hash
}

A compact aggregate signature MAY be introduced later.

The first version MAY store individual signed attestations.

⸻

26. Finalization Rules

A block becomes finalized only if:

1. the block is fully valid;
2. its parent is finalized;
3. the producer was eligible;
4. the delay proof is valid;
5. the validator committee is correct;
6. all included attestations are valid;
7. no validator is counted more than once;
8. approving weight reaches threshold;
9. the certificate references exactly the proposed block;
10. the node has not already finalized another block at the same height;
11. no critical safe-mode condition exists.

⸻

27. Finality

Q1-CON-031 — Immediate protocol finality

Once a valid finalization certificate is accepted, the block is final under normal Q1 operation.

The system MUST NOT require additional confirmation blocks for protocol finality.

User interfaces MAY still display operational confidence or synchronization status separately.

⸻

Q1-CON-032 — No ordinary reorganization below finality

Honest nodes MUST NOT reorganize finalized blocks under normal fork-choice rules.

⸻

Q1-CON-033 — Conflicting finality

If a node observes two valid-looking finalization certificates for different blocks at the same height, it MUST:

* stop producing;
* stop attesting;
* stop finalizing;
* preserve all evidence;
* enter safe mode;
* notify operators;
* expose the conflict through diagnostics.

It MUST NOT choose one certificate silently.

⸻

28. Fork Choice

Fork choice applies only to non-finalized proposals.

The preferred proposal SHOULD be selected using deterministic priority:

1. valid proposal for the current highest round;
2. lowest eligible candidate index within that round;
3. earliest complete valid proposal observed locally;
4. deterministic block-hash ordering only as an explicit final tie-breaker.

However, local observation time MUST NOT independently create finality.

Once a valid finalization certificate exists, it overrides all non-finalized proposals.

⸻

29. Multiple Proposals

29.1 Same producer, same round

If one producer signs two different proposals for the same height and round, this is producer equivocation.

Both proposals and signatures SHALL be stored as evidence.

Validators SHOULD refuse to attest to either after detecting the conflict unless later protocol rules specify otherwise.

⸻

29.2 Different candidates in overlapping windows

If a fallback producer proposes before its valid window, its proposal MUST be rejected.

If network delay causes honest nodes to observe proposals in different orders, the protocol MUST rely on candidate-window validity and finalization threshold, not local arrival order alone.

⸻

29.3 Multiple valid proposals

Only one proposal can obtain a valid finalization certificate if fewer than one-third of committee weight equivocates, under the intended committee assumptions.

This property MUST be tested rather than merely assumed.

⸻

30. Producer Timeout and Fallback

Q1-CON-034 — Timeout

If the active producer does not publish a complete valid proposal before its window expires, nodes SHALL activate the next candidate.

⸻

Q1-CON-035 — Fallback activation

Fallback activation MUST be deterministic from:

* block height;
* round;
* candidate index;
* configured window duration;
* bounded network-time rules;
* signed timeout observations where required.

⸻

Q1-CON-036 — No centralized timeout authority

No single server may declare producer failure for the whole network.

⸻

Q1-CON-037 — Simplified prototype fallback

For the initial local prototype, synchronized round timers MAY be used to activate fallback.

This MUST be labeled experimental and MUST be tested under clock skew.

⸻

31. Round Timeout

If all producer candidates fail, or no proposal reaches finalization threshold, the round MAY time out.

The next round SHALL:

* retain the same target block height;
* use an incremented round number;
* derive a new round seed;
* select a new candidate list;
* select a new committee;
* generate a new delay challenge.

Transactions remain pending unless expired or invalidated.

⸻

32. Round Advancement

A node may advance to the next round when protocol-defined timeout evidence exists.

The exact round-change certificate remains an open design decision.

The initial private-testnet implementation MAY use a signed timeout threshold from the current committee.

Conceptual:

RoundTimeoutCertificate {
    chain_id
    block_height
    expired_round
    committee_root
    timeout_attestations
    timeout_weight
}

A next round SHOULD require at least the same two-thirds threshold or another explicitly defined safe threshold.

⸻

33. Network Partition Behavior

Q1 MUST distinguish safety from liveness during partitions.

33.1 Minority partition

A partition holding less than the finalization threshold MUST NOT finalize new blocks.

It MAY:

* receive transactions;
* maintain a local mempool;
* observe peers;
* wait for reconnection.

⸻

33.2 Supermajority partition

A partition containing sufficient valid committee weight MAY continue finalization if all other protocol requirements are satisfied.

⸻

33.3 Competing supermajorities

The intended committee model assumes that two conflicting blocks cannot both obtain valid two-thirds certificates unless sufficient validators equivocate or membership assumptions fail.

Detection of conflicting certificates triggers safe mode.

⸻

33.4 Partition recovery

After reconnection:

* finalized certificates are authoritative;
* unfinalized proposals are discarded or archived;
* stale mempool entries are reevaluated;
* equivocation evidence is exchanged;
* nodes resynchronize to the finalized chain.

⸻

34. Consensus Safety Assumption

For a committee with total weight W, Q1 v0.1 intends to preserve safety when less than one-third of committee weight signs conflicting proposals.

Conceptually:

ByzantineWeight < W / 3

This is an explicit model assumption, not a guarantee.

The test plan MUST challenge this assumption.

⸻

35. Liveness Assumption

Q1 v0.1 intends to make progress when:

* enough eligible producers are online;
* at least the finalization threshold of committee weight is online and responsive;
* network delay remains within configured bounds;
* honest nodes share compatible protocol state;
* the delay engine completes;
* critical safe-mode conditions are absent.

If these conditions do not hold, the network MAY stop finalizing.

⸻

36. Equivocation Evidence

36.1 Producer equivocation

Evidence consists of two distinct validly signed proposals from the same producer for the same height and round.

⸻

36.2 Validator equivocation

Evidence consists of two distinct positive attestations signed by the same validator for different blocks at the same height and round.

⸻

36.3 Evidence structure

EquivocationEvidence {
    evidence_version
    chain_id
    offense_type
    offender_id
    block_height
    round_number
    signed_object_a
    signed_object_b
    evidence_hash
    reporter_id_optional
}

⸻

Q1-CON-038 — Objective verification

Equivocation evidence MUST be independently and deterministically verifiable.

AI classification is not required.

⸻

37. Penalties

Q1 v0.1 MAY initially use virtual or testnet-only penalties.

Possible consequences include:

* loss of round reward;
* temporary cooldown;
* reduced participation score;
* temporary role suspension;
* removal from private-testnet registry;
* test collateral slashing in later experiments.

No permanent economic slashing rule is approved in this version.

Penalty rules MUST NOT be invented in implementation without specification.

⸻

38. Consensus Reputation

Participation reputation MAY be recorded for experimentation.

Possible inputs:

* successful proposal rate;
* correct attestation rate;
* availability;
* equivocation history;
* invalid proposal history;
* missed production windows;
* protocol compatibility.

Reputation MUST NOT become an irreversible monopoly mechanism.

It SHOULD:

* decay over time;
* have bounded influence;
* be independently calculated;
* exclude unverifiable AI judgments from consensus weight.

⸻

39. Delay and Consensus Relationship

The delay engine proves completion of the configured sequential task.

It does not independently prove:

* correct transactions;
* correct balances;
* committee approval;
* producer honesty;
* HDD physical authenticity;
* finality.

A valid block requires both:

ValidDelayProof
AND
ValidBlockState
AND
ValidFinalizationCertificate

⸻

40. HDD and Consensus Relationship

The HDD module is experimental.

For Q1 v0.1:

* consensus MUST work with HDD disabled;
* HDD output MUST NOT be the sole source of producer eligibility;
* HDD telemetry MUST NOT be treated as physical truth;
* forged HDD telemetry MUST NOT independently finalize a block;
* HDD contribution MAY affect experimental reward or eligibility only if explicitly enabled in genesis.

Recommended initial mode:

HDD_MODE = TELEMETRY_ONLY

Later experimental modes MAY include:

HDD_MODE = DELAY_INPUT
HDD_MODE = REWARD_MODIFIER
HDD_MODE = ELIGIBILITY_MODIFIER

Each mode MUST be separately versioned and tested.

⸻

41. AI and Consensus Relationship

AI MUST remain outside the consensus decision path.

AI MAY:

* detect unusual voting;
* identify repeated producer concentration;
* flag timing anomalies;
* compare HDD patterns;
* suggest attack scenarios;
* generate operator alerts.

AI MUST NOT:

* cast validator votes;
* modify committee membership;
* reject a mathematically valid certificate;
* create a finalization certificate;
* confiscate funds;
* alter participant weight automatically;
* suspend consensus identities without deterministic protocol evidence.

⸻

42. Consensus State Machine

Each node SHALL maintain a consensus state machine.

WAITING_FOR_PARENT
        ↓
HEIGHT_INITIALIZED
        ↓
ROUND_INITIALIZED
        ↓
CANDIDATES_SELECTED
        ↓
COMMITTEE_SELECTED
        ↓
PRODUCER_WINDOW_ACTIVE
        ↓
PROPOSAL_RECEIVED
        ↓
PROPOSAL_VALIDATING
        ↓
PROPOSAL_VALID
        ↓
ATTESTATION_SENT_OR_OBSERVED
        ↓
QUORUM_REACHED
        ↓
FINALIZATION_CERTIFICATE_BUILT
        ↓
BLOCK_FINALIZED
        ↓
NEXT_HEIGHT

Alternative transitions:

PROPOSAL_INVALID
PRODUCER_TIMEOUT
FALLBACK_ACTIVATED
ROUND_TIMEOUT
ROUND_ADVANCED
CONFLICT_DETECTED
SAFE_MODE

⸻

43. State Transition Rules

From ROUND_INITIALIZED

The node MUST derive:

* round seed;
* producer candidates;
* committee;
* delay parameters;
* production windows.

⸻

From PRODUCER_WINDOW_ACTIVE

The node MAY accept only proposals from the currently eligible candidate.

⸻

From PROPOSAL_RECEIVED

The node MUST validate the proposal before voting.

⸻

From PROPOSAL_VALID

A selected validator MAY sign exactly one positive attestation for that height and round.

⸻

From QUORUM_REACHED

The node MUST independently validate the certificate before finalization.

⸻

From BLOCK_FINALIZED

The node MUST:

* commit ledger state;
* update finalized height;
* terminate lower or equal rounds for that height;
* reject competing proposals at that height;
* begin the next height.

⸻

44. Consensus Message Types

Q1 v0.1 SHOULD define:

CONSENSUS_STATUS
PROPOSAL_ANNOUNCEMENT
PROPOSAL_REQUEST
PROPOSAL_RESPONSE
ATTESTATION_ANNOUNCEMENT
ATTESTATION_REQUEST
ATTESTATION_RESPONSE
FINALIZATION_ANNOUNCEMENT
FINALIZATION_REQUEST
FINALIZATION_RESPONSE
TIMEOUT_ATTESTATION
ROUND_CHANGE_ANNOUNCEMENT
EQUIVOCATION_EVIDENCE
SAFE_MODE_NOTICE

Every message MUST have explicit size and rate limits.

⸻

45. Message Deduplication

Every consensus message MUST have a deterministic message identifier.

Nodes SHALL deduplicate messages using:

* message ID;
* sender;
* height;
* round;
* object hash.

Duplicate messages MUST NOT be counted twice toward quorum.

⸻

46. Consensus Synchronization

A node joining or recovering MUST first synchronize finalized consensus state.

It MUST obtain and verify:

* finalized block headers;
* finalization certificates;
* participant registry state;
* protocol parameters;
* current epoch;
* latest finalized block;
* optional snapshots.

A node MUST NOT attest or produce blocks while materially out of sync.

⸻

47. Participation Readiness

A node MAY enter producer or validator mode only if:

* its ledger is synchronized;
* its consensus parameters match;
* its clock is within operational tolerance;
* required keys are available;
* its protocol version is accepted;
* its participant status is active;
* no safe-mode condition exists.

⸻

48. Local Clock Use

Local clocks MAY be used for:

* production-window timers;
* timeout observations;
* metrics;
* log timestamps;
* operator warnings.

Local clocks MUST NOT independently determine:

* transaction ordering;
* block validity;
* finality;
* participant identity;
* account state.

Clock-dependent behavior MUST be tested with:

* positive skew;
* negative skew;
* abrupt clock changes;
* drifting clocks;
* malicious timestamps.

⸻

49. Safe Mode

A node MUST enter safe mode on detection of:

* two conflicting valid finalization certificates;
* finalized-state root inconsistency;
* critical database corruption;
* impossible participant-registry mismatch;
* unsupported consensus-critical protocol transition;
* cryptographic provider failure;
* internal deterministic execution disagreement;
* finalization certificate referencing an invalid block;
* evidence that local finalized state differs from a verified certificate.

⸻

49.1 Safe-mode behavior

In safe mode, the node MUST:

* stop block production;
* stop validator signing;
* stop finalization;
* preserve ledger and evidence;
* continue read-only APIs where safe;
* expose diagnostic reason;
* emit a critical event;
* require explicit recovery procedure.

⸻

49.2 No automatic chain choice

A node MUST NOT automatically choose between conflicting finalized histories.

⸻

50. Consensus Recovery

Recovery MAY occur only after:

* the conflict is understood;
* evidence is preserved;
* a protocol-defined or testnet-operator recovery decision is documented;
* affected nodes agree on a recovery configuration;
* the recovery action is versioned.

For private experimental testnets, recovery MAY include:

* restarting from a known finalized checkpoint;
* resetting the testnet;
* removing malicious test participants;
* changing protocol parameters;
* generating a new genesis.

Such recovery is not ordinary decentralized consensus and MUST be documented as testnet intervention.

⸻

51. Genesis Consensus Configuration

Genesis MUST define:

chain_id
protocol_version
participant_registry
producer_candidate_count
committee_size
committee_weight_model
quorum_threshold_rule
block_target_interval
producer_window_duration
round_timeout
maximum_round_number_optional
delay_engine_type
delay_difficulty
hdd_mode
penalty_mode
epoch_length
fork_choice_version
consensus_message_version

All nodes MUST reject incompatible genesis consensus configuration.

⸻

52. Recommended Initial Parameters

The following parameters are starting points only:

producer_candidate_count = 3
committee_size = 5
validator_weight_model = EQUAL
quorum_rule = TWO_THIRDS_PLUS_ONE
producer_window_duration = 20 seconds
round_timeout = 70 seconds
target_block_interval = 30 seconds
epoch_length = 100 blocks
hdd_mode = TELEMETRY_ONLY
penalty_mode = VIRTUAL

Some values may conflict operationally and MUST be refined through simulation.

For example, a 30-second target block interval may not be compatible with three sequential 20-second producer windows. The implementation MUST treat block target and worst-case round duration as separate metrics.

⸻

53. Consensus Metrics

The system MUST record:

* block height;
* round number;
* producer candidate list;
* selected committee;
* active candidate index;
* delay start and completion;
* proposal time;
* proposal propagation time;
* validation duration;
* attestation arrival times;
* quorum time;
* finalization time;
* fallback activations;
* round changes;
* missed producer windows;
* rejected proposals;
* equivocation evidence;
* committee participation rate;
* finalized blocks per participant;
* concentration metrics;
* safe-mode events.

⸻

54. Consensus Performance Targets

Initial experimental targets:

* normal block finalization within the target operating range;
* delay-proof verification significantly faster than generation;
* proposal validation before the production window expires;
* finalization with one offline validator in a five-validator committee;
* deterministic finalization across all honest nodes;
* no conflicting finalization under less than one-third Byzantine committee weight;
* recovery from temporary producer failure through fallback;
* recovery from temporary network partition without finalized-state divergence.

These are test targets, not guarantees.

⸻

55. Consensus Security Invariants

The implementation MUST test the following invariants.

Invariant 1 — Single finality

An honest node never finalizes two different blocks at the same height.

⸻

Invariant 2 — Parent finality

A finalized block always references a finalized parent.

⸻

Invariant 3 — Validity before attestation

An honest validator never attests to an invalid block.

⸻

Invariant 4 — One validator vote

An honest validator signs no more than one positive attestation per height and round.

⸻

Invariant 5 — Correct threshold

No block finalizes below the required committee threshold.

⸻

Invariant 6 — Deterministic selection

Honest nodes derive identical candidate and committee lists from identical context.

⸻

Invariant 7 — Delay binding

A delay proof cannot be reused for another producer, height, round, or chain.

⸻

Invariant 8 — Finality dominates proposals

A finalized certificate overrides every non-finalized competing proposal.

⸻

Invariant 9 — Safe conflict handling

Conflicting finalization evidence causes safe mode, not silent chain choice.

⸻

Invariant 10 — AI independence

Consensus continues when the AI observer is unavailable.

⸻

Invariant 11 — HDD independence

Base consensus continues when the HDD module is disabled.

⸻

56. Required Consensus Test Scenarios

The automated test suite MUST include:

1. normal block production;
2. primary producer success;
3. primary producer offline;
4. first fallback producer success;
5. all candidates offline;
6. round timeout;
7. next-round selection;
8. one validator offline;
9. two validators offline in a five-member committee;
10. malicious producer submits invalid block;
11. malicious producer submits invalid delay proof;
12. malicious producer equivocates;
13. validator attests without full validation;
14. validator double-votes;
15. duplicate attestation delivery;
16. malformed finalization certificate;
17. certificate below threshold;
18. incorrect committee membership;
19. incorrect candidate selection proof;
20. proposal from early fallback;
21. late proposal;
22. conflicting proposals in one round;
23. network partition before proposal;
24. network partition during voting;
25. network partition after finalization;
26. delayed attestations;
27. reordered messages;
28. replayed old proposal;
29. replayed old attestation;
30. cross-chain consensus replay;
31. wrong protocol version;
32. participant-registry mismatch;
33. producer clock skew;
34. validator clock skew;
35. node restart after signing;
36. node restart before finalization;
37. conflicting finalization certificates;
38. AI observer shutdown;
39. HDD plugin failure;
40. telemetry failure;
41. database failure during finalization;
42. synchronization before validator activation;
43. stale node attempting to vote;
44. candidate concentration over many rounds;
45. committee concentration over many rounds;
46. one-third Byzantine threshold boundary;
47. more than one-third Byzantine behavior;
48. message flooding;
49. equivocation-evidence propagation;
50. safe-mode activation and evidence preservation.

⸻

57. Minimum Consensus Prototype Acceptance

The Q1 consensus prototype is complete when:

1. at least four local nodes operate independently;
2. one deterministic participant registry is loaded;
3. each height begins at round zero;
4. producer candidates are identically selected by all nodes;
5. a committee is identically selected by all nodes;
6. the active producer executes the delay module;
7. the producer broadcasts a signed proposal;
8. committee members independently validate it;
9. validators produce signed attestations;
10. duplicate attestations are ignored;
11. the required threshold is calculated identically;
12. a finalization certificate is constructed;
13. all honest nodes finalize the same block;
14. a failed primary producer activates fallback;
15. a failed round advances deterministically;
16. invalid proposals receive no honest attestations;
17. double-voting produces objective evidence;
18. restart protection prevents accidental equivocation;
19. conflicting certificates activate safe mode;
20. disabling AI and HDD does not stop base consensus.

⸻

58. Private Testnet Acceptance Criteria

Before private distributed deployment, the consensus system MUST demonstrate:

* 1,000 consecutively finalized blocks;
* no conflicting honest finalized state;
* successful producer fallback;
* successful round advancement;
* stable committee selection;
* deterministic certificate verification;
* operation under ordinary network latency;
* operation with at least one-third of nodes temporarily offline, subject to committee composition;
* rejection of invalid delay proofs;
* detection of producer and validator equivocation;
* correct recovery after temporary partitions;
* preserved safety under tested Byzantine behavior below the intended threshold;
* measured liveness degradation above that threshold;
* complete telemetry for every round.

⸻

59. Known Limitations of v0.1

Q1 v0.1 does not yet prove:

* permissionless Sybil resistance;
* secure public participant admission;
* production-grade randomness;
* production-grade VDF security;
* resistance to large-scale network-level attacks;
* correct economic penalties;
* geographic independence;
* hardware authenticity;
* long-term incentive stability;
* public-mainnet safety.

The first participant registry may be permissioned for laboratory purposes.

This limitation MUST be stated openly.

⸻

60. Open Decisions

The following remain unresolved:

1. exact participant-admission mechanism;
2. exact producer-weight formula;
3. exact committee-selection algorithm;
4. exact randomness construction;
5. whether a VRF will be used;
6. exact cooldown rule;
7. exact reputation-decay formula;
8. exact producer-window timing model;
9. exact round-change certificate;
10. whether rejection votes are signed;
11. whether finalization signatures are aggregated;
12. exact fork-choice tie-breaker;
13. exact penalty model;
14. exact epoch length;
15. exact protocol for participant-set updates;
16. whether validators lock collateral;
17. whether HDD contribution affects eligibility;
18. whether delay executors may be separate from producers;
19. whether committees are selected per round or per height;
20. how public Sybil resistance will work;
21. how network diversity will be measured;
22. how randomness bias will be minimized;
23. how emergency recovery will operate on a future public network.

All decisions MUST be entered into OPEN_DECISIONS.md.

⸻

61. Codex Implementation Rules

Codex MUST:

1. implement consensus as an explicit state machine;
2. separate producer selection from committee selection;
3. separate block validity from finalization;
4. use checked integer arithmetic for thresholds;
5. persist validator signing history before broadcasting votes;
6. verify every attestation independently;
7. reject duplicate committee weight;
8. bind every proof and signature to chain, height, and round;
9. implement producer fallback through configurable rules;
10. implement safe mode before adversarial testing;
11. store objective equivocation evidence;
12. keep AI outside consensus;
13. keep HDD optional;
14. expose consensus metrics and rejection reasons;
15. make all experimental parameters configurable through genesis;
16. write deterministic multi-node tests;
17. simulate malicious nodes;
18. record every unresolved assumption.

Codex MUST NOT:

* finalize a block because the producer claims success;
* count unsigned votes;
* count the same validator twice;
* trust local arrival order as finality;
* allow one API call to rewrite consensus state;
* accept a delay output without verification;
* permit an early fallback proposal;
* silently resolve conflicting finality;
* use floating-point quorum calculations;
* allow AI risk scores to change votes;
* assume HDD telemetry is genuine;
* treat a permissioned prototype as proof of public decentralization.

⸻

62. Final Consensus Principle

Q1 consensus MUST preserve the following chain of authority:

A participant may become eligible only under public rules.
A producer may propose only during its assigned opportunity.
A proposal is meaningful only if its work and state are valid.
A validator may attest only after independent verification.
A block becomes final only after a verifiable supermajority certificate.
If finality conflicts, the network must stop and preserve the truth of the conflict.

Q1 v0.1 succeeds at the consensus layer when honest nodes do not need to trust:

* the producer;
* the validator aggregator;
* the HDD device;
* the AI observer;
* the explorer;
* the testnet operator;
* or local message timing

to determine whether a block is valid and finalized.
