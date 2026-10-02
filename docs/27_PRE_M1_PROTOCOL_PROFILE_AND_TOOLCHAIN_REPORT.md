# Q1 Pre-M1 Protocol Profile and Toolchain Report

Report Version: 0.1.0

Date: 2026-07-24

Status: Accepted — Bounded Finalization Follow-up Required
Gate: PRE-M1 DETERMINISM AND TOOLCHAIN READINESS

## 1. Outcome

The explicit human acceptance of ADR-0001 through ADR-0005 has been recorded.
The four required proposed normative profiles have been created, and the
stable Rust toolchain has passed an isolated smoke test.

M1 is **not authorized by this report**. The newly selected numeric registries,
limits, HRPs, strict-verification behavior, and remaining vector evidence
require human review at the gate.

Human gate decision recorded on 2026-07-24:

`PRE-M1: REPEAT — BOUNDED FINALIZATION CYCLE`

`M1: REMAINS BLOCKED`

The follow-up is limited to completing byte-level parameter choices and
independent conformance evidence in
`docs/28_PRE_M1_NORMATIVE_PARAMETER_DECISION_PACKAGE.md`.

## 2. Human Decisions Recorded

| ADR | Accepted decision | Required follow-up |
|---|---|---|
| ADR-0001 | Rust core plus bounded TypeScript/Node.js layer | Enforce ownership boundary and cross-language conformance |
| ADR-0002 | Restricted deterministic CBOR based on RFC 8949 | Approve and validate `q1-dcbor-v1` |
| ADR-0003 | SHA-256 with mandatory versioned domains | Approve domain frame and registry |
| ADR-0004 | Explicit RFC 8032-compatible Ed25519 profile | Approve strict verification and reproduce vectors |
| ADR-0005 | Q1-specific Bech32m envelope | Approve HRPs/layout and reproduce vectors in Rust |

Decision owner: Yousef Bahrami

Decision date: 2026-07-24

## 3. Profiles Created

- `docs/protocol/Q1_DETERMINISTIC_CBOR_PROFILE_V1.md`
- `docs/protocol/Q1_CRYPTOGRAPHIC_DOMAIN_REGISTRY_V1.md`
- `docs/protocol/Q1_ED25519_PROFILE_V1.md`
- `docs/protocol/Q1_ADDRESS_ENVELOPE_V1.md`

Each is marked `Proposed Normative Profile — Pre-M1 Human Approval Required`.
This preserves the accepted algorithm/architecture decisions while keeping
new wire-level choices visible.

## 4. Proposed Values Requiring Approval

### Deterministic CBOR

- records are fixed-position arrays;
- maps and tags are prohibited in V1;
- negative integers and floats are prohibited;
- consensus text defaults to printable ASCII, maximum 128 bytes;
- decoder ceilings: 16 MiB object/string, depth 16, 65,535 array items;
- unknown versions/fields and every trailing byte are rejected.

### Cryptographic domains

- binary `Q1DS` frame;
- registry version and domain ID are unsigned 16-bit big-endian;
- payload length is unsigned 64-bit big-endian;
- domain IDs `0x0001` through `0x0011` are proposed;
- transaction signing-domain selection and future Merkle profile remain
  explicitly unresolved.

### Ed25519

- pure Ed25519 only; no ctx/ph and no external prehash;
- algorithm ID `0x0001`;
- direct signing of the versioned domain frame;
- canonical, prime-subgroup public key and `R`, and `S < L`;
- separate keys for protocol roles.

### Address envelope

- HRPs `q1l`, `q1p`, `q1r`; `q1` reserved and rejected;
- envelope version `0x01`, single-key type `0x01`, Ed25519 algorithm
  `0x0001`;
- account ID is the 32-byte `ADDRESS_PAYLOAD` domain-separated SHA-256 of the
  public key;
- lowercase-only, exact 36-byte binary payload, Bech32m only.

## 5. Vector Evidence

### Reproduced by two independent standard-library implementations

Node.js `crypto` and Python `hashlib` plus an independently written Bech32m
calculation agreed on:

- empty and two-byte cryptographic-domain hash vectors;
- the address account ID;
- the `q1l`, `q1p`, and `q1r` Bech32m strings.

### Ed25519

Node.js `crypto` exactly reproduced RFC 8032 vector 1, then generated the
project-specific domain-framed vector. A second independent Ed25519
implementation has not yet reproduced the project vector. No private key was
written into the repository; the published seed is test-vector material only.

### Remaining evidence

- reproduce every vector in the selected Rust libraries;
- run the complete malformed CBOR and address rejection corpus;
- confirm strict Ed25519 subgroup behavior across the Rust core and any
  permitted cross-language verifier;
- add object-specific vectors after their schemas are separately approved.

## 6. Rust Installation

Rust was absent at the start of this work. The official rustup installer was
downloaded over TLS to a temporary path.

Installer observations:

```text
installer SHA-256:
6c30b75a75b28a96fd913a037c8581b580080b6ee9b8169a3c0feb1af7fe8caf

rustup-init:
1.29.0 (d243d7d4e 2026-03-04)
```

Installation profile:

```text
stable-x86_64-apple-darwin
minimal profile
components: rustfmt, clippy
PATH modification: disabled
```

Observed tool versions:

```text
rustc 1.97.1 (8bab26f4f 2026-07-14)
cargo 1.97.1 (c980f4866 2026-06-30)
rustfmt 1.9.0-stable (8bab26f4f6 2026-07-14)
clippy 0.1.97 (8bab26f4f6 2026-07-14)
host: x86_64-apple-darwin
```

The binaries are at `$HOME/.cargo/bin/`; the repository contains no
toolchain-generated source or build artifacts.

## 7. Isolated Smoke Test

Location:

`/private/tmp/q1-rust-smoke-20260724`

Creation:

```text
cargo new --lib --vcs none
```

This was Cargo's default library template only. It contains no Q1 protocol
logic and is not a Git repository.

| Check | Result |
|---|---|
| Project creation | Passed |
| `cargo build` | Passed |
| `cargo test` | Passed: 1 unit test, 0 failures; 0 doc tests |
| `cargo fmt --check` | Passed |
| `cargo clippy --all-targets -- -D warnings` | Passed |

All commands returned exit code 0. The execution wrapper also printed
Docker-instance warnings before commands; those warnings were unrelated to
Cargo and did not change any result.

## 8. Scope Compliance

Created:

- decision records and protocol-profile documentation;
- traceability, glossary, specification, and changelog updates;
- one disposable default Rust library outside the repository.

Not created:

- ledger or transaction implementation;
- consensus or finality implementation;
- wallet or networking implementation;
- delay-engine implementation;
- production cryptographic integration;
- a repository commit or remote push.

## 9. Handoff Checks

Executed on 2026-07-24:

| Check | Result |
|---|---|
| `python3 scripts/check_documentation.py` | Passed: 0 blocking failures |
| Requirement definitions | 391 unique; 0 duplicates |
| Decision definitions | 26 unique; 0 duplicates |
| Markdown links | 0 broken |
| macOS metadata | 0 findings |
| Built-in high-confidence secret scan | 0 findings |
| Additional `rg` credential-pattern scan | 0 findings (`rg` exit 1 means no match) |
| `git diff --check` | Passed, exit 0 |
| Changed/untracked file review | Only the documented Phase 0.2/pre-M1 specification work |
| Commit/push | Neither performed |

The documentation checker also reported non-blocking inherited findings:
27 planned/missing file-token references, 10 historical legacy-path mentions,
and 103 placeholder/open-question lines. These are visible specification
inventory, not newly hidden failures.

## 10. Next Safe Step

Human review should approve or revise each item in the four profiles'
“Decisions Required” sections. If approved, the next pre-M1 iteration should
select the minimal Rust adapters and reproduce the full corpus without
implementing ledger, transaction, consensus, wallet, networking, or delay
logic.
