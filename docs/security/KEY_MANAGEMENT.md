# Q1 Key Management

Version: 0.1.0
Status: M0 Draft

No production key-management system or cryptographic algorithm is approved.

## Required role separation

Distinct keys are required for:

- wallet spending;
- node/network identity;
- block production;
- validator attestation;
- testnet registry administration, if approved;
- release signing;
- treasury/governance, if later approved.

Key reuse across these roles is prohibited unless a future specification
explicitly proves and approves an exception.

## Baseline controls

- generate secret keys using an approved operating-system CSPRNG through a
  reviewed library;
- fail closed if secure randomness fails;
- encrypt wallet private keys at rest using algorithms selected by later ADR;
- never log secrets, recovery phrases, passwords, or raw keystore material;
- never pass passwords as command-line arguments;
- keep test fixtures synthetic and visibly non-production;
- zeroization, memory locking, hardware storage, rotation, and backup formats
  remain implementation/security decisions requiring review.

## Lifecycle placeholders

Generation → storage → use → backup → rotation → revocation → destruction

For each role, later revisions must define owner, authorization, storage,
backup, recovery, rotation trigger, compromise response, and audit evidence.

## Open dependencies

- ADR-0004 signature algorithm;
- wallet KDF/cipher and keystore format;
- participant-registry key rotation;
- release-signing policy;
- treasury/governance authorization;
- quantum-migration strategy.

