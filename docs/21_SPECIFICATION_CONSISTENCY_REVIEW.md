Q1 Specification Consistency Review

Review Version: 0.1.0
Date: 2026-07-24
Status: Draft for Human Review
Scope: Repository documentation only; no protocol behavior is approved by this review

⸻

1. Executive Finding

Q1 has a coherent research philosophy, strong separation of consensus from AI and HDD experimentation, and unusually explicit safety goals. It is not yet ready for protocol-core implementation.

The principal blockers are:

1. requirement identifiers are not globally unique;
2. the decision register does not contain the decisions that specifications require it to contain;
3. cryptography and canonical serialization are unselected;
4. private-testnet participant admission, producer selection, committee selection, randomness, round change, fork choice, and safe-mode exit are incomplete;
5. fee, reward, issuance, remainder, and genesis-supply rules are incomplete;
6. several consensus data structures lack an approved byte-level schema;
7. referenced engineering and security documents do not yet exist;
8. the repository is not currently a Git worktree, despite Git checkpoints being mandatory under YOS.

Conclusion: proceed with Horizon 0 and documentation/ADR work only. Do not begin M1 or consensus-critical code.

⸻

2. Review Method and Authority

Documents were reviewed using the repository's stated authority:

* human instruction and AGENTS.md;
* YOS collaboration documents;
* PROJECT.md;
* docs/00 through docs/20;
* OPEN_DECISIONS.md and IMPLEMENTATION_NOTES.md.

Facts, proposals, and unresolved decisions are separated below. Recommendations do not approve protocol rules.

⸻

3. Confirmed Strengths

* The charter consistently defines Q1 as experimental research, not a production or investment product.
* Consensus validity is explicitly isolated from AI output.
* HDD participation is designed as removable and must not be treated as proof of physical hardware.
* Integer monetary accounting and deterministic execution are repeated across the system, ledger, tokenomics, API, and build documents.
* Safe mode, conflicting-finality handling, evidence preservation, and adversarial testing are first-class concepts.
* The roadmap uses explicit gates and does not authorize mainnet.
* The architecture favors replaceable interfaces for cryptography, delay, storage, networking, HDD, and observation.

⸻

4. Inconsistencies

4.1 Requirement-ID collisions — BLOCKING

A mechanical scan found 63 identifiers defined more than once. The same identifiers define different requirements in different normative documents. Examples:

* Q1-CON-001: block-state lifecycle in 02_SYSTEM_REQUIREMENTS.md; consensus height in 05_CONSENSUS.md.
* Q1-DLY-001: challenge derived from finalized data in 02_SYSTEM_REQUIREMENTS.md; full canonical challenge construction in 06_DELAY_ENGINE.md.
* Q1-HDD-001: plugin isolation in 02_SYSTEM_REQUIREMENTS.md; deterministic dataset generation in 07_HDD_LAB_MODULE.md.
* Q1-NET-001: authenticated/signed communications in 02_SYSTEM_REQUIREMENTS.md; key separation in 08_NODE_AND_NETWORKING.md.
* Q1-WAL-001: CLI-first wallet in 02_SYSTEM_REQUIREMENTS.md; user control of spending keys in 09_WALLET.md.
* Q1-AI-001: observer outside consensus in 02_SYSTEM_REQUIREMENTS.md; AI non-authority in 11_AI_OBSERVER.md.
* Q1-SEC-001: reject unauthorized balance creation in 02_SYSTEM_REQUIREMENTS.md; cryptographic security assumption in 12_SECURITY_MODEL.md.
* Q1-API-001 and Q1-TST-001 are likewise reused with different meanings.

Impact: code comments, tests, audit findings, and traceability cannot refer to an identifier unambiguously.

Required resolution: approve one ID ownership scheme, migrate IDs, and publish an old-to-new mapping before creating the traceability baseline.

4.2 Genesis allocation equality

04_LEDGER_AND_TRANSACTIONS.md requires allocation totals not to exceed configured genesis supply. 10_TOKENOMICS.md requires them to equal configured genesis supply.

Impact: the state root and supply invariant differ when an unallocated reserve exists.

Required decision: either require equality, or explicitly define where unallocated supply exists and whether it is issued, circulating, or inaccessible.

4.3 Normative strength for monetary JSON

16_API_SPECIFICATION.md says monetary JSON values MUST be decimal strings. 09_WALLET.md and 04_LEDGER_AND_TRANSACTIONS.md use SHOULD for closely related behavior.

Recommended interpretation for review, not approval: the API's MUST should govern wire responses; wallet internals need a separate rule. Text should be normalized so there is no apparent exception.

4.4 Open-decision policy versus current register

Every major component specification says its unresolved decisions MUST be recorded in OPEN_DECISIONS.md. That file currently contains a generic template and examples, not the project register.

Impact: hidden dependency chains and no reliable milestone gate.

4.5 Repository-path drift

* The actual YOS directory is YOS_v1.0, while PROJECT.md and other examples refer to YOS/.
* The actual ADR directory is ADR/, while build instructions require docs/adr/.
* Plain text and Plain text.md contain competing directory sketches and have no declared authority.

Required decision: select canonical paths, migrate deliberately, and remove or archive scratch artifacts after confirmation.

4.6 Version metadata drift

YOS_v1.0 contains documents labeled v0.2 and v0.3 while the folder and top-level framework claim v1.0. This may be legitimate document history, but no versioning rule explains the relationship.

Impact: agents cannot determine whether lower-version files are current.

⸻

5. Missing Definitions and Ambiguities

Consensus-critical:

* canonical serialization and rejection of non-canonical encodings;
* hash, signature, address, and key encodings;
* whether transaction IDs include signature bytes;
* state, transaction, and receipt commitment structures;
* participant registry admission/update/removal and operator identity;
* producer weighting, deterministic candidate order, cooldown, and fallback;
* committee selection, randomness/VRF, membership snapshot, and committee exclusion;
* exact round timing, timeout evidence, round-change certificate, and late-message rules;
* fork-choice tie-breaker before finality;
* validator signing/locking behavior across rounds;
* safe-mode entry persistence and approved exit/recovery procedure;
* formal delay construction versus the explicitly non-production sequential-hash placeholder;
* exact relationship between delay work and producer windows.

Economic:

* genesis allocation invariant;
* issuance model and maximum-supply semantics;
* fee formula, minimum fee, congestion adjustment, and fee-limit settlement;
* reward shares, validator division, and integer remainder destination;
* burn policy, reward maturity, penalties, and treasury authority.

Operational:

* storage engine and atomic commit/recovery semantics;
* P2P transport, authenticated handshake, discovery, and peer identity rotation;
* snapshot format/trust model;
* keystore cipher, KDF, backup, recovery phrase, and auto-lock defaults;
* API authentication/authorization and administrative roles;
* configuration precedence and consensus-parameter mutability;
* release signing, dependency policy, incident response, and threat register.

Specification-language ambiguity:

* recommended initial values are sometimes adjacent to MUST/SHALL behavior without a clear declaration of whether genesis configuration makes them normative;
* several “where practical” clauses affect validator/producer separation without defining deterministic fallback;
* “authenticated or cryptographically signed messages where appropriate” does not define which message classes require which protection;
* “at least two-thirds” is clear arithmetically, but eligible weight changes, exclusions, and membership snapshot timing remain undefined.

⸻

6. Architectural Risks

Critical:

* Safety cannot be evaluated until admission and Sybil assumptions are explicit.
* Determinism can fail at serialization, ordering, tree construction, time handling, configuration, or integer-remainder boundaries.
* A two-thirds certificate is not sufficient by itself without a stated fault/admission model and locking/round-change proof.
* Recovery from conflicting finality risks introducing hidden governance authority unless the safe-mode exit is specified before testing.

High:

* The proposed monorepo has many top-level modules before the primary language is selected; mirroring conceptual components one-to-one may create unnecessary package and build complexity.
* Node, consensus, ledger, and protocol boundaries overlap in the architecture sketches and need an explicit dependency-direction ADR.
* Telemetry is intentionally untrusted, but research conclusions can still be corrupted by spoofed or non-comparable measurements.
* Optional HDD and AI modules are removable in principle, but configuration and test matrices must prove their removal does not change consensus bytes or state.
* Wallet and node APIs expose a large early attack surface; M0/M1 should define schemas only, not activate broad administrative endpoints.

Medium:

* The API specification is much broader than the first executable milestone, increasing maintenance and false-completeness risk.
* Governance is comprehensive but mostly future-facing; implementation of governance automation would be premature.
* No source-code license decision currently exists.

⸻

7. Implementation Blockers

Blocks M0 completion:

* primary language/toolchain decision;
* canonical ADR location and repository layout;
* license decision or explicit placeholder;
* CI platform and single developer command;
* Git repository/checkpoint availability;
* minimum security-document set.

Blocks M1:

* canonical serialization;
* hash function;
* signature algorithm;
* address encoding;
* integer bounds;
* stable, unique requirement IDs and test-vector ownership.

Blocks M2 and later:

* transaction-ID rule;
* state/transaction/receipt roots;
* genesis supply invariant;
* fee and reward placeholder boundaries.

Blocks M5–M7:

* authenticated network design;
* participant registry;
* randomness and selection;
* round-change/locking/fork choice;
* delay construction and timing integration.

⸻

8. Missing Referenced Artifacts

Some are expected future outputs, but their status is not consistently labeled at each reference:

* CONTRIBUTING.md, SECURITY.md, CHANGELOG.md, LICENSE;
* docs/22_REQUIREMENTS_TRACEABILITY_SKELETON.md;
* docs/security/THREAT_REGISTER.md, docs/security/INCIDENT_RESPONSE.md, and docs/security/KEY_MANAGEMENT.md;
* docs/security/DEPENDENCY_POLICY.md and docs/security/RELEASE_SECURITY.md;
* M0_REPORT.md and docs/build milestone plans/reports;
* governance process/role/conflict/emergency/treasury documents;
* future mainnet documents and external-review reports.

Recommendation: add an artifact index with status values PLANNED, REQUIRED_FOR_M0, REQUIRED_FOR_LATER_GATE, or NOT_AUTHORIZED.

⸻

9. ADR Candidates

Blocking before protocol-core work:

* ADR-0001 Primary implementation language and workspace toolchain.
* ADR-0002 Canonical serialization.
* ADR-0003 Hash function and domain-separation scheme.
* ADR-0004 Signature algorithm and key encoding.
* ADR-0005 Address encoding and network separation.

Required for M0 or early M1:

* ADR-0006 Repository/module boundaries and dependency direction.
* ADR-0007 Embedded storage and atomic commit model.
* ADR-0008 P2P transport and authenticated handshake.
* ADR-0009 API framework and API-boundary policy.
* ADR-0010 Test orchestration, deterministic vectors, fuzzing, and CI.
* ADR-0011 Requirement-ID namespace and traceability ownership.
* ADR-0012 Initial participant registry for private/local test networks.
* ADR-0013 Genesis configuration and supply invariant.
* ADR-0014 Consensus time source and clock-tolerance model.
* ADR-0015 Safe-mode persistence and exit authority.

Research ADRs that must not be prematurely approved:

* formal VDF construction;
* HDD consensus/economic role;
* public Sybil resistance;
* public monetary policy;
* public governance and emergency recovery.

⸻

10. Review Decision

Recommended gate result:

Phase 0.1 — REPEAT

Reason:

The design intent is sufficiently clear to continue specification work, but blocking contradictions and missing definitions are neither resolved nor fully registered. M0 planning may proceed. M1 and protocol implementation must wait.

⸻

11. Approved Resolution Record

Resolution Date: 2026-07-24

The original findings above are preserved as the historical Phase 0.1 review.
The following resolutions were subsequently approved:

* requirement identifiers use globally unique document-aware domains;
* canonical repository paths are `YOS/` and `docs/adr/`;
* genesis allocations must equal declared genesis supply exactly;
* the v0.1 private-testnet registry is genesis-defined, permissioned, and
  equal-weighted;
* initial integer reward remainders go to the protocol treasury;
* fee burn is disabled for the initial private-testnet model;
* Git initialization and the normalization work are authorized.

Affected documents and final normalization evidence are recorded in
`docs/25_PHASE_0_1_NORMALIZATION_REPORT.md`.
