How to Break Q1

13_HOW_TO_BREAK_Q1.md

Project: Q1 Experimental Distributed Ledger
Protocol Version: 0.1
Document Version: 0.1.0
Status: Adversarial Research Draft
Classification: Experimental — Controlled Testing Only — Not for Unauthorized Use

⸻

1. Purpose

This document defines the adversarial testing strategy for Q1 v0.1.

Its purpose is to identify how Q1 can fail before the system is exposed to real financial use.

The project SHALL actively attempt to break:

* ledger integrity;
* transaction authorization;
* consensus safety;
* consensus liveness;
* finality;
* producer selection;
* validator committees;
* delay proofs;
* HDD experiments;
* networking;
* synchronization;
* wallets;
* tokenomics;
* treasury accounting;
* AI observation;
* storage;
* configuration;
* software builds;
* operational processes.

This document is not a promise that every possible attack is known.

It is a requirement that every known assumption be challenged.

⸻

2. Adversarial Principle

Q1 SHALL follow this rule:

A feature is not trusted because it works under honest conditions.
It is trusted only after serious attempts to misuse, bypass, falsify, overload, replay, centralize, and economically exploit it.

The objective is not to defend the original design.

The objective is to determine whether the design deserves to survive.

⸻

3. Testing Authorization

All adversarial activity MUST occur only against:

* local development networks;
* private Q1 testnets;
* systems explicitly owned or authorized for testing;
* simulated environments;
* disposable test accounts;
* controlled infrastructure.

This document MUST NOT be used to attack third-party networks, wallets, services, devices, or users.

⸻

4. Attack Outcome Categories

Each attack SHALL receive one outcome:

BLOCKED
DETECTED
CONTAINED
DEGRADED
SUCCESSFUL
INCONCLUSIVE
NOT_IMPLEMENTED

Definitions:

BLOCKED

The attack fails before causing the intended effect.

DETECTED

The attack may begin, but the system identifies objective or strong evidence.

CONTAINED

The attack affects a limited component but does not spread to protected state.

DEGRADED

Performance or liveness is reduced without violating core safety.

SUCCESSFUL

The attacker achieves the defined objective.

INCONCLUSIVE

Results are insufficient or contradictory.

NOT_IMPLEMENTED

Required test tooling does not yet exist.

⸻

5. Severity Levels

CRITICAL
HIGH
MEDIUM
LOW
INFORMATIONAL

Critical examples

* unauthorized supply creation;
* theft without a valid signature;
* conflicting finalized blocks;
* accepted forged finalization certificate;
* remote extraction of private keys;
* silent deterministic state divergence.

High examples

* sustained consensus halt;
* committee capture;
* reliable validator equivocation;
* practical delay-proof forgery;
* wallet signing deception;
* network-wide eclipse.

⸻

6. Attack Record

Every adversarial test MUST produce:

AttackRecord {
    attack_id
    title
    category
    objective
    protected_asset
    assumptions_tested
    attacker_capabilities
    preconditions
    environment
    attack_steps
    expected_secure_behavior
    actual_behavior
    evidence
    outcome
    severity
    reproducibility
    affected_versions
    mitigation
    regression_test
    residual_risk
}

⸻

7. Rules of Engagement

Attack tests MUST:

1. use isolated test data;
2. preserve logs and packet traces where lawful;
3. record exact software versions;
4. record genesis and configuration hashes;
5. define success before execution;
6. avoid changing multiple variables without documentation;
7. distinguish protocol failure from implementation bug;
8. create regression tests after confirmed failures;
9. stop if testing threatens unrelated infrastructure;
10. report uncertainty honestly.

⸻

8. Primary Attack Surfaces

Q1 SHALL attack the following surfaces:

Ledger
Transactions
Cryptography
Wallet
Consensus
Producer Selection
Committee Selection
Delay Engine
HDD Module
Networking
Synchronization
Storage
Tokenomics
Treasury
AI Observer
APIs
Configuration
Build and Supply Chain
Operations
Governance
Human Factors

⸻

9. Ledger Attacks

Q1-BRK-LED-001 — Unauthorized issuance

Objective: Create units without an authorized genesis, reward, or issuance rule.

Methods:

* malformed reward records;
* integer overflow;
* duplicate reward inclusion;
* fake genesis transaction;
* negative burn;
* state database mutation;
* replayed block reward;
* role duplication.

Expected secure behavior:

* block rejection;
* supply mismatch detection;
* safe mode if finalized state is inconsistent.

Severity:

CRITICAL

⸻

Q1-BRK-LED-002 — Balance underflow or overflow

Attempt:

* maximum integer values;
* subtracting beyond balance;
* reward totals near numeric limits;
* fee arithmetic overflow;
* supply multiplication overflow.

Expected:

* checked arithmetic failure;
* no silent wrapping;
* no partial state mutation.

⸻

Q1-BRK-LED-003 — State-root deception

Attempt to propose a block containing:

* valid transactions but false state root;
* altered account state;
* correct transaction root with incorrect receipts;
* valid parent with unrelated state snapshot.

Expected:

* deterministic recomputation;
* proposal rejection.

⸻

Q1-BRK-LED-004 — Partial state commit

Crash the node during:

* transaction execution;
* receipt persistence;
* canonical index update;
* finalization commit.

Expected:

* atomic rollback or consistent recovery;
* no partially finalized block.

⸻

Q1-BRK-LED-005 — Database rollback

Restore an old ledger database while preserving newer validator or producer keys.

Objective:

* cause replay;
* double-signing;
* stale-state voting;
* duplicate spending.

Expected:

* rollback detection;
* signing protection;
* synchronization before participation.

⸻

10. Transaction Attacks

Q1-BRK-TX-001 — Double spend

Create two validly signed transactions using the same nonce.

Test:

* send to different nodes;
* vary fees;
* delay propagation;
* place each in competing proposals.

Expected:

* at most one can finalize;
* the other becomes stale.

⸻

Q1-BRK-TX-002 — Cross-chain replay

Submit a transaction signed for another chain ID.

Expected:

* deterministic rejection.

⸻

Q1-BRK-TX-003 — Cross-version replay

Reinterpret an old transaction encoding under a newer version.

Expected:

* unsupported or incompatible version rejection.

⸻

Q1-BRK-TX-004 — Signature-field substitution

Alter after signing:

* recipient;
* amount;
* fee limit;
* nonce;
* validity window;
* memo hash;
* chain ID.

Expected:

* signature failure.

⸻

Q1-BRK-TX-005 — Serialization ambiguity

Try:

* duplicate fields;
* alternative integer encodings;
* reordered map fields;
* trailing bytes;
* malformed optional values;
* visually identical Unicode values.

Expected:

* non-canonical encoding rejection.

⸻

Q1-BRK-TX-006 — Future nonce exhaustion

Submit many transactions far above the current nonce.

Objective:

* consume mempool memory;
* block legitimate transactions;
* exploit wallet nonce logic.

Expected:

* bounded nonce gap;
* per-account limits;
* eviction.

⸻

Q1-BRK-TX-007 — Replacement abuse

Attempt:

* underpriced replacement;
* rapid replacement loops;
* conflicting replacements sent to different peers;
* replacement after finalization.

Expected:

* deterministic local policy;
* no finalized replacement;
* bounded resource use.

⸻

Q1-BRK-TX-008 — Expiration manipulation

Attempt inclusion:

* before valid_from_height;
* after valid_until_height;
* during node clock skew.

Expected:

* height-based deterministic rejection.

⸻

11. Wallet Attacks

Q1-BRK-WAL-001 — Clipboard substitution

Replace a copied recipient address before confirmation.

Expected:

* full address redisplay;
* checksum validation;
* user confirmation.

⸻

Q1-BRK-WAL-002 — Wrong-network deception

Connect the wallet to a node on another genesis or chain.

Expected:

* chain and genesis mismatch warning;
* signing or submission blocked.

⸻

Q1-BRK-WAL-003 — Fee inflation by malicious node

A node reports an excessive fee estimate.

Expected:

* visible maximum fee;
* optional multi-node comparison;
* no silent signing.

⸻

Q1-BRK-WAL-004 — False finality report

A malicious node reports a pending transaction as finalized.

Expected:

* compatibility checks;
* additional-node verification where enabled;
* no blind trust in explorer output.

⸻

Q1-BRK-WAL-005 — Keystore corruption

Modify:

* ciphertext;
* authentication tag;
* algorithm metadata;
* public key;
* address.

Expected:

* corruption detection;
* no automatic overwrite.

⸻

Q1-BRK-WAL-006 — Password leakage

Inspect:

* command-line arguments;
* shell history;
* logs;
* process list;
* crash report.

Expected:

* no password exposure.

⸻

Q1-BRK-WAL-007 — Offline package tampering

Modify an unsigned transaction package during air-gap transfer.

Expected:

* checksum mismatch or changed preview;
* offline signer refusal.

⸻

Q1-BRK-WAL-008 — Backup substitution

Replace a backup with another valid but unrelated encrypted wallet.

Expected:

* wallet identity and address confirmation;
* user-visible restore details.

⸻

12. Cryptographic Attacks

Q1-BRK-CRY-001 — Malformed public keys

Submit:

* invalid encodings;
* oversized keys;
* unsupported algorithms;
* boundary values.

Expected:

* safe rejection;
* no crash.

⸻

Q1-BRK-CRY-002 — Signature malleability

Determine whether one authorization can produce multiple accepted identifiers or signatures.

Expected:

* canonical signature rules;
* deterministic transaction identity policy.

⸻

Q1-BRK-CRY-003 — Domain confusion

Attempt to reuse:

* transaction signature as attestation;
* attestation signature as proposal;
* node identity signature as validator vote.

Expected:

* domain-separated verification failure.

⸻

Q1-BRK-CRY-004 — Randomness failure

Replace secure randomness with predictable output during key generation.

Expected:

* initialization failure or detectable test mode;
* no silent fallback.

⸻

Q1-BRK-CRY-005 — Algorithm downgrade

Advertise or submit weaker or obsolete cryptographic versions.

Expected:

* whitelist enforcement;
* unsupported version rejection.

⸻

13. Producer Selection Attacks

Q1-BRK-SEL-001 — Selection inconsistency

Provide identical consensus context to multiple nodes and test whether they derive different candidate lists.

Expected:

* identical output.

Severity if successful:

CRITICAL

⸻

Q1-BRK-SEL-002 — Grinding

Attempt to influence the next selection seed by:

* changing transaction order;
* withholding candidate blocks;
* selecting among multiple valid proposals;
* altering optional fields;
* manipulating delay output.

Expected:

* bias measured;
* exploitable influence documented.

⸻

Q1-BRK-SEL-003 — Repeated producer domination

Simulate one participant with superior hardware, uptime, or many identities.

Measure:

* block share;
* cooldown effectiveness;
* reward concentration;
* long-term selection variance.

⸻

Q1-BRK-SEL-004 — Registry duplication

Create several participant records controlled by one operator.

Expected:

* expose current Sybil weakness;
* quantify influence.

Q1 v0.1 MAY fail this test due to the permissioned laboratory registry.

Failure MUST be documented rather than hidden.

⸻

14. Committee Attacks

Q1-BRK-COM-001 — Committee capture

Attempt to control at least one-third or two-thirds of selected committee weight.

Measure:

* probability;
* identity-cost assumptions;
* operator concentration;
* repeated capture.

⸻

Q1-BRK-COM-002 — Duplicate committee identity

Attempt to count one validator more than once through:

* duplicate registry records;
* alternative keys;
* malformed membership proofs;
* aggregate duplication.

Expected:

* one count per committee identity.

⸻

Q1-BRK-COM-003 — Producer-validator conflict

Attempt to let the active producer vote for its own block where exclusion is required.

Expected:

* excluded or zero-weight vote.

⸻

Q1-BRK-COM-004 — Stale committee

Use committee membership from:

* previous height;
* previous round;
* previous epoch;
* another chain.

Expected:

* rejection.

⸻

15. Consensus Attacks

Q1-BRK-CON-001 — Invalid proposal

Propose a block with:

* invalid transaction;
* incorrect roots;
* unauthorized reward;
* invalid delay proof;
* wrong parent;
* unsupported version.

Expected:

* no honest attestation.

⸻

Q1-BRK-CON-002 — Producer equivocation

One producer signs two blocks for the same height and round.

Expected:

* objective evidence;
* no silent reward;
* alerts and penalty eligibility.

⸻

Q1-BRK-CON-003 — Validator double vote

One validator signs two conflicting proposals at the same height and round.

Expected:

* both signatures preserved;
* duplicate weight excluded;
* equivocation evidence generated.

⸻

Q1-BRK-CON-004 — Threshold miscalculation

Test committee sizes and weights around boundary values.

Examples:

* 3 members;
* 4 members;
* 5 members;
* uneven weights;
* maximum integer values.

Expected:

* exact integer threshold;
* no rounding ambiguity.

⸻

Q1-BRK-CON-005 — Forged finalization certificate

Attempt:

* invalid signatures;
* duplicate validators;
* wrong committee;
* insufficient weight;
* wrong block hash;
* wrong parent;
* wrong round.

Expected:

* rejection.

⸻

Q1-BRK-CON-006 — Conflicting finality

Inject two conflicting, structurally valid-looking certificates.

Expected:

* safe mode;
* no automatic chain choice.

⸻

Q1-BRK-CON-007 — Early fallback

Fallback producer publishes before its assigned window.

Expected:

* proposal rejection.

⸻

Q1-BRK-CON-008 — Late primary proposal

Primary proposal arrives after fallback activation.

Expected:

* deterministic handling;
* no local-arrival ambiguity that creates conflicting finality.

⸻

Q1-BRK-CON-009 — Endless round change

Cause:

* producer failures;
* withheld attestations;
* delayed timeout messages.

Measure:

* liveness degradation;
* resource consumption;
* safe timeout behavior.

⸻

Q1-BRK-CON-010 — Restart double-signing

Restart a validator after it signs, then present another proposal.

Expected:

* persistent signing record prevents conflicting vote.

⸻

16. Delay Engine Attacks

Q1-BRK-DLY-001 — Proof replay

Reuse delay proof across:

* height;
* round;
* producer;
* candidate index;
* chain;
* difficulty;
* engine version.

Expected:

* rejection.

⸻

Q1-BRK-DLY-002 — Shortcut search

Attempt optimized execution using:

* vectorization;
* GPU;
* FPGA;
* ASIC assumptions;
* batch computation;
* precomputation;
* memory-time tradeoffs.

Measure:

* speedup;
* energy efficiency;
* fairness impact.

⸻

Q1-BRK-DLY-003 — Parameter explosion

Submit:

* excessive iteration count;
* oversized proof;
* malformed checkpoints;
* memory-intensive parameters.

Expected:

* bounded rejection before resource exhaustion.

⸻

Q1-BRK-DLY-004 — Verification DoS

Flood validators with expensive invalid proofs.

Expected:

* cheap prechecks;
* bounded queues;
* rate limits;
* consensus traffic protection.

⸻

Q1-BRK-DLY-005 — Checkpoint forgery

Modify:

* checkpoint index;
* checkpoint value;
* commitment root;
* interval.

Expected:

* verification failure.

⸻

Q1-BRK-DLY-006 — Artificial sleep

Replace sequential work with wall-clock waiting.

Expected:

* no valid mathematical proof.

⸻

Q1-BRK-DLY-007 — Previous producer bias

Measure whether the previous producer can influence a future challenge through block contents or withholding.

⸻

17. HDD Attacks

Q1-BRK-HDD-001 — SSD substitution

Run the HDD workload entirely on SSD.

Objective:

* determine whether verification can distinguish it.

Expected research result:

* likely indistinguishable without trusted hardware;
* limitation explicitly recorded.

⸻

Q1-BRK-HDD-002 — RAM-disk substitution

Load the full dataset into memory.

Measure:

* response speed;
* energy;
* commitment validity;
* feasibility.

⸻

Q1-BRK-HDD-003 — Cache attack

Preload selected or entire dataset into cache.

Test:

* warm cache;
* cold cache;
* repeated challenge;
* dataset smaller than RAM.

⸻

Q1-BRK-HDD-004 — Fake metadata

Report false:

* model;
* serial number;
* RPM;
* capacity;
* rotational status;
* temperature;
* energy.

Expected:

* telemetry may be fooled;
* consensus remains unaffected.

⸻

Q1-BRK-HDD-005 — Commitment replay

Reuse commitment across another round or producer.

Expected:

* challenge binding rejects it.

⸻

Q1-BRK-HDD-006 — Remote outsourcing

Execute HDD challenge on another host or cloud service.

Objective:

* measure whether location or ownership can be proven.

Expected:

* likely not provable;
* economic impact documented.

⸻

Q1-BRK-HDD-007 — Dataset duplication

Copy one dataset across many identities or devices.

Measure:

* reward or eligibility amplification;
* duplicate-dataset detection limits.

⸻

Q1-BRK-HDD-008 — Virtual disk swarm

Create many virtual disks and claim many devices.

Expected:

* expose why device count cannot equal participant count.

⸻

Q1-BRK-HDD-009 — Unsafe path escape

Attempt directory traversal, symlink abuse, or path substitution to write outside the Q1 test directory.

Expected:

* strict path isolation;
* no arbitrary file modification.

Severity if successful:

CRITICAL

⸻

Q1-BRK-HDD-010 — Wear amplification

Cause repeated or oversized writes through manipulated configuration.

Expected:

* hard limits;
* writes disabled by default;
* safety warnings.

⸻

18. Networking Attacks

Q1-BRK-NET-001 — Eclipse attack

Attempt to control all peers of one target node.

Methods:

* Sybil peer records;
* prefix flooding;
* bootstrap poisoning;
* repeated honest-peer disconnection;
* connection-slot exhaustion.

Measure:

* time to isolation;
* false chain-view acceptance;
* recovery.

⸻

Q1-BRK-NET-002 — False highest chain

Advertise an extremely high finalized height without valid certificates.

Expected:

* no trust in height claim;
* certificate-based verification.

⸻

Q1-BRK-NET-003 — Handshake flood

Open many incomplete or invalid handshakes.

Expected:

* timeout;
* bounded resources;
* rate limiting.

⸻

Q1-BRK-NET-004 — Decompression bomb

Send highly compressed data with extreme expansion.

Expected:

* ratio and size limits;
* safe disconnection.

⸻

Q1-BRK-NET-005 — Slow-read and slow-write

Maintain connections while transferring very slowly.

Expected:

* timeouts;
* slot recovery.

⸻

Q1-BRK-NET-006 — Consensus starvation

Flood the node with low-priority transaction traffic.

Expected:

* consensus messages retain priority.

⸻

Q1-BRK-NET-007 — Duplicate flood

Repeatedly send known transaction, block, and attestation IDs.

Expected:

* known-object cache;
* no repeated expensive processing.

⸻

Q1-BRK-NET-008 — Peer-record poisoning

Share large numbers of unreachable or attacker-controlled peers.

Expected:

* bounded peer store;
* reachability testing;
* diversity policies.

⸻

Q1-BRK-NET-009 — Partition

Separate the testnet into multiple groups.

Test:

* minority partition;
* majority partition;
* committee split;
* reconnection;
* stale proposals.

Expected:

* safety preserved;
* liveness may degrade;
* certificate-based recovery.

⸻

19. Synchronization Attacks

Q1-BRK-SYN-001 — Malicious snapshot

Provide a snapshot with:

* wrong state root;
* valid metadata but altered chunks;
* wrong finalized block;
* wrong participant set.

Expected:

* rejection.

⸻

Q1-BRK-SYN-002 — Incomplete history

Serve a valid prefix while omitting later blocks or certificates.

Expected:

* cross-checking with other peers;
* no false finality.

⸻

Q1-BRK-SYN-003 — Alternate history

Provide a chain with valid-looking blocks but insufficient or forged certificates.

Expected:

* rejection.

⸻

Q1-BRK-SYN-004 — Resource exhaustion

Provide many small ranges, inconsistent responses, or huge snapshot manifests.

Expected:

* bounded requests;
* peer penalties;
* retry with other peers.

⸻

20. Economic Attacks

Q1-BRK-ECO-001 — Negative-fee extraction

Attempt to configure or exploit fees below zero.

Expected:

* protocol prohibition;
* transaction rejection.

⸻

Q1-BRK-ECO-002 — Self-transaction farming

Create circular or self-controlled transfers to earn more than the paid fee.

Expected:

* no net-positive reward loop.

⸻

Q1-BRK-ECO-003 — Transaction splitting

Split one transfer into many smaller transfers to reduce total fee.

Expected:

* no economic advantage where each transaction consumes resources.

⸻

Q1-BRK-ECO-004 — Congestion manufacturing

Flood blocks to raise fees for others.

Measure:

* attacker cost;
* fee response;
* persistence;
* network revenue;
* user harm.

⸻

Q1-BRK-ECO-005 — Sybil reward farming

Create many identities to multiply:

* validator rewards;
* producer opportunities;
* subsidies;
* HDD rewards;
* faucet distributions.

Expected:

* expose unsolved identity assumptions;
* prevent fixed per-identity giveaways without protection.

⸻

Q1-BRK-ECO-006 — Multi-role concentration

One operator controls:

* producer;
* delay executor;
* several validators;
* treasury recipient;
* archive services.

Measure:

* reward concentration;
* censorship ability;
* finality influence.

⸻

Q1-BRK-ECO-007 — Treasury capture

Attempt:

* hidden allocation;
* unauthorized withdrawal;
* compromised treasury key;
* reward remainder manipulation.

Expected:

* transparent accounting;
* no hidden issuance.

⸻

Q1-BRK-ECO-008 — Rounding theft

Exploit integer reward remainders over many blocks.

Expected:

* deterministic and publicly defined remainder destination.

⸻

Q1-BRK-ECO-009 — Reward without finality

Attempt to spend reward from:

* proposed block;
* attested but unfinalized block;
* rejected block;
* stale round.

Expected:

* impossible before finality and maturity.

⸻

21. AI Observer Attacks

Q1-BRK-AI-001 — Telemetry poisoning

Submit coordinated fake metrics to trigger false alarms.

Expected:

* evidence-class awareness;
* source tracking;
* no consensus effect.

⸻

Q1-BRK-AI-002 — Prompt injection

Place malicious instructions inside logs or metadata.

Expected:

* treated as untrusted data;
* no tool or privilege escalation.

⸻

Q1-BRK-AI-003 — Alert flood

Generate many low-quality anomalies.

Expected:

* deduplication;
* rate limiting;
* prioritization.

⸻

Q1-BRK-AI-004 — Model replacement

Replace model artifacts with altered files.

Expected:

* artifact-hash mismatch;
* model loading failure or quarantine.

⸻

Q1-BRK-AI-005 — False accusation

Create behavior that appears malicious but is benign.

Expected:

* uncertainty language;
* no automatic punishment;
* human review.

⸻

Q1-BRK-AI-006 — Observer outage

Stop the observer during an attack.

Expected:

* consensus unaffected;
* detection capability degraded visibly.

⸻

22. API Attacks

Q1-BRK-API-001 — Unauthorized administration

Attempt to access:

* shutdown;
* peer ban;
* configuration reload;
* snapshot;
* producer enablement.

Expected:

* authentication failure.

⸻

Q1-BRK-API-002 — Direct balance mutation

Search for any endpoint or hidden command capable of editing balances.

Expected:

* no such path.

Severity if successful:

CRITICAL

⸻

Q1-BRK-API-003 — Input injection

Fuzz:

* JSON;
* path parameters;
* query strings;
* headers;
* oversized bodies;
* malformed numbers.

Expected:

* bounded safe rejection.

⸻

Q1-BRK-API-004 — Public/admin boundary bypass

Attempt to route public P2P or API traffic into administrative handlers.

Expected:

* strict separation.

⸻

23. Storage Attacks

Q1-BRK-STO-001 — Ledger file modification

Alter local block or state records.

Expected:

* integrity mismatch;
* resynchronization or safe mode.

⸻

Q1-BRK-STO-002 — Index corruption

Modify transaction index while preserving canonical blocks.

Expected:

* index rebuild;
* no consensus-state change.

⸻

Q1-BRK-STO-003 — Disk-full condition

Exhaust storage during:

* block commit;
* telemetry write;
* snapshot creation;
* HDD dataset generation.

Expected:

* safe failure;
* no partial finalization.

⸻

Q1-BRK-STO-004 — Symlink and path attacks

Redirect storage, wallet, or HDD paths.

Expected:

* path validation;
* no secret or system-file overwrite.

⸻

24. Configuration Attacks

Q1-BRK-CFG-001 — Local consensus override

Change local:

* committee size;
* fee rules;
* difficulty;
* reward shares;
* block limits.

Expected:

* mismatch rejection;
* node cannot participate incompatibly.

⸻

Q1-BRK-CFG-002 — Genesis substitution

Replace genesis with a modified file using the same network name.

Expected:

* genesis hash mismatch;
* network separation.

⸻

Q1-BRK-CFG-003 — Unsafe defaults

Test whether a fresh install exposes:

* admin API publicly;
* plaintext keys;
* unrestricted HDD writes;
* unlimited peers;
* test mode on public interface.

Expected:

* secure defaults.

⸻

25. Build and Supply-Chain Attacks

Q1-BRK-SUP-001 — Dependency compromise

Replace a pinned dependency with altered code.

Expected:

* checksum or lockfile mismatch;
* build failure or alert.

⸻

Q1-BRK-SUP-002 — Malicious build artifact

Create a binary that differs from the documented source commit.

Expected:

* reproducible-build mismatch;
* unsigned release warning.

⸻

Q1-BRK-SUP-003 — Secret inclusion

Scan build artifacts and source history for:

* private keys;
* passwords;
* API tokens;
* recovery seeds.

Expected:

* automated secret detection.

⸻

Q1-BRK-SUP-004 — Model supply-chain attack

Alter AI model or feature artifact.

Expected:

* artifact verification failure.

⸻

26. Operational Attacks

Q1-BRK-OPS-001 — Key theft

Copy validator or producer key files.

Test impact:

* unauthorized signing;
* equivocation;
* role impersonation.

Mitigation tests:

* file permissions;
* encrypted storage;
* rotation;
* evidence detection.

⸻

Q1-BRK-OPS-002 — Operator misconfiguration

Set:

* wrong chain;
* wrong ports;
* exposed admin API;
* excessive HDD writes;
* weak wallet password.

Expected:

* validation and warnings.

⸻

Q1-BRK-OPS-003 — Simultaneous node restart

Restart a large fraction of the testnet.

Measure:

* recovery;
* liveness;
* sync load;
* signing protection.

⸻

Q1-BRK-OPS-004 — Correlated infrastructure failure

Shut down all nodes on one provider, host, or network range.

Measure:

* actual operator diversity;
* recovery.

⸻

27. Human-Factor Attacks

Q1-BRK-HUM-001 — Phishing wallet

Present a false wallet interface requesting seed words.

Expected project response:

* clear documentation;
* no normal workflow requiring seed submission.

⸻

Q1-BRK-HUM-002 — Misleading transaction summary

Hide or shorten recipient, fee, or network.

Expected:

* complete preview;
* explicit confirmation.

⸻

Q1-BRK-HUM-003 — Fake Q1 update

Distribute an unofficial binary.

Expected future controls:

* signed releases;
* checksums;
* clear update process.

⸻

Q1-BRK-HUM-004 — False profit claim

Represent testnet rewards as guaranteed income.

Expected:

* explicit experimental warnings;
* no investment promises.

⸻

28. Composite Attacks

Q1 MUST test attacks combining several weak signals.

Composite A — Producer cartel

* many identities;
* superior delay hardware;
* shared network;
* transaction censorship;
* reward concentration.

Composite B — Validator eclipse

* isolate validators;
* provide delayed proposals;
* suppress honest attestations;
* induce conflicting local views.

Composite C — HDD reward farming

* virtual disks;
* duplicated datasets;
* fake telemetry;
* Sybil identities;
* remote execution.

Composite D — Wallet deception

* malicious node;
* wrong fee estimate;
* false finality;
* clipboard substitution.

Composite E — Supply-chain takeover

* altered dependency;
* malicious node binary;
* stolen validator keys;
* modified AI model.

Composite tests are required because real attacks rarely remain inside one module.

⸻

29. Red-Team Profiles

The test orchestrator SHOULD support:

MALICIOUS_USER
MALICIOUS_PRODUCER
MALICIOUS_VALIDATOR
SYBIL_OPERATOR
ECLIPSE_OPERATOR
FEE_MANIPULATOR
HDD_FRAUD_OPERATOR
MALICIOUS_SYNC_PEER
COMPROMISED_NODE_OPERATOR
TELEMETRY_POISONER

Each profile SHALL have configurable capabilities.

⸻

30. Fault Injection

The orchestrator MUST support:

* process crash;
* network delay;
* packet loss;
* message duplication;
* message reordering;
* disk failure;
* disk full;
* clock skew;
* CPU throttling;
* memory pressure;
* invalid proofs;
* key replacement;
* corrupted databases;
* disabled telemetry;
* AI outage.

⸻

31. Attack Test Isolation

Dangerous tests SHOULD run in:

* containers;
* virtual machines;
* dedicated test directories;
* non-production credentials;
* isolated networks;
* disposable genesis configurations.

Raw disk and destructive storage tests require explicit opt-in.

⸻

32. Evidence Collection

For every serious test, collect where applicable:

* node logs;
* consensus events;
* signed objects;
* packet traces;
* database snapshots;
* configuration;
* genesis hash;
* software commit;
* dependency lockfile;
* host metrics;
* AI alerts;
* attack-controller logs.

⸻

33. Reproducibility

A confirmed vulnerability MUST have:

* minimal reproduction;
* deterministic test where possible;
* exact configuration;
* expected and actual behavior;
* affected version range;
* regression test.

⸻

34. Regression Requirement

Every fixed security failure MUST produce an automated regression test.

A critical vulnerability fix MUST NOT be considered complete without a test preventing recurrence where technically possible.

⸻

35. Stop Conditions

An adversarial test MUST stop when:

* it affects unauthorized systems;
* it risks permanent loss of non-test data;
* it leaks real secrets;
* it exceeds agreed resource limits;
* the environment can no longer preserve evidence;
* the safety owner terminates the test.

⸻

36. Attack Prioritization

Tests SHOULD be prioritized in this order:

1. unauthorized issuance;
2. conflicting finality;
3. signature and key failures;
4. supply and state divergence;
5. consensus threshold bypass;
6. wallet theft or deception;
7. remote node compromise;
8. delay-proof forgery;
9. network isolation;
10. economic extraction;
11. HDD fraud;
12. AI manipulation;
13. performance degradation.

⸻

37. Minimum Adversarial Milestone

The first adversarial milestone is complete when Q1 has tested:

1. double spending;
2. unauthorized reward creation;
3. state-root mismatch;
4. producer equivocation;
5. validator double-voting;
6. forged finalization certificate;
7. fallback timing abuse;
8. delay-proof replay;
9. excessive proof parameters;
10. HDD replay;
11. SSD and RAM substitution;
12. handshake flood;
13. invalid transaction flood;
14. network partition;
15. eclipse simulation;
16. malicious snapshot;
17. wallet network mismatch;
18. false finality report;
19. negative-fee attempt;
20. Sybil reward farming;
21. AI telemetry poisoning;
22. database corruption;
23. genesis substitution;
24. safe-mode activation;
25. evidence preservation.

⸻

38. Private Testnet Red-Team Gate

Before a private distributed testnet is considered stable, Q1 MUST demonstrate:

* no unauthorized issuance under tested attacks;
* no conflicting finality below the assumed Byzantine threshold;
* objective equivocation detection;
* safe handling of malformed proofs;
* successful partition recovery;
* bounded resource use during floods;
* no plaintext key leakage in standard operation;
* exact supply accounting;
* HDD fraud cannot independently affect finality;
* AI manipulation cannot affect consensus;
* confirmed vulnerabilities produce regression tests.

⸻

39. Public Testnet Prerequisites

A public experimental testnet MUST NOT begin until:

* critical local attack cases pass;
* private testnet survives sustained adversarial testing;
* disclosure process exists;
* signed or reproducible releases exist;
* default configurations are hardened;
* rate limits are validated;
* key management is reviewed;
* Sybil limitations are publicly documented;
* economic risks are disclosed;
* incident response is defined.

⸻

40. Failure Interpretation

A successful attack is not automatically the end of Q1.

It may indicate:

* implementation defect;
* invalid protocol assumption;
* poor parameter choice;
* missing operational control;
* unnecessary module;
* unsolved research problem.

The project SHALL respond by:

1. preserving evidence;
2. reproducing the issue;
3. classifying its root cause;
4. deciding whether to fix, isolate, redesign, or remove the component;
5. updating specifications;
6. adding regression tests.

⸻

41. Component Removal Rule

A component SHOULD be removed if repeated testing shows that it:

* adds no measurable security;
* introduces severe attack surface;
* cannot be independently verified;
* causes harmful centralization;
* creates unacceptable energy or hardware waste;
* cannot fail safely.

HDD and AI remain explicitly subject to this rule.

⸻

42. Known Expected Failures

Q1 v0.1 is expected to have unresolved weaknesses in:

* public Sybil resistance;
* permissionless participant admission;
* physical HDD authenticity;
* specialized delay hardware advantage;
* public DDoS resistance;
* economic market behavior;
* recovery from true conflicting finality.

These are research targets.

They MUST not be falsely reported as solved.

⸻

43. Open Adversarial Questions

1. Can a producer bias future selection through block-content grinding?
2. Can committees be captured cheaply through many identities?
3. Can specialized hardware dominate sequential delay?
4. Can fallback timing create honest conflicting views?
5. Can a malicious node exploit fee-estimation trust?
6. Can transaction spam starve proof verification?
7. Can HDD evidence offer any property unavailable through faster storage?
8. Can one operator cheaply multiply every rewarded role?
9. Can a testnet recovery mechanism become hidden central authority?
10. Can AI alerts cause operators to harm an otherwise healthy network?
11. Can a malicious dependency alter canonical serialization?
12. Can database rollback cause accidental equivocation?
13. Can snapshot distribution become a central trust point?
14. Can reward variance force participants into centralized pools?
15. Can network diversity be measured without creating surveillance?

All unresolved questions MUST be tracked in OPEN_DECISIONS.md or the threat register.

⸻

44. Codex Implementation Rules

Codex MUST:

1. provide malicious-node modes;
2. provide deterministic attack seeds;
3. support network partition and delay injection;
4. support malformed object generation;
5. support consensus equivocation tests;
6. support HDD simulation and fraud modes;
7. support fee and reward attack simulations;
8. preserve signed evidence;
9. generate structured attack reports;
10. create regression tests for confirmed defects;
11. isolate destructive tests;
12. require explicit opt-in for raw disk operations;
13. expose test outcome and severity;
14. distinguish protocol failure from local-policy failure;
15. keep attack tools out of normal production execution paths.

Codex MUST NOT:

* run destructive attacks by default;
* target third-party systems;
* conceal successful attacks;
* automatically downgrade critical severity;
* delete evidence after failure;
* treat one passing test as proof of security;
* claim that simulated Sybil resistance equals permissionless security;
* allow attack tooling to access real wallet secrets.

⸻

45. Final Adversarial Principle

Q1 MUST preserve this principle:

We do not ask whether Q1 works when everyone behaves correctly.
We ask what happens when users lie, producers cheat, validators collude, networks split, disks fail, models hallucinate, keys are stolen, incentives are exploited, and the code receives inputs its designers never expected.

A strong result is not a network that never fails.

A strong result is a network that:

* rejects what it can verify as false;
* limits what it cannot fully trust;
* stops safely when finality becomes uncertain;
* preserves evidence;
* exposes its remaining weaknesses;
* and improves after every successful attack.

Q1 will deserve release only after its creators have tried, repeatedly and seriously, to make it fail.