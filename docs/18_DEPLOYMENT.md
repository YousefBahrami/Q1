Q1 Deployment Specification

18_DEPLOYMENT.md

Project: Q1 Experimental Distributed Ledger
Protocol Version: 0.1
Deployment Version: 0.1.0
Status: Draft for Engineering and Operations Review
Classification: Experimental — Development and Private Testnet Only

⸻

1. Purpose

This document defines how Q1 SHALL be deployed from source code to executable environments.

Deployment covers:

* build artifacts;
* runtime configuration;
* node installation;
* wallet installation;
* explorer deployment;
* AI Observer deployment;
* simulation services;
* telemetry infrastructure;
* backups;
* upgrades;
* recovery;
* monitoring;
* operational security.

Deployment does not define protocol rules.

It defines how protocol implementations are operated.

⸻

2. Deployment Principles

Q1 SHALL follow these operational principles.

DEP-001 — Reproducibility

A deployment SHALL be reproducible from:

* source revision;
* tagged release;
* dependency lockfile;
* configuration;
* genesis file.

⸻

DEP-002 — Immutable Releases

Every released build SHALL have:

* version;
* build identifier;
* cryptographic hash;
* release notes.

⸻

DEP-003 — Environment Isolation

Development, testing and private testnet SHALL remain isolated.

Configuration MUST NOT be reused automatically between environments.

⸻

DEP-004 — Least Privilege

Every service SHALL execute with only the permissions it requires.

⸻

DEP-005 — Replaceable Components

Explorer, AI Observer, Simulator and Telemetry MUST be deployable independently from consensus nodes.

⸻

3. Deployment Environments

Q1 SHALL define:

DEV
LOCALNET
PRIVATE_TESTNET
RESEARCH
RELEASE_CANDIDATE

No production/mainnet environment is defined in v0.1.

⸻

4. Environment Goals

Development

Purpose:

* coding;
* debugging;
* unit testing.

Topology:

1 node
1 wallet

⸻

Localnet

Purpose:

Consensus development.

Topology:

4–7 nodes
local machine(s)

⸻

Private Testnet

Purpose:

Real distributed validation.

Topology:

7+ independent nodes
multiple hosts
archive node
observer node

⸻

Research

Purpose:

Benchmarking and HDD experiments.

Consensus participation MAY be disabled.

⸻

Release Candidate

Purpose:

Final verification before broader distribution.

⸻

5. Deployment Architecture

Recommended layout:

q1-node
q1-wallet
q1-explorer
q1-telemetry
q1-observer
q1-simulator
q1-test-orchestrator

Each SHOULD be independently deployable.

⸻

6. Directory Layout

Example:

/opt/q1/
    bin/
    config/
    data/
    logs/
    snapshots/
    backups/
    telemetry/

User secrets SHOULD NOT be stored inside executable directories.

⸻

7. Configuration Files

Separate configuration by purpose:

node.toml
network.toml
wallet.toml
telemetry.toml
observer.toml
simulation.toml

Consensus parameters SHALL originate from genesis or finalized protocol rules—not local configuration.

⸻

8. Genesis Handling

Every deployment MUST specify exactly one genesis file.

Node startup MUST verify:

* genesis hash;
* chain identifier;
* protocol version.

Mismatch SHALL stop startup.

⸻

9. Build Artifacts

Each release SHOULD produce:

q1-node
q1-wallet
q1-explorer
q1-observer
q1-simulator
checksums.txt
release_notes.md

⸻

10. Versioning

Every artifact SHALL expose:

* software version;
* protocol version;
* build hash;
* git commit (if available).

⸻

11. Node Installation

Node installation SHALL include:

* executable;
* configuration;
* genesis;
* logging;
* service definition.

Startup MUST validate configuration before joining the network.

⸻

12. Wallet Installation

Wallet SHALL operate independently.

Private keys MUST remain local.

Wallet installation MUST NOT require:

* explorer;
* AI Observer;
* simulator.

⸻

13. Explorer Deployment

Explorer SHALL consume finalized chain data.

Explorer SHALL NEVER become consensus authority.

⸻

14. AI Observer Deployment

Observer SHALL subscribe to telemetry.

Observer SHALL NOT:

* sign blocks;
* vote;
* modify balances;
* change protocol state.

⸻

15. Telemetry Deployment

Telemetry SHOULD operate asynchronously.

Node execution MUST continue even if telemetry becomes unavailable.

⸻

16. Simulation Deployment

Simulation SHALL execute independently.

Simulation MUST NEVER connect directly to modify live ledger state.

⸻

17. Storage

Consensus storage SHALL be separated from:

* telemetry;
* logs;
* simulation output.

⸻

18. Logging

Logs SHALL rotate automatically.

Recommended levels:

ERROR
WARN
INFO
DEBUG
TRACE

TRACE SHOULD remain disabled outside development.

⸻

19. Backup Strategy

Backup SHALL include:

* configuration;
* genesis;
* databases;
* snapshots;
* logs (optional).

Wallet backups SHALL remain separate.

⸻

20. Snapshot Strategy

Snapshots MUST reference finalized heights.

Restoring a snapshot SHALL verify:

* chain ID;
* genesis hash;
* snapshot integrity.

⸻

21. Upgrade Procedure

Upgrade steps:

1. verify release;
2. backup;
3. stop node;
4. upgrade binaries;
5. migrate database if required;
6. restart;
7. verify synchronization.

⸻

22. Rollback

Rollback SHALL be supported only to compatible database versions unless migration explicitly supports reversal.

⸻

23. Monitoring

Monitor:

* uptime;
* peer count;
* synchronization;
* finalized height;
* CPU;
* memory;
* storage;
* delay metrics;
* HDD metrics;
* network latency.

⸻

24. Health Checks

Health endpoints SHALL distinguish:

* process alive;
* ready for service;
* synchronized;
* participating.

⸻

25. Secrets

Secrets include:

* wallet keys;
* validator keys;
* producer keys;
* admin credentials.

Secrets SHALL:

* remain encrypted at rest where possible;
* never appear in logs;
* never be committed to source control.

⸻

26. Key Rotation

Operational credentials SHOULD support rotation.

Consensus identities require protocol-aware handling.

⸻

27. Firewall Guidance

Only required ports SHOULD be exposed.

Administrative endpoints SHOULD bind to localhost by default.

⸻

28. TLS

Remote administrative interfaces SHOULD require encrypted transport.

Local development MAY use plaintext.

⸻

29. Containers

Container deployment MAY be supported.

Container images SHOULD be:

* minimal;
* reproducible;
* versioned.

Containers SHALL NOT embed wallet secrets.

⸻

30. Resource Limits

Each deployment SHOULD define:

* CPU limits;
* memory limits;
* storage limits;
* file descriptor limits.

⸻

31. Time Synchronization

Nodes SHOULD maintain reasonably accurate system clocks.

Protocol ordering remains height/round based.

⸻

32. Incident Recovery

Recovery procedure SHALL include:

1. preserve evidence;
2. isolate affected node;
3. restore from verified snapshot if required;
4. rejoin network.

⸻

33. Safe Mode Operations

When safe mode activates:

* production stops;
* diagnostics remain available;
* evidence is preserved;
* recovery follows documented procedure.

⸻

34. Release Checklist

Before distributing a build:

* tests passed;
* hashes generated;
* release notes written;
* vulnerabilities reviewed;
* documentation updated.

⸻

35. Deployment Acceptance

Deployment is considered successful when:

* node starts;
* configuration validates;
* joins peers;
* synchronizes;
* finalizes correctly (where applicable);
* survives restart;
* exposes health endpoints;
* produces expected telemetry.

⸻

36. Known Limitations

v0.1 does not define:

* cloud-specific deployment;
* Kubernetes orchestration;
* production HA clusters;
* public mainnet rollout.

⸻

37. Open Decisions

Outstanding deployment questions include:

1. preferred packaging format;
2. container strategy;
3. service manager;
4. automatic update policy;
5. backup retention;
6. snapshot compression;
7. monitoring stack;
8. metrics backend;
9. release signing;
10. artifact distribution.

All SHALL be tracked in:

OPEN_DECISIONS.md

⸻

38. Codex Implementation Rules

Codex MUST:

1. separate executable components;
2. validate configuration before startup;
3. verify genesis compatibility;
4. expose health and readiness;
5. isolate secrets;
6. separate consensus storage from telemetry;
7. support reproducible builds;
8. generate release hashes;
9. support graceful shutdown;
10. document deployment steps.

Codex MUST NOT:

* auto-create wallet keys without user intent;
* bypass configuration validation;
* expose admin endpoints publicly by default;
* couple AI Observer to consensus execution;
* require simulator for node operation.

⸻

39. Final Deployment Principle

Deployment exists to make software reproducible—not mysterious.

A correctly deployed node SHALL behave exactly as specified, regardless of who installs it.

Operational convenience MUST never override protocol correctness or security.

The deployment process succeeds when another engineering team can reproduce the same network from the published specifications, verify every artifact, and reach the same deterministic protocol behavior without relying on undocumented knowledge.