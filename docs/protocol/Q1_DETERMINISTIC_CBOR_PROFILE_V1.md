# Q1 Deterministic CBOR Profile V1

Version: 0.1.0

Profile identifier: `q1-dcbor-v1`

Status: Normative — Approved 2026-07-24

Authority: ADR-0002

Standard basis: RFC 8949

## 1. Scope

This profile defines the only CBOR language permitted for Q1
consensus-critical objects. Generic CBOR acceptance is insufficient. API JSON
and non-consensus diagnostics are outside this profile.

An encoder or decoder is conforming only if it produces or accepts exactly the
intersection of:

1. the object's approved schema;
2. RFC 8949 core deterministic encoding requirements; and
3. the additional restrictions below.

## 2. Allowed Data Model

| CBOR major type | V1 rule |
|---|---|
| 0 — unsigned integer | Allowed only in schema-declared fields |
| 1 — negative integer | Prohibited in V1 consensus schemas |
| 2 — byte string | Allowed; definite length only |
| 3 — text string | Allowed only for explicitly declared identifier fields |
| 4 — array | Required representation for records and ordered collections |
| 5 — map | Prohibited in V1 consensus objects |
| 6 — tag | Prohibited; the V1 tag allowlist is empty |
| 7 — simple/float | Only `false`, `true`, or `null` where a schema explicitly declares that exact value; floats and all other simple values are prohibited |

Records SHALL be fixed-position arrays. The first item SHALL be a
schema-defined object version or discriminant. An optional field SHALL use the
schema's exact representation; omission and `null` are not interchangeable.

### 2.1 Amount schema rule

Q1 `Amount` is a `u128` but SHALL NOT use CBOR major type 0. Its canonical
representation is major type 2 with definite length 16 (`0x50`) followed by
the exact unsigned big-endian `u128::to_be_bytes()` payload. Leading zero bytes
are required. Every alternate CBOR type or byte-string length is rejected.

## 3. Canonical Encoding

- Integer arguments and all length arguments SHALL use the shortest RFC 8949
  encoding.
- All strings and arrays SHALL use definite lengths.
- Indefinite-length items and the break code are prohibited.
- Maps are prohibited. Consequently map-key ordering does not arise in V1, and
  duplicate map keys SHALL be rejected as an out-of-profile map before their
  keys are interpreted.
- Tags are prohibited. The tag allowlist is empty.
- Floating-point values, including infinities and NaNs, are prohibited.
- Reserved and unassigned additional-information values are prohibited.
- A decoder SHALL consume exactly one top-level object and reject every
  trailing byte, including a second well-formed CBOR item.

Exact re-encoding comparison MAY be used as an additional check, but it does
not replace structural limit enforcement or explicit profile rejection.

## 4. Schema and Unknown Fields

- A record array SHALL have exactly the length declared by its schema.
- Shorter and longer arrays are invalid; longer arrays are not an extension
  mechanism.
- Unknown object versions, discriminants, enum values, and algorithm
  identifiers SHALL be rejected.
- A future extension requires a new recognized version and an explicit
  activation rule. Implementations SHALL NOT ignore unknown trailing fields.
- Integers SHALL be range-checked against their schema before conversion to an
  implementation integer type.

## 5. Text Policy

Consensus-critical text is exceptional. A schema may permit text only for a
named identifier field and must define its grammar and maximum encoded length.

The V1 default grammar is printable ASCII bytes `0x21` through `0x7e`, with a
maximum UTF-8 length of 128 bytes. Spaces, control characters, non-ASCII
Unicode, Unicode normalization, and case folding are prohibited unless a
later approved field-specific profile replaces this default. Human-facing
labels and free-form text SHALL NOT be consensus fields.

## 6. Resource Limits

These are decoder safety ceilings, not permission for every object to reach
them:

| Limit | Proposed V1 value |
|---|---:|
| Top-level encoded object | 16,777,216 bytes (16 MiB) |
| Nesting depth, including top level | 16 |
| Items in one array | 65,535 |
| One byte string | 16,777,216 bytes |
| One text string | 128 bytes unless an approved schema is stricter |

Map entries are exactly zero because maps are prohibited.

These are consensus-format ceilings only. They are not API request limits,
network-frame limits, or permission for a block to reach 16 MiB. API and
network limits remain separately governed; every consensus object schema must
remain at or below these format ceilings.

Every object schema SHALL set a smaller bound where its protocol meaning
requires one. A decoder SHALL enforce limits while reading, before allocation
or recursion can exceed them.

## 7. Core Conformance Vectors

Hex strings contain the complete encoded object.

### 7.1 Accepted

| Meaning | Canonical hex |
|---|---|
| unsigned 0 | `00` |
| unsigned 23 | `17` |
| unsigned 24 | `1818` |
| unsigned 256 | `190100` |
| empty byte string | `40` |
| byte string `00ff` | `4200ff` |
| empty text | `60` |
| text `q1` | `627131` |
| array `[1, h'00ff']` | `82014200ff` |
| array `[1, false, null]` | `8301f4f6` |

These primitives are accepted only in a schema position that permits their
type and value.

### 7.2 Rejected

| Hex | Reason |
|---|---|
| `1801` | non-shortest encoding of 1 |
| `1817` | non-shortest encoding of 23 |
| `190018` | non-shortest encoding of 24 |
| `5f4100ff` | indefinite byte string |
| `7f6171ff` | indefinite text |
| `9f01ff` | indefinite array |
| `a0` | map prohibited |
| `a201000101` | map prohibited and duplicate key |
| `c001` | tag prohibited |
| `f90000` | floating point prohibited |
| `8201` | truncated array |
| `0102` | trailing second item |
| `63713100` | NUL/control byte in text |
| `ff` | break code outside an indefinite item and prohibited globally |

Object-specific canonical and rejection vectors remain mandatory before an
object schema is implemented.

### 7.3 Amount vectors

| Amount | Canonical CBOR hex |
|---:|---|
| 0 | `5000000000000000000000000000000000` |
| 1 | `5000000000000000000000000000000001` |
| 255 | `50000000000000000000000000000000ff` |
| 256 | `5000000000000000000000000000000100` |
| `u32::MAX` | `50000000000000000000000000ffffffff` |
| `u64::MAX` | `500000000000000000ffffffffffffffff` |
| `u64::MAX + 1` | `5000000000000000010000000000000000` |
| `2^127` | `5080000000000000000000000000000000` |
| `u128::MAX` | `50ffffffffffffffffffffffffffffffff` |

Amount decoding rejects:

| Hex | Reason |
|---|---|
| `40` | zero-length byte string |
| `4100` | one-byte representation |
| `480000000000000000` | eight-byte representation |
| `4f000000000000000000000000000000` | 15-byte representation |
| `510000000000000000000000000000000000` | 17-byte representation |
| `00` | CBOR integer zero |
| `1bffffffffffffffff` | CBOR integer `u64::MAX` |
| `5f5000000000000000000000000000000000ff` | indefinite byte string |
| `d84050000000000000000000000000000000` | tagged 16-byte string |
| `20` | negative integer |
| `f90000` | floating point |

Rust, Node, and Python conformance artifacts reproduce the nine accepted
vectors. Rust and Node reject all eleven invalid vectors.

### 7.4 Domain-framed SHA-256

For research only, each accepted primitive above was framed under reserved
non-consensus domain `CONFORMANCE_TEST = 0xfffe`. Expected results are:

| CBOR hex | SHA-256 |
|---|---|
| `00` | `c242badf2a7053e3548d02ae01ea0800c87c71ebe4eb1dc0eec0fddb974ce436` |
| `17` | `4adcdf4071c4478536a988b3dfc473c7b7ffb76d1f4ebecba639204765f09f2b` |
| `1818` | `31f19adf028ff2c7983e1426c39e7b36ed3bf089fc69dd671331879acdc840de` |
| `190100` | `d319b70e47d2deb3cb916aa012bf5824052cf268e6c1a96937390b35ff8e668f` |
| `40` | `bf7de4aa8e9c8db991579ff730fc082adc84031ff82b630355ea83139f7c54ab` |
| `4200ff` | `5851610f6d3e5a03e85dfc61b0a7d0bab68450407967b7bf096853660d8c9e77` |
| `60` | `44065713db1ba1a081f8ffeee02f806e24a5eebd7d5995d5b1eec1f40445d29e` |
| `627131` | `66cb198dacad25ef9980b33691e262dc2c8178de2a7716bb784ffc78f24b8071` |
| `82014200ff` | `2c3e55ac9121c1cf87117a8f3b587eb9d61bc3fec2db80c6b49c3f10b6423fe4` |
| `8301f4f6` | `23fab59a3c52f6da3ebea503f13832f73591a22ba9a52dc0eb36d0c014ac8dab` |

Rust and Node research validators agreed on all 24 acceptance/rejection
results. Rust, Node, and Python reproduced the domain construction where
applicable. These scripts are research evidence, not protocol modules.

## 8. Conformance Requirements

- The Rust implementation SHALL be the consensus-critical codec owner under
  ADR-0001.
- Every other implementation SHALL run the identical byte corpus and shall
  not normalize rejected input into an accepted value.
- Fuzzing SHALL cover parse acceptance, exact bytes, resource ceilings,
  truncation, and trailing data.
- A library upgrade SHALL rerun the full corpus before release.

## 9. Approval Record

Decision rows: PREM1-CBOR-001 through PREM1-CBOR-020 in
`docs/28_PRE_M1_NORMATIVE_PARAMETER_DECISION_PACKAGE.md`.

All rows and the fixed-width Amount rule were approved by Yousef Bahrami on
2026-07-24. Future changes require explicit versioning and human approval.
