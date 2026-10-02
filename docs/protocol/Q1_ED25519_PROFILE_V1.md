# Q1 Ed25519 Profile V1

Version: 0.1.0

Algorithm identifier: proposed `0x0001`

Status: Normative — Approved 2026-07-24

Authority: ADR-0004

Standard basis: RFC 8032

## 1. Exact Variant

Q1 V1 uses pure Ed25519 from RFC 8032.

- Ed25519ctx is prohibited.
- Ed25519ph is prohibited.
- External prehashing of the signed message is prohibited.
- A library API that does not make the selected variant unambiguous SHALL NOT
  be used.

The message supplied to Ed25519 is the exact
`Q1DomainFrameV1(domain_id, canonical_payload)` byte sequence from the
cryptographic domain registry.

## 2. Encodings

| Item | Wire encoding |
|---|---|
| Algorithm ID | unsigned 16-bit value `0x0001` in the containing schema |
| Public key | 32-byte compressed Edwards-y encoding defined by RFC 8032 |
| Signature | 64 bytes: encoded `R` (32) followed by scalar `S` (32) |
| Private seed | 32 bytes; key-custody input only, never a protocol field |

No PEM, DER, JWK, hex, or Base64 wrapper is part of a consensus object.

## 3. Strict Verification

A conforming verifier SHALL:

1. require an exact 32-byte public key and exact 64-byte signature;
2. require canonical encodings of public point `A` and signature point `R`;
3. reject a public key or `R` that fails point decoding, uses a non-canonical
   encoding, is the identity, or has small order;
4. interpret `S` as little-endian and reject unless `0 <= S < L`;
5. compute the RFC 8032 pure-Ed25519 challenge over the exact domain frame;
6. check the cofactorless RFC 8032 verification equation;
7. return invalid, never panic, on every malformed input.

Individual verification defines validity. Batch verification, if later
authorized, SHALL return the same result for every corpus entry and SHALL
fall back safely when a batch fails.

Candidate V1 semantics match `ed25519-dalek 3.0.0` `verify_strict`: canonical
encodings and small-order points are rejected, while non-small-order points
with a mixed torsion component are not rejected merely for lacking full
prime-order-subgroup membership. Requiring full subgroup membership is the
preserved stricter alternative. This distinction is consensus-critical and
requires explicit human approval plus a malformed-point corpus.

## 4. Signed-Message Construction

Each signature role SHALL use its assigned domain. For canonical payload `P`:

```text
message = Q1DomainFrameV1(role_domain_id, P)
signature = Ed25519.Sign(role_private_key, message)
```

The payload SHALL exclude the signature field being produced and SHALL use
the role's approved canonical signing schema. A verifier reconstructs the
same payload and frame; it SHALL NOT accept caller-provided arbitrary message
bytes as if they were a typed protocol object.

Chain/network binding, height, round, and other replay boundaries must appear
in the role schema where required. Domain separation alone does not supply
missing context.

## 5. Key Generation and Custody

- Generate each 32-byte seed using an operating-system CSPRNG through a
  reviewed library. RNG failure is fatal; fallback entropy is prohibited.
- Never derive protocol keys from passwords, timestamps, account addresses,
  JavaScript `Math.random`, or an unapproved common master key.
- The consensus core may receive signing results or a narrowly scoped signer
  interface; raw private seeds SHALL NOT cross into TypeScript, logs,
  telemetry, canonical objects, or test fixtures.
- At rest, production/private-testnet keys require encrypted storage or an
  approved hardware/isolated signer. Exact custody technology remains an
  operations/security decision.
- Backup, rotation, revocation, deletion, and recovery procedures must be
  documented before persistent validator or producer keys exist.

## 6. Role Separation

Transaction/account, block producer, validator attestation, governance,
treasury, node transport, and release-signing roles SHALL use independently
generated keys. Reusing the Ed25519 primitive does not authorize reusing a
key. Domain identifiers and key roles SHALL both be checked.

## 7. Test Vectors

### 7.1 Official RFC 8032 Vector 1

```text
seed:
9d61b19deffd5a60ba844af492ec2cc4
4449c5697b326919703bac031cae7f60

public key:
d75a980182b10ab7d54bfed3c964073a
0ee172f3daa62325af021a68f707511a

message: (empty)

signature:
e5564300c360ac729086e2cc806e828a
84877f1eb8e5d974d873e06522490155
5fb8821590a33bacc61e39701cf9b46b
d25bf5f0595bbe24655141438e7a100b
```

This vector verifies the primitive only. Q1 never signs an unframed empty
consensus message.

### 7.2 Required Q1 Project Vector

The project vector SHALL use:

```text
seed = 000102030405060708090a0b0c0d0e0f
       101112131415161718191a1b1c1d1e1f
domain = TRANSACTION_SIGNING (0x0001)
canonical payload = 82014200ff
```

The exact frame, public key, and signature are
shown below:

```text
domain frame:
5131445300010001000000000000000582014200ff

public key:
03a107bff3ce10be1d70dd18e74bc0996
7e4d6309ba50d5f1ddc8664125531b8

signature:
355e9ab16419b61d545e9d0e61402352
d85ea87d0335eafbf9ec0c1c9a9bcb9e
7d22645a93042a8632973d83e8fb5a83
945dd53f09e0b288723513bdcb32e908
```

Two independently maintained implementations reproduced this vector exactly
on 2026-07-24:

| Implementation | Version/underlying code | Valid | Mutated message | Mutated signature |
|---|---|---|---|---|
| Rust `ed25519-dalek` | 3.0.0; curve25519-dalek 5.0.0 | strict verify passed | rejected | rejected |
| Node.js `node:crypto` | Node 20.17.0; OpenSSL 3.0.13+quic | verify passed | rejected | rejected |

The Rust implementation is pure Rust/dalek. Node uses its OpenSSL-backed
Ed25519 implementation; they are not two wrappers around the same library.

### 7.3 Mandatory Negative Corpus

Reject:

- public keys of lengths 0, 31, 33, and 64;
- signatures of lengths 0, 63, and 65;
- non-canonical point encodings;
- identity, small-order, non-curve, and non-prime-subgroup public keys or `R`;
- `S = L`, `S > L`, and all-`ff` `S`;
- correct signature under the wrong domain, key role, chain, or payload;
- Ed25519ctx/Ed25519ph output presented as pure Ed25519;
- any one-bit mutation of key, frame, or signature.

## 8. Migration

Algorithm ID `0x0001` identifies this exact profile, not every behavior a
library labels “Ed25519.” A future algorithm or materially changed verifier
uses a new algorithm/profile ID and explicit activation. Old signatures remain
interpretable only under their original object and algorithm versions;
downgrade and implicit fallback are prohibited.

## 9. Candidate Rust Adapter

Conditional recommendation:

```text
ed25519-dalek = 3.0.0
default features = false
features = fast, zeroize
verification = VerifyingKey::verify_strict
```

Reasons:

- typed signing and verifying keys and an explicit strict API;
- the crate forbids unsafe code in its own source; the curve backend remains a
  separate dependency-review boundary;
- usable without `std`, although no_std is not an initial node requirement;
- fixed Q1 vector compatibility and established validation-vector tests;
- the historical double-public-key signing-oracle advisory affected versions
  below 2.0.0 and is patched in this candidate.
- the curve25519-dalek scalar-subtraction timing advisory is patched from
  4.1.3 onward; the locked candidate uses curve25519-dalek 5.0.0.

Limitations:

- the isolated combined Ed25519/SHA experiment locks 23 packages;
- strict behavior is dalek-specific and must become a language-neutral corpus;
- no current formal third-party audit report was located during this review;
- `ring 0.17.14` and `ed25519-compact 2.3.1` remain alternatives but were not
  shown to expose the exact candidate strict semantics as clearly.

This is a library recommendation, not authorization to create the permanent
Q1 adapter.

## 10. Approval Record

Decision rows: PREM1-SIG-001 through PREM1-SIG-011 in
`docs/28_PRE_M1_NORMATIVE_PARAMETER_DECISION_PACKAGE.md`.

All rows, including `ed25519-dalek 3.0.0` and its `verify_strict` semantics,
were approved by Yousef Bahrami on 2026-07-24. Persistent key custody remains
outside M1.1.
