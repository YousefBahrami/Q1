# Q1 experimental network research update

**DRAFT — prepared for review, not posted.** 2026-10-07.

Q1 has completed private two-host ledger experiments using an SSH-carried test
transport. A separate permissioned native transport implementation now carries
Q1 test messages directly over mutually authenticated TLS 1.3. Native testing so
far runs on one host with fresh test credentials; it does not establish native
communication between independent hosts.

The native local experiment reproduced signed transfers, a replacement producer,
progress with two of three voters, voter restart, stale-peer catch-up and common
final state after all processes restarted. Fee and supply invariants remained
unchanged. Negative tests cover invalid identity, network, signature and frame
inputs, plus replay after restart. Ambiguous requests remain reserved and fail
closed; full operational recovery and public load defenses need further work.

Public Testnet remains gated. Native two-host testing awaits a suitable private
route, and three independent validator hosts remain unproven. These results do
not establish Byzantine-voter tolerance, permissionless resource uniqueness,
production readiness or independent security-audit completion. All balances are
experimental test state; no monetary issuance or sale is enabled.

The next engineering step is direct private native inter-host acceptance, followed
by independent-host and public-node safety work. Follow development or review the
[published experimental source](https://github.com/YousefBahrami/Q1). This native
implementation has not yet been published; existing released source has its own
version and limitations. No host details, operational scripts or private evidence
are part of this draft update.
