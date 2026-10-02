# ADR-0004 — Signature Algorithm

Status: Accepted
Date: 2026-07-24
Decision Owner: Yousef Bahrami
Related Decision: DEC-Q1-003
Blocking: M1

## Context

Q1 needs a mature signature algorithm for transactions, block proposals,
validator attestations, and other protocol roles. Role-key separation remains
mandatory. This ADR does not decide multisignature treasury design, aggregate
signatures, key derivation, or post-quantum migration.

## Decision Drivers

- security maturity and reviewed implementations;
- public-key/signature size;
- signing and verification cost;
- malleability and canonical verification;
- deterministic signing behavior;
- batch-verification implications;
- safe key management;
- hardware-wallet, browser, and WebAssembly feasibility;
- cross-language independent clients;
- explicit algorithm migration.

## Options

### Option A — Ed25519

RFC 8032 specifies Ed25519 encodings, signing, verification, and vectors.

Advantages:

- 32-byte public key and 64-byte signature;
- deterministic signing procedure;
- simple fixed-width wire representation;
- broad Rust, Go, Node, browser/WebCrypto, and Wasm implementations;
- no ECDSA per-message random nonce interface;
- modern standardization and test vectors.

Disadvantages:

- strict verification details and accepted encodings must still be specified;
- hardware-wallet support varies by vendor/product;
- no native signature aggregation;
- pure Ed25519, Ed25519ctx, and Ed25519ph must not be mixed.

Risks:

- libraries differing on malformed/non-canonical public keys or signatures;
- batch verification changing behavior if not tested against individual
  verification;
- deterministic signing amplifying damage if a key is stolen; it does not
  replace secure key custody.

### Option B — ECDSA over secp256k1

Advantages:

- extensive distributed-ledger deployment;
- strong hardware-wallet and ecosystem experience;
- compact 33-byte compressed public keys; raw signatures can be 64 bytes under
  a fixed profile.

Disadvantages:

- requires deterministic nonce generation or equally strong nonce handling;
- signature encoding and low-S normalization/rejection must be specified;
- ECDSA admits multiple signature representations without strict rules;
- browser WebCrypto does not offer universal secp256k1 support.

Risks:

- nonce failure exposes private keys;
- DER/fixed-width and recovery-ID confusion;
- accepting high-S and low-S forms creates malleability;
- ecosystem conventions silently becoming Q1 protocol rules.

### Option C — ECDSA over P-256

Advantages:

- NIST FIPS 186-5 ecosystem and broad platform/WebCrypto support;
- strong enterprise/HSM and hardware-backed key support;
- mature implementations across languages.

Disadvantages:

- same ECDSA nonce, malleability, and encoding complexity as secp256k1;
- protocol must fix raw versus DER signatures and low-S behavior;
- less alignment with some distributed-ledger tooling than secp256k1.

Risks:

- platform APIs accepting/producing different encodings;
- nonce generation and canonical-S mistakes.

### Option D — BLS-family aggregate signature

Advantages:

- signature aggregation could reduce certificate size;
- useful for large committees in later research.

Disadvantages:

- substantially more complex validation, subgroup, proof-of-possession, and
  pairing implementation surface;
- larger keys or signatures depending on orientation;
- hardware-wallet/browser availability and implementation maturity are less
  favorable for an initial ordinary transfer wallet.

Risks:

- rogue-key/subgroup mistakes;
- aggregation obscuring which individual signature failed;
- premature optimization before committee size is decided.

Assessment:

Not recommended for Q1 v0.1. Preserve as future certificate research only.

## Comparative Matrix

Scale: 5 = strongest fit.

| Criterion | Ed25519 | secp256k1 ECDSA | P-256 ECDSA | BLS family |
|---|---:|---:|---:|---:|
| Security maturity | 5 | 5 | 5 | 3 |
| Public-key size | 5 | 4 | 4 | 3 |
| Signature size | 5 | 5 fixed / 3 DER | 5 fixed / 3 DER | 4 |
| Verification cost | 4 | 4 | 4 | 2 |
| Signing cost/simplicity | 5 | 3 | 3 | 2 |
| Malleability ergonomics | 5 strict | 2 without low-S | 2 without low-S | 3 |
| Deterministic signing | 5 native | 4 via RFC 6979 | 4 via RFC 6979 | 3 |
| Batch/aggregation | 3 batch only | 2 | 2 | 5 |
| Library maturity | 5 | 5 | 5 | 3 |
| Hardware-wallet feasibility | 3–4 | 5 | 5 | 2 |
| Browser/WebCrypto | 4 | 2 | 5 | 2 |
| WebAssembly | 5 | 4 | 4 | 3 |
| Independent-client support | 5 | 5 | 5 | 3 |
| Initial audit simplicity | 5 | 3 | 3 | 1 |

Hardware scores require vendor-specific verification before wallet decisions.

## Canonical Profile Requirements

If Ed25519 is selected:

- exact RFC 8032 Ed25519 variant;
- 32-byte public key and 64-byte signature;
- strict malformed/non-canonical rejection;
- sign Q1 domain-separated canonical bytes;
- verify each signature independently in the initial implementation;
- no silent context/prehash variant;
- no public-key recovery convention.

If ECDSA is selected:

- exact curve and hash/prehash rules;
- RFC 6979 or approved nonce procedure;
- fixed-width public-key and signature encoding;
- strict range checks;
- low-S creation and high-S rejection;
- no DER/recovery ID in consensus bytes unless explicitly specified.

## Security Impact

No custom curve arithmetic is authorized. Use reviewed libraries, record their
validation behavior, and fuzz malformed inputs. Spending, producer, validator,
node, release, and treasury keys must remain separate.

## Determinism Impact

Verification must be deterministic and identical across clients. Signatures
bind the exact ADR-0002 bytes and domain. Batch verification, if later enabled,
must return the same validity result as individual verification.

## Implementation Impact

The cryptography provider should expose typed role-aware operations rather than
a generic “sign arbitrary bytes” interface in application code. Algorithm and
encoding identifiers must be explicit at version boundaries.

## Testing Impact

- official positive/negative vectors;
- malformed key/signature and boundary scalars;
- field/domain/chain/height/round mutations;
- wrong role key;
- non-canonical/malleability vectors;
- repeat-signing behavior;
- batch versus individual equivalence if used;
- cross-language/browser/Wasm vectors;
- secure-random failure for key generation.

## Deployment Impact

Wallet, validator, and producer key storage must support the selected key type.
Hardware-wallet support is not assumed until tested. Release signing remains a
separate policy even if it reuses the primitive.

## Migration Impact

Transactions and accounts need explicit algorithm/address versions. Migration
must allow old signatures to remain verifiable without permitting downgrade or
cross-algorithm replay. ADR-0005 must encode an address version independently
of one permanent signature assumption.

## Recommendation

Clearly labeled recommendation:

Use **Ed25519 as specified by RFC 8032** for Q1 v0.1 transaction, producer, and
validator signatures, with strict fixed-width verification and individual
signature validation.

Why:

- smallest initial encoding and verification rule surface;
- deterministic signing and mature cross-language vectors;
- strong Rust/Go/Node/Wasm/browser feasibility;
- avoids ECDSA nonce and low-S/DER consensus complexity;
- does not prematurely introduce aggregation.

Preserved alternatives:

- P-256 if enterprise hardware-backed/WebCrypto interoperability is a dominant
  human requirement;
- secp256k1 if specific hardware-wallet and distributed-ledger tooling
  interoperability is prioritized above simpler canonical rules.

## Unresolved Questions

- Which hardware wallets/platform secure enclaves are required?
- Does Q1 require browser signing in Horizon 1?
- Which strict-verification libraries behave identically in the chosen
  languages?
- Should transaction and validator roles use the same primitive but always
  separate keys?
- When should post-quantum migration be researched?

## Evidence and Limitations

No cryptographic library, key generation, signing, verification, batch, browser,
hardware-wallet, or performance experiment was run. Cost and ecosystem scores
are qualitative and require implementation-specific confirmation.

Primary sources:

- RFC 8032 EdDSA:
  https://www.rfc-editor.org/info/rfc8032/
- NIST FIPS 186-5:
  https://csrc.nist.gov/pubs/fips/186-5/final
- RFC 6979 deterministic ECDSA:
  https://datatracker.ietf.org/doc/html/rfc6979
- W3C Web Cryptography Level 2:
  https://www.w3.org/TR/webcrypto-2/
- RFC 8812 secp256k1 registrations:
  https://www.rfc-editor.org/info/rfc8812/

## Approval Record

Accepted by Yousef Bahrami on 2026-07-24, with the explicit RFC 8032-compatible
profile in `docs/protocol/Q1_ED25519_PROFILE_V1.md` required before M1 may
begin. Ed25519, Ed25519ctx, and Ed25519ph must not be mixed implicitly.
