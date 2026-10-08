# TESTNET_FAILOVER_V0 — actual Q1 ledger results

Measured 2026-10-05/06. PUBLIC technical evidence, not a public-network release.
[Before-code mapping](Q1_TESTNET_FAILOVER_V0_INTEGRATION_MAP.md),
[runner and assumptions](../../research/testnet_ledger_v0/README.md),
[frozen acceptance](../reports/research/testnet-ledger-v0-2026-10-05/acceptance.json),
[checksum](../reports/research/testnet-ledger-v0-2026-10-05/SHA256SUMS).

## Implemented and observed

Six actual loopback processes: two eligible non-voting producers, three fixed
voters and a learner. Majority prepare/adoption, durable reservations, contextual
Ed25519 signatures, exact canonical signed transfers, full-state commitment,
atomic journals and authenticated catch-up/replay. The Rust backend reuses actual
Q1 accounting, including whole-block rollback, nonce/signature/fee checks and
supply conservation. No Python substitute for ledger execution. No added numeric
domain or relaxed DelayEngine::NONE guard. PrivateTestnet capability is explicit.

| Required failure | Observed result |
|---|---|
| Producer exits before proposal | Exit 71; replacement at next ballot finalized first signed transfer |
| Producer exits after proposal, before votes | Exit 72; next producer finalized height 2 |
| Producer exits after majority, before publication | Exit 73; replacement adopted exactly the previously chosen value, ignoring its different requested amount |
| Replacement restarts | Persisted height/root/history recovered unchanged |
| Voter + producer crash | Two remaining voters finalized; absent voter and learner later caught up |
| Stale producer returns | Old-height prepare rejected; restored producer replayed current state |
| Conflicting proposal | A second independently valid transfer could not replace the same-ballot reservation, including after voter restart; conflicting finalized history rejected |
| Network replay after failover | Previously accepted request rejected after durable restart |

All six processes ended at height 4: sender 956, recipient 40, sender nonce 4,
reward pool 4, supply 1000 unchanged. Every final StateRoot:

`dd3009600af6ed5a288103509a5280fdf953365121d1948c724828874684d9e1`

All six subsequently restarted from disk and reproduced that root. This is a
full Q1 StateRoot, not the old research-log digest. A new randomly generated chain
nonce changes genesis and roots across new runs; frozen replay uses the recorded
nonce and certificates and must reproduce this exact root.

## Tests and limits

`python3 scripts/check_all.py` passed: workspace format/Clippy (warnings denied),
Rust tests/build/Rustdoc (warnings denied), independent protocol/LOCALNET vectors,
real four-process LOCALNET acceptance/CLI, historical conformance, prior v0/v1
and isolated failover tests, three useful-resource tests, six ledger-backend
adversarial tests, integrated six-process acceptance and authenticated replay.
The new Rust backend adds three profile/genesis/overflow tests. Documentation
retains historical advisories; blocking failures are zero.

Initial integration execution failed because genesis used CBOR text not permitted
by Q1 policy. It now uses canonical byte strings for profile/policy markers; the
strict existing CBOR policy was preserved. A wrong expected value-field count and
an unused import were also corrected before successful acceptance/full checks.
No failed run is counted as evidence of success.

LOCALNET retained its accepted final root
`b7ec7d47f4cf37d029e74bab02d8bc2aab191f5a5733f92ac4310d919183b087`.
Existing LOCALNET schema/vector bytes are unchanged. Resource evidence is not a
voting weight, mining reward or service payment.

Safety assumes honest authorized voters, fixed membership and durable storage
without rollback. Two-of-three is not tolerance of one Byzantine voter. The
controller observes process exit and deterministically advances the ballot;
a distributed production failure detector, partition liveness, public transport,
independent machines and general wallet/transaction submission are not delivered.
The new runner uses public fixture keys and bounded histories, not production
custody. No Mainnet, monetary issuance or public Testnet is activated.

New integration code was verified locally. Public documentation CI cannot certify
this unpublished code; remote publication/testing of the implementation is a
separate reviewed source update. No background worker is left running.
