# Q1 ledger integration: TESTNET_FAILOVER_V0

Controlled private test profile; public fixture seeds; loopback only. No real
value. Two eligible non-voting producers, three fixed voters and one learner.
The Python coordination runner calls the Rust `q1-testnet` backend to execute
actual canonical signed transfers and commit the full Q1 StateSnapshot before
voting, adopting or replaying. It never substitutes a text-log hash for StateRoot.

```sh
cargo build --locked -p q1-testnet
python3 -m unittest discover -s research/testnet_ledger_v0 -p 'test_*.py'
python3 research/testnet_ledger_v0/run.py --output /tmp/q1-ledger-new-run
python3 research/testnet_ledger_v0/run.py --replay /tmp/q1-ledger-new-run/acceptance.json
```

Use a new output directory. Six actual processes are started, killed, restarted
and cleaned up. A fresh chain nonce changes genesis/root each run. Replay the
frozen history with its recorded nonce to reproduce its exact StateRoot.

The before-code [integration map](../../docs/testnet/Q1_TESTNET_FAILOVER_V0_INTEGRATION_MAP.md)
defines the boundary. Existing LOCALNET wire objects are not replaced: a separate
TESTNET signed JSON envelope contains canonical BlockBodyV1 bytes and the expected
full StateRoot. New genesis commits all role keys, supply/allocations and test
policy. Only NetworkClass::PrivateTestnet can initialize this execution capability;
LOCALNET keeps its former constructor, schemas, golden vectors and NONE guard.
No delay witness or mining/issuance rule is added.

For height h and ballot b, producer = `[3,4][(h-1+b)%2]`. The deterministic failure
trigger is **controller-observed process exit** followed by the next ballot, not
a production distributed timeout. Prepare gathers two signed durable promises.
The next producer must adopt the highest previously accepted immutable value,
including its original signature and origin ballot. Votes certify its value hash
at the new coordination ballot. Journal fsync/rename occurs before signed replies.
Workers reexecute every certificate from genesis on disk recovery/catch-up.

Bounds: 16 blocks/history, 64 KiB network/journal, 256 KiB Rust request, known
fixture membership, public fixture signing keys, honest authorized voters and a
reachable majority. JSON must be canonical and signature/context-bound. Replayed
requests, invalid ledger bodies/roots, duplicate/unauthorized votes and conflicting
finalized history are rejected. These checks do not create Byzantine quorum safety:
one dishonest voter in 2-of-3 can break quorum-intersection assumptions. No claim
of Internet transport safety, multi-machine tests or production wallet custody.

The harness tests all eight requested crash/recovery/replay scenarios, verifies
no conflicting finality on honest nodes, checks balances/nonce/pool/supply and
restarts all six journals. Deliberately signed hostile fixtures use test-only key
access; they do not model an attacker stealing production keys.
