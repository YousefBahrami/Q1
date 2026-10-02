Q1 Delay Engine Specification

06_DELAY_ENGINE.md

Project: Q1 Experimental Distributed Ledger
Protocol Version: 0.1
Document Version: 0.1.0
Status: Draft for Engineering Review
Classification: Experimental — Not for Production or Financial Use

⸻

1. Purpose

This document defines the Q1 Delay Engine.

The Delay Engine is responsible for creating and verifying a measurable period of sequential work before a selected producer may publish a candidate block.

The purpose of the Delay Engine is not to maximize computation.

Its purpose is to test whether Q1 can create a meaningful block-production cost through:

* sequential work;
* limited parallel advantage;
* public verification;
* challenge binding;
* configurable difficulty;
* low verification cost;
* measurable energy use;
* replaceable implementations.

This document defines:

* delay-engine responsibilities;
* challenge construction;
* execution rules;
* proof structure;
* verification rules;
* difficulty parameters;
* benchmark requirements;
* failure behavior;
* the initial prototype delay;
* the transition path toward a formal verifiable delay function;
* the optional relationship with HDD experiments.

⸻

2. Core Principle

Q1 does not define useful work as maximum energy expenditure.

Q1 defines the initial delay objective as:

A selected producer must complete a challenge-bound sequence of dependent operations that cannot be validly skipped, precomputed, reused, or verified only by trust.

The Delay Engine MUST distinguish between:

* generation cost, paid by the active producer;
* verification cost, paid by validators and full nodes.

The desired relationship is:

verification_cost << generation_cost

Q1 v0.1 may not fully achieve this property in its first simplified implementation.

Any implementation that does not provide strong asymmetry MUST be explicitly labeled experimental.

⸻

3. Delay Engine Role

The Delay Engine SHALL:

1. derive or accept a canonical delay challenge;
2. execute a configured sequential function;
3. produce a deterministic output;
4. produce a proof or verification artifact;
5. expose generation metrics;
6. allow independent deterministic verification;
7. bind the result to a producer, height, round, chain, and parent block;
8. reject replayed or mismatched proofs;
9. support configurable difficulty;
10. support multiple interchangeable delay implementations.

The Delay Engine MUST NOT:

* determine block validity by itself;
* choose the block producer;
* finalize a block;
* modify balances;
* trust local timestamps as proof;
* depend on AI output;
* assume HDD telemetry is genuine;
* allow the producer to select a favorable challenge.

⸻

4. Delay Engine Interface

All delay implementations MUST satisfy one stable interface.

DelayEngine {
    engine_id()
    engine_version()
    capabilities()
    validate_parameters(parameters)
    derive_challenge(context)
    execute(challenge, parameters)
    verify(challenge, output, proof, parameters)
    estimate_generation_cost(parameters, hardware_profile_optional)
    estimate_verification_cost(parameters)
    encode_output(output)
    decode_output(bytes)
    encode_proof(proof)
    decode_proof(bytes)
}

The consensus layer MUST interact only through this interface.

⸻

5. Supported Delay Engine Classes

Q1 v0.1 SHALL recognize the following implementation classes.

5.1 Mock Delay Engine

Purpose:

* unit tests;
* API tests;
* state-machine tests;
* rapid local debugging.

Properties:

* no security claim;
* deterministic;
* near-zero delay;
* MUST NOT be enabled in security or performance tests.

Identifier:

q1-delay-mock-v1

⸻

5.2 Sequential Hash Delay

Purpose:

* first executable prototype;
* testing challenge binding;
* testing difficulty control;
* testing producer timing;
* testing proof transport;
* measuring real generation cost.

Properties:

* repeated dependent hashing;
* limited ordinary parallelization for one chain;
* expensive verification unless checkpoints or proof methods are added;
* NOT a formal production-grade VDF.

Identifier:

q1-delay-sequential-hash-v1

⸻

5.3 Checkpointed Sequential Hash Delay

Purpose:

* research into partial verification;
* corruption detection;
* bounded recomputation;
* proof-size tradeoffs.

Properties:

* sequential hash chain;
* periodic committed checkpoints;
* verification MAY sample or recompute segments;
* sampling alone MUST NOT be claimed as full cryptographic verification.

Identifier:

q1-delay-checkpointed-hash-v1

⸻

5.4 Formal VDF Delay

Purpose:

* future production-oriented research;
* strong sequentiality;
* compact proof;
* efficient verification.

Properties:

* based on a published, reviewed construction;
* MUST use established cryptographic libraries where available;
* MUST undergo independent security review before production claims.

Temporary identifier:

q1-delay-vdf-experimental-v1

No specific formal VDF construction is approved in this document.

⸻

5.5 HDD Composite Delay

Purpose:

* experimental combination of mathematical delay and HDD workload.

Properties:

* HDD contribution is challenge-bound;
* mathematical verification remains authoritative;
* HDD telemetry is not treated as physical truth;
* base consensus MUST function without this engine.

Identifier:

q1-delay-hdd-composite-v1

⸻

6. Delay Context

The Delay Engine SHALL receive a deterministic context.

DelayContext {
    protocol_version
    chain_id
    block_height
    round_number
    candidate_index
    producer_id
    parent_block_hash
    parent_finalization_certificate_hash
    round_seed
    delay_engine_id
    delay_engine_version
    delay_difficulty
    consensus_parameters_hash
    optional_hdd_mode
}

All consensus-critical fields MUST be committed into the challenge.

This conceptual DelayContext and the challenge sketch below do not define the
approved BlockHeaderBodyV1 wire schema. DEC-Q1-027 Session 5C excludes
candidate_index, delay_engine_id, delay_difficulty, and full proof material
from Header V1. The Header contains one `DelayEvidenceHash`; engine ID,
difficulty, output, and proof belong to the separate five-field
DelayEvidenceV1. The existing DELAY_OUTPUT domain is not reinterpreted, and
the required DELAY_EVIDENCE domain has no registered numeric assignment yet.
No delay engine, challenge construction, verifier, or runtime activation is
authorized by that schema decision.

⸻

7. Challenge Construction

Q1-DLY-001 — Canonical challenge

The challenge SHALL be derived conceptually as:

DelayChallenge =
    HASH(
        DOMAIN_Q1_DELAY_CHALLENGE_V1
        || protocol_version
        || chain_id
        || block_height
        || round_number
        || candidate_index
        || producer_id
        || parent_block_hash
        || parent_finalization_certificate_hash
        || round_seed
        || delay_engine_id
        || delay_engine_version
        || delay_difficulty
        || consensus_parameters_hash
        || optional_hdd_seed
    )

⸻

Q1-DLY-002 — Freshness

A valid challenge MUST be unique to:

* one network;
* one parent finalized block;
* one block height;
* one round;
* one candidate position;
* one producer identity;
* one delay implementation;
* one difficulty setting.

⸻

Q1-DLY-003 — Precomputation resistance

The challenge MUST depend on finalized data unavailable before the previous block finalizes.

⸻

Q1-DLY-004 — Producer binding

A proof generated for one producer MUST NOT be valid for another producer.

⸻

Q1-DLY-005 — Round binding

A proof generated for one round MUST NOT be valid in a later round at the same height.

⸻

Q1-DLY-006 — Engine binding

A proof generated under one delay-engine version MUST NOT be interpreted under another.

⸻

8. Sequential Hash Delay v1

The first executable delay prototype MAY use a dependent hash chain.

8.1 Initial state

x_0 =
    HASH(
        DOMAIN_Q1_DELAY_INITIAL_STATE
        || DelayChallenge
    )

⸻

8.2 Iteration

For iteration index i from 1 to N:

x_i =
    HASH(
        DOMAIN_Q1_DELAY_STEP
        || DelayChallenge
        || encode_u64(i)
        || x_(i-1)
    )

Where:

N = delay_iterations

The output is:

DelayOutput = x_N

⸻

8.3 Sequential dependency

Each step depends on the immediately previous step.

The producer cannot compute x_i without first obtaining x_(i-1).

This reduces the benefit of ordinary parallelization for a single chain.

It does not prove that specialized hardware cannot achieve major speedups.

⸻

9. Sequential Hash Proof v1

The simplest full-verification proof is:

DelayProof {
    proof_version
    engine_id
    engine_version
    challenge_hash
    iteration_count
    final_output
}

In this simplest mode, a validator verifies by recomputing the entire sequence.

This mode provides correctness testing but poor verification asymmetry.

It MUST be labeled:

FULL_RECOMPUTATION_RESEARCH_MODE

⸻

10. Checkpointed Proof Mode

The engine MAY emit checkpoints at fixed intervals.

Example:

CheckpointInterval = K

Checkpoints:

x_K
x_2K
x_3K
...
x_N

Proof:

CheckpointedDelayProof {
    proof_version
    challenge_hash
    iteration_count
    checkpoint_interval
    checkpoint_hashes[]
    final_output
    checkpoint_commitment_root
}

⸻

10.1 Full checkpoint verification

A verifier MAY recompute every segment between checkpoints.

This does not reduce total hashing substantially but supports:

* parallel verification of separate segments;
* fault localization;
* proof integrity testing;
* checkpoint commitment experiments.

⸻

10.2 Sampled verification

A verifier MAY randomly select some segments for experimental checking.

Sampled verification MUST NOT be treated as complete cryptographic proof.

It MAY be used for:

* telemetry;
* fraud-detection research;
* AI anomaly input;
* benchmark comparison.

⸻

11. Formal VDF Requirements

A future formal VDF implementation MUST satisfy the following before being accepted as production-oriented.

Q1-DLY-007 — Sequentiality claim

The construction MUST have a published security argument that generation requires a defined sequence of dependent operations under stated assumptions.

⸻

Q1-DLY-008 — Efficient verification

Verification SHOULD be substantially cheaper than generation.

Target:

verification_time <= 1% of generation_time

This is a research target, not a mandatory mathematical constant.

⸻

Q1-DLY-009 — Compact proof

The proof SHOULD remain small enough for routine block propagation.

⸻

Q1-DLY-010 — Public construction

The construction MUST be publicly documented and independently reviewable.

⸻

Q1-DLY-011 — No custom cryptography by convenience

Q1 MUST NOT invent an unreviewed VDF merely to claim novelty.

⸻

Q1-DLY-012 — Trusted setup disclosure

If the selected VDF requires trusted setup, this MUST be documented and evaluated.

⸻

Q1-DLY-013 — Parameter transparency

All security and performance parameters MUST be public and versioned.

⸻

12. Difficulty Model

Delay difficulty controls the amount of required sequential work.

For Sequential Hash Delay v1:

delay_difficulty = iteration_count

Where:

iteration_count > 0

⸻

Q1-DLY-014 — Integer difficulty

Difficulty MUST be represented using checked integer arithmetic.

⸻

Q1-DLY-015 — Bounds

Genesis MUST define:

minimum_delay_difficulty
maximum_delay_difficulty
initial_delay_difficulty

Nodes MUST reject blocks containing difficulty outside allowed bounds.

⸻

Q1-DLY-016 — Consensus value

The required difficulty for each round MUST be derived from finalized consensus state.

A producer MUST NOT choose its own difficulty.

⸻

13. Difficulty Adjustment

Q1 v0.1 SHOULD initially use simple epoch-based adjustment.

The objective is to maintain a target median delay-generation duration.

Let:

T_target = target delay time
T_observed = median successful delay time in previous adjustment window
D_old = previous difficulty

Conceptual update:

D_raw =
    D_old × T_target / T_observed

Because floating-point arithmetic is prohibited in consensus, implementation MUST use bounded integer ratios.

Conceptual integer form:

D_raw =
    floor(
        D_old
        × T_target_units
        / max(T_observed_units, 1)
    )

⸻

Q1-DLY-017 — Bounded adjustment

Difficulty MUST NOT change without limit in one adjustment.

Recommended initial bounds:

minimum change = -12.5%
maximum change = +12.5%

Equivalent integer ratio bounds MAY be used.

⸻

Q1-DLY-018 — Adjustment interval

Recommended initial adjustment interval:

100 finalized blocks

This value is experimental.

⸻

Q1-DLY-019 — Robust statistic

Adjustment SHOULD use a median or trimmed mean rather than a simple average to reduce manipulation by outliers.

⸻

Q1-DLY-020 — Telemetry versus consensus

Consensus adjustment MUST rely only on protocol-committed or deterministically derived measurements.

Unverified local CPU telemetry MUST NOT directly set consensus difficulty.

⸻

14. Measuring Delay Without Trusting Local Time

This is a central research problem.

Local clocks are not reliable proof of elapsed time.

Therefore Q1 v0.1 SHALL distinguish:

14.1 Protocol work measure

For Sequential Hash Delay:

verified iteration count

This is deterministic.

⸻

14.2 Observed wall-clock duration

Measured locally for telemetry.

This is not authoritative.

⸻

14.3 Network-observed timing

Proposal and attestation timing observed by multiple nodes.

This MAY inform research and future designs but is not sufficient alone to prove sequential execution.

⸻

14.4 Formal VDF work measure

A valid VDF proof under agreed parameters.

This is the intended future authoritative mechanism.

⸻

15. Producer Execution Procedure

A selected producer SHALL:

1. Confirm current finalized parent.
2. Confirm current height and round.
3. Confirm candidate eligibility.
4. Load required delay parameters.
5. Derive canonical challenge.
6. Record local telemetry start event.
7. Execute selected delay engine.
8. Optionally execute HDD-linked operations.
9. Produce output and proof.
10. Verify its own proof locally.
11. Record local completion telemetry.
12. Insert challenge, output, and proof into block proposal.
13. Sign and broadcast proposal.

A producer MUST NOT broadcast a proposal with an unverified local proof.

⸻

16. Validator Verification Procedure

A validator SHALL:

1. Decode delay fields.
2. Verify engine ID and version.
3. Verify configured engine is allowed.
4. Reconstruct delay context.
5. Recompute canonical challenge.
6. Compare challenge hashes.
7. Verify difficulty.
8. Decode output and proof.
9. Execute delay-engine verification.
10. Reject on any mismatch.
11. Record verification time.
12. Continue with block-state validation only if delay verification succeeds.

The exact ordering between delay verification and transaction execution MAY be optimized, but cheap rejection checks SHOULD occur first.

⸻

17. Delay Proof Rejection Codes

The implementation MUST define stable rejection codes.

Recommended:

DLY_UNSUPPORTED_ENGINE
DLY_UNSUPPORTED_VERSION
DLY_INVALID_PARAMETERS
DLY_WRONG_CHAIN
DLY_WRONG_HEIGHT
DLY_WRONG_ROUND
DLY_WRONG_PRODUCER
DLY_WRONG_CANDIDATE_INDEX
DLY_PARENT_MISMATCH
DLY_CHALLENGE_MISMATCH
DLY_DIFFICULTY_MISMATCH
DLY_MALFORMED_OUTPUT
DLY_MALFORMED_PROOF
DLY_PROOF_VERIFICATION_FAILED
DLY_OUTPUT_MISMATCH
DLY_REPLAY_DETECTED
DLY_PROOF_TOO_LARGE
DLY_VERIFICATION_RESOURCE_LIMIT
DLY_ENGINE_INTERNAL_ERROR

⸻

18. Proof Size Limits

Genesis or protocol configuration MUST define:

maximum_delay_output_size
maximum_delay_proof_size

Proofs exceeding limits MUST be rejected before expensive verification where possible.

⸻

19. Resource Limits

Delay verification MUST be protected against denial-of-service attacks.

Nodes MUST enforce:

* maximum proof bytes;
* maximum decoding depth;
* maximum iteration count;
* maximum memory allocation;
* maximum verification time policy;
* supported engine whitelist;
* supported parameter ranges.

A malformed proof MUST NOT crash the node.

⸻

20. Parallelism Rules

20.1 Generation

For a single Sequential Hash Delay chain, each iteration depends on the previous one.

Multiple producers may execute independent challenges in parallel.

A single producer MAY use parallelism for:

* input preparation;
* telemetry;
* HDD work;
* block construction;
* unrelated validation.

It MUST NOT claim multiple independent chains as one sequential proof unless the engine specification permits it.

⸻

20.2 Verification

Checkpointed segments MAY be verified in parallel in research mode.

Formal VDF verification MAY use implementation-specific parallelism.

Verification must remain deterministic.

⸻

21. Hardware Variance

Different hardware will complete the same difficulty at different speeds.

Q1 v0.1 does not assume equal hardware performance.

The system MUST measure:

* CPU model where available;
* architecture;
* thread count;
* implementation version;
* generation duration;
* verification duration;
* energy estimate;
* thermal throttling indicators where available.

These metrics are observational.

They MUST NOT be treated as trusted identity.

⸻

22. Hardware Advantage and Fairness

Q1 does not claim that sequential delay removes all hardware advantage.

The research objective is to test whether it can reduce the value of massive parallel hardware compared with unrestricted hash racing.

The following MUST be measured:

* speed ratio between ordinary CPU classes;
* speed ratio between CPU and GPU where applicable;
* speed ratio between CPU and specialized implementation;
* energy per completed delay;
* cost per completed delay;
* producer concentration under hardware heterogeneity;
* impact of candidate limits on total wasted work.

⸻

23. Multiple Candidate Work

Q1 selects multiple fallback candidates.

To reduce wasted work, candidate execution modes MAY include:

Mode A — Sequential activation

Only the currently active candidate starts delay work.

Advantage:

* lowest duplicated energy.

Disadvantage:

* slower fallback.

⸻

Mode B — Staggered preparation

Fallback candidates begin partial preparation but not full proof execution.

Advantage:

* moderate fallback speed.

Disadvantage:

* implementation complexity.

⸻

Mode C — Parallel execution

All candidates execute delay work simultaneously.

Advantage:

* fastest fallback.

Disadvantage:

* duplicated energy.

Recommended initial mode:

SEQUENTIAL_ACTIVATION

Alternative modes SHALL be tested later.

⸻

24. Delay Completion and Producer Window

A valid proof does not permit publication outside the producer’s valid window.

A proposal requires both:

ValidDelayProof
AND
ValidProducerWindow

A producer finishing early MUST wait until its window permits publication if protocol rules require a later start.

A producer finishing after its window expires loses the opportunity for that round.

⸻

25. Early Completion

Q1 v0.1 MUST consider whether very fast hardware can finish delay substantially earlier than the intended target.

Potential responses for future experiments include:

* increasing global difficulty;
* using candidate-specific target windows;
* adding deterministic release timing;
* using formal VDF calibration;
* reducing weight of repeated fast dominance;
* changing producer-selection fairness.

No hidden artificial sleep SHOULD be treated as proof.

Operating-system sleep is not a cryptographic delay.

⸻

26. Artificial Waiting

Q1 MUST distinguish between:

* sequential work;
* wall-clock waiting;
* device sleeping;
* network delay;
* HDD latency.

Only a verified engine output counts as delay work.

A producer cannot satisfy the delay requirement by merely waiting.

⸻

27. HDD Composite Mode

If HDD Composite Delay is enabled, the challenge MAY be split into:

MathChallenge
HDDChallenge

Conceptually:

HDDSeed =
    HASH(
        DOMAIN_Q1_HDD_SEED
        || DelayChallenge
    )

The HDD module produces:

HDDCommitment

The mathematical initial state MAY then be:

x_0 =
    HASH(
        DOMAIN_Q1_COMPOSITE_INITIAL_STATE
        || DelayChallenge
        || HDDCommitment
    )

⸻

Q1-DLY-021 — No HDD sole authority

A valid HDD commitment alone MUST NOT satisfy the delay requirement.

⸻

Q1-DLY-022 — Replay resistance

The HDD commitment MUST be bound to the current delay challenge.

⸻

Q1-DLY-023 — Simulated hardware acceptance

The system MUST support simulated HDD outputs for adversarial tests.

⸻

Q1-DLY-024 — Mode declaration

The block MUST declare the configured HDD participation mode.

A producer MUST NOT substitute another mode.

⸻

28. HDD Failure Behavior

If HDD is:

TELEMETRY_ONLY

then HDD failure MUST NOT invalidate an otherwise valid block.

If HDD is:

DELAY_INPUT

then failure to produce the required commitment prevents that producer from creating a valid proposal.

The exact behavior MUST be defined in genesis.

⸻

29. AI Relationship

The AI Observer MAY analyze:

* unusual delay speed;
* repeated identical checkpoint patterns;
* suspicious hardware claims;
* verification failures;
* HDD and mathematical timing mismatch;
* producer dominance;
* energy anomalies.

AI MUST NOT declare a delay proof valid.

AI MUST NOT override deterministic verification.

⸻

30. Delay Engine Versioning

Every proof MUST include:

engine_id
engine_version
proof_version
parameter_version

Unknown versions MUST be rejected.

Engine upgrades affecting consensus require:

* protocol-version update;
* documented activation height;
* migration rules;
* compatibility tests;
* rollback or safe-mode planning.

⸻

31. Benchmark Harness

Q1 SHALL include a dedicated delay benchmark tool.

Suggested commands:

q1-delay benchmark
q1-delay generate
q1-delay verify
q1-delay compare
q1-delay profile
q1-delay export

Benchmark output SHOULD include:

engine_id
engine_version
difficulty
challenge_hash
hardware_profile
generation_time
verification_time
proof_size
output_size
cpu_time
memory_peak
energy_estimate
implementation_build
operating_system
success_or_failure

⸻

32. Energy Measurement

The Delay Engine MUST measure or estimate:

* idle energy baseline;
* generation energy;
* verification energy;
* energy per iteration;
* energy per successful block;
* duplicated candidate energy;
* HDD-related energy where enabled.

Q1 MUST NOT claim environmental superiority without comparative measurement.

⸻

33. Comparative Test Modes

The benchmark suite MUST compare:

Mock Delay
Sequential Hash Delay
Checkpointed Sequential Hash Delay
Formal VDF candidate
Delay plus HDD

Where possible, tests SHOULD compare:

* one candidate;
* three candidates;
* sequential activation;
* parallel candidate execution;
* low difficulty;
* target difficulty;
* high difficulty;
* ordinary laptop;
* desktop;
* server;
* heterogeneous nodes.

⸻

34. Deterministic Test Vectors

Every engine version MUST publish deterministic test vectors.

A test vector SHALL include:

context
encoded_challenge
difficulty
expected_output
expected_proof
verification_result

All supported implementations MUST pass the same vectors.

⸻

35. Sequential Hash Test Vector Example

Illustrative only:

chain_id = "q1-local-1"
block_height = 10
round_number = 0
candidate_index = 0
producer_id = "producer-A"
parent_block_hash = "..."
difficulty = 1000

The repository MUST contain exact canonical bytes and expected output generated from the final serialization rules.

Informal examples in this document are not normative test vectors.

⸻

36. Delay Engine State

A producer delay process MAY have states:

NOT_STARTED
CHALLENGE_DERIVED
RUNNING
CHECKPOINT_RECORDED
COMPLETED
SELF_VERIFIED
ATTACHED_TO_PROPOSAL
FAILED
CANCELLED
EXPIRED

The node MUST record the terminal reason.

⸻

37. Cancellation

Delay execution SHOULD support safe cancellation when:

* another block finalizes;
* producer window expires;
* round advances;
* node enters safe mode;
* process shuts down;
* engine detects invalid parameters.

Cancellation MUST NOT produce a valid proof.

⸻

38. Crash Recovery

The initial Sequential Hash Delay MAY restart from the beginning after a crash.

Checkpoint recovery MAY be supported later.

If checkpoint recovery is implemented:

* checkpoints MUST be bound to the exact challenge;
* corrupted checkpoints MUST be detected;
* checkpoints MUST NOT permit cross-round reuse;
* restored execution MUST produce the same final output.

⸻

39. Remote Execution

Q1 v0.1 MUST assume that producers may outsource computation.

The protocol cannot reliably prove physical location.

Therefore delay validity MUST depend on the proof, not on the producer’s claim about where it ran.

Future economic rules MAY study outsourcing effects.

⸻

40. Precomputation Attacks

The test suite MUST attempt:

* generating proofs before parent finalization;
* reusing a previous challenge;
* changing producer identity after generation;
* changing candidate index;
* changing round;
* changing difficulty;
* changing engine version;
* partial precomputation tables;
* checkpoint reuse;
* cached output substitution.

Every modified context MUST invalidate the proof.

⸻

41. Shortcut Attacks

The research team MUST attempt to find:

* algebraic shortcuts;
* batch-computation advantages;
* vectorized implementations;
* GPU acceleration;
* FPGA or ASIC acceleration;
* memory-time tradeoffs;
* checkpoint forgery;
* proof malleability;
* verification bypasses.

Any discovered shortcut MUST be documented in 13_HOW_TO_BREAK_Q1.md.

⸻

42. Delay Manipulation by Previous Block Producer

Because the challenge uses previous finalized data, the previous producer may have limited influence over some inputs.

The protocol MUST analyze whether a previous producer can:

* choose among multiple block contents;
* manipulate a delay output;
* withhold a block;
* search for a favorable next-round seed;
* alter optional HDD commitments;
* bias future producer selection.

Mitigation MAY require future randomness beacons or threshold randomness.

⸻

43. Verification Caching

Nodes MAY cache successful proof verification keyed by:

engine_id
engine_version
challenge_hash
difficulty
output_hash
proof_hash

Caching MUST NOT allow an object valid in one context to be reused in another.

⸻

44. Light-Client Verification

Q1 v0.1 does not require light clients to verify full delay proofs independently.

Future light-client options MAY include:

* compact VDF proof verification;
* finalized-header verification;
* committee certificate verification;
* trusted checkpoint mode for experiments.

No final design is approved here.

⸻

45. Error Handling

The Delay Engine MUST fail closed.

On:

* malformed proof;
* unsupported version;
* arithmetic overflow;
* excessive allocation;
* internal inconsistency;
* challenge mismatch;
* engine crash;

the object MUST be rejected or the producer process safely stopped.

The engine MUST NOT return “valid” on an internal error.

⸻

46. Logging

Delay logs MUST include:

* challenge hash;
* engine ID;
* version;
* difficulty;
* producer ID;
* block height;
* round;
* start event;
* checkpoint events where enabled;
* completion event;
* self-verification result;
* cancellation reason;
* error code.

Logs MUST NOT expose private signing keys.

⸻

47. Telemetry Events

Recommended events:

delay.challenge_created
delay.execution_started
delay.checkpoint_reached
delay.execution_completed
delay.self_verification_passed
delay.self_verification_failed
delay.verification_started
delay.verification_completed
delay.verification_failed
delay.cancelled
delay.window_expired
delay.engine_error
delay.hdd_input_received

⸻

48. Security Invariants

The implementation MUST test:

Invariant 1 — Context binding

A proof is valid only for its exact context.

⸻

Invariant 2 — Deterministic output

The same challenge and parameters produce the same output.

⸻

Invariant 3 — Verification agreement

All honest validators return the same verification result.

⸻

Invariant 4 — No replay

A proof from another height, round, producer, candidate, chain, or engine version fails.

⸻

Invariant 5 — Bounded parameters

No block can demand unbounded validator work.

⸻

Invariant 6 — Optional HDD

Base delay verification remains possible with HDD disabled unless the network explicitly activates a composite mode.

⸻

Invariant 7 — AI independence

AI availability has no effect on proof validity.

⸻

Invariant 8 — Failure closure

Internal errors never produce successful verification.

⸻

49. Required Tests

The automated test suite MUST include:

1. valid sequential hash proof;
2. wrong final output;
3. wrong challenge;
4. wrong producer;
5. wrong candidate index;
6. wrong height;
7. wrong round;
8. wrong chain;
9. wrong parent hash;
10. wrong difficulty;
11. unsupported engine;
12. unsupported engine version;
13. malformed proof;
14. proof truncation;
15. proof extension;
16. excessive proof size;
17. zero difficulty;
18. excessive difficulty;
19. arithmetic overflow;
20. deterministic repeated execution;
21. concurrent independent executions;
22. cancellation;
23. producer-window expiration;
24. node restart;
25. checkpoint corruption;
26. checkpoint replay;
27. sampled verification experiment;
28. full recomputation verification;
29. HDD disabled;
30. HDD telemetry-only mode;
31. HDD delay-input mode;
32. fake HDD commitment;
33. AI observer offline;
34. telemetry offline;
35. GPU benchmark;
36. multi-core benchmark;
37. low-end CPU benchmark;
38. high-end CPU benchmark;
39. duplicated candidate energy test;
40. formal VDF integration test when available.

⸻

50. Minimum Prototype Acceptance

The Delay Engine prototype is complete when:

1. a canonical challenge is derived identically by all honest nodes;
2. an eligible producer executes sequential delay;
3. the output is deterministic;
4. the proof is attached to a block proposal;
5. validators independently verify the proof;
6. altered context invalidates the proof;
7. replay fails;
8. unsupported engine versions fail;
9. generation and verification metrics are recorded;
10. cancellation works;
11. producer fallback cancels stale delay work;
12. HDD can be disabled;
13. AI can be disabled;
14. malicious proofs do not crash validators;
15. test vectors are published.

⸻

51. Research Acceptance for Formal VDF Migration

A formal VDF candidate may replace the prototype delay only after:

* public construction review;
* deterministic implementation;
* independent test vectors;
* measurable generation-verification asymmetry;
* acceptable proof size;
* cross-platform consistency;
* adversarial testing;
* documented assumptions;
* no unresolved critical implementation failures;
* explicit protocol activation plan.

⸻

52. Known Limitations

Sequential Hash Delay v1 does not provide:

* compact proof;
* cheap full verification;
* protection against specialized hardware advantage;
* formal proof of sequentiality;
* public-mainnet-grade security;
* perfect elapsed-time proof.

It is a prototype instrument.

Its purpose is to make the consensus pipeline executable and measurable.

⸻

53. Open Decisions

The following remain unresolved:

1. exact hash algorithm;
2. exact sequential-hash encoding;
3. exact initial iteration count;
4. exact target delay duration;
5. exact adjustment interval;
6. exact adjustment bounds;
7. whether checkpoints are included in milestone one;
8. exact checkpoint interval;
9. whether sampled verification is useful;
10. which formal VDF construction will be evaluated first;
11. whether VDF setup is acceptable;
12. whether delay execution and block building run concurrently;
13. whether fallback candidates perform preparation;
14. exact HDD composite construction;
15. whether HDD commitments influence mathematical input;
16. exact proof size limits;
17. exact verification resource limits;
18. whether formal VDF verification is required on light clients;
19. how hardware dominance will be measured;
20. how future randomness prevents grinding.

All decisions MUST be recorded in OPEN_DECISIONS.md.

⸻

54. Codex Implementation Rules

Codex MUST:

1. implement the Delay Engine behind a stable interface;
2. provide mock and sequential-hash implementations;
3. label the simplified implementation non-production;
4. bind challenges to chain, height, round, producer, candidate, parent, engine, and difficulty;
5. use canonical serialization;
6. use checked integer arithmetic;
7. implement deterministic test vectors;
8. implement proof-size and resource limits;
9. fail closed on internal errors;
10. support cancellation;
11. emit structured telemetry;
12. isolate HDD logic behind an adapter;
13. keep AI outside proof verification;
14. make difficulty configurable through consensus parameters;
15. separate local timing telemetry from validity;
16. include benchmark tooling;
17. document every optimization;
18. preserve reproducibility across supported platforms.

Codex MUST NOT:

* use sleep() as proof of delay;
* trust a producer-reported duration;
* treat HDD telemetry as mathematical proof;
* accept a proof without reconstructing the challenge;
* silently convert unsupported versions;
* use floating-point arithmetic in consensus difficulty;
* create custom cryptographic claims without documentation;
* make the Delay Engine responsible for finality;
* permit unlimited proof sizes;
* allow a producer to choose its own challenge or difficulty.

⸻

55. Final Delay Principle

Q1 Delay Engine MUST preserve this principle:

Time itself cannot be submitted as data.
A device’s claim that it waited cannot be trusted.
A producer must instead present the result of a challenge-bound sequence whose completion can be independently checked.
The network does not reward heat, noise, motion, or electricity merely because they occurred.
It rewards only work that satisfies public rules, survives verification, and contributes to a valid finalized block.

Q1 v0.1 succeeds at the delay layer when the network can distinguish:

* waiting from verified work;
* local timing from consensus evidence;
* physical telemetry from mathematical proof;
* experimental machinery from protocol authority;
* and a reusable output from a challenge-bound result.
