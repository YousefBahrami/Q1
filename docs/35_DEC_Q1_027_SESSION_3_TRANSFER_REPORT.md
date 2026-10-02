# DEC-Q1-027 Session 3 Transfer Report

Report Version: 0.1.0

Review Date: 2026-07-24

Decision Owner: Yousef Bahrami

Gate Outcome: **APPROVED WITH RECORDED REVISIONS**

Implementation Authorization: **NONE**

Session 4 Authorization: **NONE**

## 1. Scope

Session 3 reviewed and recorded only:

- SCHEMA-TX-001 through SCHEMA-TX-006;
- SCHEMA-ECON-001;
- the structural validity-window, Nonce, and recipient-encoding rules attached
  to those decisions.

It did not authorize Rust structs, codecs, parsers, validators, vectors,
runtime logic, ledger execution, replay behavior, mempool behavior, fee
calculation, wallet logic, networking, consensus behavior, or Session 4.

## 2. Reviewed Decision Record

| Decision ID | Original proposal | Final human status | Approved revision |
|---|---|---|---|
| SCHEMA-TX-001 | Twelve-field TransactionBodyV1 and three-field SignedTransactionV1 | Approved with revision | rename to nine-field TransferBodyV1 and three-field SignedTransferV1 |
| SCHEMA-TX-002 | Serialize and cross-check sender address and public key | Approved with revision | serialize only sender public key; derive sender account/address |
| SCHEMA-TX-003 | ID includes complete signature-bearing envelope | Approved | TransferSigningPayload frames canonical body; TransferId hashes canonical SignedTransferV1 |
| SCHEMA-TX-004 | Distinct fixed-width FeeLimit | Approved | retain approved fixed bytes16 field shape only |
| SCHEMA-TX-005 | Reserve nullable memo position | Rejected | remove memo entirely; future memo requires a new schema version |
| SCHEMA-TX-006 | Transfer-only body with serialized TransactionType | Approved with revision | TransferBodyV1 is transfer-only and has no transaction_type field |
| SCHEMA-ECON-001 | Freeze FeeLimit and defer fee behavior | Approved | wire representation only; all economic behavior remains future work |

## 3. Approved Canonical Schema

```text
TransferBodyV1 = [
    schema_version: u16 = 1,
    chain_id: ChainId,
    sender_public_key: Ed25519PublicKey,
    recipient_address: AddressEnvelope,
    amount: Amount,
    fee_limit: FeeLimit,
    nonce: Nonce,
    valid_from_height: Height,
    valid_until_height: Height
]
```

```text
SignedTransferV1 = [
    body: TransferBodyV1,
    signature_algorithm: SignatureAlgorithm,
    signature: Ed25519Signature
]
```

Both arrays have exact fixed lengths: nine and three respectively. Unknown,
missing, or trailing fields are not accepted under the approved common rules.

## 4. Signing and Identity

```text
TransferSigningPayload =
    Q1DomainFrameV1(
        TRANSACTION_SIGNING,
        CanonicalCBOR(TransferBodyV1)
    )
```

```text
TransferId =
    Q1HashV1(
        TRANSACTION_ID,
        CanonicalCBOR(SignedTransferV1)
    )
```

The signature is excluded from the signing payload and included in TransferId.
No self-derived identifier is serialized. Existing transaction domains are
unchanged.

## 5. Removed Redundancy

- `sender_address` is removed because it is deterministically derived from the
  sole serialized sender public key under the approved address profile.
- `transaction_type` is removed because TransferBodyV1 denotes only ordinary
  value transfer; reserved protocol classes require independent schemas or
  later protocol versions.
- `memo_hash` is removed with no reserved or null position because memo
  functionality is not part of V1.

These removals ensure one canonical source of sender identity and avoid fields
whose only V1 value would be redundant or inactive.

## 6. Structural Conditions

- FeeLimit remains a semantic type distinct from Amount and uses its approved
  fixed 16-byte unsigned encoding.
- Nonce is a semantic `u64` using shortest canonical CBOR unsigned encoding.
- `valid_from_height <= valid_until_height`.
- Nonce and both validity heights are signed and included in TransferId.
- No infinite-lifetime sentinel is approved.
- Recipient consensus bytes contain only binary AddressEnvelope; Bech32m text
  and HRP strings remain outside canonical serialization.

No replay, account-state, expiration, fee, refund, or execution behavior is
implied.

## 7. Remaining Dependencies

- normative registration of previously approved pending domains;
- separately authorized M1.1 RoundNumber correction;
- lower object-specific resource limits;
- future economic decisions for all fee behavior;
- independent schemas or protocol versions for reserved transaction classes;
- an explicit Session 4 authorization before the separate Merkle decision.

No Merkle or later object decision row was reviewed in Session 3.

## 8. Files Changed

- `docs/32_DEC_Q1_027_SCHEMA_DECISION_PACKAGE.md`;
- `docs/35_DEC_Q1_027_SESSION_3_TRANSFER_REPORT.md`;
- `docs/01_GLOSSARY.md`;
- `docs/03_ARCHITECTURE.md`;
- `docs/04_LEDGER_AND_TRANSACTIONS.md`;
- `docs/09_WALLET.md`;
- `docs/22_REQUIREMENTS_TRACEABILITY_SKELETON.md`;
- `OPEN_DECISIONS.md`;
- `CHANGELOG.md`;
- `PROJECT.md`;
- `README.md`.

Previously uncommitted M1.2 decision-gate documentation remains part of the
same pending working-tree change set.

## 9. No-Implementation Confirmation

No source code, crate, struct, codec, parser, validator, vector, protocol
implementation, runtime logic, ledger execution, mempool behavior, fee
calculation, wallet logic, networking, or consensus behavior was created or
changed.

## 10. Recommendation for Session 4

Wait for explicit Session 4 authorization. Under the approved gate order,
Session 4 should address the separate Merkle decision before BlockHeader. Do
not begin Merkle review or implementation automatically.
