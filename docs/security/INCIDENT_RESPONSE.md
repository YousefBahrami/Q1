# Q1 Incident Response

Version: 0.1.0
Status: M0 Draft
Related Decision: DEC-Q1-008

## Scope

This draft covers local and private-testnet incidents only. It creates no
public-network emergency authority.

## Principles

- preserve evidence before remediation where safe;
- protect keys, personal data, and unrelated infrastructure;
- distinguish protocol failure from implementation/operation failure;
- enter Safe Mode automatically on specified critical conditions;
- never select between conflicting valid finalization certificates silently;
- never exit Safe Mode through a simple administrative disable switch;
- record uncertainty, failed actions, and human approvals.

## Lifecycle

1. Detect and assign an incident identifier.
2. Classify severity using docs/12_SECURITY_MODEL.md.
3. Contain affected test infrastructure.
4. Preserve objective evidence and hashes.
5. Analyze cause and affected versions/configurations.
6. Propose mitigation and recovery.
7. Obtain required human approvals.
8. Execute only the approved recovery package.
9. Verify invariants and monitor.
10. Publish an internal post-incident record and regression-test plan.

## Safe-mode recovery package

Required fields:

- incident identifier;
- reason code;
- preserved evidence references and hashes;
- expected finalized checkpoint;
- expected genesis hash and chain ID;
- required software version;
- recovery action;
- authorized human approvals;
- execution and audit record.

Exact validation rules, approval threshold, checkpoint policy, and post-conflict
history treatment remain OPEN under DEC-Q1-008 and DEC-Q1-020.

## Prohibited shortcuts

- undocumented state edits;
- direct balance repair;
- hidden recovery keys;
- deleting incident evidence to restore service;
- treating AI output as recovery authority;
- an unaudited `POST /safe-mode/disable` operation.

## Placeholders requiring approval

- private intake channel;
- incident commander/roles;
- severity-to-notification timing;
- evidence retention and privacy rules;
- recovery-package signature/authorization format.

