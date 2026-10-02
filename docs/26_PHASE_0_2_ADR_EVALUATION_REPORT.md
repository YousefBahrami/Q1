# Q1 Phase 0.2 ADR Evaluation Report

Report Version: 0.1.0
Date: 2026-07-24
Status: Ready for Human Review
Phase: Horizon 0, Phase 0.2

## 1. Outcome

ADR-0001 through ADR-0005 were expanded from preliminary comparisons into
human decision packages.

All five are:

`READY_FOR_DECISION`

None is:

`ACCEPTED`

No ledger, transaction, consensus, wallet, networking, delay, cryptographic,
serialization, or other protocol implementation was created.

## 2. Summary

| ADR | Question | Recommendation | Preserved Alternative | Readiness |
|---|---|---|---|---|
| ADR-0001 | Language and repository architecture | Rust consensus-critical core plus a deliberately narrow TypeScript application/tooling layer | Go core plus TypeScript; Go-only core; TypeScript-only core under strict controls | READY_FOR_DECISION |
| ADR-0002 | Canonical serialization | Q1 Restricted Deterministic CBOR profile | Purpose-built fixed binary; Borsh-style deterministic format | READY_FOR_DECISION |
| ADR-0003 | Consensus object hash | SHA-256 with mandatory Q1 domain separation | SHA3-256; BLAKE3-256 after evidence | READY_FOR_DECISION |
| ADR-0004 | Signature algorithm | Strict RFC 8032 Ed25519 without initial aggregation | P-256 for platform/HSM priority; secp256k1 for wallet/ecosystem priority | READY_FOR_DECISION |
| ADR-0005 | Address text encoding | Q1-specific Bech32m envelope | Base58Check if hardware-wallet/compactness priorities dominate | READY_FOR_DECISION |

These recommendations are analysis, not project decisions.

## 3. ADR-0001 Readiness

Options assessed:

- Rust core;
- Go core;
- TypeScript/Node.js core;
- systems-language core plus TypeScript application/tooling layer.

TypeScript was assessed substantively:

- strengths: npm ecosystem, web/API/SDK/documentation productivity, developer
  availability, testing, and AI-agent iteration;
- consensus risks: Number/BigInt discipline, JSON integers, erased types and
  runtime validation, event-loop blocking, worker complexity, GC/JIT behavior,
  deterministic bytes, dependency/install surface;
- hybrid opportunities: generated SDKs, versioned APIs, Wasm adapters, and
  cross-language vectors;
- hybrid costs: two toolchains, boundary drift, duplicated rules, and release
  complexity.

Recommendation distinctions:

- best consensus-critical core: Rust;
- best pragmatic core alternative: Go;
- best user/developer-facing language: TypeScript;
- best overall architecture: narrow Rust + TypeScript hybrid.

Evidence limitation:

Rust/Cargo was not installed. No comparative build or performance experiment
was run. The recommendation is architectural inference, not a measured result.

## 4. ADR-0002 Readiness

Options assessed:

- purpose-built canonical binary;
- restricted deterministic CBOR;
- deterministic Protocol Buffers under strict constraints;
- Borsh-style deterministic encoding.

Important finding:

Official Protocol Buffers documentation states deterministic serialization is
not canonical. It is therefore not recommended for bytes Q1 hashes or signs.

The recommended CBOR profile:

- definite lengths;
- preferred shortest integers/lengths;
- schema arrays rather than maps;
- no floats, tags, indefinite encodings, or permissive unknown fields;
- strict rejection outside the Q1 profile.

An illustrative transfer object, candidate byte sequence, and negative-vector
strategy were added. They are explicitly non-normative and were not verified
by an installed CBOR implementation.

## 5. ADR-0003 Readiness

Candidates assessed:

- SHA-256;
- SHA3-256;
- BLAKE2 profile;
- BLAKE3-256 profile.

Recommendation:

SHA-256 for v0.1 consensus object hashing because standards maturity,
cross-language/browser/hardware availability, and independent-client
feasibility outweigh unproven throughput needs.

The ADR separates:

- consensus object hashing;
- address checksum;
- non-security implementation hashes;
- delay/VDF research.

No custom hash is proposed.

## 6. ADR-0004 Readiness

Candidates assessed:

- Ed25519;
- secp256k1 ECDSA;
- P-256 ECDSA;
- BLS-family aggregation as a future alternative.

Recommendation:

Strict RFC 8032 Ed25519 for v0.1 transactions, producers, and validators, with
separate role keys and individual verification.

Rationale:

- fixed-size canonical profile;
- deterministic signing procedure;
- avoids ECDSA nonce and low-S/DER consensus rules;
- cross-language and browser/Wasm feasibility;
- no premature aggregation.

No library, browser, hardware wallet, or performance measurement was performed.

## 7. ADR-0005 Readiness

Candidates assessed:

- Q1-specific Bech32m profile;
- Base58Check;
- checksummed hexadecimal;
- CashAddr-style profile;
- Multibase envelope.

Recommendation:

Q1-specific Bech32m envelope with:

- visible network HRP;
- address-format version;
- payload-type identifier;
- strict lowercase canonical form;
- no automatic correction.

The address envelope remains independent of whether the payload is a public
key, key hash, multisignature policy, or future account type.

Development/local/private HRPs in the ADR are examples, not approved values.

## 8. Blocking Questions

Before acceptance, the human must decide:

ADR-0001:

- whether stronger Rust core assurance is worth higher initial complexity;
- whether the TypeScript layer is authorized immediately or only when a
  consumer exists;
- whether a disposable Rust-versus-Go spike is required.

ADR-0002:

- whether positional array schemas are acceptable;
- whether strict canonicality is checked directly or by exact re-encoding;
- whether an independent schema language such as CDDL is required.

ADR-0003:

- whether universal SHA-256 platform support outweighs SHA3/BLAKE domain or
  performance properties;
- exact Q1 domain-tag grammar.

ADR-0004:

- hardware-wallet and secure-enclave priorities;
- strict Ed25519 library behavior across selected languages;
- role-key primitive reuse versus key separation.

ADR-0005:

- final test-network HRPs;
- version/type bit layout;
- account payload derivation and size;
- hardware-wallet display support.

None of these questions authorizes protocol implementation.

## 9. Cross-ADR Dependencies

```text
ADR-0001 language/architecture
    ├── constrains library evidence for ADR-0002
    ├── constrains library evidence for ADR-0003
    ├── constrains library/hardware evidence for ADR-0004
    └── constrains SDK/Wasm evidence for ADR-0005

ADR-0002 canonical bytes
    ├── input to ADR-0003 object hashing
    └── input to ADR-0004 signatures

ADR-0003 object hash
    ├── commitment/address-payload research
    └── does not select the ADR-0005 text checksum

ADR-0004 signature/key encoding
    └── informs ADR-0005 initial payload type

ADR-0005 address envelope
    └── deliberately keeps future payload algorithms versioned
```

Potential cycle:

Language choice evaluates available serialization/crypto libraries, while
serialization/crypto choices affect language ecosystem suitability.

Cycle-breaking rule:

1. choose language architecture using standards-level and ecosystem evidence;
2. choose format/algorithms at the specification level;
3. validate exact libraries in the selected language before M1 implementation;
4. if required libraries fail conformance/security review, reopen the affected
   ADR rather than silently substituting behavior.

## 10. Recommended Decision Order

1. ADR-0001 — language and repository architecture
2. ADR-0002 — canonical serialization
3. ADR-0003 — consensus object hash
4. ADR-0004 — signature algorithm
5. ADR-0005 — address encoding

ADR-0005 may approve the Bech32m envelope while leaving exact payload
derivation to a later specification record.

## 11. Sources Consulted

Language/runtime/tooling:

- Rust Book and Cargo documentation;
- Go documentation, fuzzing, and race detector;
- TypeScript BigInt documentation;
- Node.js event loop and worker threads;
- npm dependency security and provenance documentation.

Serialization:

- RFC 8949;
- official Protocol Buffers encoding and non-canonical serialization warning;
- Borsh specification.

Hashing:

- NIST FIPS 180-4 and hash-function project;
- RFC 7693;
- BLAKE3 specification repository.

Signatures:

- RFC 8032;
- NIST FIPS 186-5;
- RFC 6979;
- W3C Web Cryptography Level 2;
- RFC 8812.

Addresses:

- BIP 173;
- BIP 350;
- Bitcoin Base58Check developer reference;
- CashAddr reference;
- Multibase specification.

Exact URLs are recorded in each ADR.

## 12. Commands and Versions

Commands actually run:

```text
git status --short --branch
git log -1 --format=...
git remote -v
rustc --version
cargo --version
go version
node --version
npm --version
python3 --version
```

Observed:

- Phase 0.1 checkpoint:
  `c26aafed24d90b4b3b64ed0ef179a5d59f6bbcc5`;
- remote: none;
- Go: 1.25.5 darwin/amd64;
- Node.js: 20.17.0;
- npm: 10.8.3;
- Python: 3.13.3;
- Rust/Cargo: not installed;
- no remote configured.

The previously untracked scratch files `Plain text` and `Plain text.md` were
not present in the working tree during the final Phase 0.2 check. This task did
not delete or modify them.

## 13. Experiments

No prototype or benchmark was created.

Not tested:

- language build speed, binary size, throughput, memory, or Codex error rate;
- serialization-library conformance;
- candidate CBOR bytes in an independent implementation;
- cryptographic library APIs or performance;
- browser/Wasm parity;
- hardware-wallet behavior;
- address typo detection or QR size.

Recommendations in these areas are based on standards, documented runtime
properties, architectural reasoning, and stated Q1 priorities—not local
measurement.

## 14. Risks

- accepting the matrix numbers as measurements;
- choosing Rust without accepting its team/iteration cost;
- creating both language layers before an authorized consumer exists;
- using generic CBOR library defaults rather than the Q1 restricted profile;
- assuming SHA-256 selection provides domain separation automatically;
- accepting permissive Ed25519 verification behavior;
- copying Bitcoin address semantics into a Q1 Bech32m envelope;
- approving algorithms before exact library conformance review.

## 15. Phase 0.2 Gate Recommendation

Recommendation:

**READY FOR EXPLICIT HUMAN DECISION**

Phase 0.2 should remain open until Yousef explicitly accepts, rejects, or sends
back ADR-0001 through ADR-0005.

M1 remains unauthorized.
