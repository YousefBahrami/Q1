Q1 Development and Research Roadmap

20_ROADMAP.md

Project: Q1 Experimental Distributed Ledger
Protocol Version: 0.1
Roadmap Version: 0.1.0
Status: Initial Strategic Roadmap
Classification: Experimental — Research and Engineering Planning Document

⸻

1. Purpose

This document defines the staged roadmap for Q1.

It translates the Q1 specification set into a sequence of:

* decisions;
* engineering milestones;
* research experiments;
* security reviews;
* deployment phases;
* governance transitions;
* release gates;
* continuation or termination decisions.

The roadmap is evidence-driven.

Q1 SHALL NOT advance merely because:

* significant time has been invested;
* substantial code has been written;
* the project has gained attention;
* test units have acquired speculative interest;
* the founders remain emotionally committed;
* a complex mechanism appears innovative.

Each phase must earn the right to continue.

⸻

2. Roadmap Principle

Q1 SHALL follow this principle:

First define.
Then build.
Then measure.
Then attack.
Then compare.
Then decide whether to continue.

The roadmap is not a promise that every listed phase will occur.

It is a controlled sequence of gates.

At any gate, the correct decision may be:

ADVANCE
ADVANCE_WITH_RESTRICTIONS
REPEAT
REDESIGN
REMOVE_COMPONENT
PAUSE
STOP_PROJECT

Stopping a weak design before public exposure is a successful research outcome.

⸻

3. Roadmap Scope

The roadmap covers:

* specification completion;
* engineering preparation;
* repository foundation;
* protocol primitives;
* ledger implementation;
* wallet implementation;
* local single-node execution;
* peer-to-peer networking;
* committee consensus;
* delay-engine experiments;
* HDD laboratory experiments;
* economic simulation;
* AI Observer deployment;
* adversarial testing;
* private testnet;
* independent review;
* public research release;
* possible public testnet;
* long-term governance transition;
* possible production-network research.

No production mainnet is authorized by this roadmap.

⸻

4. Roadmap Horizons

Q1 SHALL use five planning horizons.

Horizon 0 — Specification and readiness
Horizon 1 — Executable prototype
Horizon 2 — Adversarial private testnet
Horizon 3 — Public research network
Horizon 4 — Production feasibility decision

Each horizon contains several phases and decision gates.

⸻

5. Current Project Position

At Roadmap Version 0.1.0, Q1 has produced the initial specification structure:

00_PROJECT_CHARTER.md
01_GLOSSARY.md
02_SYSTEM_REQUIREMENTS.md
03_ARCHITECTURE.md
04_LEDGER_AND_TRANSACTIONS.md
05_CONSENSUS.md
06_DELAY_ENGINE.md
07_HDD_LAB_MODULE.md
08_NODE_AND_NETWORKING.md
09_WALLET.md
10_TOKENOMICS.md
11_AI_OBSERVER.md
12_SECURITY_MODEL.md
13_HOW_TO_BREAK_Q1.md
14_TEST_PLAN.md
15_SIMULATION_PLAN.md
16_API_SPECIFICATION.md
17_CODEX_BUILD_INSTRUCTIONS.md
18_DEPLOYMENT.md
19_GOVERNANCE.md
20_ROADMAP.md

The existence of these documents does not mean the protocol is complete.

They form the starting engineering hypothesis.

⸻

6. Horizon 0 — Specification and Readiness

Objective

Prepare Q1 for disciplined implementation before writing consensus-critical code.

Primary outputs

Specification consistency report
OPEN_DECISIONS.md
IMPLEMENTATION_NOTES.md
Requirements traceability matrix
Architecture Decision Record candidates
Threat register
Initial repository plan
Milestone M0 plan

⸻

7. Phase 0.1 — Specification Freeze for Review

Tasks

1. collect all Q1 documents in one repository;
2. normalize terminology;
3. assign stable requirement identifiers;
4. identify contradictory rules;
5. identify missing definitions;
6. identify placeholder values;
7. identify consensus-critical open decisions;
8. identify non-consensus implementation decisions;
9. verify document references;
10. create a specification changelog.

Required review questions

* Are transaction fields completely defined?
* Is canonical serialization selected?
* Are cryptographic algorithms selected?
* Is the participant-registry model sufficiently defined for a private testnet?
* Is producer selection deterministic?
* Is committee selection deterministic?
* Is round-change behavior implementable?
* Is fee accounting complete?
* Is reward remainder handling explicit?
* Is safe-mode behavior implementable?
* Can HDD and AI be disabled without altering consensus?

Exit gate

Phase 0.1 passes when all blocking contradictions are either:

* resolved;
* isolated behind explicitly approved placeholders;
* or recorded as implementation blockers.

⸻

8. Phase 0.2 — Architecture Decision Records

Required initial ADRs

ADR-0001 — Primary implementation language
ADR-0002 — Canonical serialization
ADR-0003 — Hash function
ADR-0004 — Signature algorithm
ADR-0005 — Address encoding
ADR-0006 — Storage engine
ADR-0007 — P2P transport
ADR-0008 — API framework
ADR-0009 — Test orchestration strategy
ADR-0010 — Initial participant registry

Exit gate

No protocol-core implementation begins until at least the first five ADRs are approved.

⸻

9. Phase 0.3 — Codex Readiness

The first Codex task SHALL be analytical rather than generative.

Preferred task:

Read all Q1 specification documents.
Do not implement protocol code.
Produce:
- a consistency report;
- OPEN_DECISIONS.md;
- ADR candidates;
- a requirement traceability skeleton;
- the proposed repository structure;
- the M0 implementation plan.
Identify contradictions, unsafe assumptions, missing definitions,
and decisions that block implementation.

Exit gate

Human review confirms that Codex has understood the system boundaries and has not invented material protocol rules.

⸻

10. Horizon 0 Success Criteria

Horizon 0 is complete when:

* the document set is internally navigable;
* all blocking open decisions are visible;
* implementation authority is clear;
* security assumptions are explicit;
* milestone plans exist;
* the initial repository can be created without inventing consensus rules.

⸻

11. Horizon 1 — Executable Prototype

Objective

Build the smallest complete Q1 network capable of finalizing test transfers across independent local nodes.

Target outcome

4 local nodes
2 wallets
1 common genesis
deterministic producer selection
deterministic committee selection
sequential delay
validator attestations
finalization certificate
node restart and resynchronization
HDD disabled by default
AI disabled without consensus impact

⸻

12. Phase 1.1 — Engineering Foundation

Corresponds to:

Build Milestone M0

Outputs

* monorepo;
* build tooling;
* CI;
* formatting;
* linting;
* secret scanning;
* dependency auditing;
* configuration schemas;
* genesis schema;
* empty test harnesses;
* documentation structure.

Gate

A new developer can clone, build, test, and lint the repository with one documented workflow.

⸻

13. Phase 1.2 — Protocol Primitives

Corresponds to:

Build Milestone M1

Outputs

* typed protocol identifiers;
* checked monetary types;
* canonical serialization;
* hashing;
* signatures;
* address derivation;
* domain separation;
* deterministic test vectors.

Critical gate

Two independent executions must produce identical bytes, hashes, addresses, and transaction identifiers.

Any unexplained platform-dependent output blocks advancement.

⸻

14. Phase 1.3 — Ledger and Transactions

Corresponds to:

Build Milestone M2

Outputs

* account state;
* balances;
* nonces;
* transfer transactions;
* validation;
* receipts;
* transaction roots;
* state roots;
* supply accounting;
* atomic temporary execution.

Required evidence

* 10,000 generated valid transaction sequences;
* 10,000 generated invalid transaction sequences;
* exact state determinism;
* no unauthorized issuance;
* no arithmetic wrap;
* no state change after failed transactions.

⸻

15. Phase 1.4 — Single-Node Development Chain

Corresponds to:

Build Milestone M3

Outputs

* genesis loading;
* local mempool;
* block builder;
* block processor;
* persistent database;
* local development finality;
* node health API;
* restart recovery.

Required warning

Single-node finality must remain visibly labeled as development-only.

Gate

One node creates and retains 1,000 valid local development blocks across restarts.

⸻

16. Phase 1.5 — Wallet and Public API

Corresponds to:

Build Milestone M4

Outputs

* encrypted local wallet;
* address creation;
* wallet backup;
* wallet restore;
* transaction preview;
* local signing;
* fee estimate;
* transaction submission;
* status tracking;
* offline signing;
* watch-only mode.

Demonstration

Wallet A receives genesis test units.
Wallet A sends Q1T to Wallet B.
Wallet B observes finalization.
Wallet A is restored on another environment.
The restored wallet derives the same address.

⸻

17. Phase 1.6 — Multi-Node Networking

Corresponds to:

Build Milestone M5

Outputs

* node identities;
* authenticated handshake;
* static peers;
* bootstrap discovery;
* transaction relay;
* block relay;
* synchronization;
* rate limits;
* message-size limits;
* peer scoring;
* restart and resynchronization.

Gate

Four independent node processes reach identical ledger state through P2P exchange.

One newly created node must synchronize from peers without importing an authoritative database manually.

⸻

18. Phase 1.7 — Q1 Consensus

Corresponds to:

Build Milestone M6

Outputs

* consensus context;
* round state machine;
* deterministic producer candidates;
* validator committee;
* proposals;
* attestations;
* quorum calculation;
* finalization certificate;
* fallback;
* round advancement;
* signing protection;
* equivocation evidence;
* safe mode.

Gate

The local network must demonstrate:

1. primary producer success;
2. primary producer failure;
3. fallback success;
4. invalid proposal rejection;
5. duplicate vote rejection;
6. validator restart protection;
7. forged certificate rejection;
8. conflicting certificate safe mode.

⸻

19. Phase 1.8 — Delay Engine Prototype

Corresponds to:

Build Milestone M7

Outputs

* mock delay engine;
* sequential hash delay;
* challenge binding;
* difficulty configuration;
* proof verification;
* cancellation;
* benchmark tooling;
* telemetry.

Decision question

Does the delay mechanism provide sufficient experimental value to justify continued integration?

The prototype is not required to solve production VDF security.

It must make the timing and proof pipeline executable.

⸻

20. Horizon 1 Exit Demonstration

Horizon 1 completes only when the following demonstration succeeds:

1. Start four independent Q1 nodes.
2. Load one common genesis.
3. Create two encrypted wallets.
4. Submit a signed transfer.
5. Select a producer.
6. Execute challenge-bound delay.
7. Build and propagate a valid block.
8. Collect committee attestations.
9. Construct a finalization certificate.
10. Finalize the same block on all honest nodes.
11. Stop one node.
12. Continue where quorum allows.
13. Restart the node.
14. Resynchronize it.
15. Disable AI Observer.
16. Disable HDD module.
17. Continue base consensus.
18. Verify exact supply and balances.

⸻

21. Horizon 1 Decision Gate

Possible decisions:

Advance

The prototype is deterministic, testable, and sufficiently modular.

Advance with restrictions

The core works, but some modules remain experimental or disabled.

Redesign

Consensus, delay, ledger, or networking assumptions are materially flawed.

Stop

The core cannot preserve basic deterministic safety without unacceptable complexity.

⸻

22. Horizon 2 — Adversarial Private Testnet

Objective

Operate Q1 across multiple physical hosts and attempt to break every significant assumption.

Target topology

7 or more nodes
3 or more physical hosts
2 or more network providers where possible
1 archive node
1 observer-only node
1 NAT-restricted node
1 slow node
1 malicious test node

⸻

23. Phase 2.1 — HDD Laboratory

Corresponds to:

Build Milestone M8

Initial mode

HDD_MODE = TELEMETRY_ONLY

Required experiments

* HDD versus SSD;
* HDD versus RAM disk;
* HDD versus virtual disk;
* cached versus uncached workload;
* local versus remote storage;
* duplicated datasets;
* multiple devices;
* energy measurement;
* wear estimation;
* safe device disconnection;
* path and symlink attacks.

Primary decision question

Does HDD provide measurable value that cannot be obtained more safely or simply through ordinary mathematical or archival mechanisms?

⸻

24. HDD Decision Gate

Retain for further consensus research

Only if HDD demonstrates measurable, challenge-bound value and acceptable cost.

Retain as optional archival or research module

If HDD is useful for storage or measurement but not consensus security.

Remove from consensus

If SSD, RAM, caching, outsourcing, simulation, or unverifiable telemetry defeats the intended property.

Removal is an approved successful outcome.

⸻

25. Phase 2.2 — Tokenomics Implementation

Corresponds to:

Build Milestone M9

Initial economic scope

* test-unit issuance;
* non-negative fees;
* size-based fees;
* simple congestion adjustment;
* producer reward;
* delay reward;
* validator reward;
* treasury allocation;
* deterministic remainder;
* exact supply accounting.

Prohibited at this phase

* public token sale;
* market-price input;
* guaranteed yield;
* negative anonymous fees;
* production staking;
* unlimited HDD rewards;
* permanent monetary policy.

⸻

26. Phase 2.3 — Economic Simulation

Required scenarios:

* low network use;
* high network use;
* sustained congestion;
* participant exit;
* hardware-cost inequality;
* energy-price increase;
* transaction splitting;
* artificial volume;
* Sybil reward farming;
* pool formation;
* reward concentration;
* issuance-dominant security;
* fee-dominant security;
* hybrid security;
* 10-year and 50-year horizons.

Gate

No economic configuration advances if it contains:

* unauthorized value creation;
* obvious infinite reward loops;
* negative-fee extraction;
* trivial identity farming;
* unexplained reward concentration;
* unaccounted supply.

⸻

27. Phase 2.4 — Explorer, Telemetry, and AI Observer

Corresponds to:

Build Milestone M10

Initial observer approach

1. deterministic security rules;
2. statistical anomaly detection;
3. optional model adapters;
4. explanation and incident reporting.

AI Observer restrictions

The observer must have:

no consensus vote
no signing key
no ledger write access
no penalty authority
no wallet secret access

Required observer evidence

* objective equivocation detection;
* finality conflict detection;
* producer concentration reporting;
* validator concentration reporting;
* partition indicators;
* delay anomalies;
* HDD anomalies;
* false-positive workflow;
* operation without observer availability.

⸻

28. Phase 2.5 — Adversarial Framework

Corresponds to:

Build Milestone M11

Required malicious profiles

* malicious user;
* malicious producer;
* malicious validator;
* Sybil operator;
* eclipse operator;
* fee manipulator;
* HDD fraud operator;
* malicious synchronization peer;
* telemetry poisoner.

Required fault injection

* node crash;
* disk full;
* database corruption;
* clock skew;
* packet loss;
* network delay;
* partition;
* HDD disconnect;
* AI outage;
* telemetry outage.

⸻

29. Phase 2.6 — Private Testnet Deployment

Corresponds to:

Build Milestone M12

Required outputs

* node package;
* wallet package;
* explorer;
* observer;
* test orchestrator;
* genesis tooling;
* deployment guide;
* operator guide;
* incident guide;
* key-management guide;
* backup and recovery guide.

Required run

1,000 consecutive finalized blocks
10,000 finalized transfers
multiple node restarts
at least one controlled partition
at least one producer failure
at least one malicious validator scenario
at least one invalid delay-proof attack

⸻

30. Phase 2.7 — Red-Team Gate

The private testnet must be attacked according to:

12_SECURITY_MODEL.md
13_HOW_TO_BREAK_Q1.md
14_TEST_PLAN.md

Critical mandatory attacks

* unauthorized issuance;
* double spend;
* state-root mismatch;
* producer equivocation;
* validator equivocation;
* forged finalization certificate;
* conflicting finality;
* proof replay;
* verification DoS;
* eclipse simulation;
* partition;
* malicious snapshot;
* wallet network mismatch;
* negative-fee attempt;
* Sybil reward farming;
* genesis substitution;
* database rollback;
* secret leakage scan.

⸻

31. Horizon 2 Exit Criteria

Horizon 2 is complete only when:

* no open critical vulnerability remains;
* all honest nodes preserve identical finalized state;
* supply remains exact;
* critical failures trigger safe mode;
* red-team findings have regression tests;
* HDD has a documented retention or removal decision;
* AI failure has no consensus impact;
* reward concentration is measured;
* energy consumption is measured;
* known limitations are documented;
* seven-day endurance testing has completed or has a documented blocker.

⸻

32. Horizon 2 Decision Gate

Advance to public research

Only if the network is stable enough to expose to external researchers without misleading them about security or value.

Extend private testing

If critical safety is preserved but performance, economics, or operations remain immature.

Redesign core components

If consensus, randomness, delay, admission, or economics produce unacceptable concentration or attackability.

Stop project

If the design fails its foundational objectives or cannot improve without becoming a conventional high-cost or centralized system.

⸻

33. Horizon 3 — Public Research Network

Objective

Expose Q1 to independent developers, researchers, node operators, reviewers, and adversarial participants while preserving its experimental status.

This horizon is optional.

⸻

34. Public Research Preconditions

Before Horizon 3:

* source code must be published under an approved license;
* builds must be signed or reproducible;
* security reporting must exist;
* public QIP process must exist;
* test units must be clearly non-investment units;
* participant-admission limitations must be disclosed;
* Sybil limitations must be disclosed;
* wallet warnings must be visible;
* destructive HDD behavior must be disabled;
* no public promise of profitability may exist.

⸻

35. Phase 3.1 — Independent Specification Review

Independent reviewers SHOULD evaluate:

* ledger rules;
* consensus safety;
* finality;
* cryptography;
* randomness;
* delay design;
* wallet security;
* networking;
* tokenomics;
* governance;
* deployment.

Required output

INDEPENDENT_REVIEW_REPORT.md
PROJECT_RESPONSE_TO_REVIEW.md

Material findings must be resolved, accepted as limitations, or block advancement.

⸻

36. Phase 3.2 — Independent Client Feasibility

Q1 SHOULD encourage at least one independent implementation effort or independent protocol-vector verifier.

Initial goal:

* independently parse transactions;
* verify signatures;
* reconstruct blocks;
* verify state roots;
* verify certificates;
* verify economic calculations.

A full second client is not immediately required.

The goal is to expose undocumented implementation behavior.

⸻

37. Phase 3.3 — Public Testnet Alpha

Possible topology:

project-operated bootstrap nodes
independent community full nodes
permissioned or limited validator set
public wallets
public explorer
public telemetry dashboard
public issue tracker

The validator or producer admission model must remain explicitly described.

Alpha test units SHALL have no guaranteed continuity.

The network may reset.

⸻

38. Public Alpha Research Questions

* Can independent operators deploy nodes?
* Can nodes synchronize reliably?
* Does one implementation dominate operationally?
* Can public traffic overwhelm APIs?
* How quickly do Sybil peers appear?
* Does reward variance create pools?
* Are fees understandable?
* Are wallets used safely?
* Does participant concentration rise?
* Are observers producing useful or noisy alerts?
* Do external researchers find new attack paths?

⸻

39. Phase 3.4 — Public Adversarial Program

The project MAY create:

* vulnerability disclosure;
* public bug bounty;
* attack competitions;
* simulation challenges;
* independent economic analysis grants;
* HDD falsification challenges;
* delay optimization challenges.

Attack incentives must not encourage attacks against unrelated systems.

⸻

40. Phase 3.5 — Delay Engine Decision

By this phase, Q1 must decide whether to:

retain sequential hash only as prototype
adopt a reviewed formal VDF
replace delay with another reviewed mechanism
remove delay from consensus
redesign producer cost entirely

Formal VDF adoption requires

* reviewed construction;
* compact proof;
* efficient verification;
* deterministic implementation;
* cross-platform test vectors;
* security review;
* activation plan.

⸻

41. Phase 3.6 — Participant Admission Decision

Q1 must confront the public Sybil-resistance problem.

Candidate research directions MAY include:

* stake with concentration limits;
* resource commitments;
* identity-based systems;
* invitation or federation;
* hybrid admission;
* rotating committees;
* verifiable service contribution;
* another reviewed model.

No solution may be adopted solely because it is fashionable.

The design must be tested against:

* wealth concentration;
* identity farming;
* hardware concentration;
* cartel formation;
* censorship;
* public accessibility.

⸻

42. Phase 3.7 — Economic Policy Decision

Public research must determine whether Q1 can support a defensible long-term policy.

Questions:

* Is supply capped, bounded-inflationary, or adaptive?
* How is long-term security funded?
* Are fees affordable for ordinary use?
* Are small participants viable?
* Do reward pools centralize?
* Is treasury power excessive?
* Does the system reward useful security or waste?
* Can policy survive low adoption?
* Can policy survive high adoption?

No market launch occurs before these questions have credible answers.

⸻

43. Phase 3.8 — Governance Transition

The project SHOULD move from founder-led research toward:

maintainer-led engineering
independent security review
multi-party treasury control
public QIP process
distributed release authority
documented emergency committee

Founder authority must become more limited and explicit.

⸻

44. Horizon 3 Exit Criteria

Horizon 3 is complete when:

* independent nodes operate;
* independent reviewers have assessed Q1;
* critical public findings are resolved;
* participant admission has a credible direction;
* delay mechanism has a formal decision;
* economic policy has a defensible research conclusion;
* governance authority is less concentrated;
* releases are verifiable;
* the network has survived sustained public adversarial use;
* project claims remain consistent with evidence.

⸻

45. Horizon 3 Decision Gate

Proceed to production-feasibility research

Only if Q1 demonstrates a meaningful advantage over simpler existing designs.

Remain a research network

If the system provides useful experimentation but lacks production-ready economics or Sybil resistance.

Publish findings and stop protocol development

If research is valuable but Q1 does not justify becoming a financial network.

Redesign as a different system

If useful components belong in another architecture, such as:

* archival network;
* verifiable delay research;
* distributed simulation framework;
* consensus laboratory;
* wallet-security toolkit.

⸻

46. Horizon 4 — Production Feasibility Decision

Objective

Determine whether Q1 should ever become a production-value network.

This horizon does not presume that the answer is yes.

⸻

47. Production Feasibility Questions

Q1 must answer:

1. What unique problem does Q1 solve better than simpler systems?
2. Is its consensus secure under public admission?
3. Is its security budget sustainable?
4. Can ordinary users participate?
5. Are fees suitable for intended use?
6. Does the delay mechanism justify its cost?
7. Has HDD been retained only where genuinely useful?
8. Can the network resist operator concentration?
9. Can governance resist founder, treasury, validator, and wealth capture?
10. Can independent clients interoperate?
11. Are wallet and key risks acceptable?
12. Are releases reproducible?
13. Can critical incidents be handled without hidden sovereignty?
14. Are legal and regulatory risks understood?
15. Is public financial use ethically and operationally justified?

⸻

48. Production Feasibility Evidence

Required evidence SHOULD include:

* independent cryptographic review;
* independent consensus review;
* economic audits;
* long-horizon simulations;
* public adversarial results;
* multi-client vectors;
* operational history;
* governance history;
* treasury controls;
* legal assessment;
* energy and environmental assessment;
* public risk disclosures.

⸻

49. Mainnet Specification Requirement

If production feasibility is approved, Q1 must create a new specification set.

It must not reuse v0.1 documents as if they were production-ready.

Required future documents MAY include:

MAINNET_CHARTER.md
MAINNET_CONSENSUS.md
MAINNET_MONETARY_POLICY.md
MAINNET_GENESIS_POLICY.md
MAINNET_PARTICIPANT_ADMISSION.md
MAINNET_GOVERNANCE.md
MAINNET_SECURITY_REVIEW.md
MAINNET_INCIDENT_RESPONSE.md
MAINNET_LEGAL_DISCLOSURES.md

⸻

50. No Automatic Mainnet Transition

Public testnet success MUST NOT automatically trigger:

* token sale;
* exchange listing;
* public financial promotion;
* production genesis;
* migration of test balances;
* promise of future value.

Any production launch requires separate approval and documentation.

⸻

51. Component-Specific Roadmaps

51.1 Ledger

Prototype account ledger
→ property-tested state engine
→ snapshot support
→ independent verification
→ production-feasibility review

51.2 Consensus

Private registry
→ deterministic committees
→ adversarial private testnet
→ public admission research
→ formal safety review

51.3 Delay

Mock
→ sequential hash
→ benchmarks
→ formal VDF candidates
→ retain, replace, or remove

51.4 HDD

Telemetry only
→ comparative lab testing
→ optional archival research
→ bounded reward research if justified
→ retain or remove

51.5 Wallet

CLI wallet
→ offline signing
→ hardened desktop interface
→ hardware-signer adapters
→ public usability review

51.6 AI Observer

Rule engine
→ statistical observer
→ model adapters
→ independent evaluation
→ keep strictly non-consensus

51.7 Tokenomics

Test issuance
→ deterministic accounting
→ simulation
→ private-testnet measurement
→ public research
→ production policy decision

⸻

52. Suggested Time Planning

Exact dates SHALL be set only after Horizon 0 review.

A provisional engineering order MAY be:

Months 0–2:
Specification review, ADRs, M0
Months 2–5:
Protocol primitives, ledger, single node
Months 5–8:
Wallet, APIs, networking
Months 8–12:
Consensus, delay engine, safe mode
Months 12–15:
HDD lab, tokenomics, telemetry
Months 15–18:
AI Observer, simulator, adversarial tooling
Months 18–24:
Private distributed testnet and endurance testing
After Month 24:
Independent review and public research decision

These periods are planning estimates, not commitments.

Progress depends on:

* team size;
* code quality;
* security findings;
* funding;
* independent review;
* redesign requirements.

⸻

53. Roadmap Without Calendar Dependence

The authoritative roadmap SHALL remain gate-based rather than date-based.

A delayed but correct phase is preferable to an on-time unsafe release.

No phase advances because a public announcement promised a date.

⸻

54. Team Growth Roadmap

Initial stage

Possible roles may be combined:

* founder;
* protocol designer;
* specification editor;
* Codex-assisted implementer.

Prototype stage

Required responsibilities:

* protocol engineer;
* systems engineer;
* security reviewer;
* test engineer.

Private-testnet stage

Additional responsibilities:

* operations engineer;
* wallet engineer;
* economic modeler;
* adversarial tester;
* data and telemetry engineer.

Public research stage

Additional independent roles:

* cryptographic reviewer;
* external client team;
* governance facilitator;
* treasury stewards;
* legal counsel;
* public security researchers.

One person may initially perform several roles, but concentration must be disclosed.

⸻

55. Funding Roadmap

Q1 funding SHOULD advance gradually.

Possible stages:

Founder-funded specification
Small prototype budget
Research grants
Private engineering support
Independent audit funding
Public research grants
Institutional or foundation funding

Funding MUST NOT force premature financial issuance.

⸻

56. No Premature Token Funding

Q1 SHOULD NOT fund early development through a public token sale merely because building infrastructure is expensive.

Before any future public issuance, the project must have:

* working code;
* tested consensus;
* credible economics;
* legal review;
* public risk disclosure;
* treasury controls;
* governance rules.

⸻

57. Intellectual Property and Publication Roadmap

Q1 SHOULD determine:

* source-code license;
* specification license;
* trademark policy;
* patent posture if relevant;
* research-publication policy;
* contributor agreement requirements.

The project SHOULD preserve the ability for independent compatible implementations to exist.

⸻

58. Research Publication Plan

Q1 MAY publish reports on:

* sequential delay benchmarks;
* HDD falsification experiments;
* energy comparisons;
* committee concentration;
* Sybil simulations;
* reward variance;
* private-testnet attack results;
* safe-mode behavior;
* governance capture models.

Reports must distinguish:

* measurement;
* simulation;
* inference;
* unresolved uncertainty.

⸻

59. Roadmap Metrics

The roadmap SHOULD track:

Engineering

* requirements covered;
* test coverage;
* open critical defects;
* deterministic vectors;
* milestone completion.

Network

* finalized blocks;
* finality time;
* fallback rate;
* synchronization success;
* partition recovery.

Security

* attacks attempted;
* attacks blocked;
* vulnerabilities found;
* regression tests added;
* safe-mode events.

Economics

* supply accuracy;
* fee distribution;
* reward concentration;
* small-participant viability;
* security budget.

Research

* HDD value;
* delay asymmetry;
* energy use;
* simulator calibration;
* independent review findings.

Governance

* authority concentration;
* active reviewers;
* treasury signers;
* QIPs;
* unresolved dissent.

⸻

60. Quarterly or Milestone Review

During active development, Q1 SHOULD perform a roadmap review after each major milestone or at least every three months.

The review SHALL ask:

* What was completed?
* What failed?
* Which assumptions changed?
* Which modules should be removed?
* Which risks increased?
* What evidence supports continuation?
* What resources are required?
* Has the project drifted from its charter?

⸻

61. Roadmap Change Procedure

Material roadmap changes SHOULD include:

* reason;
* affected phases;
* changed assumptions;
* security impact;
* funding impact;
* schedule impact;
* approval;
* version update.

The roadmap MUST not be silently rewritten to make missed goals appear completed.

⸻

62. Critical Stop Conditions

Q1 SHOULD stop or fundamentally redesign if evidence shows:

* persistent unauthorized issuance risk;
* inability to prevent conflicting honest finality;
* no viable public Sybil-resistance path;
* security dependent on unverifiable hardware claims;
* energy cost comparable to conventional proof-of-work without additional benefit;
* inevitable extreme operator concentration;
* economics dependent on perpetual speculative growth;
* hidden administrative control required for stability;
* wallet security unsuitable for intended users;
* governance cannot limit founder or treasury sovereignty;
* project complexity greatly exceeds measurable value.

⸻

63. Component Removal Conditions

Remove HDD from consensus if

* physical origin cannot be verified;
* faster media dominates;
* wear and energy outweigh benefit;
* centralization increases;
* useful storage is not achieved.

Remove AI models if

* false positives overwhelm operators;
* models add no measurable detection value;
* privacy costs are unjustified;
* operators begin treating inference as truth.

Replace sequential delay if

* verification cost is excessive;
* specialized hardware dominates;
* no formal sequentiality path exists;
* simpler mechanisms perform better.

Replace economic rules if

* fees become unusable;
* rewards create Sybil extraction;
* small participants cannot remain viable;
* security budget collapses.

⸻

64. Success Definitions

Q1 may succeed in several ways.

Protocol success

A secure and useful distributed ledger emerges.

Research success

Q1 discovers and publishes valuable findings even if no production network launches.

Component success

Specific components become useful independently, such as:

* simulation framework;
* delay benchmark suite;
* consensus testing toolkit;
* HDD falsification research;
* AI security observer;
* wallet-security architecture.

Educational success

Q1 becomes a clear, reproducible model for teaching distributed-system design.

Production launch is not the only definition of success.

⸻

65. Failure Definitions

Q1 fails if it:

* hides known weaknesses;
* promotes value before security;
* continues only because of sunk cost;
* presents permissioned control as decentralization;
* rewards unverifiable work;
* allows AI to become hidden authority;
* creates environmental cost without measurable benefit;
* uses governance language to conceal founder sovereignty;
* turns test units into speculative promises prematurely;
* refuses to remove failed components.

⸻

66. Decision Record at Every Major Gate

Every major gate SHALL produce:

ROADMAP_DECISION_ID
Date
Phase
Evidence reviewed
Tests passed
Tests failed
Critical risks
Open decisions
Decision
Conditions
Responsible reviewers
Dissent
Next authorized phase

⸻

67. Repository Roadmap Structure

Recommended:

roadmap/
├── README.md
├── ROADMAP_DECISIONS.md
├── milestone_reports/
├── quarterly_reviews/
├── risk_reviews/
├── component_decisions/
└── archived_versions/

⸻

68. Initial Authorized Next Step

After completing this document set, the next authorized project action is:

Specification consistency and implementation-readiness review

Not:

Immediate full blockchain implementation

The first Codex execution SHALL therefore produce:

1. a consistency report;
2. blocking open decisions;
3. ADR candidates;
4. requirement traceability skeleton;
5. repository structure;
6. M0 plan.

⸻

69. First Engineering Deliverable

The first executable engineering deliverable SHALL be:

Q1 M0 Repository Foundation

It SHALL contain no unauthorized consensus invention.

⸻

70. First Public Demonstration

The first meaningful technical demonstration SHOULD show:

* four local nodes;
* independent node identities;
* two local wallets;
* one valid transfer;
* sequential delay;
* committee attestations;
* one finalization certificate;
* exact shared state;
* node restart;
* synchronization;
* AI disabled;
* HDD disabled.

The demonstration SHOULD not include price, investment, or public-token claims.

⸻

71. First Research Publication

The first research publication SHOULD preferably concern evidence rather than promotion.

Candidate title:

Q1: An Experimental Framework for Limited Producer Work,
Committee Finality, Replaceable Delay Mechanisms,
and Falsifiable HDD Participation

The publication SHOULD disclose:

* permissioned prototype limitations;
* sequential-hash limitations;
* HDD authenticity limitations;
* Sybil-resistance limitations;
* economic uncertainty;
* measured results.

⸻

72. First External Review Request

External reviewers SHOULD receive:

* the specification set;
* source commit;
* deterministic vectors;
* test reports;
* threat register;
* known limitations;
* explicit review questions.

They SHOULD NOT be asked merely whether they “like” the idea.

⸻

73. Long-Term Q1 Vision

The long-term vision is not merely to create another tradable token.

Q1 aims to test whether a distributed-value system can be:

* understandable;
* widely participatory;
* resistant to hidden authority;
* economically accountable;
* environmentally measurable;
* modular enough to remove failed assumptions;
* secure through verification rather than mythology.

Whether Q1 itself becomes that system remains an open question.

⸻

74. Open Roadmap Decisions

The following remain unresolved:

1. primary implementation language;
2. initial engineering team structure;
3. development budget;
4. expected M0 start date;
5. private-testnet host strategy;
6. independent security-review timing;
7. formal VDF research partner;
8. participant-admission research path;
9. source-code license;
10. trademark strategy;
11. legal entity;
12. public research publication venue;
13. public testnet admission model;
14. bug-bounty funding;
15. treasury formation;
16. external client funding;
17. public telemetry policy;
18. test-unit continuity policy;
19. production-feasibility authority;
20. final stop criteria ownership.

All decisions MUST be tracked in:

OPEN_DECISIONS.md

⸻

75. Codex Implementation Rules

Codex MUST:

1. treat roadmap phases as authorization boundaries;
2. implement only the currently approved milestone;
3. create a report at every milestone;
4. preserve failed tests and findings;
5. update the roadmap when approved scope changes;
6. distinguish completed work from planned work;
7. refuse to label placeholders as production mechanisms;
8. maintain component-removal capability;
9. keep HDD and AI optional;
10. preserve gate-based progression;
11. link milestones to requirements and tests;
12. record deterministic seeds and artifact hashes;
13. avoid public-release automation before approval;
14. avoid mainnet code paths not authorized by specifications;
15. maintain known-limitations documents;
16. generate decision-support evidence rather than promotional conclusions;
17. support independent reproducibility;
18. keep simulation separate from live state;
19. stop progression after critical invariant failure;
20. never reinterpret roadmap ambition as permission to bypass safety gates.

Codex MUST NOT:

* build all milestones in one uncontrolled pass;
* create a production genesis;
* implement a token sale;
* enable public financial use;
* hide unresolved security findings;
* retain failed HDD or AI mechanisms merely because code exists;
* present schedule estimates as guarantees;
* treat public attention as a release criterion;
* advance after critical safety failure;
* convert testnet activity into claims of proven value;
* rewrite milestone history to conceal delays or failures;
* assume that continuation is always the correct outcome.

⸻

76. Final Roadmap Principle

Q1 SHALL preserve this principle:

A roadmap is not a prophecy.
It is a sequence of questions that the project must answer with evidence.

The first question is not:

How quickly can Q1 become valuable?

It is:

Can Q1 become correct?

The next question is not:

How many users can be attracted?

It is:

Can independent users verify and safely use it?

The next question is not:

Can the project avoid failure?

It is:

Can it expose failure before failure harms the public?

Q1 advances only when:

* specifications are clearer than the code they govern;
* tests are stronger than the claims they examine;
* attacks are treated as information;
* weak components can be removed;
* economics remain accountable;
* governance remains limited;
* and every new phase is justified by what the previous phase actually proved.

The final objective is not to force Q1 into existence.

The objective is to determine, honestly and rigorously, whether Q1 deserves to exist.