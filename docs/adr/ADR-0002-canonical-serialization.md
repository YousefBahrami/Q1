# ADR-0002 — Canonical Serialization

Status: Accepted
Date: 2026-07-24
Decision Owner: Yousef Bahrami
Related Decision: DEC-Q1-002
Blocking: M1

## Context

Transactions, blocks, signatures, certificates, delay challenges, genesis
commitments, and consensus messages require one accepted byte representation.
Equal protocol values must never hash or sign differently.

API JSON is not consensus serialization.

## Decision Drivers

- one unique byte representation;
- canonical integers, lengths, and optionals;
- explicit unknown-field behavior;
- no implicit map ordering;
- signature and hash stability;
- cross-language independent implementations;
- bounded parser complexity and fuzzability;
- understandable version evolution.

## Options

### Option A — Purpose-built Q1 canonical binary encoding

Possible restricted design:

- fixed field order;
- fixed-width or one explicitly canonical variable-width integer encoding;
- explicit length prefixes;
- tagged unions with fixed discriminants;
- no maps, floats, implicit defaults, or unknown fields;
- version at the outer object boundary.

Advantages:

- smallest semantic surface;
- exact control of rejection rules and limits;
- straightforward byte-level specification for independent clients.

Disadvantages:

- Q1 owns the full format, parser, tooling, reviews, and migration burden;
- high risk of subtle overflow, ambiguity, or evolution mistakes;
- code generation and cross-language libraries must be built or maintained.

Risks:

- “simple custom encoding” becoming under-specified;
- implementation behavior replacing the written byte grammar;
- incompatible independent parsers.

### Option B — Restricted deterministic CBOR profile

RFC 8949 defines core deterministic encoding restrictions. Base CBOR still has
features Q1 does not need.

Proposed Q1 profile if selected:

- definite lengths only;
- preferred shortest integer and length encodings;
- arrays for consensus records, not maps;
- byte strings and UTF-8 text only where the schema requires them;
- no floats, tags, indefinite items, duplicate keys, or generic simple values;
- explicit null only for a schema-defined optional field;
- strict decode that rejects any valid-CBOR-but-outside-profile input;
- version/discriminant as the first array item.

Advantages:

- IETF standard with multi-language implementations;
- deterministic encoding rules already documented;
- compact integers and byte strings;
- usable by Rust, Go, TypeScript, and independent clients.

Disadvantages:

- generic libraries may accept a much larger language than Q1 permits;
- strict profile validation may require wrapper code or re-encoding checks;
- array-position schemas are less self-describing during debugging.

Risks:

- different libraries interpreting “canonical” differently;
- accidentally enabling maps, tags, floats, or indefinite encodings;
- accepting non-preferred encodings that hash differently.

### Option C — Deterministic Protocol Buffers under constraints

Advantages:

- mature schema language and code generation;
- excellent multi-language tooling and schema evolution support;
- familiar API/service ecosystem.

Disadvantages:

- official documentation explicitly states deterministic serialization is not
  canonical;
- field order is not guaranteed and unknown-field handling blocks stable
  canonicalization;
- output may vary with schema, build, binary, or library changes.

Risks:

- hashes and signatures becoming fragile across clients or upgrades;
- a Q1 canonicalization layer duplicating the wire format and defeating the
  primary simplicity benefit.

Assessment:

Suitable for non-consensus APIs if separately approved. Not recommended for
bytes directly hashed or signed by Q1.

### Option D — Borsh-style mature deterministic format

Borsh specifies a non-self-describing canonical encoding with ordered struct
fields, little-endian integers, explicit dynamic-container lengths, and sorted
unordered containers.

Advantages:

- designed for deterministic bytes and security-sensitive state;
- small, learnable format;
- existing Rust, JavaScript/TypeScript, and other implementations.

Disadvantages:

- ecosystem breadth and implementation maturity are less uniform than CBOR or
  Protocol Buffers;
- language implementations and schema-evolution behavior require independent
  verification;
- unordered-container support is unnecessary if Q1 prohibits maps in
  consensus objects.

Risks:

- selecting an ecosystem implementation rather than normatively specifying the
  exact profile;
- library drift across languages.

## Comparative Matrix

Scale: 5 = strongest fit.

| Criterion | Purpose-built | Restricted deterministic CBOR | Strict Protobuf | Borsh-style |
|---|---:|---:|---:|---:|
| Unique byte representation | 5 if fully specified | 5 with strict profile | 2 | 5 |
| Integer canonicalization | 5 | 5 | 3 | 5 |
| Unknown-field safety | 5 fail-closed | 5 fail-closed profile | 1 | 4 |
| Optional-field clarity | 5 | 4 | 4 semantic / 2 byte | 4 |
| Map-order risk | 5 if prohibited | 5 if prohibited | 2 | 4 |
| Cross-language support | 2 | 5 | 5 | 3 |
| Signature/hash stability | 5 | 5 | 1 | 5 |
| Schema evolution | 2 | 3 | 5 semantic / 1 byte | 3 |
| Implementation complexity | 2 | 4 | 3 with constraints | 4 |
| Fuzzing surface | 5 small | 4 restricted | 2 large/unknowns | 4 |
| Independent-client feasibility | 3 | 5 | 4 APIs / 2 hashes | 4 |
| Standards maturity | 1 Q1-owned | 5 | 4 tooling, non-canonical | 3 |

## Concrete Research Object

Illustrative only; this is not an approved transaction schema:

```text
TransferVectorV1 {
  version = 1
  chain_id = h'71312d6c6f63616c2d31'  // "q1-local-1"
  nonce = 7
  amount = 100000000
  recipient = h'000102...1f'          // 32 bytes
  memo_hash = absent
}
```

Under the candidate restricted-CBOR array profile:

```text
[
  1,
  h'71312d6c6f63616c2d31',
  7,
  100000000,
  h'000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f',
  null
]
```

Candidate bytes:

```text
86
01
4a 71312d6c6f63616c2d31
07
1a 05f5e100
5820 000102030405060708090a0b0c0d0e0f
     101112131415161718191a1b1c1d1e1f
f6
```

Concatenated candidate hex:

```text
86014a71312d6c6f63616c2d31071a05f5e1005820
000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1ff6
```

This vector demonstrates test strategy only. Field order, chain-ID type,
recipient payload, and optional representation require separate specification
approval.

## Byte-Level Test-Vector Strategy

For every protocol object publish:

- semantic field table;
- canonical hex;
- object hash after ADR-0003;
- signature after ADR-0004 where applicable;
- valid decoding result;
- minimum and maximum values;
- mutation corpus with expected stable rejection codes.

Mandatory negative vectors:

- overlong integer and length encodings;
- reordered or duplicate fields;
- unknown field/discriminant/version;
- absent versus explicit-null mismatch;
- invalid UTF-8;
- truncated and trailing bytes;
- excessive length/depth;
- maps, floats, tags, or indefinite items if outside the profile;
- encode→decode→encode byte inequality.

At least two independent language implementations must reproduce vectors before
M1 exits.

## Security Impact

Canonical rejection prevents malleability and cross-client hash divergence.
Resource bounds, strict UTF-8, checked length arithmetic, recursion limits, and
fuzzing remain mandatory. A standardized base format does not make a permissive
library safe by default.

## Determinism Impact

Serialization is upstream of all hashes, signatures, IDs, roots, and
certificates. Libraries must not serialize language-native maps or default
values implicitly.

## Implementation Impact

A restricted CBOR profile requires:

- a schema layer independent of generic CBOR values;
- canonical encoder;
- strict validating decoder;
- Q1 profile conformance tests for every supported library.

A purpose-built codec requires more initial specification and cross-language
work. Protobuf may remain useful for service APIs, but not consensus hashing.

## Testing Impact

Parser fuzzing and cross-language golden vectors are release gates. Decoders
must test rejection, not merely successful round trips.

## Deployment Impact

Protocol version and serialization-profile ID must be recorded in genesis,
handshakes, stored objects, and release metadata. API JSON remains separate.

## Migration Impact

Changing canonical bytes changes transaction IDs, block hashes, signatures,
roots, stored data, and network compatibility. Upgrades need a new protocol
version and explicit activation rule; silent library upgrades may not alter
bytes.

## Recommendation

Clearly labeled recommendation:

Adopt a **Q1 Restricted Deterministic CBOR Profile** based on RFC 8949, using
schema-defined arrays and prohibiting maps, floats, tags, indefinite lengths,
and permissive unknown fields in consensus objects.

Why:

- stronger standards and cross-language position than a Q1-only format;
- materially smaller canonicalization risk than Protocol Buffers;
- better independent-client feasibility than relying on one ecosystem;
- sufficiently compact and understandable when aggressively profiled.

Preserved alternative:

A purpose-built fixed binary encoding is credible if strict-CBOR library
behavior proves inconsistent across the selected Rust/Go/TypeScript stack.
Borsh-style encoding is the secondary mature candidate if its cross-language
conformance can be demonstrated.

## Unresolved Questions

- Is array-position readability acceptable?
- Must unknown versions fail before full decoding?
- Are any consensus text strings needed beyond chain/network identifiers?
- Should the decoder verify canonicality directly or by exact re-encoding?
- Which language libraries pass the profile corpus?
- Does Q1 need an independent CDDL or equivalent schema artifact?

## Evidence and Limitations

No serialization library was installed and no executable vector was generated.
The candidate bytes above were derived directly from the RFC 8949 encoding
rules and require independent verification before approval.

Primary sources:

- RFC 8949 deterministic CBOR:
  https://www.rfc-editor.org/info/rfc8949/
- Protocol Buffers non-canonical warning:
  https://protobuf.dev/programming-guides/serialization-not-canonical/
- Protocol Buffers encoding:
  https://protobuf.dev/programming-guides/encoding/
- Borsh specification:
  https://borsh.io/

## Approval Record

Accepted by Yousef Bahrami on 2026-07-24, with the restricted deterministic
CBOR profile in `docs/protocol/Q1_DETERMINISTIC_CBOR_PROFILE_V1.md` required
before M1 may begin. Generic CBOR is not an approved substitute.
