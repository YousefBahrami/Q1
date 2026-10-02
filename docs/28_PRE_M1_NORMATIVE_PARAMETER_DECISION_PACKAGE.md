# Q1 Pre-M1 Normative Parameter Decision Package

Package Version: 0.1.0

Date: 2026-07-24

Status: Approved

Gate: PRE-M1 COMPLETE

M1.1: Authorized

## 1. Purpose and Decision Rule

This package records the approved byte-level choices. Yousef Bahrami approved
all 53 parameter rows on 2026-07-24. Passing conformance tests established
reproducibility; the recorded human decision established project policy.

The accepted architecture-level choices in ADR-0001 through ADR-0005 are not
reopened.

## 2. Deterministic CBOR Decision Matrix

| ID | Decision item | Recommended value | Alternatives | Reason | Security impact | Compatibility impact | Human approval |
|---|---|---|---|---|---|---|---|
| PREM1-CBOR-001 | Record model | Fixed-position arrays; first item version/discriminant | Integer-key maps; combined arrays/maps | Smallest parser surface and exact bytes | Eliminates duplicate/order ambiguity | Less self-describing; schemas mandatory | Approved — 2026-07-24 |
| PREM1-CBOR-002 | Maps and keys | Maps prohibited; map-entry limit 0 | Unsigned integer keys ordered by encoded bytes; text keys | V1 has no demonstrated map need | Removes duplicate-key and sorting attacks | Generic CBOR maps rejected | Approved — 2026-07-24 |
| PREM1-CBOR-003 | Field identity | Array position is normative | Explicit numeric field IDs | Exact schemas and compact bytes | Prevents alias/duplicate fields | Evolution requires a new object version | Approved — 2026-07-24 |
| PREM1-CBOR-004 | Unsigned integers | Allowed; shortest RFC 8949 form only | Fixed-width custom integers | Native CBOR deterministic form | Rejects malleable representations | Strict wrapper needed around permissive libraries | Approved — 2026-07-24 |
| PREM1-CBOR-005 | Negative integers | Prohibited in V1 | Permit schema-bound negative values | No current consensus quantity requires them | Reduces range/underflow cases | Later use requires a new approved schema/version | Approved — 2026-07-24 |
| PREM1-CBOR-006 | Byte strings | Allowed only by schema | Prohibit; encode as arrays/text | Required for keys, hashes, signatures | Must enforce pre-allocation lengths | Universally supported | Approved — 2026-07-24 |
| PREM1-CBOR-007 | Text strings | Exceptional; schema-declared printable ASCII `0x21..0x7e`, max 128 bytes | UTF-8+NFC; prohibit all text | Avoids normalization and locale ambiguity | Rejects controls, confusables, normalization differences | Non-ASCII identifiers unavailable in V1 | Approved — 2026-07-24 |
| PREM1-CBOR-008 | Boolean/null | `false`, `true`, and `null` only in exact schema positions | Prohibit all; numeric flags | Useful for explicit booleans/optionals without generic simple values | Position/type checking mandatory | Standard CBOR primitives | Approved — 2026-07-24 |
| PREM1-CBOR-009 | Collections | Definite length only | Permit indefinite with normalization | Unique bytes and bounded allocation | Rejects streaming/break ambiguity | Encoders must know length | Approved — 2026-07-24 |
| PREM1-CBOR-010 | Tags | Empty allowlist; all tags prohibited | Small numeric allowlist | No current semantic need | Removes hidden alternate semantics | Tagged generic values rejected | Approved — 2026-07-24 |
| PREM1-CBOR-011 | Floats/simple values | All floats prohibited; only schema-position false/true/null | Preferred float encoding | Consensus arithmetic is integer-only | Eliminates NaN, signed-zero, precision divergence | Generic numeric APIs need typed validation | Approved — 2026-07-24 |
| PREM1-CBOR-012 | Trailing bytes | Reject every trailing byte/second item | Framed stream permits multiple items | Exactly one object per payload | Prevents parser differential and smuggling | Caller must frame externally | Approved — 2026-07-24 |
| PREM1-CBOR-013 | Nesting depth | 16 levels including top level | 8; 32; 64 | Above expected schemas while bounding recursion | Limits stack/CPU exhaustion | Enforced identically in all decoders | Approved — 2026-07-24 |
| PREM1-CBOR-014 | Byte-string maximum | 16 MiB format ceiling; every schema sets a lower bound | 1 MiB; 64 MiB | Supports bounded block/snapshot artifacts without unbounded allocation | Still requires check-before-allocation | Independent clients need the same ceiling | Approved — 2026-07-24 |
| PREM1-CBOR-015 | Text maximum | 128 encoded bytes and field-specific lower limits | 64; 256; unlimited | Identifiers should remain short | Bounds validation and storage | ASCII makes byte/character length identical | Approved — 2026-07-24 |
| PREM1-CBOR-016 | Array entries | 65,535 hard ceiling; schema-specific lower limit | 4,096; 1,000,000 | Fits `u16` audit reasoning and exceeds expected records | Bounds loop/allocation work | Large collections require chunking/version change | Approved — 2026-07-24 |
| PREM1-CBOR-017 | Total canonical object | 16 MiB hard format ceiling | 1 MiB; 64 MiB | A hard parser ceiling distinct from initial ~1 MB block policy | Bounds memory amplification | Lower network/schema limits still apply | Approved — 2026-07-24 |
| PREM1-CBOR-018 | Unknown/additional fields | Reject exact-array-length mismatch and unknown values | Ignore trailing fields; preserve unknowns | Consensus must not interpret different schemas | Fail-closed evolution | Version upgrades require explicit activation | Approved — 2026-07-24 |
| PREM1-CBOR-019 | Future versions | Unknown version rejected before semantic processing | Best-effort decode | No implicit forward compatibility in consensus | Prevents premature rule activation | Coordinated version support required | Approved — 2026-07-24 |
| PREM1-CBOR-020 | API/network separation | CBOR limits govern consensus bytes only; API and network framing have separate limits | Reuse one global limit | Different trust and framing boundaries | Avoids a permissive API limit weakening consensus | API/network specs must publish their own limits | Approved — 2026-07-24 |

### Limit boundaries

- **Consensus format ceiling:** the proposed 16 MiB/depth-16/65,535-item
  limits above. These are absolute decoder ceilings.
- **Consensus object/schema limits:** always lower or equal. The existing
  experimental block recommendation of “1 MB” remains a separately configured
  block rule and must be made byte-exact before block implementation.
- **API limits:** not consensus. Recommended operational starting points are
  256 KiB request bodies and paginated responses capped at 4 MiB, subject to
  docs/16 review.
- **Network limits:** not decided here because transport framing is unresolved.
  A network message must never cause the embedded consensus object to exceed
  its schema or 16 MiB format ceiling. Transport overhead and maximum frame
  size remain docs/08 decisions.

This distinction prevents a transport or HTTP setting from becoming a
consensus rule silently.

## 3. Domain Registry Decision Matrix

### Alternatives

| Construction | Benefit | Risk |
|---|---|---|
| A — variable human-readable labels | Readable dumps | Length/normalization/case/registration ambiguity |
| B — numeric domain ID only | Compact | Weak protocol/version binding and easy cross-project reuse |
| C — fixed prefix + version + fixed-width ID | Unambiguous, compact, append-only registry | Registry governance and binary tooling required |

Recommendation: C.

| ID | Decision item | Recommended value | Alternatives | Reason | Security impact | Compatibility impact | Human approval |
|---|---|---|---|---|---|---|---|
| PREM1-DOM-001 | Magic | ASCII `Q1DS`, hex `51314453` | Longer label; no magic | Binds framing to Q1 domain separation | Reduces cross-protocol collisions | Four fixed bytes everywhere | Approved — 2026-07-24 |
| PREM1-DOM-002 | Registry version | `0x0001`, unsigned 16-bit BE | u8; varint | Fixed-width and ample migration space | Prevents reinterpretation under a new grammar | Version change is explicit | Approved — 2026-07-24 |
| PREM1-DOM-003 | Domain ID | unsigned 16-bit BE | u8; u32; strings | Compact and sufficient append-only registry | No ambiguous label encoding | Simple across Rust/TS/other clients | Approved — 2026-07-24 |
| PREM1-DOM-004 | Payload length | unsigned 64-bit BE included before payload | Exclude length; varint | Makes framing self-delimiting independent of outer transport | Prevents concatenation ambiguity | Streaming hash can emit known length first | Approved — 2026-07-24 |
| PREM1-DOM-005 | Payload placement | Exact canonical payload immediately after length | Hash of payload; nested CBOR wrapper | Minimal and byte-exact | No delimiter ambiguity | Direct streaming after header | Approved — 2026-07-24 |
| PREM1-DOM-006 | Chain/network identity | Required inside every replay-sensitive canonical payload; not duplicated in domain header | Put in header; both | Avoids per-chain domain explosion and inconsistent duplicate values | Missing chain ID is a schema failure | Schemas must carry chain identity | Approved — 2026-07-24 |
| PREM1-DOM-007 | Hash construction | `SHA-256(frame)` | Hash domain then payload; HMAC | Matches ADR-0003 and tested frame | Explicit call-site domain required | Standard SHA-256 streaming API | Approved — 2026-07-24 |
| PREM1-DOM-008 | Signature message | Sign frame bytes directly with pure Ed25519 | Sign SHA-256(frame) | Avoids an undocumented prehash variant | Domain binds role without double semantic | Message sizes bounded by schema | Approved — 2026-07-24 |
| PREM1-DOM-009 | Registration | Append-only human-approved registry; IDs never reused; ADR or profile update and vectors required | First-come implementation allocation | Traceable protocol governance | Prevents collisions/reassignment | Future clients can reject unknown IDs | Approved — 2026-07-24 |
| PREM1-DOM-010 | Reserved ranges | `0x0001..0x7fff` approved protocol; `0x8000..0xfffd` reserved; `0xfffe` research conformance only; `0xffff` invalid | No ranges | Separates protocol and test use | Prevents test domains entering consensus | Simple range checks | Approved — 2026-07-24 |

Exact frame:

```text
51314453 || 0001 || domain_id:u16be || payload_length:u64be || payload
```

The exact eight-byte prefix for domain `D` is:

```text
513144530001 || D
```

Proposed registry:

| Name | ID | Exact prefix | Purpose | Input | Output |
|---|---:|---|---|---|---|
| `TRANSACTION_SIGNING` | `0001` | `5131445300010001` | Bind a transaction signature | Canonical unsigned/signing transaction | Domain frame signed by Ed25519 |
| `TRANSACTION_ID` | `0002` | `5131445300010002` | Identify a complete transaction | Canonical signed transaction | 32-byte SHA-256 |
| `BLOCK_HEADER_SIGNING` | `0003` | `5131445300010003` | Bind producer signature | Canonical unsigned header | Domain frame signed by Ed25519 |
| `BLOCK_ID` | `0004` | `5131445300010004` | Identify a block | Canonical signed header | 32-byte SHA-256 |
| `PROPOSAL_SIGNING` | `0005` | `5131445300010005` | Bind a proposal | Canonical unsigned proposal | Domain frame signed by Ed25519 |
| `ATTESTATION_SIGNING` | `0006` | `5131445300010006` | Bind an attestation | Canonical unsigned attestation | Domain frame signed by Ed25519 |
| `FINALIZATION_CERTIFICATE` | `0007` | `5131445300010007` | Commit finality evidence | Canonical finalization certificate | 32-byte SHA-256 |
| `PARTICIPANT_RECORD` | `0008` | `5131445300010008` | Commit registry membership | Canonical participant record | 32-byte SHA-256 |
| `MERKLE_LEAF` | `0009` | `5131445300010009` | Separate tree leaves | `tree_profile_id:u16be || leaf_bytes` | 32-byte SHA-256 |
| `MERKLE_INTERNAL` | `000a` | `513144530001000a` | Separate internal nodes | `tree_profile_id:u16be || left:32 || right:32` | 32-byte SHA-256 |
| `DELAY_CHALLENGE` | `000b` | `513144530001000b` | Bind delay input | Canonical delay challenge | 32-byte SHA-256 |
| `DELAY_OUTPUT` | `000c` | `513144530001000c` | Commit delay result/proof | Canonical delay output | 32-byte SHA-256 |
| `GENESIS` | `000d` | `513144530001000d` | Identify initial state/config | Canonical genesis object | 32-byte SHA-256 |
| `SNAPSHOT` | `000e` | `513144530001000e` | Commit snapshot metadata | Canonical snapshot manifest | 32-byte SHA-256 |
| `ADDRESS_PAYLOAD` | `000f` | `513144530001000f` | Derive account identifier | Validated 32-byte Ed25519 public key | 32-byte SHA-256 |
| `CONFORMANCE_TEST` | `fffe` | `513144530001fffe` | Non-consensus research vectors | Test bytes only | 32-byte SHA-256 |

Merkle tree algorithms and object schemas are not authorized by assigning
these domains. They remain separate decisions. The canonical registry copy is
proposed in `docs/protocol/Q1_CRYPTOGRAPHIC_DOMAIN_REGISTRY_V1.md`.

## 4. Ed25519 Decision Matrix

| ID | Decision item | Recommended value | Alternatives | Reason | Security impact | Compatibility impact | Human approval |
|---|---|---|---|---|---|---|---|
| PREM1-SIG-001 | Variant | Pure Ed25519 | Ed25519ctx; Ed25519ph | ADR-0004 and broad interoperability | No implicit variant mixing | Standard fixed API | Approved — 2026-07-24 |
| PREM1-SIG-002 | Message | Exact Q1 domain frame of canonical signing payload | SHA-256(frame); raw payload | Role separation without undocumented prehash | Replay context must be in payload | Both tested implementations agree | Approved — 2026-07-24 |
| PREM1-SIG-003 | Public key | Exact 32-byte RFC 8032 compressed Edwards-y | DER/JWK wrapper | Consensus-fixed bytes | Reject length/point errors | Wrappers remain storage/API only | Approved — 2026-07-24 |
| PREM1-SIG-004 | Signature | Exact 64-byte `R || S` | DER; recoverable form | RFC encoding and fixed length | Reject alternative encodings | Universal Ed25519 form | Approved — 2026-07-24 |
| PREM1-SIG-005 | Scalar rule | Reject unless `0 <= S < L` | Reduce modulo L | Prevents scalar malleability | Required for strict validity | `verify_strict` satisfies it | Approved — 2026-07-24 |
| PREM1-SIG-006 | Point rule | Canonical `A` and `R`; reject decode failure, identity, and small-order points; allow non-small-order mixed-torsion components consistent with `ed25519-dalek 3.0.0 verify_strict` | Require prime-order subgroup; plain RFC/ZIP-215 acceptance | Matches a concrete, testable Rust API; current profile's full-subgroup wording is not implemented by dalek | Different strict profiles can split consensus; exact corpus mandatory | Independent clients must match this rule, not generic “Ed25519” | Approved — 2026-07-24 |
| PREM1-SIG-007 | Verification API | Individual `VerifyingKey::verify_strict`; no batch validity in V1 | `verify`; batch verification | Misuse-resistant explicit strict call | Rejects weak public keys/noncanonical encodings | Rust-specific adapter rule; semantic corpus remains language-neutral | Approved — 2026-07-24 |
| PREM1-SIG-008 | Key generation | Reviewed library + OS CSPRNG; fatal RNG failure; independent keys per role | Derived/shared role keys | Avoids weak/predictable/reused keys | Key compromise remains operational risk | Signer boundary required | Approved — 2026-07-24 |
| PREM1-SIG-009 | Algorithm ID | `0x0001` means this exact Q1 profile | Library name as ID; no ID | Supports migration without reinterpretation | Prevents downgrade/semantic drift | Envelope and signed objects carry ID | Approved — 2026-07-24 |
| PREM1-SIG-010 | Migration | New algorithm/profile ID plus explicit activation; no fallback | Reuse ID after library change | Old signatures retain exact meaning | Prevents downgrade and cross-algorithm confusion | Multi-version readers must dispatch explicitly | Approved — 2026-07-24 |
| PREM1-SIG-011 | Rust adapter candidate | `ed25519-dalek = 3.0.0`, defaults off, `fast` + `zeroize`; conditional on full edge corpus | `ring 0.17.14`; `ed25519-compact 2.3.1` | Explicit `verify_strict`, typed signing/verifying keys, no unsafe in ed25519-dalek, no_std-capable core, fixed API | Historical pre-2.0 oracle advisory is patched; mixed-torsion semantics must be approved | Pure Rust/Wasm-friendly; 23 locked packages in experiment | Approved — 2026-07-24 |

### Rust library evaluation

| Criterion | ed25519-dalek 3.0.0 | ring 0.17.14 | ed25519-compact 2.3.1 |
|---|---|---|---|
| Strict API visibility | Explicit `verify_strict` and weak-key checks | One verification profile; strict edge semantics less explicit in public API | Compact API; exact Q1 edge behavior still requires corpus |
| Misuse resistance | Separate signing/verifying types; zeroize default available | High-level opaque APIs | Small API, optional self-verify |
| Dependencies/build | Pure Rust; 23 locked packages in this combined SHA experiment | Native/assembly build surface | Very small/zero dependency configuration |
| no_std/Wasm | Relevant and supported; no_std not required for initial node | Platform/build constraints | Explicitly no_std/Wasm friendly |
| Security evidence | Public validation corpus; historical `<2.0.0` advisory fixed in 2.0.0; curve timing advisory fixed in 4.1.3 and candidate uses 5.0.0; no current formal audit was located in this review | Long deployment history; no Q1-specific edge proof | Formally verified field arithmetic claimed by maintainer; no Q1-specific edge proof |
| Q1 vector | Passed | Not run as Rust candidate | Not run |

Recommendation is conditional, not permanent adapter authorization. Before M1,
the selected semantics must run a full C2SP/CCTV-style malformed-point corpus
against Rust and every consensus verifier.

## 5. Independent Ed25519 Reproduction

Test-only seed:

```text
000102030405060708090a0b0c0d0e0f
101112131415161718191a1b1c1d1e1f
```

Exact message:

```text
5131445300010001000000000000000582014200ff
```

Expected public key:

```text
03a107bff3ce10be1d70dd18e74bc0996
7e4d6309ba50d5f1ddc8664125531b8
```

Expected signature:

```text
355e9ab16419b61d545e9d0e61402352
d85ea87d0335eafbf9ec0c1c9a9bcb9e
7d22645a93042a8632973d83e8fb5a83
945dd53f09e0b288723513bdcb32e908
```

| Implementation | Underlying implementation | Result | Mutated message | Mutated signature |
|---|---|---|---|---|
| Rust 1.97.1 + `ed25519-dalek 3.0.0` | curve25519-dalek 5.0.0, pure Rust | Exact key/signature; strict verify passed | Rejected | Rejected |
| Node.js 20.17.0 `node:crypto` | OpenSSL 3.0.13+quic Ed25519 | Exact key/signature; verify passed | Rejected | Rejected |

These are independently maintained implementations and do not share a binding
or cryptographic implementation. They agreed; the disagreement stop condition
was not triggered.

## 6. Address Decision Matrix

| ID | Decision item | Recommended value | Alternatives | Reason | Security impact | Compatibility impact | Human approval |
|---|---|---|---|---|---|---|---|
| PREM1-ADR-001 | HRPs | `q1l` localnet, `q1p` private, `q1r` research; `q1` reserved and rejected | Longer words; different prefixes | Short, visibly distinct, under 90 chars | Wrong-network checks remain mandatory | Same payload yields distinct checksum/address | Approved — 2026-07-24 |
| PREM1-ADR-002 | Envelope version | `u8 = 0x01` | 5-bit version; u16 | Explicit Q1 binary payload, not witness semantics | Unknown values fail closed | Easy independent parsing | Approved — 2026-07-24 |
| PREM1-ADR-003 | Address type | `u8 = 0x01` single Ed25519 account | Combined version/type bits | Separates account semantics | Prevents algorithm/type confusion | Future type gets new value | Approved — 2026-07-24 |
| PREM1-ADR-004 | Algorithm ID | `u16be = 0x0001` Q1 Ed25519 profile | u8; inferred from length | Aligns signature registry and migration | No implicit primitive inference | Two fixed bytes | Approved — 2026-07-24 |
| PREM1-ADR-005 | Reserved flags | None in V1 | One flags byte; reserved bytes | Reserved zero fields invite inconsistent validation | Smaller parser and no ignored bits | Future layout uses new version | Approved — 2026-07-24 |
| PREM1-ADR-006 | Account identifier | Full 32-byte domain-separated SHA-256 of 32-byte public key | Raw key; 20/24/28-byte truncation | Hides direct key bytes until disclosed, fixed algorithm-aware derivation, full hash security | ~128-bit collision security for 256-bit digest; no truncation loss | 36-byte envelope, 75-character address | Approved — 2026-07-24 |
| PREM1-ADR-007 | Network binding | HRP/checksum only; account ID is network-independent | Include network in account hash | Enables same account identifier under different explicit network HRPs | Parser must reject wrong HRP | Same payload cross-network vectors are simple | Approved — 2026-07-24 |
| PREM1-ADR-008 | Encoding | Bech32m constant `0x2bc830a3`, six checksum characters | Bech32; Base58Check | ADR-0005 and tested checksum | Detects common transcription errors; no correction | Standard Bech32m primitives, Q1 payload semantics | Approved — 2026-07-24 |
| PREM1-ADR-009 | Bit conversion | 8-to-5 in order, zero-pad final group; decoder rejects nonzero padding or >4 leftover bits | No padding; length prefix | Standard unambiguous conversion | Rejects alternate encodings | Common Bech32 libraries can expose raw conversion | Approved — 2026-07-24 |
| PREM1-ADR-010 | Case/whitespace | Lowercase ASCII only; reject uppercase, mixed case, and all whitespace | Accept all-uppercase; trim | One canonical text form, no hidden cleanup | Prevents normalization/copy ambiguity | Some generic Bech32 decoders need stricter wrapper | Approved — 2026-07-24 |
| PREM1-ADR-011 | Length | Exactly 36 decoded bytes and maximum 90 text characters | Variable payload; larger maximum | Exact V1 layout and base compatibility | Rejects truncation/extension | Current addresses are 75 characters | Approved — 2026-07-24 |
| PREM1-ADR-012 | Failure behavior | Reject checksum, unsupported fields, reserved HRP, wrong network; never auto-correct | Warn-only or correction | Fail closed for value transfer | Prevents cross-network submission | UI must surface explicit error | Approved — 2026-07-24 |

Account identifier analysis:

- raw public key keeps 32 bytes but exposes key material immediately and ties
  addresses directly to one key encoding;
- full SHA-256 keeps the same 32-byte size, provides approximately 128-bit
  generic collision security, and supports typed algorithm/version fields;
- a 20-byte truncation would provide approximately 80-bit generic collision
  security, 24 bytes approximately 96 bits, and 28 bytes approximately 112
  bits. No truncation is recommended because the address-length saving is not
  worth lowering collision security;
- hashing is pseudonymity, not privacy. Public-key disclosure in a transaction
  still links the key to the account identifier;
- the envelope identifies a single-key account in V1 and does not authorize a
  broader account model.

## 7. Conformance Evidence and Commands

Artifacts:

`research/pre_m1_conformance/`

Exact commands:

```text
$HOME/.cargo/bin/cargo test --locked
$HOME/.cargo/bin/cargo run --locked
node node_conformance.js
python3 python_vectors.py
$HOME/.cargo/bin/cargo fmt --check
$HOME/.cargo/bin/cargo clippy --all-targets --locked -- -D warnings
$HOME/.cargo/bin/cargo tree --locked
```

Observed:

- Rust: 3 tests passed, 0 failed;
- CBOR: 10 valid accepted and 14 invalid/non-canonical rejected by Rust and
  Node research validators;
- Ed25519: exact cross-implementation agreement; valid accepted; mutated
  message and signature rejected;
- domain/address: Rust, Node, and Python agreed;
- Node address parser rejected wrong network, checksum mutation, uppercase,
  whitespace, unsupported version, unsupported algorithm, incorrect length,
  and invalid padding.

Tool versions:

| Tool/library | Version |
|---|---|
| rustc | 1.97.1 (`8bab26f4f`, 2026-07-14) |
| Cargo | 1.97.1 (`c980f4866`, 2026-06-30) |
| ed25519-dalek | 3.0.0 |
| curve25519-dalek | 5.0.0 |
| sha2 | 0.11.0 |
| Node.js | 20.17.0 |
| Node-linked OpenSSL | 3.0.13+quic |
| Python | 3.13.3 |
| Standalone OpenSSL CLI | 3.6.3 |

Language-neutral vectors are in
`research/pre_m1_conformance/vectors.json`. `Cargo.lock` pins the Rust graph.

Repository checks:

```text
python3 scripts/check_documentation.py
rg <high-confidence credential patterns> .
git diff --check
git status --short
```

- 391 requirement definitions, 391 unique, 0 duplicates;
- 26 decision definitions, 26 unique, 0 duplicates;
- 0 broken Markdown links;
- 0 high-confidence secret-pattern findings;
- 0 macOS metadata findings;
- `git diff --check` passed.

The documentation checker retains 27 non-blocking planned/missing file-token
references and historical/open-question inventory. They are not conformance
failures and were not silently removed.

## 8. Human Decision Record

Every row was explicitly approved on 2026-07-24. The decision included:

1. array-only CBOR and the 16 MiB format ceilings;
2. domain numeric assignments and chain identity in payload only;
3. dalek-compatible mixed-torsion strict semantics;
4. the full 32-byte hashed account identifier and HRP set;
5. the conditional `ed25519-dalek 3.0.0` recommendation.

Gate outcome:

- `PRE-M1: COMPLETE`
- `M1: AUTHORIZED`
- implementation authority limited to `M1.1 — Protocol Primitives`
