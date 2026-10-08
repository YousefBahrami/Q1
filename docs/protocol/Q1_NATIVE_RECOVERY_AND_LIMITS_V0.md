# Native recovery and bounded sessions — TESTNET implementation record

2026-10-07. Authority: human next-phase instruction after b8ecb63.
No consensus/quorum/economic change. This extends the private native adapter;
public exposure remains prohibited. Status and test evidence are recorded separately.

An explicit RECOVER frame uses type 3 and the original request's sequence,
operation, exact signed payload, chain/genesis and authenticated peer pair.
Its digest is that of the original type-1 REQUEST. It is not a new transaction.
The sender holds its pending body until a conclusive response is durable locally.

Receiver outcomes: NOT_RECEIVED (5) only if the sequence exceeds its high-water
reservation; completed result (0) from an atomic application receipt; unknown (3)
if work began but no conclusive receipt exists; retired/conflict (1/2) fail closed.
The receiver can retry the original dispatch during explicit recovery if the
serialized worker proves it never began that request. Ordinary duplicate REQUESTs
retain rejection semantics. RECOVER never invents a new nonce, transfer or ballot.

Native workers persist request digest + exact signed result together with their
journal state using one fsync/rename. Only serialization/commit hooks are added
to the existing coordinator; legacy journal bytes and ledger decisions are retained.
Interrupted producer coordination without a final receipt returns STOP with current
verified state: it must not rerun an amount against a later height. A receipt is
execution evidence from an authenticated peer, not a new finality certificate.

A bounded local dialer retains one outgoing TLS session per configured peer.
Incoming/outgoing sessions are separate, allowing simultaneous directions without
reentering the synchronous worker. This is a paired-session prototype, not a
multiplexed single-stream protocol. One active call per outgoing peer; backpressure
rejects concurrent excess instead of building an unbounded queue. Legacy one-shot
clients remain usable. Neither arrival order nor session ownership grants votes.

Implemented limits: two pending handshakes, eight authenticated sessions, two inbound
sessions per peer, one dispatched request at a time, bounded frame buffers and
per-peer request windows; shared diagnostic rate cap; 5s handshake/partial-frame,
30s idle; bounded outgoing/local queues. Real measured results must identify
limitations; this is not exhaustive DoS resistance or a public-launch approval.

## Verified behavior and limits — 2026-10-07

The full local check passed, including 30 native Python tests and seven native
Rust tests. Exact local barriers exercise before-receive, received-before-dispatch,
after-vote, after-finalization-before-ack, sender restart and receiver restart.
These barriers are local test environment hooks, never peer-controlled commands.

`pending CONFIG PEER` exposes request identity/stage without keys. `recover CONFIG
PEER` uses the durable original identity. A completed sender response is saved before
CLI output; repeating recovery after the pending record clears returns that saved
result. The bridge explicitly reconciles an older pending request before allowing
a new operation. Failed reconciliation blocks replacement. Only the latest receipt
per authenticated sender is retained; retired requests fail closed.

Resource policy: two pre-auth slots, eight authenticated inbound sessions, two
inbound sessions per peer, six local dialer handlers and at most one outgoing
session per peer. One active call per outgoing peer; contention returns backpressure.
Each active exchange reserves two 64 KiB buffers from a shared 1 MiB budget;
there is no unbounded request queue. This is a network-buffer reservation ceiling,
not a whole-process RSS ceiling. TLS buffers are additionally capped at 64 KiB per
session; library/parser/thread overhead is outside the 1 MiB accounting. Native
worker receipts are separately bounded at 1 MiB; each result is at most 64 KiB.
Sender journal size remains bounded; oversized combined records fail closed.

Per-peer frames: 32 per fixed one-second window. Diagnostics: 20 per fixed window,
512-character detail cap, suppressed-event count. Window boundaries allow bursts;
disk log rotation remains an operator task. Handshake/HELLO and partial frames have
5-second phase bounds; authenticated idle connections close after 30 seconds.
Pre-auth saturation tests show existing authenticated progress; they do not prove
new-peer fairness under sustained/distributed attack or exhaustive DoS resistance.

Tests cover reuse, stale connection, peer restart, bidirectional simultaneous calls,
same-peer connection/call contention, rate limits, idle expiry and RAII budget release.
Two directional sessions deliberately avoid synchronous application reentrancy;
there is no single-stream multiplexing or lexicographic initiator implementation.

The native journal wrapper requires fresh TESTNET state; automatic migration of
older native journals is not supported. All peers must use the same reviewed build.
RECOVER extends v0 with type 3/status 5; mixed-build rolling compatibility is not
established. Frozen original HELLO/request/response vectors remain unchanged;
recovery is covered by real TLS integration tests, not an additional frozen vector.
Intermediate producer work with only a started receipt stops for reconciliation;
automatic reconstruction of unfinished coordination remains open. No exactly-once
network delivery claim is made. Consensus/state execution rules are unchanged.
