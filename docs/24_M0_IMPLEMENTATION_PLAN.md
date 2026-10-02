Q1 Milestone M0 Implementation Plan

Version: 0.1.0
Status: Proposed — no implementation authorized by this document

⸻

1. Objective

Create a reproducible engineering foundation without implementing protocol behavior or silently selecting consensus rules.

⸻

2. Entry Gate

Required before M0 execution:

* human approval of ADR process and canonical ADR path;
* ADR-0001 decision for primary language/toolchain;
* approved repository/module-boundary direction;
* decision on license or explicit temporary placeholder;
* Git repository availability and checkpoint policy;
* decision-register triage completed for M0 dependencies.

M0 may prepare ADR drafts before this gate. It may not treat drafts as decisions.

⸻

3. Work Packages

M0.1 — Specification hygiene

* approve requirement-ID namespace;
* generate the complete collision report;
* migrate identifiers with an alias table;
* normalize document paths and cross-references;
* label planned artifacts and placeholders;
* establish specification changelog rules.

Evidence: duplicate-ID check passes; internal links resolve; human approves migration.

M0.2 — Decision and ADR foundation

* convert OPEN_DECISIONS.md into the active Q1 register;
* create docs/adr/README.md and ADR template;
* draft ADR-0001 through ADR-0011 as needed;
* approve at least ADR-0001 for M0;
* preserve rejected alternatives.

Evidence: every M0 choice links to a decision or approved ADR.

M0.3 — Repository baseline

* create only the approved M0 directories;
* add CONTRIBUTING.md, SECURITY.md, CHANGELOG.md;
* add license or approved placeholder;
* add build and developer instructions;
* define generated-file and artifact policies.

Evidence: fresh-clone instructions contain no undocumented manual steps.

M0.4 — Toolchain and dependency controls

* pin compiler/runtime and package manager;
* commit dependency lockfile;
* configure formatter and linter;
* define dependency review and audit;
* configure secret scanning;
* capture tool versions.

Evidence: clean build environment reproduces tool checks.

M0.5 — Configuration and genesis schemas

* define configuration file schema and precedence;
* separate consensus parameters from local operations;
* reject unknown/unsafe fields;
* define test-only genesis schema without deciding unresolved economics;
* label all placeholder parameters;
* add valid and invalid fixtures.

Evidence: schema tests pass; no schema default silently becomes a protocol rule.

M0.6 — Test and CI foundation

* define unit/integration/specification test layout;
* add empty/smoke test target;
* add deterministic seed and fixture conventions;
* configure CI for build, format, lint, test, secret scan, dependency audit, duplicate requirement IDs, and broken references;
* define one local command equivalent to required CI checks.

Evidence: the same command passes locally and in CI.

M0.7 — Security foundation

* create threat register;
* create vulnerability-reporting and incident-response documents;
* define key-management scope for later milestones;
* define dependency and release-signing policies;
* ensure no live secret or production endpoint is required.

Evidence: security review confirms fail-safe defaults for the foundation.

M0.8 — M0 report and gate

* record files and approved decisions;
* record commands and exact test results;
* list failures and untested areas;
* include artifact hashes and tool versions;
* update roadmap/traceability status;
* request human gate review.

⸻

4. Explicit Non-Goals

M0 must not implement:

* transaction or block serialization;
* cryptographic choices not approved by ADR;
* ledger state transitions;
* producer or committee selection;
* networking;
* delay or HDD work;
* wallet key handling;
* economic formulas;
* AI observer behavior;
* deployment to a public environment.

⸻

5. Risks and Controls

| Risk | Control |
|---|---|
| Toolchain choice biases protocol design | ADR comparison and reversible workspace |
| Placeholder becomes protocol default | explicit placeholder type/status and gate test |
| Schema mixes local and consensus settings | separate namespaces and validation |
| CI gives false confidence | label smoke checks; no protocol claims |
| Future empty packages create coupling | create packages milestone by milestone |
| Documentation drifts | reference and duplicate-ID checks in CI |
| Secrets enter fixtures | synthetic fixtures and secret scanning |

⸻

6. Exit Gate

M0 passes only when a new developer can clone the repository and run one documented command that:

* builds the empty engineering workspace;
* runs tests;
* checks formatting and linting;
* validates schemas;
* scans for secrets;
* audits dependencies;
* validates requirement-ID uniqueness and document references.

Additionally:

* the command and tool versions are documented;
* the lockfile exists;
* all failures are resolved or explicitly block the gate;
* no consensus-critical behavior has been invented;
* a human approves the M0 report.

⸻

7. Recommended Next Safe Step

Review the applied requirement-ID migration and the Proposed ADR-0001 through
ADR-0005 drafts. Do not scaffold the language workspace or begin M1 until those
ADRs are explicitly approved.
