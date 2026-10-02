# ADR-0003 — Hash Function

Status: Accepted
Date: 2026-07-24
Decision Owner: Yousef Bahrami
Related Decisions: DEC-Q1-003; DEC-Q1-025
Blocking: M1

## Context

Q1 needs a mature cryptographic hash for consensus object IDs, commitments,
state/transaction/receipt trees, certificates, and protocol transcripts.

This decision must not silently decide:

- address text checksum;
- non-security hash tables;
- sequential-delay construction;
- a future formal VDF.

## Decision Drivers

- security maturity, collision and preimage resistance;
- 256-bit fixed output;
- reviewed specifications and libraries;
- domain separation;
- cross-platform deterministic behavior;
- native and WebAssembly availability;
- hardware behavior and ordinary-node accessibility;
- migration/versioning.

## Options

### Option A — SHA-256

Advantages:

- long deployment history and NIST standardization;
- fixed 256-bit output;
- exceptionally broad native, browser, hardware, and cross-language support;
- easy independent-client implementation and test-vector availability.

Disadvantages:

- not intrinsically domain separated;
- length-extension properties matter if it is misused as a naive MAC or
  ambiguous concatenation;
- hardware acceleration may widen producer/attacker performance differences,
  although ordinary verification also benefits.

Risks:

- treating concatenated fields as self-delimiting instead of hashing canonical
  bytes;
- confusing consensus hashing with the delay/VDF mechanism.

### Option B — SHA3-256

Advantages:

- NIST-standardized, 256-bit output;
- sponge construction avoids SHA-256-style length extension;
- mature specifications and broad library support.

Disadvantages:

- usually less universally hardware-accelerated than SHA-256;
- browser/platform APIs may not expose it as consistently;
- may offer no practical Q1 benefit if domain separation and canonical bytes
  are already correct.

Risks:

- ecosystem/library gaps increasing custom adapters or dependencies.

### Option C — BLAKE2s-256 or BLAKE2b-256 profile

Advantages:

- RFC-specified, mature and fast in software;
- supports personalization/keyed modes;
- broad systems-language availability.

Disadvantages:

- Q1 must select exactly one variant and output profile;
- personalization features can create domain inconsistency if not specified
  byte-for-byte;
- browser/WebCrypto availability is less universal than SHA-256.

Risks:

- confusion between BLAKE2s and BLAKE2b outputs;
- cross-language libraries exposing different feature sets.

### Option D — BLAKE3-256 profile

Advantages:

- high software throughput, parallelism, XOF, and explicit context/derive-key
  modes;
- strong modern Rust/Go/JavaScript/Wasm implementations.

Disadvantages:

- newer and less standardized than NIST SHA families or RFC 7693;
- broader feature surface requires one strict Q1 profile;
- performance advantage is workload/hardware dependent and unmeasured here.

Risks:

- optimizing before Q1 knows whether hashing is a bottleneck;
- independent clients depending on fewer or less mature implementations.

## Comparative Matrix

Scale: 5 = strongest fit.

| Criterion | SHA-256 | SHA3-256 | BLAKE2 profile | BLAKE3-256 |
|---|---:|---:|---:|---:|
| Security maturity | 5 | 5 | 4 | 3 |
| Collision/preimage target | 5 | 5 | 5 | 5 |
| Fixed 256-bit simplicity | 5 | 5 | 4 | 4 |
| Cross-language libraries | 5 | 4 | 4 | 4 |
| Browser/WebAssembly access | 5 | 3 | 3 | 4 |
| Hardware availability | 5 | 3 | 4 | 4 |
| Software performance | 3 | 3 | 4 | 5 |
| Built-in domain features | 2 | 3 | 4 | 5 |
| Independent-client ease | 5 | 4 | 4 | 3 |
| Standards status | 5 | 5 | 4 | 2 |
| Migration/tool stability | 5 | 5 | 4 | 3 |

## Domain Separation

Whatever primitive is selected, Q1 must hash:

```text
HashInput =
  DomainTagLength
  || DomainTagBytes
  || ProtocolVersion
  || CanonicalObjectBytes
```

The exact grammar belongs to ADR-0002 test vectors. Domain tags must be unique
constants for transaction ID, block hash, state node, transaction tree node,
receipt tree node, address payload, consensus message, certificate, and delay
challenge.

No delimiter-free string concatenation is permitted.

## Security Impact

Hash security depends on canonical, unambiguous input and correct domain
separation. The selected library must be maintained and testable against
official vectors. Q1 must not implement the primitive itself.

## Determinism Impact

All candidates are deterministic for identical bytes. Cross-client divergence
would more likely arise from serialization, domain construction, output
encoding, or library-profile differences than from the primitive itself.

## Implementation Impact

Expose one narrow provider:

```text
hash(domain, canonical_bytes) -> Hash256
```

Do not expose keyed/XOF/personalization variants through the general consensus
interface unless separately specified.

## Testing Impact

- official algorithm vectors;
- Q1 domain vectors;
- empty/minimum/maximum canonical payloads;
- one-shot versus streaming equality;
- domain and single-byte mutation inequality;
- cross-language and WebAssembly equality;
- output text-encoding tests separate from hash computation.

## Deployment Impact

The algorithm/profile and domain registry must be versioned in genesis and
protocol metadata. Hardware acceleration must never become a validity
requirement.

## Migration Impact

Migration changes every affected identifier and commitment. A future algorithm
requires a new protocol version/activation and must not reinterpret old hashes.
Hash values need an algorithm context where they cross version boundaries.

## Distinct Uses

- **Consensus object hashing:** selected by this ADR.
- **Address checksum:** selected by ADR-0005; may use the address format's
  checksum rather than truncating the consensus hash.
- **Non-security hash tables/caches:** implementation detail; must never enter
  consensus output.
- **Delay/VDF:** DEC-Q1-025 and delay specifications; no inference from this
  ADR.

## Recommendation

Clearly labeled recommendation:

Use **SHA-256** for Q1 v0.1 consensus object hashing, wrapped by mandatory Q1
domain separation and canonical serialization.

Why:

- strongest interoperability and independent-client position;
- mature standard and broad native/browser/hardware support;
- fixed, understandable profile;
- Q1 has not demonstrated a performance need that justifies a newer or less
  universal primitive.

Preserved alternatives:

- SHA3-256 if avoiding SHA-2 construction properties is given greater weight
  than universal platform support;
- BLAKE3-256 if reproducible benchmarks later show a material bottleneck and
  independent-client/library review is satisfactory.

## Unresolved Questions

- Is universal WebCrypto/hardware availability a formal requirement?
- Should the domain tag grammar include chain ID as a field or rely on the
  canonical object's chain ID?
- Which tree construction and leaf/internal-node domains are approved?
- Are any stored hash fields required to carry an explicit algorithm ID in
  v0.1?

## Evidence and Limitations

No hash performance benchmark or library API experiment was run. Performance
scores are qualitative. No formal cryptographic review was performed.

Primary sources:

- NIST FIPS 180-4:
  https://csrc.nist.gov/pubs/fips/180-4/upd1/final
- NIST hash-function project/FIPS 202 references:
  https://csrc.nist.gov/Projects/hash-functions
- RFC 7693 BLAKE2:
  https://datatracker.ietf.org/doc/rfc7693/
- BLAKE3 specification repository:
  https://github.com/BLAKE3-team/BLAKE3-specs

## Approval Record

Accepted by Yousef Bahrami on 2026-07-24. SHA-256 is approved for
consensus-critical hashing. Every such hash must use an explicit, versioned
domain from `docs/protocol/Q1_CRYPTOGRAPHIC_DOMAIN_REGISTRY_V1.md`; raw
consensus-object hashing is prohibited.
