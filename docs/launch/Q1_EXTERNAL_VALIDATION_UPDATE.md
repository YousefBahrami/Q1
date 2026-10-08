# Q1 experimental native research — public source and reproduction

2026-10-08. Native authenticated transport and its research tests are now public
at [source commit `735b3fe`](https://github.com/YousefBahrami/Q1/commit/735b3fe1bc73a73bb40083ff2860b0d6e8dcc19c).
[GitHub CI](https://github.com/YousefBahrami/Q1/actions/runs/37835065229) passed for
that exact commit. The [initial CI portability failure and correction](../reports/Q1_NATIVE_CI_PORTABILITY_20261008.md) remain documented. This is a source/testing milestone, not a new release tag,
Public Testnet, Mainnet or monetary distribution.

## What has been demonstrated

- Published LOCALNET is reproducible: signed transfers, deterministic ledger/state,
  conserved test supply, voter restart and catch-up have executable acceptance.
- Native mutual-TLS authentication, replay reservations, explicit request recovery,
  bounded connection reuse and resource limits have public code/tests.
- One-host native acceptance and its negative/crash tests passed locally and in
  GitHub CI. Reproduce with the [instructions and scope](../releases/Q1_NATIVE_CANDIDATE_CI_SCOPE.md).
- A previous private two-ledger-host experiment demonstrated cross-host ledger,
  producer failover and catch-up using **SSH-carried** protocol traffic. This is
  maintainer-recorded evidence; private operational details are not published.

## What has not been demonstrated

Real native inter-host acceptance; three independent validator hosts; Public
Testnet safety/readiness; Byzantine tolerance; permissionless resource uniqueness;
production mining; Mainnet; monetary issuance/distribution. The private route for
real native inter-host testing remains deferred. One-host CI is not a substitute.
Uncertain intermediate producer work can stop safely instead of recovering
unconditionally. Basic resource limits are not exhaustive DoS resistance. No complete
independent security audit exists. Experimental resource evidence grants no rewards
or extra consensus voting power. No public Q1 protocol port is opened by this work.

## Help produce external evidence

Independent successful **and failed** reproductions are welcome. Negative results,
protocol criticism and reproducible defects are useful findings, not unwelcome feedback.

1. Build/reproduce the published LOCALNET or the pinned native source above. Report
   the commit, OS/architecture/toolchain, command, expected/actual outcome and minimal
   sanitized logs through [GitHub Issues](https://github.com/YousefBahrami/Q1/issues).
2. Review code and distributed-systems assumptions: durable reservations, ambiguous
   requests, certificates, failover, catch-up and bounded network queues. Explain
   counterexamples and the fault model they require; passing tests is not a proof
   of a general protocol claim.
3. Criticize mining/resource research against alias, shared-storage, reconstruction
   and outsourcing attacks. Propose a falsifiable improvement using the
   [proof/service/reward/consensus boundary](../research/Q1_RESOURCE_ECONOMIC_BOUNDARY_V1.md).
4. Express future testnet/node interest in a non-sensitive issue with OS/architecture
   and desired contribution. This is interest only, not admission to a running public
   testnet, promised launch timing or reward entitlement.

Do not post IPs, private topology, credentials, personal records or raw sensitive
logs. Vulnerability details must follow [SECURITY.md](../../SECURITY.md), not public
issues. No investment, purchase, token access or favorable conclusion is requested.

The [external evidence register](../reports/Q1_EXTERNAL_VALIDATION_REGISTER.md)
separates hosted automation from independent participant reports. No independent
participant reproduction is recorded yet. The first independently reported success
or failure will be a milestone with its actual scope and limitations.
