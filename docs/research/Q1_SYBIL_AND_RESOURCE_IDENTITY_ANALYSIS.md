# Q1 Sybil and resource identity analysis

Date: 2026-10-05. Research proposal; no Mainnet identity, reward percentage,
proof-of-stake rule or KYC admission requirement is selected. Identity splitting
is a **priority protocol blocker**, not a benchmark detail.

## Implementation checkpoint

The [v1 result](Q1_MINING_DELAY_EXPERIMENT_V1_RESULTS.md) now measures zero
additional credit across 15 key/commitment/budget presentations of the same
assigned units. It relies on a trusted allocation oracle and does not close
permissionless resource uniqueness. No economic credit becomes consensus power.

## Observed attack and honest comparison

The [v0 experiment](Q1_MINING_DELAY_EXPERIMENT_V0.md) compared eight small owners
of one unit each with a large owner of 64 units. With fixed epoch budget 7200,
the large owner's unsplit shares were 11.11% (equal per identity), 88.89%
(proportional units), 33.33% (cap four per identity), and 50% (square root).
Splitting its same 64 units into 64 identities raised all four shares to
88.89%. No new underlying resource was required by the simulation. The
proportional case did not show this particular advantage; that does not prove
its input units correspond to independent scarce resources.

An owner field in a simulation is an omniscient test oracle, not a protocol
capability. Sixty-four independent one-unit owners and one owner operating
64 keys can present identical public records. Private keys prove control of
keys. They do not prove independent economic ownership.
[Douceur's Sybil analysis](https://www.microsoft.com/en-us/research/publication/the-sybil-attack/)
is relevant background on identities and resource assumptions, not a proof
that every resource-based scheme is impossible or that Q1 solves the problem.

## Splitting invariance requirement

Let x be a fixed independently accounted contribution. Require the same total
credit when attributed to one key or partitioned across keys:
`F(x) = sum(F(x_i))` for partitions with `sum(x_i)=x`.
With ordinary nonnegative, monotone weights, linear contribution weights meet
this additive requirement. Per-key equality, caps and concave weights do not:
`sqrt(64)=8`, while `64*sqrt(1)=64`; `min(64,4)=4`, while
`64*min(1,4)=64`. Normalizing into a fixed reward pool preserves the attack
against other participants. Open issuance additionally increases total output
when an identity-dependent total weight increases.

Linear accounting leaves large resource owners with large shares. It is not
equal-per-person fairness, decentralization or a profitability guarantee.
Giving small **owners** a bonus requires credible owner grouping; free keys
cannot supply it. Pooling several honest owners reverses the splitting test
and must not destroy total contribution credit either.

Candidate research accounting: credit accepted atomic resource units first,
deduplicate `(allocation, unit, epoch)`, then aggregate by beneficiary key.
Any integer rounding or fractional carry belongs to those stable units, not
newly generated identities. A per-identity largest-remainder allocation can
create rounding advantages even when the underlying real-valued weights are
linear. Test exact integer conservation and all regroupings; no signup bonus,
minimum per-key payout or per-key cap may silently bypass this rule.

This is a requirement for a future simulator, **not an implemented fix**.
A unique root is not a unique resource: labels, encodings and keys can produce
different roots over reused bytes. Independent random assignments in v1 reduce
that particular alias within a trusted lab but do not prove unique hardware,
exclude outsourcing or create a permissionless scarce-resource allocation.

## Resource identity candidates

Assessments are conditional design judgments; no candidate is selected.

| Candidate | Sybil cost | Forgeability | Transferability | Centralization risk | Verify cost | Entry cost | Privacy | Specialized hardware risk |
|---|---|---|---|---|---|---|---|---|
| Node identity / endpoint | Almost free additional processes or endpoints | Signed endpoint control can be checked; one physical machine cannot | Keys/endpoints can move | Hosting concentration and IP-based exclusion | Low signature/connection cost | Existing machine/network | IP/uptime exposure | Low cryptographic need; network advantage remains |
| Cryptographic participant key | Key generation is cheap | Signature forgery hard under assumptions; creating aliases easy | Secret transfer or delegated signing | Per-key rewards favor large alias operators | Low | Very low | Pseudonymous but linkable | No necessary special device |
| Storage commitment | Hash/root creation alone is cheap; data cost depends on assignment | Membership forgery hard; fictitious capacity claims remain possible | Data/root service can be outsourced | Shared providers and precomputation | Sample proofs plus metadata | Data generation/storage | Access patterns and assignments link participants | RAM/SSD/accelerator advantage possible |
| Independently measured resource unit | Must incur additional independently verified contribution, if measurable | Duplicate/alias units are the central unsolved risk | Beneficiary may change; freeze attribution per epoch | Allocation authority or measurement monopoly | Audits, global deduplication and accounting | Bounded corpus/network capacity | Cross-key unit linkage | Efficient storage/compute can dominate |
| Hardware-bound commitment / attestation | Device purchase if manufacturer identity is reliable | Cloning/compromised roots/relay must be addressed | Resale and remote attestation service | Manufacturer roots and device availability | Certificates, revocation, challenge checks | Hardware and replacement costs | Stable device identifiers | High vendor lock-in and exclusion |
| Stake/resource hybrid — comparison only | Capital plus resource under specified rules | Keys do not prove lawful ownership; borrowed/delegated capital complicates attribution | Usually transferable/delegable | Wealth and custody concentration | Stake state plus resource checks | Monetary capital would be required | On-chain holdings linkable | Hardware advantage may remain |
| Other measured contribution, e.g. delivered bytes or useful compute | Actual delivery/computation only if independently audited | Self-dealing traffic, recycled jobs and colluding clients | Outsource work or sell service | Job allocator/customer monopoly | Useful-work verification may be expensive | Workload-specific bandwidth/compute | Client/data confidentiality issues | Specialized accelerators or bandwidth providers |

Hardware attestation is an alternative to research, not a substitute secretly
added to v1. Stake and real-world identity are not approved shortcuts. Neither
hardware serial numbers nor a fee for creating a key proves owner uniqueness.

## Explicit adversarial cases and required outcomes

| Attack / control | Future experiment | Required interpretation |
|---|---|---|
| One owner, one key versus 64 keys | Keep exact same units, deadlines, budget and availability | Equal total accepted credit before and after regrouping |
| Many genuinely independent owners | Same unit records with different oracle owner labels | Protocol output cannot change merely because the simulator knows owners |
| Large versus small resource owner | Compare 1, 64 and 256 units, then all partitions | Report proportional concentration separately from split advantage |
| Root relabel / chunk permutation | Commit same underlying bytes under new identifiers | Root count alone must not mint extra unit credit |
| Shared service / outsourced proof | Many keys query one storage backend | A pass does not demonstrate independent copies or operators |
| Replayed unit / challenge | Submit one proof repeatedly, across keys or epochs | Reject context mismatch; never credit the same assignment twice |
| Selective answering | Answer cheap challenges, reconnect after hard ones | Count every issued challenge; no reset of denominator or allocation |
| Grinding and precomputation | Generate keys/roots to choose favorable samples | Commit assignment before independent nonce; measure leakage/grinding possibilities |
| Rounding / minimum payout | Split near integer boundaries with tiny budgets | Exact aggregate invariance or an explicit resource-unit-based error bound independent of key count |
| Transfer / pooling | Change beneficiary at epoch boundary and combine owners | One authorized beneficiary per unit/epoch; no double attribution or key-count bonus |
| Correlated hardware / availability | Concurrent challenges to aliases on one host | Expose shared failures; do not reinterpret them as proof of independent hardware |

Unit transfer rules, challenge scheduling across participants and assignment
authenticity need concrete designs before coding. Concurrent deadlines can
measure contention but cannot prove that identical bytes occupy separate
physical copies. Distinct random corpora entail different logical information;
they still permit efficient storage, RAM and remote service.

## Exit criteria and unresolved decisions

Before resource evidence affects rewards or eligibility, require: a stated
scarce contribution; threat model for assignment and collusion; independent
measurement and anti-duplication argument; exact splitting/pooling invariance
tests including rounding; adversarial v1 evidence; and review of centralization,
privacy and honest entry costs. A simulator pass closes only accounting tests.

Current result: Q1 has a reproducible evidence lab and a demonstrated Sybil
weakness in several reward models. It does **not** have production mining,
proven Sybil resistance or economically justified issuance. Continue
[v1 design](Q1_MINING_DELAY_EXPERIMENT_V1_DESIGN.md) and keep all experiment
outputs outside consensus and monetary state. Mainnet identity remains open.
