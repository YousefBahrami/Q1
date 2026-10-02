Q1 Wallet Specification

09_WALLET.md

Project: Q1 Experimental Distributed Ledger
Protocol Version: 0.1
Document Version: 0.1.0
Status: Draft for Engineering Review
Classification: Experimental — Not for Production or Financial Use

⸻

1. Purpose

This document defines the Q1 Wallet for protocol version 0.1.

The wallet is the primary user-facing component for:

* generating cryptographic keys;
* deriving addresses;
* protecting local secrets;
* querying finalized balances;
* building transactions;
* estimating fees;
* signing transactions;
* submitting transactions;
* tracking transaction status;
* exporting and importing wallet data;
* supporting offline signing;
* warning users about experimental-network risks.

The Q1 Wallet MUST be designed so that a user can transfer test units without needing to understand internal consensus mechanics.

At the same time, it MUST not hide security-critical facts such as:

* which network is active;
* whether the transaction is finalized;
* what fee may be charged;
* whether the wallet has a valid backup;
* whether the node response is trusted or independently verified;
* whether the software is running in experimental mode.

ADR-0004 and ADR-0005 select Ed25519 and a Q1-specific Bech32m envelope.
Wallet implementation remains unauthorized until their proposed V1 profiles
under `docs/protocol/` pass the pre-M1 gate. Private key material must never be
delegated to the TypeScript UI layer merely because that layer is permitted by
ADR-0001.

⸻

2. Wallet Design Principles

Q1-WAL-001 — User control

The user MUST control wallet spending keys.

A normal node, explorer, AI observer, or public API MUST NOT hold the user’s private key.

⸻

Q1-WAL-002 — Local signing

Transaction signing SHOULD occur locally on the user’s device.

The wallet MUST NOT transmit private keys or recovery secrets to a node.

⸻

Q1-WAL-003 — Explicit finality

The wallet MUST distinguish:

pending
included
attested
finalized
failed
expired
replaced

A transaction MUST NOT be shown as final merely because it was submitted or entered a mempool.

⸻

Q1-WAL-004 — Network clarity

The wallet MUST prominently display the active network.

Examples:

Q1 Development
Q1 Localnet
Q1 Private Testnet
Q1 Public Testnet

The wallet MUST make it difficult to confuse test units with any future production asset.

⸻

Q1-WAL-005 — Secure defaults

The default wallet configuration MUST prefer:

* encrypted local storage;
* explicit user confirmation;
* safe fee limits;
* network mismatch protection;
* recipient checksum validation;
* no secret logging;
* no automatic key export;
* no hidden background signing.

⸻

Q1-WAL-006 — Simplicity before feature expansion

Q1 v0.1 MUST begin with a command-line wallet.

A graphical wallet MAY be added later.

The first wallet SHOULD prioritize:

* correctness;
* observability;
* deterministic behavior;
* testability;
* user comprehension.

⸻

3. Wallet Scope

Q1 v0.1 wallet SHALL support:

* wallet creation;
* key generation;
* encrypted local key storage;
* address display;
* balance query;
* finalized nonce query;
* fee estimation;
* transaction creation;
* transaction preview;
* transaction signing;
* transaction submission;
* transaction-status tracking;
* transaction-history query;
* wallet backup;
* wallet restore;
* offline signing;
* network selection;
* export of public wallet information;
* safe testnet warnings.

⸻

4. Out of Scope

The following are not required in v0.1:

* real-money custody;
* exchange integration;
* fiat purchase;
* hardware-wallet production support;
* biometric-only recovery;
* cloud key custody;
* social recovery;
* multi-signature accounts;
* smart-contract interaction;
* token swaps;
* NFTs;
* cross-chain bridges;
* mobile production wallet;
* browser extension wallet;
* merchant payment SDK;
* hidden balances;
* privacy transactions;
* staking dashboard;
* automatic tax reporting.

These MAY be researched later.

⸻

5. Wallet Architecture

The wallet SHOULD be divided into:

Q1 Wallet
├── Wallet Core
├── Key Manager
├── Encrypted Keystore
├── Address Manager
├── Network Profile Manager
├── Transaction Builder
├── Fee Estimator
├── Transaction Signer
├── Node Client
├── Status Tracker
├── Backup and Restore
├── Offline Signing
├── CLI Interface
└── Structured Audit Logger

The CLI MUST not directly manipulate encrypted key files.

It MUST use the wallet core interfaces.

⸻

6. Wallet Core Interface

WalletCore {
    create_wallet(options)
    open_wallet(wallet_id, credentials)
    close_wallet(wallet_id)
    list_wallets()
    generate_account(wallet_id)
    list_accounts(wallet_id)
    get_address(wallet_id, account_id)
    get_balance(wallet_id, account_id, network)
    get_nonce(wallet_id, account_id, network)
    estimate_fee(transaction_request)
    build_transaction(transaction_request)
    preview_transaction(unsigned_transaction)
    sign_transaction(wallet_id, account_id, unsigned_transaction, credentials)
    submit_transaction(signed_transaction, node_endpoint)
    get_transaction_status(transaction_id)
    list_transactions(filters)
    export_public_data(wallet_id)
    create_backup(wallet_id, options)
    restore_backup(backup, credentials)
    export_unsigned_transaction(transaction)
    import_signed_transaction(transaction)
}

⸻

7. Wallet Types

Q1 v0.1 MAY support three wallet types.

7.1 Standard local wallet

Stores encrypted private keys locally.

Recommended default.

⸻

7.2 Watch-only wallet

Stores:

* addresses;
* public keys;
* transaction history;
* balance data.

It MUST NOT contain spending keys.

It can:

* monitor accounts;
* build unsigned transactions;
* export transactions for offline signing.

⸻

7.3 Offline signer wallet

Designed to operate without network access.

It can:

* import unsigned transaction packages;
* verify transaction details;
* sign;
* export signed transactions.

It SHOULD avoid node and explorer connectivity.

⸻

8. Wallet Identity

A wallet file SHOULD contain a wallet-level identifier.

WalletMetadata {
    wallet_format_version
    wallet_id
    wallet_type
    created_at_local
    account_count
    active_network_profiles[]
    backup_status
    wallet_label_optional
}

wallet_id is local metadata.

It MUST NOT be treated as a blockchain identity.

⸻

9. Key Model

Each spend-capable account MUST include:

WalletAccount {
    account_id
    key_type
    public_key
    encrypted_private_key
    address
    derivation_path_optional
    created_at_local
    account_label_optional
}

⸻

Q1-WAL-007 — Private key isolation

Private keys MUST remain inside the key-management boundary.

⸻

Q1-WAL-008 — No plaintext persistence

Private keys MUST NOT be stored in plaintext by default.

⸻

Q1-WAL-009 — Memory handling

Decrypted private-key material SHOULD remain in memory only for the shortest practical duration.

Sensitive buffers SHOULD be cleared where the implementation language and runtime permit.

⸻

Q1-WAL-010 — Key reuse

The wallet MUST NOT reuse node identity keys as spending keys.

⸻

10. Cryptographic Key Generation

Key generation MUST use a cryptographically secure random source.

The wallet MUST reject initialization if secure randomness is unavailable.

The key-generation implementation MUST:

* use the protocol-approved signature algorithm;
* use reviewed cryptographic libraries;
* validate generated keys;
* derive and verify the corresponding address;
* never print raw private keys to ordinary logs.

⸻

11. Recovery Material

The wallet SHOULD support human-manageable recovery material if compatible with the selected key architecture.

Possible future approach:

recovery phrase
+
optional passphrase

For v0.1, one of the following MAY be implemented:

* encrypted backup file only;
* deterministic seed and recovery phrase;
* both.

The selected method MUST be documented clearly.

⸻

Q1-WAL-011 — No false recovery promise

The wallet MUST NOT imply that Q1 operators can restore lost keys.

⸻

Q1-WAL-012 — Recovery verification

The wallet SHOULD offer a recovery test.

A recovery test MUST verify that:

* backup material is readable;
* expected addresses can be derived;
* the user understands the backup location;
* restoration succeeds in a temporary environment.

⸻

12. Keystore

The keystore MUST contain encrypted key material and minimum required metadata.

Recommended conceptual structure:

EncryptedKeystore {
    keystore_version
    wallet_id
    account_id
    public_key
    address
    encryption_algorithm
    key_derivation_algorithm
    key_derivation_parameters
    salt
    nonce_or_iv
    ciphertext
    authentication_tag
    checksum
}

⸻

13. Keystore Encryption

The wallet SHOULD use:

* an authenticated encryption algorithm;
* a memory-hard password-based key derivation function;
* unique random salt;
* unique nonce or initialization vector;
* explicit algorithm versioning.

Weak or obsolete password-hashing schemes MUST NOT be used.

Exact algorithms remain an open engineering decision and require security review.

⸻

14. Wallet Password

A password protects the encrypted local keystore.

The wallet SHOULD:

* warn against weak passwords;
* avoid arbitrary low maximum lengths;
* support spaces and long passphrases;
* avoid transmitting the password;
* avoid storing the password;
* apply bounded retry delays locally;
* never treat the password as a recovery substitute.

Loss of both password and recovery material may permanently prevent access.

⸻

15. Wallet Locking

A wallet MAY be:

LOCKED
UNLOCKED
SIGNING
CLOSED

The wallet SHOULD lock automatically after a configurable inactivity period.

Recommended default:

5 minutes

The user MAY choose a shorter or longer period.

Long-duration unlocking SHOULD trigger a warning.

⸻

16. Account and Address Creation

The wallet MUST:

1. generate or derive a valid key pair;
2. validate the public key;
3. derive the correct network address;
4. verify the address checksum;
5. store encrypted private material;
6. display the new address;
7. recommend backup if none exists.

⸻

17. Address Display

The wallet SHOULD provide:

* plain text address;
* copy function where UI permits;
* QR representation;
* network label;
* checksum validation result;
* optional shortened display for visual use;
* full display before transaction confirmation.

The wallet MUST NOT sign based solely on a visually shortened address.

⸻

18. Recipient Validation

Before transaction construction, the wallet MUST verify:

* address encoding;
* checksum;
* supported address version;
* intended network;
* recipient is not empty;
* recipient differs from sender under v0.1 rules.

A network mismatch MUST cause rejection or a high-severity warning requiring explicit correction.

⸻

19. Network Profiles

The wallet SHALL support network profiles.

NetworkProfile {
    profile_name
    chain_id
    genesis_hash
    address_prefix
    node_endpoints[]
    explorer_endpoints_optional[]
    protocol_version
    asset_symbol
    experimental_warning
}

⸻

Q1-WAL-013 — Trusted configuration

Network profiles MUST be loaded from:

* bundled configuration;
* explicit user import;
* local configuration.

Remote changes MUST NOT silently alter:

* chain ID;
* genesis hash;
* address prefix;
* signing rules.

⸻

Q1-WAL-014 — Network switch

Switching networks MUST be explicit.

Balances and history from different networks MUST remain separated.

⸻

20. Node Client

The wallet communicates with one or more Q1 nodes.

The Node Client SHOULD support:

* node status;
* network identity verification;
* balance query;
* nonce query;
* fee estimation;
* transaction submission;
* transaction lookup;
* finalized-height query;
* block lookup where needed.

⸻

21. Node Trust Model

A wallet connected to one node receives claims from that node.

The wallet MUST distinguish:

* cryptographically verifiable data;
* node-reported local status;
* explorer-derived data;
* unverified convenience data.

For v0.1, the wallet MAY trust a configured private-testnet node operationally, but the interface SHOULD be designed for multi-node cross-checking.

⸻

22. Multi-Node Querying

The wallet SHOULD support querying more than one node.

It MAY compare:

* chain ID;
* genesis hash;
* finalized height;
* finalized block hash;
* balance;
* nonce;
* transaction status.

Conflicting finalized-state claims SHOULD trigger a warning.

The wallet MUST NOT silently choose a materially conflicting node response.

⸻

23. Balance Semantics

The wallet SHOULD display:

Finalized Balance
Pending Outgoing
Pending Incoming
Maximum Reserved Fees
Available Finalized Balance

Recommended:

AvailableFinalizedBalance =
    FinalizedBalance
    - PendingOutgoingAmount
    - PendingMaximumFees

This is a wallet-side planning value, not consensus state.

⸻

24. Balance Query Response

The wallet SHOULD expect:

AccountBalanceResponse {
    address
    finalized_balance
    finalized_nonce
    finalized_height
    finalized_block_hash
    state_root
}

All monetary JSON values SHOULD use decimal strings.

⸻

25. Transaction Request

User-facing transaction input:

TransactionRequest {
    network_profile
    wallet_id
    account_id
    recipient_address
    amount_display_units
    fee_preference
    fee_limit_optional
    validity_window_optional
}

The wallet converts display units to base units using checked integer arithmetic.

The resulting consensus object is the approved nine-field TransferBodyV1.
Only the sender public key is serialized; the wallet derives and displays the
sender address for confirmation but does not insert it into consensus bytes.
The recipient is serialized as the binary AddressEnvelope. V1 has no
transaction-type or memo field. Wallet fee selection and expiration behavior
remain separate future decisions outside the wire-schema authorization.

⸻

26. Amount Entry

The wallet MUST:

* parse decimal text deterministically;
* reject excessive decimal places;
* reject negative values;
* reject zero;
* reject scientific notation unless explicitly supported;
* reject values exceeding protocol limits;
* display the exact base-unit conversion before signing.

Floating-point arithmetic MUST NOT be used.

⸻

27. Fee Preferences

The wallet MAY offer:

ECONOMY
STANDARD
PRIORITY
CUSTOM

These are user-interface choices.

The wallet MUST convert them into:

* estimated required fee;
* proposed fee limit;
* expected inclusion behavior.

The wallet MUST not imply guaranteed inclusion time.

⸻

28. Fee Estimation

The wallet SHALL request fee estimates from one or more nodes.

The response SHOULD include:

FeeEstimate {
    estimated_required_fee
    recommended_fee_limit
    minimum_fee
    congestion_component
    estimate_height
    estimate_expiration
    confidence_or_source_count_optional
}

The user MUST see:

* estimated fee;
* maximum authorized fee;
* transfer amount;
* total maximum deduction.

⸻

29. Fee Limit

Before signing, the wallet MUST verify:

finalized_balance >= amount + fee_limit

or clearly warn that pending transactions may make the transaction fail.

The wallet MUST never silently increase the signed fee limit after confirmation.

⸻

30. Nonce Selection

The wallet SHALL query the finalized nonce.

It MUST also inspect its own pending transactions.

Recommended next nonce:

next_nonce =
    max(
        finalized_nonce,
        highest_contiguous_local_pending_nonce + 1
    )

The wallet MUST detect:

* stale pending transactions;
* nonce gaps;
* conflicting same-nonce transactions;
* replaced transactions.

⸻

31. Transaction Validity Window

The wallet SHOULD automatically propose:

valid_from_height =
    current_finalized_height + 1
valid_until_height =
    valid_from_height + configured_window

The user MAY customize the window in advanced mode.

The wallet MUST display expiration meaning in plain language.

⸻

32. Transaction Preview

Before signing, the wallet MUST display a complete preview.

Minimum preview:

Network
Sender
Recipient
Amount
Estimated Fee
Maximum Fee
Maximum Total Deduction
Nonce
Valid From Height
Valid Until Height
Memo Hash, if present
Transaction Version

The user MUST confirm after seeing this information.

⸻

33. Human-Readable Confirmation

A CLI confirmation MAY resemble:

Network: Q1 Private Testnet
From: q1p...
To: q1p...
Amount: 12.50000000 Q1T
Estimated fee: 0.00001000 Q1T
Maximum fee: 0.00002000 Q1T
Maximum total deduction: 12.50002000 Q1T
Nonce: 7
Valid through block: 450
Type "confirm" to sign:

High-value or unusual transactions MAY require stronger confirmation.

⸻

34. Signing Procedure

The wallet SHALL:

1. build the canonical unsigned transaction;
2. display preview;
3. obtain confirmation;
4. unlock the relevant account key;
5. reconstruct canonical signing bytes;
6. sign locally;
7. verify the signature locally;
8. derive the transaction ID;
9. clear sensitive signing material;
10. return the signed transaction.

⸻

35. Local Self-Verification

After signing, the wallet MUST verify:

* signature validity;
* public-key/address relationship;
* canonical serialization;
* transaction ID;
* network and chain ID;
* amount;
* fee limit;
* nonce;
* validity window.

A locally invalid signed transaction MUST NOT be submitted.

⸻

36. Transaction Submission

The wallet MAY submit to:

* one configured node;
* several configured nodes;
* a local full node;
* a remote private-testnet node.

Submission success means only that a node received or accepted the transaction.

It does not mean finalization.

⸻

37. Submission Response

The node response SHOULD include:

TransactionSubmissionResponse {
    transaction_id
    node_id
    accepted
    mempool_status
    rejection_code_optional
    rejection_message_optional
    node_finalized_height
}

The wallet MUST preserve the signed transaction even if submission fails.

⸻

38. Resubmission

The wallet MAY resubmit the same signed transaction to another node.

Because the transaction ID is deterministic, honest nodes SHOULD treat it as the same transaction.

Resubmission MUST NOT alter signed fields.

⸻

39. Transaction Tracking

The wallet SHALL track a transaction using:

* transaction ID;
* local creation time;
* submission nodes;
* current node responses;
* finalized inclusion proof where available.

The wallet SHOULD poll or subscribe for status updates.

⸻

40. User-Visible Statuses

The wallet MUST support:

DRAFT
SIGNED
SUBMITTED
MEMPOOL_ACCEPTED
QUEUED_FOR_NONCE
BROADCAST
INCLUDED_IN_PROPOSAL
ATTESTED
FINALIZED
REJECTED
REPLACED
EXPIRED
DROPPED
UNKNOWN

⸻

41. Finalized Status

A transaction is finalized only when the wallet receives evidence that:

* the transaction is in a block;
* the block is finalized;
* the block belongs to the configured chain;
* the node’s finalized state is compatible with the wallet network profile.

Future wallet versions SHOULD verify inclusion proofs directly.

⸻

42. Unknown Status

UNKNOWN means the queried node does not currently know the transaction.

It MUST NOT automatically mean:

* invalid;
* deleted globally;
* expired;
* stolen;
* finalized elsewhere.

The wallet SHOULD query additional nodes where configured.

⸻

43. Rejection Handling

The wallet SHOULD translate machine rejection codes into clear explanations.

Example:

TX_NONCE_TOO_LOW

User message:

This transaction uses a nonce that has already been consumed.
Refresh the account state before sending again.

The underlying machine code SHOULD remain visible in advanced mode.

⸻

44. Transaction Replacement

If replacement is supported, the wallet SHALL allow the user to replace a pending transaction with:

* the same sender;
* the same nonce;
* a higher valid fee limit;
* a new recipient or amount only after explicit warning.

The wallet MUST clearly state that replacement is not guaranteed until the new transaction finalizes.

⸻

45. Cancel Transaction

A protocol-level cancellation MAY be represented as a replacement transaction sending value back to the same controlled account or another permitted destination.

Because self-transfers are rejected in v0.1, the exact cancellation design remains open.

The wallet MUST NOT display a “cancel” button unless the underlying replacement semantics are implemented and explained.

⸻

46. Transaction History

The wallet SHOULD display:

* transaction ID;
* direction;
* counterparty address;
* amount;
* estimated or charged fee;
* nonce;
* status;
* block height;
* finalized time as observed;
* network;
* rejection reason.

History derived from an explorer MUST be labeled as external index data.

⸻

47. Public Export

The wallet MAY export:

wallet_id
wallet_type
accounts[]
public_keys[]
addresses[]
labels_optional
network_profiles[]

It MUST NOT include private keys or recovery material.

⸻

48. Backup

A backup MAY include:

* encrypted keystore;
* wallet metadata;
* deterministic seed;
* network profiles;
* labels;
* local transaction metadata.

The wallet MUST clearly distinguish:

* secret backup;
* public export;
* watch-only export.

⸻

49. Backup Formats

Recommended backup types:

ENCRYPTED_FULL_BACKUP
RECOVERY_MATERIAL_BACKUP
WATCH_ONLY_BACKUP
PUBLIC_ACCOUNT_EXPORT

Each format MUST contain an explicit type identifier.

⸻

50. Backup Creation

Before backup creation, the wallet SHOULD display:

* what the backup contains;
* whether it can spend funds;
* whether it is encrypted;
* restoration requirements;
* storage safety warning.

The wallet MUST NOT upload backups automatically.

⸻

51. Backup Verification

After creating a backup, the wallet SHOULD:

* verify file integrity;
* decrypt in-memory using provided credentials;
* validate expected accounts;
* compare addresses;
* confirm backup version;
* optionally perform test restoration.

⸻

52. Restore Procedure

The wallet SHALL:

1. detect backup format;
2. verify integrity;
3. request required credentials;
4. decrypt securely;
5. validate key material;
6. derive addresses;
7. compare stored and derived addresses;
8. create a new local wallet record;
9. require network profile confirmation;
10. optionally resynchronize history.

⸻

53. Duplicate Restore

Restoring the same keys more than once creates multiple wallet instances controlling the same accounts.

The wallet MUST warn:

* transactions can be created from either copy;
* nonce conflicts may occur;
* backups are not independent funds;
* compromise of any copy compromises the same keys.

⸻

54. Offline Signing Architecture

Offline signing SHALL use two packages.

54.1 Unsigned transaction package

UnsignedTransactionPackage {
    package_version
    network_profile_summary
    unsigned_transfer
    expected_sender_address
    expected_transaction_summary
    source_node_status_optional
    package_checksum
}

⸻

54.2 Signed transaction package

SignedTransactionPackage {
    package_version
    signed_transfer
    transfer_id
    signer_address
    package_checksum
}

⸻

55. Offline Signing Procedure

Online device:

1. queries balance and nonce;
2. builds unsigned transaction;
3. exports package;
4. transfers package to offline signer.

Offline signer:

1. verifies package checksum;
2. verifies network;
3. verifies sender;
4. displays full transaction preview;
5. signs;
6. verifies signature;
7. exports signed package.

Online device:

1. imports signed package;
2. verifies signature and transaction ID;
3. submits transaction;
4. tracks status.

⸻

56. Air-Gap Transfer

Unsigned and signed packages MAY be transferred using:

* removable media;
* QR sequences;
* local file transfer;
* another explicit user-controlled method.

The wallet MUST assume the transfer medium may be untrusted.

Checksums and signatures MUST detect modification.

⸻

57. QR Support

The wallet MAY support QR codes for:

* addresses;
* unsigned transaction packages;
* signed transaction packages;
* payment requests;
* watch-only exports.

Large packages MAY require multi-part QR encoding.

Exact format remains an open decision.

⸻

58. Payment Request

A payment request MAY include:

PaymentRequest {
    request_version
    network
    recipient_address
    amount_optional
    memo_hash_optional
    expiration_optional
    request_id_optional
}

A payment request is not a transaction.

Its optional memo is off-chain request metadata and MUST NOT be copied into
TransferBodyV1, which has no memo field.

The payer wallet MUST independently validate and confirm all fields.

⸻

59. Wallet Audit Log

The wallet SHOULD maintain a local audit log of non-secret events.

Examples:

wallet.created
wallet.opened
wallet.locked
account.created
backup.created
backup.verified
transaction.built
transaction.signed
transaction.submitted
transaction.finalized
transaction.rejected
network.changed

The log MUST NOT include:

* private keys;
* recovery phrases;
* wallet password;
* decrypted seed;
* raw signing secrets.

⸻

60. Wallet Privacy

The wallet SHOULD minimize data leakage.

It SHOULD avoid:

* sending all addresses to unnecessary third parties;
* querying one public explorer for every account where avoidable;
* uploading wallet labels;
* exposing recovery status;
* including identifying metadata in transactions.

Q1 v0.1 does not provide strong transaction privacy.

⸻

61. Explorer Use

The wallet MAY use an explorer for convenience.

Explorer data MUST be labeled as:

indexed external data

The explorer MUST NOT be treated as the source of spending authority or private keys.

⸻

62. Node Verification

Before using a node, the wallet SHOULD verify:

* chain ID;
* genesis hash;
* protocol version;
* finalized block hash;
* node identity where configured;
* secure transport where available.

A node reporting another network MUST be rejected.

⸻

63. Secure Transport

Remote wallet-node communication SHOULD use authenticated encryption.

For private local development, plaintext localhost transport MAY be permitted.

The wallet MUST warn before sending transactions over insecure remote transport.

Transaction signatures protect transaction integrity but not network metadata or privacy.

⸻

64. Clipboard Safety

A graphical future wallet SHOULD defend against clipboard substitution.

The CLI SHOULD:

* redisplay the full recipient before signing;
* optionally show grouped characters;
* require confirmation;
* support QR-based entry.

The wallet MUST not assume copied text remained unchanged.

⸻

65. Address Book

A future address book MAY store:

label
address
network
notes_local
verification_status

Address-book entries are local convenience records.

They do not prove legal identity.

⸻

66. High-Value Warning

The wallet SHOULD allow configurable high-value thresholds.

Above the threshold, it MAY require:

* re-entry of recipient suffix;
* second confirmation;
* longer review;
* offline signing recommendation;
* backup-status warning.

These controls are wallet security features, not protocol rules.

⸻

67. Testnet Faucet

A private testnet MAY provide a faucet service.

The wallet MAY request test units from it.

The faucet:

* is not part of consensus;
* does not create units outside genesis or authorized protocol rules;
* transfers from a funded faucet account;
* MUST be labeled experimental.

⸻

68. Command-Line Interface

Suggested command structure:

q1-wallet create
q1-wallet open
q1-wallet lock
q1-wallet account new
q1-wallet account list
q1-wallet address show
q1-wallet balance
q1-wallet fee estimate
q1-wallet tx build
q1-wallet tx preview
q1-wallet tx sign
q1-wallet tx send
q1-wallet tx status
q1-wallet tx list
q1-wallet tx replace
q1-wallet backup create
q1-wallet backup verify
q1-wallet backup restore
q1-wallet export public
q1-wallet export watch-only
q1-wallet offline export-unsigned
q1-wallet offline sign
q1-wallet offline import-signed
q1-wallet network list
q1-wallet network use

⸻

69. CLI Output Modes

The CLI SHOULD support:

human-readable
json

JSON mode is useful for automation.

JSON output MUST never include secrets unless a dedicated explicit secret-export command is invoked.

⸻

70. Secret Export

Direct private-key or recovery export SHOULD be disabled by default.

If supported for development:

* command name must be explicit;
* wallet must be unlocked;
* warning must be shown;
* output destination must be explicit;
* operation must not appear in normal logs;
* feature SHOULD be disabled in safer builds.

⸻

71. Error Codes

Recommended wallet errors:

WALLET_NOT_FOUND
WALLET_ALREADY_EXISTS
WALLET_LOCKED
WALLET_PASSWORD_INVALID
WALLET_KEYSTORE_CORRUPTED
WALLET_BACKUP_INVALID
WALLET_BACKUP_VERSION_UNSUPPORTED
WALLET_RECOVERY_FAILED
WALLET_SECURE_RANDOM_UNAVAILABLE
WALLET_NETWORK_NOT_FOUND
WALLET_NETWORK_MISMATCH
WALLET_NODE_UNAVAILABLE
WALLET_NODE_CHAIN_MISMATCH
WALLET_NODE_GENESIS_MISMATCH
WALLET_BALANCE_QUERY_FAILED
WALLET_NONCE_QUERY_FAILED
WALLET_FEE_ESTIMATE_FAILED
WALLET_ADDRESS_INVALID
WALLET_SELF_TRANSFER
WALLET_AMOUNT_INVALID
WALLET_AMOUNT_PRECISION_EXCEEDED
WALLET_INSUFFICIENT_BALANCE
WALLET_FEE_LIMIT_INVALID
WALLET_NONCE_CONFLICT
WALLET_TRANSACTION_EXPIRED
WALLET_SIGNING_FAILED
WALLET_SIGNATURE_SELF_CHECK_FAILED
WALLET_SUBMISSION_FAILED
WALLET_STATUS_UNKNOWN
WALLET_INTERNAL_ERROR

⸻

72. Wallet State

A wallet runtime MAY have:

CLOSED
OPEN_LOCKED
OPEN_UNLOCKED
SIGNING
BACKING_UP
RESTORING
SYNCING_METADATA
ERROR

Signing and backup operations SHOULD be mutually controlled to avoid race conditions.

⸻

73. Concurrent Use

The wallet MUST handle multiple processes or instances safely.

It SHOULD use:

* file locks;
* database transactions;
* nonce conflict detection;
* atomic keystore writes;
* clear error messages.

Two wallet copies controlling the same account may still create conflicting transactions.

The wallet MUST warn where detected.

⸻

74. Atomic Keystore Writes

Keystore updates MUST be atomic.

Recommended pattern:

write temporary file
verify temporary file
fsync where supported
rename atomically
retain recoverable previous version where configured

A crash MUST not silently destroy all wallet keys.

⸻

75. Corruption Detection

The wallet MUST detect:

* malformed keystore;
* authentication-tag failure;
* checksum mismatch;
* unsupported format;
* missing required fields;
* public-key/address mismatch.

Corrupted files MUST NOT be overwritten automatically.

⸻

76. Recovery from Partial Failure

If a non-secret metadata database fails but encrypted keys remain valid, the wallet SHOULD support rebuilding:

* addresses;
* public metadata;
* transaction history;
* network profiles.

Private keys remain authoritative for account control.

⸻

77. Wallet Update Compatibility

Wallet formats MUST be versioned.

An update MUST define:

* supported old versions;
* migration path;
* backup recommendation;
* rollback behavior;
* failure recovery.

The wallet MUST not silently downgrade or reinterpret unknown formats.

⸻

78. Remote Signing

Q1 v0.1 SHOULD NOT require remote signing.

A future remote signer MAY use:

* hardware device;
* local signing daemon;
* HSM;
* secure enclave.

Any remote signing design MUST show transaction details to the signer and preserve user authorization.

⸻

79. Hardware Wallet Future Interface

The architecture SHOULD reserve:

SignerProvider {
    get_public_key()
    get_address()
    sign_transaction(unsigned_payload)
    confirm_on_device()
}

No production hardware-wallet support is required in v0.1.

⸻

80. Threat Model

The wallet MUST consider:

* malware;
* stolen keystore;
* weak password;
* clipboard replacement;
* phishing node;
* fake network profile;
* malicious explorer;
* modified unsigned transaction;
* compromised backup;
* accidental deletion;
* disk corruption;
* shoulder surfing;
* terminal history;
* shell logging;
* key reuse;
* nonce conflicts;
* social engineering;
* malicious wallet build.

⸻

81. Malware Limitation

Software encryption cannot fully protect keys while the wallet is unlocked on a compromised device.

The wallet MUST not overstate security.

Offline signing and future hardware signing SHOULD be recommended for higher-risk use.

⸻

82. Terminal Safety

The CLI SHOULD avoid:

* accepting passwords as command-line arguments;
* printing secrets;
* storing secrets in shell history;
* exposing secrets in process lists.

Passwords SHOULD be entered through secure prompt input.

⸻

83. Build Authenticity

Future release processes SHOULD provide:

* signed binaries;
* reproducible builds;
* checksums;
* release notes;
* source-code tags.

Q1 v0.1 MAY begin with source builds but MUST document the exact commit used.

⸻

84. Experimental Warning

The wallet MUST display a clear warning on startup.

Suggested:

Q1 v0.1 is an experimental research network.
Q1T units have no guaranteed financial value.
Do not use real funds or reuse important private keys.

The warning MAY be suppressible only through explicit configuration in test automation.

⸻

85. No Real-Key Reuse

The wallet SHOULD warn users not to import or reuse keys from:

* Bitcoin;
* Ethereum;
* other cryptocurrencies;
* production wallets;
* personal identity systems.

Q1 test keys SHOULD be unique to Q1.

⸻

86. Telemetry

Wallet telemetry MUST be optional and privacy-minimized.

It MAY include:

* software version;
* command success or failure;
* node response latency;
* transaction status timing;
* non-secret error codes.

It MUST NOT include:

* private keys;
* recovery phrase;
* password;
* full wallet contents;
* personal labels;
* raw unsigned secrets.

⸻

87. Wallet Metrics

The test environment SHOULD measure:

* wallet creation success;
* signing duration;
* fee-estimate accuracy;
* submission latency;
* time to mempool acceptance;
* time to finalization;
* restore success;
* backup verification rate;
* user error frequency;
* nonce-conflict rate;
* node-disagreement events.

⸻

88. Required Functional Tests

The automated suite MUST include:

1. wallet creation;
2. secure random failure;
3. account creation;
4. address derivation;
5. address checksum validation;
6. wrong-network address;
7. wallet lock and unlock;
8. wrong password;
9. encrypted keystore persistence;
10. corrupted keystore;
11. balance query;
12. nonce query;
13. fee estimation;
14. valid transaction build;
15. zero amount;
16. negative amount;
17. excessive precision;
18. amount overflow;
19. insufficient balance;
20. self-transfer;
21. invalid recipient;
22. network mismatch;
23. transaction preview;
24. signing;
25. local signature self-check;
26. transaction ID derivation;
27. submission accepted;
28. submission rejected;
29. node unavailable;
30. status tracking;
31. finalized status;
32. expired transaction;
33. conflicting nonce;
34. replacement transaction;
35. backup creation;
36. backup verification;
37. backup corruption;
38. restore;
39. duplicate restore;
40. watch-only wallet;
41. offline unsigned export;
42. offline signing;
43. signed import;
44. altered offline package;
45. multi-node disagreement;
46. wallet file lock;
47. crash during keystore write;
48. metadata rebuild;
49. experimental warning;
50. secret-log scanning.

⸻

89. Security Tests

The test team MUST attempt:

* password brute-force against test files;
* keystore tampering;
* ciphertext replacement;
* nonce reuse;
* clipboard address substitution;
* malicious node fee inflation;
* false finalized status;
* wrong-chain node response;
* unsigned transaction modification;
* signed transaction modification;
* backup theft;
* shell-history leakage;
* process-list leakage;
* terminal output leakage;
* race conditions between wallet instances;
* fake explorer history;
* compromised watch-only data;
* transaction replacement confusion.

⸻

90. Minimum Wallet Prototype Acceptance

The Q1 wallet prototype is complete when:

1. a user can create an encrypted wallet;
2. a user can create at least one account;
3. the wallet displays a valid network-specific address;
4. the wallet can query finalized balance;
5. the wallet can query nonce;
6. the wallet can estimate a fee;
7. the wallet can build a canonical unsigned transaction;
8. the wallet displays a complete preview;
9. the wallet signs locally;
10. the wallet verifies its own signature;
11. the wallet submits to a node;
12. the wallet tracks status through finalization;
13. wrong-network addresses are rejected;
14. private keys never reach the node;
15. backup creation and restoration succeed;
16. watch-only mode works;
17. offline signing works;
18. corrupted keystores are detected;
19. secrets do not appear in normal logs;
20. testnet warnings remain visible.

⸻

91. Private Testnet Acceptance

Before private distributed use, the wallet MUST demonstrate:

* successful use against multiple nodes;
* stable network-profile separation;
* correct finalized balances after 10,000 transfers;
* correct nonce management;
* fee-estimation error measurement;
* no accidental plaintext secret storage;
* successful restore on another machine;
* successful offline signing;
* safe behavior under node disagreement;
* clear rejection messages;
* reproducible transaction IDs;
* compatibility with the finalized transaction specification.

⸻

92. Known Limitations

Q1 Wallet v0.1 does not yet guarantee:

* malware resistance;
* hardware isolation;
* social recovery;
* strong privacy;
* mobile security;
* browser-extension safety;
* legal identity verification;
* chargebacks;
* administrator recovery;
* protection from user-confirmed fraud;
* production custody standards.

These limitations MUST remain explicit.

⸻

93. Open Decisions

The following remain unresolved:

1. exact signature algorithm;
2. exact encrypted-keystore algorithms;
3. exact password KDF;
4. whether deterministic seed phrases are included in milestone one;
5. exact recovery phrase format;
6. exact address encoding;
7. exact CLI framework;
8. exact wallet database;
9. exact transaction-history source;
10. exact multi-node cross-checking rules;
11. exact fee-preference mapping;
12. exact default validity window;
13. exact replacement workflow;
14. whether transaction cancellation is exposed;
15. exact QR format;
16. exact payment-request format;
17. whether public keys are stored per account or derived;
18. whether multiple accounts derive from one seed;
19. exact auto-lock duration;
20. exact backup format;
21. whether wallet metadata is encrypted;
22. whether a future GUI shares the same wallet process;
23. whether hardware-wallet adapters are included in v0.1 architecture;
24. exact reproducible-build process;
25. whether wallet telemetry is enabled by default in private testing.

All decisions MUST be recorded in OPEN_DECISIONS.md.

⸻

94. Codex Implementation Rules

Codex MUST:

1. separate wallet core from CLI;
2. separate key storage from node communication;
3. encrypt private keys at rest;
4. use secure random generation;
5. avoid command-line password arguments;
6. use checked integer arithmetic for amounts;
7. reuse canonical transaction serialization from the protocol library;
8. display complete transaction preview before signing;
9. verify signatures locally after signing;
10. verify network and genesis identity;
11. distinguish pending from finalized;
12. preserve signed transactions after submission failure;
13. implement watch-only mode;
14. implement offline signing packages;
15. implement backup verification;
16. use atomic keystore writes;
17. detect file corruption;
18. keep secrets out of logs;
19. expose stable error codes;
20. retain the experimental warning.

Codex MUST NOT:

* transmit private keys to a node;
* trust explorer data as spending authority;
* use floating-point values for amounts;
* silently switch networks;
* sign without user-visible details;
* mark mempool acceptance as final;
* store wallet passwords;
* print recovery phrases in ordinary output;
* overwrite corrupted keystores automatically;
* imply recoverability by project operators;
* reuse node keys as wallet keys;
* hide maximum fee authorization.

⸻

95. Final Wallet Principle

The Q1 Wallet MUST preserve this principle:

The network may verify a signature, but only the user should authorize it.
A node may report a balance, but the wallet must know which network and finalized state that report belongs to.
A transaction may be submitted quickly, but it becomes final only through consensus.
A password may protect a file, but only a valid backup protects against loss.
Convenience may simplify the experience, but it must never silently replace control.

Q1 succeeds at the wallet layer when an ordinary user can:

* create an account;
* understand the active network;
* see the exact amount and maximum fee;
* authorize a transaction locally;
* track it honestly;
* recover the wallet from a valid backup;
* and keep control of spending keys

without trusting an explorer, node operator, AI observer, or central custodian.
