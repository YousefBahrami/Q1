Q1 Initial Repository Structure Proposal

Version: 0.1.0
Status: Proposed — requires ADR approval

⸻

1. Principle

Use a monorepo, but begin with milestone-oriented structure rather than creating every future subsystem as an empty package. Conceptual boundaries remain documented even when no code directory exists yet.

⸻

2. M0 Structure

Q1/
├── README.md
├── PROJECT.md
├── AGENTS.md
├── LICENSE                         # or approved decision placeholder
├── CHANGELOG.md
├── CONTRIBUTING.md
├── SECURITY.md
├── OPEN_DECISIONS.md
├── IMPLEMENTATION_NOTES.md
├── docs/
│   ├── 00_PROJECT_CHARTER.md
│   ├── ...
│   ├── 20_ROADMAP.md
│   ├── 21_SPECIFICATION_CONSISTENCY_REVIEW.md
│   ├── 22_REQUIREMENTS_TRACEABILITY_SKELETON.md
│   ├── 23_INITIAL_REPOSITORY_STRUCTURE.md
│   ├── 24_M0_IMPLEMENTATION_PLAN.md
│   ├── adr/
│   │   ├── README.md
│   │   └── ADR-0001-primary-language.md
│   ├── build/
│   │   └── M0_PLAN.md
│   └── security/
│       ├── THREAT_REGISTER.md
│       ├── INCIDENT_RESPONSE.md
│       ├── KEY_MANAGEMENT.md
│       ├── DEPENDENCY_POLICY.md
│       └── RELEASE_SECURITY.md
├── configs/
│   ├── schema/
│   └── examples/
├── genesis/
│   ├── schema/
│   └── examples/
├── scripts/
├── tests/
│   ├── specification/
│   └── fixtures/
└── toolchain-defined workspace files

⸻

3. Deferred Until Authorized Milestones

Create implementation packages only when their milestone begins:

* M1: protocol and crypto;
* M2: ledger;
* M3: node runtime and storage;
* M4: wallet and public API;
* M5: network;
* M6: consensus;
* M7: delay;
* M8: hdd_lab;
* M9: economics;
* M10: explorer, telemetry, ai_observer;
* M11: simulator and adversarial testnet tools;
* M12: deployment packaging.

This avoids empty architecture and lets the selected language determine the exact package layout.

⸻

4. Dependency Direction

Proposed rule for ADR review:

protocol types and pure rules
← cryptographic provider interfaces
← ledger state transition
← consensus rules
← node orchestration
← network/API/storage adapters
← wallet, explorer, telemetry, AI, simulation

No dependency may point from deterministic protocol code to:

* operating-system HDD commands;
* AI/model code;
* HTTP handlers;
* explorer/indexer storage;
* wall-clock-only decisions;
* administrative mutation logic.

⸻

5. Path Resolution

Approved on 2026-07-24:

* the canonical ADR path is `docs/adr/`;
* the canonical collaboration-framework path is `YOS/`;
* report categories live under `docs/reports/`.

`Plain text` and `Plain text.md` remain unclassified scratch artifacts and must
not be deleted without human approval.
