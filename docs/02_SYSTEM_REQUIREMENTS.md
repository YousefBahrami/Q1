Q1 System Requirements Specification

02_SYSTEM_REQUIREMENTS.md

Project: Q1 Experimental Distributed Ledger
Protocol Version: 0.1
Document Version: 0.1.0
Status: Draft for Engineering Review
Classification: Experimental — Not for Production or Financial Use

⸻

1. Purpose

This document defines the system requirements for the first executable version of Q1.

The purpose of Q1 v0.1 is not to launch a public cryptocurrency, create market value, or compete with existing blockchain networks.

The purpose is to build a measurable experimental distributed network capable of:

* creating and transferring native digital units;
* maintaining a shared ledger among independent nodes;
* producing blocks without a permanent central server;
* testing a fair-delay-based consensus model;
* testing an optional HDD-based physical delay module;
* measuring performance, energy use, security, and decentralization;
* exposing weaknesses before any public release.

Q1 v0.1 shall be treated as a laboratory system.

⸻

2. Normative Language

The keywords used in this document have the following meanings:

* MUST: mandatory for Q1 v0.1;
* MUST NOT: prohibited in Q1 v0.1;
* SHOULD: strongly recommended unless a documented technical reason prevents it;
* SHOULD NOT: generally prohibited unless justified;
* MAY: optional;
* EXPERIMENTAL: subject to change or removal;
* OUT OF SCOPE: not part of Q1 v0.1.

Codex and all future contributors MUST implement mandatory requirements unless an explicit specification amendment is recorded.

⸻

3. System Mission

Q1 v0.1 MUST determine whether a distributed ledger can combine:

1. lightweight public validation;
2. limited block-producer selection;
3. verifiable sequential delay;
4. optional HDD participation;
5. deterministic node consensus;
6. dynamic but understandable fees;
7. multi-role reward distribution;
8. AI-assisted anomaly observation;
9. measurable energy efficiency;
10. resistance to common blockchain attacks.

⸻

4. Core Design Principles

Q1-SYS-001 — Deterministic validity

All honest nodes receiving the same valid state and the same data MUST reach the same result regarding:

* transaction validity;
* block validity;
* ledger state;
* account balances;
* block finality.

AI output MUST NOT be required to reach deterministic consensus.

⸻

Q1-SYS-002 — Modular architecture

The following components MUST be independently replaceable:

* ledger model;
* signature system;
* delay engine;
* HDD module;
* block-producer selection algorithm;
* fee engine;
* reward engine;
* networking layer;
* AI observer.

Removing the HDD module or AI observer MUST NOT require rewriting the entire system.

⸻

Q1-SYS-003 — Public verifiability

Any full node MUST be able to independently verify:

* every accepted transaction;
* every accepted block;
* every state transition;
* every producer signature;
* every delay proof;
* every reward allocation.

No hidden authority MAY certify a valid block.

⸻

Q1-SYS-004 — Low entry barrier

An ordinary personal computer SHOULD be capable of running a full testnet node.

A lightweight validator SHOULD be capable of running on devices with limited resources.

Specialized hardware MUST NOT be required for basic transaction validation.

⸻

Q1-SYS-005 — Measurability

Every important protocol action MUST produce telemetry suitable for:

* debugging;
* performance analysis;
* security analysis;
* energy estimation;
* network simulation;
* protocol comparison.

⸻

Q1-SYS-006 — No financial promises

Q1 v0.1 MUST NOT include:

* public token sales;
* profit guarantees;
* exchange listing mechanisms;
* fiat purchase systems;
* investment marketing;
* promises of future market value.

All units created in Q1 v0.1 are test units only.

⸻

5. System Scope

Q1 v0.1 SHALL include:

* a native test asset;
* public/private key generation;
* signed transactions;
* balances;
* transaction propagation;
* a transaction pool;
* block creation;
* block validation;
* chain synchronization;
* producer selection;
* sequential delay execution;
* delay-proof verification;
* optional HDD experiments;
* validator voting or attestation;
* block finalization;
* reward calculation;
* fee calculation;
* telemetry;
* local and distributed testnet deployment;
* attack simulation;
* a command-line wallet;
* a basic block explorer;
* an AI observation interface.

⸻

6. Out of Scope for Version 0.1

The following features MUST NOT be treated as production requirements:

* smart contracts;
* decentralized applications;
* NFTs;
* bridges to other blockchains;
* stablecoins;
* privacy coins;
* zero-knowledge payments;
* public exchange trading;
* fiat gateways;
* mobile production wallets;
* hardware wallets;
* governance voting with real economic effect;
* public mainnet;
* anonymous public mining pools;
* legal tender functionality;
* irreversible real-value payments;
* quantum-resistant cryptography as a completed feature;
* automatic protocol changes made by AI.

These MAY be researched later.

⸻

7. System Actors

7.1 User

A user creates wallets and submits signed transactions.

A user MUST NOT need to run a block producer.

⸻

7.2 Lightweight Validator

A lightweight validator:

* verifies transaction signatures;
* checks basic block data;
* maintains limited state;
* propagates network messages;
* MAY receive a small experimental reward.

It MUST NOT perform heavy computation.

⸻

7.3 Full Node

A full node:

* stores the complete canonical ledger;
* verifies all transactions;
* verifies all blocks;
* reconstructs state;
* participates in synchronization;
* detects conflicting chains;
* exposes data to the explorer.

⸻

7.4 Block Producer

A block producer:

* becomes eligible through the selection mechanism;
* builds a candidate block;
* performs the required delay procedure;
* signs and publishes the block;
* receives a reward only after finalization.

⸻

7.5 Delay Executor

The delay executor performs the sequential delay process.

In v0.1, the block producer and delay executor MAY be the same entity.

The architecture MUST allow them to be separated later.

⸻

7.6 HDD Participant

An HDD participant runs the experimental HDD module.

The HDD participant:

* receives a challenge;
* performs defined disk operations;
* returns a result;
* provides telemetry;
* MAY contribute data to block production.

HDD participation MUST be optional in the earliest development mode.

⸻

7.7 Validator Committee

A validator committee is a temporarily selected group that:

* verifies a candidate block;
* verifies its delay proof;
* checks transaction execution;
* issues signed attestations;
* participates in finalization.

⸻

7.8 AI Observer

The AI observer:

* reads telemetry;
* identifies anomalies;
* assigns non-binding risk scores;
* prepares reports;
* proposes test scenarios.

The AI observer MUST NOT:

* create ledger state;
* sign transactions;
* hold user private keys;
* finalize blocks;
* override deterministic protocol rules;
* blacklist a node by itself.

⸻

7.9 Network Operator

A network operator deploys or monitors an experimental node.

The network operator MUST NOT possess privileged consensus authority merely because they deployed the software.

⸻

8. Operating Modes

Q1 v0.1 MUST support at least four operating modes.

Q1-SYS-007 — Single-node development mode

Purpose:

* debugging;
* wallet testing;
* transaction testing;
* block-format testing.

Requirements:

* one local node;
* deterministic block production;
* no real consensus claim;
* clearly marked as non-distributed.

⸻

Q1-SYS-008 — Local multi-node mode

Purpose:

* networking;
* synchronization;
* consensus development;
* fault simulation.

Requirements:

* minimum 4 local node processes;
* independent keys;
* separate data directories;
* configurable latency;
* configurable packet loss;
* configurable malicious behavior.

⸻

Q1-SYS-009 — Private distributed testnet

Purpose:

* testing on separate physical machines;
* observing real network conditions;
* testing HDD modules;
* energy measurement.

Requirements:

* minimum 7 independent nodes;
* minimum 3 separate physical hosts;
* no shared consensus key;
* persistent ledger;
* reproducible deployment instructions.

⸻

Q1-SYS-010 — Public experimental testnet

This mode is NOT required for the first code milestone.

It MAY be activated only after:

* security review;
* stable private testnet operation;
* reproducible builds;
* documented attack tests;
* wallet safety review.

⸻

9. Cryptographic Identity Requirements

Q1-SYS-011

Each wallet MUST have:

* a private key;
* a public key;
* a derived address.

⸻

Q1-SYS-012

Private keys MUST NOT be transmitted to any node.

⸻

Q1-SYS-013

Private keys MUST NOT be stored in plaintext by default.

⸻

Q1-SYS-014

The first implementation SHOULD use a widely reviewed digital signature algorithm.

The algorithm MUST be abstracted behind a cryptographic interface so it can later be replaced.

⸻

Q1-SYS-015

Every transaction MUST contain a valid digital signature.

⸻

Q1-SYS-016

Every candidate block MUST be signed by its producer.

⸻

Q1-SYS-017

Every validator attestation MUST be individually signed.

⸻

Q1-SYS-018

The protocol MUST reject malformed, invalid, or replayed signatures.

⸻

10. Ledger Model Requirements

Q1-SYS-019

Q1 v0.1 MUST use one clearly defined ledger model.

For implementation simplicity, the initial recommendation is an account-based model.

Each account MUST contain at least:

* address;
* balance;
* transaction nonce.

⸻

Q1-SYS-020

The ledger MUST support a native unit.

Temporary working unit name:

Q1 Test Unit

Temporary internal symbol:

Q1T

This name has no branding significance.

⸻

Q1-SYS-021

Balances MUST be represented as integers.

Floating-point arithmetic MUST NOT be used for balances, rewards, or fees.

⸻

Q1-SYS-022

The smallest unit MUST be fixed in the protocol configuration.

Recommended initial precision:

[
1\ Q1T = 100,000,000\ base\ units
]

This parameter MAY be changed before the public testnet.

⸻

Q1-SYS-023

State transitions MUST be deterministic.

⸻

Q1-SYS-024

A transaction MUST NOT create a negative balance.

⸻

Q1-SYS-025

The ledger MUST reject duplicate transaction identifiers.

⸻

Q1-SYS-026

The ledger MUST support reconstructing current state from the genesis block.

⸻

Q1-SYS-027

The ledger SHOULD support state snapshots for faster synchronization.

Snapshots MUST be verifiable against the canonical chain.

⸻

11. Genesis Requirements

Q1-SYS-028

Every network instance MUST begin from a genesis configuration.

⸻

Q1-SYS-029

The genesis configuration MUST contain:

* chain identifier;
* protocol version;
* genesis timestamp;
* initial account allocations;
* initial validator set;
* initial consensus parameters;
* block-time target;
* fee parameters;
* reward parameters;
* delay parameters;
* HDD module status;
* network name;
* genesis hash.

⸻

Q1-SYS-232

The sum of all explicit genesis allocations MUST equal the declared genesis
supply exactly.

Every allocation MUST contain:

* recipient;
* amount;
* purpose;
* lock or vesting rule, if any.

Any treasury, faucet, research, or reserve amount MUST be an explicit
allocation to a defined address. No implicit or unassigned genesis reserve is
permitted.

⸻

Q1-SYS-030

The genesis file MUST be human-readable.

Recommended format:

* JSON;
* TOML;
* or YAML.

⸻

Q1-SYS-031

All nodes on the same network MUST use the same genesis hash.

Nodes with a different genesis hash MUST refuse synchronization.

⸻

12. Transaction Requirements

Every transaction MUST include at least:

* protocol version;
* chain identifier;
* sender address;
* recipient address;
* amount;
* fee limit;
* sender nonce;
* creation timestamp or validity window;
* transaction type;
* public key or key reference;
* digital signature;
* transaction identifier.

⸻

Q1-SYS-032

A transaction MUST be uniquely identifiable by a deterministic hash.

⸻

Q1-SYS-033

A transaction MUST be rejected if:

* the signature is invalid;
* the sender balance is insufficient;
* the nonce is invalid;
* the amount is zero or negative;
* the fee is below the required minimum;
* the transaction is malformed;
* the chain identifier is incorrect;
* the transaction is expired;
* the transaction has already been confirmed.

⸻

Q1-SYS-034

The system MUST support at least the following transaction types:

1. native value transfer;
2. protocol reward;
3. genesis allocation.

⸻

Q1-SYS-035

Protocol reward transactions MUST NOT be manually created by normal users.

⸻

Q1-SYS-036

A user MUST be able to estimate the fee before signing.

⸻

Q1-SYS-037

The transaction format MUST be versioned.

⸻

Q1-SYS-038

The transaction serialization format MUST be canonical.

The same transaction data MUST always produce the same transaction hash.

⸻

13. Transaction Pool Requirements

Q1-SYS-039

Each full node MUST maintain a local transaction pool.

⸻

Q1-SYS-040

Only transactions passing preliminary validation MAY enter the pool.

⸻

Q1-SYS-041

The pool MUST prevent duplicate entries.

⸻

Q1-SYS-042

The pool MUST have configurable limits for:

* maximum transaction count;
* maximum memory use;
* maximum transaction size;
* maximum transactions per sender.

⸻

Q1-SYS-043

The pool MUST include spam resistance.

Spam controls MAY include:

* minimum fee;
* per-sender rate limits;
* peer rate limits;
* transaction-size limits;
* temporary peer penalties.

⸻

Q1-SYS-044

Transactions MUST be removed from the pool when:

* confirmed;
* expired;
* invalidated;
* replaced under explicit replacement rules;
* evicted due to resource limits.

⸻

14. Block Requirements

Every block MUST contain at least:

* protocol version;
* chain identifier;
* block height;
* previous block hash;
* block timestamp;
* transaction root;
* state root;
* producer identifier;
* producer signature;
* producer-selection proof;
* delay challenge;
* delay output;
* delay proof;
* optional HDD commitment;
* validator attestations or finalization certificate;
* fee summary;
* reward summary;
* block hash.

⸻

Q1-SYS-045

The block hash MUST be deterministically derived from the canonical block header.

⸻

Q1-SYS-046

A block MUST reference exactly one parent block.

⸻

Q1-SYS-047

A block MUST be rejected if:

* the parent is unknown;
* the producer was not eligible;
* the delay proof is invalid;
* any included transaction is invalid;
* the state root is incorrect;
* the transaction root is incorrect;
* rewards are incorrectly calculated;
* fees are incorrectly calculated;
* the producer signature is invalid;
* the block exceeds configured limits.

⸻

Q1-SYS-048

Maximum block size MUST be configurable in testnet mode.

Recommended initial value:

1 MB

This value is experimental.

⸻

Q1-SYS-049

Maximum transaction count per block MUST be configurable.

⸻

Q1-SYS-050

The protocol MUST define a target block interval.

Recommended initial experimental target:

30 seconds

The system MUST allow testing alternative values such as:

* 10 seconds;
* 20 seconds;
* 60 seconds;
* 120 seconds.

⸻

15. Block Producer Selection Requirements

Q1-SYS-233

The Q1 v0.1 private testnet MUST use a genesis-defined
`PERMISSIONED_PRIVATE_TESTNET_REGISTRY` with equal producer-eligibility and
validator weights.

The registry MUST be labeled as temporary, permissioned, and private-testnet
only. It MUST NOT be represented as public Sybil resistance or future public
participant admission.

⸻

Q1-SYS-051

Block producers MUST be selected through a deterministic and publicly verifiable procedure.

⸻

Q1-SYS-052

The selection result MUST derive from data unavailable before the previous block was finalized.

⸻

Q1-SYS-053

The selection process MUST prevent a producer from choosing its own favorable challenge.

⸻

Q1-SYS-054

The first implementation MAY use a weighted lottery.

⸻

Q1-SYS-055

The weight formula MUST be configurable.

Candidate weight inputs MAY include:

* recent availability;
* valid participation history;
* prior correct attestations;
* resource contribution;
* HDD experiment participation;
* time since last selection;
* anti-concentration factor.

⸻

Q1-SYS-056

Raw wealth MUST NOT provide unlimited linear control over selection probability.

⸻

Q1-SYS-057

The system MUST include a mechanism reducing repeated selection of the same producer.

⸻

Q1-SYS-058

The selected producer list MUST be independently computable by all full nodes.

⸻

Q1-SYS-059

More than one candidate SHOULD be selected per block round to provide fallback.

Recommended initial candidate count:

3

⸻

Q1-SYS-060

If the first candidate fails to publish before the deadline, the next candidate MUST become eligible according to deterministic rules.

⸻

16. Delay Engine Requirements

Q1-SYS-061

The delay engine MUST receive a challenge derived from finalized network data.

⸻

Q1-SYS-062

The challenge MUST include at least:

* previous finalized block hash;
* round identifier;
* producer identifier;
* protocol domain separator.

⸻

Q1-SYS-063

The delay function MUST require sequential work.

⸻

Q1-SYS-064

The delay output MUST be difficult to compute substantially faster through ordinary parallelization.

⸻

Q1-SYS-065

The delay proof MUST be significantly cheaper to verify than to produce.

⸻

Q1-SYS-066

Verification MUST be deterministic.

⸻

Q1-SYS-067

Delay difficulty MUST be configurable.

⸻

Q1-SYS-068

The system MUST record:

* delay start time;
* delay completion time;
* CPU usage;
* memory usage;
* hardware description;
* verification time;
* proof size.

Local timestamps are telemetry only and MUST NOT independently establish consensus validity.

⸻

Q1-SYS-069

The delay module MUST be replaceable.

The first prototype MAY use a simplified sequential hash chain before integrating a formal VDF.

⸻

Q1-SYS-070

A simplified delay implementation MUST be clearly labeled:

NON-SECURE RESEARCH DELAY

It MUST NOT be represented as production-grade VDF security.

⸻

17. HDD Laboratory Module Requirements

Q1-SYS-071

The HDD module MUST be implemented as an experimental plugin.

⸻

Q1-SYS-072

The core network MUST remain functional when the HDD module is disabled.

⸻

Q1-SYS-073

The HDD module MUST accept a fresh unpredictable challenge.

⸻

Q1-SYS-074

The module MAY perform combinations of:

* sequential writes;
* random reads;
* deterministic file generation;
* challenge-dependent block access;
* seek-pattern execution;
* data commitment generation;
* read-back verification.

⸻

Q1-SYS-075

The HDD response MUST be bound to:

* the current challenge;
* the participant identity;
* the current round;
* the selected data set.

⸻

Q1-SYS-076

Previously recorded HDD output MUST NOT be valid for a new challenge.

⸻

Q1-SYS-077

The module MUST record:

* device model;
* interface type;
* capacity;
* rotational speed where available;
* operating-system disk identifier;
* read throughput;
* write throughput;
* random-access latency;
* sequential-access latency;
* operation duration;
* bytes read;
* bytes written;
* estimated energy use;
* reported errors.

⸻

Q1-SYS-078

The protocol MUST NOT assume that HDD telemetry proves physical truth.

⸻

Q1-SYS-079

HDD results MUST initially be treated as one of the following:

* experimental input;
* eligibility modifier;
* reward modifier;
* anti-simulation research signal.

HDD results MUST NOT initially be the sole basis of block validity.

⸻

Q1-SYS-080

The system MUST support simulated HDD participants so that physical and virtual results can be compared.

⸻

Q1-SYS-081

The test suite MUST attempt:

* replay attacks;
* cached-result attacks;
* fake telemetry;
* SSD substitution;
* RAM-disk substitution;
* virtual-disk substitution;
* remote HDD outsourcing;
* parallel HDD execution;
* manipulated timestamps.

⸻

18. Validator Committee Requirements

Q1-SYS-082

Each round MUST have a deterministically selected validator committee.

⸻

Q1-SYS-083

The committee size MUST be configurable.

Recommended initial sizes:

* local test: 3 validators;
* private testnet: 5 validators;
* extended testnet: 7 or more validators.

⸻

Q1-SYS-084

A validator MUST independently verify the full candidate block before attesting.

⸻

Q1-SYS-085

A validator MUST NOT attest to two conflicting blocks at the same height and round.

⸻

Q1-SYS-086

Conflicting signed attestations MUST be detectable and stored as evidence.

⸻

Q1-SYS-087

Initial finalization SHOULD require at least two-thirds of committee weight.

⸻

Q1-SYS-088

Validator weight MUST NOT rely only on physical network proximity.

Geographical or network-distance diversity MAY be used as a committee-selection factor, but it MUST NOT replace cryptographic identity and signed attestations.

⸻

Q1-SYS-089

The protocol SHOULD attempt committee diversity across:

* IP ranges;
* autonomous systems;
* geographic regions;
* operating systems;
* node operators;
* device types.

These signals MUST be treated as imperfect.

⸻

19. Consensus and Finalization Requirements

Q1-SYS-090

The protocol MUST define explicit states for a block:

* proposed;
* verified;
* attested;
* finalized;
* rejected;
* orphaned.

⸻

Q1-SYS-091

A proposed block MUST NOT immediately become final.

⸻

Q1-SYS-092

Finalization MUST require a valid finalization certificate.

⸻

Q1-SYS-093

The finalization certificate MUST contain sufficient signed attestations to prove the required threshold.

⸻

Q1-SYS-094

Once a block is finalized, honest nodes MUST NOT reorganize the chain below that block under normal protocol operation.

⸻

Q1-SYS-095

Conflicting finalized blocks MUST be treated as a critical consensus failure.

⸻

Q1-SYS-096

The system MUST halt or enter safe mode if conflicting finalization certificates are detected.

⸻

Q1-SYS-097

Fork-choice rules MUST be deterministic.

⸻

Q1-SYS-098

The protocol MUST define behavior when:

* no candidate publishes;
* multiple candidates publish;
* the committee is offline;
* the network is partitioned;
* delay proofs arrive late;
* attestations conflict;
* nodes disagree on current round.

⸻

Q1-SYS-099

Liveness MUST NOT override safety.

If the system cannot safely finalize, it SHOULD stop finalization rather than accept conflicting states.

⸻

20. Time Requirements

Q1-SYS-100

Local system clocks MUST NOT be treated as the sole source of truth.

⸻

Q1-SYS-101

Block timestamps MUST be checked against protocol-defined tolerance.

⸻

Q1-SYS-102

A node with a severely incorrect clock MUST be warned and MAY be prevented from producing blocks.

⸻

Q1-SYS-103

Round progression SHOULD depend primarily on:

* finalized block data;
* delay completion;
* signed network messages;
* bounded timing windows.

⸻

Q1-SYS-104

The system MUST simulate clock manipulation attacks.

⸻

21. Fee Requirements

Q1-SYS-105

Every normal transfer MUST pay a non-negative fee.

⸻

Q1-SYS-106

Anonymous transactions MUST NOT receive a net negative fee.

⸻

Q1-SYS-107

The initial fee model MUST be understandable to users.

Recommended structure:

[
TotalFee = BaseFee + CongestionFee - LimitedDiscount
]

Subject to:

[
TotalFee \geq MinimumFee
]

⸻

Q1-SYS-108

The fee estimator MUST display the expected fee before signing.

⸻

Q1-SYS-109

The fee algorithm MUST be deterministic from public network data.

⸻

Q1-SYS-110

The fee model MUST include anti-spam protection.

⸻

Q1-SYS-111

Fee parameters MUST be configurable in testnet mode.

⸻

Q1-SYS-112

The system MUST test behavior under:

* empty blocks;
* low traffic;
* normal traffic;
* sudden congestion;
* sustained congestion;
* spam transactions;
* high-value transfers;
* many low-value transfers.

⸻

22. Reward Requirements

Q1-SYS-113

Block rewards MUST be generated only according to protocol rules.

⸻

Q1-SYS-114

Reward allocation MUST be deterministic and independently verifiable.

⸻

Q1-SYS-115

The initial reward may be divided among:

* block producer;
* delay executor;
* validator committee;
* network relayers;
* HDD participants;
* development or protocol treasury.

⸻

Q1-SYS-116

The exact percentages MUST remain configurable during simulation.

⸻

Q1-SYS-117

No actor MUST receive a reward for invalid, late, duplicate, or conflicting work.

⸻

Q1-SYS-118

A producer whose block is rejected MUST NOT receive the producer reward.

⸻

Q1-SYS-119

A validator signing conflicting blocks MUST be penalizable.

⸻

Q1-SYS-120

The prototype MAY implement virtual penalties before implementing locked collateral.

⸻

Q1-SYS-121

Reward issuance MUST be included in supply accounting.

⸻

23. Supply Requirements

Q1-SYS-122

Q1 v0.1 MUST define a test supply model.

⸻

Q1-SYS-123

The test supply model MUST record:

* genesis allocation;
* block issuance;
* fee collection;
* fee burning, if any;
* treasury allocation;
* validator rewards;
* producer rewards;
* total circulating test units.

⸻

Q1-SYS-124

The supply model MUST use integer arithmetic.

⸻

Q1-SYS-125

No final monetary policy is approved at this stage.

⸻

Q1-SYS-126

Alternative supply models MUST be testable through configuration.

Candidate simulations MAY include:

* fixed maximum supply;
* declining issuance;
* adaptive issuance;
* security-budget-based issuance;
* fee-dominant long-term issuance.

⸻

24. Networking Requirements

Q1-SYS-127

Nodes MUST communicate over authenticated or cryptographically signed messages where appropriate.

⸻

Q1-SYS-128

The network MUST support peer discovery.

⸻

Q1-SYS-129

The network MUST support manually configured peers for private testnets.

⸻

Q1-SYS-130

A node MUST maintain configurable limits for:

* inbound peers;
* outbound peers;
* message size;
* message rate;
* pending requests;
* synchronization bandwidth.

⸻

Q1-SYS-131

The network MUST detect malformed messages.

⸻

Q1-SYS-132

Nodes MUST be able to temporarily disconnect abusive peers.

⸻

Q1-SYS-133

Peer penalties MUST NOT directly change ledger state.

⸻

Q1-SYS-134

The network MUST support chain synchronization from genesis.

⸻

Q1-SYS-135

The network SHOULD support snapshot-assisted synchronization.

⸻

Q1-SYS-136

The system MUST test:

* message delay;
* packet loss;
* duplicate messages;
* reordered messages;
* network partitions;
* eclipse attempts;
* peer flooding;
* partial node outages.

⸻

25. Node Requirements

Q1-SYS-137

A node MUST have a unique node key separate from wallet spending keys.

⸻

Q1-SYS-138

A node MUST store its data in a configurable directory.

⸻

Q1-SYS-139

A node MUST support graceful shutdown.

⸻

Q1-SYS-140

A node MUST recover from ordinary restart without corrupting the ledger.

⸻

Q1-SYS-141

A node MUST detect corrupted local data.

⸻

Q1-SYS-142

A node MUST expose health information.

⸻

Q1-SYS-143

A node MUST provide structured logs.

⸻

Q1-SYS-144

A node MUST support at least:

* full-node mode;
* validator mode;
* producer mode;
* observer mode.

Modes MAY be combined on one machine during early testing.

⸻

26. Wallet Requirements

Q1-SYS-145

The first wallet MUST be a command-line wallet.

⸻

Q1-SYS-146

The wallet MUST support:

* key generation;
* address generation;
* balance query;
* transaction creation;
* fee estimation;
* transaction signing;
* transaction submission;
* transaction-status query.

⸻

Q1-SYS-147

The wallet MUST display the network name and chain identifier.

⸻

Q1-SYS-148

The wallet MUST warn users that Q1 v0.1 units have no guaranteed financial value.

⸻

Q1-SYS-149

The wallet MUST NOT automatically upload private keys.

⸻

Q1-SYS-150

The wallet SHOULD support encrypted local key storage.

⸻

Q1-SYS-151

The wallet SHOULD support offline transaction signing.

⸻

Q1-SYS-152

The wallet MUST clearly distinguish:

* pending;
* confirmed;
* finalized;
* failed;
* expired transactions.

⸻

27. Explorer Requirements

Q1-SYS-153

The explorer MUST display:

* latest blocks;
* block height;
* block hash;
* producer;
* transaction count;
* block time;
* finalization status;
* delay-proof summary;
* fees;
* rewards;
* validator attestations.

⸻

Q1-SYS-154

The explorer MUST allow lookup by:

* block height;
* block hash;
* transaction hash;
* address.

⸻

Q1-SYS-155

The explorer MUST display a clear experimental-network warning.

⸻

Q1-SYS-156

The first explorer MAY be local and minimal.

⸻

28. AI Observer Requirements

Q1-SYS-157

The AI observer MUST operate outside the deterministic consensus path.

⸻

Q1-SYS-158

Consensus MUST remain functional if the AI observer is offline.

⸻

Q1-SYS-159

The AI observer MAY analyze:

* abnormal block timing;
* repeated producer selection;
* validator collusion indicators;
* unusual peer patterns;
* HDD telemetry anomalies;
* fee manipulation;
* transaction spam;
* geographic concentration;
* energy anomalies;
* repeated failed proofs.

⸻

Q1-SYS-160

Every AI-generated alert MUST include:

* alert category;
* risk score;
* supporting data;
* model version;
* timestamp;
* explanation where available.

⸻

Q1-SYS-161

AI findings MUST be labeled as:

* informational;
* suspicious;
* high risk;
* critical.

⸻

Q1-SYS-162

An AI alert MUST NOT automatically confiscate funds, reject blocks, or alter consensus.

⸻

Q1-SYS-163

The AI observer MUST support false-positive and false-negative evaluation.

⸻

Q1-SYS-164

AI model changes MUST be versioned.

⸻

29. Telemetry Requirements

Q1-SYS-165

Telemetry MUST be enabled by default in private experimental networks.

⸻

Q1-SYS-166

Telemetry MUST be configurable.

⸻

Q1-SYS-167

Telemetry MUST NOT collect wallet private keys or seed phrases.

⸻

Q1-SYS-168

Telemetry MUST record at least:

* node identifier;
* software version;
* protocol version;
* operating system;
* CPU architecture;
* memory use;
* CPU use;
* disk use;
* network throughput;
* peer count;
* block propagation time;
* transaction propagation time;
* delay execution time;
* delay verification time;
* HDD-operation metrics;
* consensus-round duration;
* rejected transactions;
* rejected blocks;
* forks;
* validator participation;
* producer participation;
* estimated energy use.

⸻

Q1-SYS-169

Telemetry data SHOULD use structured formats.

Recommended:

* JSON Lines;
* Prometheus metrics;
* CSV exports.

⸻

Q1-SYS-170

The test environment SHOULD support visualization dashboards.

⸻

30. Energy Measurement Requirements

Q1-SYS-171

Q1 MUST NOT claim energy superiority without measurement.

⸻

Q1-SYS-172

The prototype MUST estimate or measure energy usage for:

* idle node operation;
* transaction validation;
* block production;
* delay execution;
* HDD operations;
* delay-proof verification;
* chain synchronization.

⸻

Q1-SYS-173

Energy reporting MUST include the measurement method.

⸻

Q1-SYS-174

The system SHOULD calculate:

* energy per block;
* energy per finalized transaction;
* energy per active node;
* energy spent on rejected work;
* energy spent on duplicated candidate work.

⸻

Q1-SYS-175

The system MUST compare multiple configurations, including:

* delay engine only;
* delay engine plus HDD;
* one candidate;
* multiple candidates;
* low traffic;
* high traffic.

⸻

31. Security Requirements

Q1-SYS-176

The system MUST reject unauthorized balance creation.

⸻

Q1-SYS-177

The system MUST reject double spending.

⸻

Q1-SYS-178

The system MUST prevent transaction replay across different chain identifiers.

⸻

Q1-SYS-179

The system MUST detect conflicting validator signatures.

⸻

Q1-SYS-180

The system MUST test malicious producers.

⸻

Q1-SYS-181

The system MUST test malicious validators.

⸻

Q1-SYS-182

The system MUST test Sybil-node creation.

⸻

Q1-SYS-183

The system MUST test network partitions.

⸻

Q1-SYS-184

The system MUST test manipulated local clocks.

⸻

Q1-SYS-185

The system MUST test forged HDD telemetry.

⸻

Q1-SYS-186

The system MUST test invalid delay proofs.

⸻

Q1-SYS-187

The system MUST test long-range and alternate-chain attempts where applicable.

⸻

Q1-SYS-188

Critical cryptographic failures MUST cause safe rejection, not silent acceptance.

⸻

Q1-SYS-189

Private keys, passwords, and secrets MUST NOT appear in normal logs.

⸻

32. Privacy Requirements

Q1-SYS-190

Q1 v0.1 is not a privacy blockchain.

⸻

Q1-SYS-191

Users MUST be informed that addresses, balances, and transactions may be publicly visible.

⸻

Q1-SYS-192

Telemetry MUST avoid unnecessary personal data.

⸻

Q1-SYS-193

IP addresses and geographic data SHOULD be anonymized or minimized in exported research reports.

⸻

33. Performance Requirements

Initial targets are experimental and MAY change.

Q1-SYS-194

A normal transaction SHOULD be validated locally within:

100 milliseconds

under ordinary test conditions.

⸻

Q1-SYS-195

A delay proof SHOULD be verified significantly faster than it is generated.

Initial research target:

[
VerificationTime \leq 1% \times GenerationTime
]

This is a target, not yet a guaranteed property.

⸻

Q1-SYS-196

A finalized block SHOULD propagate to most reachable testnet nodes within:

5 seconds

under normal private-testnet conditions.

⸻

Q1-SYS-197

The network SHOULD maintain operation when up to one-third of validator weight is unavailable or malicious, subject to the final consensus design.

⸻

Q1-SYS-198

The node MUST remain stable under at least:

* 10 transactions per second;
* 100 transactions per second in stress mode;
* 10,000 pending transactions;
* 100 connected simulated peers.

These are prototype stress targets.

⸻

34. Reliability Requirements

Q1-SYS-199

A node MUST restart without losing finalized ledger data.

⸻

Q1-SYS-200

Temporary network disconnection MUST NOT corrupt local state.

⸻

Q1-SYS-201

A recovering node MUST be able to synchronize with honest peers.

⸻

Q1-SYS-202

Malformed peer messages MUST NOT crash the node.

⸻

Q1-SYS-203

An HDD-module failure MUST NOT automatically corrupt the core ledger.

⸻

Q1-SYS-204

An AI-observer failure MUST NOT stop consensus.

⸻

35. Configuration Requirements

Q1-SYS-205

Experimental parameters MUST be configurable without changing source code.

⸻

Q1-SYS-206

Configurable parameters MUST include:

* chain identifier;
* block interval;
* block size;
* candidate count;
* committee size;
* finalization threshold;
* delay difficulty;
* HDD module enabled or disabled;
* HDD workload;
* fee parameters;
* reward distribution;
* transaction-pool limits;
* peer limits;
* telemetry level.

⸻

Q1-SYS-207

Consensus-critical configuration MUST be part of genesis or protocol versioning.

⸻

Q1-SYS-208

Nodes with incompatible consensus configuration MUST refuse participation.

⸻

36. Logging Requirements

Q1-SYS-209

All components MUST use structured logging.

⸻

Q1-SYS-210

Logs MUST support at least:

* debug;
* info;
* warning;
* error;
* critical.

⸻

Q1-SYS-211

Consensus-critical events MUST have stable event identifiers.

⸻

Q1-SYS-212

Each block round MUST have a traceable round identifier.

⸻

Q1-SYS-213

Logs MUST allow reconstruction of why a block or transaction was rejected.

⸻

37. API Requirements

Q1-SYS-214

The node MUST expose an API for:

* node status;
* peer count;
* latest block;
* block lookup;
* transaction lookup;
* address balance;
* transaction submission;
* fee estimation;
* telemetry query;
* validator status;
* producer status.

⸻

Q1-SYS-215

Administrative APIs MUST be separated from public read APIs.

⸻

Q1-SYS-216

Administrative APIs MUST require authentication.

⸻

Q1-SYS-217

The API MUST be versioned.

Recommended prefix:

/api/v1/

⸻

38. Development Requirements

Q1-SYS-218

The codebase MUST be organized into independent modules.

⸻

Q1-SYS-219

Consensus logic MUST be isolated from user-interface code.

⸻

Q1-SYS-220

Cryptographic operations MUST be isolated behind clear interfaces.

⸻

Q1-SYS-221

The HDD module MUST be isolated behind a plugin interface.

⸻

Q1-SYS-222

The AI observer MUST be a separate service or process.

⸻

Q1-SYS-223

Every mandatory requirement SHOULD map to:

* implementation code;
* unit tests;
* integration tests;
* or an explicit deferred-status record.

⸻

Q1-SYS-224

The project MUST support reproducible local setup.

⸻

Q1-SYS-225

The project MUST include:

* README;
* build instructions;
* test instructions;
* architecture overview;
* configuration examples;
* sample genesis file.

⸻

39. Testing Requirements

Q1-SYS-226

Every deterministic protocol rule MUST have automated tests.

⸻

Q1-SYS-227

Tests MUST include:

* unit tests;
* integration tests;
* multi-node tests;
* adversarial tests;
* performance tests;
* recovery tests;
* HDD tests;
* AI-observer evaluation tests.

⸻

Q1-SYS-228

The system MUST support deterministic test seeds.

⸻

Q1-SYS-229

A failed consensus test MUST block release.

⸻

Q1-SYS-230

The test environment MUST be able to create malicious nodes.

⸻

Q1-SYS-231

A malicious node MUST be configurable to:

* submit invalid transactions;
* propose invalid blocks;
* equivocate;
* delay messages;
* drop messages;
* replay messages;
* forge HDD telemetry;
* submit invalid delay proofs;
* flood peers;
* manipulate timestamps.

⸻

40. Minimum Viable Prototype

The first working Q1 prototype is complete only when all of the following exist:

1. four local nodes can start independently;
2. each node has its own identity;
3. a genesis block is loaded;
4. wallets can create signed transactions;
5. transactions propagate between nodes;
6. valid transactions enter the pool;
7. invalid transactions are rejected;
8. a producer is selected;
9. a delay challenge is generated;
10. the producer executes the delay module;
11. a candidate block is created;
12. validators independently verify it;
13. at least two-thirds of the committee attest;
14. the block becomes finalized;
15. all honest nodes reach the same balance state;
16. the explorer displays the block;
17. telemetry records the entire process;
18. restarting a node preserves finalized state;
19. one malicious node cannot create unauthorized funds;
20. the HDD module can be enabled and disabled without breaking the network.

⸻

41. Private Testnet Acceptance Criteria

Q1 v0.1 may be considered ready for a private distributed testnet only if:

Functional

* 1,000 sequential blocks finalize successfully;
* 10,000 valid transfers complete;
* no honest node ends with a different finalized balance state;
* restart and resynchronization succeed;
* invalid transactions are rejected;
* invalid blocks are rejected.

Consensus

* producer failure triggers fallback;
* validator failure does not corrupt state;
* conflicting proposals are handled deterministically;
* finalization certificates verify independently;
* no conflicting finalized blocks occur.

Security

* double-spend attempts fail;
* replay attempts fail;
* invalid signatures fail;
* forged reward transactions fail;
* forged delay proofs fail;
* conflicting validator signatures are detected;
* fake HDD telemetry does not independently create valid blocks.

Performance

* resource use is measured;
* block time remains within configured tolerance;
* delay generation and verification metrics are recorded;
* transaction propagation is measured;
* block propagation is measured.

Recovery

* node crash recovery succeeds;
* temporary network partition recovery succeeds;
* corrupted local database is detected;
* resynchronization from honest peers succeeds.

⸻

42. Failure Conditions

The prototype MUST be considered unsuccessful in its current form if any of the following occur repeatedly:

* honest nodes finalize different states;
* a user creates funds outside protocol rules;
* invalid delay proofs are accepted;
* HDD telemetry alone can forge block eligibility;
* one ordinary participant can cheaply create unlimited consensus identities;
* AI availability becomes necessary for consensus;
* block verification is more expensive than block production;
* ordinary nodes cannot keep up with the ledger;
* energy use rises without measurable security benefit;
* the protocol cannot recover from ordinary node failures;
* the system cannot explain why a block was accepted or rejected.

An unsuccessful result is valid research output and MUST be documented.

⸻

43. Open Research Decisions

The following questions remain intentionally unresolved:

1. Which formal VDF or sequential-delay construction will be used?
2. Does HDD participation create a measurable security benefit?
3. Should HDD participation affect eligibility, rewards, or only research telemetry?
4. What prevents large operators from creating many identities?
5. What producer-selection weighting is fairest?
6. How should reputation decay over time?
7. What validator committee size is sufficient?
8. What block interval produces the best balance?
9. What fee formula discourages spam without blocking ordinary users?
10. What long-term issuance model can maintain security?
11. Can node diversity be measured without harming privacy?
12. How should network partitions be resolved?
13. What penalty model is effective without creating plutocracy?
14. Which roles deserve reward?
15. How much duplicated work is acceptable?
16. At what point does the HDD module cease to be useful?
17. Can AI identify attacks early enough to be operationally useful?
18. How can the protocol remain understandable as complexity grows?

These questions MUST be addressed through simulation and testing, not assumptions alone.

⸻

44. Codex Implementation Rule

Codex MUST treat this document as an engineering contract.

When a requirement is ambiguous, Codex MUST:

1. identify the ambiguity;
2. record it in OPEN_DECISIONS.md;
3. choose the simplest reversible implementation;
4. isolate the assumption behind a configurable interface;
5. create a test demonstrating the assumed behavior.

Codex MUST NOT silently invent permanent consensus rules.

⸻

45. Final Requirement

The first version of Q1 MUST be understandable enough that:

* a user can send a transaction;
* a developer can run a node;
* a researcher can reproduce an experiment;
* a security analyst can attack the network;
* and every honest node can explain why a block became final.

Q1 v0.1 is not required to prove that the project has already succeeded.

It is required to make success or failure measurable.
