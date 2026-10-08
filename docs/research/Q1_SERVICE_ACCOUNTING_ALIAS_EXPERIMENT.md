# Useful-service accounting under identity splitting

2026-10-07. Analytical research only. Permissionless resource uniqueness remains
**UNSOLVED**. MORE RESOURCE does not grant MORE CONSENSUS VOTING POWER.

Reproduce: `python3 research/resource_uniqueness_v0/service_accounting.py`.
Two accounting tests are included in the resource-uniqueness suite. The 24 cases
cross six labels (duplicate commitments, shared storage, outsourcing, leasing,
reconstruction and useful delivery) with 1/2/4/8 aliases. These are bookkeeping
cases, not measured implementations of six physical adversaries.

Under externally authorized job assignment and an externally verified receipt,
settling each `(job, unit)` once pays 10 hypothetical accounting units regardless
of alias count. Neither the identity nor physical uniqueness of storage is proven.
A fixed global budget alone fails the desired incentive property: a per-key lottery
against eight other identities increases share from 1/9 to 1/2 with eight aliases.
No probabilistic resource-security or real reward claim follows from this toy model.

| Mechanism | Accounting observation | Missing real-world evidence |
|---|---|---|
| Duplicate commitments | Repeated receipt cannot settle twice | Binding receipt to independently authorized demand |
| Shared storage | Aliases do not create additional buyer jobs in the model | Fair assignment and correlated availability |
| Outsourcing | Delivery could be paid regardless of operator | Service attribution, failures and enforceable obligations |
| Leasing | Rented capacity may satisfy demand without owning disks | Whether payments cover actual useful service, not ownership claims |
| Reconstruction | Reconstructed correct output may satisfy a job | Latency/availability verification and limits of claimed storage service |
| Useful delivery | A genuinely additional job can earn another payment | Buyer Sybils, fake demand, objective verification and disputes |

Authorized demand/assignment and verified receipts are explicit **oracles**, not
implemented cryptographic or market mechanisms. A buyer-controlled subsidy can
create fake demand; an identity-based allocation lottery can remain Sybil-vulnerable
even when settlement is deduplicated. Next research should define who pays, what
independent demand costs, assignment fairness, service verification and adversarial
expected *net* revenue. No storage proof is connected to consensus or issuance;
no reward percentage, token economics or resource-identity solution is selected.
