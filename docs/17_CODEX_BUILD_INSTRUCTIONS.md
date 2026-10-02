Q1 Codex Build Instructions

17_CODEX_BUILD_INSTRUCTIONS.md

Project: Q1 Experimental Distributed Ledger
Protocol Version: 0.1
Build Plan Version: 0.1.0
Status: Initial Engineering Execution Plan
Classification: Experimental — Local and Private Test Environments Only

⸻

1. Purpose

This document defines the implementation instructions for Codex and all future Q1 developers.

It converts the Q1 specification set into an ordered engineering program.

Codex SHALL use this document to determine:

* what must be built first;
* which modules depend on other modules;
* which implementation decisions may be made locally;
* which decisions require explicit documentation;
* which tests must pass before advancing;
* when development must stop;
* what artifacts must be generated;
* how failures must be reported;
* how specifications and code must remain synchronized.

The objective is not to generate the largest possible codebase.

The objective is to produce the smallest complete Q1 prototype that can:

* execute deterministic transactions;
* maintain a valid ledger;
* operate across several independent nodes;
* perform delay work;
* finalize blocks through committee attestations;
* survive ordinary failures;
* expose measurable behavior;
* and be attacked safely.

⸻

2. Authority of Documents

Codex SHALL treat the Q1 documents as the engineering authority in the following order:

1. Explicit human instruction for the current approved task
2. Security and safety constraints
3. 02_SYSTEM_REQUIREMENTS.md
4. 12_SECURITY_MODEL.md
5. 03_ARCHITECTURE.md
6. Consensus-critical component specifications
7. 17_CODEX_BUILD_INSTRUCTIONS.md
8. API, deployment, testing, and roadmap documents
9. Existing code
10. Comments and assumptions inside implementation

Existing code MUST NOT silently override a written protocol rule.

If implementation and specification conflict, Codex MUST:

1. stop work on the conflicting behavior;
2. identify the conflict;
3. record it;
4. recommend whether the code or specification should change;
5. wait for an approved resolution where the conflict is material.

⸻

3. Mandatory Input Documents

Before beginning implementation, Codex MUST read:

00_PROJECT_CHARTER.md
01_GLOSSARY.md
02_SYSTEM_REQUIREMENTS.md
03_ARCHITECTURE.md
04_LEDGER_AND_TRANSACTIONS.md
05_CONSENSUS.md
06_DELAY_ENGINE.md
07_HDD_LAB_MODULE.md
08_NODE_AND_NETWORKING.md
09_WALLET.md
10_TOKENOMICS.md
11_AI_OBSERVER.md
12_SECURITY_MODEL.md
13_HOW_TO_BREAK_Q1.md
14_TEST_PLAN.md
15_SIMULATION_PLAN.md
16_API_SPECIFICATION.md
17_CODEX_BUILD_INSTRUCTIONS.md

Codex MUST NOT begin by generating arbitrary blockchain boilerplate before reviewing these documents.

⸻

4. Build Philosophy

Q1-CBX-001 — Correctness before complexity

Codex MUST prefer:

* explicit state machines;
* small pure functions;
* typed interfaces;
* deterministic data structures;
* testable modules;
* readable code;
* clear errors.

It MUST avoid premature:

* microservices;
* sharding;
* smart contracts;
* bridges;
* token features;
* GUI complexity;
* performance optimizations;
* hardware assumptions.

⸻

Q1-CBX-002 — Small complete milestones

Each milestone MUST produce a working and testable result.

Codex MUST NOT create dozens of incomplete modules in parallel.

⸻

Q1-CBX-003 — Test-first protocol development

Consensus-critical behavior SHOULD be implemented in this order:

1. Test vector
2. Interface
3. Deterministic implementation
4. Unit tests
5. Property tests
6. Integration
7. Documentation

⸻

Q1-CBX-004 — No hidden assumptions

Any implementation assumption not already defined in the specifications MUST be added to:

OPEN_DECISIONS.md

or:

IMPLEMENTATION_NOTES.md

depending on whether the matter is unresolved or merely implementation-specific.

⸻

Q1-CBX-005 — Experimental honesty

Codex MUST label experimental modules accurately.

It MUST NOT describe:

* sequential hashing as a formal VDF;
* HDD telemetry as proof of physical rotation;
* permissioned localnet as permissionless decentralization;
* test units as valuable currency;
* AI alerts as protocol truth.

⸻

5. Initial Implementation Strategy

Q1 SHOULD begin as a monorepo.

The preferred first architecture is:

Q1/
├── docs/
├── protocol/
├── crypto/
├── ledger/
├── consensus/
├── delay/
├── hdd_lab/
├── network/
├── node/
├── wallet/
├── explorer/
├── telemetry/
├── ai_observer/
├── simulator/
├── testnet/
├── configs/
├── genesis/
├── scripts/
└── tests/

Codex MUST preserve clear module boundaries even if several modules initially run in one process.

⸻

6. Language Selection

ADR-0001 selects a strictly bounded hybrid architecture:

* Rust owns the consensus-critical core;
* TypeScript/Node.js may own approved application and developer-facing layers;
* TypeScript SHALL NOT independently redefine consensus-critical rules;
* shared behavior must cross a controlled Rust interface or normative vectors
  with mandatory cross-language conformance.

The authoritative scope boundary and rationale are recorded in:

`docs/adr/ADR-0001-primary-language.md`

M1 remains blocked until the required protocol profiles and Rust toolchain
evidence pass the pre-M1 gate.

⸻

7. Architecture Decision Records

Material engineering decisions MUST use Architecture Decision Records.

Recommended format:

docs/adr/
├── ADR-0001-primary-language.md
├── ADR-0002-canonical-serialization.md
├── ADR-0003-hash-function.md
├── ADR-0004-signature-algorithm.md
├── ADR-0005-address-encoding.md
├── ADR-0006-storage-engine.md
├── ADR-0007-p2p-transport.md
└── ...

Each ADR MUST contain:

Title
Status
Context
Decision
Alternatives
Consequences
Security Impact
Test Impact
Migration Impact

⸻

8. Prohibited Initial Features

Codex MUST NOT implement the following unless later approved:

* smart contracts;
* arbitrary on-chain code;
* decentralized exchange;
* bridge;
* token sale;
* real mainnet;
* staking derivatives;
* browser extension wallet;
* mobile mining;
* production slashing;
* negative transaction fees;
* AI-controlled consensus;
* HDD-based finality;
* direct administrator balance editing;
* hidden recovery key;
* market-price oracle;
* automatic governance execution.

These features are outside Q1 v0.1.

⸻

9. Build Milestones

The implementation SHALL proceed through the following milestones:

M0 — Repository and engineering foundation
M1 — Protocol primitives and cryptography
M2 — Ledger and transaction engine
M3 — Single-node chain
M4 — Wallet and public node API
M5 — Multi-node networking
M6 — Consensus and finalization
M7 — Delay Engine integration
M8 — HDD Laboratory module
M9 — Tokenomics and economic accounting
M10 — Explorer, telemetry, and AI Observer
M11 — Simulation and adversarial framework
M12 — Private testnet packaging

A milestone MUST NOT begin if its entry gate is not satisfied.

⸻

10. Milestone M0 — Repository Foundation

Objective

Create a clean, reproducible engineering workspace.

Required outputs

README.md
CONTRIBUTING.md
SECURITY.md
LICENSE or license decision placeholder
OPEN_DECISIONS.md
IMPLEMENTATION_NOTES.md
CHANGELOG.md
docs/adr/
configs/
genesis/
tests/
CI configuration
formatter configuration
linter configuration
dependency lockfile

Tasks

1. create repository structure;
2. add document files;
3. add build instructions;
4. choose preliminary toolchain;
5. configure formatting;
6. configure linting;
7. configure unit-test execution;
8. configure secret scanning;
9. configure dependency auditing;
10. create CI;
11. create development configuration schema;
12. create test-only genesis schema;
13. create structured logging conventions.

Required tests

* repository builds;
* formatter passes;
* linter passes;
* empty test suite runs;
* configuration schema validates;
* secret scanner passes;
* dependency lockfile exists.

Exit gate

M0 passes only when a new developer can clone the repository and run one documented command that:

builds
tests
lints

the empty engineering foundation.

⸻

11. Milestone M1 — Protocol Primitives and Cryptography

Objective

Create deterministic protocol types and cryptographic interfaces.

Required modules

protocol/types
protocol/serialization
protocol/errors
crypto/hash
crypto/signatures
crypto/addresses
crypto/randomness

Required types

At minimum:

ChainId
ProtocolVersion
BlockHeight
RoundNumber
Nonce
Amount
Fee
Difficulty
Address
PublicKey
Signature
Hash
TransactionId
BlockHash
ParticipantId
NodeId

Rules

* monetary types MUST wrap checked integers;
* protocol identifiers MUST not be ordinary interchangeable strings internally;
* canonical serialization MUST be isolated;
* domain separators MUST be constants;
* parsing MUST be strict;
* unknown versions MUST fail.

Required ADRs

canonical serialization
hash algorithm
signature algorithm
address encoding
integer representation

Required tests

* deterministic serialization;
* round-trip encoding;
* malformed input rejection;
* stable test vectors;
* signature generation and verification;
* signature-field mutation rejection;
* domain separation;
* address checksum;
* wrong-network address rejection;
* secure-random failure handling.

Exit gate

M1 passes only when two independent executions produce identical bytes and hashes for all published protocol test vectors.

⸻

12. Milestone M2 — Ledger and Transaction Engine

Objective

Implement the deterministic account ledger.

Required modules

ledger/accounts
ledger/state
ledger/transactions
ledger/receipts
ledger/roots
ledger/supply
ledger/fees_placeholder
ledger/rewards_placeholder

Required functions

validate_transaction_static()
validate_transaction_against_state()
apply_transaction()
execute_block_transactions()
calculate_transaction_root()
calculate_receipt_root()
calculate_state_root()
verify_supply_conservation()

Initial transaction types

TRANSFER
GENESIS_ALLOCATION
PROTOCOL_REWARD

PROTOCOL_REWARD MAY remain internal until M9.

Required behaviors

* account creation;
* balance changes;
* nonce validation;
* transfer execution;
* expiration;
* fee limit;
* deterministic receipt;
* temporary state;
* atomic execution;
* replay rejection;
* supply conservation.

Required tests

At minimum all mandatory transaction and ledger tests from:

04_LEDGER_AND_TRANSACTIONS.md
14_TEST_PLAN.md

Property tests

Must include:

* no negative balances;
* ordinary transfer preserves supply;
* nonce increments exactly once;
* failed transaction preserves state;
* replay fails;
* execution is deterministic;
* arithmetic never wraps.

Exit gate

M2 passes only when:

* 10,000 generated valid transaction sequences execute deterministically;
* 10,000 generated invalid sequences do not alter state;
* supply remains exact;
* all test vectors pass.

⸻

13. Milestone M3 — Single-Node Chain

Objective

Create one node capable of producing and persisting a local finalized chain without distributed consensus.

This milestone is a development scaffold, not the final consensus model.

Required modules

node/runtime
node/config
node/storage
node/mempool
node/block_builder
node/block_processor
node/chain_manager
node/api_health

Required behavior

* load genesis;
* open database;
* create accounts from genesis;
* accept signed transactions;
* place transactions in mempool;
* build deterministic local blocks;
* execute temporary state;
* commit blocks atomically;
* persist receipts;
* restart;
* recover state;
* expose health and status.

Local development finality

M3 MAY use:

LOCAL_SINGLE_NODE_FINALITY

This mode MUST:

* be clearly separated from Q1 consensus;
* be disabled outside development configurations;
* never be described as decentralized finality.

Required tests

* startup;
* genesis hash;
* transaction submission;
* block construction;
* atomic commit;
* restart;
* database failure;
* state-root verification;
* mempool removal;
* stale transaction removal.

Exit gate

M3 passes when one local node finalizes 1,000 development blocks and retains exact state after restart.

⸻

14. Milestone M4 — Wallet and Public Node API

Objective

Allow a user to create an encrypted wallet and submit a transfer.

Required wallet modules

wallet/core
wallet/keystore
wallet/accounts
wallet/signer
wallet/transaction_builder
wallet/node_client
wallet/backup
wallet/offline
wallet/cli

Required node APIs

GET  /api/v1/status
GET  /api/v1/health
GET  /api/v1/readiness
GET  /api/v1/accounts/{address}
POST /api/v1/fees/estimate
POST /api/v1/transactions
GET  /api/v1/transactions/{id}
GET  /api/v1/blocks/latest

Required wallet commands

create
account new
address show
balance
tx build
tx preview
tx sign
tx send
tx status
backup create
backup verify
backup restore
offline export-unsigned
offline sign
offline import-signed

Security rules

* no plaintext private keys;
* no password in command arguments;
* no secret logs;
* complete transaction preview;
* wrong-network rejection;
* local signature self-check;
* atomic keystore writes.

Required tests

All mandatory wallet tests from 09_WALLET.md.

Exit gate

M4 passes when:

1. Wallet A and Wallet B are created.
2. Wallet A receives genesis test units.
3. Wallet A signs a transfer to Wallet B.
4. The node accepts and finalizes it in development mode.
5. Both balances update.
6. Backup and restore reproduce the same addresses.
7. Offline signing succeeds.

⸻

15. Milestone M5 — Multi-Node Networking

Objective

Allow independent nodes to discover, authenticate, communicate, and synchronize.

Required modules

network/transport
network/identity
network/handshake
network/messages
network/peer_manager
network/discovery
network/propagation
network/rate_limit
network/scoring
node/sync

Initial transport

Use the transport selected by ADR.

Required behavior

* distinct node identities;
* authenticated handshake;
* chain and genesis verification;
* static peers;
* bootstrap peers;
* status exchange;
* transaction announcement and request;
* block announcement and request;
* known-object cache;
* size limits;
* rate limits;
* graceful disconnect;
* full sync from genesis.

Required local topology

4 independent node processes

Each with separate:

* identity;
* port;
* data directory;
* database;
* configuration.

Required adversarial tests

* malformed handshake;
* wrong chain;
* genesis mismatch;
* duplicate connection;
* message flood;
* oversized payload;
* slow peer;
* false highest height;
* bootstrap shutdown;
* node restart and resync.

Exit gate

M5 passes when four nodes independently reach identical state through network propagation and one newly started node synchronizes from peers.

⸻

16. Milestone M6 — Consensus and Finalization

Objective

Replace single-node development finality with Q1 committee consensus.

Required modules

consensus/context
consensus/rounds
consensus/producer_selection
consensus/committee_selection
consensus/proposals
consensus/attestations
consensus/finalization
consensus/fallback
consensus/round_change
consensus/evidence
consensus/safe_mode

Initial participant model

A genesis-defined private participant registry MAY be used.

It MUST be labeled:

PERMISSIONED_PRIVATE_TESTNET_REGISTRY

Initial selection model

Use the simplest deterministic approved model.

If weighting remains unresolved, begin with:

* equal eligible producer weight;
* equal validator weight;
* deterministic seeded selection;
* producer cooldown if already fully specified.

Codex MUST NOT invent a complex reputation system in M6.

Required behavior

* round initialization;
* deterministic candidate list;
* deterministic committee;
* producer windows;
* block proposal;
* validator verification;
* attestation;
* threshold;
* finalization certificate;
* fallback;
* round timeout;
* signing protection;
* equivocation evidence;
* safe mode.

Required tests

All critical consensus tests from:

05_CONSENSUS.md
12_SECURITY_MODEL.md
13_HOW_TO_BREAK_Q1.md
14_TEST_PLAN.md

Exit gate

M6 passes only when:

* at least four nodes finalize the same blocks;
* primary producer failure activates fallback;
* invalid blocks receive no honest vote;
* duplicate votes are not counted;
* validator restart cannot cause accidental double-vote;
* forged certificates fail;
* conflicting certificates trigger safe mode.

⸻

17. Milestone M7 — Delay Engine Integration

Objective

Require selected producers to complete challenge-bound sequential work.

Required implementations

MockDelayEngine
SequentialHashDelayEngine

Required interfaces

derive_challenge()
execute()
verify()
cancel()
benchmark()

Required behavior

* challenge bound to chain, height, round, producer, candidate, parent, engine, and difficulty;
* deterministic output;
* proof transport;
* full recomputation verification in prototype mode;
* cancellation after round loss;
* proof-size limit;
* difficulty bounds;
* structured telemetry.

Required warning

The implementation and documentation MUST state:

Sequential Hash Delay is not a formal VDF.

Required benchmarks

* generation time;
* verification time;
* proof size;
* CPU use;
* memory use;
* energy estimate where possible.

Exit gate

M7 passes when altered context invalidates proofs and delay failure prevents the producer from proposing a valid block without affecting other nodes.

⸻

18. Milestone M8 — HDD Laboratory Module

Objective

Implement HDD as an isolated research plugin.

Default configuration

HDD_MODE = TELEMETRY_ONLY

Required modules

hdd_lab/interfaces
hdd_lab/discovery
hdd_lab/safety
hdd_lab/dataset
hdd_lab/workloads
hdd_lab/commitments
hdd_lab/simulator
hdd_lab/telemetry
hdd_lab/benchmarks

Required first workload

RANDOM_READ_TRANSFORM_COMMIT_V1

Required safety rules

* explicit test directory;
* no raw disk access;
* no system disk writes;
* write workloads disabled;
* bounded dataset size;
* bounded workload;
* cancellation;
* path traversal protection;
* symlink protection.

Required comparisons

* HDD;
* SSD;
* RAM disk;
* virtual disk.

Consensus rule

HDD failure in telemetry-only mode MUST NOT invalidate a block.

Exit gate

M8 passes when the plugin:

* creates a deterministic dataset;
* derives a challenge-bound workload;
* produces a commitment;
* records metrics;
* survives device failure safely;
* can be disabled without changing base consensus.

⸻

19. Milestone M9 — Tokenomics and Economic Accounting

Objective

Replace fee and reward placeholders with deterministic economic rules.

Required modules

economics/parameters
economics/fees
economics/issuance
economics/rewards
economics/maturity
economics/treasury
economics/burn
economics/accounting

Initial economic configuration

Use configurable private-testnet values.

Do not encode permanent monetary policy.

Required behavior

* base issuance;
* transaction fee;
* size fee;
* simple finalized-block congestion multiplier;
* fee limit;
* producer reward;
* delay reward;
* validator reward;
* treasury allocation;
* deterministic remainder;
* optional burn;
* reward maturity if enabled;
* supply accounting.

Default prohibited behaviors

negative fees
market-price inputs
HDD reward
unbounded adaptive issuance

Required tests

* fee conservation;
* supply conservation;
* no reward before finality;
* no unauthorized reward;
* duplicate role accounting;
* deterministic rounding;
* multi-node economic agreement.

Exit gate

M9 passes when all nodes independently derive the same fees, rewards, treasury state, and total supply across 10,000 finalized transactions.

⸻

20. Milestone M10 — Explorer, Telemetry, and AI Observer

Objective

Make the network observable without giving observation authority.

Explorer requirements

finalized blocks
transactions
accounts
supply
fees
rewards
indexed height

Explorer MUST be read-only.

Telemetry requirements

* common event envelope;
* metrics;
* structured logs;
* block-round reconstruction;
* network metrics;
* delay metrics;
* HDD metrics;
* economic metrics.

AI Observer initial scope

Begin with:

1. deterministic rule engine;
2. statistical baseline;
3. optional model adapter.

Do not begin with a highly autonomous language model.

Required initial alerts

* producer equivocation;
* validator equivocation;
* finality conflict;
* producer concentration;
* validator concentration;
* repeated round timeout;
* delay anomaly;
* HDD telemetry anomaly;
* network partition signal.

Required isolation

Observer process MUST have:

no ledger write credential
no signing key
no wallet key
no admin authority

Exit gate

M10 passes when the observer can be terminated without affecting block finalization and all alerts identify evidence class and uncertainty.

⸻

21. Milestone M11 — Simulation and Adversarial Framework

Objective

Create reproducible large-scale and hostile experimentation.

Required simulator features

* deterministic seed;
* node agents;
* operator ownership;
* network latency;
* transaction generation;
* producer selection;
* committees;
* delay profiles;
* economic accounting;
* energy estimates;
* malicious agents;
* parameter sweeps;
* raw result export.

Required attack profiles

malicious user
malicious producer
malicious validator
Sybil operator
eclipse operator
HDD fraud operator
fee manipulator
telemetry poisoner

Required fault injection

node crash
partition
packet loss
clock skew
disk full
database corruption
HDD disconnect
AI outage

Required reports

* test run;
* attack result;
* economic concentration;
* energy comparison;
* simulation assumptions.

Exit gate

M11 passes when identical seeds reproduce identical logical results and at least the minimum adversarial milestone in 13_HOW_TO_BREAK_Q1.md is automated.

⸻

22. Milestone M12 — Private Testnet Packaging

Objective

Package Q1 for controlled multi-host operation.

Required outputs

node binary or package
wallet binary or package
indexer
explorer
AI Observer
test orchestrator
configuration examples
genesis generator
deployment scripts
operator guide
security guide
backup guide
incident guide

Required supported environment

At least:

Linux

Additional platforms MAY remain development-only until verified.

Required topology

7 nodes
3 or more hosts
1 archive node
1 observer node
1 malicious test node

Required acceptance

* 1,000 consecutive blocks;
* 10,000 finalized transfers;
* exact state roots;
* exact supply;
* successful fallback;
* restart recovery;
* partition recovery;
* no unresolved critical defects;
* complete telemetry;
* published internal limitations.

⸻

23. Build Order Within Each Milestone

Codex SHOULD use this order:

1. Read specifications
2. Identify requirement IDs
3. Identify unresolved decisions
4. Write or update ADRs
5. Define interfaces
6. Define data types
7. Create test vectors
8. Write unit tests
9. Implement minimum behavior
10. Run tests
11. Add integration tests
12. Add telemetry
13. Add documentation
14. Produce milestone report

⸻

24. Task Decomposition Format

Before coding a milestone, Codex SHALL create:

docs/build/MX_PLAN.md

Example contents:

Objective
Inputs
Decisions required
Modules
Interfaces
Tasks
Tests
Risks
Exit criteria
Deferred work

Tasks SHOULD be small enough to review independently.

⸻

25. Codex Work Report

After each implementation task, Codex SHOULD report:

Task completed
Files created
Files changed
Tests added
Tests run
Results
Open issues
Specification conflicts
Next safe task

Codex MUST not claim a test passed if it was not run.

⸻

26. Definition of Done

A task is complete only when:

* implementation exists;
* tests exist;
* tests pass;
* errors are handled;
* logs avoid secrets;
* documentation is updated;
* formatting and linting pass;
* assumptions are recorded.

A milestone is complete only when its exit gate passes.

⸻

27. Test Failure Rules

If a test fails, Codex MUST:

1. preserve the failing input;
2. preserve the deterministic seed;
3. identify whether failure is:
    * implementation;
    * specification;
    * parameter;
    * environment;
4. fix only after root-cause analysis;
5. add a regression test;
6. rerun relevant suites.

Codex MUST NOT simply rerun flaky consensus tests until they pass.

⸻

28. Stop-Work Conditions

Codex MUST stop advancing the affected milestone when any of the following occurs:

* unauthorized supply creation;
* deterministic state divergence;
* accepted invalid signature;
* accepted forged certificate;
* conflicting honest finality;
* secret leakage;
* unsafe arbitrary file write;
* unexplained consensus nondeterminism;
* database corruption treated as success;
* specification contradiction affecting validity.

The system MAY continue diagnostic work, but no release gate may advance.

⸻

29. Security-Critical Code Rules

Security-critical modules include:

canonical serialization
cryptography
ledger state transition
supply accounting
consensus
attestation signing protection
finalization
delay verification
wallet keystore
administrative authorization

For these modules:

* no unchecked arithmetic;
* no silent error conversion;
* no panics on untrusted input where avoidable;
* no hidden global mutable state;
* no nondeterministic iteration;
* no direct external side effect inside pure validation functions;
* no unreviewed unsafe code;
* no secret logging.

⸻

30. Determinism Rules

Consensus-critical code MUST avoid:

* floating-point values;
* local timezone;
* locale-sensitive formatting;
* unordered map iteration;
* filesystem order;
* local wall-clock ordering;
* random values not derived from protocol context;
* database result order without sorting;
* platform-dependent integer behavior.

Cross-platform test vectors MUST be generated.

⸻

31. Error-Handling Rules

Errors MUST be:

* typed;
* stable where externally visible;
* contextual;
* non-secret;
* mapped to protocol or API error codes.

Codex MUST distinguish:

invalid external input
temporary operational failure
internal invariant failure
safe-mode condition

These MUST not be collapsed into one generic error.

⸻

32. Logging Rules

Structured logging SHALL include:

event type
component
node ID
chain ID
height
round
request ID
object hash
result
error code

where applicable.

Logs MUST NOT include:

* private keys;
* wallet password;
* recovery phrase;
* raw administrative token;
* unencrypted seed.

⸻

33. Configuration Rules

Configuration MUST be divided into:

consensus-critical
operational
experimental
secret

Consensus-critical configuration MUST come from genesis or finalized protocol parameters.

Operational configuration MAY be local.

Secrets MUST not be stored in ordinary configuration files where avoidable.

⸻

34. Genesis Rules

Codex SHALL implement:

* genesis schema;
* genesis validator;
* genesis hash;
* deterministic genesis state;
* example development genesis;
* example localnet genesis;
* example private-testnet genesis.

Changing genesis creates a different chain.

⸻

35. Database Rules

The storage layer MUST:

* use atomic commits;
* separate ledger and telemetry;
* support restart;
* detect schema version;
* support migrations;
* support corruption diagnostics;
* not expose vendor-specific behavior to protocol logic.

The selected database requires an ADR.

⸻

36. API Rules

Codex MUST implement APIs from 16_API_SPECIFICATION.md progressively.

It MUST not implement all endpoints before the underlying service exists.

Every endpoint requires:

* schema;
* authentication class;
* rate limit;
* tests;
* stable errors;
* documentation.

⸻

37. Dependency Rules

Codex MUST:

* minimize dependencies;
* pin versions;
* use lockfiles;
* prefer mature libraries;
* avoid abandoned cryptographic packages;
* record licenses;
* scan vulnerabilities;
* document high-risk dependencies.

A new cryptographic or networking dependency requires review.

⸻

38. Generated Code Rules

Generated code MAY be used for:

* API clients;
* schemas;
* serialization helpers.

Generated code MUST:

* be reproducible;
* identify its generator version;
* not conceal protocol logic;
* be reviewable;
* be regenerated through documented commands.

⸻

39. Unsafe Code and Native Extensions

If the selected language permits unsafe code, it MUST be isolated and justified.

Native extensions and FFI MUST be avoided initially unless needed for:

* reviewed cryptographic library;
* formal VDF library;
* hardware measurement.

Every FFI boundary requires tests and documentation.

⸻

40. Concurrency Rules

Codex MUST keep deterministic state changes serialized or transactionally controlled.

Concurrent tasks MAY handle:

* networking;
* transaction validation;
* proof verification;
* telemetry;
* API requests.

But finalized state mutation MUST have one explicit commit path.

⸻

41. Resource-Limit Rules

Every externally triggered operation requires limits.

At minimum:

* request bytes;
* message bytes;
* proof bytes;
* transaction size;
* block size;
* mempool size;
* queue size;
* peer count;
* concurrent validations;
* dataset size;
* simulation workload.

Unbounded external allocation is prohibited.

⸻

42. Feature Flags

Experimental features MUST use explicit configuration flags.

Examples:

enable_mock_delay
enable_hdd_lab
enable_hdd_write_tests
enable_ai_model_adapter
enable_test_attack_api
enable_single_node_finality

Unsafe development flags MUST be disabled in private-testnet builds by default.

⸻

43. Build Profiles

Recommended build profiles:

development
test
localnet
private-testnet
research
release-candidate

A future production profile is not approved.

Each profile MUST clearly define enabled features.

⸻

44. Test Keys and Credentials

Test keys MUST:

* be clearly marked;
* remain inside test resources;
* never protect real assets;
* never be reused in public environments.

Development genesis MAY use deterministic test keys.

Wallet software MUST warn when known test keys are imported.

⸻

45. Documentation Synchronization

When implementation changes protocol behavior, Codex MUST update:

* relevant specification;
* ADR;
* tests;
* changelog;
* API schema;
* traceability matrix.

Code comments alone are insufficient.

⸻

46. Open Decision Handling

When Codex reaches an unresolved design decision, it SHALL classify it.

Type A — Safe implementation detail

Examples:

* internal file name;
* local helper structure;
* test directory naming.

Codex MAY decide and document it in IMPLEMENTATION_NOTES.md.

Type B — Architectural decision

Examples:

* database;
* transport;
* language;
* serialization.

Codex MUST create an ADR.

Type C — Consensus or economic rule

Examples:

* quorum formula;
* fee formula;
* hash algorithm;
* participant weighting;
* issuance.

Codex MUST NOT decide silently.

It must record the issue in OPEN_DECISIONS.md and request or await approved resolution before final implementation.

⸻

47. Placeholder Rules

Codex MAY use placeholders only when:

* clearly labeled;
* isolated behind an interface;
* disabled outside development;
* covered by replacement tests;
* listed in deferred work.

Examples:

MockDelayEngine
DevelopmentProducerSelector
TestGenesisRegistry

A placeholder MUST NOT silently remain in a release candidate.

⸻

48. Simulation Before Optimization

Before introducing a complex mechanism, Codex SHOULD first implement it in the simulator where practical.

Examples:

* adaptive fee rules;
* producer weighting;
* HDD rewards;
* reputation;
* reward smoothing;
* governance.

This reduces risk of embedding untested economic behavior into consensus code.

⸻

49. Performance Optimization Rules

Optimization begins only after correctness tests pass.

Every optimization MUST:

* preserve test vectors;
* preserve deterministic results;
* include benchmarks;
* include before-and-after measurements;
* avoid hidden security assumptions.

Codex MUST not optimize by weakening verification.

⸻

50. Cryptographic Upgrade Path

Cryptographic algorithms MUST be accessed through providers and versioned identifiers.

Codex SHALL avoid embedding one algorithm throughout unrelated code.

Future upgrade support SHOULD exist for:

* hash functions;
* signatures;
* VDF;
* address versions.

Unknown algorithms MUST fail closed.

⸻

51. Multiple Client Readiness

Q1 v0.1 may have one implementation.

However, Codex MUST publish enough deterministic vectors for a future independent client to implement:

* transactions;
* blocks;
* roots;
* signatures;
* consensus objects;
* delay proofs;
* fee and reward calculations.

Implementation-specific behavior MUST not become undocumented consensus.

⸻

52. Security Review Artifacts

Before M12, Codex SHALL ensure the repository contains:

SECURITY.md
docs/security/THREAT_REGISTER.md
docs/security/INCIDENT_RESPONSE.md
docs/security/KEY_MANAGEMENT.md
docs/security/RELEASE_SECURITY.md
docs/security/DEPENDENCY_POLICY.md

These MAY begin as drafts and mature through milestones.

⸻

53. Milestone Report Format

Each milestone MUST produce:

docs/build/MX_REPORT.md

The report SHALL include:

Objective
Completed work
Architecture decisions
Files and modules
Tests executed
Pass/fail summary
Benchmarks
Known defects
Security findings
Deferred work
Exit-gate result
Recommendation

Recommendation values:

ADVANCE
ADVANCE_WITH_RESTRICTIONS
REPEAT
REDESIGN
STOP_COMPONENT
STOP_PROJECT_PHASE

⸻

54. Build Command Standardization

The repository SHOULD expose simple top-level commands.

Conceptually:

q1 build
q1 test
q1 lint
q1 format
q1 localnet up
q1 localnet down
q1 wallet
q1 simulate
q1 benchmark

These MAY be implemented through a task runner, Makefile, or project-native tooling.

A developer SHOULD not need to remember many unrelated low-level commands.

⸻

55. Local Development Workflow

Recommended workflow:

1. Clone repository.
2. Validate toolchain.
3. Build all enabled development components.
4. Run unit tests.
5. Generate development genesis.
6. Start local node.
7. Create wallet.
8. Run smoke transfer.
9. Shut down cleanly.

This workflow MUST be documented in README.md.

⸻

56. Localnet Workflow

Recommended:

1. Generate node identities.
2. Generate participant registry.
3. Generate localnet genesis.
4. Create separate configurations.
5. Start nodes.
6. Wait for synchronization.
7. Run wallet transfers.
8. Observe finalization.
9. Inject producer failure.
10. Verify fallback.
11. Collect report.

⸻

57. No Background Authority

Codex MUST NOT create:

* central coordinator required for finality;
* hidden master node;
* mandatory cloud service;
* administrator that signs for validators;
* explorer-based consensus;
* AI-based approval service.

The test orchestrator may coordinate tests, but must not be required for ordinary consensus.

⸻

58. Role Separation

The implementation SHALL keep separate:

Node identity
Producer identity
Validator identity
Wallet account
Administrative identity
Telemetry service identity

Even when one operator controls several roles, keys and permissions MUST remain distinct.

⸻

59. HDD Removal Readiness

Codex MUST implement HDD so the module can be removed by:

* disabling a feature flag;
* removing the plugin;
* leaving consensus code unchanged.

If removing HDD requires rewriting ledger or consensus, the architecture has failed.

⸻

60. AI Removal Readiness

Codex MUST implement AI Observer so it can be stopped or removed without:

* consensus failure;
* node failure;
* wallet failure;
* ledger changes;
* economic changes.

⸻

61. Test Orchestrator Isolation

Attack and fault-injection features MUST be:

* in separate test modules;
* disabled in normal builds;
* authenticated in test environments;
* unable to access real wallet secrets;
* unable to target arbitrary external hosts by default.

⸻

62. Security Disclosure During Development

If Codex discovers a critical vulnerability, it MUST:

1. stop public exposure;
2. preserve evidence;
3. create a private issue or security record;
4. identify affected versions;
5. create a minimal reproduction;
6. recommend mitigation;
7. add regression tests after correction.

It MUST not conceal the issue in a generic refactor.

⸻

63. Prohibited Claims in Generated Documentation

Codex MUST NOT write that Q1 is:

* secure;
* quantum secure;
* environmentally superior;
* decentralized;
* fraud-proof;
* unhackable;
* investment-grade;
* production-ready;

unless the claim is precisely scoped and supported by evidence.

Preferred language:

experimental
measured under stated conditions
not yet proven
private-testnet only
research result

⸻

64. Minimum Coding Standards

Code SHOULD be:

* typed;
* formatted;
* documented at public interfaces;
* free of dead code;
* free of unexplained magic numbers;
* explicit about units;
* explicit about versions;
* explicit about failure.

Protocol constants MUST have names and specification references.

⸻

65. Requirement References in Code

Consensus-critical modules SHOULD reference requirement IDs.

Example:

Implements Q1-CON-022 and Q1-CON-024.

This improves traceability.

Comments MUST not merely repeat code.

They SHOULD explain protocol intent or security rationale.

⸻

66. Unit Naming

All units MUST be explicit.

Examples:

duration_ms
size_bytes
amount_q1u
energy_wh
block_height
round_number

Ambiguous names such as:

value
time
size

SHOULD be avoided in protocol interfaces.

⸻

67. Time and Clock Rules

Codex MUST distinguish:

protocol height
protocol round
virtual simulation time
local monotonic time
wall-clock timestamp

Wall-clock timestamps MUST not determine ledger ordering.

Timeouts SHOULD use monotonic clocks.

⸻

68. Monetary Rules

Codex MUST:

* use checked integer types;
* serialize JSON amounts as strings;
* forbid negative fees;
* verify supply after each block;
* verify fee distribution;
* verify reward distribution;
* version economic rules.

⸻

69. Finality Rules

Codex MUST never use the word or state:

FINALIZED

unless a valid finalization certificate has been verified, except in explicitly labeled single-node development mode.

Mempool acceptance and proposal inclusion are not finality.

⸻

70. Safe Mode Implementation Priority

Safe mode MUST be implemented during M6, not postponed until the end.

A consensus prototype without safe mode MUST not proceed to adversarial distributed testing.

⸻

71. Evidence Persistence

The following MUST be persistable:

* producer equivocation;
* validator equivocation;
* conflicting certificates;
* invalid finalized-state evidence;
* supply mismatch evidence;
* attack test artifacts.

Evidence storage MUST be separate from ordinary telemetry where appropriate.

⸻

72. Release Candidate Requirements

A release candidate for private testing requires:

* clean build;
* passing required tests;
* no open critical defect;
* signed or hashed artifacts;
* documented commit;
* documented genesis;
* configuration examples;
* migration notes;
* known limitations;
* rollback plan.

⸻

73. Public Testnet Restriction

Codex MUST NOT create or deploy a public testnet merely because M12 passes.

Public testnet requires a separate approved milestone and updated security review.

⸻

74. Mainnet Restriction

No mainnet implementation, deployment, genesis, token sale, or financial launch is authorized by this document.

Any future mainnet requires:

* new specification set;
* independent security review;
* public threat review;
* legal and regulatory review;
* finalized economics;
* public incident plan;
* separate approval.

⸻

75. Minimal First Build Target

The first meaningful Q1 executable target SHOULD be:

q1-node
q1-wallet
q1-localnet

The smallest successful demonstration is:

1. start four nodes;
2. load one genesis;
3. select a producer;
4. execute delay;
5. propose a block;
6. collect validator attestations;
7. finalize;
8. transfer Q1T between two wallets;
9. restart one node;
10. resynchronize;
11. disable HDD and AI without affecting consensus.

⸻

76. First Codex Execution Prompt

When implementation begins, Codex SHOULD receive a task similar to:

Read all Q1 specification documents and do not write protocol code yet.
Create:
1. a specification consistency report;
2. OPEN_DECISIONS.md;
3. ADR candidates;
4. the proposed repository structure;
5. the M0 implementation plan;
6. a requirement traceability skeleton.
Identify contradictions, missing definitions, unsafe assumptions, and decisions that block implementation.
Do not invent consensus or economic rules.

This SHALL be the preferred first engineering action.

⸻

77. Second Codex Execution Prompt

After M0 approval:

Implement Milestone M0 only.
Create the repository foundation, build tooling, CI, linting, formatting, secret scanning, configuration schemas, genesis schema, documentation structure, and empty test harnesses.
Do not implement transactions, consensus, wallet, or networking yet.
Produce M0_REPORT.md and stop after the M0 exit gate.

⸻

78. Subsequent Prompt Rule

Each Codex prompt SHOULD authorize only one milestone or one bounded task.

Avoid prompts such as:

Build the entire blockchain.

Preferred:

Implement canonical serialization and its deterministic test vectors for M1.
Do not implement networking or consensus.

⸻

79. Human Review Points

Human approval SHOULD occur after:

* language ADR;
* serialization ADR;
* cryptographic ADRs;
* M2 ledger;
* M6 consensus;
* M7 delay;
* M8 HDD findings;
* M9 tokenomics;
* M11 adversarial results;
* M12 private-testnet report.

Codex MUST not treat automated tests as approval of unresolved protocol design.

⸻

80. Final Codex Principle

Codex SHALL preserve this principle:

Build only what is specified.
Test every claim.
Isolate every experiment.
Record every assumption.
Stop at every critical contradiction.
Never hide uncertainty behind working code.

Codex is not asked to prove Q1 correct by producing many files.

It is asked to help Q1 discover whether it can become correct.

The build is successful when:

* each module has a clear authority boundary;
* deterministic code agrees across nodes;
* users retain their keys;
* finality requires valid distributed evidence;
* failed experiments can be removed;
* critical failures stop safely;
* every unit remains accounted;
* every milestone leaves reproducible evidence;
* and the next engineering decision is based on what was actually built and tested.

Q1 must emerge from Codex not as a mysterious machine, but as a system whose every important rule, interface, assumption, failure, and test can be read, challenged, and reproduced.
