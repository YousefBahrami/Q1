# Q1 M1.2 Protocol Object Schemas Report

Report Version: 0.1.0

Date: 2026-07-24

Milestone: M1.2 — Protocol Object Schemas and Golden Vectors

Gate Finding: **BLOCKED BY NORMATIVE SCHEMA DECISIONS**

Recommendation: **REPEAT M1.2 REVIEW AFTER DEC-Q1-027**

## 1. Outcome

The M1.2 authorization was evaluated against the repository's approved
specifications, ADRs, cryptographic profiles, and open decisions.

No required object currently has enough approved information to define one
exact fixed-position CBOR schema without inventing protocol behavior.
Accordingly:

- no `crates/q1-protocol-types` crate was created;
- no object encoder, decoder, constructor, hash, or signing API was created;
- no `vectors/protocol_objects/v1/` directory was created;
- no candidate bytes were mislabeled as normative golden vectors;
- no ledger, transaction execution, consensus, networking, wallet, delay,
  mining, synchronization, or runtime logic was created.

This is the required stop behavior, not an implementation failure.

## 2. Approved Foundation

The following foundations are approved and reusable after the schemas are
decided:

| Foundation | Approved rule |
|---|---|
| Serialization | Restricted deterministic CBOR V1 |
| Object shape | Fixed-position arrays unless an approved schema says otherwise |
| Hash | SHA-256 over the approved Q1 domain frame |
| Signature | Pure Ed25519 over exact domain-frame bytes, strict verification |
| Amount | CBOR byte string, exactly 16 big-endian bytes |
| Height | Canonical CBOR unsigned integer through the M1.1 type |
| Hash/key/signature bytes | Exact fixed-width byte strings in containing schemas |
| Domains | IDs `0x0001` through `0x000f` in the approved registry |
| Chain binding | Chain identity is required inside replay-sensitive payloads |

These rules constrain a schema but do not supply its missing fields or
encodings.

## 3. Cross-Cutting Blockers

All eight objects are affected by one or more of these unresolved decisions:

1. **ChainId encoding:** no approved byte representation, exact length, or
   validation rule exists. ADR-0002 calls its example illustrative and says
   the ChainId type still requires separate approval.
2. **Schema field positions:** conceptual braces/lists do not establish array
   positions or exact signed/complete forms.
3. **Enum discriminants:** transaction types, participant roles/status,
   validation results, purpose codes, and several algorithm/configuration
   values lack approved numeric values.
4. **Optional fields:** memo, deactivation height, HDD commitment, lock/vesting,
   optional genesis allocations, maximum round, and optional signatures do
   not have one approved absence representation or activation rule.
5. **Collection ordering and uniqueness:** allocations, participants,
   attestations, chunks, transactions, proofs, and configuration collections
   lack complete ordering and duplicate-rejection rules.
6. **Size/resource limits:** most object fields and collections lack exact
   consensus limits.
7. **Self-reference:** several conceptual objects contain their own hash or
   ID without defining whether that field is excluded from hash input.
8. **Dependent object schemas:** proposals, certificates, genesis, and
   snapshots embed objects or commitments whose own exact forms are open.

DEC-Q1-027 records the required normative decision package.
The proposed package is now available at
`docs/32_DEC_Q1_027_SCHEMA_DECISION_PACKAGE.md`; none of its rows is approved
by its creation.

## 4. Object-by-Object Findings

### 4.1 GenesisManifest — Blocked

Known requirements include chain identifier, protocol version, declared
supply, explicit allocations, participant registry, consensus/economic/delay
parameters, and the invariant:

```text
sum(all explicit genesis allocations) = declared genesis supply
```

Domain `GENESIS` (`0x000d`) is approved.

Missing decisions:

- exact manifest field list and fixed positions;
- ChainId encoding;
- whether observational genesis timestamp is canonical and its integer unit;
- allocation entry schema, purpose-code values, lock/vesting representation,
  ordering, and duplicate-address behavior;
- whether treasury/faucet/research/reserve are ordinary explicit allocation
  entries or separate fields;
- participant-record schema and registry ordering;
- exact consensus/economic/delay configuration schemas;
- genesis hash exclusion rule, since the conceptual configuration includes
  its own hash;
- network-name treatment and all size limits.

The supply equality validator cannot be attached to a canonical manifest until
the allocation and declared-supply fields are fixed. Implementing only a
partial economic manifest would conflict with the broader genesis
requirements.

### 4.2 ParticipantRecord — Blocked

Domain `PARTICIPANT_RECORD` (`0x0008`) and a genesis-defined permissioned
private-testnet registry are approved.

The current field list is explicitly `SHOULD`-level and contains unresolved
role capabilities, optional deactivation, eligibility status, participation
score, cooldown, penalty, and protocol-version semantics. Missing:

- participant identifier derivation/width;
- node, producer, and validator key algorithm/role representation;
- role bit/discriminant values;
- activation/deactivation absence rule;
- eligibility/status values;
- whether mutable score/cooldown/penalty belong in the committed record;
- record version versus protocol version;
- chain binding, ordering, uniqueness, and size limits.

### 4.3 TransactionEnvelope — Blocked

Domains `TRANSACTION_SIGNING` (`0x0001`) and `TRANSACTION_ID` (`0x0002`) are
approved. The ledger specification explicitly states that the exact
transaction signing schema remains blocked.

Missing:

- ChainId encoding;
- numeric transaction-type values and whether one envelope covers protocol
  reward/genesis allocation as well as user transfer;
- a separately approved `FeeLimit` semantic type and canonical encoding;
- sender public-key presence versus key reference;
- exact address representation inside CBOR;
- validity-bound constraints and whether timestamp is absent;
- memo-hash activation and absence representation;
- whether transaction ID includes signature bytes;
- unsigned versus complete array positions and size limits.

DEC-Q1-009 and DEC-Q1-021 also leave commitment and fee-related dependencies
open. No balances or transfer execution were considered.

### 4.4 BlockHeader — Blocked

Domains `BLOCK_HEADER_SIGNING` (`0x0003`) and `BLOCK_ID` (`0x0004`) are
approved.

There is no approved exact BlockHeader field table. Missing:

- ChainId encoding and complete array positions;
- producer identifier/key representation;
- transaction-tree construction and empty root;
- whether receipt root is mandatory;
- state commitment construction;
- exact delay/HDD commitment presence and widths;
- whether the signed header contains a producer signature or the signature is
  outside it;
- whether Block ID includes signature bytes;
- version relationships and size limits.

No timestamp or fork-choice field was inferred.

### 4.5 BlockProposal — Blocked

Domain `PROPOSAL_SIGNING` (`0x0005`) is approved. The conceptual proposal
contains selection proofs, delay data, optional HDD data, a block body, block
hash, and producer signature.

Missing:

- exact unsigned and complete proposal forms;
- an approved proposal object-ID domain, if a distinct proposal ID is needed;
- candidate-index and producer-selection proof schema, dependent on
  DEC-Q1-005 and DEC-Q1-018;
- delay challenge/output/proof schemas, dependent on DEC-Q1-010;
- HDD optional semantics;
- block body and BlockHeader schemas;
- signature placement and all ordering/limits.

Implementing these fields would cross into unresolved selection and delay
protocol design.

### 4.6 Attestation — Blocked

Domain `ATTESTATION_SIGNING` (`0x0006`) is approved, and the conceptual signing
field list binds chain, height, round, block hash, validator, and validation
result.

Missing:

- ChainId and validator-ID encodings;
- exact versioned array positions;
- numeric validation-result values;
- committee-membership proof schema;
- whether the observational timestamp is excluded, prohibited, or included
  only in the complete object;
- unsigned/complete form relationship;
- attestation object ID, if required;
- maximum sizes.

The repository requires positive individual signatures but does not yet
provide a complete canonical Attestation schema.

### 4.7 FinalizationCertificate — Blocked

Domain `FINALIZATION_CERTIFICATE` (`0x0007`) is approved.

The stop conditions apply directly:

- signer/attestation ordering is undefined;
- duplicate signer handling is not encoded as a schema rule;
- individual attestations are only a `MAY` for the first version;
- future aggregate signatures remain possible and no V1 container choice is
  approved;
- committee root construction, weight widths, threshold representation, and
  attestation schema are unresolved;
- the conceptual certificate contains `certificate_hash`, creating an
  undefined self-exclusion rule.

No threshold calculation or finality logic was implemented.

### 4.8 SnapshotManifest — Blocked

Domain `SNAPSHOT` (`0x000e`) is approved. The networking specification says a
snapshot `MAY` contain a conceptual list, which is not a normative schema.

Missing:

- exact manifest field list and positions;
- ChainId encoding;
- chunk-manifest/chunk-hash schema, ordering, count, size, and duplicate rules;
- participant-set and protocol-parameter commitment definitions;
- finalization-certificate reference/proof representation;
- snapshot-hash self-exclusion rule;
- signer/signature requirements, if any;
- compression/content profile and resource limits.

No synchronization, state reconstruction, or snapshot service was created.

## 5. Domain Usage After Approval

The existing registry already covers:

| Object/use | Domain |
|---|---|
| Transaction unsigned signing form | `TRANSACTION_SIGNING` |
| Complete transaction ID | `TRANSACTION_ID` |
| Unsigned block header | `BLOCK_HEADER_SIGNING` |
| Complete signed block header ID | `BLOCK_ID` |
| Unsigned proposal | `PROPOSAL_SIGNING` |
| Unsigned attestation | `ATTESTATION_SIGNING` |
| Finalization certificate hash | `FINALIZATION_CERTIFICATE` |
| Participant record hash | `PARTICIPANT_RECORD` |
| Genesis manifest hash | `GENESIS` |
| Snapshot manifest hash | `SNAPSHOT` |

No domain ID was added. Whether proposals or attestations require separate
stored object IDs must be decided before allocating any new domain.

## 6. Objects Completed and Vector Inventory

Objects completed: **0 of 8**

Objects blocked: **8 of 8**

Normative valid golden vectors created: **0**

Normative invalid golden vectors created: **0**

Rust protocol-object vector tests created: **0**

Node/Python protocol-object conformance extensions: **0**

Creating the requested minimum vector inventory before schema approval would
freeze arbitrary choices and was therefore prohibited.

## 7. Public API and Repository Structure

No public API was added. The approved M1.1 API remains unchanged.

`crates/q1-protocol-types` is a reasonable future crate boundary because it
can depend only on `q1-primitives`, but creating an empty or speculative crate
would imply false implementation progress. Its creation is deferred until at
least one complete schema is approved.

## 8. Required Human Decision Package

For each object, approve one table containing:

1. schema version and supported value;
2. fixed array length;
3. field name at every zero-based position;
4. semantic Rust type;
5. exact CBOR type and fixed width/limit;
6. enum/discriminant values;
7. required/absent rule for every optional concept;
8. collection ordering and duplicate policy;
9. unsigned/signing form;
10. complete/stored form;
11. object-ID input and domain;
12. signature role/domain and algorithm field placement;
13. chain-binding rule;
14. malformed-input rejection codes;
15. minimum and maximum resource limits.

The package should first decide the shared ChainId type. It should then decide
the least-dependent objects before compound objects:

1. ParticipantRecord;
2. TransactionEnvelope;
3. Attestation;
4. BlockHeader;
5. GenesisManifest;
6. BlockProposal;
7. FinalizationCertificate;
8. SnapshotManifest.

This order is a recommendation for decision preparation, not protocol law.

## 9. Files Changed in This Review

- `README.md`;
- `PROJECT.md`;
- `OPEN_DECISIONS.md`;
- `CHANGELOG.md`;
- `docs/01_GLOSSARY.md`;
- `docs/22_REQUIREMENTS_TRACEABILITY_SKELETON.md`;
- `docs/31_M1_2_PROTOCOL_OBJECT_SCHEMAS_REPORT.md`.

No source, Cargo, lockfile, vector, ADR, or approved protocol-profile file was
changed.

## 10. Verification Results

No source or dependency file changed, but the complete required verification
set was rerun against the repository after this decision review.

| Command/check | Exact result |
|---|---|
| `cargo fmt --all -- --check` | Passed, exit 0 |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | Passed, exit 0 |
| `cargo test --workspace --all-targets --all-features` | Passed: 16 unit + 11 integration + 6 property = 33; 0 failed |
| `cargo doc --workspace --no-deps` | Passed, exit 0 |
| `cargo audit` | Passed, exit 0; 57 lockfile dependencies scanned against 1169 advisories; no advisory reported |
| Research `cargo test --locked` | Passed: 4 tests, 0 failed |
| Research `cargo run --locked` | Passed: 10 CBOR valid, 14 rejected, 9 Amount valid, 11 rejected |
| Research rustfmt/Clippy | Passed |
| `node node_conformance.js` | Passed; Node/OpenSSL vectors remained identical |
| `python3 python_vectors.py` | Passed; address/account and 9 Amount vectors remained identical |
| `python3 scripts/check_documentation.py` | Passed with 0 blocking failures |
| Requirement-ID scan | 392 definitions, 392 unique, 0 duplicates |
| Decision-ID scan | 27 definitions, 27 unique, 0 duplicates |
| Broken Markdown links | 0 |
| High-confidence secret patterns | 0 |
| macOS metadata | 0 |
| `git diff --check` plus explicit new-file whitespace scan | Passed |

The documentation checker continues to report inherited non-blocking
inventory: 27 planned/missing file tokens, 10 historical legacy-path mentions,
and 121 placeholder/open-decision lines.

No M1.2 protocol-object golden-vector cross-language check exists because
creating those vectors before schema approval is the blocker. Existing M1.1
cross-language evidence remains green.

## 11. Gate Recommendation

**REPEAT M1.2 REVIEW AFTER DEC-Q1-027**

M1.2 should remain authorized in bounded scope but implementation-blocked
until the human decision package freezes enough information for at least one
object. Approval can be incremental per object; it need not resolve unrelated
ledger execution or consensus algorithms, but compound schemas cannot be
approved while their embedded types remain undefined.

Do not authorize M1.3 or any runtime subsystem from this report.
