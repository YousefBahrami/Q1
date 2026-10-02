# ADR-0001 — Primary Language and Repository Architecture

Status: Accepted
Date: 2026-07-24
Decision Owner: Yousef Bahrami
Related Decision: DEC-Q1-012
Blocking: M0 toolchain completion and M1

## Context

Q1 needs a deterministic, security-sensitive core and productive user-facing,
developer-facing, research, and operational tooling. The entire repository
does not need to use one language.

This ADR compares:

- Option A — Rust core;
- Option B — Go core;
- Option C — TypeScript/Node.js core;
- Option D — Rust or Go core plus a TypeScript/Node.js application/tooling
  layer.

No implementation language is approved by this document.

## Decision Drivers

- memory safety;
- determinism;
- concurrency and networking;
- cryptographic and database ecosystems;
- fuzzing and testing;
- cross-platform support and build reproducibility;
- performance predictability;
- dependency risk;
- development speed and small-team maintainability;
- Codex implementation reliability;
- independent-client feasibility;
- WebAssembly support;
- npm ecosystem access;
- long-term maintenance.

## Options

### Option A — Rust core

Advantages:

- ownership and type checking can prevent broad memory and concurrency errors
  before execution;
- explicit integer types, enums, newtypes, and error handling fit deterministic
  protocol modeling;
- no garbage collector in the core runtime;
- strong native and WebAssembly targets;
- good fit for a small, auditable protocol library with carefully restricted
  unsafe/FFI use.

Disadvantages:

- highest initial learning and compiler-feedback cost;
- async runtime, serialization, storage, and framework choices can fragment
  architecture;
- dependency features, proc macros, unsafe code, and FFI enlarge review scope;
- slower first implementation is plausible for a small team.

Risks:

- overengineering through advanced type abstractions;
- false confidence that the borrow checker provides protocol correctness;
- reduced contributor availability compared with TypeScript;
- Codex may require more compile-correct iteration for ownership/lifetime
  issues.

### Option B — Go core

Advantages:

- small language surface and direct code review;
- mature standard networking/concurrency model;
- integrated unit tests, benchmarks, coverage-guided fuzzing, and race detector;
- explicit module/version tooling and relatively simple cross-compilation;
- fast iteration is favorable for a small initial team and Codex.

Disadvantages:

- garbage collection and runtime scheduling require latency/resource
  measurement;
- race detector observes executed paths; the type system does not prevent all
  shared-state races;
- fewer type-level tools for preventing interchange of protocol identifiers;
- error-heavy code can become repetitive or inconsistently wrapped.

Risks:

- goroutine scheduling or map iteration leaking into consensus behavior;
- accidental aliasing/shared mutable state;
- cgo/native database or cryptography dependencies weakening build simplicity.

### Option C — TypeScript/Node.js core

Advantages:

- fastest access to npm, web/API tooling, SDK generation, documentation,
  explorer/wallet UI, testing frameworks, and developer availability;
- strong AI-agent productivity and short edit/test cycles;
- one language can span API, CLI, web, SDK, simulation, and tooling;
- Node asynchronous I/O is productive for APIs and orchestration;
- BigInt exists as a distinct TypeScript/JavaScript numeric domain.

Disadvantages:

- compile-time types are erased; all external inputs need explicit runtime
  validation;
- `number` and `bigint` discipline must be enforced at every boundary;
- JSON does not natively provide the approved consensus integer model;
- CPU-heavy verification can block the event loop; workers introduce message,
  lifecycle, and deployment complexity;
- garbage collection and JIT/runtime behavior reduce performance
  predictability;
- npm can produce a large transitive supply-chain surface;
- deterministic serialization cannot be delegated to normal JavaScript object
  or JSON behavior.

Risks:

- accidental `number` conversion or JSON precision loss;
- consensus behavior depending on object/map ordering or runtime/library
  versions;
- unbounded synchronous parsing/cryptography blocking all node I/O;
- duplicate validation logic across static TypeScript types and runtime schemas;
- higher dependency count and install-script exposure.

Required controls if selected:

- branded/wrapped BigInt monetary and height types;
- prohibit `number` in consensus-domain arithmetic;
- protocol bytes handled through typed arrays and a dedicated canonical codec;
- strict runtime schemas at every boundary;
- pinned runtime/package manager/lockfile;
- dependency allowlist and install-script policy;
- CPU work isolated and bounded;
- cross-runtime/cross-version test vectors.

### Option D — Systems-language core plus TypeScript layer

Possible shape:

- Rust or Go owns canonical protocol types, validation, hashing, signatures,
  state transition, and consensus;
- TypeScript owns explorer, wallet presentation, SDKs, documentation tooling,
  dashboards, orchestration, and selected simulations;
- communication uses versioned APIs and published language-neutral test
  vectors;
- WebAssembly may expose selected pure verification/client functions to web
  environments after the native API is stable.

Advantages:

- strongest language fit per layer;
- npm productivity without placing JavaScript runtime behavior in consensus;
- web SDK and wallet/explorer work can proceed naturally;
- an external TypeScript client exercises independent-client assumptions.

Disadvantages:

- two toolchains, dependency policies, CI matrices, release artifacts, and
  vulnerability streams;
- generated SDK/API drift;
- FFI/Wasm/API boundary versioning and error translation;
- temptation to duplicate transaction or validation rules in TypeScript.

Risks:

- two implementations accidentally becoming competing authorities;
- cross-language BigInt/byte/optional-field mismatches;
- broader release and deployment complexity before the team can sustain it.

Required boundary:

- TypeScript may construct typed requests and verify published client-level
  proofs, but consensus validity has exactly one core authority;
- no handwritten duplicate canonical serializer;
- SDKs generated from versioned schemas where practical;
- canonical byte vectors are language-neutral;
- Wasm is an adapter artifact, not a second specification.

## Comparative Matrix

Scale: 5 = strongest fit for Q1 under this criterion; 1 = weakest. Scores are
reasoned estimates, not benchmark results.

| Criterion | Rust core | Go core | TS/Node core | Hybrid |
|---|---:|---:|---:|---:|
| Memory safety | 5 | 4 | 4 | 5 |
| Consensus determinism ergonomics | 5 | 4 | 2 | 5 |
| Concurrency safety/clarity | 4 | 4 | 3 | 4 |
| Networking productivity | 4 | 5 | 5 | 5 |
| Cryptographic ecosystem fit | 5 | 4 | 3 | 5 |
| Database ecosystem fit | 4 | 4 | 4 | 4 |
| Fuzzing | 4 | 5 | 3 | 5 |
| Testing | 4 | 5 | 5 | 5 |
| Cross-platform native support | 4 | 5 | 5 | 4 |
| Build reproducibility potential | 4 | 5 | 3 | 3 |
| Performance predictability | 5 | 4 | 2 | 5 core / 3 total |
| Low dependency risk | 3 | 4 | 2 | 2 |
| Development speed | 3 | 5 | 5 | 4 |
| Small-team maintainability | 3 | 5 | 4 | 3 |
| Codex implementation reliability | 3 | 5 | 5 | 4 |
| Independent-client feasibility | 5 | 5 | 4 | 5 |
| WebAssembly support | 5 | 2 | 5 host | 5 |
| npm ecosystem access | 1 | 1 | 5 | 5 |
| Long-term protocol maintenance | 5 | 4 | 2 | 5 core / 3 total |

The numeric matrix expresses this ADR's weighting assumptions; it is not a
measurement and should not be added arithmetically without human priorities.

## Security Impact

Rust provides the strongest compile-time native memory-safety model, but unsafe
and dependencies still require review. Go provides memory-safe ordinary code
with runtime race detection, but correctness depends more on execution tests
and concurrency discipline. TypeScript avoids native memory corruption in
ordinary code but has the largest risks around runtime validation, integer
discipline, event-loop denial of service, and dependency surface.

A hybrid design reduces consensus exposure to Node risks but increases
boundary, build, and supply-chain surface.

## Determinism Impact

All options require explicit rules for:

- canonical bytes and rejection of non-canonical inputs;
- ordered collections;
- checked integer arithmetic;
- clock isolation;
- concurrency-independent state transitions;
- stable errors and version handling.

Rust makes invalid typed states hardest to express. Go can remain deterministic
with simple pure packages and strict review. TypeScript requires the strongest
linting, runtime validation, BigInt, and byte-boundary controls.

## Implementation Impact

- Rust maximizes core assurance potential but raises initial implementation
  friction.
- Go minimizes core/toolchain friction and is the most pragmatic single
  systems-language option.
- TypeScript maximizes application-layer productivity.
- Hybrid maximizes architectural fit but must begin narrowly: one core
  toolchain plus TypeScript only when the first SDK/application layer is
  authorized.

## Testing Impact

Rust:

- unit/integration/doc tests;
- property/fuzz tooling selected as dependencies;
- compile-time concurrency checks plus runtime tests.

Go:

- integrated unit/benchmark/fuzz/race tooling;
- race tests only cover executed behavior.

TypeScript:

- unit/property/fuzz options are plentiful;
- runtime-schema, BigInt/JSON, event-loop delay, worker failure, and multiple
  Node version tests are mandatory.

Hybrid:

- all core tests plus cross-language golden vectors, generated-client
  compatibility, Wasm/native parity where used, and release-matrix tests.

## Deployment Impact

Rust and Go can produce native node binaries. Node requires a pinned runtime or
bundled distribution and dependency installation policy. Hybrid requires native
core artifacts plus application packages and coordinated version manifests.

## Migration Impact

The protocol specification, vectors, and schemas must remain
language-independent. A later core-language migration is expensive but
possible if byte-level and state-transition vectors are complete. A hybrid
boundary makes user tooling replaceable without changing consensus.

## Recommendation

Clearly labeled recommendation:

1. **Best consensus-critical core:** Rust, if Q1 accepts a slower M0/M1 ramp in
   exchange for stronger compile-time memory and state-modeling guarantees.
2. **Best pragmatic alternative core:** Go, if small-team speed, simpler
   tooling, and Codex first-pass reliability are weighted above Rust's stronger
   type/ownership guarantees.
3. **Best user/developer-facing language:** TypeScript/Node.js for wallet
   presentation, explorer, SDKs, APIs, documentation tooling, and dashboards.
4. **Best overall repository architecture:** a deliberately narrow hybrid:
   Rust core plus TypeScript application/tooling workspace, with versioned API
   boundaries and language-neutral vectors. Do not create the TypeScript layer
   until its first authorized consumer exists.

This recommendation is based primarily on architectural inference and official
tool/runtime characteristics, not local comparative benchmarks. Rust was not
installed in the review environment. If human priorities favor fastest
small-team execution, **Go core + TypeScript layer** is the credible preserved
alternative.

## Unresolved Questions

- Is the team willing to absorb Rust's learning/iteration cost?
- Which platforms and hardware wallets are required?
- Is browser-side pure verification needed in Horizon 1 or later?
- When does a TypeScript workspace become justified by an authorized consumer?
- What maximum dependency/unsafe/FFI surface is acceptable?
- Should a disposable Rust-versus-Go parser/vector spike precede approval?

## Evidence and Limitations

Local versions observed:

- Go 1.25.5;
- Node.js 20.17.0;
- npm 10.8.3;
- Rust/Cargo not installed.

No build, performance, binary-size, memory, or Codex-error-rate comparison was
run. Version-specific ecosystem packages were not selected.

Primary sources:

- Rust concurrency and ownership:
  https://doc.rust-lang.org/stable/book/ch16-00-concurrency.html
- Cargo testing:
  https://doc.rust-lang.org/cargo/commands/cargo-test.html
- Go documentation and modules:
  https://go.dev/doc/
- Go fuzzing:
  https://go.dev/doc/security/fuzz/
- Go race detector:
  https://go.dev/doc/articles/race_detector
- TypeScript BigInt:
  https://www.typescriptlang.org/docs/handbook/release-notes/typescript-3-2.html
- Node worker threads:
  https://nodejs.org/api/worker_threads.html
- Node event-loop blocking:
  https://nodejs.org/en/learn/asynchronous-work/dont-block-the-event-loop
- npm security and provenance:
  https://docs.npmjs.com/packages-and-modules/securing-your-code/

## Approval Record

Accepted by Yousef Bahrami on 2026-07-24.

Approved architecture: a strictly bounded hybrid with a Rust
consensus-critical core and TypeScript/Node.js application and
developer-facing layers. TypeScript must not independently redefine
consensus-critical serialization, hashing, signing, fee, reward, or
state-transition rules. Shared behavior must cross a controlled Rust
library/Wasm boundary or be governed by normative vectors and mandatory
cross-language conformance tests.
