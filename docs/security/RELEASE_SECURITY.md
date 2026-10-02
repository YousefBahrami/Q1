# Q1 Release Security

Version: 0.1.0
Status: M0 Draft

No public binary release or deployment is authorized.

## Required future release properties

- source revision and specification versions recorded;
- clean CI with exact commands and tool versions;
- pinned dependency lockfile and completed audit;
- reproducible-build evidence where feasible;
- artifact hashes;
- signed release metadata under a dedicated release key;
- documented known limitations and unresolved security findings;
- no development placeholder represented as production;
- rollback and incident-response plan;
- independent review at the roadmap gate that requires it.

## Separation

Release keys must not be wallet, validator, producer, node, treasury, or
ordinary maintainer-authentication keys.

## Open decisions

- signing algorithm and key custody;
- authorized release managers and approval threshold;
- artifact distribution;
- reproducibility environment;
- revocation and compromised-key procedure;
- transparency log or equivalent publication record.

