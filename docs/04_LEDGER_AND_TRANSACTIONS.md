Q1 Ledger and Transactions Specification

04_LEDGER_AND_TRANSACTIONS.md

Project: Q1 Experimental Distributed Ledger
Protocol Version: 0.1
Document Version: 0.1.0
Status: Draft for Engineering Review
Classification: Experimental — Not for Production or Financial Use

⸻

1. Purpose

This document defines the ledger model, native test asset, accounts, balances, transaction structure, canonical serialization, signatures, nonces, fees, receipts, state transitions, transaction lifecycle, and double-spend protection for Q1 v0.1.

The objectives of this specification are to ensure that:

* every honest node interprets transactions identically;
* balances are created and changed only through protocol rules;
* transaction identifiers are deterministic;
* transaction signatures are independently verifiable;
* replay and double-spend attempts are rejected;
* state transitions are atomic and reproducible;
* transaction status is understandable to users;
* the first prototype remains simple enough to implement, inspect, and test.

⸻

2. Ledger Model

Q1-LTX-001 — Account-based ledger

Q1 v0.1 MUST use an account-based ledger model.

Each account SHALL be represented by a unique address and a deterministic account state.

The minimum account state is:

AccountState
├── address
├── balance
├── nonce
└── account_version

The initial account model MUST NOT include executable code, smart-contract storage, delegation, or arbitrary metadata.

⸻

Q1-LTX-002 — Rationale

The account-based model is selected for Q1 v0.1 because it is:

* easier for ordinary users to understand;
* simpler for wallet balance display;
* suitable for sequential nonce protection;
* straightforward for fee calculation;
* easier to inspect during early testing;
* sufficient for native value transfer.

This choice is not permanent.

A future protocol version MAY introduce:

* UTXO-based transfers;
* hybrid accounts;
* contract accounts;
* multi-asset accounts;
* privacy-preserving balances.

⸻

3. Native Test Asset

Q1-LTX-003 — Temporary asset identity

The native asset of Q1 v0.1 SHALL use the temporary technical identity:

Asset Name: Q1 Test Unit
Asset Symbol: Q1T
Base Unit Symbol: q1u

These names are implementation placeholders and MUST NOT be treated as final branding.

⸻

Q1-LTX-004 — Integer representation

All balances, fees, rewards, and transfer amounts MUST be represented using unsigned integers.

Floating-point arithmetic MUST NOT be used in any consensus-critical financial calculation.

Recommended initial precision:

1 Q1T = 100,000,000 q1u

Therefore:

1 q1u = 0.00000001 Q1T

⸻

Q1-LTX-005 — Maximum numeric range

The implementation MUST define a maximum integer range for monetary values.

The recommended internal type is:

unsigned 128-bit integer

If the selected implementation language does not safely support native 128-bit unsigned integers, the system MUST use a reviewed arbitrary-precision integer implementation with explicit upper bounds.

Arithmetic overflow and underflow MUST cause transaction rejection or safe node failure.

They MUST NOT wrap silently.

⸻

4. Account State

Each account MUST contain:

AccountState {
    address: Address,
    balance: UInt,
    nonce: UInt64,
    account_version: UInt16
}

⸻

Q1-LTX-006 — Balance

balance represents the amount of native base units currently controlled by the account.

A balance MUST satisfy:

0 ≤ balance ≤ MAX_MONEY

Negative balances are impossible and MUST be rejected.

⸻

Q1-LTX-007 — Account nonce

nonce represents the number of successfully finalized user-originated transactions previously executed from the account.

The initial account nonce SHALL be:

0

The first valid transaction submitted from an account MUST use:

nonce = 0

After that transaction is finalized, the account nonce becomes:

1

The next valid transaction MUST therefore use:

nonce = 1

⸻

Q1-LTX-008 — Account creation

A normal account is considered to exist when:

* it receives a positive finalized balance;
* it appears in the genesis allocation;
* or it is otherwise created through an authorized future protocol transaction.

An address with no stored account state SHALL be interpreted as:

balance = 0
nonce = 0
account_version = CURRENT_ACCOUNT_VERSION

The implementation MAY avoid physically storing zero-balance unused accounts.

⸻

Q1-LTX-009 — Empty accounts

An account with:

balance = 0

and no required protocol metadata MAY be removed from active state storage.

Its historical transactions MUST remain available in block history or archival indexes.

⸻

5. Address Model

Q1-LTX-010 — Address derivation

A user address MUST be deterministically derived from the user public key and network parameters.

The normative V1 envelope and proposed derivation are defined in
`docs/protocol/Q1_ADDRESS_ENVELOPE_V1.md`. Until that profile passes the
pre-M1 gate, the following remains conceptual:

address_payload =
    HASH(
        address_domain_separator
        || network_identifier
        || public_key_type
        || public_key_bytes
    )

The displayed address SHALL contain:

* network prefix;
* encoded payload;
* integrity checksum.

⸻

Q1-LTX-011 — Network separation

Addresses MUST indicate or encode the intended network.

The proposed V1 HRPs distinguish localnet, private testnet, and research
networks. A future public-network HRP is reserved but disabled. Development,
public-testnet, and mainnet policy beyond those entries remains unapproved.

A wallet MUST warn or reject when an address belongs to a different network.

⸻

Q1-LTX-012 — Address encoding

The human-readable address encoding SHOULD:

* avoid visually ambiguous characters;
* include checksum protection;
* use one strict canonical case;
* remain short enough for ordinary use;
* support QR encoding;
* provide deterministic validation.

ADR-0005 selects a Q1-specific Bech32m envelope. Exact V1 parameters remain
subject to approval of `docs/protocol/Q1_ADDRESS_ENVELOPE_V1.md`.

⸻

Q1-LTX-013 — Public key disclosure

TransferBodyV1 includes only the sender public key. The sender address/account
is deterministically derived from that key using the approved address profile
and MUST NOT be serialized redundantly.

⸻

6. Cryptographic Domains

Every signed or hashed protocol object MUST use explicit domain separation.

The versioned binary framing and domain identifiers are defined by
`docs/protocol/Q1_CRYPTOGRAPHIC_DOMAIN_REGISTRY_V1.md`. Ad hoc string labels
such as `Q1_TX_V1` or `Q1_BLOCK_V1` are not substitutes for registry entries.
Q1_ATTESTATION_V1
Q1_ADDRESS_V1
Q1_RECEIPT_V1
Q1_STATE_V1
Q1_MERKLE_TX_V1

Domain separation prevents one type of signed object from being misinterpreted as another type.

⸻

7. Transaction Types

The following are protocol transaction classes, not a shared V1 wire
discriminant. TransferBodyV1 has no `transaction_type` field. Genesis
allocation, protocol reward, protocol penalty, and any other reserved class
require an independent schema or future protocol version.

7.1 Native transfer

Transfers native units from one user-controlled account to another.

type = TRANSFER

⸻

7.2 Genesis allocation

Creates the initial balances defined in the genesis block.

The canonical GenesisManifest is an independent protocol object and is not a
SignedBlockHeaderV1. The first signed block is height 1 and references the
governing GenesisId through ParentReferenceV1.

type = GENESIS_ALLOCATION

This transaction type MUST appear only in the genesis block.

Users MUST NOT be able to submit it.

⸻

7.3 Protocol reward

Creates protocol-authorized rewards after valid block finalization.

type = PROTOCOL_REWARD

This transaction type MUST be generated deterministically by protocol rules.

Users MUST NOT be able to sign or submit arbitrary reward transactions.

⸻

7.4 Protocol penalty

The architecture MAY reserve:

type = PROTOCOL_PENALTY

for experimental punishment or virtual slashing.

It MUST NOT be activated until exact penalty rules are defined in later specifications.

⸻

8. Native Transfer Structure

TransferBodyV1 is a fixed array of exactly nine fields:

```text
TransferBodyV1 = [
    schema_version,
    chain_id,
    sender_public_key,
    recipient_address,
    amount,
    fee_limit,
    nonce,
    valid_from_height,
    valid_until_height
]
```

SignedTransferV1 is a fixed array of exactly three fields:

```text
SignedTransferV1 = [
    body,
    signature_algorithm,
    signature
]
```

`transaction_type`, `sender_address`, and `memo_hash` are absent. They have no
reserved position or mandatory-null representation in V1.

The signing payload and identifier are:

```text
TransferSigningPayload =
    Q1DomainFrameV1(
        TRANSACTION_SIGNING,
        CanonicalCBOR(TransferBodyV1)
    )

TransferId =
    Q1HashV1(
        TRANSACTION_ID,
        CanonicalCBOR(SignedTransferV1)
    )
```

The signature is excluded from the signing payload and included in TransferId.
No self-derived identifier is serialized.

⸻

9. Transaction Field Definitions

9.1 schema_version

Identifies the transaction serialization and validation rules.

Initial value:

1

Unknown transaction versions MUST be rejected unless explicitly supported.

⸻

9.2 chain_id

Identifies the intended network.

A transaction for another chain MUST be rejected.

Example development identifiers:

q1-dev-1
q1-local-1
q1-private-1

⸻

9.3 sender_public_key

The public key used to verify the transaction signature.

The sender account/address is deterministically derived from this key. No
sender address is serialized in TransferBodyV1.

⸻

9.4 recipient_address

The destination is serialized only as the binary AddressEnvelope. No
human-readable Bech32m text or HRP string enters consensus bytes.

The recipient MAY be a previously unused valid address.

The sender and recipient MAY be the same address, but such a transaction SHOULD normally be rejected as economically meaningless unless future testing requires it.

For Q1 v0.1, self-transfers SHOULD be rejected.

⸻

9.5 amount

The number of base units transferred to the recipient.

It MUST satisfy:

amount > 0

⸻

9.6 fee_limit

FeeLimit is the distinct fixed-width semantic type approved by
SCHEMA-COMMON-021. Session 3 freezes only its 16-byte unsigned wire shape. Fee
calculation, charging, settlement, congestion, minimums, burn, issuance,
treasury, refund, and execution rules are outside this schema's authorization.

⸻

9.7 nonce

Nonce is a semantic `u64` encoded as the shortest canonical CBOR unsigned
integer. It is included in the signing payload and TransferId. Replay and
account-state behavior remain outside Session 3.

⸻

9.8 valid_from_height

The lower bound of the signed validity window.

⸻

9.9 valid_until_height

The upper bound of the signed validity window.

The structural rule is:

```text
valid_from_height <= valid_until_height
```

Both fields participate in TransferSigningPayload and TransferId. No
infinite-lifetime sentinel is approved. Expiration behavior is outside
Session 3.

⸻

9.10 signature_algorithm

Identifies the cryptographic signature scheme.

Only algorithms enabled by the protocol version MAY be accepted.

⸻

9.14 signature

The digital signature over the canonical unsigned transaction payload.

⸻

10. Unsigned Transaction Payload

The signature MUST be calculated over a canonical unsigned payload containing all transaction fields except:

* signature;
* transaction identifier;
* local wallet metadata;
* local timestamps not committed by protocol.

The exact transaction signing schema remains blocked for M1. Once approved,
its canonical unsigned bytes SHALL be framed with the selected transaction
signature domain and signed according to the Q1 Ed25519 profile. Field-list
concatenation is not a wire format.

Changing any committed field MUST invalidate the signature.

⸻

11. Canonical Serialization

Q1-LTX-014 — Deterministic bytes

The same logical transaction MUST always serialize to the exact same byte sequence.

Canonical serialization MUST define:

* field order;
* integer byte order;
* integer width or canonical variable-length encoding;
* string encoding;
* array encoding;
* optional-field representation;
* enum representation;
* maximum field lengths;
* absence representation.

⸻

Q1-LTX-015 — No ambiguous encodings

The protocol MUST reject:

* duplicate fields;
* unknown critical fields;
* non-canonical integer encodings;
* alternative encodings of the same value;
* invalid UTF-8 where strings are permitted;
* trailing unparsed bytes;
* fields outside allowed size limits.

⸻

Q1-LTX-016 — Serialization format decision

ADR-0002 selects the restricted deterministic CBOR profile in
`docs/protocol/Q1_DETERMINISTIC_CBOR_PROFILE_V1.md`. Generic CBOR and Protocol
Buffers are not approved for consensus bytes.

Human-readable JSON MUST NOT be used as the sole consensus serialization unless strict canonicalization is formally defined and tested.

⸻

Q1-LTX-024 — Fixed-width Amount encoding

`Amount` SHALL be an unsigned 128-bit protocol value in the inclusive range
zero through `2^128 - 1`.

Its one canonical CBOR representation SHALL be a definite-length byte string
containing exactly 16 bytes:

```text
amount_payload = amount.to_be_bytes()
amount_cbor    = 0x50 || amount_payload
```

The payload is unsigned, big-endian, and left zero-padded. CBOR unsigned
integers are prohibited for `Amount`, including values that fit within
`u64`. Shorter, longer, indefinite, tagged, negative, floating-point, text,
array, map, or trailing-byte alternatives SHALL be rejected without
normalization.

JSON APIs SHALL represent an `Amount` as a base-10 decimal string, never a
JSON number.

`Amount` remains semantically distinct from Fee, Reward, Supply, Balance,
raw `u128`, and byte strings. No decision in this requirement authorizes those
types or any ledger behavior.

⸻

12. Transaction Identifier

The transaction identifier SHALL be:

transaction_id =
    Q1HashV1(
        TRANSACTION_ID,
        canonical_signed_transaction_bytes
    )

The transaction ID MUST:

* be deterministic;
* change if any signed field changes;
* change if the signature changes;
* be unique with cryptographically negligible collision probability;
* be independently recomputable by every node.

Wallet-local labels MUST NOT affect the transaction ID.

⸻

13. Signature Validation

A transfer signature is valid only if all of the following are true:

1. the signature algorithm is enabled;
2. the public key encoding is valid;
3. the sender address derives from the public key;
4. the canonical unsigned payload is correctly reconstructed;
5. the signature verifies over that payload;
6. the signature is not malformed;
7. algorithm-specific validation rules pass.

A valid signature proves authorization by the holder of the corresponding private key.

It does not prove:

* the sender’s legal identity;
* lawful purpose;
* physical presence;
* ownership of a device;
* absence of coercion;
* absence of key theft.

⸻

14. Preliminary Transaction Validation

Before entering the mempool, a transaction MUST pass preliminary validation.

The order SHOULD be:

1. Decode transaction.
2. Enforce size limits.
3. Verify transaction version.
4. Verify chain identifier.
5. Verify transaction type.
6. Verify address formats.
7. Verify public key format.
8. Verify sender-address derivation.
9. Verify amount > 0.
10. Verify fee_limit ≥ minimum possible fee.
11. Verify validity window.
12. Verify nonce is not below current account nonce.
13. Verify sender balance appears sufficient.
14. Verify signature.
15. Verify transaction ID.
16. Check known duplicate status.
17. Apply local rate limits.
18. Admit or reject.

Preliminary acceptance does not guarantee final execution.

The sender state may change before inclusion.

⸻

15. Mempool Nonce Rules

For each sender, the mempool MUST organize transactions by nonce.

Example:

Current finalized account nonce: 5
Mempool transactions:
nonce 5 → executable
nonce 6 → queued
nonce 7 → queued

A gap MUST prevent later transactions from becoming executable.

Example:

nonce 5 → present
nonce 6 → missing
nonce 7 → present

nonce 7 remains queued and MUST NOT be included before nonce 6.

⸻

Q1-LTX-017 — Stale nonce

A transaction with:

transaction.nonce < account.nonce

MUST be rejected as stale or already consumed.

⸻

Q1-LTX-018 — Future nonce

A transaction with:

transaction.nonce > account.nonce

MAY be temporarily retained as a future transaction, subject to:

* per-account queue limits;
* maximum nonce gap;
* expiration;
* fee rules;
* memory constraints.

⸻

Q1-LTX-019 — Maximum nonce gap

The mempool MUST define a configurable maximum future nonce gap.

Recommended initial value:

32

Transactions beyond this gap SHOULD be rejected to prevent memory abuse.

⸻

16. Transaction Replacement

Q1 v0.1 MAY support controlled replacement of a pending transaction.

A replacement transaction MUST:

* use the same sender;
* use the same nonce;
* be validly signed;
* pay a meaningfully higher fee;
* satisfy all current validation rules.

Recommended rule:

replacement_required_fee_limit ≥
old_fee_limit + max(
    absolute_replacement_increment,
    old_fee_limit × replacement_percentage
)

Initial recommendation:

replacement_percentage = 10%

The exact rule remains configurable.

Replacement MUST affect only mempool policy.

A finalized transaction can never be replaced.

⸻

17. Balance Requirement

At execution time, the sender MUST satisfy:

sender.balance ≥ amount + required_fee

fee_limit is an authorization ceiling, not necessarily the charged fee.

The block MUST be rejected if it includes a transaction that causes insufficient balance under deterministic execution order.

⸻

18. Transaction Ordering

Q1-LTX-020 — Deterministic execution order

Transactions within a block MUST have one explicit deterministic order.

The block producer MAY select transactions under a configurable policy, but once included, the block order is authoritative and MUST be validated identically.

⸻

Q1-LTX-021 — Same-sender ordering

Transactions from the same sender MUST execute in strictly increasing consecutive nonce order.

⸻

Q1-LTX-022 — Candidate ordering policy

The initial block-builder policy SHOULD consider:

1. executable nonce;
2. effective fee priority;
3. arrival ordering as a local tie-breaker;
4. deterministic transaction-ID ordering as final tie-breaker.

Arrival time MUST NOT be a consensus validity rule.

Two producers MAY build different valid candidate blocks from different mempool views.

⸻

19. Atomic State Transition

Every transaction MUST execute atomically.

A native transfer either:

* applies completely;
* or has no state effect.

For a valid transfer:

sender.balance =
    sender.balance - amount - required_fee
sender.nonce =
    sender.nonce + 1
recipient.balance =
    recipient.balance + amount

Fee distribution is recorded according to protocol rules defined in 10_TOKENOMICS.md.

No intermediate partial state may become visible as finalized state.

⸻

20. Transfer State-Transition Algorithm

The following is legacy conceptual execution pseudocode, not an approved
Session 3 execution rule. It is retained for later ledger review and MUST NOT
be implemented from the approved wire schema alone:

function apply_transfer(state, tx, block_context):
    require tx.chain_id == block_context.chain_id
    require supported(tx.schema_version)
    sender_address = derive_address(tx.sender_public_key)
    require valid_address(sender_address)
    require valid_address(tx.recipient_address)
    require sender_address != tx.recipient_address
    require verify_signature(tx)
    require tx.amount > 0
    require block_context.height >= tx.valid_from_height
    require block_context.height <= tx.valid_until_height
    sender = state.get_or_default(sender_address)
    recipient = state.get_or_default(tx.recipient_address)
    require tx.nonce == sender.nonce
    required_fee = calculate_fee(tx, block_context)
    require required_fee <= tx.fee_limit
    require sender.balance >= tx.amount + required_fee
    new_sender_balance =
        checked_sub(sender.balance, tx.amount + required_fee)
    new_recipient_balance =
        checked_add(recipient.balance, tx.amount)
    sender.balance = new_sender_balance
    sender.nonce = checked_add(sender.nonce, 1)
    recipient.balance = new_recipient_balance
    state.put(sender)
    state.put(recipient)
    return receipt(
        status = SUCCESS,
        charged_fee = required_fee,
        amount_transferred = tx.amount
    )

Every operation MUST use checked arithmetic.

This conceptual algorithm does not approve replay, expiration, fee, balance,
receipt, or state-transition behavior.

⸻

21. Failed Transactions

Q1 v0.1 SHALL NOT include invalid user transactions in finalized blocks.

If a transaction fails deterministic pre-execution because of:

* invalid signature;
* invalid nonce;
* insufficient funds;
* expiration;
* incorrect fee limit;
* malformed address;
* arithmetic failure;
* unsupported version;

the entire candidate block MUST be rejected if the producer included it.

This keeps the v0.1 ledger simple.

Future protocol versions MAY introduce chargeable failed executions for smart-contract operations.

⸻

22. Transaction Receipts

Every successfully finalized transaction MUST produce a deterministic receipt.

Minimum receipt structure:

TransactionReceipt {
    receipt_version
    transaction_id
    block_hash
    block_height
    transaction_index
    transaction_type
    sender_address
    recipient_address
    amount
    charged_fee
    sender_nonce_before
    sender_nonce_after
    status
}

Possible initial status:

SUCCESS

This receipt shape remains unapproved and is not part of TransferBodyV1.
`transaction_type` and the derived sender address may appear in future
receipt/API views without entering signed transfer bytes.

Local systems MAY also represent non-final statuses, but those are not finalized protocol receipts.

⸻

Q1-LTX-023 — Receipt commitment

ReceiptRoot is omitted from BlockHeaderV1. Its profile remains inactive until
receipt schema, correspondence, order, status/error encoding, empty-sequence
semantics, limits, and vectors are separately approved.

Approved eventual Header commitment membership includes:

transaction_root
participant_root
state_root
delay_evidence_commitment

TransactionRoot commits to the exact ordered BlockBodyV1 transfer sequence.
ParticipantRoot commits to the active pre-H ParticipantSetV1. StateRoot is
approved only as a required typed field shape. DelayEvidenceHash commits to
the complete canonical DelayEvidenceV1 under the future DELAY_EVIDENCE domain.
StateRoot construction and all valid BlockHeaderV1 instantiation remain
blocked by the state-model gate and the other Session 5C blockers.

⸻

23. Transaction Lifecycle

A transaction MAY pass through the following user-visible states:

CREATED
SIGNED
SUBMITTED
ACCEPTED_IN_MEMPOOL
QUEUED_FOR_NONCE
BROADCAST
INCLUDED_IN_PROPOSAL
ATTESTED
FINALIZED
REJECTED
REPLACED
EXPIRED
DROPPED

⸻

23.1 Created

The wallet has constructed an unsigned transaction.

⸻

23.2 Signed

The wallet has signed the transaction.

⸻

23.3 Submitted

The wallet has sent the transaction to at least one node.

⸻

23.4 Accepted in mempool

A node has accepted the transaction into its local mempool.

This does not guarantee finalization.

⸻

23.5 Queued for nonce

The transaction is valid but depends on missing earlier sender nonces.

⸻

23.6 Broadcast

The transaction has been announced or forwarded to peers.

⸻

23.7 Included in proposal

The transaction appears in a candidate block.

It is not yet final.

⸻

23.8 Attested

The candidate block has received validator attestations but has not yet reached finalization threshold.

⸻

23.9 Finalized

The transaction is included in a finalized block.

This is the authoritative successful state.

⸻

23.10 Rejected

A node or block validator rejected the transaction.

The rejection reason SHOULD be exposed.

⸻

23.11 Replaced

Another valid transaction with the same sender nonce replaced it under mempool rules.

⸻

23.12 Expired

The current block height exceeded valid_until_height.

⸻

23.13 Dropped

A local node removed the transaction due to resource limits, peer policy, or restart behavior.

Dropped does not necessarily mean globally invalid.

⸻

24. Rejection Codes

Q1 SHALL use stable machine-readable transaction rejection codes.

Recommended initial codes:

TX_DECODE_ERROR
TX_TOO_LARGE
TX_UNSUPPORTED_VERSION
TX_WRONG_CHAIN
TX_UNSUPPORTED_TYPE
TX_INVALID_SENDER_ADDRESS
TX_INVALID_RECIPIENT_ADDRESS
TX_SELF_TRANSFER
TX_INVALID_PUBLIC_KEY
TX_ADDRESS_KEY_MISMATCH
TX_INVALID_SIGNATURE
TX_ZERO_AMOUNT
TX_AMOUNT_OVERFLOW
TX_FEE_LIMIT_TOO_LOW
TX_INSUFFICIENT_BALANCE
TX_NONCE_TOO_LOW
TX_NONCE_TOO_HIGH
TX_NONCE_GAP_TOO_LARGE
TX_NOT_YET_VALID
TX_EXPIRED
TX_DUPLICATE
TX_REPLACEMENT_UNDERPRICED
TX_MEMPOOL_FULL
TX_RATE_LIMITED
TX_INTERNAL_SAFE_FAILURE

Human-readable explanations MAY vary by interface.

Consensus behavior MUST use stable codes.

⸻

25. Double-Spend Protection

Q1 v0.1 protects against double spending through four layers.

25.1 Signature authorization

Only the holder of the correct private key can authorize spending from an account, subject to key security assumptions.

⸻

25.2 Balance validation

The sender cannot spend more than:

current balance

under deterministic block execution.

⸻

25.3 Sequential nonce

Only one finalized transaction can consume a specific sender nonce.

Two transactions with the same nonce conflict.

⸻

25.4 Finalized ledger agreement

Once one transaction is included in finalized state, another conflicting transaction using the consumed nonce is invalid.

⸻

26. Conflicting Transactions

Suppose two signed transactions exist:

TX-A:
sender = Alice
nonce = 5
recipient = Bob
TX-B:
sender = Alice
nonce = 5
recipient = Carol

Both MAY be cryptographically valid.

They cannot both execute.

Whichever transaction is included first in the finalized canonical state consumes nonce 5.

The other becomes stale and MUST be rejected.

Mempools MAY retain only one under replacement rules.

⸻

27. Replay Protection

Q1 MUST prevent multiple replay categories.

27.1 Same-chain replay

The account nonce prevents the same finalized transaction from executing twice.

⸻

27.2 Cross-chain replay

The signed chain_id prevents a transaction signed for one Q1 network from being valid on another.

⸻

27.3 Cross-version replay

The signed transaction version and domain separator prevent interpretation under incompatible formats.

⸻

27.4 Expiration replay

The validity window prevents old unconfirmed transactions from remaining valid indefinitely.

⸻

28. Genesis Allocations

Genesis allocations MUST be defined in the genesis configuration.

Each allocation SHALL include:

address
amount
allocation_purpose

The genesis process MUST:

1. validate all addresses;
2. validate all amounts;
3. reject duplicate ambiguous entries;
4. sum all allocations;
5. require the total to equal the configured genesis supply exactly;
6. create deterministic initial account state;
7. calculate the genesis state root.

Genesis allocations MUST NOT use ordinary user signatures.

Their authority derives solely from the agreed genesis configuration.

Any treasury, faucet, research, or reserve amount MUST be represented as an
explicit allocation to a defined address. No implicit, hidden, or unassigned
genesis remainder is permitted.

⸻

29. Protocol Rewards

Protocol rewards MUST be generated from finalized protocol events.

Reward creation SHALL be deterministic.

A reward record MUST identify:

reward_recipient
reward_role
reward_amount
source_block
source_round
reward_rule_version

Examples of roles:

BLOCK_PRODUCER
DELAY_EXECUTOR
VALIDATOR
HDD_PARTICIPANT
TREASURY

The exact reward calculations are defined in 10_TOKENOMICS.md.

No user-submitted transaction may impersonate a protocol reward.

⸻

30. Fee Accounting

For each finalized transfer, the receipt MUST record:

charged_fee

The block fee summary MUST record at least:

total_fees_collected
total_fees_distributed
total_fees_burned
total_fees_to_treasury

The following conservation rule MUST hold:

total_fees_collected
=
total_fees_distributed
+
total_fees_burned
+
total_fees_to_treasury

No fee units may disappear without being accounted for.

⸻

31. Supply Conservation

For every finalized block, the node MUST be able to prove:

new_total_supply
=
previous_total_supply
+
authorized_new_issuance
-
authorized_burn

Ordinary transfers MUST NOT change total supply.

They only move balances and distribute fees.

The sum of all account balances plus any explicitly tracked protocol balances MUST equal total circulating supply.

⸻

32. State Root

Q1 blocks MUST commit to the resulting ledger state through a deterministic state root.

StateRoot profile ID `0x0005` is reserved inactive. The shared indexed-sequence
Merkle profile approved in DEC-Q1-027 Session 4 MUST NOT be applied to
StateRoot. A separate state-model decision must define canonical keys and
values, ordering, duplicate rejection, inclusion/non-inclusion and empty-state
semantics, update/proof models, and persistence assumptions.

For each account, the committed value SHOULD include:

address
balance
nonce
account_version

The exact state-tree structure remains a protocol decision.

It MUST support:

* deterministic root calculation;
* independent reconstruction;
* membership proofs in future;
* absence proofs where practical;
* consistent ordering;
* versioning.

⸻

33. Transaction Root

Transactions in a block MUST be committed through a deterministic transaction root.

The root MUST depend on:

* exact transaction order;
* exact transaction IDs;
* canonical tree construction;
* explicit behavior for empty blocks;
* explicit behavior for odd tree layers.

Two blocks with the same transactions in a different order MUST produce different transaction roots.

⸻

34. Receipt Root

Receipts SHOULD be committed through a deterministic receipt root.

This enables proof of:

* successful execution;
* charged fee;
* transaction index;
* finalized inclusion.

⸻

35. Block Execution Procedure

For each proposed block, a validator MUST:

1. Load the finalized or valid parent state.
2. Verify block transaction count and size.
3. Verify transaction root.
4. Initialize temporary state.
5. Process transactions in listed order.
6. Reject on the first invalid transaction.
7. Produce deterministic receipts.
8. Calculate receipt root.
9. Calculate resulting state root.
10. Verify fee summary.
11. Verify protocol reward records.
12. Verify total supply transition.
13. Compare all computed roots and summaries.
14. Accept or reject the candidate block.

Temporary execution MUST NOT mutate finalized state before block finalization.

⸻

36. State Commit Procedure

After a valid block receives a valid finalization certificate:

1. Reconfirm parent remains canonical.
2. Reconfirm block hash.
3. Reconfirm finalization certificate.
4. Begin atomic database transaction.
5. Persist block.
6. Persist receipts.
7. Persist consensus evidence.
8. Commit state changes.
9. Update canonical-chain index.
10. Update finalized height.
11. Remove confirmed mempool transactions.
12. Invalidate stale conflicting mempool transactions.
13. Commit database transaction.
14. Emit finalized events.

If any critical database operation fails, the node MUST NOT claim successful finalization locally.

⸻

37. Fork and Proposal State

A node MAY temporarily store multiple valid proposals for the same height and round.

Balances from unfinalized proposals MUST NOT be shown as finalized.

Wallets and explorers MUST distinguish:

pending proposal balance
finalized balance

The authoritative spendable balance for v0.1 SHOULD be the finalized balance.

A wallet MAY display expected pending changes separately.

⸻

38. Finality and User Confirmation

Q1 v0.1 SHALL treat a transaction as final when:

* its containing block is valid;
* the block has a valid finalization certificate;
* the block is committed to the canonical finalized chain.

The wallet MUST NOT label a transaction “finalized” merely because:

* it entered the mempool;
* one producer proposed it;
* one validator attested;
* the explorer observed it;
* the local node tentatively executed it.

⸻

39. Account Query Semantics

A node API querying an account SHOULD return:

{
  "address": "...",
  "finalized_balance": "100000000",
  "finalized_nonce": 4,
  "pending_outgoing_amount": "25000000",
  "pending_fees_maximum": "1000",
  "next_recommended_nonce": 5,
  "finalized_height": 120,
  "state_root": "..."
}

Consensus monetary values SHOULD be serialized as decimal strings in JSON APIs to prevent integer precision loss in clients.

⸻

40. Transaction Query Semantics

A transaction query SHOULD return:

transaction_id
transaction_version
type
sender
recipient
amount
fee_limit
charged_fee_if_finalized
nonce
validity_window
status
first_seen_local
included_block
included_height
transaction_index
finalization_status
rejection_code_if_known

first_seen_local is not consensus data.

⸻

41. Wallet Construction Rules

The wallet MUST:

1. query finalized sender state;
2. determine the next recommended nonce;
3. estimate the current required fee;
4. choose a fee limit;
5. construct a validity window;
6. validate recipient address;
7. display amount and maximum fee;
8. request user confirmation;
9. sign canonical payload;
10. compute transaction ID;
11. submit to one or more nodes;
12. track status.

The wallet MUST NOT rely only on untrusted explorer data for signing decisions.

⸻

42. Offline Signing

Q1 SHOULD support offline signing.

An unsigned transaction package MUST contain all fields required for signing.

The online device MAY construct the unsigned transaction.

The offline device:

* verifies transaction details;
* signs the canonical payload;
* returns the signed transaction.

The private key remains offline.

The signed transaction MAY then be broadcast by an untrusted online device.

⸻

43. Key Loss and Recovery

Q1 v0.1 SHALL not include protocol-level password recovery or key reversal.

If a private key is lost:

* the protocol cannot reconstruct it;
* funds controlled by that key may become permanently inaccessible.

Wallet software SHOULD support:

* encrypted backups;
* recovery phrases if the selected key architecture permits;
* explicit backup warnings;
* test recovery procedures.

Recovery design is primarily a wallet concern, not a ledger override.

⸻

44. Key Compromise

If a private key is stolen, validly signed transactions may be accepted.

Q1 v0.1 SHALL not include:

* transaction reversal by administrators;
* account freezing by AI;
* emergency central confiscation;
* identity-based recovery.

Future versions MAY research:

* multi-signature accounts;
* recovery guardians;
* delayed high-value transfers;
* programmable spending policies.

⸻

45. Transaction Size Limits

The protocol MUST define a maximum serialized transaction size.

Recommended v0.1 value:

4 KB

Ordinary native transfers SHOULD be substantially smaller.

Oversized transactions MUST be rejected before expensive cryptographic processing where possible.

⸻

46. Memo and Data Limits

TransferBodyV1 has no memo field, reserved position, or null slot.
Future memo functionality requires a new schema version.

The transaction MUST NOT contain:

* arbitrary files;
* unrestricted messages;
* executable code;
* large embedded documents.

Q1 v0.1 is a value-transfer ledger, not a general data-storage network.

⸻

47. Mempool Resource Protection

Each node MUST enforce configurable limits for:

* total mempool bytes;
* total transaction count;
* transactions per sender;
* future nonce count per sender;
* transaction submissions per peer;
* signature validations per time window;
* maximum orphaned nonce duration.

Under pressure, eviction policy SHOULD prefer removing:

1. expired transactions;
2. invalidated transactions;
3. underpriced replacement candidates;
4. transactions with lowest effective fee priority;
5. distant future-nonce transactions;
6. oldest low-priority entries.

Eviction is local policy and does not invalidate the transaction globally.

⸻

48. Transaction Propagation

Nodes SHOULD propagate valid transaction announcements rather than repeatedly sending full payloads to every peer.

A possible flow:

1. Node receives valid transaction.
2. Node announces transaction ID to peers.
3. Interested peers request missing transaction.
4. Node sends canonical transaction bytes.
5. Receiving peer independently validates.

The networking layer MUST prevent:

* announcement floods;
* repeated duplicate payloads;
* unlimited unsolicited transaction delivery.

⸻

49. Transaction Privacy Statement

Q1 v0.1 transactions are public ledger records.

The protocol does not hide:

* sender address;
* recipient address;
* amount;
* fee;
* nonce;
* timing by block;
* transaction relationships.

Users MUST be informed that address pseudonymity is not identity privacy.

⸻

50. Determinism Requirements

All consensus-critical ledger behavior MUST be deterministic across:

* operating systems;
* CPU architectures;
* database backends;
* node roles;
* geographic regions;
* implementation languages, if multiple clients later exist.

The implementation MUST avoid consensus dependence on:

* floating-point calculations;
* locale-sensitive sorting;
* local time zones;
* operating-system file ordering;
* non-deterministic map iteration;
* random iteration order;
* database query order without explicit sorting;
* hardware-specific arithmetic behavior.

⸻

51. Property-Based Testing Requirements

The ledger implementation SHOULD include property-based tests for:

Conservation

Ordinary transfers do not change total supply.

⸻

Non-negativity

No valid state contains a negative balance.

⸻

Nonce monotonicity

A finalized user transaction increments its sender nonce exactly once.

⸻

Determinism

The same initial state and ordered transactions produce the same final state and roots.

⸻

Atomicity

A failed transaction leaves state unchanged.

⸻

Replay rejection

A finalized transaction cannot execute again.

⸻

Overflow safety

Values near numeric limits cannot wrap or create funds.

⸻

Signature binding

Changing any signed field invalidates the signature.

⸻

52. Required Ledger Test Scenarios

The automated test suite MUST include at least:

1. valid transfer;
2. zero-amount transfer;
3. insufficient balance;
4. invalid signature;
5. wrong sender public key;
6. wrong chain ID;
7. expired transaction;
8. not-yet-valid transaction;
9. duplicate transaction;
10. repeated nonce;
11. stale nonce;
12. future nonce;
13. excessive nonce gap;
14. two conflicting transactions;
15. valid replacement;
16. underpriced replacement;
17. self-transfer;
18. maximum-value transfer;
19. arithmetic overflow attempt;
20. fee above fee limit;
21. recipient account creation;
22. sender balance becomes zero;
23. node restart with pending transactions;
24. block containing invalid transaction;
25. transaction-root mismatch;
26. state-root mismatch;
27. receipt-root mismatch;
28. unauthorized reward transaction;
29. cross-chain replay;
30. transaction serialization ambiguity;
31. altered memo hash;
32. invalid address checksum;
33. malformed public key;
34. malformed signature;
35. supply-conservation violation.

⸻

53. Minimum Ledger Acceptance Criteria

The ledger and transaction subsystem is ready for integration when:

* two wallets generate independent addresses;
* genesis allocates test units;
* one wallet sends a signed transfer;
* all honest nodes derive the same transaction ID;
* all honest nodes verify the same signature result;
* the transaction enters mempools;
* a block producer includes it;
* validators execute the same state transition;
* all validators calculate the same roots;
* the block finalizes;
* all honest nodes report identical balances and nonces;
* replaying the transaction fails;
* conflicting same-nonce transactions cannot both finalize;
* total supply remains correct;
* restarting nodes preserves finalized state;
* transaction receipts remain queryable.

⸻

54. Open Decisions

The following decisions remain unresolved:

1. exact signature algorithm;
2. exact hash function;
3. exact address encoding;
4. exact canonical serialization format;
5. exact state-tree construction;
6. exact transaction-tree construction;
7. whether transaction ID includes signature bytes;
8. whether self-transfers are permanently forbidden;
9. exact transaction validity window;
10. exact maximum transaction size;
11. exact replacement fee rule;
12. whether memo hashes are enabled in v0.1;
13. whether receipt roots are mandatory in the first milestone;
14. whether zero-balance accounts are pruned immediately;
15. exact integer maximum and supply ceiling;
16. whether public keys remain embedded in every transaction;
17. whether protocol rewards are encoded as transactions or block-level state operations.

All decisions MUST be recorded in OPEN_DECISIONS.md.

⸻

55. Codex Implementation Rules

Codex MUST:

1. represent all monetary values as checked integers;
2. isolate canonical serialization in a dedicated module;
3. isolate signature verification behind the cryptography provider;
4. calculate transaction IDs only from canonical bytes;
5. implement stable rejection codes;
6. implement state transitions as deterministic functions;
7. avoid modifying finalized state during candidate execution;
8. execute block transactions inside temporary or transactional state;
9. test nonce conflicts thoroughly;
10. test supply conservation;
11. reject unauthorized reward creation;
12. separate mempool policy from consensus validity;
13. serialize large integers as strings in JSON APIs;
14. never log private keys or recovery phrases;
15. make every open assumption configurable or explicitly documented.

Codex MUST NOT:

* use floating-point values for amounts;
* trust wallet-calculated balances;
* trust explorer-calculated state;
* permit direct API balance editing;
* accept a transaction based only on transaction ID;
* execute unknown transaction versions;
* silently wrap arithmetic;
* treat mempool acceptance as finality;
* use local transaction arrival time as consensus ordering;
* accept non-canonical encodings.

⸻

56. Final Ledger Principle

The Q1 ledger MUST preserve a simple rule:

A unit may be created only by an explicit protocol rule.
It may move only through a valid state transition.
It may be spent only by authorized signature and correct nonce.
It becomes final only when the network finalizes the containing block.
Every change must be reproducible, accountable, and independently verifiable.

Q1 v0.1 succeeds at the ledger layer when no node needs to trust a wallet, explorer, producer, HDD device, AI model, or human operator to determine a valid balance.
