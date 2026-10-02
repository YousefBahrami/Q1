# Q1 Address Envelope V1

Version: 0.1.0

Envelope version: proposed `0x01`

Status: Normative — Approved 2026-07-24

Authority: ADR-0005

Encoding basis: Bech32m checksum and alphabet from BIP 350

## 1. Scope

This is a Q1-defined payload carried by a Bech32m text envelope. It does not
inherit Bitcoin witness versions, script programs, or address semantics.

## 2. Network HRPs

| Network | Proposed HRP | V1 behavior |
|---|---|---|
| Localnet | `q1l` | Enabled |
| Private testnet | `q1p` | Enabled |
| Research network | `q1r` | Enabled |
| Future public network | `q1` | Reserved; parsing SHALL reject until separately activated |

An implementation configured for one network SHALL reject every address with
a different HRP, even if its checksum and payload are otherwise valid.

## 3. Binary Payload

Before 8-to-5-bit conversion, the V1 payload is:

```text
envelope_version : u8       = 0x01
address_type     : u8       = 0x01  # single Ed25519 account
algorithm_id     : u16be    = 0x0001
account_id       : bytes32
```

Total binary payload length is exactly 36 bytes.

For address type `0x01`:

```text
account_id =
    Q1HashV1(ADDRESS_PAYLOAD, ed25519_public_key_32_bytes)
```

The public key must first satisfy the Q1 Ed25519 profile. Address text is an
identifier, not proof of key possession.

Unknown envelope versions, address types, and algorithm IDs are rejected. A
future type or algorithm uses a new registered value; payload lengths are
type-specific and are never guessed.

V1 has no flags or reserved bytes. Ignored zero fields would create
non-canonical interpretations; a future layout uses a new envelope version.

The full digest gives approximately 128-bit generic collision security. A
20-, 24-, or 28-byte truncation would provide approximately 80-, 96-, or
112-bit generic collision security respectively. No truncation is recommended.
Using the raw public key would not shorten this envelope and would disclose
the key immediately. Hashing is pseudonymity, not privacy: later public-key
disclosure still links the key and address.

## 4. Bech32m Encoding

1. Select the network HRP.
2. Convert the complete 36-byte payload from 8-bit groups to 5-bit groups,
   preserving order and using zero padding only in the final group.
3. Append the six-character Bech32m checksum using constant `0x2bc830a3`.
4. Emit `<hrp>1<data><checksum>` in lowercase ASCII.

The parser SHALL reject non-zero padding, more than four padding bits, or any
decoded length other than the exact type length. Bech32 checksum
`0x00000001` is invalid; only Bech32m is accepted.

## 5. Canonical Text and Parser Rejection

Only lowercase is canonical and accepted. Although the base specifications
can represent all-uppercase strings, Q1 rejects uppercase and mixed case.

Reject:

- leading/trailing/internal whitespace or Unicode;
- characters outside printable ASCII;
- uppercase or mixed case;
- missing, extra, or misplaced separator;
- HRP outside the registry or wrong for the configured network;
- reserved `q1` HRP before activation;
- invalid checksum or Bech32 rather than Bech32m checksum;
- invalid alphabet character;
- invalid 5-to-8-bit padding;
- decoded payload not exactly 36 bytes;
- unknown version, type, or algorithm;
- an algorithm/type combination not registered together;
- total encoded length greater than 90 characters;
- any string altered by normalization, correction, or truncation.

Software SHALL NOT automatically correct an address. It may report a
human-readable error, but submission requires the exact canonical text.

## 6. Human, QR, and Copy/Paste Rules

- UI SHALL display the HRP/network and full address before signing or sending.
- UI SHALL preserve lowercase and use a monospace-capable display where
  practical.
- Truncated display is permitted only as a secondary view; confirmation must
  make the full value inspectable.
- QR payload is the canonical lowercase address as ASCII/UTF-8 text, with no
  URI prefix in V1.
- Clipboard handling SHALL reject hidden whitespace and Unicode lookalikes
  rather than silently stripping or substituting them.
- A scanned address is subjected to the same parser and wrong-network checks
  as typed text.

## 7. Test Vectors

### 7.1 Required Valid Vectors

Using the Ed25519 public key and account ID generated from the project vector
in the Ed25519 profile, publish one exact address for each enabled HRP:

```text
public key:
03a107bff3ce10be1d70dd18e74bc0996
7e4d6309ba50d5f1ddc8664125531b8

account ID:
8db5455cb537a01b61de9fccde4beee
46959a216d116be1a9d66d4d40bb5b38f

binary payload:
010100018db5455cb537a01b61de9fccde4beee46959a216d116be1a9d66d4d40bb5b38f

q1l:
q1l1qyqsqqvdk4z4edfh5qdkrh5len0yhmhyd9v6y9k3z6lp48tx6n2qhddn3ujwz45w

q1p:
q1p1qyqsqqvdk4z4edfh5qdkrh5len0yhmhyd9v6y9k3z6lp48tx6n2qhddn3u6q78rg

q1r:
q1r1qyqsqqvdk4z4edfh5qdkrh5len0yhmhyd9v6y9k3z6lp48tx6n2qhddn3u4puyfx
```

Node.js and Python standard-library implementations independently reproduced
the domain hash and all three Bech32m strings on 2026-07-24. The selected Rust
experiment independently reproduced them on 2026-07-24 as well.

### 7.2 Mandatory Invalid Variants

For every valid vector, reject:

- one data-character substitution;
- one checksum-character substitution;
- adjacent-character transposition where checksum verification fails;
- uppercase and mixed-case forms;
- prepended/appended ASCII space and newline;
- wrong-network HRP with an otherwise correctly recomputed checksum;
- HRP `q1`;
- Bech32 checksum constant;
- removed/extra data character;
- non-zero or excessive padding;
- version `0x00` or `0x02`;
- address type `0x00` or `0x02`;
- algorithm ID `0x0000` or `0x0002`;
- 35-byte and 37-byte decoded payloads.

The isolated Node strict parser exercised wrong network, checksum mutation,
uppercase, whitespace, unsupported version, unsupported algorithm, incorrect
length, and a correctly checksummed data sequence with invalid padding. All
were rejected.

Exact rejection examples:

| Input | Expected rejection |
|---|---|
| valid `q1p` vector while configured for `q1l` | wrong network |
| `q1l1qyqsqqvdk4z4edfh5qdkrh5len0yhmhyd9v6y9k3z6lp48tx6n2qhddn3ujwz45q` | checksum mutation |
| `Q1l1qyqsqqvdk4z4edfh5qdkrh5len0yhmhyd9v6y9k3z6lp48tx6n2qhddn3ujwz45w` | mixed case |
| `q1l1qyqsqqvdk4z4edfh5qdkrh5len0yhmhyd9v6y9k3z6lp48tx6n2qhddn3upmp2c55` | correctly checksummed but invalid padding |
| `q1l1qgqsqqvdk4z4edfh5qdkrh5len0yhmhyd9v6y9k3z6lp48tx6n2qhddn3u04nv77` | unsupported envelope version `0x02` |
| `q1l1qyqsqq5dk4z4edfh5qdkrh5len0yhmhyd9v6y9k3z6lp48tx6n2qhddn3uu78978` | unsupported algorithm `0x0002` |
| `q1l1qyqsqqvdk4z4edfh5qdkrh5len0yhmhyd9v6y9k3z6lp48tx6n2qhddn0jr2vf` | correctly checksummed 35-byte payload |

## 8. Migration

The HRP binds network; the envelope version binds layout; address type binds
account semantics; and algorithm ID binds public-key derivation. None may be
inferred from another. A new public network, payload construction, or
cryptographic algorithm requires explicit registry updates and activation.

## 9. Approval Record

Decision rows: PREM1-ADR-001 through PREM1-ADR-012 in
`docs/28_PRE_M1_NORMATIVE_PARAMETER_DECISION_PACKAGE.md`.

All rows were approved by Yousef Bahrami on 2026-07-24. The reserved `q1` HRP
remains inactive and conveys no public-network or mainnet authorization.
