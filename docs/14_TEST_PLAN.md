Q1 Test Plan

14_TEST_PLAN.md

Project: Q1 Experimental Distributed Ledger
Protocol Version: 0.1
Document Version: 0.1.0
Status: Draft for Engineering, Security, and Research Review
Classification: Experimental — Controlled Test Environments Only

⸻

1. Purpose

This document defines the complete testing strategy for Q1 v0.1.

It converts the requirements, architecture, consensus model, delay engine, HDD module, wallet, tokenomics, AI Observer, security model, and adversarial plan into an executable and measurable test program.

The Q1 Test Plan SHALL determine:

* whether individual components behave correctly;
* whether independent nodes reach identical results;
* whether consensus preserves safety;
* whether the network preserves liveness under expected conditions;
* whether attacks are detected, blocked, or contained;
* whether HDD contributes measurable value;
* whether AI provides useful observation without becoming authoritative;
* whether the economic model remains internally consistent;
* whether nodes recover from failures;
* whether the prototype is reproducible;
* and whether Q1 deserves advancement to the next development phase.

Passing tests does not prove absolute security.

Failing tests does not automatically terminate the project.

Every test result is engineering evidence.

The pre-M1 corpus defined under `docs/protocol/` is a prerequisite to M1. It
must cover canonical and rejected CBOR, domain framing, official and Q1
Ed25519 vectors, address validity and wrong-network rejection, and
cross-language equality where a TypeScript consumer exists.

Requirement `Q1-LTX-024` Amount tests SHALL cover the approved nine boundary/canonical
vectors; checked arithmetic; conversions; and rejection of short, long,
integer, indefinite, tagged, negative, floating-point, text, array, map, and
trailing-byte representations. Cross-language research artifacts SHALL agree
on acceptance and rejection.

⸻

2. Testing Philosophy

Q1 SHALL follow these testing principles.

Q1-TST-001 — Test behavior, not intention

A specification claim is not accepted merely because the implementation appears to follow it.

The relevant behavior MUST be observed and measured.

⸻

Q1-TST-002 — Determinism before scale

Q1 MUST first prove that several honest nodes derive identical results before attempting large-scale performance tests.

⸻

Q1-TST-003 — Safety before liveness

A network that stops safely is preferable to a network that continues with conflicting finalized states.

⸻

Q1-TST-004 — Failure is valid output

A failed experiment MUST be recorded without concealment.

It MAY result in:

* code correction;
* parameter correction;
* specification amendment;
* architectural redesign;
* module restriction;
* module removal.

⸻

Q1-TST-005 — Reproducibility

Every material test SHOULD be reproducible using:

* a fixed software commit;
* a known genesis file;
* a known configuration;
* a deterministic test seed;
* a documented environment;
* preserved raw results.

⸻

Q1-TST-006 — Isolation

Dangerous, destructive, adversarial, or high-load tests MUST run only in authorized and isolated environments.

⸻

Q1-TST-007 — Requirement traceability

Every mandatory specification requirement SHOULD map to one or more:

* unit tests;
* integration tests;
* system tests;
* adversarial tests;
* manual review items;
* or explicitly deferred items.

⸻

3. Test Objectives

The Q1 v0.1 test program SHALL answer the following questions.

3.1 Ledger

Can Q1 maintain exact balances, nonces, roots, fees, rewards, and supply across independent nodes?

⸻

3.2 Transactions

Can Q1 accept authorized transfers and reject invalid, duplicate, replayed, expired, or conflicting transactions?

⸻

3.3 Consensus

Can Q1 select producers and committees deterministically and finalize exactly one valid block per height under its stated assumptions?

⸻

3.4 Delay

Can Q1 bind sequential work to a fresh challenge and verify that work consistently?

⸻

3.5 HDD

Does HDD participation add measurable utility, cost, diversity, or security beyond mathematical delay?

⸻

3.6 Networking

Can nodes discover peers, exchange data, resist malformed traffic, synchronize, survive partitions, and recover?

⸻

3.7 Wallet

Can a user securely create keys, sign locally, understand fees, submit a transaction, track finality, back up, and restore?

⸻

3.8 Tokenomics

Can Q1 account for every unit and maintain a coherent incentive model under ordinary and adversarial behavior?

⸻

3.9 AI Observer

Can the observer find meaningful patterns without modifying consensus or generating unacceptable noise?

⸻

3.10 Security

Can Q1 reject known attack patterns, fail safely, preserve evidence, and disclose unresolved weaknesses?

⸻

4. Test Levels

Q1 SHALL use the following test levels.

L0 — Static and specification checks
L1 — Unit tests
L2 — Component integration tests
L3 — Single-node system tests
L4 — Local multi-node tests
L5 — Private distributed testnet tests
L6 — Adversarial and red-team tests
L7 — Performance and endurance tests
L8 — Public experimental testnet tests

A higher level MUST NOT begin until its required entry conditions are satisfied.

⸻

5. Level 0 — Static and Specification Checks

Level 0 includes:

* specification consistency review;
* requirement identifier validation;
* schema validation;
* linting;
* type checking;
* dependency scanning;
* secret scanning;
* static security analysis;
* formatting;
* documentation-link validation;
* configuration-schema validation;
* genesis-schema validation.

Level 0 MUST detect contradictions such as:

* incompatible block interval and producer windows;
* reward weights not summing correctly;
* unsupported engine versions;
* conflicting field definitions;
* missing consensus parameters.

⸻

6. Level 1 — Unit Tests

Unit tests SHALL isolate one deterministic behavior.

Targets include:

* canonical serialization;
* hashes;
* signatures;
* address derivation;
* transaction validation;
* nonce rules;
* fee calculation;
* reward allocation;
* state transitions;
* Merkle roots;
* producer selection;
* committee selection;
* quorum calculation;
* delay challenge derivation;
* delay proof verification;
* HDD challenge selection;
* wallet amount parsing;
* keystore encryption;
* peer-score local policy;
* AI rule detectors.

Unit tests SHOULD execute quickly and without external infrastructure.

⸻

Merkle sequence conformance requirements

DEC-Q1-027 Session 4 records future conformance coverage for every activated
Merkle profile. It does not authorize creation of vectors or implementation.

Independent Rust and at least two other language implementations must later
agree byte-for-byte for:

* zero through five items;
* the same canonical item at different indexes;
* mutated index, profile ID, item length, and item bytes;
* changed item order;
* every applicable odd-node promotion case;
* leaf hashes, internal hashes, typed empty roots, and final roots.

Tests must also show that Merkle processing neither sorts nor normalizes input,
that profile root types are not interchangeable, and that owning schemas bind
the corresponding item count.

Future Session 5B schema conformance must cover, after separate implementation
authorization:

* BlockBodyV1 exact length, ordered transfers, zero-transfer body, and
  unknown/missing/trailing-field rejection;
* TransactionCount equality with transfer-array length and TransactionRoot
  leaf count, including `u32` overflow rejection;
* ParticipantSetV1 exact length, reference-height binding, active-record
  filtering, non-empty requirement, raw ParticipantId ordering, and duplicate
  rejection;
* ParticipantCount equality with participant-array length and ParticipantRoot
  leaf count;
* producer membership, role-key resolution, activation interval, and signature
  verification split between structural and historical/state validation;
* ReceiptRoot absence and rejection of null, empty, opaque, or trailing
  substitutes;
* rejection of arbitrary StateRoot bytes before the state-model gate;
* semantic non-interchangeability of all root and count types.

This records future coverage only and creates no tests or vectors.

Future Session 5C schema conformance, only after separate implementation and
domain-registration authorization, must cover:

* the exact eleven-field BlockHeaderBodyV1 order and rejection of missing,
  unknown, optional, or trailing fields;
* protocol_version zero/unknown/inactive rejection and shortest encoding;
* RoundNumber `u32` bounds, zero validity, and non-interchangeability with
  Slot or the legacy composite Round;
* complete signed-envelope payload and BlockId recomputation;
* rejection of every excluded Header field or placeholder;
* DelayEvidenceV1 exact five-field shape, definite byte strings, and
  engine-profile resource limits;
* NONE exact bytes for `[1, 0, 0, h'', h'']` plus network-policy acceptance
  and rejection;
* DelayEvidenceHash domain separation from GenericHash and DELAY_OUTPUT;
* independent Rust, Node, and Python golden and rejection vectors.

No such tests or vectors are authorized or created by Session 5C.

⸻

7. Level 2 — Component Integration Tests

Component integration tests SHALL verify interfaces between modules.

Examples:

* wallet core and protocol serializer;
* transaction gateway and mempool;
* mempool and block builder;
* block processor and state engine;
* consensus engine and delay adapter;
* node runtime and P2P transport;
* HDD plugin and delay engine;
* finalized blocks and indexer;
* telemetry collector and AI Observer;
* tokenomics and ledger accounting;
* keystore and offline signer.

Mocks MAY be used, but critical integrations MUST later be tested with real implementations.

⸻

8. Level 3 — Single-Node System Tests

Single-node tests SHALL verify:

* startup;
* genesis loading;
* local wallet transactions;
* deterministic block creation;
* state persistence;
* restart;
* explorer indexing;
* API operation;
* telemetry;
* safe-mode handling.

Single-node mode does not prove distributed consensus.

It is used to confirm complete local execution paths.

⸻

9. Level 4 — Local Multi-Node Tests

The minimum local network SHALL include:

4 independent node processes
4 node identities
separate ports
separate data directories
separate databases
configurable node roles

Recommended initial topology:

Node A — producer + full node
Node B — validator + full node
Node C — validator + full node
Node D — validator + observer

Later tests SHOULD use five or seven validators where quorum behavior requires it.

⸻

10. Level 5 — Private Distributed Testnet

The minimum recommended private distributed environment SHALL contain:

7 nodes
3 or more physical hosts
2 or more network paths or providers where possible
1 archive node
1 observer-only node
1 node behind NAT
1 intentionally slow node
1 malicious test node

This level SHALL measure real:

* latency;
* propagation;
* host variance;
* disk behavior;
* clock drift;
* restart behavior;
* energy use.

⸻

11. Level 6 — Adversarial and Red-Team Tests

Level 6 SHALL implement the scenarios in:

13_HOW_TO_BREAK_Q1.md

Red-team tests MUST define:

* attacker capabilities;
* attack objective;
* expected secure behavior;
* success condition;
* stop condition;
* evidence collection;
* severity;
* required regression test.

⸻

12. Level 7 — Performance and Endurance

This level SHALL measure:

* throughput;
* latency;
* resource use;
* energy;
* storage growth;
* long-run stability;
* memory leaks;
* queue growth;
* database behavior;
* reward and fee behavior;
* network recovery.

Endurance tests SHOULD run for:

1 hour
6 hours
24 hours
7 days

Longer tests MAY be added after stability improves.

⸻

13. Level 8 — Public Experimental Testnet

Public testnet testing is outside the first implementation milestone.

It MAY begin only after the release gates in this document are satisfied.

Public testnet SHALL remain explicitly experimental.

No real-value promise may be attached to it.

⸻

14. Test Categories

The complete suite SHALL contain:

Functional
Deterministic
Cryptographic
Ledger
Transaction
Consensus
Delay
HDD
Networking
Synchronization
Wallet
Economic
AI Observer
Security
Recovery
Performance
Energy
Compatibility
Upgrade
Usability
Endurance

⸻

15. Test Case Structure

Every formal test SHOULD use:

TestCase {
    test_id
    title
    category
    requirement_ids[]
    specification_sources[]
    objective
    preconditions
    topology
    configuration
    input_data
    execution_steps
    expected_result
    acceptance_condition
    evidence_to_collect
    cleanup
    automation_status
    severity_if_failed
}

⸻

16. Test Identifier Convention

Recommended identifiers:

Q1-TST-UNIT-XXXX
Q1-TST-LED-XXXX
Q1-TST-TX-XXXX
Q1-TST-CON-XXXX
Q1-TST-DLY-XXXX
Q1-TST-HDD-XXXX
Q1-TST-NET-XXXX
Q1-TST-WAL-XXXX
Q1-TST-ECO-XXXX
Q1-TST-AI-XXXX
Q1-TST-SEC-XXXX
Q1-TST-REC-XXXX
Q1-TST-PERF-XXXX

Identifiers MUST remain stable after publication.

⸻

17. Test Environments

Q1 SHALL define standard environments.

Environment A — Developer

one computer
mock delay allowed
small dataset
temporary databases

⸻

Environment B — Deterministic Localnet

4–7 processes
fixed test seed
fixed genesis
simulated network conditions
mock or sequential delay

⸻

Environment C — Physical HDD Lab

physical HDD
SSD comparison
RAM-disk comparison
energy instrumentation where available
dedicated test directory

⸻

Environment D — Private Distributed Testnet

multiple hosts
real network latency
mixed device classes
persistent data

⸻

Environment E — Adversarial Sandbox

isolated network
malicious nodes
fault injector
disposable data
no real credentials

⸻

18. Standard Test Data

The repository SHOULD contain:

* deterministic key pairs for test-only use;
* test addresses;
* sample genesis files;
* valid transactions;
* invalid transactions;
* valid blocks;
* invalid blocks;
* valid attestations;
* conflicting attestations;
* valid certificates;
* invalid certificates;
* delay test vectors;
* HDD test datasets;
* wallet backups;
* malformed message corpus;
* AI telemetry traces.

Test private keys MUST never be used outside test networks.

⸻

19. Deterministic Seeds

Tests involving selection, topology, attack timing, or random inputs SHOULD support a fixed seed.

A failed test report MUST record the seed.

Example:

test_seed = 20260723

A reproduced test with the same seed and configuration SHOULD generate the same logical protocol behavior.

⸻

20. Test Orchestrator

The test orchestrator SHALL support commands conceptually similar to:

q1-test create
q1-test run
q1-test suite
q1-test status
q1-test stop
q1-test collect
q1-test compare
q1-test report

For multi-node environments:

q1-testnet up
q1-testnet down
q1-testnet restart
q1-testnet partition
q1-testnet heal
q1-testnet delay
q1-testnet loss
q1-testnet attack
q1-testnet load

⸻

21. Requirement Traceability Matrix

The repository MUST maintain:

docs/22_REQUIREMENTS_TRACEABILITY_SKELETON.md

The file remains a skeleton until its stated authority gate passes.

Example:

Requirement: Q1-LTX-007
Description: Account nonce increments after finalized transfer
Tests:
- Q1-TST-TX-0012
- Q1-TST-LED-0008
- Q1-TST-CON-0041
Status: Covered

Possible statuses:

UNCOVERED
PARTIALLY_COVERED
COVERED
DEFERRED
NOT_APPLICABLE

No mandatory requirement SHOULD remain uncovered without explicit approval.

⸻

22. Ledger Test Suite

The ledger suite MUST test:

* exact equality between declared genesis supply and the sum of all explicit
  genesis allocations;
* rejection of any implicit or unassigned genesis remainder;

1. genesis state creation;
2. correct genesis state root;
3. account creation;
4. valid balance transfer;
5. nonce increment;
6. recipient creation;
7. zero balance;
8. zero amount rejection;
9. negative amount rejection;
10. insufficient balance;
11. arithmetic overflow;
12. arithmetic underflow;
13. deterministic state execution;
14. atomic failure;
15. state-root mismatch;
16. transaction-root mismatch;
17. receipt-root mismatch;
18. unauthorized issuance;
19. incorrect burn;
20. incorrect treasury balance;
21. snapshot reconstruction;
22. database restart;
23. index rebuild;
24. total-supply conservation.

⸻

23. Transaction Test Suite

The transaction suite MUST test:

* canonical encoding;
* deterministic transaction ID;
* valid signature;
* invalid signature;
* altered signed fields;
* malformed public key;
* wrong address derivation;
* wrong chain ID;
* unsupported version;
* self-transfer rejection;
* stale nonce;
* future nonce;
* excessive nonce gap;
* duplicate transaction;
* transaction replacement;
* underpriced replacement;
* expiration;
* not-yet-valid transaction;
* fee limit;
* maximum size;
* replay.

Property tests SHOULD generate large sets of random valid and invalid transactions.

⸻

24. Consensus Test Suite

The consensus suite MUST test:

1. identical candidate selection;
2. identical committee selection;
3. primary producer success;
4. primary producer timeout;
5. fallback activation;
6. all candidates unavailable;
7. round advancement;
8. valid attestation;
9. invalid attestation;
10. duplicate attestation;
11. validator double vote;
12. producer equivocation;
13. quorum boundary;
14. insufficient quorum;
15. valid certificate;
16. invalid certificate;
17. wrong committee;
18. wrong round;
19. early fallback;
20. late proposal;
21. conflicting proposals;
22. one validator offline;
23. one-third Byzantine boundary;
24. more-than-one-third Byzantine behavior;
25. conflicting finality;
26. safe-mode entry;
27. restart after signing;
28. stale node participation;
29. participant-registry mismatch;
30. finality persistence.

⸻

25. Consensus Model Checking

The project SHOULD use model-based or state-machine testing for:

* round transitions;
* fallback transitions;
* quorum calculation;
* finality;
* timeout certificates;
* restart behavior.

Where practical, a simplified formal model MAY be created to explore:

* safety invariants;
* liveness assumptions;
* conflicting messages;
* reordered delivery.

The model is not a substitute for implementation testing.

⸻

26. Delay Engine Test Suite

The delay suite MUST test:

* deterministic challenge;
* context binding;
* producer binding;
* height binding;
* round binding;
* chain binding;
* difficulty binding;
* engine-version binding;
* valid sequential proof;
* wrong output;
* malformed proof;
* proof replay;
* proof-size limit;
* parameter bounds;
* cancellation;
* timeout;
* self-verification;
* verification-resource limits;
* checkpoint corruption;
* hardware benchmark variance;
* energy measurement;
* formal VDF adapter readiness.

⸻

27. Delay Benchmark Matrix

The benchmark matrix SHOULD include:

Low-end laptop CPU
Modern laptop CPU
Desktop CPU
Server CPU
GPU attempt where applicable

For each:

generation time
verification time
energy estimate
proof size
memory peak
temperature or throttling indicators

The key comparison SHALL be generation cost versus verification cost.

⸻

28. HDD Test Suite

The HDD suite MUST test:

* device discovery;
* device qualification;
* deterministic dataset;
* dataset corruption;
* challenge-bound access;
* commitment generation;
* replay rejection;
* cancellation;
* device disconnection;
* insufficient space;
* cache effects;
* HDD versus SSD;
* HDD versus RAM disk;
* virtual disk;
* remote storage;
* duplicate dataset;
* multiple disks;
* RAID;
* path traversal;
* symlink abuse;
* write limits;
* wear;
* energy;
* consensus operation with HDD disabled.

⸻

29. HDD Experimental Matrix

Each HDD experiment SHOULD record:

device type
capacity
interface
reported RPM
filesystem
dataset size
chunk size
sample count
cache mode
workload type
duration
latency distribution
throughput
energy estimate
errors

Comparisons MUST use the same workload and challenge where technically appropriate.

⸻

30. Networking Test Suite

The network suite MUST test:

* handshake;
* identity verification;
* genesis mismatch;
* unsupported version;
* peer discovery;
* bootstrap independence;
* peer exchange;
* transaction relay;
* block relay;
* attestation relay;
* certificate relay;
* duplicate handling;
* rate limiting;
* message-size limits;
* decompression limits;
* request timeout;
* retry;
* peer ban;
* prefix diversity;
* peer rotation;
* slow peer;
* connection churn;
* eclipse;
* partition;
* recovery;
* consensus-priority protection.

⸻

31. Synchronization Test Suite

The synchronization suite MUST test:

1. sync from genesis;
2. header-first sync;
3. block-range sync;
4. snapshot-assisted sync;
5. invalid snapshot;
6. corrupted chunk;
7. false highest height;
8. alternate history;
9. incomplete history;
10. multiple sync peers;
11. interrupted sync;
12. restart during sync;
13. stale participant set;
14. mempool reevaluation;
15. recovery from prolonged disconnection.

All synchronized nodes MUST reach the same finalized state root.

⸻

32. Wallet Test Suite

The wallet suite MUST test:

* wallet creation;
* secure random failure;
* key generation;
* encrypted keystore;
* lock and unlock;
* wrong password;
* address derivation;
* wrong-network address;
* balance query;
* nonce query;
* fee estimation;
* exact amount parsing;
* excessive decimals;
* transaction preview;
* signing;
* local verification;
* submission;
* status tracking;
* false finality;
* backup;
* restore;
* duplicate restore;
* offline signing;
* package tampering;
* watch-only mode;
* crash during keystore write;
* secret leakage.

⸻

33. Tokenomics Test Suite

The economic suite MUST test:

* genesis allocation totals;
* fixed issuance;
* alternative issuance configuration;
* fee calculation;
* congestion steps;
* fee limit;
* non-negative fee;
* fee distribution;
* fee burn;
* treasury allocation;
* producer reward;
* delay reward;
* validator reward;
* reward remainder;
* reward maturity;
* multi-role reward;
* duplicate reward prevention;
* invalid block reward prevention;
* supply conservation;
* artificial volume;
* transaction splitting;
* Sybil reward farming;
* reward concentration.

⸻

34. AI Observer Test Suite

The AI suite MUST test:

* telemetry ingestion;
* malformed telemetry;
* evidence classes;
* objective equivocation alerts;
* finality conflict alerts;
* concentration metrics;
* partition detection;
* eclipse suspicion;
* spam detection;
* delay anomaly;
* HDD anomaly;
* economic anomaly;
* alert deduplication;
* false-positive workflow;
* false-negative evaluation;
* model outage;
* model replacement;
* prompt injection;
* privacy filter;
* secret filter;
* observer shutdown without consensus impact.

⸻

35. Security Test Suite

The security suite SHALL integrate the scenarios from:

12_SECURITY_MODEL.md
13_HOW_TO_BREAK_Q1.md

It MUST include:

* cryptographic misuse;
* malformed inputs;
* unauthorized issuance;
* double spending;
* equivocation;
* quorum forgery;
* finality conflict;
* rollback;
* key leakage;
* API bypass;
* path traversal;
* dependency tampering;
* genesis substitution;
* protocol downgrade;
* economic extraction;
* AI poisoning.

⸻

36. Fuzz Testing

Fuzzing targets SHALL include:

transaction decoder
block decoder
network frame decoder
attestation decoder
certificate decoder
delay proof decoder
HDD evidence decoder
snapshot decoder
wallet backup decoder
public API inputs

Fuzzing MUST verify:

* no crashes;
* no uncontrolled memory use;
* no silent acceptance;
* stable rejection behavior;
* no secret exposure.

⸻

37. Property-Based Testing

Q1 SHOULD use property-based tests for:

* supply conservation;
* fee conservation;
* reward conservation;
* deterministic serialization;
* deterministic state transition;
* nonce monotonicity;
* replay rejection;
* quorum threshold;
* one final block per height;
* context binding;
* atomicity.

Generated failing examples MUST be preserved as regression cases.

⸻

38. Fault Injection

The test orchestrator SHALL support:

process termination
CPU throttling
memory pressure
disk full
disk corruption
network delay
packet loss
packet duplication
message reordering
clock skew
DNS failure
bootstrap outage
AI outage
telemetry outage
HDD disconnection
database lock

⸻

39. Recovery Test Suite

Recovery tests MUST include:

1. normal node restart;
2. forced process termination;
3. crash during transaction handling;
4. crash during block proposal;
5. crash during attestation;
6. crash during finalization;
7. crash during database commit;
8. restart after validator signature;
9. disk-full recovery;
10. state corruption detection;
11. index rebuild;
12. resynchronization;
13. HDD dataset recovery;
14. wallet keystore recovery;
15. observer restart;
16. partition healing;
17. known-checkpoint recovery.

⸻

40. Safe-Mode Tests

Safe-mode tests MUST verify activation on:

* conflicting finalization certificates;
* supply mismatch;
* finalized-state mismatch;
* database corruption;
* cryptographic provider failure;
* unsupported mandatory protocol version;
* impossible participant-set mismatch.

While in safe mode, the node MUST:

* stop producing;
* stop attesting;
* stop finalizing;
* preserve evidence;
* allow safe diagnostics.

⸻

41. Performance Metrics

The performance suite SHALL measure:

transaction validation latency
transaction propagation latency
block construction latency
delay generation time
delay verification time
block validation time
attestation latency
time to quorum
time to finality
sync throughput
API latency
CPU use
memory use
disk use
network bandwidth

⸻

42. Initial Performance Targets

Initial research targets:

ordinary transaction validation:
≤ 100 ms under normal local conditions
block propagation:
majority of private-testnet nodes within 5 seconds
transaction load:
10 transactions per second stable baseline
stress load:
100 transactions per second for bounded periods
mempool:
10,000 pending transactions
simulated peers:
100 connections or peer actors
delay proof verification:
substantially faster than generation where supported

Targets are not production guarantees.

⸻

43. Performance Profiles

Tests SHOULD define:

BASELINE
NORMAL
STRESS
SATURATION
RECOVERY

Baseline

Minimal traffic and stable nodes.

Normal

Expected private-testnet traffic.

Stress

Heavy but bounded load.

Saturation

Load intentionally exceeds capacity.

Recovery

Load returns to normal and queues must drain safely.

⸻

44. Energy Testing

Energy tests SHALL measure or estimate:

* idle node energy;
* transaction validation energy;
* delay generation energy;
* delay verification energy;
* HDD workload energy;
* synchronization energy;
* rejected-work energy;
* duplicate-candidate energy;
* energy per finalized block;
* energy per finalized transaction.

Every report MUST identify the measurement method.

⸻

45. Energy Comparison Configurations

Required comparisons:

delay engine only
delay engine + HDD telemetry
one producer candidate
three producer candidates
sequential candidate activation
parallel candidate execution
low transaction load
high transaction load

Q1 MUST NOT claim energy superiority without measured comparison.

⸻

46. Storage Growth Testing

The project SHALL measure:

* ledger growth per block;
* state growth;
* receipt growth;
* evidence growth;
* telemetry growth;
* snapshot size;
* HDD dataset size;
* indexer growth.

Projections SHOULD be calculated for:

1 day
1 month
1 year
10 years

These projections must state their assumptions.

⸻

47. Endurance Tests

A long-running network test MUST monitor:

* finalized-state consistency;
* memory growth;
* database growth;
* peer churn;
* missed rounds;
* reward accounting;
* fee accounting;
* delay performance;
* HDD errors;
* AI alert volume;
* resource leaks.

A seven-day run SHOULD become a private-testnet release gate.

⸻

48. Compatibility Tests

Q1 SHOULD test across:

* Linux;
* macOS;
* Windows where supported;
* x86-64;
* ARM64;
* different filesystem types;
* different database backends where applicable.

Consensus outputs MUST remain identical.

⸻

49. Cross-Implementation Testing

If Q1 later has more than one client implementation, all clients MUST pass common:

* serialization vectors;
* transaction vectors;
* block vectors;
* consensus vectors;
* delay vectors;
* economic vectors.

No public network should depend on one implementation indefinitely without acknowledging implementation monoculture risk.

⸻

50. Upgrade Tests

Protocol upgrade tests MUST include:

* compatible software upgrade;
* incompatible node rejection;
* activation-height behavior;
* mixed-version network;
* storage migration;
* rollback attempt;
* unknown object version;
* wallet-format migration;
* model-version migration.

⸻

51. Usability Testing

The project SHOULD test whether a new user can:

* install Q1;
* create a wallet;
* back it up;
* obtain test units;
* send a transaction;
* understand the fee;
* distinguish pending from finalized;
* restore the wallet;
* run a node.

Usability failures are important because misunderstanding can become a security failure.

⸻

52. Test Evidence

Every automated test run SHOULD produce:

run_id
suite
software_commit
build_hash
genesis_hash
configuration_hash
test_seed
environment
start_time
end_time
pass_count
fail_count
skip_count
raw_logs
metrics
artifacts
report

⸻

53. Test Result Status

Each test SHALL end with:

PASS
FAIL
SKIPPED
BLOCKED
INCONCLUSIVE
EXPECTED_FAILURE

EXPECTED_FAILURE may be used only for explicitly documented unresolved research limitations.

It MUST NOT be used to hide regressions.

⸻

54. Severity of Test Failure

Test failures SHALL be classified:

CRITICAL
HIGH
MEDIUM
LOW
INFORMATIONAL

Any failure involving:

* unauthorized issuance;
* conflicting honest finality;
* secret extraction;
* deterministic state divergence;
* accepted forged certificate

is critical.

⸻

55. Release Blocking Rules

A build MUST NOT advance if:

* any critical test fails;
* any consensus safety invariant fails;
* supply accounting diverges;
* wallet secrets appear in logs;
* malformed traffic crashes multiple nodes;
* conflicting finality does not trigger safe mode;
* regression tests for known critical defects fail.

⸻

56. Flaky Tests

A flaky test is one that passes and fails without an explained environmental cause.

Flaky consensus or ledger tests MUST be treated as serious defects.

The project MUST:

* record the seed;
* preserve logs;
* reproduce;
* isolate nondeterminism;
* avoid simply rerunning until green.

⸻

57. Test Coverage

The project SHOULD measure:

* line coverage;
* branch coverage;
* requirement coverage;
* protocol-rule coverage;
* attack-scenario coverage.

Code coverage alone does not prove behavioral coverage.

Consensus-critical modules SHOULD receive the highest coverage target.

⸻

58. Recommended Coverage Targets

Initial targets:

Protocol core: ≥ 90% line and strong branch coverage
Ledger and transaction rules: ≥ 90%
Consensus state machine: ≥ 90%
Cryptographic adapters: ≥ 85%
Wallet core: ≥ 85%
Networking: ≥ 75%
HDD experimental module: ≥ 75%
AI Observer: measured by detector evaluation, not only line coverage

Targets may be revised with justification.

⸻

59. Continuous Integration

Every code change SHOULD run:

formatting
linting
type checks
unit tests
core property tests
secret scanning
dependency checks
selected integration tests

Changes to consensus-critical code MUST additionally run:

* multi-node deterministic tests;
* transaction vectors;
* block vectors;
* quorum tests;
* supply-conservation tests.

⸻

60. Nightly Test Suite

A nightly suite SHOULD run:

* full integration tests;
* multi-node tests;
* fuzzing batches;
* malicious-node tests;
* performance smoke tests;
* wallet backup and restore;
* AI evaluation;
* HDD simulation tests.

Physical HDD tests MAY run on dedicated hardware separately.

⸻

61. Weekly Test Suite

A weekly suite SHOULD include:

* long-duration multi-node run;
* network partitions;
* load tests;
* fault injection;
* dependency audit;
* storage growth report;
* economic simulation batch;
* model drift checks;
* physical HDD benchmarks where available.

⸻

62. Milestone Test Gates

Milestone M0 — Specification readiness

Required:

* requirements reviewed;
* architecture mapped;
* test identifiers defined;
* open decisions recorded.

⸻

Milestone M1 — Protocol core

Required:

* transaction and ledger unit tests;
* deterministic serialization;
* supply conservation;
* valid state roots.

⸻

Milestone M2 — Single-node prototype

Required:

* wallet transaction;
* local block;
* persistence;
* explorer;
* telemetry.

⸻

Milestone M3 — Local consensus

Required:

* four nodes;
* producer selection;
* delay execution;
* committee attestation;
* finalization;
* fallback.

⸻

Milestone M4 — Adversarial localnet

Required:

* malicious node;
* equivocation;
* invalid proofs;
* partition;
* safe mode.

⸻

Milestone M5 — Private distributed testnet

Required:

* seven nodes;
* multiple hosts;
* 1,000 finalized blocks;
* 10,000 transfers;
* exact state agreement.

⸻

Milestone M6 — Endurance and security

Required:

* seven-day run;
* red-team suite;
* no unresolved critical defects;
* energy and concentration report.

⸻

Milestone M7 — Public testnet candidate

Required:

* independent review;
* hardened defaults;
* disclosure process;
* signed builds;
* published limitations.

⸻

63. Minimum Viable Prototype Test Gate

The first Q1 prototype passes when:

1. four nodes start independently;
2. nodes load the same genesis;
3. wallets create signed transactions;
4. transactions propagate;
5. invalid transactions are rejected;
6. producer selection is identical;
7. committee selection is identical;
8. delay work completes;
9. block proposal is valid;
10. validators attest;
11. threshold is reached;
12. block finalizes;
13. all honest nodes have identical state;
14. replay fails;
15. restart preserves state;
16. fallback works;
17. HDD can be disabled;
18. AI can be disabled;
19. telemetry reconstructs the round;
20. no supply mismatch exists.

⸻

64. Private Testnet Exit Criteria

A private testnet phase is complete only when:

* 10,000 or more valid transactions finalize;
* 1,000 or more consecutive blocks finalize;
* honest state roots remain identical;
* producer fallback succeeds;
* partition recovery succeeds;
* no unauthorized issuance occurs;
* all rewards and fees reconcile;
* node restarts succeed;
* invalid traffic remains bounded;
* observed energy is documented;
* reward concentration is documented;
* unresolved risks are published internally.

⸻

65. Public Testnet Entry Criteria

A public experimental testnet MAY begin only if:

* no open critical vulnerability exists;
* no unexplained consensus divergence exists;
* safe mode is implemented and tested;
* wallet backup and restore are reliable;
* test units are clearly non-investment experimental units;
* public participant-admission limitations are disclosed;
* Sybil limitations are disclosed;
* signed or reproducible builds exist;
* incident response exists;
* vulnerability reporting exists;
* default APIs are hardened;
* destructive HDD functions are disabled by default.

⸻

66. Defect Lifecycle

A defect SHALL move through:

NEW
TRIAGED
REPRODUCED
ASSIGNED
IN_PROGRESS
FIXED
REGRESSION_TESTED
VERIFIED
CLOSED
DEFERRED
REJECTED

Critical defects require:

* immediate preservation of evidence;
* affected-version identification;
* regression test;
* specification review.

⸻

67. Protocol Defect Versus Implementation Defect

Every serious failure MUST be classified as:

IMPLEMENTATION_DEFECT
SPECIFICATION_DEFECT
PARAMETER_DEFECT
ARCHITECTURAL_DEFECT
OPERATIONAL_DEFECT
RESEARCH_LIMITATION

This classification determines whether the response is:

* code fix;
* document amendment;
* parameter change;
* redesign;
* operational control;
* acknowledged limitation.

⸻

68. Specification Amendment

If tests reveal that a specification rule is wrong, the project MUST:

1. preserve the original result;
2. create an issue;
3. explain the conflict;
4. amend the relevant document;
5. version the amendment;
6. update implementation;
7. update tests;
8. update traceability.

The code MUST NOT silently become the new specification.

⸻

69. Test Reports

The project SHOULD produce:

UNIT_TEST_REPORT
INTEGRATION_TEST_REPORT
LOCALNET_REPORT
PRIVATE_TESTNET_REPORT
RED_TEAM_REPORT
PERFORMANCE_REPORT
ENERGY_REPORT
HDD_REPORT
TOKENOMICS_REPORT
AI_EVALUATION_REPORT
RELEASE_GATE_REPORT

⸻

70. Executive Test Summary

Every milestone report SHOULD state:

* what was tested;
* what passed;
* what failed;
* what remains untested;
* critical risks;
* changes made;
* recommendation.

Recommended conclusion values:

ADVANCE
ADVANCE_WITH_RESTRICTIONS
REPEAT_TESTING
REDESIGN
STOP_COMPONENT
STOP_RELEASE

⸻

71. Raw Data Preservation

Raw test data SHOULD be retained long enough to support:

* reproduction;
* comparison;
* incident investigation;
* model evaluation;
* publication.

Secret or personal data MUST be removed or protected.

⸻

72. Test Data Integrity

Material test artifacts SHOULD include cryptographic hashes.

Reports SHOULD reference:

* log hashes;
* configuration hashes;
* binary hashes;
* dataset hashes;
* model hashes.

⸻

73. Human Review

Automated pass results do not replace human review for:

* cryptographic design;
* consensus assumptions;
* wallet user experience;
* economic incentives;
* AI explanations;
* HDD physical claims;
* security architecture.

⸻

74. Independent Review

Before public testnet, Q1 SHOULD seek independent review of:

* cryptography;
* consensus;
* wallet security;
* tokenomics;
* networking;
* build pipeline.

Internal testing alone is insufficient for a credible public security claim.

⸻

75. Known Testing Limitations

Even a comprehensive test plan cannot fully simulate:

* global public behavior;
* unknown zero-day vulnerabilities;
* nation-state attacks;
* real market speculation;
* long-term governance capture;
* future specialized hardware;
* all social-engineering attacks;
* decades of protocol use.

These limits MUST remain explicit.

⸻

76. Open Decisions

The following remain unresolved:

1. final implementation language and test framework;
2. final CI platform;
3. model-checking tooling;
4. fuzzing framework;
5. physical energy-measurement tools;
6. standard benchmark hardware;
7. performance target revisions;
8. seven-day endurance environment;
9. external review process;
10. public bug-bounty timing;
11. raw test-data retention period;
12. test-result publication policy;
13. cross-platform support scope;
14. public-testnet scale target;
15. coverage thresholds for networking and experimental modules;
16. independent client strategy;
17. exact release-gate authority;
18. handling of expected research failures;
19. simulation versus physical-test balance;
20. long-term compatibility test infrastructure.

All decisions MUST be recorded in OPEN_DECISIONS.md.

⸻

77. Codex Implementation Rules

Codex MUST:

1. create automated tests alongside implementation;
2. map tests to requirement IDs;
3. provide deterministic test seeds;
4. isolate test data and credentials;
5. implement local multi-node orchestration;
6. implement malicious-node profiles;
7. support fault injection;
8. preserve logs and evidence;
9. generate machine-readable results;
10. fail CI on critical invariant violations;
11. implement regression tests for confirmed defects;
12. distinguish expected research limitations from regressions;
13. provide benchmark tooling;
14. provide coverage reports;
15. support HDD simulation before physical testing;
16. ensure AI tests cannot affect consensus;
17. keep destructive tests opt-in;
18. record software, genesis, and configuration hashes;
19. support reproducible test runs;
20. maintain the requirement traceability matrix.

Codex MUST NOT:

* skip failed tests silently;
* rerun flaky consensus tests until they pass without investigation;
* use production credentials;
* run destructive HDD tests by default;
* target third-party systems;
* mark mempool acceptance as transaction success;
* claim security from code coverage alone;
* modify specifications implicitly through test assumptions;
* delete evidence of failed critical tests;
* treat public testnet deployment as proof of safety.

⸻

78. Final Testing Principle

Q1 testing MUST preserve this principle:

A specification describes what we intend.
Code expresses what we built.
Testing reveals what the system actually does.

Q1 shall not advance because its architecture is elegant, its code compiles, its creators believe in it, or its test unit acquires attention.

It advances only when evidence shows that:

* honest nodes agree;
* invalid actions fail;
* attacks are limited or exposed;
* critical conflicts stop safely;
* resources remain measurable;
* economics remain accountable;
* users retain control of keys;
* and every known failure becomes a documented path to improvement.

Q1 succeeds in testing not when every experiment passes, but when every important claim can be challenged, measured, reproduced, and judged.
