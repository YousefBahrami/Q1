# Q1 Mainnet issuance and mining thesis

Date: 2026-10-03

Status: historical reconstruction and research comparison; NOT an approved
Mainnet economic specification. No issuance, distribution, sale, or network
deployment is authorized by this document. No consensus code is changed.

**Before creating the first economically real Q1, define exactly why it is
created, who earns it, and at what cost.** A software balance alone establishes
neither economic value nor a legal entitlement.

Follow-up: the [mining and issuance recovery](Q1_MINING_AND_ISSUANCE_RECOVERY.md)
adds a deeper classified inventory, HDD/delay pipeline, dated conflict table
and reward-flow analysis. The balanced dashboard (separate planning document omitted from this candidate)
coordinates all five continuing workstreams. This thesis remains the companion
for genesis/code-path analysis and the initial issuance comparison.

## 1. Evidence and authority

Human decisions take precedence over approved specifications, ADRs, draft
documents, implementation, and research suggestions, in that order. A draft
using SHALL/MUST is not evidence that its undecided economics were approved.

The review searched current documents and all reachable historical Markdown/text
document blobs in the working repository, excluding personal collaboration
material: 192 distinct blobs inspected, 152 keyword matches across 71 paths.
Terms covered mining, block production, rewards, issuance, supply, hardware/HDD,
delay/proof evidence, resource contribution, selection, treasury, and genesis.
Relevant original drafts were compared with later decision records and code.
The earliest available specification checkpoint is dated 2026-07-24. This
reconstruction does not claim to recover earlier unwritten intentions, deleted
unreachable history, or documents outside the available repository.

Source map (section numbers refer to headings inside the linked documents):

- [Tokenomics draft](../10_TOKENOMICS.md), especially §§3–12, 15–42, 48–54,
  67–79: original economic laboratory and alternative policies.
- [Decision register](../../OPEN_DECISIONS.md): DEC-Q1-004–007, 010, 016–027
  and the later LOCALNET-only authority. Its statuses control approval claims.
- [Consensus draft](../05_CONSENSUS.md), [delay draft](../06_DELAY_ENGINE.md)
  and [HDD laboratory draft](../07_HDD_LAB_MODULE.md): intended production
  sequence, candidate delay constructions, and hardware hypotheses.
- [Governance draft](../19_GOVERNANCE.md), especially §§52–53, 69–70, and
  [roadmap draft](../20_ROADMAP.md), especially §§55–56: treasury and funding.
- [Normalization report](../25_PHASE_0_1_NORMALIZATION_REPORT.md),
  [schema decision package](../32_DEC_Q1_027_SCHEMA_DECISION_PACKAGE.md), and
  [schema checkpoint review](../42_DEC_Q1_027_SESSIONS_1_TO_5C_CHECKPOINT_REVIEW.md):
  exact-sum correction and limits of individual schema approvals.
- [LOCALNET execution profile](../protocol/Q1_LOCALNET_V0.md),
  [milestone evidence](../reports/milestones/LOCALNET_V0.md), and
  [release notes](../releases/v0.1.0-localnet.1.md): implemented, bounded scope.
- [Public testnet proposal](Q1_PUBLIC_TESTNET_V0_PLAN.md),
  [differentiation audit](Q1_DIFFERENTIATION_AUDIT.md),
  [early revenue comparison](Q1_EARLY_REVENUE_OPTIONS.md), and
  [monetary-distribution boundary](../releases/PRE_OFFER_BOUNDARY.md): subsequent
  research and approval gates, not a Mainnet authorization.

The original tokenomics, HDD, governance, and roadmap files have no subsequent
content revisions in the available Git history. Their older recommendations
must therefore be read alongside later scoped decisions, not as current runtime
behavior. The accepted ADRs 0001–0005 select engineering primitives; none
selects a Mainnet supply schedule or mining mechanism.

## 2. Original issuance thesis and chronological reconstruction

### What was originally intended

The earliest written thesis is an **economic laboratory for a native unit**:
explicit genesis allocations, potentially new units on finalized blocks,
transaction fees, and accountable rewards for useful protocol roles. It is not
a finalized scarce-money or hardware-mining launch plan.

The draft names a test unit Q1TestUnit/Q1T and recommends an integer base unit
with an illustrative denomination of 100,000,000 base units per Q1T. Neither
that denomination nor any Mainnet supply value is thereby approved. Its
suggested first experiment combines configurable genesis balances with fixed
issuance per finalized block. The example of 10 Q1T per block is expressly an
experimental parameter, not a promised reward.

Producer, delay executor, and finalization participants were possible reward
roles; a development treasury was another possible destination. The draft's
35/25/30/10 split among those four destinations is historical illustration
only. This report neither adopts those percentages nor proposes replacements.
HDD telemetry, AI output, and unverifiable relay claims do not establish reward
entitlement. The suggested ten-block maturity is also an experiment, not an
approved Mainnet vesting rule. Sources: tokenomics §§3–5, 9–12, 29–42;
DEC-Q1-021–023.

Delay was intended to impose challenge-bound sequential effort on a selected
producer, with independently verifiable evidence. HDD research asked whether
ordinary drives add useful, reproducible cost. Neither document establishes
that HDDs are necessary, advantageous, or resistant to cheaper emulation.

### Timeline: intention, approval, supersession, open work

| Available record | What it establishes | What it does not establish |
|---|---|---|
| 2026-07-24 initial specification checkpoint | Draft economic laboratory; genesis, issuance, fee and reward alternatives; sequential delay and HDD research; transparent treasury aspirations | Final supply, monetary rewards, hardware security, public admission, or launch approval |
| 2026-07-24 recorded decisions and normalization | DEC-004 permissioned private participants with equal initial weights; DEC-006 exact allocation sum; DEC-016 initial private rounding remainder to treasury; DEC-017 private burn disabled and fees to reward pool | Public Sybil resistance, treasury percentages, permissionless selection, or Mainnet fee/issuance policy |
| July schema decisions, checkpointed 2026-07-25 | Canonical identities, signed transfers, roots, and header commitments; subsequent RoundNumber correction | Complete monetary state, generic genesis vesting, an approved delay engine, or reward execution |
| 2026-10-01–02 scoped LOCALNET decisions and implementation | Fee exactly one base unit; issuance/burn/distribution zero; explicit fee pool; fixed producer plus three voters and two votes required; deterministic NONE evidence; complete local genesis/state and recovery | Closure of general DEC-021–023, Mainnet quorum, real delay cost, or public monetary policy |
| 2026-10-02 public source release | Reproducible experimental LOCALNET source under Apache-2.0, released from a clean snapshot | A public network, economic Q1, token distribution, or independently audited security |
| 2026-10-02–03 post-release research and current instruction | Public-testnet proposal, identity/revenue analysis, and authorization to reconstruct issuance choices | Approval to implement those proposals or choose an issuance winner |

Dates above distinguish recorded decisions from Git checkpoint dates; they do
not invent a precise time of approval where the record supplies none.

**Superseded or narrowed:** the older possibility of an implicit unallocated
genesis remainder is closed by DEC-006: every reserve must be explicit and
allocations must sum exactly. The initial general fee/issuance suggestions are
replaced operationally by the narrower LOCALNET rules, only for that profile.
Historical delay-schema placeholders are superseded by the registered local
evidence commitment (`DELAY_EVIDENCE = 0x0014`); registering a domain does not
approve a resource engine. Approval of a minimal local genesis does not approve
the generic genesis/vesting alternatives in the schema decision package.

**Still open:** producer/committee selection (005), general economics (007,
021–023), delay/HDD role (010), seed/randomness (018), round changes (019),
public admission (024), formal VDF path (025), public governance (026), and
remaining general compound/state schemas (027 and associated state decisions).
The public quorum/fault model is also unresolved. LOCALNET authorization does
not silently close any of these broader questions.

The draft's capped, perpetual, and adaptive issuance scenarios remain research.
Its label “fixed genesis” in a faucet experiment must not be confused with a
lifetime ban on subsequent issuance. A fixed maximum with future block rewards
is likewise different from creating every unit at genesis.

## 3. What is implemented today: test balances, not Mainnet Q1

**LOCALNET balances are test state. They are not Mainnet Q1, carry no monetary
claim, and have no approved conversion or redemption into future Q1. The
1,000-unit fixture does not determine Mainnet supply.** Resetting a test chain
creates a separate experiment; it does not create a claim on a future chain.

### Exact balance creation and subsequent execution

1. [`tests/common/mod.rs::genesis`](../../crates/q1-localnet/tests/common/mod.rs)
   explicitly constructs sorted allocations: public test sender 1,000 base
   units, public test recipient zero; declared supply 1,000. Test key seeds are
   intentionally public. [`examples/fixture.rs`](../../crates/q1-localnet/examples/fixture.rs)
   uses this helper to emit the local genesis and test keys.
2. [`Genesis::new`](../../crates/q1-localnet/src/genesis.rs) validates LOCALNET
   identity, canonical ordering, fixed participants, and the allocation sum
   through `Ledger::from_allocations`. Its policy commits fee=1, no issuance,
   no burn, no rewards, quorum=2, round=0, NONE delay, and no rotation.
   `decode_canonical` accepts this exact policy, not a configurable mint switch.
   The constructor accepts other valid declared test amounts: 1,000 is a fixture
   choice, not a universal supply constant.
3. `Genesis::initial_state`, called by
   [`Chain::new`](../../crates/q1-localnet/src/block.rs), calls
   [`Ledger::from_allocations`](../../crates/q1-localnet/src/ledger.rs).
   That function inserts the declared balances with nonce zero, verifies their
   checked sum equals declared supply, and starts at height zero with pool zero.
   This explicit initialization is where the initial units enter ledger state;
   there is no earned-resource event or prior transfer funding them.
4. [`StateSnapshot::new`](../../crates/q1-localnet/src/state.rs) requires ledger
   supply to equal genesis supply. Its canonical root commits chain/genesis,
   height, supply, reward pool, accounts, and the fixed participant registry.
5. `Ledger::apply_block` validates on a candidate copy; `apply_transfer` moves
   amount to the recipient and exactly one base unit to the pool. Sender pays
   amount plus actual fee, not FeeLimit. Rejected transactions/blocks do not
   commit balances, fees, or partial rewards. `check_supply` requires
   `sum(account balances) + reward_pool == total_supply`.
6. `Chain::propose` computes the resulting state before signing the proposal;
   `validate_proposal` reexecutes it; `commit` verifies the certificate before
   installing that state. [`PersistentChain`](../../crates/q1-localnet/src/store.rs)
   creation/opening, archive replay, vote reservations, and atomic commits
   preserve this execution. Restoring a snapshot is not an authorized mint path.
7. [`q1-node`](../../crates/q1-node/src/main.rs) loads the validated genesis and
   persistent chain. [`wallet-new`](../../crates/q1-node/src/cli.rs) generates a
   key/address, not a funded account. [`scripts/localnet.py`](../../scripts/localnet.py)
   obtains fixture balances from the example and signs transfers of existing
   units; it does not provide issuance or a minting faucet.

The accepted four-process sequence ends at height 4 with sender 956, recipient
40, nonce 4, pool 4, and supply 1,000. Its common StateRoot is
`b7ec7d47f4cf37d029e74bab02d8bc2aab191f5a5733f92ac4310d919183b087`.
This is recorded local acceptance evidence, not a new economic experiment or
an estimate of Mainnet performance. See the linked milestone report.

### Code and schema surfaces a future approved profile must address

These are dependency findings, **not instructions to loosen LOCALNET guards**.
Mainnet work needs a separately approved/versioned profile; preserve the frozen
local behavior and reference vectors.

| Surface | Required design work before an economic implementation |
|---|---|
| `q1-localnet/src/lib.rs`, `genesis.rs`; fixture generator and node startup | Separate network/profile admission, canonical economic parameters, actual genesis allocations and custody, reproducible genesis ceremony; never reuse public fixture keys |
| `ledger.rs`: initialization, restore, execution, conservation | Authorized creation/burn transitions if chosen; deterministic fees and reward settlement; exact-once accounting, overflow handling, and maturity/locks if chosen. Fixed-genesis-only policy would retain no subsequent minting |
| `state.rs`: constructor, encoder/decoder, `apply_body`, root | Supply currently equals genesis forever. A progressive profile needs approved cumulative accounting and committed reward/lock state; distinguish issued, burned, spendable, and reserved quantities |
| `block.rs`: propose, validate, certificate verification, commit | Reward entitlement and settlement order; producer selection, evidence binding, missed/invalid contributions, replay and equivocation rules; root must be independently computable before signing |
| `q1-protocol-types/src/block_body.rs`, signed/header schemas, domain registry | Body is transfer-only. Decide deterministic implicit issuance versus an explicit system record; version only affected schemas/domains and resolve commitment dependencies |
| `q1-protocol-types/src/delay.rs` and production validation | Replace NONE in the new profile only after selecting/reviewing a real resource mechanism; preserve rejection of NONE outside LOCALNET. An enum naming Mainnet is not an implemented Mainnet |
| `store.rs`, node synchronization and restart paths | Replay every new economic transition, authenticate state/parameter history, preserve reservations and atomicity through crashes; no double issue or reward on retry/recovery |
| Node CLI/status, wallet accounting, deployment tools | Display test/monetary network identity correctly; new genesis and custody flow; show locked/mature amounts if applicable; wallet generation must never imply entitlement |
| Accounting/state/block/policy/recovery tests, reference vectors and acceptance scripts | New-profile independent reconstruction, no double rewards, invalid-work rejection, supply conservation with issuance/burn, certificate variations, restart/replay, boundaries and adversarial cases; retain LOCALNET vectors |

`Amount`'s integer representation supplies checked numerical building blocks;
it does not select a cap, denomination, or emission schedule. Not every row
requires edits under every model, but every reachable execution/recovery path
above requires review for the selected model.

**Reward ordering is a concrete unresolved dependency.** Today's producer signs
StateRoot before the voters sign. Paying whichever voters later appear in a
certificate cannot silently change that already signed root. A valid two-vote
certificate and a valid three-vote certificate could otherwise imply different
balances for the same proposal. Future options include deterministic settlement
of earlier finalized contributions or a separately specified commitment phase;
no option is selected here. Similarly, putting the current block's own hash in
a reward record committed by that block can create a hash cycle. The draft
RewardRecord (§74) needs an explicit source-block/reference and settlement rule.

## 4. What Q1 calls mining

**Q1 does not yet have a complete, approved, implemented mining mechanism.**
The term appears in the tokenomics discussion of “Mining or Production Pools”
(§67), which studies coordination risks; that heading does not define a mining
algorithm. Today's fixed producer and voters perform authorized signed state
transitions. They earn no issuance or distributed fees.

| Necessary component | Historical intent | Present evidence and missing decision |
|---|---|---|
| Scarce contribution | Sequential computational effort; possible HDD access costs | No admitted scarce-resource entitlement. CPU time per challenge does not itself limit identities; disk model/serial/time reports are untrusted |
| Work/evidence | Challenge-bound dependent hash chain, possible formal VDF, optional experimental disk commitments | NONE evidence is exactly `[1, 0, 0, h'', h'']`; no meaningful resource work is required by it |
| Verification | Deterministic recomputation or efficient proof verification | Delay draft §§8–10 specifies a prototype chain and full recomputation; checkpoint sampling explicitly is not a complete cryptographic proof. Formal construction/setup/verifier remain undecided |
| Producer selection | Select an eligible producer, then perform its bound delay; choose finalization participants | Public eligibility, weighting, seed bias resistance, grinding limits and rotation remain open. Fixed local identities cannot establish public Sybil resistance |
| Attack cost | Make specified attacks expensive using verifiable contribution | No measured lower bound or approved cost model. Sequential work for one identity may coexist with many parallel identities; faster hardware, outsourcing, caching and withheld work require analysis |
| Reward earned | Valid finalized producer/delay/validation contribution, possibly treasury share | No approved Mainnet schedule, recipients, shares, maturity or settlement. Local issuance and distributions are zero |

The prototype dependent hash chain is concrete research pseudocode, not an
implemented reward-bearing system. Full recomputation also charges validators
substantial verification cost. The draft's cheap-verification target is a goal,
not a demonstrated property. A VDF concerns sequential evaluation and efficient
verification; it does not by itself specify membership, rewards, or a complete
consensus protocol. Background: [Boneh et al., Verifiable Delay Functions](https://eprint.iacr.org/2018/601).

The HDD draft explicitly permits removing hardware from the design if it adds
no justified benefit. Telemetry-only is its recommended initial research mode;
HDD output alone does not create eligibility, consensus validity, or rewards.
Claims about physical drives need tests against RAM/SSD, virtual or remote
storage, caching, precomputation, replay, wear, and specialized hardware.

Before using “mining” as a description of Q1, approve the resource and threat
model, challenge freshness/binding, proof and verifier, deterministic difficulty,
admission/selection, measured attack economics, earned reward and settlement,
and invalid/equivocating behavior. Local clocks, AI assessments, claimed energy
use, hardware identifiers, or a market price cannot silently become consensus
inputs. Hashing finalized context alone does not prove unbiased randomness.

## 5. Three issuance options — no winner selected

The following is conditional analysis, not approved Q1 policy or measured
economic performance. Issuance and fee redistribution are different operations:
moving existing coins out of a reserve, vesting account, or fee pool is not
new issuance. A cap with future emissions is not the same as all supply existing
at genesis. Burn policy is a separate decision in every model.

An accounting relationship for evaluating the alternatives is
`net supply at h = genesis supply + cumulative authorized issuance through h - cumulative burn through h`.
Fees, reserve releases and vesting unlocks do not increase this net supply.
The older draft calls issued-minus-burned supply “circulating”; a future policy
must distinguish that total from liquid/spendable market float. A predictable
per-height emission is not automatically a predictable annual emission when
block production can stall or change pace.

| Criterion | A. Fixed genesis supply, no later issuance | B. Reward-based progressive issuance | C. Genesis plus progressive rewards |
|---|---|---|---|
| Creation rule | All units explicitly allocated initially | New units earned at defined valid events; genesis may be zero/minimal as explicitly decided | Initial explicit allocations plus later earned issuance |
| Security incentives | Fees and releases of existing reserves must fund service; reserve depletion matters | Issuance can subsidize early service without transaction demand; units alone guarantee no purchasing power | Early reserve and emission can both fund service, with greater accounting complexity |
| Decentralization | Allocation/distribution determine initial concentration; fixed supply does not decentralize control | Accessible participation could broaden ownership; operator scale and pools may concentrate it | Distribution can broaden later, but founder/reserve control may persist |
| Bootstrapping | Allocate operating/faucet funds transparently; zero usage means few fees | Specify how first rewards and empty blocks work, especially when nobody can initially pay fees | Initial balances can pay fees while later rewards fund contribution |
| Founder/project funding | Visible development allocation, sponsorship, or services; selling allocation is a separate act | Project must earn rewards under identical rules, receive an explicit protocol share, or use outside funding | Visible initial funding plus earned rewards or an explicit treasury mechanism |
| Validator/miner incentives | Fee/released-reserve rewards require an actual distribution rule | Reward schedule and eligibility are central; no automatic entitlement from running a process | Requires rules separating reserve-funded payments, fees, and newly issued rewards |
| Supply predictability | Initial gross supply known; later burn, if chosen, changes net supply | Predictable if deterministic schedule/cap; activity/adaptive schedules can reduce predictability | Initial supply plus explicit emission schedule/cap and any burn |
| Sybil resistance | Not provided by fixed supply; if stake used, initial ownership/control is critical | Issuance is not admission control; more identities must not manufacture more entitlement | Must address both initial ownership and duplicate resource/identity claims |
| Delay/resource relationship | Valid work could earn existing fees/reserve units; no need to mint for every job | Valid finalized resource contribution could earn new units, only after proving its security purpose | Can separate startup allocation from resource-earned issuance; avoid double payment for the same contribution |
| Near-zero project capital | Creating allocations costs no external token purchase, but creates no operating cash or user demand | Existing hardware can test rewards; electricity, bandwidth, wear, maintenance and review still cost resources | Initial treasury may hold units without cash; neither component removes operational costs |
| Premine/concentration risk | Entire initial allocation deserves disclosure; concentration depends on recipients and locks | Low genesis allocation does not prevent early insiders/hardware advantages or permissioned reward capture | Both initial allocation and early privileged issuance can concentrate supply |
| Long-term sustainability | Fees must eventually cover costs or finite reserves deplete; demand unproven | Ending rewards creates a later fee dependency; perpetual rewards dilute holdings; neither assures sufficient security | Needs a reserve/emission transition plan and controls against discretionary expansion |

For all three, a resource contribution must have a useful security role before
rewarding it; expenditure alone is not evidence that the chain is protected.
The participant's costs and benefits depend on hardware and usage, while future
unit value is unknown. No monetary attack budget or profitability can currently
be calculated responsibly.

An explicit incentive rule can fund block production with new units and/or
fees, as illustrated in [Bitcoin's original paper, §6](https://bitcoin.org/bitcoin.pdf).
Different protocols explicitly separate proposer and attester rewards and
penalties; see [Ethereum's reward documentation](https://ethereum.org/developers/docs/consensus-mechanisms/pos/rewards-and-penalties/).
These are background comparisons, not authority to import either mechanism,
stake requirement, denomination, rate, or security claim into Q1.

For option B with zero genesis balances, a startup rule is indispensable:
who can participate before anyone owns coins, and can a valid empty block earn
the first reward? Today's body format permits empty transfers, but current
execution still creates no units. Merely changing a reward constant would not
solve admission, fee affordability, or reward authorization.

## 6. Founder and treasury options — no percentages chosen

The draft treasury is intended for research, implementation, audits,
infrastructure and grants, not hidden discretionary minting. Governance draft
§§52–53 proposes separated authority and multi-party controls; even its example
signer structure is not a final public governance decision. Founder status does
not itself establish permanent unilateral authority (§§69–70).

“No external capital” below means no prerequisite purchase of Q1 or investment
round to define the mechanism. It does not mean free hardware, labor, review,
custody, administration, or guaranteed cash revenue.

| Mechanism | Capital and when anything is received | Does funding require sale of Q1? | Concentration/governance risks and prerequisites |
|---|---|---|---|
| Genesis development treasury | Can allocate without outside cash; treasury initially receives units, not revenue; no network usage required for allocation | Obtaining fiat from those units normally entails sale or another separately reviewed arrangement; allocating alone requires no sale | Publish allocation/purpose, spending authority, accounts and reports; avoid one-key control and hidden reserves |
| Founder allocation | No outside capital needed to record allocation; immediately concentrates units unless locked | Allocation itself need not be a sale; converting it to cash is a separate reviewed transaction | Explicit justification, disclosure, custody, conflicts and governance limits; founder allocation is not an earned mining reward |
| Vesting of an allocation | Timing restriction, not a funding source or issuance mechanism by itself | Unlocking does not require sale; cash realization may | Approve beneficiary, start/cliff/duration, revocation and deterministic locks; do not present an off-chain promise as implemented enforcement |
| Protocol treasury fee | Can be specified without investment; receives funds only when fee-paying usage occurs | Fee receipt needs no sale; cash expenses may require conversion | Approve fee incidence and split, on-chain accounting, governance and sustainability; today's fee pool has no treasury payout |
| Project-operated nodes earning rewards | Existing suitable hardware may suffice for tests; operational cost starts before rewards; issuance could reward valid blocks without user transactions, fee-only income needs usage | Earning units needs no sale; paying fiat bills may require conversion | Same public eligibility rules, no privileged hidden rewards; operator concentration, conflicts and custody; first establish the mechanism |
| Commercial engineering/support/private integration | Potentially uses existing skills/hardware and a narrow prepaid deliverable; revenue depends on a customer, not Mainnet transactions | No Q1 token or token sale required | Clear scope, payment/tax/jurisdiction review and honest experimental limitations; avoid unsupported availability/security promises |

Sponsorship or suitable grants can also fund work without Q1 distribution,
subject to the applicable payment and reporting arrangements. None has been
secured by this research. The lowest-capital near-term suggestion remains a
small paid technical service and eligible sponsorship, as compared in
[early revenue options](Q1_EARLY_REVENUE_OPTIONS.md). It is not permission to
open payments or contact customers. Revenue from such services must remain
separate from unapproved promises of future coins, yield, or investment returns.

## 7. Zero-capital bootstrap — proposed gates, not deployment approval

Prefer existing equipment, bounded experiments, reproducible scripts, and
explicit stop conditions over a speculative launch budget. No new hardware
purchase, cloud subscription, token sale, or monetary trading is assumed.

| Stage | Smallest credible experiment and evidence | Gate before advancing |
|---|---|---|
| Public source release — completed | Reproduce the published LOCALNET with its original supply, keys and vectors | Preserve test-only labeling; do not reinterpret its history as a monetary chain |
| Multi-machine nonmonetary testnet — proposed | Inventory already available machines/operators and approved private connectivity; deploy a bounded profile across independent hosts; measure signed transfers, disconnections, catch-up, partitions and restart | Approve transport authentication, peer identities, resource bounds, custody and fault model first. Current loopback TCP cannot just be exposed. Four processes on one laptop do not prove independent failure domains |
| Real resource experiment, synthetic rewards — proposed | Use available CPU/storage; implement only a separately approved candidate; bind challenges, measure work/verifier time, parallel identities, emulation, replay, energy/wear where measurable; run deterministic test issuance/reward accounting | Keep coins synthetic and resettable. Measure useful security properties and adversarial cost; reject the hardware hypothesis if unsupported. Self-reported benchmarks do not become consensus authority |
| Economic and genesis rehearsal — proposed | Compare the three models using reproducible workloads, low/no demand, concentration and declining/ending rewards; publish candidate allocations/locks and a deterministic reward/state replay with synthetic units | Approve model, parameters, attack/fault assumptions, public admission, issuance recipients and settlement; independent review of consensus/cryptography/custody and economic/legal readiness. No automatic conversion of test balances |
| First economically real Q1 — NOT authorized | Only after gates: execute a reviewed, explicit genesis or first reward event using new protected keys/network identity and auditable allocation/issuance evidence | Human approval must identify why each first unit exists, recipient/eligibility, actual contribution or allocation justification, costs, maximum authority, custody and rollback/incident limits |
| Legally reviewed distribution — NOT authorized | Execute only the specifically approved distribution process with applicable disclosures and controls; no assumed sale/listing/trading | Review issuer/offeror, jurisdiction, economics, white paper/disclosures, AML/KYC/sanctions where required, and exchange/listing strategy, including a decision to defer listings |

Legal review is a prerequisite to the **first monetary event**, not a service
to add afterward. A genesis allocation or first reward may itself be a
distribution; if so, the last two stages must be approved together before
either occurs. This is a planning boundary, not a legal conclusion about Q1.
Activity, parties and jurisdictions remain unspecified. Relevant background:
[FATF's risk-based virtual-asset guidance](https://www.fatf-gafi.org/en/publications/Fatfrecommendations/Guidance-rba-virtual-assets-2021.html).
Apply the existing [pre-offer boundary](../releases/PRE_OFFER_BOUNDARY.md).

The existing public-testnet proposal remains unapproved. A private multi-machine
experiment is a smaller intermediate proposal, not an exception to its
security gates. NONE delay cannot be relabeled a real resource experiment;
retaining it for transport tests would require an explicit scoped profile
decision and cannot support mining claims.

Independent review, additional hosts or legal advice may be unavailable without
funding. Volunteer/in-kind contributions are possibilities, not assumed
resources. If those gates cannot be met, remain at nonmonetary testing and fund
the missing work through appropriate services/sponsorship. Near-zero capital
does not justify bypassing a gate or assigning a token a price.

## 8. What is missing and decisions required from the human

No Mainnet winner is selected. The current research supports asking these
questions in dependency order; it does not change decision-register statuses.

1. **Purpose and resource hypothesis:** what useful security contribution should
   earn Q1? Approve a bounded delay-only experiment, optional HDD comparison,
   or a different explicitly specified hypothesis. Accept the possibility that
   HDD evidence adds no value and is removed.
2. **Participation and finality:** define public admission/Sybil resistance,
   producer and committee selection, seed/grinding controls, fault model,
   quorum, rounds/failover, and the role of resources versus authority.
3. **Supply model:** choose A, B or C only after reviewing trade-offs. Specify
   denomination, genesis amount, any cap/schedule/end condition, empty-block
   entitlement, burn policy and controlled version/activation rules. No number
   in LOCALNET or an old example supplies these decisions.
4. **Earned rewards and deterministic settlement:** identify who earns what
   event, what evidence establishes it, how fees differ from issuance, how
   delayed/certificate-based settlement works, and how duplicates, rounding,
   maturity, missed work and equivocation are handled. Resolve state/hash cycles.
5. **Founder/treasury governance:** justify any initial allocation, reserve,
   vesting and project-operated rewards; disclose beneficiaries and controls.
   Separate governance privileges from the possession of units. Choose amounts
   or percentages only in the later approved economic specification.
6. **Small next experiment and resources:** authorize a bounded multi-machine
   nonmonetary milestone, name actually available hosts/operators, and specify
   acceptance and failure criteria. Approve any transport/profile changes
   before implementation. No public validator admission follows automatically.
7. **Before monetary creation or distribution:** approve responsible entities,
   jurisdictions, required legal review/disclosures/controls, protected custody,
   independent technical review, and an explicit genesis/issuance ceremony.
   The first allocation must explain its justification even if it involves no
   mining cost. Do not promise market value, redemption, profitability, or audit
   completion that has not occurred.

The next proposed human decision is the **scope of a nonmonetary resource and
multi-machine experiment**, not a supply percentage or token-sale launch.
The code presently demonstrates deterministic local accounting and recovery.
It does not yet demonstrate a public resource security model or an economically
sustainable reward system.

### Verification of this research checkpoint

On 2026-10-03, the existing `q1-localnet` accounting, state, blocks, policy and
recovery test targets ran: **22 tests passed, zero failed**. Command:

```sh
cargo test --locked -p q1-localnet --test accounting --test state --test blocks --test recovery --test policy
```

The shell did not initially resolve `cargo`; rerunning via the installed
`~/.cargo/bin/cargo` completed successfully. `python3 scripts/check_documentation.py`
reported zero broken Markdown links and zero blocking failures; historical
placeholder/missing-reference advisories remain. `git diff --check` passed.
The change set contains only this research document, `PROJECT.md`,
`CHANGELOG.md` and `genesis/README.md`. No new resource experiment, Mainnet
economic test, full-suite rerun, or remote CI run is claimed for this checkpoint.
