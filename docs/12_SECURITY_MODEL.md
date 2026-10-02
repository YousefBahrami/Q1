Q1 Security Model

12_SECURITY_MODEL.md

Project: Q1 Experimental Distributed Ledger
Protocol Version: 0.1
Document Version: 0.1.0
Status: Draft for Security and Engineering Review
Classification: Experimental — Not for Production or Financial Use

⸻

1. Purpose

This document defines the security model of Q1 v0.1.

It specifies:

* protected assets;
* trust boundaries;
* security assumptions;
* attacker classes;
* consensus threats;
* cryptographic threats;
* wallet threats;
* node and network threats;
* delay-engine threats;
* HDD threats;
* AI Observer threats;
* economic threats;
* software supply-chain threats;
* operational threats;
* safe-failure requirements;
* incident evidence;
* security testing;
* disclosure and remediation principles.

The purpose of this document is not to claim that Q1 is secure.

Its purpose is to make security claims explicit, limited, testable, and falsifiable.

ADR-0002 through ADR-0005 select the initial serialization, hash, signature,
and address foundations. Their strict parser, domain, malformed-input,
key-custody, and wrong-network rules are specified in the proposed V1 profiles
under `docs/protocol/`; generic library defaults do not satisfy those rules.

⸻

2. Security Philosophy

Q1 SHALL follow this principle:

No component is trusted merely because it belongs to the project.
Every authority must be limited.
Every important claim must be independently verifiable.
Every critical failure must be visible.
When safety and progress conflict, safety takes priority.

Q1 security is based on layered defenses.

No single mechanism is assumed sufficient.

⸻

3. Security Objectives

Q1 v0.1 SHALL attempt to preserve:

3.1 Ledger integrity

Unauthorized units MUST NOT be created.

Balances MUST change only through valid protocol rules.

⸻

3.2 Authorization integrity

Only valid signatures may authorize spending or consensus actions.

⸻

3.3 Consensus safety

Honest nodes MUST NOT finalize conflicting blocks under the stated fault assumptions.

⸻

3.4 Consensus liveness

The network SHOULD continue finalizing blocks when sufficient honest participants and network connectivity exist.

⸻

3.5 Finality integrity

A block MUST NOT be treated as final without a valid finalization certificate.

⸻

3.6 Replay resistance

Protocol objects MUST NOT be reusable across incompatible:

* chains;
* heights;
* rounds;
* roles;
* protocol versions;
* challenges.

⸻

3.7 Availability

Ordinary faults and malformed traffic SHOULD NOT crash the entire network.

⸻

3.8 Key confidentiality

Private keys and recovery secrets MUST remain confidential.

⸻

3.9 Deterministic verification

Honest nodes MUST reach the same validity result from the same inputs.

⸻

3.10 Evidence preservation

Objective attacks and critical failures MUST produce preservable evidence.

⸻

4. Security Non-Objectives

Q1 v0.1 does not guarantee:

* anonymity;
* legal identity;
* transaction reversibility;
* recovery from stolen keys;
* protection from malware on an unlocked wallet;
* public-mainnet Sybil resistance;
* nation-state censorship resistance;
* perfect DDoS resistance;
* physical HDD authenticity;
* trusted geolocation;
* secure public participant admission;
* market-price stability;
* regulatory compliance;
* quantum resistance;
* bug-free software;
* complete AI attack detection.

These limitations MUST remain explicit.

⸻

5. Protected Assets

Q1 security SHALL protect the following assets.

5.1 Ledger state

* balances;
* nonces;
* total supply;
* treasury balances;
* reward state;
* finalized block history.

⸻

5.2 Consensus state

* participant registry;
* producer selection;
* committee selection;
* attestations;
* finalization certificates;
* protocol parameters;
* epoch state.

⸻

5.3 Cryptographic secrets

* wallet private keys;
* recovery seeds;
* node identity private keys;
* producer signing keys;
* validator attestation keys;
* administrative credentials.

⸻

5.4 Protocol availability

* P2P connectivity;
* block propagation;
* attestation propagation;
* synchronization;
* finalization progress.

⸻

5.5 Software integrity

* source code;
* build outputs;
* release artifacts;
* dependencies;
* configuration;
* genesis files;
* model artifacts.

⸻

5.6 Research integrity

* telemetry;
* benchmark data;
* attack results;
* AI Observer reports;
* energy measurements;
* HDD experiment results.

⸻

6. Trust Model

Q1 SHALL minimize required trust.

6.1 Untrusted users

Users may submit:

* invalid transactions;
* malformed data;
* conflicting transactions;
* spam;
* misleading metadata.

Nodes MUST verify independently.

⸻

6.2 Untrusted peers

Peers may lie about:

* finalized height;
* chain state;
* identity;
* availability;
* network location;
* hardware;
* protocol behavior.

⸻

6.3 Untrusted producers

A producer may:

* propose invalid blocks;
* censor transactions;
* equivocate;
* submit invalid delay proofs;
* manipulate block contents;
* withhold proposals;
* abuse fallback timing.

⸻

6.4 Untrusted validators

A validator may:

* sign invalid blocks;
* double-vote;
* remain offline;
* delay attestations;
* collude;
* selectively censor.

⸻

6.5 Untrusted HDD module

The HDD module may:

* fabricate telemetry;
* use SSD or RAM;
* replay old outputs;
* outsource work;
* misreport energy;
* misreport physical device properties.

⸻

6.6 Untrusted AI Observer

The AI Observer may:

* make mistakes;
* be manipulated;
* hallucinate;
* miss attacks;
* generate false positives;
* become unavailable.

Therefore it has no consensus authority.

⸻

6.7 Untrusted operators

Operators may misconfigure nodes, expose keys, alter software, or attempt administrative bypass.

Administrative control MUST NOT permit ordinary rewriting of finalized state.

⸻

6.8 Untrusted software dependencies

Third-party libraries may contain:

* vulnerabilities;
* malicious code;
* compromised updates;
* inconsistent behavior.

Dependencies MUST be minimized, pinned, audited, and reproducibly built where practical.

⸻

7. Trust Boundaries

Q1 SHALL recognize at least these boundaries:

Wallet ↔ Node
Node ↔ Peer
Consensus Core ↔ Storage
Consensus Core ↔ Delay Engine
Consensus Core ↔ HDD Plugin
Node ↔ AI Observer
Node ↔ Telemetry Backend
Operator ↔ Administrative API
Build System ↔ Dependencies
Release Artifact ↔ User

Every boundary MUST have:

* input validation;
* explicit permissions;
* bounded resource use;
* failure handling;
* logging.

⸻

8. Attacker Classes

Q1 SHALL consider the following attacker classes.

8.1 External network attacker

Capabilities:

* connect as peers;
* flood messages;
* send malformed data;
* delay or drop traffic;
* attempt eclipse;
* attempt partition.

⸻

8.2 Malicious user

Capabilities:

* submit invalid or conflicting transactions;
* spam;
* exploit fee rules;
* exploit wallet behavior.

⸻

8.3 Malicious producer

Capabilities:

* produce invalid blocks;
* equivocate;
* manipulate transaction inclusion;
* withhold valid blocks;
* submit forged delay or HDD claims.

⸻

8.4 Malicious validator

Capabilities:

* sign invalid blocks;
* double-vote;
* withhold attestations;
* collude;
* delay finality.

⸻

8.5 Sybil operator

Capabilities:

* create many identities;
* dominate peer tables;
* seek committee influence;
* farm rewards;
* simulate decentralization.

⸻

8.6 Privileged operator attacker

Capabilities:

* access servers;
* modify configuration;
* steal role keys;
* alter telemetry;
* manipulate deployments.

⸻

8.7 Supply-chain attacker

Capabilities:

* compromise source dependencies;
* compromise build systems;
* distribute malicious binaries;
* alter model files;
* alter genesis or configuration.

⸻

8.8 Economic attacker

Capabilities:

* manipulate fees;
* create artificial volume;
* form cartels;
* exploit rewards;
* create many identities;
* attack treasury rules.

⸻

8.9 Advanced hardware attacker

Capabilities:

* build optimized delay hardware;
* use SSD, RAM, FPGA, GPU, ASIC;
* fake HDD behavior;
* centralize execution.

⸻

8.10 AI-focused attacker

Capabilities:

* poison telemetry;
* inject misleading logs;
* evade models;
* flood alerts;
* replace model artifacts;
* exploit prompt injection.

⸻

9. Security Assumptions

Q1 v0.1 depends on explicit assumptions.

Q1-SEC-001 — Cryptographic assumptions

Approved cryptographic primitives are assumed secure against practical attacks under current knowledge.

⸻

Q1-SEC-002 — Key secrecy

Honest participants are assumed to protect private keys.

A compromised key may validly authorize malicious actions.

⸻

Q1-SEC-003 — Byzantine threshold

Consensus safety assumes less than one-third of relevant committee weight equivocates under the intended model.

⸻

Q1-SEC-004 — Deterministic execution

Honest implementations are assumed to execute consensus rules identically.

⸻

Q1-SEC-005 — Network availability

Liveness assumes sufficient communication among honest participants.

⸻

Q1-SEC-006 — Finalized participant registry

Committee and producer selection assume a shared finalized participant set.

⸻

Q1-SEC-007 — Delay verification

Security assumes delay proofs are verified correctly.

⸻

Q1-SEC-008 — Software integrity

Honest nodes are assumed to run non-malicious software builds.

⸻

Q1-SEC-009 — Prototype admission limitation

The first private testnet may use a permissioned participant registry.

This does not prove public permissionless security.

⸻

10. Core Security Invariants

Invariant 1 — No unauthorized issuance

Only explicit protocol rules may create units.

⸻

Invariant 2 — No negative balances

No finalized account may have a negative balance.

⸻

Invariant 3 — One nonce, one finalized spend

One account nonce may be consumed by at most one finalized transaction.

⸻

Invariant 4 — One finalized block per height

An honest node finalizes at most one block at each height.

⸻

Invariant 5 — Valid parent

Every finalized block references a finalized parent.

⸻

Invariant 6 — Valid threshold

No finalization occurs below the required committee threshold.

⸻

Invariant 7 — Delay binding

A delay proof is valid only for its exact context.

⸻

Invariant 8 — Signature binding

Changing signed data invalidates the signature.

⸻

Invariant 9 — AI independence

Consensus continues without AI.

⸻

Invariant 10 — HDD independence

Base consensus continues without HDD.

⸻

Invariant 11 — Accounting conservation

All supply, fee, reward, burn, and treasury changes remain accounted.

⸻

Invariant 12 — Safe conflict response

Conflicting finality causes safe mode.

⸻

11. Cryptographic Security

Q1 MUST use reviewed cryptographic libraries.

The cryptographic provider SHALL isolate:

* hashes;
* signatures;
* key generation;
* secure randomness;
* address derivation;
* commitments.

Custom cryptography MUST NOT be introduced merely for novelty.

⸻

12. Cryptographic Threats

The security test plan MUST consider:

* signature forgery;
* key substitution;
* signature malleability;
* hash collision;
* hash preimage attacks;
* weak randomness;
* address collision;
* domain-separation failure;
* canonical-serialization ambiguity;
* invalid-curve or malformed-key attacks;
* oversized cryptographic objects;
* algorithm downgrade.

⸻

13. Domain Separation

Every signed and hashed protocol object MUST use explicit domain separation.

Failure to separate domains may permit:

* transaction signature reuse as attestation;
* block signature reinterpretation;
* cross-protocol replay;
* version confusion.

⸻

14. Randomness Security

Secure randomness is required for:

* key generation;
* handshake challenges;
* local non-consensus sampling;
* future VRF or selection tools.

Failure of secure randomness MUST cause safe rejection.

The system MUST NOT silently fall back to predictable randomness.

⸻

15. Canonical Serialization Security

Consensus objects MUST have one canonical byte representation.

The implementation MUST reject:

* duplicate fields;
* alternative integer encodings;
* ambiguous optional fields;
* trailing bytes;
* unsupported critical fields;
* non-deterministic map order.

⸻

16. Wallet Security

The wallet MUST protect against:

* plaintext key storage;
* weak key derivation;
* password leakage;
* shell-history leakage;
* clipboard substitution;
* wrong-network transactions;
* malicious node responses;
* backup corruption;
* transaction alteration;
* nonce conflicts.

⸻

17. Wallet Compromise

If malware controls an unlocked wallet, Q1 cannot guarantee key safety.

Mitigations include:

* offline signing;
* auto-lock;
* encrypted keystore;
* complete transaction preview;
* future hardware signing;
* signed release artifacts.

⸻

18. Key Loss

Q1 v0.1 has no central key recovery.

Loss of key and recovery material may cause permanent loss of access.

This is a security property and usability risk.

⸻

19. Key Theft

A valid signature from a stolen key is indistinguishable from an authorized signature at protocol level.

Future recovery or delayed-spend mechanisms are outside v0.1.

⸻

20. Node Key Separation

Wallet, node identity, producer, and validator keys MUST remain separate.

This reduces the impact of one key compromise.

⸻

21. Validator Signing Protection

Validators MUST persist signing history before broadcasting attestations.

This prevents accidental double-signing after restart or rollback.

⸻

22. Consensus Threats

The security model SHALL test:

* invalid proposal;
* producer equivocation;
* validator equivocation;
* quorum forgery;
* committee manipulation;
* participant registry divergence;
* round manipulation;
* fallback abuse;
* finality conflict;
* long-range history;
* stale-node voting;
* unsafe recovery.

⸻

23. Producer Equivocation

A producer signing two different proposals at the same height and round produces objective evidence.

The system MUST:

* detect it;
* preserve both signed proposals;
* prevent silent reward;
* support testnet penalties.

⸻

24. Validator Equivocation

A validator signing conflicting blocks at the same height and round produces objective evidence.

The system MUST:

* detect duplicate signer identities;
* exclude double-counting;
* store evidence;
* support penalty experiments.

⸻

25. Quorum Forgery

A finalization certificate MUST verify:

* committee definition;
* membership;
* individual signatures;
* unique validators;
* total weight;
* threshold;
* exact block binding.

⸻

26. Committee Capture

A malicious operator may attempt to dominate committees through:

* many identities;
* participant-registry manipulation;
* weighting exploitation;
* operator concentration.

Q1 v0.1 does not yet solve public Sybil resistance.

Private testnet claims MUST remain limited accordingly.

⸻

27. Randomness Grinding

Participants may attempt to manipulate selection seeds by:

* choosing transaction sets;
* withholding blocks;
* selecting among candidate outputs;
* influencing previous delay outputs.

Randomness bias MUST be measured and documented.

⸻

28. Censorship

A producer or validator cartel may censor transactions.

Q1 SHOULD measure:

* inclusion delay;
* repeated exclusion;
* participant-level censorship patterns;
* fallback inclusion behavior.

Q1 v0.1 does not guarantee censorship resistance.

⸻

29. Liveness Attacks

Attackers may:

* withhold proposals;
* withhold attestations;
* trigger repeated round timeouts;
* isolate validators;
* flood consensus queues.

The network SHOULD preserve safety even when liveness fails.

⸻

30. Finality Conflict

Two conflicting valid-looking finalization certificates are a critical failure.

The node MUST enter safe mode.

It MUST NOT select one silently.

⸻

31. Network Security

The network layer MUST defend against:

* malformed frames;
* oversized messages;
* connection floods;
* handshake floods;
* slow peers;
* decompression bombs;
* invalid object floods;
* peer-table poisoning;
* eclipse attacks;
* partition attacks;
* replay.

⸻

32. Eclipse Resistance

Mitigations include:

* multiple outbound peers;
* prefix diversity;
* independent bootstrap sources;
* peer rotation;
* persistent known peers;
* certificate verification;
* false-tip rejection.

No mitigation is assumed perfect.

⸻

33. DDoS Resistance

Q1 v0.1 SHALL use:

* rate limits;
* bounded queues;
* cheap validation first;
* message priorities;
* connection limits;
* timeouts;
* peer penalties;
* consensus traffic protection.

Q1 does not claim immunity to large-scale DDoS.

⸻

34. Synchronization Attacks

A malicious peer may provide:

* false highest height;
* invalid blocks;
* invalid certificates;
* corrupt snapshots;
* incomplete history;
* inconsistent participant registry.

Nodes MUST verify every consensus-critical object.

⸻

35. Snapshot Security

Snapshots MUST be tied to a finalized block and state root.

A snapshot server is not trusted.

Invalid snapshots MUST not alter state.

⸻

36. Delay Engine Security

The delay engine SHALL be tested against:

* replay;
* context substitution;
* challenge manipulation;
* proof forgery;
* proof malleability;
* parameter inflation;
* verification DoS;
* shortcut attacks;
* hardware acceleration;
* precomputation.

⸻

37. Sequential Hash Limitation

Sequential Hash Delay v1 is not a formal VDF.

It does not provide cheap full verification.

It may be vulnerable to specialized optimization.

It MUST remain labeled non-production.

⸻

38. Difficulty Attacks

Attackers may attempt to:

* submit excessive difficulty;
* reduce difficulty;
* manipulate adjustment inputs;
* exploit integer arithmetic;
* induce oscillation.

Difficulty parameters MUST come from finalized consensus state and use bounded integer adjustment.

⸻

39. HDD Security

The HDD module SHALL be treated as untrusted.

Threats include:

* SSD substitution;
* RAM-disk substitution;
* virtual disk;
* cache replay;
* remote outsourcing;
* fake metadata;
* fake latency;
* dataset duplication;
* commitment replay;
* proof-size DoS.

⸻

40. Physical Authenticity Limitation

Q1 cannot reliably prove mechanical rotation from ordinary software telemetry.

No security claim may depend on such proof.

⸻

41. HDD Safety

The module MUST prevent:

* arbitrary file overwrite;
* raw disk modification by default;
* uncontrolled writes;
* hidden wear;
* unsafe dataset growth.

⸻

42. AI Observer Security

The AI Observer SHALL be protected against:

* telemetry poisoning;
* prompt injection;
* model replacement;
* alert flooding;
* data exfiltration;
* unauthorized API access;
* model drift;
* over-trust.

⸻

43. AI Authority Limitation

AI output MUST NOT:

* change balances;
* reject valid blocks;
* cast votes;
* penalize funds;
* alter participant weight;
* change consensus.

⸻

44. Economic Security

The economic model SHALL be tested against:

* negative-fee extraction;
* subsidy farming;
* Sybil farming;
* reward loops;
* artificial volume;
* fee manipulation;
* cartel formation;
* reward concentration;
* treasury capture;
* transaction splitting.

⸻

45. Supply Integrity

All issuance, burns, fees, rewards, and treasury movements MUST be reconstructable.

Any supply mismatch is a critical failure.

⸻

46. Treasury Security

Treasury keys and governance introduce centralization risk.

The treasury MUST:

* use transparent addresses;
* expose flows;
* use strong key controls;
* avoid hidden issuance authority.

⸻

47. Software Supply-Chain Security

Q1 SHOULD implement:

* dependency pinning;
* lockfiles;
* checksum verification;
* minimal dependencies;
* security scanning;
* reproducible builds;
* signed release artifacts;
* release provenance;
* protected branches;
* code review.

⸻

48. Dependency Risk

A dependency MUST be evaluated for:

* maintenance status;
* license;
* known vulnerabilities;
* transitive dependencies;
* cryptographic correctness;
* platform behavior.

⸻

49. Build Security

Build systems MUST avoid:

* unpinned downloads;
* hidden generated code;
* secret leakage;
* unsigned artifacts;
* unknown compiler versions.

⸻

50. Genesis Security

The genesis file defines:

* chain identity;
* initial supply;
* participants;
* consensus parameters;
* economic parameters.

Any genesis alteration creates a different network.

Genesis hashes MUST be verified.

⸻

51. Configuration Security

Consensus-critical configuration MUST not be silently changed locally.

Nodes with incompatible configuration MUST refuse participation.

⸻

52. Administrative API Security

Administrative APIs MUST:

* require authentication;
* bind locally by default;
* separate permissions;
* log actions;
* rate limit;
* avoid direct balance mutation.

⸻

53. Secret Management

Secrets MUST NOT appear in:

* logs;
* telemetry;
* crash reports;
* command-line arguments;
* public configuration;
* AI model input.

⸻

54. Database Security

Nodes MUST protect against:

* corruption;
* partial writes;
* rollback;
* unauthorized modification;
* inconsistent indexes.

Critical database inconsistency MUST trigger safe mode or halted participation.

⸻

55. Atomic Finalization

Finalization state changes MUST be atomic.

A crash during commit MUST not produce a partially finalized ledger.

⸻

56. Rollback Attacks

An attacker with filesystem access may attempt to restore older node state.

Validator signing history and finalized checkpoints MUST detect dangerous rollback.

⸻

57. Time Security

Local clocks are untrusted.

They MAY support operational timers but MUST NOT independently determine finality or transaction order.

Clock attacks MUST be simulated.

⸻

58. Logging Security

Logs MUST:

* preserve critical evidence;
* avoid secrets;
* use stable event IDs;
* resist uncontrolled growth;
* support integrity checks where practical.

⸻

59. Telemetry Security

Telemetry is not trusted consensus data.

It may be incomplete, manipulated, delayed, or absent.

⸻

60. Safe Failure

Critical components MUST fail closed.

Examples:

* invalid signature → reject;
* unknown protocol version → reject;
* malformed proof → reject;
* database corruption → stop participation;
* conflicting finality → safe mode;
* AI failure → continue consensus;
* HDD failure in telemetry-only mode → continue consensus.

⸻

61. Safe Mode Triggers

Safe mode MUST activate for:

* conflicting finalization certificates;
* finalized-state mismatch;
* critical database corruption;
* impossible supply mismatch;
* cryptographic provider failure;
* unknown mandatory protocol transition;
* local deterministic disagreement;
* invalid finalized certificate accepted by local state.

⸻

62. Safe Mode Behavior

In safe mode, a node MUST:

* stop producing;
* stop attesting;
* stop finalizing;
* preserve evidence;
* expose read-only diagnostics;
* notify operators;
* prevent automatic destructive recovery.

⸻

63. Incident Evidence

Evidence MAY include:

* signed conflicting proposals;
* signed conflicting attestations;
* invalid certificates;
* state-root mismatches;
* supply mismatches;
* corrupted database proofs;
* logs;
* packet captures where lawful;
* test orchestrator records.

⸻

64. Evidence Integrity

Evidence SHOULD include:

* hashes;
* timestamps as observations;
* source identity;
* protocol context;
* raw signed objects;
* software version;
* chain ID;
* block height;
* round.

⸻

65. Incident Severity

Recommended levels:

INFO
LOW
MEDIUM
HIGH
CRITICAL

Examples:

* malformed peer message → low;
* invalid proposal → medium;
* validator equivocation → high;
* conflicting finality → critical;
* unauthorized issuance → critical.

⸻

66. Security Event Codes

Recommended:

SEC_INVALID_SIGNATURE
SEC_UNAUTHORIZED_ISSUANCE
SEC_DOUBLE_SPEND_ATTEMPT
SEC_PRODUCER_EQUIVOCATION
SEC_VALIDATOR_EQUIVOCATION
SEC_FINALITY_CONFLICT
SEC_SUPPLY_MISMATCH
SEC_STATE_ROOT_MISMATCH
SEC_GENESIS_MISMATCH
SEC_PROTOCOL_DOWNGRADE
SEC_DELAY_PROOF_FORGERY
SEC_HDD_REPLAY
SEC_ECLIPSE_SUSPECTED
SEC_PARTITION_SUSPECTED
SEC_DDOS_SUSPECTED
SEC_SECRET_EXPOSURE
SEC_DATABASE_CORRUPTION
SEC_MODEL_ARTIFACT_MISMATCH
SEC_SUPPLY_CHAIN_RISK
SEC_SAFE_MODE_ENTERED

⸻

67. Threat Modeling Method

Q1 SHOULD maintain a living threat register.

Each threat record SHOULD include:

threat_id
asset
attacker
attack_path
preconditions
impact
likelihood
detectability
current_controls
residual_risk
test_case
owner
status

⸻

68. Security Review Gates

Before each major milestone, Q1 SHOULD perform:

Gate 1 — Local prototype review

Focus:

* basic cryptography;
* state integrity;
* unsafe file operations;
* key leakage;
* invalid-block rejection.

Gate 2 — Multi-node review

Focus:

* consensus;
* networking;
* equivocation;
* synchronization;
* partitions.

Gate 3 — Private testnet review

Focus:

* operational security;
* supply chain;
* deployment;
* monitoring;
* incident response.

Gate 4 — Public testnet review

Focus:

* permissionless exposure;
* DDoS;
* Sybil risk;
* economic attacks;
* disclosure process.

⸻

69. Security Testing Classes

The project MUST include:

* unit tests;
* property tests;
* fuzzing;
* integration tests;
* adversarial multi-node tests;
* load tests;
* fault injection;
* dependency scanning;
* static analysis;
* secret scanning;
* reproducible-build checks;
* manual review.

⸻

70. Fuzzing Targets

Fuzzing SHOULD target:

* transaction decoding;
* block decoding;
* message framing;
* signatures;
* attestations;
* certificates;
* delay proofs;
* HDD evidence;
* snapshots;
* wallet backups;
* API inputs.

⸻

71. Property-Based Security Tests

Properties SHALL include:

* no unauthorized supply creation;
* no negative balances;
* deterministic state transitions;
* one finalized block per height;
* threshold correctness;
* replay rejection;
* domain separation;
* atomic failure;
* fee conservation;
* reward conservation.

⸻

72. Required Security Scenarios

The test suite MUST include:

1. forged transaction signature;
2. altered signed transaction;
3. same-chain replay;
4. cross-chain replay;
5. double spend;
6. unauthorized reward;
7. supply overflow;
8. block with invalid state root;
9. block with invalid delay proof;
10. producer equivocation;
11. validator equivocation;
12. forged finalization certificate;
13. duplicate validator counting;
14. committee mismatch;
15. participant registry mismatch;
16. network partition;
17. eclipse attempt;
18. peer flood;
19. decompression bomb;
20. invalid snapshot;
21. database rollback;
22. node restart after signing;
23. wallet keystore corruption;
24. key leakage scan;
25. fake HDD metadata;
26. HDD replay;
27. RAM-disk substitution;
28. AI telemetry poisoning;
29. prompt injection;
30. economic reward loop;
31. negative-fee attempt;
32. Sybil reward farming;
33. treasury misaccounting;
34. dependency tampering;
35. genesis alteration;
36. protocol downgrade;
37. conflicting finality;
38. safe-mode activation;
39. evidence preservation;
40. recovery from known checkpoint.

⸻

73. Security Acceptance Criteria

Q1 v0.1 security is acceptable for a private testnet only if:

* no unauthorized issuance succeeds;
* double-spend attempts fail;
* invalid signatures fail;
* conflicting votes are detected;
* invalid certificates fail;
* supply accounting remains exact;
* malformed traffic does not crash nodes;
* database corruption is detected;
* key material does not appear in normal logs;
* AI can fail without consensus impact;
* HDD forgery cannot independently finalize blocks;
* safe mode activates on conflicting finality;
* all critical evidence is preserved.

⸻

74. Vulnerability Disclosure

Q1 SHOULD maintain a documented disclosure process.

It SHOULD include:

* reporting channel;
* acknowledgement;
* severity triage;
* embargo expectations;
* remediation workflow;
* credit policy;
* public advisory process.

⸻

75. Severity Triage

Suggested categories:

Critical:
unauthorized issuance, private-key extraction, conflicting finality
High:
remote node compromise, validator signing bypass, major DoS
Medium:
limited DoS, privacy leak, local privilege issue
Low:
minor information leak, non-critical misconfiguration

⸻

76. Patch Process

Critical security patches SHOULD include:

* issue description;
* affected versions;
* exploit conditions;
* mitigation;
* fixed version;
* test coverage;
* migration instructions;
* disclosure timing.

⸻

77. Emergency Changes

Emergency protocol changes are dangerous.

They MUST be:

* documented;
* versioned;
* reviewed;
* activated explicitly;
* limited to testnet until validated.

No hidden emergency key may rewrite balances or finality.

⸻

78. Security Documentation

The repository SHOULD contain:

SECURITY.md
docs/security/THREAT_REGISTER.md
docs/security/INCIDENT_RESPONSE.md
docs/security/DEPENDENCY_POLICY.md
docs/security/RELEASE_SECURITY.md
docs/security/KEY_MANAGEMENT.md

⸻

79. Known Critical Open Risks

Q1 v0.1 has unresolved risks in:

* public Sybil resistance;
* secure participant admission;
* randomness bias;
* formal VDF choice;
* committee capture;
* economic concentration;
* public DDoS resistance;
* hardware acceleration;
* recovery from conflicting finality;
* long-term monetary security.

These risks MUST not be hidden.

⸻

80. Open Decisions

The following remain unresolved:

1. final cryptographic algorithms;
2. formal VDF construction;
3. public participant admission;
4. Sybil resistance;
5. validator collateral;
6. penalty rules;
7. randomness beacon;
8. key rotation;
9. multisignature treasury;
10. public testnet disclosure policy;
11. reproducible-build tooling;
12. dependency approval process;
13. secure release signing;
14. incident response authority;
15. emergency recovery model;
16. safe-mode exit procedure;
17. privacy retention;
18. public peer-discovery security;
19. archive availability security;
20. quantum-migration plan.

All decisions MUST be recorded in OPEN_DECISIONS.md.

⸻

81. Codex Implementation Rules

Codex MUST:

1. fail closed on security-critical errors;
2. separate keys by role;
3. use reviewed cryptographic libraries;
4. use canonical serialization;
5. apply explicit domain separation;
6. enforce checked arithmetic;
7. preserve objective evidence;
8. implement safe mode;
9. isolate administrative APIs;
10. validate all boundary inputs;
11. enforce resource limits;
12. provide stable security event codes;
13. support fuzzing;
14. support malicious test nodes;
15. scan logs for secrets;
16. pin dependencies;
17. verify genesis;
18. test rollback protection;
19. keep AI and HDD outside core trust;
20. document every security assumption.

Codex MUST NOT:

* invent hidden administrative overrides;
* accept unknown protocol versions;
* use one key for multiple sensitive roles;
* trust local timestamps as proof;
* trust device labels;
* trust explorer state;
* ignore supply mismatches;
* continue finalizing after conflicting certificates;
* suppress critical errors silently;
* expose secrets through logs or telemetry;
* claim production security from prototype tests.

⸻

82. Final Security Principle

Q1 security MUST preserve this principle:

Trust is not granted by role, hardware, intelligence, reputation, or proximity.
Trust must be replaced wherever possible by verification.
Where verification is impossible, authority must be limited.
Where uncertainty becomes critical, the system must stop safely.
Where failure occurs, evidence must survive.

Q1 succeeds at the security layer when it can clearly answer:

* what is protected;
* who may attack it;
* what assumptions remain;
* how an attack is detected;
* what the system does next;
* what evidence survives;
* and which risks are still unresolved

without pretending that complexity, physical machinery, artificial intelligence, or decentralization language alone creates security.
