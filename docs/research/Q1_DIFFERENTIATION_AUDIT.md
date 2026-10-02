# Q1 technical differentiation audit

Review date: 2026-10-02. Baseline: public release `v0.1.0-localnet.1`,
public root `2f3b95375be9bb38aa8e2196759c2bb8554d68b9`.
Status: evidence-based review, not a protocol approval or marketing statement.

**LOCALNET v0 does not yet establish a unique reason for a new cryptocurrency.**
It establishes a small, executable research baseline. A useful implementation
and a new protocol contribution are different claims. No feature reviewed below
currently qualifies as demonstrated protocol novelty.

## Method and authority

The [implemented local profile](../protocol/Q1_LOCALNET_V0.md), source and tests
are implementation evidence. [Approved decisions](../../OPEN_DECISIONS.md)
establish authority; draft requirements describe intent rather than executable
capability. The [publication record](../releases/POST_RELEASE_REVIEW.md)
links successful remote runs and the reproduced acceptance result.

Primary external sources were checked on the review date. Comparisons establish
prior art for particular properties, not equivalence of entire protocols or
comparative performance. This is a bounded technical review, not an exhaustive
literature/patent search, independent security audit or novelty certification.

Classification:

- **A — standard blockchain engineering:** established technique or requirement.
- **B — Q1-specific implementation choice:** exact Q1 bytes, composition or scope,
  without a claim that the technique is new.
- **C — genuinely differentiated protocol idea:** requires an exact construction,
  a meaningful distinction from prior art and supporting evidence. None established.
- **D — unproven research hypothesis:** a question worth testing, not an achievement.

## 1. Deterministic canonical protocol encoding — A / B

- **FEATURE:** one accepted byte representation and explicit signing/hash domains.
- **CURRENT IMPLEMENTATION STATUS:** implemented restricted deterministic CBOR,
  typed fields and domain framing; malformed/noncanonical inputs rejected.
- **EVIDENCE:** [CBOR](../../crates/q1-primitives/src/cbor.rs),
  [domains](../../crates/q1-primitives/src/domain.rs),
  [primitive tests](../../crates/q1-primitives/tests/primitives.rs).
- **WHAT PROBLEM IT ADDRESSES:** inconsistent hashes, signature ambiguity and
  reuse of a signed object in the wrong protocol context.
- **WHETHER EXISTING NETWORKS ALREADY DO THIS:** yes, deterministic serialization
  is established. [RFC 8949 §4.2](https://www.rfc-editor.org/rfc/rfc8949.html#section-4.2)
  specifies deterministic CBOR; Ethereum documents
  [RLP and SSZ](https://ethereum.org/developers/docs/data-structures-and-encoding/).
- **WHAT IS ACTUALLY NOVEL IN Q1:** no novelty established. Q1's restricted arrays,
  fixed monetary representation and registered domains are concrete B choices.
- **WHAT REMAINS UNPROVEN:** broad hostile-input coverage, independent complete
  implementations and compatibility under future schema upgrades.

## 2. Independent protocol/reference vectors — A / B

- **FEATURE:** frozen reference bytes, commitments and rejection cases.
- **CURRENT IMPLEMENTATION STATUS:** 23 protocol vectors checked by Rust/Node/Python,
  and 13 LOCALNET vectors checked by Rust and independent Python reconstruction.
- **EVIDENCE:** [runner](../../scripts/check_all.py),
  [Python protocol checks](../../scripts/protocol_reference.py),
  [Node checks](../../scripts/protocol_reference.mjs),
  [LOCALNET reconstruction](../../scripts/localnet_reference.py).
- **WHAT PROBLEM IT ADDRESSES:** catching interpretation and serialization mistakes
  that a single implementation can reproduce consistently but incorrectly.
- **WHETHER EXISTING NETWORKS ALREADY DO THIS:** yes; Ethereum publishes
  [cross-client consensus test formats](https://github.com/ethereum/consensus-specs/blob/master/tests/formats/README.md).
- **WHAT IS ACTUALLY NOVEL IN Q1:** no new testing method; Q1 contributes its exact,
  publicly reproducible corpus and small reference tools.
- **WHAT REMAINS UNPROVEN:** these are not three independent full nodes. Python
  treats Ed25519 signatures as fixture inputs; Node/OpenSSL supplies independent
  signature checking. Shared fixture mistakes and uncovered states remain possible.

## 3. Full deterministic StateRoot — A / B

- **FEATURE:** commitment to complete local account/nonce/pool/supply/authority state.
- **CURRENT IMPLEMENTATION STATUS:** full canonical snapshot hash, including
  GenesisId binding fixed policy; no partial or light-client proofs.
- **EVIDENCE:** [state](../../crates/q1-localnet/src/state.rs),
  [state tests](../../crates/q1-localnet/tests/state.rs), local profile schemas.
- **WHAT PROBLEM IT ADDRESSES:** nodes apparently agreeing while omitting a
  consensus-relevant balance, nonce or authority field from their commitments.
- **WHETHER EXISTING NETWORKS ALREADY DO THIS:** yes; Ethereum documents
  [deterministic authenticated state roots](https://ethereum.org/developers/docs/data-structures-and-encoding/patricia-merkle-trie).
- **WHAT IS ACTUALLY NOVEL IN Q1:** the exact snapshot layout is Q1-specific;
  hashing complete state is not new.
- **WHAT REMAINS UNPROVEN:** efficient large-state updates, proof serving, pruning
  and storage scalability. Full serialization is a simplicity trade-off.

## 4. Replay/equivocation protection — A / B

- **FEATURE:** signed context, next-nonce checks and durable vote reservations.
- **CURRENT IMPLEMENTATION STATUS:** transactions bind chain/height window;
  attestations bind genesis/chain/height/round/proposal/block. Reservations survive
  process restart and reject a conflicting proposal at the same vote context.
- **EVIDENCE:** [transfer](../../crates/q1-protocol-types/src/transfer.rs),
  [store](../../crates/q1-localnet/src/store.rs),
  [recovery tests](../../crates/q1-localnet/tests/recovery.rs).
- **WHAT PROBLEM IT ADDRESSES:** repeated debits, cross-context signature reuse
  and an honest signer accidentally signing conflicting work after a crash.
- **WHETHER EXISTING NETWORKS ALREADY DO THIS:** yes; [EIP-155](https://eips.ethereum.org/EIPS/eip-155)
  binds transaction signatures to chain identity. CometBFT specifies signed votes,
  locks and [fork accountability](https://raw.githubusercontent.com/cometbft/cometbft/main/spec/consensus/consensus.md).
- **WHAT IS ACTUALLY NOVEL IN Q1:** its exact context and persistence integration,
  not the security goals or reservation technique.
- **WHAT REMAINS UNPROVEN:** malicious signers can ignore local reservations;
  disk rollback, copied keys, cross-round safety and network-wide evidence handling
  are not solved by an honest node's local journal.

## 5. Consensus architecture — A / B; future design D

- **FEATURE:** distinct producer and voters, signed certificate finalization.
- **CURRENT IMPLEMENTATION STATUS:** one fixed non-voting producer, three fixed
  voters, quorum two, round zero. No rotation, view change or general consensus
  recovery protocol. One crashed voter is tolerated while the producer operates.
- **EVIDENCE:** [committee](../../crates/q1-localnet/src/quorum.rs),
  [blocks](../../crates/q1-localnet/src/block.rs),
  [block tests](../../crates/q1-localnet/tests/blocks.rs).
- **WHAT PROBLEM IT ADDRESSES:** a bounded test of signed replicated state transitions.
- **WHETHER EXISTING NETWORKS ALREADY DO THIS:** yes; CometBFT already specifies
  [proposer rotation and multi-phase certificate consensus](https://raw.githubusercontent.com/cometbft/cometbft/main/spec/consensus/consensus.md).
- **WHAT IS ACTUALLY NOVEL IN Q1:** no novel consensus algorithm demonstrated.
  Separating roles and proposing limited candidate work are design choices.
- **WHAT REMAINS UNPROVEN:** Byzantine safety, liveness under changing leaders,
  partition behavior and the safe recovery of votes reserved for unfinished work.

## 6. Delay mechanism/design — B today; D research

- **FEATURE:** proposed context-bound, replaceable sequential delay mechanism.
- **CURRENT IMPLEMENTATION STATUS:** only canonical NONE witness
  `[1,0,0,h'',h'']` is active, explicitly restricted to Localnet. It proves no
  elapsed time or sequential work. A sequential-hash engine or VDF is not shipped.
- **EVIDENCE:** [implemented evidence](../../crates/q1-protocol-types/src/delay.rs),
  [negative activation tests](../../crates/q1-protocol-types/tests/delay.rs),
  [delay design](../06_DELAY_ENGINE.md), DEC-Q1-010/025.
- **WHAT PROBLEM IT ADDRESSES:** the proposed research asks whether delay can
  constrain timing/manipulation without excessive duplicated producer work.
- **WHETHER EXISTING NETWORKS ALREADY DO THIS:** sequential delay and its use in
  blockchain consensus have prior art; Chia combines proofs of space and VDFs in
  its [consensus design](https://docs.chia.net/chia-blockchain/green-paper/green-paper-abstract/).
- **WHAT IS ACTUALLY NOVEL IN Q1:** none established. The interaction of limited
  candidate execution, fallback, challenge binding and verification cost is a
  candidate experiment, not a demonstrated new cryptographic construction.
- **WHAT REMAINS UNPROVEN:** security benefit, unpredictability/grinding resistance,
  sequentiality assumptions, generation/verification asymmetry, hardware advantage,
  setup assumptions, energy cost and effect on liveness.

## 7. Participant selection/admission — B today; D research

- **FEATURE:** canonical identities and a common eligible participant set.
- **CURRENT IMPLEMENTATION STATUS:** fixed genesis registry and fixed local roles.
  No public admission, Sybil defense, randomized selector or membership update path.
- **EVIDENCE:** [participant schema](../../crates/q1-protocol-types/src/participant.rs),
  [genesis](../../crates/q1-localnet/src/genesis.rs), DEC-Q1-004/005/018/024.
- **WHAT PROBLEM IT ADDRESSES:** agreeing who may propose/vote; future research
  would also need to address capture, bias and exclusion.
- **WHETHER EXISTING NETWORKS ALREADY DO THIS:** permissioned sets and selected
  committees predate Q1. The original [Algorand paper](https://dspace.mit.edu/entities/publication/5a5e58cd-b866-49d0-a3b9-93ab652ce7cb)
  describes cryptographic sortition; it is not evidence that Q1 implements it.
- **WHAT IS ACTUALLY NOVEL IN Q1:** record encodings and constraints are specific;
  a differentiated admission/selection construction has not been selected.
- **WHAT REMAINS UNPROVEN:** fairness, Sybil cost, randomness bias, operator diversity,
  membership-change safety and whether participation can be meaningfully open.

## 8. Fault model — A requirement; D public claim

- **FEATURE:** explicit distinction between crash tolerance and Byzantine tolerance.
- **CURRENT IMPLEMENTATION STATUS:** tested crash/restart scenarios, including one
  offline voter and no finalization with only one vote. General fault model open.
- **EVIDENCE:** [four-process acceptance](../../scripts/localnet_acceptance.py),
  [local policy](../protocol/Q1_LOCALNET_V0.md), DEC-Q1-019/020.
- **WHAT PROBLEM IT ADDRESSES:** avoiding an unsupported inference from a small
  successful local run to safety against malicious participants.
- **WHETHER EXISTING NETWORKS ALREADY DO THIS:** yes; explicit safety assumptions
  and fault bounds are part of established BFT protocol specifications such as
  [CometBFT](https://raw.githubusercontent.com/cometbft/cometbft/main/spec/consensus/consensus.md).
- **WHAT IS ACTUALLY NOVEL IN Q1:** no new fault bound demonstrated.
- **WHAT REMAINS UNPROVEN:** quorum intersection containing an honest voter under
  the intended adversary, cross-round locking, partitions and recovery authority.

Concrete Q1-specific reasoning: certificates `{A,B}` and `{B,C}` intersect only
at B. If B is Byzantine and the producer equivocates, A and C can each validate
a different otherwise-valid proposal. Their honest journals need not detect
the other branch. The LOCALNET test does not model that adversary. Changing only
the integer quorum would still leave round-change and liveness rules undefined.

## 9. Economic model — A / B today; D public incentives

- **FEATURE:** exact integer fee accounting with an explicit reward pool.
- **CURRENT IMPLEMENTATION STATUS:** successful transfer fee=1, FeeLimit only a
  ceiling; no issuance/burn/reward distribution. Invalid blocks change nothing.
- **EVIDENCE:** [ledger](../../crates/q1-localnet/src/ledger.rs),
  [accounting tests](../../crates/q1-localnet/tests/accounting.rs), DEC-Q1-021/022/023.
- **WHAT PROBLEM IT ADDRESSES:** overcharging, partial execution and unaccounted funds.
- **WHETHER EXISTING NETWORKS ALREADY DO THIS:** deterministic fee/reward validation
  is standard; [Bitcoin Core validation](https://doxygen.bitcoincore.org/validation_8cpp_source.html)
  checks subsidy and fees. Its rules differ from this local pool.
- **WHAT IS ACTUALLY NOVEL IN Q1:** this explicit test accounting profile, not a
  novel economic mechanism or demonstrated incentive equilibrium.
- **WHAT REMAINS UNPROVEN:** sustainable security funding, spam pricing, collusion,
  rewards, issuance and participant incentives. A conserved pool proves no demand.

## 10. Hardware/resource assumptions — B limits; D benefit claims

- **FEATURE:** bounded local resource use and proposed HDD-assisted research.
- **CURRENT IMPLEMENTATION STATUS:** loopback TCP, 16 MiB frames/archives, limited
  workers; full archives rewritten on mutation. HDD module is not implemented.
- **EVIDENCE:** [wire limits](../../crates/q1-node/src/wire.rs),
  [runtime](../../crates/q1-node/src/main.rs), [HDD design](../07_HDD_LAB_MODULE.md).
- **WHAT PROBLEM IT ADDRESSES:** current limits bound a local experiment; proposed
  research asks whether ordinary storage can contribute without trusted telemetry.
- **WHETHER EXISTING NETWORKS ALREADY DO THIS:** resource controls are established;
  storage-based consensus already exists, for example
  [Chia](https://docs.chia.net/chia-blockchain/green-paper/green-paper-abstract/).
  Q1 HDD telemetry is not equivalent to a proof of space.
- **WHAT IS ACTUALLY NOVEL IN Q1:** no established hardware or energy advantage.
  The requirement that the base protocol work without HDD is a useful boundary.
- **WHAT REMAINS UNPROVEN:** measured energy, wear, bandwidth and concentration;
  RAM/SSD/cache emulation resistance; aggregate DoS resistance; growth beyond the
  archive ceiling. No green/low-cost claim follows from running on one laptop.

## 11. Recovery/catch-up — A / B

- **FEATURE:** validated replay, atomic persistence and restart synchronization.
- **CURRENT IMPLEMENTATION STATUS:** completed local history replays from configured
  genesis; a returning voter fetches certified blocks from the fixed producer.
- **EVIDENCE:** [store](../../crates/q1-localnet/src/store.rs),
  [recovery tests](../../crates/q1-localnet/tests/recovery.rs), node `catch_up`.
- **WHAT PROBLEM IT ADDRESSES:** corrupt recovery, lost finalized state and stale voters.
- **WHETHER EXISTING NETWORKS ALREADY DO THIS:** yes; CometBFT documents
  [block synchronization with commit verification](https://raw.githubusercontent.com/cometbft/cometbft/main/docs/core/block-sync.md).
- **WHAT IS ACTUALLY NOVEL IN Q1:** its compact full-archive integration and
  executable failure scenario; recovery itself is established engineering.
- **WHAT REMAINS UNPROVEN:** malicious multi-peer sync, withholding/eclipse defense,
  snapshots/checkpoints, large archives, all power-loss modes and automatic
  recovery of unfinished proposals. Validating blocks does not guarantee availability.

## 12. Supply model — A / B local rule; D Mainnet economics

- **FEATURE:** explicit allocations and a conservation invariant.
- **CURRENT IMPLEMENTATION STATUS:** genesis allocation sum equals declared supply;
  balances plus pool stay constant. The fixture uses 1000 test units.
- **EVIDENCE:** [genesis](../../crates/q1-localnet/src/genesis.rs),
  [accounting tests](../../crates/q1-localnet/tests/accounting.rs), DEC-Q1-006.
- **WHAT PROBLEM IT ADDRESSES:** hidden allocations and accidental creation/destruction.
- **WHETHER EXISTING NETWORKS ALREADY DO THIS:** explicit supply validation is
  established; [Bitcoin Core](https://doxygen.bitcoincore.org/validation_8cpp_source.html)
  enforces its own issuance rules. Identical monetary policy is not claimed.
- **WHAT IS ACTUALLY NOVEL IN Q1:** the local fixture and accounting representation,
  not conservation as a principle.
- **WHAT REMAINS UNPROVEN:** Mainnet issuance, allocation fairness, governance and
  long-term incentives. The test constant is not a supply cap or scarcity promise.

## 13. AI-related design — A boundary; D optional research

- **FEATURE:** optional observer outside consensus, without signing or custody power.
- **CURRENT IMPLEMENTATION STATUS:** an architectural constraint and draft observer
  specification; no AI runtime module in the four workspace crates.
- **EVIDENCE:** [observer specification](../11_AI_OBSERVER.md),
  [charter](../00_PROJECT_CHARTER.md), [workspace](../../Cargo.toml).
- **WHAT PROBLEM IT ADDRESSES:** potentially improving operator analysis without
  allowing probabilistic outputs to change deterministic validity or balances.
- **WHETHER EXISTING NETWORKS ALREADY DO THIS:** deterministic validity independent
  of AI is ordinary protocol engineering, visible in the cited consensus and
  validation implementations. This review does not establish precedence or
  uniqueness for a specific AI monitoring product.
- **WHAT IS ACTUALLY NOVEL IN Q1:** no AI consensus innovation; none is approved.
  AI-assisted development is distinct from a shipped AI observer.
- **WHAT REMAINS UNPROVEN:** detection quality, false positives, telemetry privacy,
  adversarial robustness and benefit over simple deterministic monitoring.

## WHAT Q1 ADDS TODAY

Q1 contributes a publicly inspectable Rust reference implementation of its exact
local rules, small cross-language byte checks and a four-process failure/recovery
experiment. Explicit authority, accounting and persistence make this a useful
baseline for controlled experiments and independent reproduction. These are
engineering deliverables, not evidence of superior decentralization, security,
throughput, energy efficiency or a need for a new monetary asset.

The immediate reason to continue Q1 is to test a precise research thesis, not
to ask people to value a token. Reusing a mature consensus engine or implementing
an application on an existing network must remain valid outcomes. If an existing
system meets the eventual requirements, a separate chain needs additional justification.

## WHAT Q1 MUST PROVE BEFORE CLAIMING NOVELTY

Proposed thesis for human review: **can limiting speculative producer work and
isolating optional delay/resource experiments reduce measured resource cost,
without worsening finality, recovery or concentration relative to a conventional
certificate-based baseline under the same adversary?** No positive answer is implied.

Required evidence before promoting any D hypothesis to C:

1. Define the intended users and unmet requirement; compare an existing-chain
   application, an established BFT ledger and a Q1-specific protocol.
2. Specify the exact algorithm and adversary. Prove safety/liveness assumptions
   or identify precisely which claim rests on simulation rather than proof.
3. Freeze a reproducible benchmark: same hardware, workload, networking, fault
   budget and security target; include idle, normal, overload and partition cases.
4. Compare baseline, delay disabled/enabled and optional HDD disabled/enabled.
   Publish latency distributions, completed work, bytes, CPU/memory, measured
   energy where instrumented, verification costs and operator concentration.
5. Test grinding/withholding, fast hardware, cached/faked storage and interrupted
   work. Define failure criteria before results: abandon the extra mechanism if
   it has no defensible benefit or breaks the required safety/liveness envelope.
6. Obtain independent reproduction and targeted adversarial/cryptographic review.
   Update the prior-art comparison around the actual construction, including
   negative results. No absence-of-prior-art claim follows from this short audit.

The [PUBLIC TESTNET plan](Q1_PUBLIC_TESTNET_V0_PLAN.md) is a separate operational
gate. Passing it would show bounded public-network operation, not establish
novelty, Mainnet readiness or economic value.
