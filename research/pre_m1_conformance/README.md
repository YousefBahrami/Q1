# Q1 Pre-M1 Conformance Research

Status: Non-protocol research artifact

Scope: byte/vector reproduction only

This directory does not implement Q1 runtime behavior and does not establish
the permanent repository architecture. It exists only to reproduce the
pre-M1 candidate vectors with isolated, pinned tools.

Test seeds and keys are public test-vector material. Never substitute
production or private-testnet keys.

## Tool versions

```text
rustc 1.97.1
cargo 1.97.1
ed25519-dalek 3.0.0
sha2 0.11.0
Node.js 20.17.0
OpenSSL used by Node.js: 3.0.13+quic
Python 3.13.3
system OpenSSL CLI: 3.6.3
```

The Node runtime reports its linked OpenSSL version directly during the test;
it may differ from the standalone `openssl` command.

## Reproduce

From this directory:

```text
$HOME/.cargo/bin/cargo test --locked
$HOME/.cargo/bin/cargo run --locked
node node_conformance.js
python3 python_vectors.py
$HOME/.cargo/bin/cargo fmt --check
$HOME/.cargo/bin/cargo clippy --all-targets --locked -- -D warnings
```

Rust and Node independently derive the same RFC 8032 and Q1 Ed25519 vector.
They do not share a cryptographic implementation: Rust uses
`ed25519-dalek`; Node uses its OpenSSL-backed `node:crypto` Ed25519 API.

Rust, Node, and Python independently reproduce domain and Bech32m vectors.
Rust and Node independently enforce the research CBOR corpus.

Rust, Node, and Python also reproduce all nine fixed-width Amount vectors.
Rust and Node reject all eleven alternate Amount representations.

## Files

- `vectors.json`: language-neutral expected values and corpus;
- `src/main.rs`: Rust/dalek, SHA-256, Bech32m, and strict-CBOR experiment;
- `node_conformance.js`: independent Node/OpenSSL and JavaScript experiment;
- `python_vectors.py`: independent SHA-256 and Bech32m reproduction.

`Cargo.lock` pins the complete Rust dependency graph after generation.
