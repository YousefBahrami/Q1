# Q1_RESOURCE_ECONOMIC_BOUNDARY_V1

2026-10-08. Research consolidation, not an issuance specification. Permissionless
economic-resource uniqueness remains unsolved. No new resource implementation is
approved by this document. More resource does not confer more consensus voting power.

| Layer | What existing evidence supports | What it does not establish |
|---|---|---|
| PROOF | Indexed content bytes match a commitment, challenge and tested response window | Physical disk identity, exclusive ownership, storage location, independent fault domains or unavoidable economic cost |
| SERVICE | Correct sampled chunks/full fixture recovery and an observed response from the lab endpoint | A paying customer, WAN SLA, persistent availability or customer demand |
| REWARD | A hypothetical deduplicated job receipt can be counted once | A monetary reward rule, permissionless fair assignment, profitable service or resistance to fake demand |
| CONSENSUS | Deterministic ledger execution and fixed authorized crash-only test roles | Resource-weighted authority, permissionless admission or Byzantine public-network safety |

Measured behavior includes canonical proof validity, local response/verification
latency, reconstruction, missing/late responses and concurrent readers sharing one
backend. Timing is an observation in the measured environment, not a cryptographic
proof of hardware or cost. Correct bytes and successful full retrieval can already
be checked against known content. Customer satisfaction for arbitrary services, useful
compute correctness and long-term availability do not follow from these fixtures.

[Useful-content v2](Q1_USEFUL_RESOURCE_V2_RESULTS.md) accepted reconstruction and
multiple labels sharing content. [Uniqueness v0](Q1_RESOURCE_UNIQUENESS_V0.md) retained
135 accepted responses across nine capability classes; [concurrent audits](Q1_RESOURCE_UNIQUENESS_CONCURRENT_V0.md)
retain 90 counterexamples. Root normalization alone cannot identify the economic
resource behind differently encoded data. Outsourcing/leasing can provide valid
service while invalidating an inference of owned or exclusive physical capacity.
Fast shared backends, compression and reconstruction still defeat per-identity
capacity rewards. Slower legitimate providers can fail tight timing thresholds.

Customer-authorized retrieval, storage commitments or evaluation work requires an
external demand event: who asked, what result, whose budget and which acceptance
rule. The [24-case accounting model](Q1_SERVICE_ACCOUNTING_ALIAS_EXPERIMENT.md)
assumes demand/assignment and receipt verification as external oracles. Its alias
invariance is conditional bookkeeping, not a solution to buyer Sybils or fake demand.
A global subsidy cap with per-key assignment can still increase an attacker's share.

A possible economic justification is payment for an independently requested and
verified service unit, from its customer's budget, with duplicate settlement
prevented. This does not justify a protocol subsidy, prescribe fees, select an
issuer or grant votes. Reconstructed delivery may be perfectly useful when the
service promises bytes, rather than exclusive physical storage. Distinct customers
can legitimately request the same data; global content dedup must not erase their
actual service demand. Availability assurance/bonds would require separate terms,
verification and dispute rules, not automatic penalties based on a timing sample.

Unsolved: permissionless underlying-resource uniqueness; fair assignment under
aliases; demand authenticity/collusion; outsourcing correlation; actual net attack
cost; objective general-service verification; enforceable service commitments;
permissionless admission and a sustainable reward source. No production mining is
implemented or established by this research.

## Next research gate

**No new mechanism has yet demonstrated improvement sufficient to authorize another
major implementation cycle.** A prospective small experiment must first state a
falsifiable thesis: with the same backend, independently fixed paid workload and
budget, splitting 1 to 8 identities does not increase expected net receipts, while
honest independent providers retain comparable fulfillment opportunities.

Pre-register assignment, demand authentication, oracle/trust assumptions, adversary
control and measurement costs. Compare against current per-key assignment and
receipt-dedup baselines; include duplicate roots, recodings, helpers, leases and
colluding buyer/provider requests. Report aggregate reward/fulfilled unit, duplicate
payments, attack cost, honest rejection and uncertainty across randomized trials.
Any extra alias reward without extra independently authorized useful work falsifies
the claim; a customer/oracle that simply forbids aliases is not permissionless proof.
A written mechanism must explain why it improves these baselines before substantial
code is resumed. No new idea yet is an acceptable result.

## Alignment with locked Q1 economic thesis

This separation supports future resource/service markets, optional assurance and
machine settlement while preserving deterministic verification and independent
consensus authority. The [locked thesis](../strategy/Q1_LOCKED_ECONOMIC_THESIS.md)
is unchanged. No reward percentage, monetary issuance or token distribution is selected.
