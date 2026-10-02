Q1 Tokenomics Specification

10_TOKENOMICS.md

Project: Q1 Experimental Distributed Ledger
Protocol Version: 0.1
Document Version: 0.1.0
Status: Draft for Economic and Engineering Review
Classification: Experimental — Not for Production, Investment, or Financial Use

⸻

1. Purpose

This document defines the experimental economic model of Q1 v0.1.

It specifies:

* the native test unit;
* supply accounting;
* genesis allocation;
* block issuance;
* transaction fees;
* congestion pricing;
* reward distribution;
* the security budget;
* penalties;
* treasury accounting;
* anti-spam rules;
* concentration limits;
* economic simulation requirements;
* economic success and failure criteria.

Q1 v0.1 does not establish a final monetary policy.

Its purpose is to create a configurable economic laboratory in which alternative models can be implemented, measured, attacked, compared, and removed.

No rule in this document constitutes:

* an investment promise;
* a price forecast;
* a guarantee of scarcity;
* a guarantee of profitability;
* a commitment to public issuance;
* or a representation that Q1 Test Units possess monetary value.

⸻

2. Economic Philosophy

Q1 economics SHALL be designed around the following principle:

Security work must receive compensation, but compensation must not reward waste, permanent concentration, fabricated participation, or activity that provides no measurable value to the network.

Q1 seeks balance among:

1. network security;
2. affordable ordinary transfers;
3. participant incentives;
4. resistance to spam;
5. bounded concentration;
6. predictable monetary accounting;
7. long-term economic sustainability;
8. understandable user experience.

No single objective may be optimized without measuring its effect on the others.

⸻

3. Economic Scope of v0.1

Q1 v0.1 SHALL support:

* a native test unit;
* genesis allocations;
* protocol issuance;
* transaction fees;
* fee collection;
* fee distribution;
* optional fee burning;
* treasury allocation;
* producer rewards;
* delay-executor rewards;
* validator rewards;
* optional network-service rewards;
* virtual penalties;
* full supply accounting;
* configurable economic scenarios;
* economic telemetry;
* deterministic replay of reward calculations.

Q1 v0.1 SHALL NOT include:

* public token sale;
* exchange listing;
* fiat purchase;
* market-making;
* lending;
* staking derivatives;
* leveraged trading;
* stablecoin mechanisms;
* guaranteed yield;
* dividend rights;
* equity rights;
* legal ownership rights in the project;
* automatic market-price intervention.

⸻

4. Native Test Unit

The temporary native asset SHALL be:

Asset Name: Q1 Test Unit
Asset Symbol: Q1T
Base Unit: q1u

Recommended precision:

1 Q1T = 100,000,000 q1u

All economic calculations MUST use base-unit integers.

Floating-point arithmetic MUST NOT be used in consensus-critical monetary calculations.

⸻

5. Economic Actors

Q1 economics MAY compensate the following roles.

5.1 User

Submits transactions and pays applicable fees.

⸻

5.2 Block Producer

Constructs and publishes a valid candidate block.

⸻

5.3 Delay Executor

Performs the required sequential delay process.

In v0.1, the block producer and delay executor MAY be the same participant.

The reward system MUST permit later separation.

⸻

5.4 Validator

Independently verifies a proposal and submits a valid attestation.

⸻

5.5 Finalization Contributor

Provides a valid attestation included in the finalization certificate.

In the first implementation, validator and finalization contributor are normally the same role.

⸻

5.6 HDD Participant

Performs optional experimental HDD work.

In the initial recommended mode:

HDD_MODE = TELEMETRY_ONLY

Therefore:

HDD consensus reward = 0

unless a separate experiment explicitly enables a reward modifier.

⸻

5.7 Relay or Availability Participant

May provide:

* message propagation;
* archival data;
* snapshots;
* block availability;
* synchronization services.

These roles SHALL NOT receive protocol rewards in the first milestone unless their contribution becomes objectively measurable.

⸻

5.8 Protocol Treasury

Receives a deterministic, publicly visible allocation for:

* development;
* testing;
* audits;
* research;
* infrastructure;
* community grants.

The treasury MUST NOT have authority to create units outside protocol rules.

⸻

6. Economic State

The protocol SHALL maintain or derive:

EconomicState {
    total_issued_supply
    total_burned_supply
    total_circulating_supply
    treasury_balance
    cumulative_fees_collected
    cumulative_fees_burned
    cumulative_rewards_paid
    current_issuance_parameters
    current_fee_parameters
    current_reward_parameters
    economic_rule_version
}

All fields MUST be reproducible from finalized history.

⸻

7. Supply Accounting

For every finalized block:

NewTotalIssuedSupply
=
PreviousTotalIssuedSupply
+
AuthorizedBlockIssuance
+
OtherAuthorizedIssuance
NewTotalBurnedSupply
=
PreviousTotalBurnedSupply
+
AuthorizedBurn
TotalCirculatingSupply
=
TotalIssuedSupply
-
TotalBurnedSupply

Ordinary transfers MUST NOT alter total supply.

⸻

8. Monetary Conservation Invariant

The following relationship MUST hold:

SumOfAllAccountBalances
+
ProtocolHeldBalances
=
TotalCirculatingSupply

Protocol-held balances MAY include:

* treasury balance;
* locked experimental collateral;
* pending protocol distributions;
* explicitly defined escrow balances.

Unaccounted disappearance or creation of units is a critical consensus failure.

⸻

9. Genesis Allocation

Genesis MUST define:

GenesisEconomicConfiguration {
    total_genesis_supply
    account_allocations[]
    treasury_allocation
    faucet_allocation_optional
    research_allocation_optional
    reserve_allocation_optional
}

Each allocation MUST include:

recipient_address
amount
purpose_code
vesting_or_lock_rule_optional

⸻

Q1-TKN-001 — Transparent genesis

All genesis allocations MUST be publicly inspectable.

⸻

Q1-TKN-002 — Deterministic total

The sum of all genesis allocations MUST equal the configured genesis supply.

⸻

Q1-TKN-003 — No hidden reserve

No undisclosed genesis allocation may exist.

A reserve is permitted only as an explicit allocation to a defined
protocol-controlled or treasury address. No implicit, hidden, or unassigned
genesis reserve is permitted.

⸻

Q1-TKN-004 — Testnet reset freedom

Private testnet genesis allocations MAY be reset between experiments.

Such resets MUST be documented and SHALL not be represented as ordinary chain continuity.

⸻

10. Initial Supply Scenarios

The simulator SHALL support at least the following models.

Scenario A — Pure test faucet

* fixed genesis supply;
* no public scarcity claim;
* test units distributed from a faucet;
* simple issuance for block testing.

⸻

Scenario B — Fixed maximum supply

* maximum supply defined;
* block issuance declines or ends;
* long-term security increasingly depends on fees.

⸻

Scenario C — Perpetual bounded issuance

* no absolute final cap;
* issuance remains small and predictable;
* ongoing issuance funds security.

⸻

Scenario D — Adaptive security issuance

* issuance adjusts within strict bounds;
* adjustment targets a security budget;
* greater complexity and manipulation risk.

The first implementation SHOULD begin with Scenario A or a simple bounded version of Scenario C.

⸻

11. Initial Experimental Issuance

Recommended first private-testnet model:

Genesis Supply = configurable
Block Issuance = fixed base units per finalized block
Maximum Supply = not economically meaningful in v0.1

Illustrative value:

BaseBlockIssuance = 10 Q1T

This value is not final and has no market implication.

The purpose is to exercise:

* reward creation;
* distribution;
* supply accounting;
* inflation measurement;
* participant incentives.

⸻

12. Issuance Authorization

New units MAY be created only through:

* genesis allocation;
* finalized block issuance;
* explicitly versioned future protocol mechanisms.

Users, nodes, wallets, producers, validators, AI systems, and administrators MUST NOT create arbitrary units.

⸻

13. Security Budget

The economic security budget for a block is:

SecurityBudget
=
BlockIssuance
+
TransactionFeesAllocatedToSecurity

The budget MAY be distributed among:

* producer;
* delay executor;
* validators;
* treasury;
* other objectively measured security roles.

Q1 MUST measure whether the security budget is sufficient to motivate participation without producing excessive issuance.

⸻

14. Sources of Participant Revenue

Participant revenue MAY come from:

Protocol Issuance
Transaction Fees
Experimental Service Rewards
Testnet Grants

Off-chain compensation is outside consensus accounting.

⸻

15. Transaction Fee Model

Q1 v0.1 SHALL use a deterministic, non-negative fee model.

Recommended structure:

TotalRequiredFee
=
BaseFee
+
SizeFee
+
CongestionFee
-
EligibleDiscount

Subject to:

TotalRequiredFee >= MinimumFee

and:

EligibleDiscount <= DiscountCap

⸻

16. Fee Components

16.1 Base Fee

A minimum charge for submitting a standard transaction.

Purpose:

* discourage unlimited free spam;
* compensate basic validation and storage;
* provide predictable minimum cost.

⸻

16.2 Size Fee

Charges for serialized transaction size.

Conceptually:

SizeFee
=
FeePerByte × TransactionSizeBytes

All calculations MUST use checked integers.

⸻

16.3 Congestion Fee

Increases when block demand exceeds available capacity.

Purpose:

* prioritize limited block space;
* reduce mempool overload;
* discourage low-value spam during congestion.

⸻

16.4 Eligible Discount

A bounded experimental reduction.

Potential future bases:

* low-traffic network condition;
* verified public-service activity;
* limited user allowance;
* testnet distribution policy.

Discounts MUST NOT create net negative fees for anonymous transactions.

⸻

17. Minimum Fee

Every ordinary transfer MUST satisfy:

RequiredFee >= MinimumFee > 0

The protocol MUST NOT pay an anonymous sender merely for creating a transaction.

This prevents direct extraction through unlimited self-generated activity.

⸻

18. Fee Limit

The sender signs:

fee_limit

The charged fee SHALL be:

ChargedFee = RequiredFee

provided:

RequiredFee <= FeeLimit

Otherwise the transaction is invalid for that block.

The difference:

FeeLimit - ChargedFee

remains in the sender account.

⸻

19. Fee Predictability

The wallet MUST be able to estimate fees before signing.

Fee parameters SHALL be derived from public finalized state.

The fee model SHOULD avoid excessive complexity.

A user SHOULD be able to understand:

* minimum fee;
* size contribution;
* congestion contribution;
* maximum authorized fee.

⸻

20. Congestion Measurement

The congestion model MAY use finalized or bounded public data such as:

* recent block utilization;
* recent eligible transaction demand;
* mempool pressure commitments in future versions;
* transaction inclusion delay;
* average block fullness.

Local mempool size alone MUST NOT determine consensus fees because honest nodes may observe different mempools.

⸻

21. Recommended Initial Congestion Formula

For v0.1, congestion MAY be based on recent finalized block utilization.

Let:

U = median utilization of previous N finalized blocks
T = target utilization

Where utilization is:

BlockUsedBytes / MaximumBlockBytes

Conceptual behavior:

if U <= T:
    CongestionMultiplier = 1
else:
    CongestionMultiplier increases within bounded limits

An integer step model is recommended initially.

Example:

Utilization <= 50%  → multiplier 1
50% < Utilization <= 75% → multiplier 2
75% < Utilization <= 90% → multiplier 3
Utilization > 90% → multiplier 5

This is experimental.

⸻

22. Percentage-of-Value Fees

A fee based purely on a percentage of transferred value SHALL NOT be the only fee component.

Reason:

* network validation cost is not proportional to transfer value;
* high-value transfers may become unnecessarily expensive;
* users may split transactions;
* value-based fees may distort ordinary use.

Q1 MAY simulate a bounded value-sensitive component, but it MUST be:

* capped;
* transparent;
* optional in experiments;
* compared against size-and-congestion pricing.

⸻

23. Proposed Value-Sensitive Experiment

An experimental fee model MAY include:

ValueComponent
=
min(
    ValueFeeCap,
    floor(TransferAmount × ValueFeeRateNumerator / ValueFeeRateDenominator)
)

It MUST use integer arithmetic.

Simulation MUST examine:

* transaction splitting;
* high-value avoidance;
* fairness;
* user behavior;
* revenue stability;
* impact on ordinary payments.

The default v0.1 implementation SHOULD keep:

ValueComponent = 0

until simulation supports its use.

⸻

24. Negative Fees

Q1 MUST NOT permit:

TotalRequiredFee < 0

A network subsidy, if tested, MUST be distributed through a separately limited mechanism rather than a negative anonymous transaction fee.

⸻

25. Fee Subsidies

Potential future subsidy mechanisms MAY include:

* limited faucet credits;
* capped account allowances;
* public-service grants;
* testnet participation rewards;
* sponsored transactions.

Every subsidy MUST have:

* explicit funding source;
* recipient limits;
* anti-Sybil controls;
* maximum amount;
* expiration;
* transparent accounting.

⸻

26. Fee Collection

For each finalized transaction:

SenderDeduction
=
TransferAmount
+
ChargedFee

The charged fee enters a deterministic fee pool associated with the finalized block.

⸻

27. Fee Distribution Conservation

For every finalized block:

TotalFeesCollected
=
FeesToParticipants
+
FeesToTreasury
+
FeesBurned

No fee amount may remain unaccounted.

⸻

28. Fee Distribution Modes

The simulator SHALL support:

Initial private-testnet configuration:

* fee burn is disabled;
* all collected transaction fees are allocated to the security reward pool;
* treasury or burn allocation requires a later approved test scenario.

Mode A — Full participant distribution

FeesBurned = 0
FeesToTreasury = 0

All fees go to security participants.

⸻

Mode B — Partial burn

A percentage of fees is destroyed.

Purpose:

* reduce circulating supply;
* offset issuance.

Risk:

* reduces security compensation.

⸻

Mode C — Treasury allocation

A percentage funds protocol development.

Risk:

* governance concentration;
* treasury dependence.

⸻

Mode D — Hybrid

Fees are divided among participants, treasury, and burn.

The first prototype SHOULD use a transparent hybrid or full distribution model.

⸻

29. Recommended Initial Reward Pool

For each finalized block:

RewardPool
=
BaseBlockIssuance
+
TotalFeesAllocatedToParticipants

Treasury and burn amounts SHALL be accounted separately.

⸻

30. Reward Roles

The first economic implementation SHALL support configurable shares for:

BLOCK_PRODUCER
DELAY_EXECUTOR
VALIDATORS
TREASURY
HDD_PARTICIPANTS
OTHER_EXPERIMENTAL_ROLE

⸻

31. Initial Reward Distribution

Recommended starting configuration:

Producer: 35%
Delay Executor: 25%
Validators: 30%
Treasury: 10%
HDD Participant: 0%

If producer and delay executor are the same participant, it receives both applicable shares.

These percentages are experimental.

⸻

32. Exact Integer Distribution

Reward percentages MUST be represented as integer weights.

Example:

producer_weight = 35
delay_weight = 25
validator_weight = 30
treasury_weight = 10
total_weight = 100

Role allocation:

RoleReward
=
floor(
    DistributableRewardPool
    × RoleWeight
    / TotalWeight
)

Any remainder MUST be handled deterministically.

For the initial private-testnet economic model:

remainder → Protocol Treasury

The treasury remainder MUST be visible in accounting and included in exact
conservation. A future approved tokenomics version MAY reconsider this rule.

⸻

33. Producer Reward

A producer receives a reward only if:

* it was eligible;
* its proposal was valid;
* the block finalized;
* it did not produce objective equivocation for that height and round;
* reward rules permit payment.

A rejected, late, malformed, or non-finalized proposal earns no producer reward.

⸻

34. Delay Executor Reward

The delay executor receives a reward only if:

* its delay proof is valid;
* the proof is included in a finalized block;
* the executor identity is correctly bound;
* no duplicate payment occurs.

A reported duration or energy use is insufficient.

⸻

35. Validator Reward

Validator reward SHALL be distributed only among validators whose valid attestations are included or otherwise objectively proven for the finalized block.

A validator MUST NOT be paid for:

* invalid signature;
* incorrect committee membership;
* duplicate attestation;
* conflicting attestation;
* late attestation beyond protocol rules;
* absence.

⸻

36. Equal Validator Distribution

Initial recommendation:

ValidatorPool
=
configured validator share
RewardPerValidValidator
=
floor(
    ValidatorPool
    / NumberOfRewardEligibleValidators
)

Remainder handling MUST be deterministic.

For the initial private-testnet economic model, validator-pool division
remainders MUST go to the Protocol Treasury.

⸻

37. Weighted Validator Distribution

Future experiments MAY weight validator rewards by:

* valid attestation weight;
* timeliness;
* availability;
* bounded service contribution.

The first implementation SHOULD avoid complex weighting.

Complex reward weighting may create manipulation incentives.

⸻

38. HDD Reward

In the default mode:

HDDReward = 0

If later enabled, HDD reward MUST be:

* capped;
* challenge-bound;
* based on accepted evidence;
* independent of unverified device labels;
* resistant to capacity monopolization;
* tested against SSD, RAM, and virtual-disk substitution.

⸻

39. Relay and Availability Rewards

Q1 v0.1 SHOULD NOT reward generic message relay because it is difficult to prove objective independent contribution.

Future rewards may be considered for:

* archived block delivery;
* snapshot availability;
* erasure-coded storage;
* verifiable data service.

No reward SHALL be created solely from self-reported bandwidth.

⸻

40. Treasury

The protocol treasury MAY receive a deterministic allocation.

Treasury use is outside consensus unless governance rules specify otherwise.

Treasury accounting MUST expose:

* treasury address;
* all inflows;
* all outflows;
* authorization method;
* remaining balance.

Q1 v0.1 MAY use a fixed testnet treasury account.

⸻

41. Treasury Risk

The project MUST test:

* treasury concentration;
* governance capture;
* insider allocations;
* dependency on ongoing treasury spending;
* treasury-key compromise;
* public transparency;
* alternative funding models.

Treasury allocation MUST NOT be described as inherently decentralized.

⸻

42. Reward Maturity

Q1 MAY require rewards to mature before spending.

Purpose:

* allow detection of invalid finalization or testnet bugs;
* reduce immediate extraction;
* simplify penalty experiments.

Recommended private-testnet experiment:

RewardMaturity = 10 finalized blocks

Maturity SHALL be represented deterministically.

⸻

43. Mature and Immature Balances

An account query MAY distinguish:

spendable_balance
immature_reward_balance
locked_balance

Immature rewards MUST NOT be spendable before maturity.

⸻

44. Penalties

Q1 v0.1 SHALL begin with virtual penalties.

Possible virtual penalties:

* reward forfeiture;
* temporary producer cooldown;
* temporary validator suspension;
* participation-score reduction;
* private-testnet registry removal.

Real collateral slashing is not required in the first milestone.

⸻

45. Objective Penalty Evidence

Economic penalties MAY rely only on objectively verifiable evidence such as:

* producer equivocation;
* validator double-voting;
* invalid signed protocol object;
* violation of a deterministic role rule.

AI risk scores alone MUST NOT trigger monetary penalties.

⸻

46. Penalty Conservation

If a monetary penalty is later implemented, confiscated units MUST be:

* burned;
* transferred to treasury;
* redistributed according to explicit rules;
* or returned under a defined appeal mechanism.

They MUST NOT disappear from accounting.

⸻

47. Missed Participation

A participant SHOULD normally lose only the reward opportunity for:

* being offline;
* missing a producer window;
* failing to attest;
* failing to complete delay work.

Additional penalties for downtime MAY discourage participation by ordinary users and MUST be tested carefully.

⸻

48. Profitability Model

Participant profitability is:

NetParticipantReturn
=
ProtocolRewards
+
FeeRewards
-
ElectricityCost
-
HardwareCost
-
BandwidthCost
-
StorageCost
-
MaintenanceCost
-
PenaltyLoss

Q1 MUST measure all material cost categories where practical.

Protocol rewards alone do not prove profitability.

⸻

49. Energy-Aware Economics

Q1 SHOULD calculate:

RewardPerEnergyUnit
SecurityBudgetPerEnergyUnit
FinalizedTransactionsPerEnergyUnit

However, consensus reward MUST NOT directly depend on self-reported energy use.

Otherwise participants could profit by exaggerating consumption.

⸻

50. Security-Cost Efficiency

A key research metric SHALL be:

SecurityCostEfficiency
=
MeasuredSecurityBenefit
/
TotalEconomicAndEnergyCost

Because security benefit is difficult to quantify, Q1 MUST use several proxy metrics rather than one unsupported number.

Potential proxies:

* attack cost;
* participant count;
* operator diversity;
* committee independence;
* finality reliability;
* invalid-block rejection;
* network recovery;
* concentration resistance.

⸻

51. Inflation

The simulator MUST calculate:

AnnualizedIssuanceRate
=
NewUnitsIssuedDuringPeriod
/
StartingCirculatingSupply

Inflation SHALL be distinguished from:

* price change;
* market capitalization;
* user adoption;
* transaction volume.

No price prediction follows automatically from issuance rate.

⸻

52. Deflation

Fee burning may create net supply reduction if:

BurnedFees > NewIssuance

Q1 MUST test whether deflation:

* harms security budget;
* encourages hoarding;
* reduces ordinary use;
* creates unstable fee dependence;
* benefits early holders disproportionately.

Deflation MUST NOT be assumed to be automatically desirable.

⸻

53. Long-Term Security Scenarios

The simulator SHALL compare:

Issuance-dominant security

Most security compensation comes from new issuance.

⸻

Fee-dominant security

Most security compensation comes from transaction fees.

⸻

Hybrid security

Both issuance and fees remain material.

⸻

Adaptive security

Issuance changes within bounds based on defined security indicators.

No scenario is approved as final.

⸻

54. Adaptive Issuance Research

A future adaptive issuance model MAY target:

* minimum active producer count;
* validator participation;
* security-budget adequacy;
* concentration limits;
* fee volatility.

However, adaptive issuance introduces manipulation risk.

It MUST NOT use:

* token market price as a direct consensus input;
* AI forecasts as automatic issuance authority;
* self-reported participant costs;
* local mempool size alone.

⸻

55. Market Price

Q1 protocol cannot determine the external market price of its native asset.

Market price, if one ever exists, depends on:

* external exchange behavior;
* supply and demand;
* liquidity;
* regulation;
* utility;
* expectations;
* speculation;
* security perception.

Consensus MUST remain valid without knowing a dollar, euro, or stablecoin price.

⸻

56. Price Oracle Prohibition

Q1 v0.1 MUST NOT use external token price as a consensus-critical input for:

* transaction fees;
* block validity;
* issuance;
* validator threshold;
* producer selection;
* penalties.

External price oracles create separate trust and manipulation risks.

⸻

57. Transaction Velocity

The system SHOULD measure:

* transactions per unit of supply;
* average transfer frequency;
* dormant balances;
* repeated self-churn;
* concentration of transfer activity;
* fee revenue per transaction.

High transaction count does not necessarily mean real adoption.

⸻

58. Artificial Volume

The economic test suite MUST attempt:

* self-transfers where permitted;
* circular transfers;
* many controlled wallets;
* transaction splitting;
* repeated low-value transfers;
* fee farming;
* subsidy farming;
* reward cycling;
* fake economic activity.

Economic rules MUST not reward volume merely because it exists.

⸻

59. Anti-Spam Economics

Spam resistance SHALL combine:

* positive minimum fee;
* transaction-size fee;
* congestion fee;
* mempool limits;
* per-sender limits;
* peer rate limits.

Economic defenses MUST work with networking defenses.

⸻

60. Dust Transactions

Q1 SHOULD define a minimum economically meaningful transfer or a fee model that makes extremely small spam unattractive.

A fixed protocol dust threshold MAY be avoided initially because future unit value is unknown.

The wallet SHOULD warn when:

TransferAmount <= RequiredFee

The protocol MAY still permit the transaction if valid.

⸻

61. Transaction Splitting

The simulator MUST test whether users can reduce fees by dividing one transfer into many smaller transfers.

The fee model SHOULD normally make splitting no cheaper when each transaction consumes separate network resources.

⸻

62. Transaction Aggregation

Future versions MAY support batch transfers.

Batching MAY reduce network cost but introduces:

* larger transaction structure;
* more complex fee rules;
* partial-failure questions.

Not required in v0.1.

⸻

63. Concentration Metrics

Q1 SHALL measure reward and participation concentration using:

* top-1 participant share;
* top-5 participant share;
* top-10 participant share;
* Gini coefficient;
* Herfindahl-Hirschman Index;
* producer-selection concentration;
* validator-reward concentration;
* operator concentration;
* hardware-class concentration;
* geographic concentration where data is available.

These are research metrics, not direct consensus facts.

⸻

64. Anti-Concentration Experiments

The simulator MAY test:

* diminishing reward returns;
* producer cooldown;
* capped role rewards;
* rotating eligibility;
* square-root weighting;
* minimum participation diversity;
* bounded reputation;
* random committee selection;
* reward smoothing.

Every anti-concentration mechanism may create new Sybil incentives.

⸻

65. One Participant, Many Identities

Economic design MUST assume that one operator may create many identities.

Therefore:

* equal reward per identity can be exploited;
* fixed per-node subsidies are unsafe without Sybil resistance;
* device count is not identity;
* IP count is not identity;
* wallet count is not identity.

Public permissionless Sybil resistance remains an open problem.

⸻

66. Reward Smoothing

Reward smoothing MAY reduce income volatility.

Possible mechanisms:

* multi-block reward windows;
* protocol pools;
* deterministic delayed distribution;
* external voluntary pools.

Protocol-level pooling can introduce complexity and concentration.

It is not required in v0.1.

⸻

67. Mining or Production Pools

Participants may voluntarily coordinate outside the protocol.

Q1 MUST assume that production pools may emerge.

Research SHALL examine:

* pool centralization;
* block censorship;
* reward custody;
* operator power;
* correlated failures;
* delegation risks.

The base protocol SHOULD not require one official pool.

⸻

68. Reward Variance

The simulator MUST calculate:

* expected reward;
* reward variance;
* time between rewards;
* effect of candidate selection;
* effect of committee selection;
* effect of participant size;
* small-participant viability.

Low expected reward with extreme variance may drive users into centralized pools.

⸻

69. User Affordability

The economic model SHOULD measure:

* median fee;
* fee as percentage of transfer;
* fee under congestion;
* minimum economically rational transfer;
* fee volatility;
* failed transaction cost;
* wallet fee-estimation accuracy.

Q1 aims to support ordinary transfers, not only high-value settlement.

⸻

70. Fee Volatility Limits

The congestion algorithm SHOULD limit abrupt fee changes.

Potential rule:

MaximumFeeParameterIncreasePerBlock = bounded percentage
MaximumFeeParameterDecreasePerBlock = bounded percentage

The exact integer rules remain open.

⸻

71. Block Capacity and Fees

Fee pressure depends partly on block capacity.

The economic simulator MUST test:

* small blocks;
* large blocks;
* short block intervals;
* long block intervals;
* validation cost;
* propagation delay;
* storage growth;
* fee revenue.

Increasing capacity is not free.

⸻

72. Economic Parameter Configuration

Genesis MUST define:

EconomicParameters {
    monetary_model_id
    initial_supply
    base_block_issuance
    maximum_supply_optional
    reward_maturity
    minimum_fee
    fee_per_byte
    congestion_model_id
    congestion_window
    congestion_thresholds
    value_fee_rate_optional
    value_fee_cap_optional
    discount_model_id
    discount_cap
    fee_burn_weight
    fee_treasury_weight
    fee_participant_weight
    producer_reward_weight
    delay_reward_weight
    validator_reward_weight
    hdd_reward_weight
    treasury_reward_weight
    penalty_mode
    economic_adjustment_interval
}

Consensus-critical economic parameters MUST be identical across nodes.

⸻

73. Economic Versioning

Every block reward and fee calculation MUST identify:

economic_rule_version

Unknown versions MUST be rejected.

Economic upgrades require:

* activation height;
* migration rules;
* simulation results;
* compatibility tests;
* supply-impact analysis;
* security-impact analysis.

⸻

74. Reward Record

Each finalized block SHALL include deterministic reward records.

RewardRecord {
    reward_version
    source_block_height
    source_block_hash
    recipient_address
    participant_id_optional
    reward_role
    gross_reward
    penalty_deduction
    net_reward
    maturity_height
}

⸻

75. Fee Summary

Each finalized block SHALL include:

BlockFeeSummary {
    total_fees_collected
    fees_to_producer
    fees_to_delay_executor
    fees_to_validators
    fees_to_treasury
    fees_to_hdd
    fees_burned
}

The sum of outputs MUST equal total fees collected.

⸻

76. Issuance Summary

Each finalized block SHALL include:

BlockIssuanceSummary {
    base_issuance
    adaptive_issuance_optional
    total_new_issuance
    total_burn
    net_supply_change
    resulting_total_issued_supply
    resulting_total_burned_supply
    resulting_circulating_supply
}

⸻

77. Deterministic Reward Calculation

Conceptual procedure:

function calculate_block_economics(
    parent_economic_state,
    block,
    valid_attestations,
    protocol_parameters
):
    total_fees = sum(transaction.charged_fee)
    fee_burn =
        allocate(total_fees, fee_burn_weight)
    fee_treasury =
        allocate(total_fees, fee_treasury_weight)
    fee_security_pool =
        total_fees - fee_burn - fee_treasury
    issuance =
        calculate_authorized_issuance(
            parent_economic_state,
            protocol_parameters,
            block.height
        )
    distributable_pool =
        issuance + fee_security_pool
    producer_reward =
        allocate(distributable_pool, producer_weight)
    delay_reward =
        allocate(distributable_pool, delay_weight)
    validator_pool =
        allocate(distributable_pool, validator_weight)
    protocol_treasury_reward =
        allocate(distributable_pool, treasury_weight)
    hdd_reward =
        allocate(distributable_pool, hdd_weight)
    distribute_validator_pool(
        validator_pool,
        reward_eligible_validators
    )
    assign_rounding_remainder()
    verify_conservation()
    return economic_transition

⸻

78. No Reward Without Finality

Rewards are created only after or as part of finalizing a valid block.

A proposed or attested but non-finalized block MUST NOT create spendable rewards.

⸻

79. Duplicate Role Rewards

If one participant performs multiple valid roles, it MAY receive multiple role rewards.

Example:

Producer + Delay Executor

However, the simulator MUST measure whether role combination creates excessive concentration.

⸻

80. Role Separation Experiments

Q1 SHALL test:

* producer and delay executor combined;
* producer and delay executor separated;
* validators independent;
* same operator controlling several roles;
* caps on multi-role rewards;
* committee exclusion of producers.

⸻

81. Economic Telemetry

The system MUST record:

* total supply;
* circulating supply;
* issuance per block;
* fee revenue;
* burn amount;
* treasury inflow;
* reward by role;
* reward by participant;
* reward concentration;
* fee distribution;
* transaction fee percentiles;
* failed participation;
* penalties;
* energy estimates;
* estimated participant costs;
* reward variance;
* economic parameter version.

⸻

82. Economic Dashboard

The private testnet SHOULD display:

* supply over time;
* issuance rate;
* fees per block;
* median transaction fee;
* reward distribution;
* treasury balance;
* burn totals;
* producer concentration;
* validator concentration;
* participant profitability estimates;
* security-budget trend;
* energy-cost trend.

Dashboard data is observational and MUST NOT control consensus.

⸻

83. Economic Simulation Engine

Q1 SHALL include a configurable economic simulator.

The simulator SHOULD support:

q1-economics simulate
q1-economics compare
q1-economics stress
q1-economics attack
q1-economics report

⸻

84. Simulation Inputs

The simulator SHALL accept:

participant_count
participant_cost_profiles
hardware_profiles
energy_prices
transaction_arrival_rate
transaction_value_distribution
block_capacity
block_interval
fee_parameters
issuance_parameters
reward_distribution
participant_uptime
Sybil_identity_count
operator_distribution
market_price_assumption_optional

Market-price assumptions are simulation-only and MUST NOT enter consensus.

⸻

85. Required Economic Scenarios

The simulator MUST test:

1. empty network;
2. low transaction volume;
3. normal volume;
4. sustained congestion;
5. sudden transaction spike;
6. spam attack;
7. many low-value transfers;
8. few high-value transfers;
9. transaction splitting;
10. circular artificial volume;
11. many small participants;
12. few large participants;
13. one dominant producer;
14. one dominant validator operator;
15. producer pooling;
16. validator collusion;
17. rising energy costs;
18. falling energy costs;
19. hardware-cost decline;
20. participant departure;
21. participant surge;
22. low token-price assumption;
23. high token-price assumption;
24. fee-only security;
25. issuance-only security;
26. hybrid security;
27. fixed supply;
28. perpetual issuance;
29. partial fee burn;
30. high treasury allocation;
31. HDD rewards enabled;
32. reward maturity;
33. penalty experiments;
34. Sybil farming;
35. subsidy farming;
36. reward-concentration limits;
37. long-term 10-year model;
38. long-term 50-year model;
39. extreme low usage;
40. extreme high usage.

⸻

86. Economic Attack Scenarios

The test team MUST attempt:

* fee manipulation;
* congestion manufacturing;
* self-transaction farming;
* reward farming;
* Sybil reward extraction;
* transaction splitting;
* treasury capture;
* validator cartel formation;
* producer cartel formation;
* block-space censorship;
* delayed transaction inclusion;
* artificial fee escalation;
* subsidy draining;
* HDD identity multiplication;
* reputation farming;
* temporary participation around reward windows;
* collusive equivocation for profit;
* market-price oracle manipulation if later introduced.

⸻

87. Sustainability Metrics

Q1 SHOULD calculate:

SecurityBudgetCoverage
=
SecurityBudget
/
EstimatedParticipantSecurityCost
FeeCoverageRatio
=
FeeRevenue
/
TotalSecurityReward
IssuanceCoverageRatio
=
NewIssuance
/
TotalSecurityReward
TreasuryRunway
=
TreasuryBalance
/
AverageTreasurySpending

Treasury spending is off-chain unless later governed by protocol.

⸻

88. Economic Success Criteria

An economic configuration MAY be considered promising if simulations show:

* sufficient participation incentive;
* affordable ordinary fees;
* bounded inflation;
* understandable rules;
* resistance to simple spam;
* no obvious infinite reward loop;
* no trivial Sybil subsidy;
* tolerable reward concentration;
* viable small participants;
* measurable security budget;
* predictable supply accounting;
* no dependence on external price oracles.

⸻

89. Economic Failure Criteria

A configuration SHOULD be rejected or redesigned if:

* arbitrary units can be created;
* rewards exceed explicit funding sources;
* negative anonymous fees enable extraction;
* self-transfers produce net profit;
* many fake identities multiply rewards cheaply;
* one participant receives persistent overwhelming control;
* ordinary fees become unusable;
* security collapses when issuance declines;
* issuance grows without bounds or justification;
* reward rules encourage wasted energy;
* HDD rewards encourage unnecessary hardware purchase;
* treasury allocation creates effective centralized ownership;
* fee volatility becomes unpredictable;
* accounting cannot be independently reconstructed.

⸻

90. Minimum Tokenomics Prototype Acceptance

The economic subsystem is complete when:

1. genesis allocations balance exactly;
2. block issuance is deterministic;
3. transaction fees are deterministic;
4. fees never become negative;
5. fee limits are enforced;
6. block rewards are distributed by explicit integer weights;
7. validator rewards include only eligible attestations;
8. invalid proposals create no rewards;
9. duplicate participants are not paid twice for one role;
10. multi-role rewards are accounted explicitly;
11. fee distribution conserves value;
12. supply transitions conserve value;
13. treasury inflows are visible;
14. burns reduce circulating supply correctly;
15. reward maturity works if enabled;
16. virtual penalties are recorded;
17. all honest nodes derive identical economic state;
18. economic telemetry is exportable;
19. alternative parameter sets can be simulated;
20. no market-price input is required.

⸻

91. Private Testnet Acceptance

Before private distributed testing is considered economically meaningful:

* at least 1,000 block rewards must be processed;
* at least 10,000 transaction fees must be calculated;
* supply must remain exactly reproducible;
* no unauthorized issuance may occur;
* fee behavior must be tested under load;
* participant rewards must be compared against measured costs;
* reward concentration must be reported;
* small-participant outcomes must be measured;
* Sybil reward attacks must be attempted;
* long-run issuance scenarios must be simulated;
* fee-only and issuance-supported security must be compared.

⸻

92. Known Limitations

Q1 Tokenomics v0.1 does not establish:

* real market value;
* legal classification;
* regulatory compliance;
* investor suitability;
* price stability;
* public-mainnet profitability;
* permanent monetary policy;
* perfect Sybil resistance;
* optimal fee pricing;
* optimal issuance;
* guaranteed decentralization;
* guaranteed participant income.

The model is a laboratory configuration.

⸻

93. Open Decisions

The following remain unresolved:

1. final monetary model;
2. whether total supply is capped;
3. initial public supply if any;
4. final genesis allocation;
5. final block issuance;
6. issuance decay or perpetuity;
7. exact security-budget target;
8. exact fee formula;
9. whether value-sensitive fees are ever used;
10. exact congestion model;
11. exact fee-adjustment limits;
12. whether fees are burned;
13. treasury percentage;
14. treasury control structure;
15. producer reward share;
16. delay-executor reward share;
17. validator reward share;
18. HDD reward policy;
19. reward maturity;
20. penalty funding destination;
21. collateral or staking requirements;
22. public Sybil resistance;
23. archive and availability rewards;
24. participant-profitability targets;
25. anti-concentration mechanism;
26. reward smoothing;
27. long-term fee-only viability;
28. adaptive issuance;
29. emergency economic changes;
30. governance process for monetary upgrades.

All decisions MUST be recorded in OPEN_DECISIONS.md.

⸻

94. Codex Implementation Rules

Codex MUST:

1. represent every amount as a checked integer;
2. isolate economic calculations in a deterministic module;
3. version all fee and reward rules;
4. enforce supply conservation;
5. enforce fee conservation;
6. prohibit negative ordinary fees;
7. enforce fee limits;
8. calculate rewards only for finalized blocks;
9. include only objectively eligible participants;
10. make all reward weights configurable through genesis;
11. handle integer remainders deterministically;
12. expose full economic summaries;
13. implement economic simulation separately from consensus;
14. keep market price outside consensus;
15. keep AI predictions outside issuance and fee validity;
16. support multiple monetary scenarios;
17. include property-based accounting tests;
18. measure reward concentration;
19. record all assumptions;
20. label Q1T as a test unit.

Codex MUST NOT:

* use floating-point monetary calculations;
* create rewards from local telemetry;
* pay a participant for self-reported energy;
* grant HDD reward merely for device claims;
* use external exchange price in consensus;
* create a hidden treasury;
* silently change issuance;
* pay non-finalized work;
* allow reward totals to exceed funding sources;
* treat high transaction volume as verified economic value;
* hard-code permanent monetary policy before simulation;
* present testnet rewards as investment returns.

⸻

95. Final Economic Principle

Q1 economics MUST preserve this principle:

Value may move between users, but ordinary transfers may not create it.
New units may arise only from explicit public rules.
Security work may earn reward only when it contributes to a valid finalized block.
Fees must discourage abuse without turning ordinary use into a privilege.
Participation must remain economically possible without rewarding infinite identities, wasted energy, or permanent dominance.
Every issued, transferred, burned, locked, rewarded, or treasury-held unit must remain accountable.

Q1 succeeds at the economic layer when every honest node can independently explain:

* where every unit came from;
* why every fee was charged;
* why every participant was rewarded;
* how much supply exists;
* what security cost was paid;
* who gained economic influence;
* and whether the network’s incentive structure remains useful under attack.
