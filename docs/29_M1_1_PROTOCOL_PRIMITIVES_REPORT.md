# Q1 M1.1 Protocol Primitives Report

Report Version: 0.1.0

Date: 2026-07-24

Status: Ready for Human Review

Milestone: M1.1 — Protocol Primitives

## 1. Outcome

The authorized `crates/q1-primitives` crate has been implemented. It contains
only deterministic protocol primitives and cryptographic/serialization
adapters.

No ledger, account state, balance, fee, reward, issuance, transaction
execution, consensus, networking, wallet, mempool, mining, delay, governance,
AI runtime, or deployment logic was created.

No commit or push is authorized by this report.

## Gate 2 Correction Addendum — 2026-07-25

The original M1.1 implementation below is preserved as historical fact. Under
DEC-Q1-M1.1 Gate 2, the primitive layer now also implements independent
`RoundNumber(u32)` with `ZERO`, standalone shortest canonical CBOR unsigned
encoding, and no Slot dependency.

Historical composite Round is deprecated legacy compatibility-only.
Historical Slot is deprecated legacy non-consensus data. Their codecs remain
unchanged, no protocol object was migrated, and legacy removal requires a
later explicit Gate.

## 2. Implemented Modules

| Module | Public responsibility |
|---|---|
| `amount` | `Amount(u128)`, checked arithmetic, safe conversions, fixed 16-byte CBOR |
| `height` | `Height(u64)`, construction, ordering, checked increment, canonical CBOR |
| `round` | Pure-data `Slot(u64)` and versioned `Round { slot, number }` |
| `hash` | Distinct immutable 32-byte Block, Transaction, State, Merkle, and Generic hashes |
| `key` | Validated Ed25519 public key, exact signature, redacted/zeroed seed placeholder |
| `address` | Strict Q1 Bech32m encode/decode for `q1l`, `q1p`, and `q1r` |
| `cbor` | Restricted deterministic encoder/decoder and resource ceilings |
| `domain` | Approved `Q1DS` framing and typed domain registry |
| `sha256` | Standard SHA-256 and domain-hash adapters |
| `ed25519` | Pure Ed25519 sign, strict verify, and domain-framed operations |
| `error` | Non-stringly, machine-readable unified error enum |
| `traits` | Minimal CanonicalEncode/Decode, DomainHash, and VerifySignature traits |

## 3. Public API Summary

Primary exported types:

```text
Amount
Height
Slot
Round
BlockHash
TransactionHash
StateHash
MerkleHash
GenericHash
Ed25519PublicKey
Ed25519Signature
Ed25519PrivateKey
Address
AddressType
Network
Error
Result
```

Primary operations:

```text
Amount::{checked_add, checked_sub, checked_mul, checked_div}
CanonicalEncode::encode_canonical
CanonicalDecode::decode_canonical
Address::{from_public_key, parse_for_network, encode}
domain::frame
sha256::hash_domain
ed25519::{public_key, sign, sign_domain, verify, verify_strict,
          verify_domain_strict}
```

The public `ed25519::verify` function intentionally delegates to
`verify_strict`; the permissive dalek verifier is not exposed as a Q1 validity
path.

## 4. Normative Amount Decision

Requirement `Q1-LTX-024` records the human decision:

- internal representation: `u128`;
- range: zero through `u128::MAX`;
- canonical CBOR: `0x50 || amount.to_be_bytes()`;
- exactly 16 big-endian payload bytes, including required leading zeros;
- CBOR unsigned-integer alternatives are always rejected;
- API JSON representation is a base-10 string, outside this crate.

Nine valid and eleven invalid Amount vectors pass in the Rust crate and
isolated research corpus. Node and Python independently reproduce the valid
vectors; Node rejects the invalid corpus.

No Balance, Fee, Reward, Supply, or other monetary type was created.

## 5. Safety Guarantees

- Workspace and crate forbid unsafe Rust.
- Monetary arithmetic uses checked operations and never wraps.
- Semantic hash types do not implicitly convert into one another.
- Hash parsing requires exactly 64 lowercase hexadecimal characters.
- CBOR rejects maps, tags, negatives, floats, indefinite lengths,
  non-shortest arguments, disallowed simple values, invalid text, excessive
  limits, truncation, and trailing bytes.
- Amount decoding requires exactly one 16-byte definite CBOR byte string.
- Domain frames have fixed magic, version, ID width, byte order, payload
  length, and placement.
- Address parsing requires lowercase printable ASCII, Bech32m, exact padding,
  exact 36-byte payload, enabled HRP, expected network, version `1`, type `1`,
  and algorithm `1`.
- The reserved public `q1` HRP is not represented by the `Network` enum.
- Public-key construction rejects malformed and weak/small-order keys.
- Signature verification uses `ed25519-dalek 3.0.0::verify_strict`.
- Private seed debug output is redacted and its in-memory array is overwritten
  on drop. This is not a key-storage guarantee.
- Every public type and invariant has Rust documentation; `missing_docs` and
  `unsafe_code` are denied by workspace lints.

## 6. Rejected Edge Cases

The executed corpus includes:

- arithmetic overflow, underflow, multiplication overflow, division by zero,
  and narrowing conversion overflow;
- maximum-height increment;
- alternative and malformed Amount encodings;
- non-shortest, indefinite, mapped, tagged, floating, duplicate-map,
  truncated, invalid-text, and trailing CBOR;
- unknown/reserved domain identifiers;
- invalid hash length and uppercase/non-hex formatting;
- short public keys/signatures, all-zero weak public key, invalid scalar,
  changed message, and changed signature;
- wrong-network address, checksum mutation, mixed/upper case, whitespace,
  invalid padding, unsupported version/algorithm, reserved HRP, and incorrect
  payload length;
- extra Round fields and out-of-range round number.

## 7. Tests

Command:

```text
$HOME/.cargo/bin/cargo test --workspace --all-targets --all-features
```

Observed result:

```text
module unit tests:      16 passed
integration/vector:     11 passed
property tests:          6 passed
doc tests:               0 run
total:                  33 passed, 0 failed
```

Property coverage includes arbitrary `u128` Amount round trips, checked-add
agreement with Rust `u128`, arbitrary `u64` Height and CBOR round trips, and
address round trips across arbitrary test seeds and enabled networks.

Cross-language research:

```text
Rust conformance tests: 4 passed
Node conformance:       passed
Python vectors:         passed
```

No line/branch coverage percentage was measured because no coverage tool is
installed. The report therefore makes no quantitative coverage claim.

## 8. Dependency Graph

Production dependencies are intentionally limited:

```text
q1-primitives
├── ed25519-dalek 3.0.0
│   ├── curve25519-dalek 5.0.0
│   ├── ed25519 3.0.0
│   ├── sha2 0.11.0
│   ├── signature 3.0.0
│   ├── subtle 2.6.1
│   └── zeroize 1.9.0
└── sha2 0.11.0
```

Development-only:

```text
proptest 1.9.0
```

The complete graph is pinned by the root `Cargo.lock`. Bech32m and restricted
CBOR do not add generic format-library dependencies.

## 9. Toolchain

```text
rustc 1.97.1 (8bab26f4f 2026-07-14)
cargo 1.97.1 (c980f4866 2026-06-30)
host: x86_64-apple-darwin
```

`rust-toolchain.toml` pins Rust 1.97.1 with rustfmt and Clippy.

## 10. Final Verification

| Check | Result |
|---|---|
| `cargo fmt --all -- --check` | Passed |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | Passed |
| `cargo test --workspace --all-targets --all-features` | Passed: 33 tests, 0 failures |
| `cargo doc --workspace --no-deps --locked` | Passed |
| Research `cargo fmt --check` | Passed |
| Research Clippy with `-D warnings` | Passed |
| Research Rust tests | Passed: 4 tests |
| Node conformance | Passed |
| Python vectors | Passed |
| `python3 -m json.tool vectors.json` | Passed |
| `python3 scripts/check_documentation.py` | Passed after removing duplicate requirement references |
| Requirement definitions | 392 unique, 0 duplicates |
| Decision definitions | 26 unique, 0 duplicates |
| Broken Markdown links | 0 |
| Secret scans | 0 findings |
| macOS metadata | 0 findings |
| `git diff --check` | Passed |

The final installed `cargo-audit` version, advisory result, dependency review,
and post-review changes are recorded in
`docs/30_M1_1_SECURITY_AND_API_REVIEW.md`.

The documentation checker continues to report 27 non-blocking planned/missing
file-token references and historical placeholder/path inventory inherited
from the specification roadmap.

## 11. Remaining Work

Within a future human-authorized primitive iteration:

- add an exhaustive external malformed-point corpus for strict Ed25519 edge
  semantics;
- add coverage tooling if a numeric threshold is approved;
- decide whether `StateHash` requires a dedicated domain before construction
  helpers are added;
- define canonical schemas for future protocol objects before encoding them.

Explicitly outside M1.1:

- every runtime subsystem and all ledger/transaction semantics;
- key generation/custody/storage and wallet behavior;
- network frames and API JSON implementation;
- Merkle algorithms despite reserved hash domains;
- any public network or mainnet behavior.
