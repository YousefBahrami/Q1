# Q1 experimental research: invitation to reproduce and review

Prepared 2026-10-08 for a public source candidate. Not a new release announcement;
publication and remote CI of this candidate are pending. The existing
[v0.1.0-localnet.1](https://github.com/YousefBahrami/Q1/releases/tag/v0.1.0-localnet.1)
release remains experimental LOCALNET.

## What has been demonstrated

LOCALNET builds and reproduces signed transfers, deterministic state, conserved test
supply and voter restart/catch-up. The research tree adds native mutual-TLS peer
identity, durable replay reservations, explicit uncertain-request recovery, bounded
connection reuse and basic resource limits, tested on one machine. An earlier private
two-host ledger experiment carried protocol traffic through SSH; operational details
are withheld. That result is a maintainer-recorded experiment, not independent
external validation. Native local tests do not replace native inter-host evidence.

## What remains unproven

Real native two-host networking; three independent validator hosts; public-network
safety; Byzantine tolerance; permissionless resource uniqueness; production mining;
Mainnet; monetary issuance and token distribution. Ambiguous intermediate producer
work can safely stop rather than recover automatically. Bounds are basic defenses,
not exhaustive DoS protection. No complete independent security audit exists.
Public Testnet is blocked, no public Q1 protocol listener is enabled, and research
resource proofs grant neither rewards nor consensus voting power.

## Participate now

- Reproduce the published LOCALNET and report commit, OS/toolchain, command and
  expected/actual result in [GitHub Issues](https://github.com/YousefBahrami/Q1/issues).
  Remove private paths, IPs, keys and personal data from logs.
- Review the candidate's code and frozen vectors when its exact reviewed revision
  is made available. Current public main must not be assumed to contain it.
- Review distributed-systems assumptions: durable reservation ordering, ambiguous
  requests, certificates, crash recovery and bounded queues. Cite a reproducible
  counterexample; distinguish a fault model concern from an observed implementation bug.
- Register non-sensitive interest in a future testnet/node role via an issue with
  OS/architecture and preferred contribution. Do not post an IP, credentials or
  home topology. This is not enrollment, a running public testnet or promised rewards.
- Contribute resource/mining research by proposing a falsifiable improvement over
  alias, shared-storage, reconstruction and outsourcing attacks; consult the
  [economic boundary](../research/Q1_RESOURCE_ECONOMIC_BOUNDARY_V1.md).

The requested result is external technical evidence, including negative findings.
No investment, token purchase or favorable conclusion is requested. Security
vulnerability details must follow [SECURITY.md](../../SECURITY.md), not public issues.
