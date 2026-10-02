Q1 Architecture Specification

03_ARCHITECTURE.md

Project: Q1 Experimental Distributed Ledger
Protocol Version: 0.1
Document Version: 0.1.0
Status: Draft for Engineering Review
Classification: Experimental — Not for Production or Financial Use

⸻

1. Purpose

This document defines the software, protocol, service, data-flow, deployment, and trust architecture for Q1 v0.1.

The architecture MUST allow Q1 to:

* operate as a multi-node distributed ledger;
* separate deterministic consensus from experimental components;
* support replaceable delay engines;
* test HDD participation without making the core dependent on it;
* use AI for observation without granting AI consensus authority;
* run locally, privately, and later on a public testnet;
* simulate honest and malicious behavior;
* measure performance, energy use, security, and decentralization;
* allow Codex and future developers to implement each component independently.

This document describes where each responsibility belongs.

ADR-0001 through ADR-0005 establish the pre-M1 foundations: a Rust
consensus-critical core, bounded TypeScript/Node.js application layer,
restricted deterministic CBOR, domain-separated SHA-256, strict-profile
Ed25519, and a Q1-specific Bech32m address envelope. Their exact proposed V1
profiles are under `docs/protocol/` and require the pre-M1 gate before
implementation.

The exact mathematical and protocol rules are defined in later documents, especially:

* 04_LEDGER_AND_TRANSACTIONS.md
* 05_CONSENSUS.md
* 06_DELAY_ENGINE.md
* 07_HDD_LAB_MODULE.md
* 08_NODE_AND_NETWORKING.md
* 10_TOKENOMICS.md
* 12_SECURITY_MODEL.md

⸻

2. Architectural Objectives

Q1 v0.1 architecture MUST satisfy the following objectives.

Q1-ARC-001 — Deterministic core

The consensus-critical core MUST produce identical results across honest nodes given identical inputs.

The following MUST remain outside deterministic consensus:

* AI inference;
* local performance measurements;
* energy estimates;
* geographic inference;
* hardware-brand identification;
* unverified HDD telemetry;
* local wall-clock observations;
* human operator decisions.

⸻

Q1-ARC-002 — Modular replacement

The architecture MUST allow replacement of:

* cryptographic algorithms;
* ledger storage backend;
* producer-selection algorithm;
* delay implementation;
* HDD workload;
* validator-committee selection;
* fee calculation;
* reward allocation;
* networking transport;
* database;
* API framework;
* AI model.

No experimental module MAY be so tightly coupled that removing it requires rewriting unrelated components.

⸻

Q1-ARC-003 — Safety isolation

A failure in an experimental component MUST NOT silently corrupt ledger state.

Specifically:

* HDD module failure MUST NOT modify balances;
* AI observer failure MUST NOT stop finalization;
* explorer failure MUST NOT stop nodes;
* telemetry failure MUST NOT change consensus;
* dashboard failure MUST NOT affect networking;
* wallet-interface failure MUST NOT change protocol rules.

⸻

Q1-ARC-004 — Observable execution

Every important protocol stage MUST emit structured events.

The architecture MUST allow a complete block round to be reconstructed from logs and telemetry.

⸻

Q1-ARC-005 — Adversarial testability

The architecture MUST permit controlled insertion of:

* invalid transactions;
* invalid blocks;
* delayed messages;
* duplicate messages;
* conflicting attestations;
* network partitions;
* fake HDD responses;
* invalid delay proofs;
* malicious peer behavior;
* clock skew;
* producer failure;
* validator failure.

⸻

Q1-ARC-006 — Progressive decentralization

Early development MAY run several roles on one machine.

However, all major roles MUST be separable into independent processes or hosts.

⸻

3. Architectural Style

Q1 v0.1 SHALL use a modular distributed architecture with a deterministic protocol core and replaceable external modules.

The initial architecture combines:

* modular monolith principles inside the core node;
* service-oriented separation for non-consensus services;
* peer-to-peer communication among nodes;
* event-driven telemetry;
* plugin interfaces for delay and HDD modules.

The first implementation SHOULD avoid unnecessary microservice complexity.

The preferred rule is:

Keep consensus-critical logic in one strongly tested node codebase; separate only components that benefit from independent failure, scaling, experimentation, or security boundaries.

⸻

4. Top-Level System Diagram

                            ┌────────────────────────────┐
                            │        Q1 Test User        │
                            └─────────────┬──────────────┘
                                          │
                                          ▼
                            ┌────────────────────────────┐
                            │         Q1 Wallet          │
                            │ Key Mgmt / Sign / Submit   │
                            └─────────────┬──────────────┘
                                          │ API
                                          ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                               Q1 NODE                                       │
│                                                                             │
│  ┌────────────┐   ┌────────────┐   ┌──────────────┐   ┌─────────────────┐   │
│  │ Public API │──▶│ Tx Gateway │──▶│ Transaction  │──▶│ Mempool         │   │
│  └────────────┘   └────────────┘   │ Validation   │   └─────────────────┘   │
│                                    └──────────────┘            │            │
│                                                                ▼            │
│  ┌────────────┐   ┌──────────────┐   ┌──────────────┐   ┌──────────────┐   │
│  │ P2P Layer  │◀─▶│ Consensus    │◀─▶│ Block        │◀─▶│ State        │   │
│  │            │   │ Engine       │   │ Processor    │   │ Transition   │   │
│  └────────────┘   └──────┬───────┘   └──────────────┘   └──────┬───────┘   │
│                           │                                      │           │
│                           ▼                                      ▼           │
│                  ┌────────────────┐                      ┌──────────────┐    │
│                  │ Delay Adapter  │                      │ Ledger Store │    │
│                  └──────┬─────────┘                      └──────────────┘    │
│                         │                                                    │
│              ┌──────────┴──────────┐                                         │
│              ▼                     ▼                                         │
│   ┌───────────────────┐  ┌───────────────────┐                              │
│   │ Sequential Delay  │  │ HDD Lab Plugin    │                              │
│   │ / Future VDF      │  │ Optional          │                              │
│   └───────────────────┘  └───────────────────┘                              │
│                                                                             │
│  ┌────────────┐   ┌─────────────┐   ┌──────────────┐                       │
│  │ Telemetry  │──▶│ Event Bus   │──▶│ Structured   │                       │
│  │ Collector  │   │             │   │ Logs/Metrics │                       │
│  └────────────┘   └─────────────┘   └──────────────┘                       │
└─────────────────────────────────────────────────────────────────────────────┘
                 │                     │                    │
                 ▼                     ▼                    ▼
      ┌─────────────────┐   ┌──────────────────┐  ┌────────────────────┐
      │ Q1 Explorer     │   │ AI Observer      │  │ Test Orchestrator  │
      │ Read-only       │   │ Non-binding      │  │ Fault Injection    │
      └─────────────────┘   └──────────────────┘  └────────────────────┘

⸻

5. Architectural Zones

Q1 architecture is divided into six trust and responsibility zones.

5.1 Deterministic Protocol Zone

This zone contains consensus-critical logic.

Components:

* canonical serialization;
* transaction validation;
* block validation;
* ledger state transition;
* producer eligibility verification;
* delay-proof verification;
* validator-attestation verification;
* finalization logic;
* fee calculation;
* reward calculation;
* fork choice;
* protocol versioning.

Any code in this zone MUST be deterministic.

⸻

5.2 Node Operations Zone

This zone manages operation of a node but does not independently define validity.

Components:

* peer management;
* data synchronization;
* local database management;
* mempool management;
* message scheduling;
* health checks;
* configuration loading;
* graceful startup and shutdown;
* local caching.

⸻

5.3 Experimental Work Zone

This zone contains replaceable research components.

Components:

* simplified sequential delay;
* formal VDF implementations;
* HDD workload engine;
* simulated HDD;
* energy estimators;
* alternative producer selectors;
* alternative committee selectors.

Outputs from this zone MUST pass deterministic verification before affecting consensus.

⸻

5.4 User Interaction Zone

Components:

* command-line wallet;
* future graphical wallet;
* explorer;
* public read API;
* transaction-submission API.

This zone MUST NOT have direct write access to ledger state.

⸻

5.5 Observation and Research Zone

Components:

* telemetry collector;
* metrics database;
* AI observer;
* dashboards;
* experiment reports;
* anomaly detection.

This zone is non-authoritative.

⸻

5.6 Test and Adversarial Zone

Components:

* test orchestrator;
* network simulator;
* malicious-node profiles;
* fault injector;
* load generator;
* replay generator;
* disk-emulation tools;
* attack scripts.

This zone MUST be isolated from production-like deployments unless explicitly enabled.

⸻

6. Core Component Inventory

Q1 v0.1 SHALL contain the following primary components.

⸻

6.1 Q1 Core Protocol Library

Responsibility

The Q1 Core Protocol Library contains pure deterministic protocol rules.

It MUST define:

* primitive protocol types;
* canonical encodings;
* transaction format;
* block format;
* account state;
* state-transition rules;
* fee rules;
* reward rules;
* signature verification;
* hash calculation;
* Merkle or commitment calculation;
* delay-proof verification interface;
* producer-selection verification;
* attestation verification;
* finalization-certificate verification;
* protocol errors.

Restrictions

The Core Protocol Library MUST NOT:

* open network sockets;
* access local wall-clock time directly;
* call AI services;
* inspect hardware brands;
* read unverified telemetry;
* mutate external databases by itself;
* contain user-interface logic;
* depend on HDD availability.

M1.2 canonical ownership boundaries:

* BlockBodyV1 contains only schema version and the ordered SignedTransferV1
  sequence;
* ParticipantSetV1 contains schema version, reference Height, and the sorted
  non-empty ParticipantRecordV1 sequence derived from finalized pre-height
  state;
* TransactionCount and ParticipantCount are derived semantic values, not
  serialized fields;
* producer membership and role-key resolution are historical/state checks
  against the ParticipantSet committed for the Header height;
* BlockHeaderBodyV1 is the exact approved eleven-field array ending in a typed
  DelayEvidenceHash; CandidateIndex, ReceiptRoot, generic body commitment,
  counts, economic values, and operational metadata are absent;
* StateRoot field shape is required, but no BlockHeaderV1 may be implemented
  or instantiated as valid before state-model approval;
* the M1.1 RoundNumber correction is closed (docs/44). ChainId, ParticipantId
  and DelayEvidence domains are now registered. The 2026-10-01 human instruction
  authorizes progressive implementation; LOCALNET NONE and fee/quorum policy
  are recorded in `docs/protocol/Q1_LOCALNET_V0.md`. BlockHeader still requires
  the real state-model decision; no arbitrary StateRoot is permitted.

Current implemented boundaries: `q1-primitives` owns scalar/cryptographic
primitives; `q1-protocol-types` owns approved immutable schemas, structural
validation and typed commitments; `q1-localnet` owns explicitly local policy
and pure atomic accounting. It performs no networking, finality, persistence,
or state-root construction. This separation keeps local policy out of the
base protocol types. Current execution status is `PROJECT.md`.

Preferred design

The core SHOULD be implemented as a reusable library imported by:

* full nodes;
* validators;
* producers;
* explorer indexers;
* simulation tools;
* test harnesses.

⸻

6.2 Q1 Node Runtime

Responsibility

The Node Runtime coordinates all operational node components.

It SHALL manage:

* node startup;
* configuration;
* node identity;
* database initialization;
* peer-to-peer networking;
* synchronization;
* mempool;
* consensus rounds;
* block production;
* block verification;
* finalization;
* API exposure;
* telemetry;
* shutdown and recovery.

Node modes

A single node binary SHOULD support configurable roles:

full
validator
producer
observer
bootstrap
archive
light

The first milestone MUST support at least:

full
validator
producer
observer

Multiple roles MAY be enabled simultaneously.

⸻

6.3 Ledger State Engine

Responsibility

The Ledger State Engine applies deterministic state transitions.

Inputs:

* parent state;
* ordered valid transactions;
* protocol parameters;
* block metadata.

Outputs:

* new state;
* state root;
* transaction receipts;
* fee summary;
* reward summary;
* execution errors.

Requirements

The engine MUST be a pure or transactionally isolated operation.

A block MUST NOT become committed until:

1. all transactions execute successfully under block rules;
2. computed roots match the block;
3. consensus validity succeeds;
4. finalization conditions are satisfied.

⸻

6.4 Ledger Storage

Responsibility

Ledger Storage persists:

* blocks;
* block headers;
* canonical-chain index;
* finalized-chain index;
* transactions;
* account state;
* state snapshots;
* attestations;
* finalization certificates;
* protocol parameters;
* evidence of equivocation;
* node metadata.

Storage separation

Consensus data and telemetry data MUST use logically separate storage.

Recommended initial separation:

data/
├── ledger/
├── state/
├── mempool/
├── peerstore/
├── evidence/
├── telemetry/
└── logs/

Database options

The first implementation MAY use:

* SQLite for early prototype;
* RocksDB;
* LevelDB;
* PostgreSQL for indexer and explorer;
* another documented embedded key-value store.

The consensus logic MUST NOT depend on vendor-specific database behavior.

⸻

6.5 Transaction Gateway

Responsibility

The Transaction Gateway accepts transactions from:

* wallet API;
* local CLI;
* peers;
* test generator.

For the approved M1.2 transfer schema, its consensus input is
SignedTransferV1 containing the exact nine-field TransferBodyV1. Sender
identity is derived from the sole serialized sender public key;
`transaction_type`, `sender_address`, and `memo_hash` are not consensus
fields. This structural statement does not authorize gateway validation,
mempool admission, fee checks, propagation, or execution.

It SHALL perform:

* format checks;
* chain-ID check;
* size check;
* signature check;
* duplicate check;
* preliminary nonce check;
* preliminary balance check;
* fee check;
* rate limiting.

Valid transactions are forwarded to the mempool and peer propagation layer.

⸻

6.6 Mempool

Responsibility

The mempool holds valid but unconfirmed transactions.

The mempool SHALL maintain:

* transaction index;
* sender index;
* nonce ordering;
* fee ordering;
* expiration;
* replacement status;
* memory limits;
* anti-spam accounting.

The mempool is local and non-consensus-critical.

Two honest nodes MAY have different mempool contents while still agreeing on finalized blocks.

⸻

6.7 Block Builder

Responsibility

The Block Builder constructs candidate blocks for an eligible producer.

It SHALL:

1. read current finalized state;
2. identify current round;
3. confirm producer eligibility;
4. select valid mempool transactions;
5. order transactions deterministically under configured rules;
6. calculate provisional state transition;
7. calculate roots and summaries;
8. obtain delay output and proof;
9. include optional HDD commitment;
10. sign candidate block;
11. publish proposal.

Restriction

The Block Builder MUST NOT finalize its own block.

⸻

6.8 Block Processor

Responsibility

The Block Processor validates received block proposals.

Validation sequence SHOULD be:

1. decode canonical block;
2. verify chain identifier;
3. verify the exact ParentReferenceV1 structure, then verify its governing
   GenesisId or immediately preceding BlockId against signed-block height;
4. verify protocol version;
5. verify producer signature;
6. verify producer eligibility;
7. verify round and height;
8. verify delay challenge;
9. verify delay proof;
10. verify optional HDD commitment rules;
11. validate each transaction;
12. execute state transition;
13. verify transaction root;
14. verify state root;
15. verify fees;
16. verify rewards;
17. verify block-size limits;
18. emit validation result.

No attestation may be produced before full validation completes.

⸻

6.9 Consensus Engine

Responsibility

The Consensus Engine coordinates:

* round lifecycle;
* candidate scheduling;
* producer fallback;
* proposal handling;
* validator committee;
* attestations;
* quorum calculation;
* finalization certificate;
* fork choice;
* safe mode.

Internal state machine

The consensus engine SHOULD model each round as:

ROUND_CREATED
    ↓
CANDIDATES_SELECTED
    ↓
DELAY_RUNNING
    ↓
PROPOSAL_RECEIVED
    ↓
PROPOSAL_VALIDATED
    ↓
ATTESTATIONS_COLLECTED
    ↓
QUORUM_REACHED
    ↓
FINALIZED

Alternative terminal states:

REJECTED
TIMED_OUT
NO_PROPOSAL
CONFLICT_DETECTED
SAFE_MODE

⸻

6.10 Producer Selection Module

Responsibility

This module computes and verifies producer eligibility.

Inputs MAY include:

* previous finalized block hash;
* epoch or round seed;
* eligible node registry;
* participation history;
* cooldown;
* configurable anti-concentration factor;
* experimental reputation;
* experimental resource contribution.

Outputs:

* ordered producer candidates;
* eligibility proof or reproducible selection result;
* fallback order.

Architectural rule

The selection algorithm MUST be accessed through an interface:

ProducerSelector
├── select_candidates(context)
├── verify_selection(candidate, proof, context)
└── explain_selection(candidate, context)

This permits later replacement without rewriting consensus.

⸻

6.11 Validator Committee Module

Responsibility

This module selects and verifies temporary validator committees.

Interface:

CommitteeSelector
├── select_committee(context)
├── verify_membership(node, proof, context)
├── compute_weight(member, context)
└── compute_threshold(committee)

Committee selection MUST be reproducible by honest nodes.

⸻

6.12 Delay Adapter

Responsibility

The Delay Adapter separates consensus from a specific delay implementation.

Interface:

DelayEngine
├── create_challenge(context)
├── execute(challenge, difficulty)
├── verify(challenge, output, proof, difficulty)
├── estimate_cost(difficulty)
├── describe_capabilities()
└── version()

Supported implementations MAY include:

SequentialHashDelay
FormalVDFDelay
MockDelay
HDDCompositeDelay

Only implementations approved by protocol configuration MAY be accepted on a network.

⸻

6.13 Sequential Delay Engine

Responsibility

The initial Sequential Delay Engine provides a simple measurable sequential computation for prototype development.

It MAY use:

* iterative hash chaining;
* repeated modular operations;
* another explicitly sequential test function.

It MUST be labeled non-production unless formal security properties are established.

It SHALL generate:

* delay output;
* proof or reproducible execution data;
* performance telemetry.

⸻

6.14 HDD Lab Plugin

Responsibility

The HDD Lab Plugin performs challenge-bound storage operations.

The plugin architecture SHALL separate:

HDD Controller
HDD Workload Generator
HDD Commitment Builder
HDD Telemetry Collector
HDD Result Verifier
HDD Simulator

Plugin interface

HDDPlugin
├── inspect_device()
├── prepare_dataset(config)
├── execute_challenge(challenge, config)
├── build_commitment(result)
├── verify_commitment(challenge, commitment, evidence)
├── export_metrics()
└── cleanup()

Architectural rule

The core consensus engine MUST communicate only through the HDD interface.

It MUST NOT directly depend on operating-system-specific disk commands.

⸻

6.15 Cryptography Provider

Responsibility

All cryptographic primitives SHALL be accessed through a cryptographic provider.

Interface categories:

* hashing;
* digital signatures;
* key generation;
* secure random generation;
* address derivation;
* canonical commitments;
* future proof systems.

Example:

CryptoProvider
├── hash(data)
├── generate_keypair()
├── sign(private_key, message)
├── verify(public_key, message, signature)
├── derive_address(public_key)
├── secure_random(bytes)
└── algorithm_metadata()

The algorithm implementation MUST use reviewed libraries.

Custom cryptographic algorithms MUST NOT be introduced casually.

⸻

6.16 Peer-to-Peer Networking Layer

Responsibility

The P2P layer handles node communication.

It SHALL support:

* peer discovery;
* manual peer configuration;
* node handshake;
* protocol negotiation;
* message signing where required;
* transaction propagation;
* block propagation;
* attestation propagation;
* synchronization;
* peer scoring;
* rate limiting;
* disconnection of abusive peers.

Message domains

Messages SHOULD be separated by domain:

network.handshake
network.peer
tx.announce
tx.request
tx.response
block.proposal
block.request
block.response
consensus.attestation
consensus.finalization
sync.status
sync.snapshot
evidence.equivocation
telemetry.optional

⸻

6.17 Synchronization Engine

Responsibility

The Synchronization Engine brings a node to the canonical finalized state.

Modes:

* full sync from genesis;
* header-first sync;
* block-range sync;
* snapshot-assisted sync;
* recovery sync.

The synchronization engine MUST verify all consensus-critical data before acceptance.

A snapshot MUST NOT be trusted solely because a peer supplied it.

⸻

6.18 Wallet Service

Responsibility

The Wallet Service manages user-side operations.

Architecture SHOULD separate:

Wallet Core
Key Store
Transaction Builder
Signer
Node Client
CLI Interface

The wallet core MUST NOT require direct database access to node internals.

It communicates through a documented API.

⸻

6.19 Explorer and Indexer

Responsibility

The explorer provides read-only visibility.

The Indexer SHALL:

* consume finalized blocks;
* index blocks;
* index transactions;
* index addresses;
* calculate searchable summaries;
* expose read APIs.

The Explorer MUST NOT be treated as a consensus source.

If explorer data conflicts with a verified full node, the node is authoritative.

⸻

6.20 Telemetry Collector

Responsibility

The Telemetry Collector receives structured non-consensus events.

Sources:

* node runtime;
* consensus engine;
* delay engine;
* HDD plugin;
* networking;
* wallet;
* test orchestrator;
* operating-system metrics.

Destinations MAY include:

* local JSONL files;
* Prometheus;
* time-series database;
* research data warehouse;
* AI observer.

Telemetry failure MUST be non-fatal unless explicitly running a test that requires it.

⸻

6.21 AI Observer

Responsibility

The AI Observer analyzes network and experiment data.

The AI Observer architecture SHOULD contain:

Telemetry Ingestor
Feature Extractor
Rule-Based Detector
Statistical Detector
AI Model Adapter
Risk Scoring Engine
Explanation Generator
Report Builder
Model Registry

Isolation

The AI Observer SHALL communicate with Q1 through read-only data channels.

It MUST NOT have:

* ledger database write credentials;
* validator signing keys;
* producer signing keys;
* wallet private keys;
* protocol-administration authority.

⸻

6.22 Test Orchestrator

Responsibility

The Test Orchestrator starts, stops, configures, and attacks test networks.

It SHALL support:

* deterministic node generation;
* temporary data directories;
* topology creation;
* latency injection;
* packet loss;
* partitioning;
* node crashes;
* process restarts;
* malicious behavior profiles;
* load generation;
* test result collection.

Suggested interface:

q1-testnet up
q1-testnet down
q1-testnet status
q1-testnet partition
q1-testnet heal
q1-testnet attack
q1-testnet load
q1-testnet report

⸻

7. Recommended Repository Structure

Q1 SHOULD initially use a monorepo to maintain consistency between protocol definitions, tests, and modules.

Q1/
├── docs/
│   ├── 00_PROJECT_CHARTER.md
│   ├── 01_GLOSSARY.md
│   ├── 02_SYSTEM_REQUIREMENTS.md
│   ├── 03_ARCHITECTURE.md
│   └── ...
│
├── protocol/
│   ├── types/
│   ├── serialization/
│   ├── transactions/
│   ├── blocks/
│   ├── state/
│   ├── fees/
│   ├── rewards/
│   ├── consensus_rules/
│   └── errors/
│
├── crypto/
│   ├── hashing/
│   ├── signatures/
│   ├── addresses/
│   └── randomness/
│
├── node/
│   ├── runtime/
│   ├── config/
│   ├── mempool/
│   ├── block_processor/
│   ├── block_builder/
│   ├── sync/
│   ├── storage/
│   ├── health/
│   └── api/
│
├── consensus/
│   ├── rounds/
│   ├── producer_selection/
│   ├── committee_selection/
│   ├── attestations/
│   ├── finalization/
│   ├── fork_choice/
│   └── safe_mode/
│
├── delay/
│   ├── interfaces/
│   ├── sequential_hash/
│   ├── mock/
│   ├── formal_vdf/
│   └── benchmarks/
│
├── hdd_lab/
│   ├── interfaces/
│   ├── device_probe/
│   ├── workloads/
│   ├── commitments/
│   ├── simulator/
│   ├── telemetry/
│   └── benchmarks/
│
├── network/
│   ├── transport/
│   ├── discovery/
│   ├── handshake/
│   ├── protocol/
│   ├── peer_manager/
│   ├── propagation/
│   └── rate_limit/
│
├── wallet/
│   ├── core/
│   ├── keystore/
│   ├── signer/
│   ├── cli/
│   └── client/
│
├── explorer/
│   ├── indexer/
│   ├── api/
│   └── web/
│
├── telemetry/
│   ├── events/
│   ├── metrics/
│   ├── exporters/
│   └── dashboards/
│
├── ai_observer/
│   ├── ingestion/
│   ├── features/
│   ├── detectors/
│   ├── models/
│   ├── reports/
│   └── registry/
│
├── simulator/
│   ├── network/
│   ├── economics/
│   ├── consensus/
│   ├── energy/
│   └── attacks/
│
├── testnet/
│   ├── orchestrator/
│   ├── topologies/
│   ├── malicious_nodes/
│   ├── load_generator/
│   └── reports/
│
├── configs/
│   ├── development/
│   ├── localnet/
│   ├── private_testnet/
│   └── examples/
│
├── genesis/
│   ├── development.json
│   ├── localnet.json
│   └── private_testnet.json
│
├── scripts/
├── tests/
│   ├── unit/
│   ├── integration/
│   ├── multi_node/
│   ├── adversarial/
│   ├── performance/
│   └── recovery/
│
├── OPEN_DECISIONS.md
├── CHANGELOG.md
├── SECURITY.md
├── CONTRIBUTING.md
└── README.md

⸻

8. Process Architecture

Q1 v0.1 MAY initially run several modules in one process.

The target process architecture is:

Process 1: q1-node
├── protocol core
├── ledger
├── mempool
├── networking
├── consensus
├── delay adapter
├── optional HDD plugin
├── node API
└── telemetry emitter
Process 2: q1-wallet
├── key store
├── signer
└── node client
Process 3: q1-indexer
├── finalized block consumer
├── search index
└── explorer API
Process 4: q1-explorer-web
└── user interface
Process 5: q1-ai-observer
├── telemetry ingestion
├── anomaly analysis
└── reporting
Process 6: q1-test-orchestrator
├── deployment
├── fault injection
└── result collection

The HDD plugin MAY run:

* inside q1-node;
* as a subprocess;
* or as a separate local service.

For safety and experimentation, a subprocess boundary is preferred after the first prototype.

⸻

9. Node Internal Architecture

q1-node
│
├── Startup Manager
│   ├── Load configuration
│   ├── Load genesis
│   ├── Validate chain settings
│   ├── Open databases
│   ├── Load node identity
│   └── Start services
│
├── API Service
│   ├── Public read API
│   ├── Transaction submission
│   ├── Admin API
│   └── Health API
│
├── P2P Service
│   ├── Discovery
│   ├── Handshake
│   ├── Peer manager
│   ├── Message router
│   └── Rate limiter
│
├── Transaction Service
│   ├── Validation
│   ├── Mempool
│   └── Propagation
│
├── Chain Service
│   ├── Block store
│   ├── State store
│   ├── Block processor
│   ├── Canonical-chain manager
│   └── Snapshot manager
│
├── Consensus Service
│   ├── Round manager
│   ├── Producer selector
│   ├── Committee selector
│   ├── Proposal manager
│   ├── Attestation manager
│   ├── Finalization manager
│   └── Fork-choice manager
│
├── Production Service
│   ├── Candidate scheduler
│   ├── Block builder
│   ├── Delay adapter
│   ├── HDD plugin adapter
│   └── Producer signer
│
├── Sync Service
│   ├── Peer status
│   ├── Header sync
│   ├── Block sync
│   ├── State sync
│   └── Recovery
│
└── Observability Service
    ├── Structured logger
    ├── Metrics
    ├── Tracing
    └── Telemetry exporter

⸻

10. Trust Boundaries

Q1 MUST explicitly recognize the following trust boundaries.

10.1 Wallet-to-node boundary

The node MUST treat every submitted transaction as untrusted.

The node MUST independently verify it.

⸻

10.2 Peer-to-peer boundary

All peer messages are untrusted until verified.

A peer identity does not imply honesty.

⸻

10.3 Node-to-delay-engine boundary

Delay output is untrusted until deterministic verification succeeds.

⸻

10.4 Node-to-HDD boundary

HDD telemetry and commitments are untrusted research data unless validated under protocol rules.

⸻

10.5 Node-to-AI boundary

AI output is advisory only.

⸻

10.6 Node-to-storage boundary

Database corruption MUST be detectable.

Stored data MUST be verified where possible before use in consensus-critical operations.

⸻

10.7 Operator-to-node boundary

An operator may control process startup and configuration, but MUST NOT be able to rewrite finalized state through ordinary administration APIs.

⸻

11. Transaction Data Flow

1. Wallet builds transaction.
2. Wallet calculates expected fee.
3. Wallet signs canonical transaction bytes.
4. Wallet submits transaction to a node.
5. Transaction Gateway validates format and limits.
6. Protocol Core verifies signature and transaction rules.
7. Mempool checks local conflicts and nonce ordering.
8. Node accepts transaction into local mempool.
9. Node propagates transaction announcement.
10. Peers request or receive transaction.
11. Eligible producer selects transaction.
12. Block Builder executes transaction against candidate state.
13. Candidate block is proposed.
14. Validators re-execute transaction.
15. Block receives attestations.
16. Block finalizes.
17. Transaction leaves mempool.
18. Wallet and explorer report finalized status.

⸻

12. Block Production Data Flow

1. Previous block becomes finalized.
2. Consensus Engine creates next round context.
3. Producer Selector computes candidate order.
4. Committee Selector computes validator committee.
5. First candidate receives or derives delay challenge.
6. Delay Engine begins sequential work.
7. Optional HDD plugin executes challenge-bound workload.
8. Producer gathers eligible mempool transactions.
9. Block Builder computes candidate state.
10. Delay output and proof become available.
11. Producer inserts delay proof and optional HDD commitment.
12. Producer signs candidate block.
13. Candidate block is broadcast.
14. Validators process the block independently.
15. Valid validators sign attestations.
16. Attestations propagate.
17. Consensus Engine verifies threshold.
18. Finalization certificate is assembled.
19. Block is committed as finalized.
20. Rewards and fees become part of finalized state.
21. Telemetry and AI observer receive events.

⸻

13. Consensus Message Flow

Primary consensus messages:

RoundAnnouncement
ProducerEligibility
BlockProposal
BlockValidationResult
ValidatorAttestation
AttestationAggregate
FinalizationCertificate
RoundTimeout
FallbackActivation
EquivocationEvidence
SafeModeNotice

Every consensus message MUST include:

* protocol version;
* chain identifier;
* block height;
* round identifier;
* sender node identifier;
* message type;
* canonical payload;
* signature where required;
* message identifier.

⸻

14. Event Architecture

All components SHOULD emit events through a common event contract.

Example event envelope:

{
  "event_id": "uuid",
  "event_type": "consensus.block_finalized",
  "event_version": "1",
  "node_id": "node-public-id",
  "chain_id": "q1-localnet",
  "block_height": 42,
  "round_id": "42-1",
  "timestamp_local": "2026-01-01T00:00:00Z",
  "severity": "info",
  "payload": {}
}

Local timestamps are observational.

They MUST NOT independently establish protocol ordering.

⸻

15. Storage Architecture

15.1 Block storage

Stores immutable block objects indexed by block hash.

⸻

15.2 Canonical-chain index

Maps block height to canonical finalized block hash.

⸻

15.3 State storage

Stores account state.

State updates MUST be atomic.

⸻

15.4 Transaction index

Maps transaction hash to:

* block;
* status;
* sender;
* recipient;
* receipt.

⸻

15.5 Consensus evidence store

Stores:

* attestations;
* finalization certificates;
* conflicting signatures;
* rejected proposals;
* attack evidence.

⸻

15.6 Snapshot store

Stores verifiable state snapshots.

⸻

15.7 Telemetry store

Must be separate from ledger state.

Telemetry loss MUST NOT change balances or block validity.

⸻

16. API Architecture

Q1 SHALL separate API domains.

Public API

Read-only and transaction-submission functions.

Examples:

GET  /api/v1/status
GET  /api/v1/blocks/latest
GET  /api/v1/blocks/{id}
GET  /api/v1/transactions/{hash}
GET  /api/v1/accounts/{address}
POST /api/v1/transactions
POST /api/v1/fees/estimate

⸻

Validator and producer API

Local authenticated controls.

Examples:

GET  /api/v1/validator/status
GET  /api/v1/producer/status
POST /api/v1/producer/enable
POST /api/v1/producer/disable

These APIs MUST NOT expose private keys.

⸻

Administrative API

Authenticated and preferably bound to localhost by default.

Examples:

POST /api/v1/admin/shutdown
POST /api/v1/admin/reload
GET  /api/v1/admin/peers
POST /api/v1/admin/peers/ban
POST /api/v1/admin/snapshot

Administrative actions MUST NOT bypass consensus rules.

⸻

Telemetry API

Read-only access to metrics.

Examples:

GET /metrics
GET /api/v1/telemetry/summary
GET /api/v1/telemetry/energy

⸻

17. Configuration Architecture

Configuration SHALL be divided into:

Local operational configuration

May differ between nodes:

* data directory;
* API bind address;
* log level;
* peer limits;
* telemetry destination;
* HDD device path;
* local wallet path;
* dashboard configuration.

⸻

Consensus configuration

MUST be identical across a network:

* chain identifier;
* block interval;
* delay implementation;
* delay difficulty;
* committee size;
* finalization threshold;
* fee rules;
* reward rules;
* block limits;
* transaction format version;
* cryptographic algorithms;
* producer-selection algorithm.

Consensus configuration MUST be committed in:

* genesis;
* protocol version;
* or finalized governance change.

⸻

18. Deployment Architecture

18.1 Single-node development

Developer Machine
├── q1-node
├── q1-wallet
├── q1-explorer
├── q1-ai-observer
└── local databases

⸻

18.2 Local multi-node network

Developer Machine
├── Node A
├── Node B
├── Node C
├── Node D
├── Wallet
├── Explorer
├── AI Observer
└── Test Orchestrator

Each node MUST use:

* separate identity;
* separate port;
* separate data directory;
* separate configuration.

⸻

18.3 Private distributed testnet

Host 1
├── Node A: producer + full
└── HDD Module A
Host 2
├── Node B: validator + full
└── HDD Module B
Host 3
└── Node C: validator + full
Host 4
└── Node D: producer + validator
Host 5
└── Node E: observer + archive
Research Host
├── Explorer
├── Telemetry Database
├── Dashboard
├── AI Observer
└── Test Controller

Consensus MUST continue when the research host is offline.

⸻

19. Containerization

Q1 SHOULD support containerized development and test deployment.

Recommended services:

q1-node-a
q1-node-b
q1-node-c
q1-node-d
q1-indexer
q1-explorer
q1-telemetry
q1-ai-observer
q1-test-controller

HDD physical tests MAY require direct host access and therefore MAY run outside containers.

Containerization MUST NOT hide or falsify hardware assumptions.

⸻

20. Failure Isolation

The architecture MUST define failure behavior.

Node API failure

Consensus MAY continue.

⸻

Explorer failure

Consensus MUST continue.

⸻

Telemetry failure

Consensus MUST continue.

⸻

AI observer failure

Consensus MUST continue.

⸻

HDD plugin failure

The affected producer MAY lose eligibility or fail its candidate round according to protocol rules.

The ledger MUST NOT become corrupted.

⸻

Delay-engine failure

The producer MUST fail safely and MUST NOT produce an unverifiable block.

⸻

Database write failure

The node MUST stop consensus participation until integrity is restored.

⸻

Conflicting finalization certificate

The node MUST enter safe mode.

⸻

Cryptographic verification failure

The relevant object MUST be rejected.

⸻

21. Safe Mode Architecture

Safe mode is a node state for critical inconsistency.

Safe mode MAY activate when:

* conflicting finalized blocks are detected;
* database integrity fails;
* genesis mismatch appears;
* protocol version is unsupported;
* critical cryptographic provider fails;
* finalization certificate is malformed but widely propagated;
* consensus-critical configuration changes unexpectedly.

In safe mode, the node SHOULD:

* stop producing blocks;
* stop attesting;
* stop finalizing;
* continue read-only diagnostics;
* preserve evidence;
* notify operators;
* expose safe-mode status;
* avoid automatic destructive recovery.

⸻

22. Key Architecture

Q1 MUST separate key types.

Wallet Spending Key
Node Identity Key
Producer Signing Key
Validator Attestation Key
Administrative API Credential
Telemetry Authentication Credential

In early prototypes, some node-role keys MAY share storage, but the architecture MUST support separation.

Wallet spending keys MUST NOT be reused as node identity keys.

⸻

23. Protocol Versioning Architecture

Every consensus-critical object MUST contain or derive a protocol version.

Version changes SHALL be classified as:

* backward-compatible;
* testnet-resetting;
* hard-fork equivalent;
* storage-migration only;
* API-only;
* non-consensus.

Protocol code SHOULD route behavior through versioned handlers:

ProtocolVersionRouter
├── v0_1
├── v0_2
└── future

Silent interpretation of unknown versions is prohibited.

⸻

24. Extensibility Points

Q1 v0.1 MUST define stable interfaces for future work.

Potential future extensions:

* formal VDF;
* proof-of-space integration;
* alternate ledger model;
* smart-contract runtime;
* privacy layer;
* quantum-resistant signatures;
* advanced light clients;
* mobile validation;
* distributed governance;
* bridge protocols;
* decentralized storage;
* multi-asset support.

These extensions MUST NOT be implemented by polluting the initial deterministic core.

⸻

25. Architectural Non-Goals

Q1 v0.1 architecture is not intended to:

* maximize transaction throughput;
* support global production traffic;
* hide all metadata;
* provide perfect hardware identity;
* make HDD data inherently truthful;
* eliminate all trusted setup in research infrastructure;
* replace legal or regulatory systems;
* support real financial custody;
* guarantee economic value;
* guarantee resistance to all future cryptanalysis.

⸻

26. Minimum Architectural Milestone

The architecture is implemented successfully when:

1. four independently configured nodes run;
2. nodes discover or connect to peers;
3. wallets submit signed transactions;
4. nodes maintain separate mempools;
5. producer selection occurs;
6. a producer executes delay work;
7. a candidate block is broadcast;
8. validators verify it independently;
9. attestations are collected;
10. a finalization certificate is created;
11. all honest nodes commit identical state;
12. an explorer displays finalized data;
13. telemetry reconstructs the round;
14. the HDD plugin can be enabled or disabled;
15. the AI observer can stop without affecting consensus;
16. a malicious test node can be injected;
17. a node can restart and resynchronize;
18. invalid proposals are rejected;
19. consensus-critical code remains independent of UI and AI;
20. each component maps to a documented interface.

⸻

27. Architectural Decisions Requiring Later Resolution

The following decisions remain open:

* primary implementation language;
* embedded database;
* P2P transport framework;
* serialization format;
* signature algorithm;
* hash algorithm;
* formal VDF construction;
* exact HDD workload;
* process boundary for HDD;
* event-bus technology;
* metrics backend;
* explorer framework;
* AI model architecture;
* deployment platform;
* light-client protocol;
* snapshot format;
* reputation storage;
* node-registry model.

Each unresolved decision MUST be recorded in OPEN_DECISIONS.md.

⸻

28. Codex Architecture Rules

Codex MUST follow these rules:

1. Do not place AI logic inside consensus code.
2. Do not place HDD operating-system commands inside protocol core.
3. Do not use floating-point arithmetic for ledger values.
4. Do not use local time as sole consensus truth.
5. Do not allow API handlers to mutate balances directly.
6. Do not permit explorer or indexer data to determine consensus.
7. Do not hide assumptions in utility functions.
8. Do not couple producer selection to a single permanent algorithm.
9. Do not couple delay verification to one implementation.
10. Do not store private keys in logs.
11. Do not create a single global administrator capable of rewriting finalized state.
12. Do not treat telemetry as verified physical truth.
13. Do not continue consensus after critical database corruption.
14. Do not silently accept unsupported protocol versions.
15. Create interfaces and tests before adding interchangeable implementations.

⸻

29. Final Architectural Principle

Q1 architecture MUST preserve one fundamental separation:

The deterministic protocol decides what is valid.
The distributed network communicates and confirms it.
Experimental hardware contributes evidence.
AI observes patterns.
Humans improve the design.
No single layer may quietly become the master of all others.

The Q1 architecture is successful when every component can explain:

* what data it receives;
* what authority it has;
* what data it produces;
* how its output is verified;
* what happens when it fails;
* and whether the network can continue without it.
