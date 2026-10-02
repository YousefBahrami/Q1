# Q1 PUBLIC TESTNET v0 — proposed engineering milestone

Date: 2026-10-02. **PLAN ONLY — NOT APPROVED FOR IMPLEMENTATION OR DEPLOYMENT.**
The public source release is complete. No public testnet currently exists.
Do not expose, tunnel or forward LOCALNET RPC to the Internet. Its local network
class, NONE witness and public fixture keys must remain confined to LOCALNET.

This plan follows the [differentiation audit](Q1_DIFFERENTIATION_AUDIT.md).
Its purpose is a bounded experiment in authenticated multi-host replication,
leader failure, adversarial synchronization and accountable operations. It is
not a token launch, a novelty claim or an implicit final Mainnet design.

## Smallest meaningful candidate

**Recommendation, subject to a new human decision:** four equal-weight validators
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

## Next decision for the human

Approve or revise the audit's research thesis first. Then decide whether the
smallest next target is the recommended permissioned four-validator BFT testnet
(requiring a proposer-voting exception), a larger separated-role experiment,
or further local research before any testnet. Explicitly settle the testnet-only
economics/delay scope at the same time. Approval must precede implementation;
Mainnet economics, permissionlessness and novelty remain separate questions.
