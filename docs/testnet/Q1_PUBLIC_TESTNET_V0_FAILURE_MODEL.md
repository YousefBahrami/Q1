# Q1 PUBLIC TESTNET v0 — failure model and hardening design

Date: 2026-10-05. **Crash-only target approved; bounded experiment implemented;
public deployment not approved.** See [measured results](Q1_TESTNET_FAILOVER_V0_RESULTS.md).
Public Testnet v0 means permissioned crash tolerance. Public Testnet v1 is the
later candidate malicious-validator/Byzantine-safety milestone.
This refines the [testnet plan](../research/Q1_PUBLIC_TESTNET_V0_PLAN.md).
Released LOCALNET stays fixed producer, three voters, quorum two and round zero.
Public-testnet quorum, round transitions, schemas and networking require an
explicit scoped decision. No Mainnet rule is selected here.

## Minimum fault target and boundaries

Approved target: a permissioned **crash-fault** experiment with three fixed,
honest voters, durable state and quorum two. It must continue with one voter
offline and an ordinary full node offline, and recover both. Authenticated
outsiders may send hostile input or withhold synchronization data. This is
not a promise to tolerate a Byzantine authorized voter.

Two quorums of size two among three voters intersect in only one voter. A
Byzantine intersection voter could sign both conflicting values. Thus this
profile cannot claim one-Byzantine-voter safety. If that is required, review a
different design, for example four voters/quorum three with a complete BFT
protocol; changing the threshold alone does not supply that protocol. Human
choice for v0 is now crash-only; do not reopen it as a blocker.

Safety means no conflicting finalized value for a height, deterministic replay,
no invalid state/fee change, and no acknowledgement before durable required
state. Liveness is conditional on an eventually stable network, a reachable
quorum, at least one eligible producer, available valid block data and adequate
disk/resources. Safety must not depend on clock accuracy. All-voter compromise,
permanent partition, simultaneous loss of a majority's disks and unlimited
volumetric denial of service are outside this minimum fault claim.

## Producer failover: REQUIRED BEFORE PUBLIC TESTNET

A fixed producer is a single point of failure even with two healthy voters.
The public milestone must therefore include a tested replacement path.
Documenting manual restarts is insufficient for the stated unattended progress
target. LOCALNET's already accepted fixed-producer behavior remains valid.

Smallest proposed rotation experiment: two registered **non-voting** producer
keys, three fixed voter keys, an ordinary full node and an isolated resource
experiment. Order producers canonically in the approved genesis profile. For
height h starting at one and coordination round r starting at zero, select
producer `P[(h-1+r) mod 2]`. This is a proposed laboratory schedule, not an
approved Q1 selection formula. No public mining weight, clock, hostname or
identity count enters it. Disconnect never reconfigures the voter committee.

Local timers may trigger a request to advance coordination, but one producer's
timer cannot erase votes, finalize a block or compel others to forget durable
state. The protocol must define authenticated round/prepare evidence, increasing
ballots, stale-message handling, locking/adoption and restart behavior before
this schedule can be implemented safely.

### Why rotation alone does not work

Current [LOCALNET schemas](../protocol/Q1_LOCALNET_V0.md) bind a vote to the
height, round, ProposalId and BlockId; the signed header includes producer and
round. A new producer re-signing the same transactions does **not** preserve
the old BlockId. Durable vote reservations cannot be cleared because a timer
expired. Two signatures may already have been collected before their producer
crashes, even if no peer has received the assembled certificate.

Conversely, an informal rule to never change a height's vote can deadlock if
different voters reserve different unchosen proposals. A complete adoption
protocol is needed; neither unconditional unlock nor permanent per-height
reservation is an adequate failover design.

### Candidate crash-only protocol to review, not a new approved consensus

[Paxos Made Simple](https://lamport.azurewebsites.net/pubs/paxos-simple.pdf)
provides the reference for majority prepare/accept and preservation of a chosen
value across proposers. A candidate mapping would durably promise an increasing
ballot, return the highest accepted ballot/value during prepare, adopt that
value from a majority's responses, and persist acceptance before replying.
When no returned response contains an accepted value, the proposer may propose
a new valid value. A majority acceptance chooses the value, regardless of
whether the proposer survives to broadcast a certificate. This reference
does not establish Byzantine safety or certify the Q1 mapping below.

Q1-specific mapping questions that must be resolved in an ADR:

| Element | Candidate mapping / required review |
|---|---|
| Immutable value | Complete original signed header/body/proposal bytes; no re-signing another producer's value |
| New coordination envelope | Carry a recovered original value under the current coordinator's authenticated envelope; do not pretend the original signed RoundNumber changed |
| Ballot versus header round | Define separately and bind both in signatures/certificates; existing LOCALNET vote schema does not express this |
| Validity | Validate original producer authority, full body, parent and state transition; specify when a previously authorized producer's value remains adoptable |
| Durable voter record | Highest promise and accepted ballot/value per height plus finalized tip; atomic persistence, no counter wrap, no rollback on restart |
| Prepare evidence | Two distinct authorized responses bound to chain/height/ballot, with full accepted value available; bounded authenticated messages |
| Finalization certificate | Define signatures over the chosen immutable value and coordination context; independent verification and replay rules |
| Round progression | Unique monotonically ordered ballots assigned to eligible coordinators, durable recovery and late-message rejection; no unilateral committee change |
| Conflicting valid certificates | Preserve evidence and enter the existing fail-safe conflict handling; never silently pick a preferred producer |

This exposes a schema/semantics dependency, not an implementation workaround.
It relates to DEC-Q1-005/018/019/020 and the draft deterministic fallback/round
documents. It uses approved signing, canonical data, fixed membership and
finalized-history principles without claiming missing round rules are approved.
Public transport and a safe non-monetary delay/profile designation also require
decisions. Do not relax LOCALNET/DEVNET-only NONE guards to create a public
network, and never reuse public fixture private keys.

Before coding, require trace review/model checking of: accepted-but-unannounced
value; one acceptance then crash; split unchosen proposals; delayed old leader;
leader restart with stale state; ballot conflict; partition/heal; and restart
between persistence and response. Model checks and failover tests have **not**
run in this documentation phase. Performance tuning cannot replace these checks.

## Untrusted sync and ordinary nodes

An ordinary node has no vote or producer authority. From pinned genesis and
profile it verifies canonical block bytes, producer authorization, certificate
membership/signatures, parent continuity, every transaction and state transition,
and the resulting StateRoot. Remote height, state snapshot and claimed root are
untrusted hints. Verify incrementally before committing; do not install a peer's
snapshot on trust. Peer selection/bootstraps provide discovery, not authority.

Persist only validated progress atomically. Reject unknown chain, gaps, duplicate
voters, forged signatures, bad roots, truncated archives and oversize frames.
Try another peer on unavailable/invalid data under bounded retry/backoff. A
conflicting apparently valid finalization follows explicit conflict handling.
Read-only ordinary nodes can serve verified history; never manufacture votes.
Resource telemetry failure or CPU exhaustion must not disable voters.

## Proposed admission and input budgets

These are **new experiment budgets to approve**, not existing Q1 wire limits.
The current local whole-archive 16 MiB limit is not a public sync design. Before
implementation reconcile any proposed block cap with profile validity rules:
transport must not silently discard a block that its consensus profile permits.

| Surface | Proposed bounded test budget | Behavior to test |
|---|---|---|
| Handshake / unauthenticated connection | 4 KiB handshake, 4 pending handshakes, 5-second completion deadline | Reject unknown profile/key and idle connections before large allocation |
| Connections / workers | 32 total connections, 8 active processing workers, 32 queued work items | Backpressure and fair per-peer limits; no thread per unbounded request |
| Transaction / proposal | 64 KiB transaction, 1 MiB complete proposal frame | Check length before allocation; profile explicitly defines corresponding validity limits |
| Certificate / round control | 64 KiB control object; accepted full value sent as separately bounded proposal frame | Fixed committee collection bounds; no recursive embedded certificates |
| Sync | 1 MiB chunk, at most 4 MiB in flight, one validated block at a time | Complete authenticated framing, bounded decompression or no compression, retries cannot grow queues |
| Canonical parsing | Depth 16; field counts fixed by schema; all byte/array limits checked | Unknown/duplicate fields, integer overflow and noncanonical bytes rejected |
| Memory / traffic | Aggregate input buffers 32 MiB, output buffers 32 MiB; test 1 MiB/s per peer, 8 MiB/s aggregate | Shared budgets override per-connection maxima; control traffic has reserved capacity |
| Disk / logs | 256 MiB experiment archive budget, 16 MiB rotated diagnostic logs | Visible capacity halt before unsafe persistence; no deletion of live safety records |

Deadlines and rate limits are local resource policy, not evidence that a voter
lost authority. Values require honest-path calibration, including low-bandwidth
hosts, before release. A 256 MiB archive cannot hold arbitrarily many 1 MiB
blocks: the run must declare its workload/horizon and stop safely at capacity.
Crash-only availability is not guaranteed against resource exhaustion beyond
these measured budgets. Internet-scale denial-of-service resistance is deferred.

## Required acceptance matrix

All following rows are future tests, not completed Public Testnet results.
Use a fresh non-monetary genesis, signed transfers, exact accounting invariants,
logged signatures and independently compared final roots.

| Injection | Expected observation |
|---|---|
| Baseline all roles | Every ordinary/voting node replays to identical finalized root; experiment grants no authority |
| Stop one voter and ordinary node | Remaining two voters finalize; membership unchanged; producer excludes itself from votes |
| Restart both | Durable votes survive; untrusted-peer replay catches up; all roots agree |
| Stop active producer | Alternate producer adopts any already chosen work safely, then progresses within the documented stable-network recovery window |
| Stop producer plus one voter | Remaining producer and two voters recover/progress after network stabilization |
| Stop two voters | No new finalization, state change or fee for rejected/unfinalized transfer |
| Partition voters 2/1, then heal | Only reachable majority may choose; minority cannot finalize; heal to one history |
| Both producers offline | Explicit halt, then safe recovery; no unauthorized role promotion |
| Crash at persistence boundaries | Never acknowledge unpersisted promises/acceptances; detect corrupt storage and fail closed |
| Untrusted sync / malformed flood within budgets | Reject forged/cross-chain/oversized/noncanonical input; bounded resource use and honest-path service |
| Delayed/duplicated messages and clock skew | No reservation expiry or duplicate fee; monotonic progress once stable |
| Stop/saturate experimental miner | No authority or shared resource exhaustion that prevents consensus progress |
| Conflicting valid certificates | Preserve evidence and stop unsafe advancement; no claim Byzantine quorum safety |

Record operator/key inventory, software/profile hashes, actual distinct hosts
and networks, faults, recovery time, all rejected messages and final roots.
Do not infer hardware independence from process IDs. The accepted LOCALNET
four-process test remains evidence only for its existing scope.

## Zero-purchase deployment profiles

| Profile | Roles and placement | Failure independence / realism | Cost and nonclaims |
|---|---|---|---|
| A — one machine | Two producer processes, three voters, ordinary node, isolated resource lab; bootstrap service co-located | Process crashes, replay and injected delay only; one disk/power/kernel/router failure affects all | Existing hardware/time/electricity; no independent-host, Internet or Sybil claim |
| B — available hosts | Distribute the same roles over inventoried borrowed/owned machines; label every co-location and network | Real cross-host latency only where available; shared ISP/router/power remain correlated | No purchase; if fewer than five independent role hosts exist, test partial topology and explicitly leave simultaneous host-fault target unproven |
| C — minimum future public | Two producer hosts plus three voter hosts with distinct failure domains; ordinary/bootstrap services may share protected hosts; lab isolated by resource budgets. Two reachable bootstrap endpoints; target at least two network paths | Five independent role hosts are needed to test one producer-host plus one voter-host failure without accidental co-loss; separate ordinary/lab hosts preferred when available | Existing volunteered hosts first. Connectivity, maintenance and review may require funding; no cloud purchase or public opening now |

Five machines in one room/cloud account are not five independent power/provider
domains. Profile C requires evidence of placement, operators, firewall/key
management, monitoring, recovery procedure and reachable networking. The older
four-host, one-producer proposal cannot prove producer-host failover; this new
five-host proposal supersedes it for that expanded failure target only.

## Required versus deferred

**Required before public testnet:** scoped fault/quorum decision; reviewed
producer recovery and durable vote/adoption rules; independent ordinary-node
verification; authenticated bounded transport; hostile sync/input tests; fresh
keys and explicit test profile; declared limits/host inventory; operational
stop/recovery procedures; passing controlled multi-host acceptance and separate
human public-deployment authorization.

**Safe to defer from this crash-only experiment:** open validator admission,
automatic committee reconfiguration, monetary rewards, resource-based producer
selection, Mainnet delay, partial state proofs, unbounded historical archives,
production custody and Internet-scale availability claims. Byzantine voter
tolerance is deferred only if the human explicitly accepts the narrower fault
target. No testnet coin has a monetary claim.
