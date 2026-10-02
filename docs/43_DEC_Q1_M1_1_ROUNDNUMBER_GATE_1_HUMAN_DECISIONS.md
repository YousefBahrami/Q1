# DEC-Q1-M1.1 RoundNumber Primitive Correction

Gate: Gate 1 — Human Decision Documentation

Review baseline: `adc7f645a8bcec396bca21a959b2ebb3c9364ce2`

Review date: 2026-07-25

Review status: **GATE 1 HUMAN DECISIONS CLOSED**

Implementation status: **NOT STARTED — NOT AUTHORIZED**

Runtime status: **NOT AUTHORIZED**

Session 6 status: **NOT AUTHORIZED**

## 1. Scope

This record closes only the human decisions identified by the read-only Gate 1
engineering review of the M1.1 RoundNumber mismatch. It does not implement
RoundNumber, deprecate Rust symbols, change exports or codecs, migrate protocol
objects, or authorize runtime behavior.

The repository remains conceptually inconsistent with DEC-Q1-027 until a
separately authorized primitive correction is implemented and validated.

## 2. Approved Human Decisions

### RN-G1-001 — Primitive Definition

RoundNumber shall be a distinct nominal primitive with the intended Rust form:

```rust
pub struct RoundNumber(u32);
```

The inner value is private.

It is independent, bounded by `u32`, comparable, orderable, hashable, and free
of Slot, Height, timing, scheduler, and runtime behavior.

It shall not be an alias, composite type, Slot-derived type, or protocol
object.

### RN-G1-002 — Wire Representation

RoundNumber uses the `u32` semantic and wire value domain.

Its future canonical representation is the shortest canonical CBOR unsigned
integer accepted by the existing primitive encoding infrastructure.

The historical composite representation `[1, slot, number]` is not a
RoundNumber representation.

No CBOR redesign is authorized.

### RN-G1-003 — Initial Value

Zero is valid and is the protocol initial round.

The future primitive shall provide:

```text
RoundNumber::ZERO
```

`Default` may also be supported only if it is explicitly documented and tested
as equal to `ZERO`.

No increment, advancement, timeout, or arithmetic API is authorized.

### RN-G1-004 — Slot Independence

RoundNumber has no semantic, structural, mathematical, serialization,
scheduling, ordering-context, or conversion dependency on Slot.

No Slot field, Slot constructor argument, implicit conversion, derivation, or
shared wire representation is permitted.

### RN-G1-005 — Historical Round

Historical composite Round is classified as:

**DEPRECATED LEGACY COMPATIBILITY-ONLY**

It is not an approved consensus primitive.

It may remain temporarily only to avoid an unreviewed public API break or
preserve an identified historical compatibility obligation. It shall not be
used by new code or new consensus APIs.

It shall not be aliased or silently transformed into RoundNumber.

Its removal requires a later explicit human Gate.

### RN-G1-006 — Historical Slot

Historical Slot is classified as:

**DEPRECATED LEGACY NON-CONSENSUS DATA TYPE**

It has no approved active consensus role.

Its presence does not authorize or imply scheduling, protocol time, wall-clock
mapping, leader selection, timeout behavior, or runtime work.

It may remain only for the same bounded compatibility period as historical
Round. No future purpose is reserved for Slot by this decision.

### RN-G1-007 — Compatibility Policy

Compatibility is staged and bounded.

During the compatibility period:

- RoundNumber is the sole approved active round primitive.
- Round and Slot may remain only as deprecated legacy exports.
- Their old codecs may remain unchanged solely if compatibility retention is
  required.
- No new use may be introduced.

The compatibility period ends at the first separately authorized
legacy-removal Gate after the correction is implemented and validated.

### RN-G1-008 — Migration Policy

Migration stages are:

1. human decision documentation;
2. separately authorized primitive correction;
3. consumer and compatibility audit;
4. separately authorized legacy-removal Gate.

No protocol object shall be migrated during the primitive correction.

### RN-G1-009 — Test Policy

Future normative tests shall cover standalone RoundNumber behavior,
boundaries, canonical encoding, malformed input rejection, ordering, zero
semantics, and Slot independence.

Legacy Round and Slot tests may remain only if explicitly labeled as legacy.

The historical vector `83010903` must never be described as a RoundNumber
representation.

No protocol-object, consensus, runtime, scheduler, delay, networking, or API
tests may be changed under primitive-only authorization.

### RN-G1-010 — Documentation Policy

Future authorized correction work shall synchronize active primitive reports,
glossary, changelog, open decisions, project status, migration notes, and
traceability.

Completed DEC-Q1-027 Session reports remain historical and must not be
substantively rewritten. Only narrow implementation-status addenda may later
be added where explicitly authorized.

## 3. Protected Non-Change Inventory

Gate 1 does not authorize changes to:

- any Rust source, test, export, trait, parser, codec, or conversion;
- Cargo manifests, lockfiles, dependencies, or build configuration;
- protocol-object schemas or their documents;
- BlockHeader, BlockProposal, Attestation, Certificate, Genesis, or Snapshot;
- Merkle structures, StateRoot, DelayEngine, DelayEvidence, or HDD behavior;
- consensus, scheduler, timeout, transition, selection, or runtime behavior;
- networking, API, ledger, wallet, account, fee, reward, or transaction logic;
- canonical CBOR infrastructure or profile;
- domain registry, hashing, signatures, addresses, or cryptography;
- completed DEC-Q1-027 Session reports.

## 4. Gate Status

Gate 1 human decisions RN-G1-001 through RN-G1-010 are closed.

The M1.1 primitive correction is not implemented. The existing public
`Round { slot, number }` and `Slot(u64)` remain unchanged in code, and the
standalone RoundNumber primitive does not yet exist.

Implementation requires a separate explicit human authorization. Runtime use,
protocol-object migration, legacy removal, Session 6, staging, commit, tag,
remote configuration, and push are not authorized by this record.

## 5. Gate 2 Implementation-Status Addendum — 2026-07-25

Gate 2 separately authorized and completed the narrow primitive correction in
the working tree for human review:

- independent `RoundNumber(u32)` is implemented with private storage,
  `RoundNumber::ZERO`, `new`, `get`, and `Default == ZERO`;
- canonical encoding is a standalone shortest CBOR unsigned integer bounded to
  `u32`;
- RoundNumber has no Slot field, constructor input, conversion, derivation, or
  shared historical wire representation;
- historical Round is deprecated legacy compatibility-only;
- historical Slot is deprecated legacy non-consensus data;
- both historical codecs remain unchanged and the vector `83010903` remains
  only a legacy Round vector;
- normative and property tests cover the approved primitive behavior.

No protocol object was migrated. Legacy removal still requires a later Gate.
Runtime use, Session 6, staging, commit, amend, tag, remote configuration, and
push remain unauthorized.
