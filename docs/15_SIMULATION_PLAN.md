Q1 Simulation Plan

15_SIMULATION_PLAN.md

Project: Q1 Experimental Distributed Ledger
Protocol Version: 0.1
Document Version: 0.1.0
Status: Draft for Engineering, Economic, and Security Review
Classification: Experimental — Simulation and Research Use Only

⸻

1. Purpose

This document defines the Q1 simulation program.

The simulation environment SHALL allow Q1 to test protocol behavior before requiring large physical networks, expensive hardware, public participants, or real economic value.

The simulator SHALL model:

* nodes;
* operators;
* wallets;
* transactions;
* mempools;
* producers;
* validator committees;
* delay engines;
* HDD participation;
* peer-to-peer networks;
* partitions;
* latency;
* failures;
* attacks;
* fees;
* rewards;
* issuance;
* energy consumption;
* hardware costs;
* participant entry and exit;
* concentration;
* governance scenarios;
* long-term protocol behavior.

The purpose is not to produce an attractive forecast.

The purpose is to discover:

* which assumptions fail;
* which variables matter;
* which incentives create unintended behavior;
* which attacks become profitable;
* which modules add value;
* and which protocol configurations deserve physical implementation.

⸻

2. Simulation Philosophy

Q1 SHALL follow these principles.

Q1-SIM-001 — Simulation is not reality

A simulation is a model of selected behavior.

It does not prove that a public network will behave identically.

⸻

Q1-SIM-002 — Assumptions must be explicit

Every simulation SHALL disclose:

* participant assumptions;
* cost assumptions;
* network assumptions;
* hardware assumptions;
* behavioral assumptions;
* attack assumptions;
* economic assumptions.

⸻

Q1-SIM-003 — Multiple models

No important decision SHOULD rely on one simulation model or one parameter set.

⸻

Q1-SIM-004 — Adversarial agents

The simulator MUST include strategic and malicious agents, not only honest participants.

⸻

Q1-SIM-005 — Reproducibility

Every simulation run SHOULD be reproducible using:

* a model version;
* configuration;
* deterministic seed;
* software commit;
* initial state.

⸻

Q1-SIM-006 — Sensitivity over certainty

The simulator SHOULD reveal how outcomes change when assumptions change.

It MUST NOT present one output as inevitable.

⸻

3. Simulation Objectives

The Q1 simulator SHALL answer questions including:

3.1 Consensus

* How often does the network finalize successfully?
* How many rounds are needed?
* How frequently is fallback activated?
* Under what conditions does safety fail?
* Under what conditions does liveness fail?
* How does committee size affect security and speed?

3.2 Delay

* How does hardware variance affect producer dominance?
* How much work is duplicated?
* How does difficulty affect finality time?
* What generation-to-verification ratio is required?
* Does sequential activation meaningfully reduce energy?

3.3 HDD

* Does HDD participation add measurable security?
* Can SSD, RAM, or cloud storage dominate?
* Does HDD create excessive wear or energy use?
* Does HDD reward cause hardware centralization?
* Does useful storage outperform artificial datasets?

3.4 Networking

* How do latency, packet loss, and topology affect finality?
* How easily can a node be eclipsed?
* What peer count and diversity are required?
* What happens during partition and recovery?

3.5 Economics

* Are rewards sufficient for participation?
* Are fees affordable?
* Does issuance remain sustainable?
* Can fake identities extract rewards?
* Does reward variance drive pool formation?
* How concentrated do production and validation become?

3.6 Long-term behavior

* What happens after one year?
* Ten years?
* Fifty years?
* Does security become fee-dependent?
* Does supply become unstable?
* Does participation collapse or centralize?

⸻

4. Simulation Layers

Q1 SHALL use several simulation layers.

S0 — Deterministic protocol simulation
S1 — Discrete-event network simulation
S2 — Agent-based economic simulation
S3 — Hardware and energy simulation
S4 — Adversarial simulation
S5 — Long-horizon system simulation
S6 — Hybrid simulation with physical nodes

⸻

5. S0 — Deterministic Protocol Simulation

Purpose:

* verify protocol state transitions;
* execute blocks without real networking;
* test producer and committee selection;
* test fees and rewards;
* verify exact state agreement.

S0 SHALL model:

* genesis;
* participant registry;
* transactions;
* block creation;
* delay completion events;
* attestations;
* finalization;
* rewards;
* supply.

S0 MUST be deterministic for a fixed seed and configuration.

⸻

6. S1 — Discrete-Event Network Simulation

S1 SHALL model time as ordered events rather than real wall-clock execution.

Possible events include:

NODE_START
NODE_STOP
PEER_CONNECT
PEER_DISCONNECT
MESSAGE_SENT
MESSAGE_DELIVERED
MESSAGE_DROPPED
TRANSACTION_CREATED
TRANSACTION_PROPAGATED
PROPOSAL_CREATED
PROPOSAL_DELIVERED
ATTESTATION_CREATED
ATTESTATION_DELIVERED
QUORUM_REACHED
BLOCK_FINALIZED
ROUND_TIMEOUT
PARTITION_STARTED
PARTITION_HEALED

The engine MUST preserve deterministic event ordering under a fixed seed.

⸻

7. S2 — Agent-Based Economic Simulation

Each participant SHALL be represented as an agent with configurable objectives and constraints.

An agent MAY decide to:

* join;
* leave;
* remain online;
* become a producer;
* become a validator;
* acquire better hardware;
* join a pool;
* split into several identities;
* censor transactions;
* attack;
* cooperate;
* hold or transfer units;
* change strategy based on profitability.

⸻

8. S3 — Hardware and Energy Simulation

S3 SHALL estimate:

* delay execution time;
* verification time;
* HDD workload time;
* energy consumption;
* hardware cost;
* device wear;
* bandwidth cost;
* storage growth.

Hardware profiles MUST be based on measured or explicitly assumed values.

⸻

9. S4 — Adversarial Simulation

S4 SHALL model attackers attempting to:

* double spend;
* dominate committees;
* dominate producer selection;
* eclipse nodes;
* partition the network;
* flood transactions;
* manipulate fees;
* farm rewards;
* fake HDD participation;
* exploit delay hardware;
* withhold blocks;
* censor transactions;
* collude.

⸻

10. S5 — Long-Horizon Simulation

S5 SHALL compress long periods of protocol activity.

Recommended horizons:

1 day
1 month
1 year
10 years
50 years
100 years

Long-horizon results MUST clearly state simplifications.

⸻

11. S6 — Hybrid Simulation

Hybrid simulation SHALL connect simulated participants to real Q1 node processes.

Examples:

* 5 physical nodes plus 1,000 simulated peers;
* real consensus nodes plus simulated transaction users;
* real HDD device plus simulated HDD competitors;
* real wallet plus simulated network load.

Hybrid simulation helps identify differences between abstract models and actual code.

⸻

12. Simulator Architecture

The simulator SHOULD contain:

Q1 Simulator
├── Scenario Loader
├── Simulation Clock
├── Event Queue
├── Randomness Manager
├── Node Agent Model
├── Operator Model
├── User and Wallet Model
├── Network Topology Model
├── Transaction Generator
├── Mempool Model
├── Consensus Model
├── Delay Model
├── HDD Model
├── Economic Model
├── Energy Model
├── Attack Model
├── Governance Model
├── Metrics Collector
├── Result Analyzer
├── Visualization Exporter
└── Report Generator

⸻

13. Simulation Clock

The simulator SHALL use a virtual clock.

Time units MAY include:

microseconds
milliseconds
seconds
blocks
rounds
epochs
days
years

The simulation clock MUST be independent of host wall-clock time.

⸻

14. Event Queue

The event queue MUST provide:

* deterministic ordering;
* bounded memory behavior;
* event cancellation;
* event priority;
* event tracing;
* reproducible scheduling.

Events sharing the same simulated time MUST use an explicit deterministic tie-breaker.

⸻

15. Randomness Manager

All simulation randomness SHALL derive from a recorded seed.

The Randomness Manager SHOULD provide separate deterministic streams for:

* participant selection;
* transaction generation;
* network latency;
* failure events;
* attack behavior;
* hardware variance;
* economic behavior.

This prevents one model change from unintentionally altering all random outcomes.

⸻

16. Simulation Scenario

Every scenario SHALL use a structured definition.

SimulationScenario {
    scenario_id
    scenario_version
    title
    purpose
    model_versions
    random_seed
    duration
    initial_state
    participant_configuration
    network_configuration
    consensus_configuration
    delay_configuration
    hdd_configuration
    economic_configuration
    attack_configuration
    metrics_configuration
    stop_conditions
}

⸻

17. Node Agent Model

A node agent SHOULD contain:

NodeAgent {
    node_id
    operator_id
    roles[]
    online_state
    protocol_version
    software_client
    hardware_profile
    location_region_optional
    network_provider
    peer_connections[]
    ledger_state
    mempool_state
    consensus_state
    economic_state
    behavior_strategy
}

⸻

18. Operator Model

Several node identities MAY belong to one operator.

Operator {
    operator_id
    controlled_nodes[]
    controlled_wallets[]
    capital
    energy_cost
    hardware_budget
    attack_budget
    strategy
    risk_tolerance
    geographic_regions[]
}

The operator model is necessary because:

Many nodes do not necessarily mean many independent people.

⸻

19. User Model

A user agent MAY have:

* wallet balance;
* transaction frequency;
* transfer-size distribution;
* fee sensitivity;
* patience;
* network access;
* preferred node;
* probability of leaving;
* probability of losing keys;
* probability of making errors.

⸻

20. Participant Behavior Classes

The simulator SHALL support:

HONEST_ALWAYS_ON
HONEST_INTERMITTENT
PROFIT_MAXIMIZER
LOW_COST_HOBBYIST
LARGE_OPERATOR
POOL_OPERATOR
CENSORING_OPERATOR
SYBIL_OPERATOR
MALICIOUS_PRODUCER
MALICIOUS_VALIDATOR
HDD_FRAUD_OPERATOR
NETWORK_ATTACKER
PASSIVE_USER
ACTIVE_USER
SPECULATIVE_HOLDER

⸻

21. Participant Entry and Exit

Agents SHALL be able to join or leave based on:

* expected reward;
* realized reward;
* operating cost;
* reward variance;
* hardware cost;
* network reliability;
* protocol changes;
* token price assumption;
* personal strategy.

This behavior MUST be configurable.

⸻

22. Hardware Profiles

Hardware profiles SHOULD include:

LOW_END_PHONE
OLD_LAPTOP
MODERN_LAPTOP
DESKTOP
WORKSTATION
SERVER
GPU_SYSTEM
SPECIALIZED_DELAY_DEVICE
CONSUMER_HDD
ENTERPRISE_HDD
SSD
NVME
RAM_DISK
CLOUD_STORAGE

A profile MAY define:

delay_speed
verification_speed
cpu_power
idle_power
storage_power
memory
network_bandwidth
hardware_cost
failure_rate

⸻

23. Hardware Calibration

Simulation values SHOULD be calibrated from:

* Q1 benchmark results;
* published hardware specifications;
* measured local tests;
* documented assumptions.

Every simulated profile MUST identify its source type:

MEASURED
PUBLISHED_SPECIFICATION
ESTIMATED
HYPOTHETICAL

⸻

24. Network Topology Models

The simulator SHALL support:

FULL_MESH
RANDOM_GRAPH
SMALL_WORLD
SCALE_FREE
REGION_CLUSTERED
PROVIDER_CLUSTERED
HUB_AND_SPOKE
ADVERSARIAL_ECLIPSE
CUSTOM

⸻

25. Network Link Model

Each link MAY define:

latency_distribution
bandwidth
packet_loss
duplication_rate
reordering_rate
connection_failure_rate

Links MAY be asymmetric.

⸻

26. Geographic and Provider Clustering

The simulator SHOULD model:

* regions;
* data centers;
* ISPs;
* autonomous systems;
* shared infrastructure.

The purpose is to measure correlated failure and apparent versus real decentralization.

⸻

27. Transaction Generation

The transaction generator SHALL support:

* constant-rate traffic;
* burst traffic;
* daily cycles;
* exponential interarrival;
* Poisson process;
* heavy-tailed transfer values;
* fixed values;
* spam;
* circular volume;
* transaction splitting;
* high-value settlement.

⸻

28. Transaction Value Distribution

Candidate distributions MAY include:

UNIFORM
LOG_NORMAL
PARETO
FIXED
CUSTOM_HISTOGRAM

Simulation values MUST remain base-unit integers.

⸻

29. Fee-Sensitive Users

User agents MAY:

* delay a transaction;
* increase fee limit;
* cancel or replace;
* leave the network;
* split transfers;
* use another payment system

based on fee levels.

This allows fee rules to affect behavior rather than merely accounting.

⸻

30. Mempool Model

The mempool model SHALL include:

* capacity;
* fee priority;
* sender nonce ordering;
* expiration;
* replacement;
* local variation;
* propagation delay;
* eviction.

Different nodes MAY have different mempool views.

⸻

31. Consensus Model

The simulation SHALL model:

* height;
* round;
* producer candidate list;
* committee;
* production window;
* delay execution;
* proposal;
* validation;
* attestation;
* threshold;
* finalization;
* timeout;
* fallback;
* safe mode.

The simulator MUST use the same conceptual state machine as 05_CONSENSUS.md.

⸻

32. Producer Selection Model

Producer selection experiments SHALL vary:

* equal probability;
* bounded weighted probability;
* uptime factor;
* cooldown;
* reputation;
* resource contribution;
* identity count;
* operator concentration.

Metrics MUST distinguish participant ID from operator ID.

⸻

33. Committee Model

Committee experiments SHALL vary:

committee size
equal versus bounded weight
selection frequency
producer exclusion
operator diversity
geographic diversity
availability
Byzantine percentage

⸻

34. Delay Model

The delay model SHALL support:

MOCK
SEQUENTIAL_HASH
CHECKPOINTED_HASH
FORMAL_VDF_PROFILE
HDD_COMPOSITE

The model MAY use measured distributions rather than execute real proofs during large simulations.

⸻

35. Delay Time Model

Delay time MAY be represented as:

DelayTime =
BaseDifficulty
/
HardwareSpeed
× NoiseFactor
× ThrottlingFactor

The exact model MUST be documented.

⸻

36. Verification Cost Model

The simulator SHALL separately model:

* producer generation cost;
* validator verification cost;
* rejected-proof cost;
* proof propagation cost.

A delay system that reduces producer waste but overwhelms validators SHALL be considered unsuccessful.

⸻

37. Candidate Execution Modes

The simulator SHALL compare:

SEQUENTIAL_ACTIVATION
STAGGERED_PREPARATION
PARALLEL_EXECUTION

Metrics:

* finality latency;
* duplicated energy;
* fallback speed;
* producer advantage;
* wasted work.

⸻

38. HDD Model

The HDD model SHALL represent:

* dataset size;
* chunk size;
* sample count;
* random-read latency;
* sequential throughput;
* cache state;
* device class;
* device count;
* energy;
* wear;
* fraud mode.

⸻

39. HDD Fraud Modes

Simulation SHALL support:

HONEST_HDD
SSD_SUBSTITUTION
RAM_CACHE
FULL_RAM_DATASET
REMOTE_STORAGE
VIRTUAL_DISKS
FAKE_TELEMETRY
DATASET_DUPLICATION
MULTI_IDENTITY_DEVICE

⸻

40. Energy Model

Energy SHALL be estimated separately for:

* idle nodes;
* transaction validation;
* delay generation;
* delay verification;
* HDD workload;
* message propagation;
* synchronization;
* rejected work;
* duplicate candidates.

⸻

41. Energy Units

Recommended unit:

watt-hours

Reports MAY also use:

kilowatt-hours
joules

Conversions MUST be explicit.

⸻

42. Hardware Wear Model

The simulator MAY estimate:

* HDD operating hours;
* spin-up cycles;
* bytes written;
* seek-intensive operations;
* SSD write volume;
* expected replacement period;
* hardware disposal.

Wear estimates MUST be labeled as approximate unless calibrated.

⸻

43. Economic Model

The simulator SHALL implement the rules in 10_TOKENOMICS.md.

It SHALL track:

* balances;
* supply;
* fees;
* rewards;
* burns;
* treasury;
* participant revenue;
* participant cost;
* profit;
* entry and exit;
* concentration.

⸻

44. External Price Assumptions

Market price MAY be introduced only as a simulation variable.

It MUST NOT be presented as a Q1 protocol output.

Candidate models:

FIXED_PRICE
RANDOM_WALK
VOLATILE_CYCLE
ADOPTION_LINKED
SECURITY_CONFIDENCE_LINKED
EXOGENOUS_SERIES

Every price model is hypothetical.

⸻

45. Participant Profitability

For each participant:

NetReturn =
Rewards
+
FeesReceived
-
EnergyCost
-
HardwareAmortization
-
BandwidthCost
-
StorageCost
-
Penalties

The simulator SHOULD track both participant and operator-level profitability.

⸻

46. Reward Variance

Metrics SHALL include:

* mean reward;
* median reward;
* variance;
* standard deviation;
* longest time without reward;
* percentage of participants never rewarded;
* pool incentive.

⸻

47. Pool Formation Model

Agents MAY join a pool when:

* individual reward variance is high;
* pool fees are acceptable;
* expected payout becomes more stable.

The simulator SHALL measure whether protocol design indirectly encourages centralized pools.

⸻

48. Sybil Model

The simulator SHALL allow one operator to create many identities.

Costs MAY include:

* registration cost;
* collateral;
* hardware;
* bandwidth;
* operational overhead;
* no cost.

Scenarios with zero identity cost are essential.

⸻

49. Attack Cost Model

For each attack, the simulator SHOULD estimate:

capital cost
operating cost
opportunity cost
expected reward loss
expected attack gain
probability of detection
penalty exposure

⸻

50. Rational Attack Decision

A profit-motivated attacker MAY attack when:

ExpectedAttackGain
>
ExpectedAttackCost
+
ExpectedPenalty
+
OpportunityCost

Behavioral and ideological attackers MAY ignore profitability.

⸻

51. Attack Scenarios

Simulation MUST include:

1. double-spend attempt;
2. producer withholding;
3. producer equivocation;
4. validator equivocation;
5. committee capture;
6. Sybil creation;
7. eclipse;
8. partition;
9. transaction flood;
10. fee manipulation;
11. censorship;
12. HDD fraud;
13. delay hardware dominance;
14. reward farming;
15. treasury capture;
16. participant collusion;
17. software monoculture failure;
18. mass node outage.

⸻

52. Governance Model

Although final governance is not yet defined, long-horizon simulation MAY model:

* parameter voting;
* foundation control;
* developer control;
* validator voting;
* token-weighted voting;
* one-participant-one-vote assumptions;
* emergency intervention;
* governance inactivity.

Governance results MUST be labeled hypothetical.

⸻

53. Protocol Upgrade Simulation

The simulator SHOULD test:

* activation height;
* mixed software versions;
* non-upgrading nodes;
* economic rule changes;
* delay-engine changes;
* participant-set changes;
* chain split;
* upgrade coordination failure.

⸻

54. Long-Term Supply Simulation

The simulator SHALL calculate:

* total issued supply;
* burned supply;
* circulating supply;
* annual issuance;
* inflation;
* fee revenue;
* security budget;
* treasury balance.

⸻

55. Long-Term Security Simulation

The simulator SHALL compare:

issuance-dominant security
fee-dominant security
hybrid security
adaptive issuance

It MUST measure whether participant incentives survive when issuance changes.

⸻

56. Simulation Metrics

The Metrics Collector SHALL record at least:

Consensus

* finalized blocks;
* failed rounds;
* average rounds per block;
* fallback rate;
* time to finality;
* conflicting finality;
* safe-mode events.

Network

* propagation latency;
* peer diversity;
* partition duration;
* message volume;
* dropped messages;
* isolated nodes.

Economic

* fees;
* issuance;
* rewards;
* participant profit;
* operator profit;
* supply;
* concentration.

Energy

* total energy;
* energy per block;
* energy per transaction;
* rejected-work energy;
* duplicate-work energy.

Security

* attack success rate;
* attack cost;
* detection rate;
* evidence generated;
* impact duration.

⸻

57. Concentration Metrics

The simulator SHALL calculate:

Top-1 share
Top-5 share
Top-10 share
Gini coefficient
Herfindahl-Hirschman Index
Nakamoto-style minimum operator count

Concentration MUST be calculated for:

* block production;
* validator influence;
* rewards;
* node operation;
* hardware ownership;
* network providers;
* pools.

⸻

58. Decentralization Distinction

The simulator MUST distinguish:

node count
participant identity count
operator count
independent infrastructure count
independent economic-control count

A network with 10,000 nodes controlled by three operators is not highly decentralized.

⸻

59. Safety Metrics

Safety metrics SHALL include:

* finalized divergence count;
* invalid finalized blocks;
* unauthorized supply events;
* double-spend finalizations;
* quorum violations;
* replay acceptance;
* deterministic-state disagreement.

Any nonzero critical safety event SHALL be prominently reported.

⸻

60. Liveness Metrics

Liveness metrics SHALL include:

* percentage of time finalizing;
* block interval distribution;
* round timeout rate;
* quorum failure rate;
* participant availability;
* recovery time after partition;
* recovery time after mass outage.

⸻

61. User Experience Metrics

Simulation MAY calculate:

* transaction wait time;
* fee paid;
* fee as percentage of transfer;
* expiration rate;
* replacement rate;
* transaction failure rate;
* user abandonment rate.

⸻

62. Simulation Stop Conditions

A scenario MUST define stop conditions such as:

* simulation duration reached;
* critical safety invariant failed;
* event count exceeded;
* all participants exited;
* supply mismatch;
* memory or resource limit;
* target number of blocks reached.

⸻

63. Critical Early Stop

The simulator SHOULD stop immediately and preserve state if:

* conflicting finality occurs;
* unauthorized issuance occurs;
* deterministic nodes diverge unexpectedly;
* supply conservation fails;
* impossible negative balance occurs.

⸻

64. Parameter Sweeps

The simulator SHALL support parameter sweeps.

Examples:

committee_size = 3, 5, 7, 11, 21
candidate_count = 1, 2, 3, 5
Byzantine_fraction = 0% to 50%
block_interval = 5s to 120s
delay_time = 1s to 120s
fee_multiplier = several bounded ranges
HDD_reward_share = 0% to configured cap

⸻

65. Monte Carlo Runs

Scenarios involving randomness SHOULD run many seeds.

Recommended minimums:

early development: 10 runs
research comparison: 100 runs
high-confidence comparison: 1,000 or more runs

The report MUST show distributions, not only averages.

⸻

66. Sensitivity Analysis

The simulator SHALL determine which assumptions most affect outcomes.

Methods MAY include:

* one-factor-at-a-time;
* factorial experiments;
* Latin hypercube sampling;
* variance-based sensitivity;
* scenario envelopes.

⸻

67. Confidence Intervals

Where statistically meaningful, reports SHOULD include:

* mean;
* median;
* percentiles;
* confidence intervals;
* sample count;
* variance.

A precise number without uncertainty SHOULD be avoided.

⸻

68. Calibration

Simulation SHALL be calibrated against real tests.

Calibration data MAY include:

* transaction validation benchmarks;
* network latency measurements;
* delay benchmarks;
* HDD benchmarks;
* energy measurements;
* node restart times;
* database growth.

⸻

69. Validation Against Physical Testnet

The project SHALL compare simulation predictions with private-testnet measurements.

Differences MUST be documented.

The simulator SHOULD be updated when real behavior materially differs.

⸻

70. Scenario Library

The repository SHOULD contain:

simulator/scenarios/
├── baseline/
├── consensus/
├── networking/
├── delay/
├── hdd/
├── economics/
├── attacks/
├── long_horizon/
└── governance/

⸻

71. Required Baseline Scenarios

Baseline 1 — Honest local network

* 7 nodes;
* no malicious agents;
* normal latency;
* stable traffic.

Baseline 2 — Heterogeneous hardware

* mixed ordinary devices;
* unequal delay speeds;
* no attackers.

Baseline 3 — Low traffic

* mostly empty blocks;
* low fee revenue.

Baseline 4 — High traffic

* sustained block congestion.

⸻

72. Required Consensus Scenarios

1. one candidate;
2. three candidates;
3. equal validators;
4. one validator offline;
5. one-third Byzantine;
6. more than one-third Byzantine;
7. repeated fallback;
8. repeated round changes;
9. committee concentration;
10. participant churn.

⸻

73. Required Network Scenarios

1. low latency;
2. high latency;
3. packet loss;
4. regional clustering;
5. provider outage;
6. validator eclipse;
7. minority partition;
8. majority partition;
9. long partition;
10. recovery.

⸻

74. Required Delay Scenarios

1. equal hardware;
2. heterogeneous hardware;
3. specialized hardware;
4. low difficulty;
5. high difficulty;
6. sequential candidate activation;
7. parallel candidates;
8. proof verification flood;
9. previous-producer grinding;
10. formal VDF profile.

⸻

75. Required HDD Scenarios

1. HDD telemetry only;
2. HDD disabled;
3. HDD delay input;
4. HDD reward;
5. SSD substitution;
6. RAM substitution;
7. cloud storage;
8. duplicated datasets;
9. one operator with many disks;
10. useful archival storage.

⸻

76. Required Economic Scenarios

1. fixed issuance;
2. perpetual bounded issuance;
3. fee-only security;
4. hybrid security;
5. partial fee burn;
6. high treasury share;
7. low token-price assumption;
8. high token-price assumption;
9. Sybil reward farming;
10. pool formation;
11. participant exit;
12. long-term security decline.

⸻

77. Required Composite Scenarios

Scenario C1 — Large operator expansion

One operator:

* purchases faster hardware;
* creates many identities;
* joins several roles;
* starts a pool;
* controls one provider cluster.

Measure all resulting concentration.

Scenario C2 — Economic downturn

* external price assumption declines;
* energy cost rises;
* small participants exit;
* fee demand remains low.

Scenario C3 — Sudden adoption

* transaction demand rises rapidly;
* fees increase;
* node bandwidth grows;
* storage expands;
* rewards increase.

Scenario C4 — Coordinated attack

* producer cartel;
* validator collusion;
* eclipse;
* transaction censorship;
* fee manipulation.

⸻

78. Counterfactual Comparison

Every major Q1 innovation SHOULD be tested against a simpler alternative.

Examples:

Q1 with HDD vs Q1 without HDD
three candidates vs one candidate
AI Observer vs deterministic rules only
adaptive fees vs fixed fees
bounded issuance vs fixed cap

If a complex mechanism performs no better than the simpler alternative, removal SHOULD be considered.

⸻

79. Simulation Report Structure

Each simulation report SHOULD contain:

report_id
scenario
model versions
seed range
assumptions
parameters
run count
results
distributions
critical events
sensitivity analysis
limitations
interpretation
recommendation

⸻

80. Recommendation Values

KEEP_CONFIGURATION
KEEP_WITH_RESTRICTIONS
ADJUST_PARAMETERS
REPEAT_WITH_MORE_DATA
REDESIGN_COMPONENT
REMOVE_COMPONENT
INCONCLUSIVE

⸻

81. Visualization Exports

The simulator MAY export:

* CSV;
* JSON;
* Parquet;
* static charts;
* interactive dashboards;
* event timelines;
* network graphs;
* concentration reports.

Visualization is explanatory.

Raw data MUST remain available.

⸻

82. Simulation API

A simulation API MAY expose:

POST /simulation/v1/runs
GET  /simulation/v1/runs/{id}
POST /simulation/v1/runs/{id}/stop
GET  /simulation/v1/runs/{id}/metrics
GET  /simulation/v1/runs/{id}/report

This API MUST not affect a live Q1 network.

⸻

83. Command-Line Interface

Suggested commands:

q1-sim run
q1-sim sweep
q1-sim monte-carlo
q1-sim compare
q1-sim calibrate
q1-sim attack
q1-sim report
q1-sim export

⸻

84. Simulator Performance

The simulator SHOULD support:

1,000 simulated nodes
10,000 simulated nodes
100,000 logical users
millions of simulated transactions
multi-year compressed runs

These are aspirational performance targets.

Correctness takes priority over scale.

⸻

85. Approximation Modes

To reach large scale, the simulator MAY offer:

FULL_DETAIL
AGGREGATED
STATISTICAL
HYBRID

Reports MUST identify which approximation mode was used.

⸻

86. Full-Detail Mode

Models individual:

* nodes;
* messages;
* transactions;
* attestations.

Best for small and adversarial scenarios.

⸻

87. Aggregated Mode

Models groups rather than each individual object.

Best for:

* long-term economics;
* large user populations;
* supply projections.

⸻

88. Statistical Mode

Uses distributions rather than explicit events.

Best for:

* parameter exploration;
* approximate long-horizon analysis.

⸻

89. Hybrid Mode

Uses detailed simulation for critical nodes and aggregated models for large background populations.

⸻

90. Simulation Integrity

The simulator MUST not silently change assumptions between compared runs.

Configuration and model hashes SHOULD be included in reports.

⸻

91. Simulator Tests

The simulator itself MUST be tested.

Required tests:

1. deterministic seed reproduction;
2. event ordering;
3. event cancellation;
4. random-stream separation;
5. supply conservation;
6. consensus-state correctness;
7. agent entry and exit;
8. attack activation;
9. metric accuracy;
10. report generation;
11. parameter sweep;
12. Monte Carlo aggregation;
13. long-horizon stability;
14. calibration import;
15. hybrid-node integration.

⸻

92. Simulation Acceptance Criteria

The simulator is ready for protocol decision support when:

1. identical seeds reproduce identical logical results;
2. supply and reward accounting remain exact;
3. honest baseline scenarios match protocol invariants;
4. network partitions behave as expected;
5. malicious agents can be configured;
6. operator concentration is measurable;
7. energy and cost assumptions are explicit;
8. parameter sweeps work;
9. Monte Carlo reports distributions;
10. physical benchmarks can calibrate model inputs;
11. raw results are exportable;
12. scenario limitations are visible.

⸻

93. Criteria for Trusting a Simulation Result

A result MAY inform engineering decisions only when:

* the scenario is relevant;
* assumptions are documented;
* the run is reproducible;
* sample size is adequate;
* sensitivity is understood;
* critical inputs are calibrated;
* alternative models have been compared;
* results do not contradict physical evidence without explanation.

⸻

94. Criteria for Rejecting a Simulation Result

A result SHOULD NOT guide protocol decisions when:

* assumptions are hidden;
* one seed is used;
* output is only an average;
* the model is uncalibrated;
* the scenario excludes rational attackers;
* operator ownership is ignored;
* external price assumptions are presented as fact;
* results cannot be reproduced;
* the model contradicts known implementation behavior.

⸻

95. Known Limitations

Simulation cannot fully reproduce:

* human culture;
* political intervention;
* unknown hardware innovation;
* public speculation;
* legal restrictions;
* undiscovered software exploits;
* black-market coordination;
* nation-state network control;
* decades of governance conflict.

Simulation reduces uncertainty.

It does not eliminate it.

⸻

96. Open Decisions

The following remain unresolved:

1. primary simulation language;
2. event-engine framework;
3. agent-model framework;
4. performance target;
5. large-scale approximation design;
6. model-calibration format;
7. hardware benchmark database;
8. network latency datasets;
9. participant behavior parameters;
10. pool formation model;
11. external price scenario policy;
12. governance simulation scope;
13. long-term storage model;
14. attack-cost model;
15. confidence-interval standard;
16. visualization framework;
17. hybrid integration protocol;
18. public release of simulation datasets;
19. independent model validation;
20. criteria for using simulation to change protocol parameters.

All decisions MUST be recorded in OPEN_DECISIONS.md.

⸻

97. Codex Implementation Rules

Codex MUST:

1. separate simulation code from live consensus code;
2. use deterministic seeds;
3. version every model;
4. record all scenario assumptions;
5. support honest and malicious agents;
6. distinguish node identity from operator identity;
7. implement exact integer monetary accounting;
8. support parameter sweeps;
9. support Monte Carlo runs;
10. export raw results;
11. report distributions, not only averages;
12. support physical benchmark calibration;
13. include energy and hardware cost;
14. support HDD fraud modes;
15. support network partition and eclipse models;
16. preserve critical simulation failures;
17. provide structured reports;
18. compare complex mechanisms with simpler alternatives;
19. mark hypothetical market-price inputs clearly;
20. maintain simulator regression tests.

Codex MUST NOT:

* connect simulation authority to a live network;
* modify real balances;
* present assumed token price as fact;
* hide model limitations;
* equate node count with operator count;
* use floating-point arithmetic for exact supply accounting;
* treat one simulation run as proof;
* tune parameters only to produce desirable outputs;
* omit attack agents from security conclusions;
* claim environmental superiority from uncalibrated estimates.

⸻

98. Final Simulation Principle

Q1 simulation MUST preserve this principle:

Before asking the world to spend electricity, buy hardware, trust software, or assign value, Q1 must first spend imagination.

The simulator exists to let the project experience:

* congestion without harming users;
* attacks without losing funds;
* centralization before it becomes permanent;
* fifty years of issuance before fifty years pass;
* thousands of failures before public deployment;
* and the removal of weak ideas before they become expensive beliefs.

Q1 succeeds in simulation when it can identify not only the configuration that performs best, but also:

* why it performs;
* which assumptions support it;
* how fragile it is;
* who benefits;
* who is excluded;
* how it fails;
* and whether a simpler system would perform better.