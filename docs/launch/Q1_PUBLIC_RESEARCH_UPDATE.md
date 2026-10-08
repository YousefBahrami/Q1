# Q1 research source update — candidate copy

Prepared 2026-10-06; not posted. Q1 source is public and LOCALNET works. This
candidate makes later resource and failover research available for technical
review once its publication is authorized; it is not yet publicly CI-validated.

New resource experiments hold one useful fixture constant while testing nine
ways to share, lease, compress, outsource or relabel its contribution. All 135
recorded access responses passed; missing and late responses failed. Eight
layout aliases of the same underlying content produced eight valid roots.
Commitment deduplication alone therefore does not solve resource uniqueness.
[Measurements and limits](../research/Q1_RESOURCE_UNIQUENESS_V0.md) distinguish
observed access, accounting counterexamples and untested physical-host claims.

Failover is integrated with real Q1 signed-transfer execution and is being tested
under the private crash-only profile. The eight local failure scenarios converged
to equal full StateRoots. [Multi-host preparation](../testnet/Q1_MULTI_HOST_V0_RUNBOOK.md)
now includes role routing, durable controller state and a host-failure matrix.
Its six-process CLI preflight is local; independent-host acceptance is pending.

Mainnet and monetary distribution are not live. No resource-ownership proof,
Byzantine-safety guarantee, reward entitlement or production-readiness claim.
Review the code, reproduce the documented experiments and report non-sensitive
findings via the [canonical repository](https://github.com/YousefBahrami/Q1).
