# Q1 TESTNET crash-failover experiment results

Date: 2026-10-05. PUBLIC technical evidence. Six actual loopback processes:
two non-voting producers, three voters and one ordinary learner. The
[implementation](../../research/testnet_failover_v0/README.md) uses real Ed25519
signatures, canonical bounded messages and durable locked/fsynced journals.
[Acceptance evidence](../reports/research/testnet-failover-v0-2026-10-05/acceptance.json)
includes the complete signed finalized history and source hashes.

## Observed behavior

- All roles agreed after the initial synthetic work item.
- One voter and the ordinary node were stopped. The active producer exited
  after two acceptance votes, before publishing a finalization certificate.
- The second producer advanced the coordination ballot after observed process
  failure, obtained signed promises, and adopted the exact previously chosen
  value. Its certificate contained exactly two distinct voter signatures.
- Restarted voter, producer and ordinary learner verified history and caught up.
- With two voters stopped, one voter could not finalize; the finalized log did
  not change. Restart and retry progressed. All six processes then recovered
  from disk with the same four-item finalized log.
- Duplicate votes, conflicting votes, invalid signatures, unauthorized voter,
  wrong proposal votes, stale/future heights, malformed certificates, oversized
  frames and replayed messages were rejected. Equivocation remained rejected
  after voter restart. Detection was exercised both by unit checks and actual
  loopback requests; the oversized length was rejected before reading a body.

Final **research-log root**, not Q1 StateRoot:

`d3de624af16d6011b3b0c7ac35d94d953a3c3813c6df8e9f25dbb5c5b876eb44`

Ten unit tests additionally cover accepted-but-unannounced adoption, late ballots,
minority-only reservations versus a later majority, invalid-sync rollback,
conflicting valid certificates, interrupted persistence, corrupt-store refusal
and cross-chain messages. Positive history replay independently re-verifies all
signatures and parents. These finite scenarios are not a formal consensus proof
or exhaustive scheduler exploration.

## Scope and remaining engineering

Public Testnet v0's approved target is a **permissioned crash-fault-tolerant
validator set**. Public Testnet v1 is a later candidate malicious-validator /
Byzantine-safety milestone. With three voters and quorum two, two quorums can
intersect in one malicious voter; that voter can support conflicting histories.
Honest durable locking and trusted authorized voters underpin the current
experiment. Equivocation detection alone cannot turn that quorum into a
one-Byzantine-tolerant design.

This is a separate TESTNET-profile replicated research log carrying immutable
signed synthetic work, not modified Q1 block/header/certificate schemas or a
new cryptocurrency ledger. No transfers, fees, resource weights or monetary
issuance occur in this experiment. The existing LOCALNET signed-transfer
acceptance remains its own test. Producer recovery integration into the Q1
ledger remains unimplemented and must reconcile header round/producer binding.

The failover trigger observes a harness-controlled process exit. Production
network failure detection, timeout progression, partitions, encrypted transport,
queue fairness and multi-host independence remain untested/unimplemented here.
Loopback processing is serial with a 64 KiB message/journal bound and socket
timeouts; it is not ready for hostile Internet deployment. A replay sequence
journal detects requests across restarts but provides no disk-rollback defense.
All fixture keys are public and all spawned workers are terminated by the
harness. No public network or Byzantine fault claim was launched.

This refines the [failure-model design](Q1_PUBLIC_TESTNET_V0_FAILURE_MODEL.md)
within the authorized bounded experiment; its broader public-hardening budgets
are not claimed as implemented. The public deployment gate stays closed until
ledger integration, hardening and independent-host acceptance are complete.
