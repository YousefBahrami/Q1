# Q1 M1.1 Security and API Review

Report Version: 0.1.0

Date: 2026-07-24

Milestone: M1.1 — Protocol Primitives

Review Gate: Final Security and API Review

Recommendation: **APPROVE M1.1 CHECKPOINT**

## 1. Scope and Authority

This review covers only `crates/q1-primitives`, its tests, its approved
normative profiles, the root Rust workspace dependency graph, and the isolated
Pre-M1 conformance artifacts.

No M1.2, ledger, transaction execution, consensus, networking, wallet, delay,
governance, AI runtime, or deployment work was performed. No commit or push
was performed.

## 2. Reviewed Files

Implementation:

- `crates/q1-primitives/Cargo.toml`;
- every file under `crates/q1-primitives/src/`;
- `crates/q1-primitives/tests/primitives.rs`;
- `crates/q1-primitives/tests/properties.rs`;
- root `Cargo.toml`, `Cargo.lock`, and `rust-toolchain.toml`.

Normative and decision sources:

- ADR-0001 through ADR-0005;
- `docs/protocol/Q1_DETERMINISTIC_CBOR_PROFILE_V1.md`;
- `docs/protocol/Q1_CRYPTOGRAPHIC_DOMAIN_REGISTRY_V1.md`;
- `docs/protocol/Q1_ED25519_PROFILE_V1.md`;
- `docs/protocol/Q1_ADDRESS_ENVELOPE_V1.md`;
- `docs/28_PRE_M1_NORMATIVE_PARAMETER_DECISION_PACKAGE.md`;
- `docs/29_M1_1_PROTOCOL_PRIMITIVES_REPORT.md`.

Conformance:

- every intended source/vector file under `research/pre_m1_conformance/`;
- `scripts/check_documentation.py`.

## 3. Public API Findings

The public modules and root re-exports are limited to approved M1.1
primitives, codecs, framing, cryptographic adapters, errors, and four small
traits. No forbidden runtime type or module is present.

Findings:

- `Amount`, `Height`, `Slot`, and `Round` keep their fields private.
- `Address` keeps every envelope field private. Construction occurs through a
  validated public key or strict expected-network parsing.
- Public-key construction validates exact length, point decoding, canonical
  recompression, and weak/small-order rejection.
- Signature bytes are syntactically fixed-width; contextual point/scalar
  validity is enforced by strict verification.
- Semantic hash wrappers have no conversions into one another. Each accepts
  only an exact 32-byte digest representation or exact 64-character lowercase
  hexadecimal text.
- `Ed25519PrivateKey` is an intentionally narrow seed placeholder. It has no
  `Clone`, `Copy`, `Display`, serialization, persistence, wallet, keystore, or
  public seed-byte accessor. `Debug` is redacted and `Drop` overwrites the
  internal array on a best-effort basis.
- Raw Ed25519 `sign`, `verify`, and `verify_strict` remain public because the
  M1.1 authorization explicitly requires those primitive adapters and RFC
  vectors. Both public verification entry points use strict semantics. They
  must not be treated as typed application/protocol-object APIs; framed
  protocol use is provided by `sign_domain` and `verify_domain_strict`.

One accidental exposure was found and corrected: raw SHA-256 was public even
though ADR-0003 permits only a domain-framed consensus-facing provider.
`sha256::hash_raw` is now private and retained solely for the internal NIST
empty-message vector. The only public SHA-256 operation is
`sha256::hash_domain`.

## 4. Invariant Findings

### Gate 2 RoundNumber correction addendum — 2026-07-25

The original review findings remain historical. The authorized correction adds
an independent `RoundNumber(u32)` with private storage, `ZERO`, numeric
ordering, standalone shortest canonical CBOR unsigned encoding, and strict
`u32` decoding bounds. It has no Slot field or conversion.

Historical Round and Slot are deprecated under narrow compatibility policy.
Their structures and canonical codecs remain unchanged. No protocol-object,
consensus, scheduler, timing, or runtime behavior was added.

### 4.1 Amount

- Internal representation is exactly `u128`.
- Canonical encoding is always `0x50` followed by `to_be_bytes()`, exactly 17
  bytes total.
- Decoding through `decode_bytes` rejects CBOR integers, negative integers,
  tags, floats, indefinite strings, wrong types, wrong byte-string lengths,
  and trailing bytes.
- Addition, subtraction, multiplication, and division use checked integer
  operations. Division by zero is structured as `DivisionByZero`.
- No floating-point type or conversion exists in the crate.
- Widening conversions exist only from `u8`, `u16`, `u32`, and `u64`.
  Narrowing to `u64` is explicit and rejects overflow.
- No `Fee`, `Reward`, `Supply`, `Balance`, or conversion between semantic
  monetary types exists.

### 4.2 Height, Slot, and Round

- `Height::checked_increment` rejects `u64::MAX`.
- `Slot` and `Round` expose no arithmetic, clock conversion, scheduling, or
  wall-clock semantics.
- Their fields are private and their Rust types are distinct, so they cannot
  be implicitly interchanged.
- Round decoding requires exactly the approved three-element versioned array
  and rejects a round number outside `u32`.

### 4.3 Hash Wrappers

- Block, transaction, state, Merkle, and generic hashes are separate nominal
  Rust types.
- No cross-wrapper `From` implementation exists.
- Parsing requires exactly 64 lowercase hexadecimal characters.
- `Display` and `Debug` are deterministic lowercase hexadecimal encodings.
- A generic digest cannot be implicitly converted into a semantic digest.

## 5. Canonical CBOR Findings

The encoder's value model cannot express maps, tags, negative integers,
floats, or unregistered simple values. The decoder:

- accepts only approved major types 0, 2, 3, 4, and approved simple values;
- rejects prohibited major types and unapproved simple values;
- rejects non-shortest argument and length encodings;
- rejects every indefinite-length form;
- rejects trailing bytes;
- does not normalize malformed input;
- checks total input size before decoding;
- checks byte/text length and array count before allocation;
- checks nesting before descending;
- uses checked cursor arithmetic;
- returns structured errors rather than indexing attacker-controlled data.

Targeted tests now cover declared byte-string and array limits before payload
allocation, excessive nesting, all approved invalid Amount forms, and
arbitrary byte sequences up to 4095 bytes without panic.

## 6. Cryptographic Findings

- Public Q1 hashing always constructs the approved `Q1DS` frame before
  SHA-256. Raw SHA-256 is private and used only by its standards test.
- Domain identifiers are a closed enum with the approved V1 values.
- Framing fixes magic, registry version, big-endian domain ID, big-endian
  `u64` payload length, and exact payload placement.
- Signing over protocol payloads signs the exact domain frame directly and
  does not prehash it.
- `verify` delegates to `verify_strict`; no permissive dalek verification path
  is exposed.
- Public keys reject malformed, non-canonical, identity, and weak encodings.
- Strict verification rejects malformed signatures and scalar violations.
- Tests reject wrong domain, payload mutation, signature mutation, and public
  key mutation; malformed key lengths 0/31/33/64 and signature lengths
  0/63/65 are covered.
- Cryptographic failures use structured errors and do not include keys,
  signatures, seeds, or arbitrary messages.
- The library name is not encoded in protocol identifiers, frames, addresses,
  keys, signatures, or other normative wire data.

The remaining security boundary is behavioral conformance to the approved
language-neutral strict Ed25519 profile, not permanent reliance on a library
brand.

## 7. Address Findings

The parser accepts only enabled `q1l`, `q1p`, and `q1r` HRPs. It rejects:

- reserved `q1` and every unknown HRP;
- uppercase, mixed case, whitespace, Unicode, and non-printable text;
- wrong enabled network with the distinct `AddressWrongNetwork` error;
- checksum mutation and Bech32 in place of Bech32m;
- invalid alphabet and padding;
- unsupported envelope version, address type, or algorithm ID;
- decoded payload lengths other than exactly 36 bytes.

The Rust, Node/OpenSSL, and Python executions reproduced identical account ID
and `q1l`/`q1p`/`q1r` valid vectors. Rust and Node continued to reject their
published invalid corpora.

## 8. Cargo Security Audit and Dependency Graph

Installed tool:

```text
cargo-audit-audit 0.22.2
```

Installation command that succeeded:

```text
$HOME/.cargo/bin/cargo install cargo-audit --locked
```

The first attempt using bare `cargo install cargo-audit --locked` failed
before installation because the managed escalated shell did not contain
`cargo` in `PATH`. Repeating with the explicit Cargo path succeeded.

Audit command:

```text
$HOME/.cargo/bin/cargo audit
```

Observed result:

```text
Loaded 1169 RustSec advisories
Scanned Cargo.lock for vulnerabilities (57 crate dependencies)
Exit code 0
No advisory was reported
```

No cryptographic dependency was upgraded.

Dependency commands:

```text
$HOME/.cargo/bin/cargo tree --locked
$HOME/.cargo/bin/cargo tree -d --locked
$HOME/.cargo/bin/cargo metadata --locked --format-version 1
```

Production dependencies remain limited to `ed25519-dalek 3.0.0` and
`sha2 0.11.0` plus their transitive graph. The only duplicate-version report
is development-only: `getrandom 0.3.4` and `getrandom 0.4.3`, both reached
through `proptest`/`tempfile`. No production duplicate was reported.

The first sandboxed metadata run could not resolve `static.crates.io` while
fetching platform-specific source packages and exited 101. It was rerun with
approved network access, downloaded the missing platform packages, emitted
complete format-version-1 metadata, and exited 0. `--locked` succeeded, so the
root lockfile was not changed by dependency resolution.

`cargo-audit` is a point-in-time advisory check, not a proof that dependencies
or cryptographic implementations contain no vulnerabilities.

## 9. Changes Made After Review

Narrow changes only:

1. made raw SHA-256 private and retained the official raw vector internally;
2. added direct Bech32-versus-Bech32m, reserved-HRP, version, type, algorithm,
   35-byte, and 37-byte address rejection tests;
3. added CBOR pre-allocation limit, nesting-limit, and arbitrary-input
   no-panic tests;
4. extended Ed25519 malformed-length, malformed-point, wrong-domain, and
   mutation coverage;
5. updated the M1.1 completion report to reflect the reviewed API and current
   test count;
6. created this security/API review report.

No normative profile, ADR, dependency version, or runtime architecture was
changed.

## 10. Complete Command Results

Workspace:

| Command | Exact outcome |
|---|---|
| `$HOME/.cargo/bin/cargo fmt --all -- --check` | Passed, exit 0 |
| `$HOME/.cargo/bin/cargo clippy --workspace --all-targets --all-features -- -D warnings` | Passed, exit 0 |
| `$HOME/.cargo/bin/cargo test --workspace --all-targets --all-features` | Passed: 16 unit + 11 integration + 6 property = 33; 0 failed |
| `$HOME/.cargo/bin/cargo doc --workspace --no-deps` | Passed, exit 0; public documentation generated |
| `$HOME/.cargo/bin/cargo audit` | Passed, exit 0; 57 dependencies scanned; no advisory reported |
| `git diff --check` | Passed, exit 0 |

Research conformance:

| Command | Exact outcome |
|---|---|
| `cargo test --locked` | Passed: 4 tests, 0 failed |
| `cargo run --locked` | Passed: 10 CBOR valid, 14 CBOR rejected, 9 Amount valid, 11 Amount rejected; project vectors reproduced |
| `node node_conformance.js` | Passed on Node 20.17.0 / OpenSSL 3.0.13+quic; vectors identical; invalid corpus rejected |
| `python3 python_vectors.py` | Passed on Python 3.13.3; account/address and 9 Amount vectors identical |
| `cargo fmt --check` | Passed |
| `cargo clippy --all-targets --locked -- -D warnings` | Passed |

Repository/documentation:

| Check | Exact outcome |
|---|---|
| `python3 scripts/check_documentation.py` | Exit 0; no blocking findings |
| Requirement-ID definitions | 392 definitions, 392 unique, 0 duplicates |
| Decision-ID definitions | 26 definitions, 26 unique, 0 duplicates |
| Markdown links | 0 broken |
| High-confidence secret patterns | 0 findings |
| macOS metadata | 0 findings |
| Build artifact ignore check | root and research `target/` ignored |

The documentation checker also reported inherited non-blocking inventory:
27 planned/missing file tokens, 10 historical legacy-path mentions, and 110
placeholder/open-decision lines.

## 11. Unresolved Risks and Restrictions

- The current malformed Ed25519 tests materially improve boundary coverage but
  are not an exhaustive third-party malformed-point corpus. Such a corpus
  remains recommended before a public network or independent client.
- Public raw Ed25519 primitive functions exist for the expressly authorized
  adapter/vector surface. Later application code must use typed,
  domain-framed operations and must not treat raw-message functions as
  protocol-object validity APIs.
- Best-effort in-memory overwrite of the private seed placeholder is not a
  key-custody, swap, core-dump, process-memory, or hardware isolation
  guarantee. Wallet and keystore work remains unauthorized.
- Property tests sample rather than exhaust the CBOR byte space. Dedicated
  fuzzing remains future authorized work.
- The documentation checker's inherited non-blocking missing/planned-file and
  placeholder inventory remains outside this focused review.
- Advisory databases and dependency state must be rechecked at later gates.

None of these restrictions changes an accepted ADR or normative M1.1
behavior, and none authorizes later milestone work.

## 12. Checkpoint Recommendation

**APPROVE M1.1 CHECKPOINT**

Evidence supports checkpointing the current M1.1 foundation: the confirmed
public hash exposure was removed, targeted security tests were added, all
workspace and cross-language checks pass, the locked dependency graph is
reproducible, and `cargo audit` completed without an advisory.

Human authorization is still required before staging or creating the
checkpoint commit. No remote push is authorized.
