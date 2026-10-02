# Q1 Threat Register

Version: 0.1.0
Status: M0 Draft — extracted from existing specifications
Sources: docs/12_SECURITY_MODEL.md; docs/13_HOW_TO_BREAK_Q1.md

This register does not claim that listed controls are implemented or
sufficient.

| Threat ID | Threat | Protected asset | Attacker | Preconditions | Impact | Specified controls | Required test/evidence | Residual risk | Status |
|---|---|---|---|---|---|---|---|---|---|
| THR-Q1-001 | Forged or altered transaction | Authorization and ledger state | Malicious user/peer | Parser or signature weakness | Unauthorized transfer | Canonical bytes, domain separation, signature verification | Forged signature, field mutation, malformed encoding | Algorithms/serialization open | OPEN |
| THR-Q1-002 | Replay and double spend | Balance, nonce, supply | Malicious user | Missing chain/version/nonce binding | Duplicate spend | Chain ID, version, validity window, nonce, finalized state | Same-chain, cross-chain, cross-version replay | Final serialization open | OPEN |
| THR-Q1-003 | Unauthorized issuance or accounting error | Supply integrity | Producer/operator/bug | Reward or integer error | Hidden inflation/loss | Checked integers, deterministic rewards, supply conservation | Unauthorized reward, overflow, exact genesis and block accounting | Economic formulas open | OPEN |
| THR-Q1-004 | Producer equivocation/censorship | Consensus safety/liveness | Producer | Selection and network opportunity | Conflicting proposals or withheld transactions | Signed proposals, evidence, fallback, committee validation | Equivocation, withholding, censorship scenarios | Round change open | OPEN |
| THR-Q1-005 | Validator equivocation/quorum forgery | Finality | Validator or external attacker | Key compromise, duplicate weight, unsafe signing | Conflicting/false finality | Independent signature verification, unique weight, signing history | Duplicate counting, forged certificate, restart-after-signing | Locking/round rules open | OPEN |
| THR-Q1-006 | Committee or registry capture | Consensus safety | Sybil/operator cartel | Weak admission or biased selection | Controlled quorum | Deterministic registry/selection; private registry label | Registry mismatch, committee bias/capture simulation | Public admission unresolved | OPEN |
| THR-Q1-007 | Randomness grinding | Fair selection | Producer/participant | Influence over seed inputs | Biased producer/committee selection | Finalized verifiable inputs; prohibited local/external entropy | Withholding, transaction influence, seed-bias simulation | Construction unresolved | OPEN |
| THR-Q1-008 | Eclipse, partition, flood, DDoS | Availability and view integrity | Network attacker/peer | Peer concentration/resource imbalance | Isolation, halt, divergent view | Authentication, diversity, limits, bounded queues | Eclipse, partition, peer flood, decompression bomb | Transport/discovery open | OPEN |
| THR-Q1-009 | Invalid sync data, snapshot, or rollback | Finalized state | Peer/operator | Weak verification or storage recovery | Corrupt/stale state | Certificate/genesis/root verification, rollback protection | Invalid snapshot, stale node, database rollback | Snapshot/storage open | OPEN |
| THR-Q1-010 | Delay-proof forgery or resource exhaustion | Producer eligibility/liveness | Producer/hardware attacker | Weak delay construction or limits | Cheap production, verification DoS | Challenge binding, proof limits, fail closed | Invalid proof, oversized proof, hardware benchmark | Formal VDF open | OPEN |
| THR-Q1-011 | HDD spoofing/replay/destructive workload | Research integrity and host data | Participant/plugin defect | Untrusted metadata or unsafe file scope | False claims, data loss, wear | Plugin isolation, telemetry-only default, explicit directory | Fake metadata, RAM disk, replay, path traversal | Physical authenticity unproven | OPEN |
| THR-Q1-012 | AI poisoning or prompt injection | Research/operator decisions | Telemetry or AI attacker | Untrusted input/model | False alert/advice | Read-only isolation, evidence classes, bounded input | Poisoned telemetry, prompt injection, observer shutdown | Model design open | OPEN |
| THR-Q1-013 | Wallet key theft/loss/corruption | Spending keys and funds | Malware/operator/error | Weak keystore, backup, UI | Unauthorized or impossible spending | Encryption, secure randomness, no logging, backup verification | Corrupt keystore, leak scan, wrong-network display | Algorithms/KDF open | OPEN |
| THR-Q1-014 | Supply-chain/release compromise | Software integrity | Dependency/release attacker | Unpinned or unsigned artifacts | Malicious node/wallet | Pin/audit dependencies, reproducible builds, signed releases | Dependency tamper and artifact verification | Tooling/signing open | OPEN |
| THR-Q1-015 | Genesis/configuration/admin mutation | Network identity and ledger state | Privileged operator | Hidden authority or unsafe API | Network split, balance/state rewrite | Genesis hash, strict config, isolated admin API, no direct balance edit | Genesis alteration, unknown config, unauthorized admin request | Auth model open | OPEN |
| THR-Q1-016 | Conflicting finalization certificates | Finality integrity | Byzantine quorum or implementation failure | Safety assumption failure | Irreconcilable histories | Automatic Safe Mode and evidence preservation | Construct conflicting certificates; verify no silent choice | Recovery package open | OPEN |
| THR-Q1-017 | Evidence/log/telemetry corruption or leakage | Incident and privacy data | Operator/attacker | Mutable logs or secret logging | Failed investigation, privacy harm | Structured event codes, evidence integrity, secret filtering | Tamper and secret-pattern scans | Retention/privacy open | OPEN |
| THR-Q1-018 | Treasury/governance capture | Economic and change authority | Steward/founder/cartel | Concentrated keys/process | Misallocation or unsafe upgrades | Explicit limited authority, audit records, future multisig/review | Governance simulation and signed-action tests | Public model open | OPEN |

## Review rule

Every threat must eventually link to globally unique requirement IDs, decision
records, tests, evidence artifacts, and an accountable review owner.

