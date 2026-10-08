# Resource uniqueness: job demand and reward invariance boundary

2026-10-07. ANALYSIS ONLY; no new mining, voting weight or reward implementation.
The [90 concurrent proofs](Q1_RESOURCE_UNIQUENESS_CONCURRENT_V0.md) remain reproducible
counterexamples, not evidence of permissionless physical-resource uniqueness.
No new benchmark measurements are asserted here. The locked economic thesis stands.

| Adversary | What a successful proof establishes | What it cannot establish | Next analytical/test condition |
|---|---|---|---|
| Same storage, many identities | Correct challenged bytes were available | Distinct physical storage per key | One customer-authorized job/epoch budget across identities, not one budget per key |
| Outsourced proof service | A response reached a verifier within its window | Local ownership or independent infrastructure | Price actual latency/availability delivered; model correlated helper failure |
| Resource leasing | Access during the audit window | Exclusive durable ownership | Define service duration, interruption and prepaid-budget handling without assuming exclusivity |
| Duplicate useful content | Content commitments and requested data match | Independent redundancy merely from distinct commitments | Pay for requested replication only; fault-domain evidence remains an external trust problem |
| Reconstructed content | Useful bytes can be regenerated in time | Persistent storage or HDD contribution | Decide whether the customer buys timely retrieval or physical retention; retrieval may legitimately include reconstruction |
| Key splitting | Multiple valid identity-bound responses | Multiple independent service demands | Evaluate aggregate claimant receipts, including cost of generating keys/challenges and colluding aliases |

A candidate **accounting condition**, not an approved reward rule: for a single
buyer-authorized job j, epoch e and maximum service budget B(j,e), aggregate settled
payments across all aliases must not exceed that budget. If a claimant can split
into k keys without delivering additional requested service, its expected aggregate
payment must not increase solely because of k. Bounding total spend alone is not
enough: a lottery over identities can redistribute that bounded budget toward a
Sybil operator. Deduplicating a content hash is also insufficient because legitimate
customers may request additional replicas or independently purchase the same data.

To test this without pretending uniqueness is solved: define externally authorized
job IDs and explicit demand quantities; adversarially submit the same acceptance
receipt across keys, jobs, epochs and settlement retries. Track distinct buyer demand,
accepted service units, aggregate payment liability and marginal operator cost.
Compare a per-key lottery with a demand-capped settlement rule, including hostile
buyers that create cheap jobs and outsource all delivery. These are next experiment
inputs; neither demand authentication nor monetary settlement is implemented here.

A permissioned buyer/operator can limit budgets and reject duplicate receipts under
its authority. This creates a trust/admission boundary, not permissionless mining.
A timing threshold cannot prove physical separation over arbitrary networks. An
independence attestation, hardware identifier or deposit may change assumptions but
needs explicit privacy/security/economic decisions; none supplies votes today.

Decision still open: what useful service is purchased, by whom, for how long, how
failed delivery/disputes are resolved, and whether any native issuance is justified
in addition to customer payments. Commercial reproduction fees can fund this work
without issuing coins. Storage quantity remains disconnected from consensus power.
