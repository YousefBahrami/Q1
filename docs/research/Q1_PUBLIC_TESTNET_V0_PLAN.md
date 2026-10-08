# Q1 PUBLIC TESTNET v0 — proposed engineering milestone

Date: 2026-10-02; latest role/deployment refinement: 2026-10-04.
**Crash-only v0 target and bounded failover experiment now approved. Public
deployment and complete ledger integration remain unfinished.**
See the [six-process experiment results](../testnet/Q1_TESTNET_FAILOVER_V0_RESULTS.md).
The older approval gates below are historical where superseded by that scope.
The public source release is complete. No public testnet currently exists.
Do not expose, tunnel or forward LOCALNET RPC to the Internet. Its local network
class, NONE witness and public fixture keys must remain confined to LOCALNET.

This plan follows the [differentiation audit](Q1_DIFFERENTIATION_AUDIT.md).
Its purpose is a bounded experiment in authenticated multi-host replication,
leader failure, adversarial synchronization and accountable operations. It is
not a token launch, a novelty claim or an implicit final Mainnet design.

## Current refinement — 2026-10-05 design, not implementation

The [failure model and hardening design](../testnet/Q1_PUBLIC_TESTNET_V0_FAILURE_MODEL.md)
classifies producer failover as **required before public testnet** and proposes
two non-voting producers, durable majority adoption review, hostile sync/input
tests and explicit budgets. Its crash-only three-voter/quorum-two candidate
needs human approval; it does not claim one-Byzantine-voter safety.
Five independent role hosts are proposed to test a producer-host failure plus
a voter-host failure. This supersedes the one-producer/four-host deployment
proposal below for that expanded target; available partial topologies may
still be rehearsed with their limitations disclosed. LOCALNET remains unchanged.
Resource v1 is design-only telemetry without authority; no public exposure yet.

## Prior role design — human decision of 2026-10-04

The next design separates bootstrap, producer, voters, ordinary full node and
experimental miner/resource contributor. The requested deployment inventory is
one producer, three voters, one ordinary node and one experimental participant.
This supersedes the earlier four-voting-proposer recommendation as the next
planning baseline, **not** as approval of a public consensus implementation or
fault model. Preserve the non-voting producer separation in this design; public
quorum, locks/round changes and producer replacement still need explicit rules.

The [isolated mining experiment](Q1_MINING_DELAY_EXPERIMENT_V0.md) now exists.
It is observable laboratory work and grants no final consensus authority.
There is no live testnet, public discovery implementation or resource integration
in the node. Earlier acceptance options below remain conditional research
alternatives; do not execute their four-voter quorum tests unchanged on three
voters or silently weaken the current LOCALNET network-class guard.

| Role | Function | Authority / boundary | Implementation status |
|---|---|---|---|
| Bootstrap | Publish/serve approved peer addresses and network identity; permit multiple seeds and independent genesis pinning | Discovery only; no voting/admission/root truth. Bootstrap compromise must not validate false history. | Design only; may colocate with ordinary node once implemented |
| Producer | Propose signed blocks under approved eligibility/round rules | No vote in this separated-role design; cannot finalize alone | Fixed LOCALNET producer works; public transport/failover not implemented |
| Three voters | Independently validate execution and authorized signed consensus phases | Only approved committee rules may finalize; disconnect never changes membership | Local 2-of-3 works; public fault/quorum/lock model unresolved |
| Ordinary full node | Verify from pinned genesis through untrusted peers; execute/recheck certificates and roots | No vote; an RPC status display is not a full node | Dedicated role not implemented; future acceptance must prove independent sync |
| Experimental miner | Run bounded resource challenge/evidence experiment and export observations | No vote, block priority, eligibility weight, balance, payout or treasury authority; no consensus keys | Standalone lab implemented; testnet collection/transport not implemented |

**CONSENSUS SECURITY:** authorized role keys, canonical signatures/objects,
deterministic state, a reviewed fault/quorum/locking protocol, durable signing
state and bounded authenticated transport. Only the scoped local subset exists
today. With three voters and quorum two, intersecting certificates may share
only one signer; if that signer equivocates, this alone does not establish
one-Byzantine safety. The desired fault tolerance must be settled explicitly.

**RESOURCE/MINING EXPERIMENT:** self-contained lab evidence plus untrusted
timing/IO telemetry. A valid Merkle transcript does not prove physical HDD use,
scarce capacity, fair selection or monetary cost. Collect its results through
separate files or a future separately bounded observer service; do not feed
them into block validity or state. Resource-process failure/flood must not
consume voter-reserved capacity or block finalization.

## Deployment profiles using existing hardware

| Profile | Minimum placement / process count | What it can establish | Limits / cost |
|---|---|---|---|
| ZERO-COST LOCAL MULTI-PROCESS | One existing physical machine, no VM required (or one existing VM), six logical workloads: producer + three voters + ordinary node + lab participant; bootstrap colocates with ordinary node | Reproducible process boundaries, crash/replay and independent-role behavior once missing roles exist | One host/kernel/power/network failure domain; no geographic/operator independence. Only four LOCALNET nodes and standalone lab are runnable today; ordinary-node/bootstrap services remain future work. No new purchase, but CPU/storage/electricity/time are real. |
| LOW-COST MULTI-MACHINE | Minimum four existing physical machines: M1 producer; M2 voter A + ordinary node/bootstrap; M3 voter B + lab participant; M4 voter C. At least two actual network connections for cross-network testing | Producer and each voter can fail on distinct hosts; observers and miner remain separate processes/users | Shared observer/voter and miner/voter resources must be disclosed, bounded and failure-tested. Independent machines do not prove independent owners. Existing machines/connectivity have not yet been inventoried. |
| Preferred extra separation, if already available | Six existing hosts/VMs on independent hosts: producer, each voter, ordinary/bootstrap, miner individually | Removes experiment/observer load from voter machines | Six VMs on one host still have one hardware failure domain. No cloud acquisition is authorized. |

Four VMs on four physical hosts can implement the minimum machine placement;
four VMs on one physical machine cannot establish four-host tolerance. A second
bootstrap endpoint can be a service on M1, without granting it any new authority.
Do not expose either service until transport/security gates and human deployment
approval are complete. A LAN rehearsal with one Internet connection is not
evidence of cross-network behavior.

Isolation requirements: separate users or sandboxes where available, dedicated
directories and keys for consensus roles, separate read-only experiment output,
no miner access to voter keys, and explicit CPU/memory/IO limits. This lab's
observed peak RSS is about 47 MiB and its largest dataset 16 MiB; these are
observations, not sizing guarantees for long-running nodes. Freeze node/load
budgets after an actual host inventory. Sequential local testing costs no new
equipment; public security review and unavailable connectivity remain unfunded.

Next B test: after implementing/approving the missing profile, kill or overload
the experimental process and confirm unchanged consensus roots/membership;
then test ordinary-node hostile sync and signed cross-host recovery. The current
phase implements the lab only. Keep the earlier PTN0-D/H/R/P gates, revising the
topology-specific quorum/partition expectations after the public fault decision.

## Earlier four-voter candidate — retained alternative, not selected baseline

**Earlier recommendation, retained only as an alternative:** four equal-weight validators
operated on independently failing hosts, with rotating proposers and a reviewed
BFT protocol using three-of-four finalization. An additional non-voting node
should join from published genesis through untrusted peers. Validators are an
explicit, permissioned test cohort; public access means reproducible source,
documented connectivity and bounded observer/client access, not open validator
admission. Recruit independent operators before claiming operator independence.

Prefer evaluating an established consensus state machine/engine before writing
another round-change protocol. Integration with Q1's execution and canonical
commitments needs an ADR; selecting CometBFT or any other dependency is not done
by this plan. Its [consensus specification](https://raw.githubusercontent.com/cometbft/cometbft/main/spec/consensus/consensus.md)
is an example of explicit voting phases, locks and proposer changes, not a proof
that Q1 inherits its guarantees.

This four-validator recommendation allows the current proposer to vote. That
**differs from the approved LOCALNET non-voting producer rule** and needs separate
approval for a new testnet profile. An alternative preserving role separation
needs at least four fixed voters plus two eligible non-voting producers to test
one voter fault and producer replacement: six role processes, with exact
operator/failure relationships specified. Neither alternative is implemented.

For equal-weight voters with n=4 and q=3, two quorums intersect in at least two
voters. At most one Byzantine voter leaves an honest intersection. This necessary
condition is not a sufficient consensus proof: durable locking, phase semantics,
view-change evidence and network assumptions still have to be specified and reviewed.
The intended candidate fault budget is at most one Byzantine validator, with
liveness after network stabilization and sufficient honest availability. No
asynchronous termination guarantee or Mainnet fault-model decision is implied.

## Classification and acceptance gates

**REQUIRED BEFORE TESTNET** means before any Internet-facing deployment of the
candidate. **CAN BE TESTNET-EXPERIMENTAL** means an approved, bounded test policy
may evolve through explicit versions/resets; its current behavior still must be
specified and tested before exposure. **DEFER TO MAINNET** means not required for
this non-value experiment, not permission to omit its testnet counterpart.

| Item | Classification | Current gap and required decision/implementation | Reviewable acceptance evidence |
|---|---|---|---|
| Producer rotation/failover | REQUIRED BEFORE TESTNET | Resolve DEC-Q1-005/019 and proposer voting role; exact rounds, timeout evidence, lock transfer and unfinished-work recovery. No retry-only local workaround. | Kill or equivocate the proposer; remaining quorum resumes after stabilization without conflicting finality; crash during each signing/persistence phase. |
| Public peer discovery | REQUIRED BEFORE TESTNET | DEC-Q1-014: initially a published, authenticated multi-seed list with explicit update authority; no DHT required. Discovery must not grant voting rights. | New node finds multiple peers; one bad or unavailable seed cannot supply an accepted false history or monopolize synchronization. |
| Authenticated networking | REQUIRED BEFORE TESTNET | Bind transport identity to its key and chain/version context; distinguish peers, authorized voters and clients. Define replay and key-rotation behavior. | Reject unknown voter roles, key substitution, wrong-genesis handshakes and cross-session/cross-chain replay. |
| Transport security assumptions | REQUIRED BEFORE TESTNET | Choose a maintained encrypted authenticated transport, e.g. TLS 1.3 with a documented trust model; consensus object signatures remain independently checked. No custom cryptographic handshake. | Peer impersonation, downgrade, expired/revoked identity and interrupted-session tests; explicit metadata/confidentiality limits. |
| DoS/resource limits | REQUIRED BEFORE TESTNET | Bound connections, pre-auth work, signature verification, threads, queues, per-peer/global bandwidth and storage. Reserve capacity for consensus under client load. | Reproducible hostile load demonstrates bounded RSS/CPU/queues/disk within an approved host budget and continued honest progress within the approved envelope. |
| Bounded messages | REQUIRED BEFORE TESTNET | Exact limits for each handshake, tx, proposal, certificate, proof and sync chunk; check before allocation/decompression. Existing 16 MiB local cap is not a public budget. | Boundary and over-limit vectors; nested/truncated frames, slow senders and decompression expansion fail without unbounded allocation or worker starvation. |
| Mempool/admission policy | REQUIRED BEFORE TESTNET | Define size/byte limits, TTL, duplicate and nonce handling, fee checks, deterministic block validity and local eviction rules. Client admission must not silently modify consensus. | Flood, stale/future nonce, replay and concurrent-submit tests; no unbounded queue; valid transactions progress under stated load assumptions. |
| Validator admission | REQUIRED BEFORE TESTNET | DEC-Q1-024: explicitly authorize a fixed test cohort, published keys and operator roles. No disconnect-based membership changes; no dynamic membership in the minimum candidate. | Unknown signers rejected; reconnect retains membership; participant manifest agrees across all nodes. |
| Permissionless admission/Sybil resistance | DEFER TO MAINNET | Remains a separate research decision; required earlier if a later testnet claims permissionlessness. | No open-admission or decentralization claim for this permissioned milestone. |
| Final quorum/fault model for this testnet | REQUIRED BEFORE TESTNET | Resolve exact weight, threshold, Byzantine/crash/network assumptions and cross-round rules (DEC-Q1-019/020). Mainnet finality parameters remain separate. | Reviewed state machine and safety argument plus model/adversarial tests, including withholding, conflicting votes and delayed old-round messages. |
| Chain identity/genesis | REQUIRED BEFORE TESTNET | New explicit network profile, fresh genesis, keys, allocations and protocol versions. Bind all roles/objects to it; no migration of LOCALNET identities or balances. | Independent genesis/root reconstruction; mixed-network signatures/configurations rejected; published manifest and checksum. |
| Sync from an untrusted peer | REQUIRED BEFORE TESTNET | Verify history from the pinned genesis, including certificate, parent, execution and root at every height. Multi-peer retry, resource caps and fork reporting. No trusting an RPC-reported root. | Corrupt, truncated, conflicting and withheld history rejected; new observer reaches the same independently computed root via another peer. |
| Storage and crash recovery | REQUIRED BEFORE TESTNET | Durable safety state across signing phases; bounded incremental history suitable for the declared experiment rather than rewriting the entire 16 MiB archive. Define disk-full and backup/rollback behavior. | Full-duration capacity run, fault injection around commit/signing, disk-full failure and restore drills; no equivocation or silent reset. |
| Equivocation evidence | REQUIRED BEFORE TESTNET | Canonical signed evidence with bounded retention/verification; distinguish accusation from verified proof. Define operator response; monetary slashing is not necessary here. | Conflicting signatures demonstrably attributable; forged/context-mismatched evidence rejected; restart preserves evidence and signing safety. |
| Fork/finality behavior | REQUIRED BEFORE TESTNET | Resolve DEC-Q1-020/008: exact treatment of unfinalized forks, conflicting valid certificates, safe halt and recovery authority. Never pick conflicting finalized histories silently. | Partition/heal scenarios preserve one finalized history within the fault model; outside-model conflicting certificates trigger visible fail-closed behavior. |
| Key custody | REQUIRED BEFORE TESTNET | Fresh private keys, explicit role separation, restricted storage, backup/restore and incident procedures. No published fixture seeds, duplicate active signers or secrets in logs. | Permission, copied-signer, rollback and rotation/restart drills; documented compromise response. |
| Observability and private security intake | REQUIRED BEFORE TESTNET | Metrics/logs for height, round, peer health, rejection, resource pressure and conflicting votes; human-approved private vulnerability channel and incident owner. | Operator can diagnose stall, recover a node and report privately; logs omit keys and unrelated personal information. |
| Upgrade/version rules | REQUIRED BEFORE TESTNET | Pin wire/schema/consensus versions; reject unsupported versions; publish operator coordination and reset rules, with no hidden update authority. | Mixed-version rejection and upgrade/reset rehearsal; resets use a distinct genesis identity, never silently rewrite finalized history. |
| Testnet fees/supply/allocation | CAN BE TESTNET-EXPERIMENTAL | Approve a separate non-value profile before launch: suggested fixed genesis units, fee=1 to an explicit pool, zero issuance/burn/distribution. LOCALNET approval does not authorize this extension. | Canonical vectors and invariant tests; published allocations; no pricing, sale, reward promises or trading integration. |
| Final Mainnet economics | DEFER TO MAINNET | DEC-Q1-021/022/023 and broader incentives remain open. Do not use local constants as permanent monetary policy. | Independent economic/adversarial review before any public-value design; see pre-offer boundary. |
| Testnet delay policy | REQUIRED BEFORE TESTNET | Explicit human decision on whether this first network tests networking/BFT without a delay security role or waits for a reviewed delay construction. Current NONE must continue rejecting non-Localnet. | New approved profile and cross-network rejection tests; no implicit fallback, relabeling or removal of the existing guard. |
| Final delay/VDF construction | DEFER TO MAINNET, conditional | May be deferred only if a human-approved testnet decision explicitly excludes delay from security. DEC-Q1-025 currently requires review before any public-network delay role. If delay is used for security, construction and cryptographic review become REQUIRED BEFORE TESTNET. | Documented assumptions, attack/cost analysis and independent cryptographic review before making any delay-security claim. |
| Security review | REQUIRED BEFORE TESTNET | Independent targeted review of consensus changes, signatures/parsers, transport, sync, resource bounds and custody; record findings and fixes. Existing CI is not an audit. | No unresolved critical/high findings for exposed scope, reproducible fixes and reviewer sign-off; scope/limitations public. |
| Tuning and optional monitoring | CAN BE TESTNET-EXPERIMENTAL | Metrics, local timeout tuning within approved rules and optional non-consensus observers; removal must not change validity. | Versioned configurations, A/B measurements and identical accepted state with optional observers disabled. |
| Production custody, Mainnet governance and public-value launch review | DEFER TO MAINNET | Broader operational/economic/governance decisions and legal/compliance review remain outside this milestone. Testnet key controls and limited operator authority are still mandatory now. | Separate approvals; no implication that this plan authorizes valuable assets or regulatory compliance. |

Every REQUIRED gate is presently open for the proposed public profile. Existing
local components provide reusable evidence but do not close these public gates.
Owners are roles to assign: project maintainer coordinates; implementer produces
artifacts; an independent reviewer assesses them; the human approves decisions
and launch. This document does not invent an existing team or booked auditor.

## Staged work and exit criteria

1. **Decision package, before implementation.** Human approves the research
   thesis, permitted cohort, voting/proposer model, consensus integration path,
   test-only economics and delay scope. Produce ADRs for rounds/finality,
   networking, identity, resource policy and upgrades; preserve all LOCALNET rules.
2. **Isolated adversarial development.** Implement the approved new profile on
   loopback/private isolated hosts first. Model-check or systematically explore
   leader changes and locks. Test each REQUIRED gate and obtain targeted review.
   No Internet exposure is authorized by reaching this stage.
3. **Preflight artifact.** Publish exact genesis, binaries/source hashes, operator
   manifest, configuration, trust assumptions, limits, incident procedures,
   security review scope and signed-off acceptance report. Select the numerical
   resource/load/time budgets before the run; do not tune pass criteria afterwards.
4. **Separate launch approval.** The human reviews actual evidence and explicitly
   authorizes a time-bounded non-value public experiment. A passing LOCALNET run
   or approval to implement is not launch authorization.
5. **Candidate public experiment.** Proposed minimum observation window: 24 hours
   on independently failing hosts, repeated under documented load and faults.
   Record all parameters, runtime versions and anonymized resource measurements.

Minimum experiment sequence:

- Start all validators and an observer from the published genesis; submit signed
  test transfers; independently reconstruct matching finalized StateRoots.
- Stop the current proposer, then test a malicious/withholding proposer within
  the approved one-fault model; demonstrate reviewed leader change and resumed
  progress. Restart and catch up without unsafe signing.
- Partition two against two: no group may finalize alone. Heal the partition;
  resolve unfinished work according to the approved round/lock protocol.
- Test three available validators versus one unavailable validator; verify the
  specified progress and safety conditions, without silently changing membership.
- Feed conflicting messages, wrong genesis, malformed frames, replayed transfers
  and hostile sync responses; inject bounded overload and disk pressure.
- Bootstrap a new observer through an untrusted peer, verify all history, switch
  peers when withheld and reach the same root as independently queried validators.
- Rehearse key compromise response, a version mismatch, a planned stop and a
  genesis-changing reset. No reset implies continuity of balances or value.

Acceptance requires zero conflicting finalized states within the approved fault
model, unchanged accounting invariants, repeatable recovery and all declared
resource budgets met. Liveness latency targets depend on the chosen protocol,
network envelope and timeout policy; those numbers must be frozen in stage 1,
not invented after observing stage 5. Evidence includes raw test configuration,
failure traces, roots, latency distributions and negative results.

Full independent review for a public-value system, production wallets, bridges,
token sale, exchange integration, pricing and monetary distribution are excluded.
See [PRE_OFFER_BOUNDARY](../releases/PRE_OFFER_BOUNDARY.md).

## PTN0: next smallest reviewable milestone

Refined on 2026-10-03 and updated for the 2026-10-04 role decision in workstream B of the
balanced program (separate planning document omitted from this candidate).
The milestone is a reproducible, nonmonetary **multi-machine network experiment**,
not another four-process loopback demonstration. Its first deliverable is the
decision package below. Separated roles are now the design baseline; the earlier
four-voter topology is an unselected alternative if the eventual fault model
requires revisiting committee size.

| Gate | Concrete deliverable | Exit evidence |
|---|---|---|
| PTN0-D — decision package | Preserve separated roles; decide exact public fault/quorum/round model, state machine/engine, delay/security exclusion, test economics, bootstrap authority and resource budgets; experimental miner has no final authority | Dated human approval/ADR; no inference of public quorum approval from LOCALNET or the role count |
| PTN0-H — host and operator inventory | Four existing separate hosts across at least two network connections using the placement above, with ordinary/bootstrap and miner colocation disclosed; document shared operators, providers, power and hardware | Capability/permission inventory; no claim of operator independence from machine count. Shared roles do not add failure domains. No private addresses/credentials in public artifacts. |
| PTN0-R — isolated implementation/rehearsal | Implement approved profile and all required gates above; run the complete sequence below on controlled hosts before public exposure | Reproducible harness, scoped independent review, negative tests and machine-readable report; actual remote-host test cannot be replaced by local containers |
| PTN0-P — deployment review | Review rehearsal, runbook, keys, bootstrap manifest, private incident contact, capacity and operator readiness | Separate human deployment approval. No Internet-facing launch in this documentation phase. |

The proposed minimal discovery is a controlled published bootstrap list with
more than one authenticated seed. It is not open validator admission. Freeze
per-message/per-peer/global limits, disk budget, target load, timeout/network
envelope and maximum recovery latency **before** running acceptance. Reuse the
earlier proposed observation window only after approval; an arbitrary green run
without a stated load/duration is not completion.

Earlier ordered acceptance for the four-voter alternative, only if that
alternative is subsequently approved. For the current separated-role design,
retain sync/signature/resource/isolation checks, but replace all quorum,
partition and proposer-replacement expectations with its explicitly approved
fault/round rules before running acceptance:

1. Independently reconstruct fresh test genesis on all four machines. Authenticate
   peers through the approved bootstrap list; reject wrong-chain and unknown-role
   messages. Record actual cross-network connections and operator/failure domains.
2. Submit signed test transfers; verify proposal and every consensus phase's
   signatures/context. Compare independently executed finalized roots at the
   same height, not merely RPC-reported strings.
3. Stop the active proposer; execute approved view change, resume within the
   frozen recovery target, restart it and catch up without unsafe signing.
   Test one unavailable voter without changing membership.
4. Partition 2/2, demonstrate no unauthorized finalization, heal and recover;
   test 3/1 progress under the approved assumptions. Include replay, equivocation,
   stale-round and crash-during-signing tests, not only clean shutdowns.
5. Start a fresh observer with only the pinned genesis and bootstrap information.
   First offer corrupt/truncated/withheld history from an untrusted peer, then
   allow another peer. Verify every accepted certificate, parent, execution and
   root; catch up to a height/root common to all honest nodes.
6. Send over-limit, truncated, wrong-version and slow frames and apply the frozen
   hostile-load envelope. Prove bounded memory/queues/disk and honest progress
   under the chosen fault/load assumptions. Persist metrics and rejection reasons.
7. Complete the observation window, stop/restart all nodes, verify recovery and
   accounting, and archive a redacted reproduction package with failures included.

Network-integrated resource participation is **NOT IMPLEMENTED** today; the
standalone v0 laboratory is implemented and measured. The human has excluded
experimental-miner final authority. PTN0-D must still decide the public network's
delay profile. Record separate statuses for laboratory evidence, network
observation and consensus authority; do not claim mining-security acceptance.
An out-of-consensus HDD benchmark may be reported as telemetry only; it must
not change eligibility, finality, balances or rewards.
If a resource role is selected, its reviewed construction, canonical vectors,
replay/forgery/shortcut tests and independent verification become mandatory gates
before success. Never bypass `LocalnetNoneEvidence` network guards.

Proposed acceptance artifact (schema not yet implemented):

| Evidence group | Required fields/content |
|---|---|
| Reproduction | Source commit, approved profile/ADR identifiers, configuration/genesis hashes, toolchain, commands and UTC run interval |
| Topology | Pseudonymous node/host/operator IDs, network/failure-domain relationships, role keys and actual connection evidence; no secrets or unrelated personal data |
| Consensus/execution | Signed test transactions, verified phase/certificate traces, each node's height/root checkpoints and conserved test-supply accounting |
| Fault/recovery | Injected events, partition boundaries, signing/persistence crash points, observed progress/recovery latency and lock/equivocation outcomes |
| Sync and limits | Rejected malicious histories/messages, successful peer fallback, configured bounds and measured peaks/load/latency |
| Resource status | Exact engine/experiment and verification results, or explicit approved exclusion; no fabricated work proof |
| Outcome | Per-check pass/fail/not-run, limitations, failed attempts and final common root after a shared barrier height |

The expected root is calculated from the new profile, genesis and signed sequence.
It is **not** required to equal LOCALNET's historical `b7ec…b087` fixture root.
Agreement among independently verifying nodes and reproduction of the same new
inputs is the criterion. No PTN0 run or artifact is claimed by this plan.

## Next decision for the human

The isolated research and separated-role scope is already approved. Next settle
the public fault/quorum/round rules and testnet-only economics/delay profile,
then identify available hosts/operators and approve the concrete implementation
package. If one-Byzantine tolerance is required, the requested three-voter
topology needs explicit reconsideration rather than an unsupported safety claim.
Mainnet economics, permissionlessness and public deployment remain separate.
