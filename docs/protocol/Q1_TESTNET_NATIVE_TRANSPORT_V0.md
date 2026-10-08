# TESTNET_NATIVE_TRANSPORT_V0

Status: **Limited permissioned implementation; one-host native tests passed.**
Updated 2026-10-07 following explicit human implementation approval. Real two-host
native acceptance is BLOCKED: no private route exists and the human instructed
continuing without public ports. This document retains the reviewed target below;
the implementation record distinguishes implemented bounds from pending targets.
No Mainnet, public network or Byzantine-consensus claim. Legacy LOCALNET bytes,
committee, fees, rewards and accepted SSH evidence remain unchanged.

## Current implementation record (2026-10-07)

The [recovery and limits extension](Q1_NATIVE_RECOVERY_AND_LIMITS_V0.md) supersedes
the initial limitations below: explicit durable RECOVER, atomic application receipts,
paired persistent TLS sessions, pre-auth/per-peer quotas, frame/log rate bounds,
idle expiry and shared network-buffer reservations are now implemented and tested
locally. All peers must use the same build; rolling compatibility is unproven.
Explicit private addressing also permits 100.64.0.0/10 for vendor-neutral overlays;
address admission does not replace mTLS identity or ledger membership.
The human chose to continue locally and select the private route later. No native
inter-host test, overlay installation or public exposure occurred.

## Historical initial record — b8ecb63

The following describes the accepted initial PARTIAL checkpoint. Its unimplemented
recovery/pooling/limits statements are historical; the extension above is current.

`crates/q1-native-transport` pins rustls 0.23.45 with ring and TLS 1.3 only.
`research/testnet_native_v0` provisions fresh TLS and ledger keys and bridges the
existing TESTNET coordinator over a private Unix socket. Literal private/loopback
addresses are required; configured DNS names are certificate expectations only,
never resolved for routing. Native-required mode fails closed without keys.

Implemented HELLO/request/response bytes are the reviewed arrays. Three frozen
schema vectors are in `vectors/testnet-native/v0/approved.tsv`, checked by Rust
and independent Python reconstruction; payloads there are synthetic schema examples,
not authenticated ledger messages. Full canonical signed payloads are exercised
by integration tests. Maximum frame is 65,536 bytes. Status 3 specifically means
APPLICATION_UNCERTAIN after a reservation and failed worker dispatch; status 0 can
carry a signed application result or an application rejection. Status 4 remains
reserved; resource exhaustion currently drops the connection.

The first implementation opens one authenticated TLS connection per RPC, closes it
after response and keeps durable counters across connections. It does not yet pool
bidirectional sessions or implement lexicographic single-connection arbitration.
This bounded subset avoids reentrant request ownership in the synchronous ledger
adapter; persistent session multiplexing requires separate validation. There is no
idle-session queue: at most eight total connections (including handshakes), each
with at most one 64 KiB input frame; no unbounded application queue. Separate two
pending-handshake slots, per-peer admission fairness and rate-limited diagnostics
are unfinished target features and public-exposure blockers, not claimed complete.

Deadlines use nonblocking sockets checked against monotonic absolute phase time,
with 1ms waits when I/O would block. An initial repeated socket-option approach
failed locally with EINVAL; it was replaced and the TLS tests rerun. Connect is 3s;
TLS/HELLO/frame 5s; application response 15s. Five reconnect attempts reuse exactly
the same request, with 250/500/1000/2000ms plus 0–25% random backoff. No retry elects
a producer. After an unsuccessful peer RPC, the bridge skips that peer for the
remainder of that single coordinator operation, preventing prepare/accept/sync
from each exhausting the same timeout; the next operation can reconnect afresh.
Membership and required certificate checks never change.

Both send counters/pending bodies and receive sequence/digest reservations persist
before delivery. Missing/ambiguous application results retain the pending operation
and block a different request. A duplicate never redispatches, including after
adapter/worker restart. Explicit recovery tooling for an uncertain operation is
not implemented: stop and preserve evidence; do not delete reservations or invent
a new transfer. This safely stalls the affected sender/recipient path, and is an
operational liveness gap before inter-host deployment. Status via another enrolled
role and independent journal replay can inform a future reviewed recovery action.

`stop CONFIG` writes a private stop marker; the server stops accepting, drains its
bounded handlers and exits. The harness then terminates the worker with its signal
handler. Remove only that stop marker before restarting with the same journals.
SIGKILL tests are crash recovery, not graceful shutdown. No autostart is installed.
JSON logs contain events/rejection causes, no request/key material. Runtime paths
and error details are internal and must be sanitized before sharing logs.

Fresh genesis commits new test public keys and chain nonce, so native and historical
SSH roots differ. Compare fee=1, no issuance/burn, 2-of-3 certificates, balance/nonce
and root convergence within each run, not equality across different genesis states.
Two producers share a fresh test-faucet sender key to reproduce the transfer test;
this is explicitly test custody, never production wallet custody. TLS identity
rotation can preserve the ledger public registry; ledger-key rotation requires a
new test genesis in v0, not an invented live committee transition.

## Scope and threat boundary

PUBLIC TESTNET v0 = permissioned participants + crash-fault target.

Authenticate transport peers and reject malicious input without claiming Byzantine
consensus safety. A partition can stop liveness; disconnect never changes committee
membership. Two voters on one host remain one host failure domain. Resource size
never supplies voting power. TCP/TLS delivery is not finality or an exactly-once
application guarantee. An authorized malicious voter, stolen key, disk rollback,
Sybil admission, global denial of service and public discovery remain outside the
crash-fault claim.

Replace SSH forwarding with a Rust transport adapter connected to the existing
verified ledger/coordination interface. Keep the current SSH harness untouched as
regression evidence. Normal peer traffic must use TCP/TLS directly; shell/SSH may
still be used for operator installation and log collection, never to carry normal
native-profile protocol messages.

Default bind is loopback. Future non-loopback tests require an explicitly approved
private interface and configured peer allowlist. Reject wildcard binds, public
addresses, multicast and unspecified peers in this v0 configuration. Resolve
configured names once, validate all resulting addresses and pin the selected private
endpoint for that connection; do not follow DNS changes into public endpoints.
Only temporary loopback listeners have been exercised. No firewall change or
private inter-host route has been activated.
A reachable SSH endpoint does not establish a direct private route; native
inter-host deployment remains gated on verifying such a route. No NAT traversal,
automatic UPnP, public relay or silent fallback to SSH is proposed.

## Peer identity and authentication

Proposed transport: TLS 1.3 with mandatory mutual certificate authentication, a
private test CA and explicit leaf-public-key pins. Use a maintained TLS library
(candidate: rustls); do not implement custom encryption or bypass standard
certificate-chain, name, validity, key-usage or handshake-signature verification.
Configure the expected server name separately from the private connection address.
Server requires client certificates; both sides additionally match a pinned SPKI
and authorized role from a local manifest. No WebPKI identity alone grants a vote.

Proposed peer ID: SHA-256 of
`ASCII("TESTNET_NATIVE_TRANSPORT_V0_PEER") || 0x00 || DER SubjectPublicKeyInfo`.
This is a profile-local transport identifier, not a new allocation in the Q1
numeric cryptographic-domain registry. Pinning is an operator enrollment step,
not permissionless resource identity. One identity is assigned to each role, even
when roles share a machine. A transport manifest maps ID, certificate pin, expected
name, private endpoint and ledger signer/role; duplicates and ambiguous mappings
are configuration errors.

Use freshly generated OS-random test credentials with restrictive file permissions.
Never reuse the SSH key or infer identity from source IP. Native private-host
activation requires replacing the public ledger fixture seeds with explicitly
provisioned test signing keys as well as TLS credentials. The present research
backend embeds public fixture signing keys; adding TLS alone cannot make those
keys secret. A key-provider/configuration seam and negative authorization tests
are implementation prerequisites, not work completed by this design.

Disable TLS early data (0-RTT) and session resumption in the initial profile. TLS
protects record integrity, while durable application counters below address
reconnect/restart replay. An allowlist/certificate update is explicit operator
action; no automatic committee reconfiguration or in-band key enrollment. Expired,
revoked or unexpected credentials fail closed. Rotation/removal closes existing
sessions before accepting the new signed-off manifest.

TLS properties and early-data replay considerations come from
[RFC 8446](https://www.rfc-editor.org/rfc/rfc8446.html), especially sections 4 and 8.
[rustls documentation](https://docs.rs/rustls/latest/rustls/) establishes a candidate
library, not a selected/pinned dependency or an audit of this proposed adapter.

## Deterministic framing proposal

TLS ALPN must be `q1-testnet-native-v0`; no version fallback. After TLS, every frame
is a four-byte unsigned big-endian payload length followed by exactly that many
bytes. Proposed limit: 65,536 payload bytes. Zero or oversized length is rejected
before allocation; partial frames have a total deadline. One connection carries
one outstanding request per direction initially; no parallel stream multiplexer.

Payload uses the existing restricted canonical CBOR subset and fixed arrays.
Require exact arity, exact integer ranges, shortest canonical encoding and no
trailing bytes. Proposed envelopes (all markers below are byte strings, not text):

```text
HELLO = [1, 0, h'544553544e45545f4e41544956455f5452414e53504f52545f5630',
         chain_id: bytes32, genesis_id: bytes32, peer_id: bytes32]
REQUEST = [1, 1, chain_id: bytes32, genesis_id: bytes32,
           sender: bytes32, recipient: bytes32, sequence: uint64,
           operation: uint, signed_application_payload: bytes]
RESPONSE = [1, 2, chain_id: bytes32, genesis_id: bytes32,
            sender: bytes32, recipient: bytes32, request_sequence: uint64,
            status: uint, signed_application_response: bytes]
```

HELLO is the first frame in each direction; the receiver compares peer ID to the
authenticated certificate and chain/genesis to its configured instance. No other
operation is accepted before both HELLOs pass. The marker decodes to the profile
name. TLS supplies session freshness; HELLO is not a bearer authentication token.
Signatures already required by the application remain mandatory and are verified
independently. Transport sender must map to the outer application signer; embedded
certificates may contain the other authorized voters and still need full verification.

Proposed operation IDs: 0=status, 1=prepare, 2=accept, 3=sync, 4=operator-run.
These are transport-local values, not changes to existing LOCALNET opcodes.
Proposed response statuses: 0=application response, 1=already seen, 2=sequence
conflict, 3=application rejection, 4=busy. Unknown values/versions fail closed.
Operator-run is allowed only for the configured controller identity; a peer cannot
gain that capability by naming it. Canonical vectors must freeze these schemas
before implementation is called interoperable. No reference-vector claim yet.

## Replay, retries and crash ordering

Sender durably stores its monotonic sequence per `(chain, genesis, recipient)` and
the exact pending request bytes **before** sending. A timeout retransmits those
same bytes with that sequence; never automatically constructs a new transfer,
nonce, amount, proposal or ballot. Counter exhaustion fails closed; do not wrap or
reset under the same keys/chain.

Receiver serializes each authenticated sender and durably records high-water
sequence plus digest of the last complete canonical REQUEST before application
dispatch. Lower sequence: reject. Equal sequence + different digest: conflict,
close/quarantine that peer. Equal sequence + same digest: return `already seen`,
without redispatch. Higher sequence: persist reservation, then dispatch at most
once in that process/storage history. Persisted reservations survive crashes.
Application vote/finality persistence remains authoritative and precedes success
responses; all failed persistence poisons the relevant worker until reviewed reopen.

This intentionally provides **no exactly-once execution promise**. A crash after
transport reservation but before dispatch may consume a request without applying
it. After reconnect, authenticated status/catch-up reconciles the application state.
If outcome is ambiguous, retain the original application bytes and require an
explicit application recovery decision before sending them in a new envelope.
Automatic reconnection is allowed; automatic uncertain-value replacement is not.
Unfinalized application reservations cannot be cleared by a new transport sequence.
Deleting transport counters or restoring an old disk snapshot is unsupported.

Responses are accepted only on the authenticated session from the expected peer,
for the single pending request sequence and configured chain/genesis. Late responses
for abandoned sessions are discarded; ledger/certificate verification still runs.
Response errors never grant membership or finality. Request digest uses the existing
SHA-256 primitive over exact REQUEST bytes with a profile-local ASCII prefix and
NUL delimiter (`TESTNET_NATIVE_TRANSPORT_V0_REQUEST\0`), not a new numeric domain.

## Bounds and connection state machine

Historical proposed operational limits, explicitly outside consensus. Actual current limits are in the extension above:

| Resource | Limit / action |
|---|---|
| TCP connect | 3 seconds; close on expiry |
| TLS handshake and HELLO | 5 seconds each; no unauthenticated application dispatch |
| Total frame read/write | 5 seconds; use an absolute deadline, not a reset per byte |
| Application response | 15 seconds; report unknown outcome and preserve pending bytes |
| Idle session | 30 seconds; status probes only when idle |
| Connections | 8 authenticated + 2 pending handshakes per node; bounded accept queue |
| Work queue | 16 frames, total 1 MiB; return busy/drop when full |
| Per-peer work | one outstanding request per direction; no unbounded task spawning |
| Retry | 250ms, 500ms, 1s, 2s, 4s, then 8s cap; random jitter 0–25% |
| Retry budget | 5 attempts per request; reconnect at capped rate while operator enables peer |
| Invalid peer/chain/signature | no immediate retry loop; fail closed and rate-limit diagnostics |

State machine: DISCONNECTED → BACKOFF → CONNECTING → TLS_AUTHENTICATED → HELLO
→ READY → DRAIN/CLOSED. Only READY dispatches. Timeout does not elect a producer
or change committee. A successful reconnect triggers read-only status comparison
and bounded verified history sync before new proposal work. A lagging node cannot
vote on a mismatched parent. Sync retains the current bounded 16-block research
archive; over-limit recovery stops explicitly, not via silent pruning.

For simultaneous dialing, the smaller lexicographic peer ID initiates the single
normal connection. Duplicate connections are closed deterministically; permissions
are symmetrical despite the dial direction. Private connectivity must allow the
chosen direction before deployment. Backoff jitter and wall-clock deadlines are
local scheduling only; never hashed into StateRoot or used as voting weight.

## Before any native deployment

1. Human review of this profile and key provisioning boundary.
2. Frozen frame vectors; incremental decoder/fuzz tests for lengths, CBOR, partial
   reads, unknown operations, duplicate fields and allocation/queue bounds.
3. TLS unknown/wrong/expired certificate, wrong chain/genesis/ALPN, role spoofing,
   bad signature, reconnect replay and disk-reservation crash tests.
4. Loopback native adapter test with fresh keys: actual signed transfers, producer
   recovery, invalid sync and unchanged economics; known-good SSH regression retained.
5. Confirm a private route and isolated listen endpoints without changing global
   production networking. Operator approves the concrete deployment separately.
6. Two-host native comparison and then independent three-host matrix when hardware
   exists. No public listening, public admission or production/security claim.

The human approved limited implementation of TLS/manifest identity, durable replay
and fresh-key provisioning in the next-phase instruction. Public exposure and
monetary operation remain unapproved. Reviewed targets above that are not in the
implementation record remain future work, not deployed behavior.
