# ADR-0005 — Address Encoding

Status: Accepted
Date: 2026-07-24
Decision Owner: Yousef Bahrami
Related Decision: DEC-Q1-003
Blocking: M1

## Context

Q1 needs a human-readable address envelope that:

- identifies the intended network and address format version;
- detects common transcription errors;
- has one canonical text form;
- remains independent from one permanent signature or account model.

This ADR selects only the textual envelope and parser rules. It must not
silently decide whether an account payload is a public key, a hash of a public
key, a script, a multisignature policy, or another future account type.

## Decision Drivers

- network/version binding;
- checksum and typo detection;
- canonical case behavior;
- human readability and copy/paste safety;
- QR suitability and length;
- strict parsing;
- cross-language support;
- future cryptographic/account migration.

## Options

### Option A — Q1-specific Bech32m profile

Conceptual envelope:

```text
<network-hrp>1<version-and-payload><bech32m-checksum>
```

Advantages:

- visible network prefix before decoding;
- restricted lowercase alphanumeric character set;
- checksum covers the human-readable prefix and data;
- clear separator and canonical lowercase form;
- mature implementations and QR-friendly case-insensitive alphabet design;
- version field can separate future payload algorithms/account types.

Disadvantages:

- Q1 must specify its own data profile rather than inherit Bitcoin witness
  semantics;
- 8-to-5-bit conversion and checksum implementation are less trivial than hex;
- addresses are longer than Base58 for the same payload.

Risks:

- mixing Bech32 and Bech32m constants;
- accepting mixed case, invalid padding, overlength input, or wrong HRP;
- library APIs silently embedding Bitcoin-specific witness rules.

### Option B — Base58Check-style profile

Conceptual envelope:

```text
Base58(version || payload || checksum)
```

Advantages:

- compact familiar distributed-ledger form;
- excludes several visually ambiguous characters;
- mature libraries and hardware-wallet ecosystem.

Disadvantages:

- network/type prefix is not directly readable before decoding;
- mixed uppercase/lowercase alphabet complicates voice/manual handling;
- Q1 must select checksum function/length and canonical leading-zero rules;
- more difficult visual network separation than an explicit HRP.

Risks:

- implementation variants and leading-zero bugs;
- copied Bitcoin checksum/version conventions becoming accidental Q1 law.

### Option C — Checksummed hexadecimal with explicit prefix

Conceptual envelope:

```text
q1-private:01:<lowercase-hex-payload>:<checksum>
```

Advantages:

- simplest byte inspection and independent implementation;
- ubiquitous encoders/decoders;
- explicit network and version fields.

Disadvantages:

- longest representation;
- larger QR codes and more manual transcription;
- Q1 must design checksum and separators;
- hexadecimal alphabet has weaker human transcription ergonomics than a
  purpose-designed address alphabet.

Risks:

- ad hoc checksum design;
- inconsistent case/prefix/whitespace normalization.

### Option D — CashAddr-style prefix + payload

Advantages:

- explicit network prefix and strong error-detection design;
- base32-style human and QR properties;
- mature use in another distributed-ledger ecosystem.

Disadvantages:

- Q1 would need a new profile and careful separation from Bitcoin Cash type
  semantics;
- smaller independent ecosystem than Bech32/Bech32m.

Risks:

- library assumptions about Bitcoin Cash payload/type sizes;
- unnecessary new profile complexity compared with Bech32m.

### Option E — Multibase envelope

Advantages:

- encoding identifies its base and permits future base choices;
- broad content-addressing ecosystem.

Disadvantages:

- no address checksum or network binding by itself;
- needs additional framing and validation;
- format flexibility conflicts with Q1's desire for one canonical user form.

Assessment:

Useful for generic identifiers, not recommended as the complete Q1 address
format.

## Comparative Matrix

Scale: 5 = strongest fit.

| Criterion | Bech32m profile | Base58Check | Checksummed hex | CashAddr profile | Multibase envelope |
|---|---:|---:|---:|---:|---:|
| Visible network binding | 5 | 3 | 5 | 5 | 1 alone |
| Version binding | 5 | 4 | 5 | 5 | 3 |
| Checksum/error detection | 5 | 4 | depends on design | 5 | 1 alone |
| Canonical case | 5 lowercase | 3 mixed | 5 lowercase | 5 lowercase | depends |
| Human readability | 5 | 4 | 3 | 5 | 3 |
| QR suitability | 4 | 5 compact | 2 | 4 | 4 |
| Length | 4 | 5 | 2 | 4 | 4 |
| Copy/paste safety | 5 | 4 | 4 | 5 | 3 |
| Parser strictness clarity | 5 | 4 | 5 if specified | 5 | 3 |
| Cross-language support | 5 | 5 | 5 | 4 | 4 |
| Crypto/account migration | 5 | 4 | 5 | 5 | 5 |
| Avoids custom checksum | 5 | 4 inherited | 1 | 4 | 1 alone |

## Proposed Bech32m Profile Requirements

If selected:

- lowercase only; mixed or uppercase input rejected rather than normalized;
- network-specific HRPs, initially proposed but not approved:
  - `q1dev`
  - `q1local`
  - `q1priv`
- one address-format version in the data section;
- payload-type/algorithm identifier separate from the text encoding;
- exact payload length defined per version/type;
- Bech32m checksum constant for Q1 versions;
- strict separator, character, padding, total-length, and checksum validation;
- no whitespace trimming, Unicode normalization, typo correction, or
  auto-network conversion;
- decoder returns typed `(network, version, payload_type, payload_bytes)`;
- wallet displays network independently before signing.

The HRP strings and binary layout remain examples until human approval.

## Security Impact

The checksum detects input errors; it does not authorize spending. Wallets must
never silently repair invalid input or change networks. Unicode lookalikes,
invisible characters, mixed case, wrong HRP, wrong version, and invalid padding
must fail.

## Determinism Impact

Each valid payload must have exactly one canonical lowercase address. Internal
protocol state should store typed binary identifiers, not address text.
Encode(decode(address)) must reproduce the exact input.

## Implementation Impact

Use one small reviewed codec or independently verified implementations. Do not
reuse Bitcoin witness-program parsing logic as Q1 address semantics.

ADR-0004 determines initial key type; a later account-payload decision
determines derivation. The address envelope only carries a versioned payload.

## Testing Impact

- published valid/invalid vectors for every HRP/version/type;
- one-character substitutions and adjacent transpositions;
- wrong checksum and wrong network;
- mixed/uppercase, whitespace, invisible Unicode, lookalikes;
- invalid characters, separator count/position, padding, and length;
- unknown version/type;
- minimum/maximum payloads;
- cross-language and QR round-trip tests;
- wallet confirmation showing network and canonical address.

## Deployment Impact

Development, local, and private networks need visually distinct HRPs.
Production/mainnet HRP is not authorized. APIs may transport addresses as
strings but nodes must decode into typed bytes before validation.

## Migration Impact

Version/type fields permit future signature/account algorithms without
reinterpreting old addresses. Existing addresses remain valid only under their
original network and version rules. A text-format change should use a new
version/HRP policy, not accept multiple canonical forms silently.

## Recommendation

Clearly labeled recommendation:

Adopt a **Q1-specific Bech32m address envelope** with an explicit network HRP,
address-format version, payload-type identifier, strict lowercase canonical
form, and no automatic correction.

Why:

- strongest visible network separation;
- mature checksum design and cross-language libraries;
- clean versioning without fixing the account/signature model forever;
- good human and QR behavior with a smaller design burden than a custom
  checksummed format.

Preserved alternative:

Base58Check remains credible if hardware-wallet interoperability and shorter
display length are valued above visible network prefixes and lowercase
canonicality.

## Unresolved Questions

- Final HRPs for development, local, and private test networks
- Exact version/type bit layout
- Account payload derivation after ADR-0004
- Desired payload size and maximum address length
- Hardware-wallet support requirements
- Whether payment requests use the same address text unchanged

## Evidence and Limitations

No address library, QR encoder, hardware wallet, typo corpus, or cross-language
implementation was tested. Comparative checksum and QR assessments are
qualitative. No production-network prefix is proposed.

Primary sources:

- BIP 173 Bech32:
  https://bips.dev/173/
- BIP 350 Bech32m:
  https://bips.dev/350/
- Bitcoin Base58Check reference:
  https://developer.bitcoin.org/reference/transactions.html
- CashAddr reference:
  https://reference.cash/protocol/blockchain/encoding/cashaddr
- Multibase specification:
  https://github.com/multiformats/multibase

## Approval Record

Accepted by Yousef Bahrami on 2026-07-24, with the Q1-specific envelope in
`docs/protocol/Q1_ADDRESS_ENVELOPE_V1.md` required before M1 may begin.
Bech32m supplies only the human-readable encoding and checksum basis; Bitcoin
witness-address semantics are not inherited.
