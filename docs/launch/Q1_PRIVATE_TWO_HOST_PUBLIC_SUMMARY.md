# Q1 private two-host research checkpoint

**PUBLIC-SAFE DRAFT — sanitized, not published.** Experimental research only.

Q1 has successfully completed a private two-host ledger experiment including
signed transfers, crash/recovery, failover and state convergence.

Three independent validator hosts and native public network transport have not
yet been completed.

The experiment checked test-balance supply conservation and fee accounting.
Additional bounded testing covered delayed peers, a dropped test connection,
replayed/malformed messages, invalid synchronization history, stale rejoin and
rejection of a corrupted copy of test state.

The result covers two real ledger hosts with SSH-carried protocol traffic. Private
role placement and operational topology are excluded. Process restarts do not
establish OS reboot, power-loss or physical host-loss behavior. Native transport
is now implemented and tested locally; real native inter-host acceptance remains
pending. See the [current update](Q1_EXTERNAL_VALIDATION_UPDATE.md).

Q1 remains an experimental permissioned, crash-fault-target test system. These
results are not a Byzantine-tolerance or security-audit claim. No public network,
mining reward or real-value issuance is launched.

Participation now: review the [published source](https://github.com/YousefBahrami/Q1),
reproduce its documented LOCALNET release, and follow technical development.
Private experiments described here are not automatically included in that release.
