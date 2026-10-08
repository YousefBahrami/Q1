# Q1 TESTNET crash-failover experiment

TESTNET RESEARCH PROFILE ONLY. **Not the released Q1 ledger or a public network.**
This implements a bounded signed replication experiment with two non-voting
producers, three voters and one ordinary learner. Existing LOCALNET Rust state,
headers, votes, fees, delay guards and selection remain unchanged.

TEST ONLY — NEVER USE FOR VALUE-BEARING NETWORKS. The Rust helper uses explicitly
public fixture seeds for Ed25519 signatures. It is not a wallet/signing service.
No dependency from production nodes to this experiment exists.

```sh
cargo build -p q1-testnet-lab-crypto --locked
python3 -m unittest discover -s research/testnet_failover_v0 -p 'test_*.py'
python3 research/testnet_failover_v0/run.py --output /tmp/q1-failover-new-run
python3 research/testnet_failover_v0/run.py --replay docs/reports/research/testnet-failover-v0-2026-10-05/acceptance.json
```

The harness starts six real Python loopback workers and terminates all of them
on exit. Use a new output directory. Roles and genesis context are fixed per run;
fresh chain context means different runs need not share log hashes. Cargo target
directory overrides are supported. Run each worker with one locked journal.

## Executed semantics

Immutable values contain original producer signature, creation ballot, height,
parent and bounded synthetic job text. New coordinators adopt exactly those
bytes; they do not re-sign an original Q1 HeaderBody. Research log roots hash
these values and **are not Q1 StateRoots**. No coins/fees are represented.

Producer schedule is `(height-1+ballot) mod 2`. In the bounded harness, observing
the active producer process exit triggers advancement by one ballot. This is
an executable deterministic crash trigger, not a production timeout/failure
 detector or partition-liveness proof. Ballots never expire durable promises.

Three voters persist promises and accepted values before signing responses.
A coordinator gathers two signed prepares, adopts the highest accepted value
if any, then collects two valid acceptance votes. Full prepare evidence is
verified on acceptance. Recovery can replace a minority-only unchosen value
when a valid higher-ballot prepare permits it; it cannot replace a chosen one.
A fixed quorum never shrinks after disconnect. Durable replay sequence checks
reject identical network requests across restart. A fresh request may retry
an idempotent consensus operation; it cannot count a vote twice.

Sync replays every signed certificate from the fixed genesis, checks original
producer signature, voter membership, distinct signatures, parent/height and
value hash, and compares existing finalized values. Failed sync rolls back.
Journal writes fsync a pending file, rename and fsync the directory; exclusive
OS locking excludes a second writer. A persistence error poisons the worker.
Corrupt committed journals fail closed; pending files are never treated as
committed. Disk rollback by an attacker is outside this crash-only model.

Transport is a four-byte length plus canonical JSON, loopback-only, at most
64 KiB per frame/journal, depth 16, 128 collection entries and two-second server
socket timeout. Processing is serial; queues, authenticated encryption, hostile
Internet fairness and scalable archive storage are not implemented. These
narrow budgets replace the design's larger proposed budgets for this experiment
only. No public-network DoS resistance is claimed.

## Remaining integration work

Map the experiment's coordination envelope to approved Q1 immutable blocks,
certificates, profile/chain guards, deterministic ledger execution and durable
store; establish production round progression and recovery under network
partitions; harden authenticated transport/queues and operate across independent
hosts. The experiment is evidence for that review, not an implicit schema change.
Public Testnet v0 targets permissioned crash tolerance. Public Testnet v1 is a
later candidate malicious-validator/Byzantine-safety milestone. Detection does
not make a three-voter quorum of two Byzantine-safe.
