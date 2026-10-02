Q1 Node and Networking Specification

08_NODE_AND_NETWORKING.md

Project: Q1 Experimental Distributed Ledger
Protocol Version: 0.1
Document Version: 0.1.0
Status: Draft for Engineering Review
Classification: Experimental — Not for Production or Financial Use

⸻

1. Purpose

This document defines the Q1 node runtime and peer-to-peer networking model.

It specifies:

* node identity;
* node roles;
* startup and shutdown;
* peer discovery;
* connection establishment;
* handshake;
* protocol negotiation;
* message framing;
* transaction propagation;
* block propagation;
* consensus-message propagation;
* chain synchronization;
* peer scoring;
* rate limiting;
* bandwidth protection;
* eclipse resistance;
* partition behavior;
* bootstrap behavior;
* recovery after disconnection;
* networking telemetry;
* adversarial test requirements.

The objective is to create a network where independently operated nodes can:

* discover one another;
* exchange authenticated protocol messages;
* maintain compatible finalized state;
* resist malformed and abusive traffic;
* continue operating despite partial failures;
* and recover without trusting a central server.

⸻

2. Core Networking Principles

2.1 Every peer is untrusted

A peer connection MUST NOT imply:

* honesty;
* protocol correctness;
* valid chain state;
* independent ownership;
* valid geography;
* valid device identity;
* or consensus authority.

Every received object MUST be independently validated.

⸻

2.2 Networking does not define validity

The network transports:

* transactions;
* blocks;
* attestations;
* certificates;
* evidence;
* synchronization data.

The deterministic protocol core decides whether these objects are valid.

⸻

2.3 No single mandatory server

Q1 MUST NOT require one permanent central server for:

* consensus;
* peer discovery;
* block validity;
* transaction propagation;
* chain synchronization.

Bootstrap services MAY exist, but nodes MUST be able to continue after bootstrap services disappear.

⸻

2.4 Bounded resource use

Every network-facing operation MUST have:

* size limits;
* rate limits;
* timeouts;
* concurrency limits;
* validation stages;
* failure handling.

⸻

2.5 Multiple independent peers

A node SHOULD maintain connections to several unrelated peers.

It SHOULD avoid dependence on:

* one IP range;
* one operator;
* one data center;
* one autonomous system;
* one transport path;
* one bootstrap source.

⸻

3. Node Roles

A Q1 node MAY operate in one or more roles.

FULL_NODE
VALIDATOR
PRODUCER
OBSERVER
ARCHIVE_NODE
BOOTSTRAP_NODE
LIGHT_NODE
INDEXER_SOURCE
TEST_NODE
MALICIOUS_TEST_NODE

The first implementation MUST support:

FULL_NODE
VALIDATOR
PRODUCER
OBSERVER

Role declarations are capabilities, not proof of honesty.

⸻

4. Node Identity

Each node MUST possess a long-lived node identity key pair distinct from wallet spending keys.

NodeIdentity {
    node_public_key
    node_id
    identity_version
}

Conceptually:

node_id =
    HASH(
        DOMAIN_Q1_NODE_ID
        || node_public_key
    )

⸻

Q1-NET-001 — Separate keys

Node identity keys MUST NOT be reused as:

* wallet spending keys;
* producer signing keys;
* validator attestation keys;
* administrative API credentials.

⸻

Q1-NET-002 — Identity persistence

A node SHOULD preserve its node identity across ordinary restarts.

⸻

Q1-NET-003 — Identity rotation

Node identity rotation MAY be supported.

Rotation MUST be explicit and MUST NOT allow reputation or penalties to be silently escaped where those features apply.

⸻

Q1-NET-004 — No proof of uniqueness

One node identity does not prove one person, one machine, or one operator.

Sybil resistance MUST NOT rely solely on node IDs.

⸻

5. Node Capability Record

During handshake, a node MAY advertise:

NodeCapabilities {
    protocol_versions[]
    chain_ids[]
    node_roles[]
    full_history
    snapshot_service
    transaction_relay
    block_relay
    consensus_relay
    archive_service
    hdd_plugin_available
    ai_observer_available
    compression_support[]
    transport_support[]
}

Capabilities are claims and MUST be treated as untrusted until behavior confirms them.

⸻

6. Node Runtime States

A node MAY pass through:

CREATED
STARTING
LOADING_CONFIG
LOADING_GENESIS
OPENING_STORAGE
LOADING_IDENTITY
STARTING_NETWORK
DISCOVERING_PEERS
SYNCHRONIZING
READY
PARTICIPATING
DEGRADED
SAFE_MODE
STOPPING
STOPPED
FAILED

A node MUST NOT produce or attest while materially unsynchronized.

⸻

7. Startup Procedure

A node SHOULD start in the following order:

1. Load local configuration.
2. Validate configuration syntax.
3. Load genesis configuration.
4. Verify genesis hash.
5. Load protocol version.
6. Open ledger storage.
7. Verify local database integrity.
8. Load node identity.
9. Load role-specific keys.
10. Initialize protocol core.
11. Initialize mempool.
12. Start networking listeners.
13. Load static peers and bootstrap addresses.
14. Begin peer discovery.
15. Determine synchronization status.
16. Synchronize finalized state.
17. Activate configured roles.
18. Expose readiness status.

⸻

8. Graceful Shutdown

A node MUST support graceful shutdown.

Shutdown SHOULD:

1. Stop accepting new admin operations.
2. Stop producer scheduling.
3. Prevent new validator signatures.
4. Cancel active delay and HDD work.
5. Stop new peer dials.
6. Flush consensus evidence.
7. Persist validator signing history.
8. Persist mempool where configured.
9. Flush ledger database.
10. Close peer connections.
11. Emit shutdown telemetry.
12. Exit cleanly.

A forced shutdown MUST not corrupt finalized state.

⸻

9. Network Transport

The initial implementation MAY use:

* TCP;
* QUIC;
* WebSocket for development tools;
* another documented reliable transport.

The transport layer MUST be abstracted.

Recommended interface:

Transport {
    listen(address)
    dial(address)
    accept()
    open_stream(protocol_id)
    send(frame)
    receive()
    close()
}

Consensus logic MUST NOT depend directly on TCP-specific behavior.

⸻

10. Addressing

A peer address SHOULD identify:

* transport;
* host or IP;
* port;
* node ID where known.

Conceptual form:

/transport/host/port/node-id

The final encoding remains an engineering decision.

Nodes MUST support manually configured peer addresses for private testnets.

⸻

11. Connection Types

Q1 SHALL distinguish:

INBOUND
OUTBOUND
STATIC
BOOTSTRAP
CONSENSUS_PREFERRED
SYNC
TRANSIENT
TEST

A single peer MAY belong to more than one category.

⸻

12. Minimum Peer Targets

Initial recommendations:

minimum_outbound_peers = 4
target_outbound_peers = 8
maximum_outbound_peers = 16
maximum_inbound_peers = 32

Private local tests MAY use smaller values.

These values MUST be configurable.

⸻

13. Peer Diversity

Outbound peer selection SHOULD avoid excessive concentration.

Diversity signals MAY include:

* distinct IP prefixes;
* distinct autonomous systems;
* distinct DNS origins;
* distinct geographic regions;
* distinct operators where known;
* distinct client versions;
* distinct transport paths.

These signals are imperfect and MUST NOT be treated as identity proof.

⸻

14. Peer Discovery Sources

Q1 MAY use multiple discovery mechanisms.

14.1 Static peers

Defined manually in configuration.

Required for early private testnets.

⸻

14.2 Bootstrap nodes

Known nodes that provide initial peer addresses.

Bootstrap nodes MUST NOT define chain truth.

⸻

14.3 Peer exchange

Connected peers may share known peer records.

⸻

14.4 DNS-based discovery

MAY be used for public testnet convenience.

It MUST NOT be the only discovery method.

⸻

14.5 Distributed peer routing

A future implementation MAY use a DHT or similar discovery network.

Not required for the first milestone.

⸻

15. Bootstrap Node Rules

Bootstrap nodes MAY:

* accept new connections;
* provide peer records;
* expose protocol versions;
* help nodes enter the network.

Bootstrap nodes MUST NOT:

* sign on behalf of other validators;
* certify balances;
* decide canonical chain state;
* bypass genesis verification;
* become mandatory after initial discovery.

A node that already knows valid peers SHOULD continue if all bootstrap nodes disappear.

⸻

16. Peer Record

A shared peer record MAY contain:

PeerRecord {
    peer_record_version
    node_id
    reachable_addresses[]
    supported_protocol_versions[]
    chain_ids[]
    capabilities[]
    record_expiration
    signer_node_id
    signature
}

A peer record is a signed claim, not proof of current reachability.

⸻

17. Handshake

Every connection MUST complete a handshake before normal protocol traffic.

Handshake goals:

* identify the peer;
* verify node identity signature;
* confirm chain compatibility;
* negotiate protocol version;
* negotiate message limits;
* exchange capabilities;
* detect self-connections;
* prevent replay of old handshakes.

⸻

18. Handshake Challenge

Each side MUST issue a fresh random challenge.

Conceptual:

HandshakeChallenge {
    challenge_version
    random_nonce
    local_node_id
    remote_address_observed
    timestamp_observational
}

The remote peer signs the challenge and session context.

⸻

19. Handshake Message

HandshakeMessage {
    handshake_version
    node_id
    node_public_key
    supported_protocol_versions[]
    chain_ids[]
    capabilities[]
    genesis_hashes[]
    best_finalized_height
    best_finalized_block_hash
    random_challenge
    response_to_remote_challenge
    session_ephemeral_key_optional
    signature
}

⸻

20. Handshake Validation

A node MUST reject the connection if:

* node ID does not match public key;
* signature is invalid;
* no supported protocol version overlaps;
* target chain is unsupported;
* genesis hash conflicts;
* peer claims the local node’s own identity;
* handshake is malformed;
* challenge response is invalid;
* message exceeds size limits;
* peer is actively banned.

⸻

21. Genesis Mismatch

Nodes with different genesis hashes MUST NOT synchronize or exchange consensus objects for the same chain.

The connection MAY remain only for explicit multi-chain support in future versions.

For v0.1, it SHOULD be closed.

⸻

22. Protocol Negotiation

The highest mutually supported compatible protocol version SHOULD be selected.

Unknown protocol versions MUST NOT be silently interpreted.

Negotiated versions SHOULD include:

* network protocol;
* transaction protocol;
* block protocol;
* consensus message protocol;
* compression;
* optional extensions.

⸻

23. Session Authentication

After handshake, messages SHOULD be bound to the authenticated session.

The transport MAY use:

* encrypted authenticated sessions;
* signed messages;
* or both.

Consensus-critical objects MUST retain their own object-level signatures even if the transport is encrypted.

Transport security does not replace protocol signatures.

⸻

24. Message Envelope

All P2P messages MUST use a canonical envelope.

MessageEnvelope {
    network_protocol_version
    chain_id
    message_type
    message_id
    request_id_optional
    response_to_optional
    sender_node_id
    payload_length
    compression
    payload_hash
    payload
    session_authentication
}

⸻

25. Message Identifier

Conceptually:

message_id =
    HASH(
        DOMAIN_Q1_NETWORK_MESSAGE
        || chain_id
        || message_type
        || canonical_payload
    )

Transport retransmission metadata MUST NOT alter the object identifier.

⸻

26. Message Size Limits

Configuration MUST define maximum sizes for:

* handshake messages;
* peer records;
* transaction announcements;
* full transactions;
* block announcements;
* block bodies;
* attestations;
* finalization certificates;
* synchronization batches;
* evidence;
* snapshots;
* compressed payloads;
* decompressed payloads.

Nodes MUST reject oversized messages before expensive processing where possible.

⸻

27. Compression Safety

If compression is enabled, nodes MUST protect against:

* decompression bombs;
* excessive expansion ratio;
* recursive compression;
* malformed streams;
* CPU exhaustion.

Configuration MUST define:

maximum_compressed_size
maximum_decompressed_size
maximum_expansion_ratio

⸻

28. Message Categories

Q1 messages SHALL be separated into domains.

HANDSHAKE
PEER_DISCOVERY
STATUS
TRANSACTION
BLOCK
CONSENSUS
SYNC
EVIDENCE
SNAPSHOT
PING
ADMIN_TEST_ONLY

Each domain MUST have independent limits and priorities.

⸻

29. Message Priority

Recommended priority order:

1. Finalization certificates
2. Validator attestations
3. Valid block proposals
4. Round and timeout messages
5. Block synchronization
6. Transaction propagation
7. Peer discovery
8. Telemetry or optional data

Low-priority traffic MUST NOT starve consensus messages.

⸻

30. Request-Response Protocol

Large objects SHOULD use request-response exchange.

Example:

Announcement → ObjectRequest → ObjectResponse

This applies to:

* transactions;
* blocks;
* attestations;
* certificates;
* evidence;
* snapshots.

Nodes SHOULD avoid sending full large objects unsolicited to every peer.

⸻

31. Transaction Propagation

Recommended flow:

1. Node accepts transaction into local mempool.
2. Node announces transaction ID to selected peers.
3. Peer checks whether transaction is already known.
4. Missing peer requests full transaction.
5. Sender transmits canonical transaction.
6. Receiver validates independently.
7. Receiver may propagate announcement onward.

⸻

32. Transaction Relay Rules

A node SHOULD relay a transaction only if:

* canonical decoding succeeds;
* size limits pass;
* chain ID is correct;
* basic validity passes;
* signature is valid;
* transaction is not known;
* local rate limits allow it.

A node MUST NOT relay clearly invalid transactions.

⸻

33. Transaction Fanout

A node need not announce every transaction to every peer.

Fanout MUST be configurable.

The test suite SHOULD compare:

* full broadcast;
* random subset;
* adaptive fanout;
* epidemic relay;
* stem-and-fluff-like research modes.

The first implementation SHOULD use simple bounded random fanout.

⸻

34. Block Propagation

A producer SHOULD broadcast a compact block announcement first.

BlockAnnouncement {
    block_hash
    block_height
    round_number
    parent_hash
    producer_id
    transaction_count
    block_size
    delay_engine_id
}

Peers missing the block request the full proposal.

⸻

35. Compact Block Research

Future versions MAY reconstruct blocks from known mempool transactions using compact references.

Not required for v0.1.

The first implementation SHOULD prioritize correctness and observability over bandwidth optimization.

⸻

36. Consensus Message Propagation

Consensus messages MUST receive high priority.

Nodes SHOULD rapidly relay:

* valid block proposals;
* attestations;
* finalization certificates;
* round timeout certificates;
* equivocation evidence;
* safe-mode notices.

Before relaying, nodes SHOULD perform cheap structural and signature checks.

Full block validation is required before a validator signs, but not necessarily before every relay.

⸻

37. Finalization Certificate Propagation

A valid finalization certificate SHOULD be announced immediately.

A receiving node MUST:

1. verify certificate structure;
2. verify committee membership;
3. verify signatures;
4. verify threshold;
5. obtain the referenced block if missing;
6. verify the block;
7. commit only after all checks pass.

A certificate without its block MUST NOT alter ledger state.

⸻

38. Peer Scoring

Nodes MAY maintain a local peer score.

Possible positive behavior:

* successful handshake;
* valid objects;
* useful synchronization data;
* timely responses;
* stable connectivity;
* diverse network path.

Possible negative behavior:

* malformed messages;
* invalid signatures;
* invalid transactions;
* invalid blocks;
* repeated duplicates;
* request timeouts;
* excessive rates;
* conflicting claims;
* protocol violations;
* unsolicited large payloads.

Peer score is local policy and MUST NOT change ledger state.

⸻

39. Peer Score Structure

PeerScore {
    connection_quality
    protocol_compliance
    useful_data_score
    invalid_data_penalty
    rate_limit_penalty
    timeout_penalty
    diversity_bonus_optional
    last_updated
}

Score calculations MAY use floating point locally because they are non-consensus.

⸻

40. Peer Penalties

Local actions MAY include:

WARN
THROTTLE
IGNORE_OBJECT
DISCONNECT
TEMPORARY_BAN
LONGER_BAN

Permanent global bans MUST NOT be inferred solely from one node’s local score.

Objective cryptographic evidence may support protocol-level penalties later.

⸻

41. Ban Records

A local ban record SHOULD include:

peer_id
reason_code
evidence_hash_optional
start_time_local
expiration_time_local
manual_or_automatic

Ban expiration MUST be configurable.

⸻

42. Rate Limiting

Nodes MUST enforce rate limits by:

* peer;
* IP or network prefix where appropriate;
* message domain;
* request type;
* object size;
* verification cost.

Recommended mechanisms:

* token bucket;
* leaky bucket;
* concurrency semaphore;
* per-peer queue;
* global emergency limits.

⸻

43. Expensive Validation Protection

Nodes SHOULD perform validation in stages.

Example:

Stage 1: frame length
Stage 2: message type and version
Stage 3: canonical decoding
Stage 4: cheap field checks
Stage 5: hash and duplicate checks
Stage 6: signature verification
Stage 7: state-dependent validation
Stage 8: expensive proof verification

Invalid traffic SHOULD be rejected as early as possible.

⸻

44. Verification Queues

Expensive work SHOULD use bounded queues.

Separate queues SHOULD exist for:

* transaction signatures;
* block validation;
* delay-proof verification;
* attestation verification;
* snapshot verification;
* HDD evidence verification.

One traffic type MUST NOT exhaust all worker capacity.

⸻

45. Backpressure

When queues reach limits, the node MAY:

* delay reads;
* reject low-priority requests;
* stop accepting new inbound peers;
* reduce transaction relay;
* preserve consensus traffic;
* disconnect abusive peers.

Backpressure MUST be observable through telemetry.

⸻

46. Ping and Liveness

Nodes SHOULD exchange lightweight liveness messages.

Ping {
    nonce
    sent_time_observational
}
Pong {
    nonce
}

Ping results MAY inform:

* connection health;
* local latency estimate;
* timeout policy;
* peer score.

They MUST NOT establish consensus time.

⸻

47. Status Exchange

Peers SHOULD periodically exchange:

NodeStatus {
    chain_id
    protocol_version
    finalized_height
    finalized_block_hash
    current_round
    current_epoch
    participant_set_root
    mempool_size_optional
    safe_mode
}

Status claims are untrusted until verified through blocks and certificates.

⸻

48. Chain Synchronization Goals

A synchronizing node MUST reach the latest valid finalized state.

Synchronization SHALL prefer finalized history over unfinalized proposals.

A node MUST verify:

* genesis;
* block headers;
* parent links;
* finalization certificates;
* protocol versions;
* state transitions or trusted snapshot proofs;
* participant-set updates.

⸻

49. Synchronization Modes

49.1 Full Sync

Downloads and verifies every finalized block from genesis.

Advantages:

* strongest independent reconstruction.

Disadvantages:

* slowest.

⸻

49.2 Header-First Sync

Downloads finalized headers and certificates first.

Then downloads block bodies.

⸻

49.3 Block-Range Sync

Requests finalized blocks in bounded ranges.

⸻

49.4 Snapshot-Assisted Sync

Downloads a state snapshot and supporting proof, then verifies later blocks.

Snapshots MUST NOT be trusted merely because many peers advertise them.

⸻

49.5 Recovery Sync

Used after local corruption or prolonged disconnection.

⸻

50. Synchronization State Machine

SYNC_NOT_STARTED
DISCOVERING_TIPS
SELECTING_PEERS
SYNCING_HEADERS
VERIFYING_HEADERS
SYNCING_BLOCKS
VERIFYING_BLOCKS
SYNCING_STATE
VERIFYING_STATE
CATCHING_UP
SYNCED
SYNC_FAILED
SAFE_MODE

⸻

51. Tip Discovery

A node SHOULD query several peers for:

* finalized height;
* finalized block hash;
* protocol version;
* participant-set root.

It MUST NOT trust the highest claimed height automatically.

The node SHOULD prefer a chain supported by valid finalization certificates.

⸻

52. Sync Peer Selection

Sync peers SHOULD be selected based on:

* compatible genesis;
* protocol support;
* valid certificate history;
* response quality;
* network diversity;
* sufficient finalized height;
* acceptable peer score.

A node SHOULD use multiple peers for cross-checking.

⸻

53. Header Verification

Each finalized header MUST be checked for:

* correct parent;
* valid height;
* valid protocol version;
* valid finalization certificate;
* correct chain ID;
* correct participant-set context;
* no conflict with known finalized history.

⸻

54. Block Verification During Sync

A full-sync node MUST independently verify:

* transactions;
* delay proof;
* producer eligibility;
* committee;
* attestations;
* state transition;
* roots;
* fees;
* rewards;
* supply.

Archive indexes are not authoritative.

⸻

55. Snapshot Structure

A snapshot MAY contain:

Snapshot {
    snapshot_version
    chain_id
    block_height
    block_hash
    state_root
    participant_set_root
    protocol_parameters_hash
    chunk_manifest
    snapshot_hash
}

⸻

56. Snapshot Verification

A node MUST verify:

* snapshot metadata;
* referenced finalized block;
* finalization certificate;
* chunk hashes;
* reconstructed state root;
* protocol compatibility.

A snapshot without a verified finalized reference MUST be rejected.

⸻

57. Snapshot Sources

Snapshots MAY be served by:

* archive nodes;
* full nodes;
* bootstrap infrastructure;
* mirrors;
* local files.

No snapshot server has special authority.

⸻

58. Mempool Synchronization

Mempools are not consensus state.

After synchronizing finalized state, a node MAY request:

* transaction announcements;
* selected pending transactions;
* local-wallet pending transactions.

It MUST reevaluate every received transaction against current finalized state.

⸻

59. Reconnection Behavior

After temporary disconnection, a node SHOULD:

1. Reconnect to diverse peers.
2. Exchange status.
3. Verify finalized tip.
4. Download missing finalized objects.
5. Reevaluate local mempool.
6. discard stale proposals.
7. detect missed equivocation evidence.
8. resume roles only after synchronization.

⸻

60. Network Partition

During partition, a node MUST preserve safety.

A minority partition without quorum MUST NOT fabricate finality.

Nodes MAY:

* collect transactions;
* maintain local mempools;
* record unfinalized proposals;
* wait for reconnection;
* expose degraded status.

⸻

61. Partition Detection

Partition indicators MAY include:

* sudden loss of many peers;
* conflicting tip claims;
* absence of expected attestations;
* reduced network diversity;
* unreachable bootstrap sources;
* sustained round timeouts.

Partition detection is observational.

It does not itself change ledger state.

⸻

62. Partition Recovery

After reconnection:

* valid finalization certificates determine finalized history;
* conflicting finalized certificates trigger safe mode;
* unfinalized local proposals are discarded or archived;
* mempool entries are reevaluated;
* peer scores are updated carefully;
* objective equivocation evidence is propagated.

⸻

63. Eclipse Attacks

An eclipse attack attempts to surround a node with attacker-controlled peers.

Q1 SHOULD mitigate eclipse risk through:

* multiple outbound peers;
* peer diversity;
* persistent known-good peers;
* periodic peer rotation;
* independent bootstrap sources;
* connection limits per network prefix;
* separate inbound and outbound slots;
* manual trusted peer options for testnets;
* status cross-checking;
* finalized-certificate verification.

⸻

64. Prefix Limits

A node SHOULD limit simultaneous peers from the same:

* IPv4 prefix;
* IPv6 prefix;
* autonomous system where known;
* DNS origin;
* operator tag where known.

These limits MUST be configurable.

⸻

65. Peer Rotation

Some outbound peers SHOULD be rotated periodically.

The node SHOULD preserve a subset of stable peers while testing new connections.

Rotation MUST NOT interrupt consensus-critical traffic unnecessarily.

⸻

66. Feeler Connections

A node MAY create short-lived test connections to discover new peers and test reachability.

Feeler connections SHOULD have strict resource limits.

⸻

67. Self-Connection Detection

A node MUST reject connections to itself.

Detection MAY use:

* node ID;
* known listening addresses;
* handshake challenge behavior.

⸻

68. Duplicate Connections

If two nodes establish multiple simultaneous connections, they SHOULD deterministically keep one preferred connection.

The rule MUST avoid endless connection replacement.

Possible tie-breaker:

keep connection selected by ordered node IDs and direction

The exact rule remains configurable.

⸻

69. NAT and Reachability

Nodes behind NAT MAY operate with outbound-only connections.

Inbound reachability MUST NOT be required for basic validation.

Producer and validator roles MAY function with outbound connectivity if protocol message propagation remains adequate.

⸻

70. Relay-Only Nodes

A node MAY operate as a relay or observer without holding consensus roles.

Relay behavior MUST still validate basic message structure and enforce limits.

⸻

71. Light Nodes

Q1 v0.1 MAY defer full light-client implementation.

A future light node MAY verify:

* finalized headers;
* committee certificates;
* inclusion proofs;
* account proofs.

It MUST not simply trust explorer responses.

⸻

72. Archive Nodes

Archive nodes retain full historical data.

They MAY serve:

* old blocks;
* receipts;
* evidence;
* snapshots;
* explorer indexers.

Archive status does not grant greater consensus weight.

⸻

73. Message Replay Protection

Nodes MUST reject or deduplicate replayed:

* handshake responses;
* transactions;
* proposals;
* attestations;
* certificates;
* timeout messages;
* evidence.

Replay protection SHALL use:

* object identifiers;
* chain ID;
* height;
* round;
* expiration;
* known-object caches.

⸻

74. Known-Object Cache

Each node SHOULD maintain bounded caches for known:

* transaction IDs;
* block hashes;
* attestation hashes;
* certificate hashes;
* evidence hashes;
* peer records.

Caches MUST have memory limits and expiration policies.

⸻

75. Flooding Attacks

The network test suite MUST attempt:

* transaction floods;
* invalid signature floods;
* duplicate announcement floods;
* block request floods;
* attestation floods;
* peer-record floods;
* handshake floods;
* connection churn;
* oversized-message attacks;
* decompression bombs;
* slow-read attacks;
* slow-write attacks;
* expensive-proof floods.

⸻

76. Slow Peer Protection

Connections MUST have:

* handshake timeout;
* read timeout;
* write timeout;
* idle timeout;
* request timeout;
* maximum pending requests.

Slow peers MAY be deprioritized or disconnected.

⸻

77. Request Tracking

A node SHOULD track outstanding requests.

PendingRequest {
    request_id
    peer_id
    request_type
    object_id
    created_time_local
    expiration_time_local
    retry_count
}

Duplicate requests SHOULD be minimized.

⸻

78. Retry Policy

Retries MUST be bounded.

A node SHOULD retry missing objects from different peers.

It MUST avoid infinite retry loops against one failing peer.

⸻

79. Data Availability

A finalized certificate is useful only if the corresponding block data is available.

Nodes SHOULD relay and retain finalized blocks.

The protocol MUST measure:

* block availability;
* peer response rates;
* block retrieval latency;
* dependence on archive nodes.

Future versions MAY add explicit data-availability mechanisms.

⸻

80. Invalid Data Evidence

Receiving invalid data does not always prove malicious intent.

Nodes SHOULD distinguish:

* malformed data;
* stale data;
* unsupported version;
* objectively signed equivocation;
* repeated protocol abuse.

Only objectively verifiable evidence should support protocol-level penalties.

⸻

81. Node Health API

A node SHOULD expose:

startup_state
sync_state
finalized_height
peer_count
outbound_peer_count
inbound_peer_count
network_diversity_summary
current_round
role_status
safe_mode
database_health
delay_engine_health
hdd_plugin_health
telemetry_health

Private peer details SHOULD require administrative access.

⸻

82. Networking Telemetry

The node MUST record:

* connection attempts;
* successful handshakes;
* failed handshakes;
* inbound and outbound peers;
* connection duration;
* bytes sent and received;
* message counts by type;
* invalid-message counts;
* duplicate-message counts;
* request latency;
* block propagation time;
* transaction propagation time;
* attestation propagation time;
* certificate propagation time;
* peer-score changes;
* disconnect reasons;
* ban actions;
* network-prefix concentration;
* synchronization progress;
* partition indicators.

⸻

83. Privacy and Metadata

Networking exposes metadata.

Q1 v0.1 does not guarantee network-layer anonymity.

Nodes and users MUST understand that peers may observe:

* IP addresses;
* connection timing;
* transaction propagation timing;
* node capabilities;
* approximate network topology.

Future privacy research MAY include:

* proxy support;
* Tor or similar networks;
* transaction-origin obfuscation;
* private peer discovery.

Not required for v0.1.

⸻

84. API and P2P Separation

Public APIs and P2P listeners MUST be logically separated.

Administrative APIs SHOULD bind to localhost by default.

A public P2P peer MUST NOT gain administrative privileges.

⸻

85. Key Storage

Node identity, producer, and validator keys MUST be protected.

Networking code MUST NOT expose private key material.

Signing SHOULD occur through a dedicated key provider.

⸻

86. Structured Network Errors

Recommended codes:

NET_HANDSHAKE_TIMEOUT
NET_HANDSHAKE_INVALID_SIGNATURE
NET_HANDSHAKE_WRONG_CHAIN
NET_HANDSHAKE_GENESIS_MISMATCH
NET_HANDSHAKE_UNSUPPORTED_VERSION
NET_SELF_CONNECTION
NET_DUPLICATE_CONNECTION
NET_MESSAGE_TOO_LARGE
NET_DECOMPRESSION_LIMIT
NET_MALFORMED_FRAME
NET_UNSUPPORTED_MESSAGE
NET_RATE_LIMITED
NET_REQUEST_TIMEOUT
NET_RESPONSE_MISMATCH
NET_INVALID_OBJECT
NET_PEER_BANNED
NET_CONNECTION_LIMIT
NET_SYNC_FAILED
NET_SNAPSHOT_INVALID
NET_PARTITION_SUSPECTED
NET_INTERNAL_ERROR

⸻

87. Configuration Parameters

Q1 networking configuration MUST include:

listen_addresses
advertised_addresses
static_peers
bootstrap_peers
minimum_outbound_peers
target_outbound_peers
maximum_outbound_peers
maximum_inbound_peers
maximum_connections_per_prefix
handshake_timeout
request_timeout
idle_timeout
message_size_limits
compression_limits
rate_limits
queue_limits
peer_rotation_interval
ban_durations
sync_batch_size
snapshot_enabled
transaction_relay_enabled
block_relay_enabled
consensus_relay_enabled

Consensus-critical network identifiers MUST align with genesis.

⸻

88. Local Multi-Node Requirements

The local test environment MUST allow:

* multiple nodes on one machine;
* separate ports;
* separate node identities;
* separate data directories;
* manual topology configuration;
* artificial latency;
* packet loss;
* message reordering;
* partitions;
* bandwidth limits;
* node restarts.

⸻

89. Private Testnet Requirements

A private distributed testnet SHOULD include:

* at least seven nodes;
* at least three physical hosts;
* more than one network provider where possible;
* at least two bootstrap sources;
* at least one archive node;
* one observer-only node;
* one node behind NAT;
* one intentionally slow node;
* one malicious test node.

⸻

90. Required Networking Tests

The automated and integration test suite MUST include:

1. valid handshake;
2. invalid handshake signature;
3. wrong chain ID;
4. genesis mismatch;
5. unsupported protocol version;
6. self-connection;
7. duplicate connection;
8. static peer connection;
9. bootstrap discovery;
10. peer exchange;
11. transaction announcement;
12. transaction request and response;
13. block announcement;
14. block request and response;
15. attestation propagation;
16. finalization certificate propagation;
17. duplicate message handling;
18. malformed frame;
19. oversized message;
20. decompression bomb;
21. invalid transaction flood;
22. invalid block flood;
23. handshake flood;
24. connection churn;
25. slow peer;
26. request timeout;
27. bounded retry;
28. peer banning;
29. ban expiration;
30. peer rotation;
31. prefix concentration limit;
32. eclipse simulation;
33. partition simulation;
34. partition recovery;
35. full sync from genesis;
36. header-first sync;
37. snapshot-assisted sync;
38. invalid snapshot;
39. stale node recovery;
40. node restart;
41. corrupted local data;
42. mismatched finalized tip claims;
43. multiple sync peers;
44. mempool reevaluation after sync;
45. consensus traffic priority under transaction flood;
46. telemetry outage;
47. explorer outage;
48. bootstrap outage after discovery;
49. outbound-only node;
50. archive block retrieval.

⸻

91. Eclipse Test Criteria

The eclipse test suite MUST attempt to:

* fill all inbound slots;
* control all bootstrap responses;
* provide many Sybil peer records;
* dominate one network prefix;
* repeatedly disconnect honest peers;
* advertise false finalized heights;
* delay honest block announcements;
* isolate a validator before voting.

The node SHOULD demonstrate:

* outbound diversity;
* finalized-certificate verification;
* resistance to false height claims;
* peer rotation;
* recovery when honest peers become available.

⸻

92. Synchronization Acceptance Criteria

Synchronization is acceptable when:

* a new node begins from genesis;
* it discovers several peers;
* it obtains finalized headers;
* it verifies certificates;
* it downloads missing blocks;
* it reconstructs or verifies state;
* its final state root matches honest peers;
* it rejects conflicting invalid history;
* it resumes participation only after synchronization.

⸻

93. Minimum Node Prototype Acceptance

The node and networking subsystem is complete when:

1. four local nodes run independently;
2. every node has a distinct identity;
3. nodes complete authenticated handshakes;
4. genesis mismatch is rejected;
5. nodes exchange status;
6. transactions propagate;
7. blocks propagate;
8. attestations propagate;
9. finalization certificates propagate;
10. duplicate objects are not repeatedly processed;
11. message limits are enforced;
12. malformed traffic does not crash nodes;
13. one node can restart and resynchronize;
14. one bootstrap node can disappear without stopping the network;
15. a transaction flood does not prevent finalization traffic;
16. a temporary partition can be created and healed;
17. minority nodes do not fabricate finality;
18. peer scoring and local bans work;
19. all network events are measurable;
20. consensus validity remains independent of peer claims.

⸻

94. Private Testnet Acceptance

Before larger deployment, the network MUST demonstrate:

* stable multi-host operation;
* at least 10,000 peer message exchanges without corruption;
* at least 1,000 finalized blocks propagated;
* no finalized-state divergence among honest nodes;
* successful bootstrap independence;
* successful full-node recovery;
* successful partition recovery;
* bounded behavior under malformed traffic;
* measured propagation latency;
* measured bandwidth use;
* maintained consensus priority under transaction load;
* no single peer able to rewrite state;
* objective evidence preserved during malicious tests.

⸻

95. Known Limitations

Q1 networking v0.1 does not yet guarantee:

* global permissionless peer discovery;
* anonymity;
* censorship resistance against nation-state network control;
* complete eclipse resistance;
* DDoS immunity;
* secure mobile background networking;
* production-grade NAT traversal;
* satellite or offline relay;
* multi-client interoperability;
* formal network synchrony assumptions.

These limitations MUST be documented.

⸻

96. Open Decisions

The following remain unresolved:

1. primary transport protocol;
2. whether QUIC is used initially;
3. exact peer-address encoding;
4. exact handshake encryption;
5. exact session key agreement;
6. whether all messages are object-signed;
7. exact discovery mechanism;
8. whether a DHT is introduced;
9. exact peer-score formula;
10. exact ban durations;
11. exact prefix diversity rules;
12. transaction fanout strategy;
13. block propagation optimization;
14. snapshot distribution design;
15. exact sync batching;
16. archive-node incentives;
17. light-client protocol;
18. proxy and privacy support;
19. public bootstrap governance;
20. data-availability guarantees;
21. message compression algorithm;
22. peer-record expiration;
23. whether node identities rotate;
24. how participant identity maps to network identity;
25. whether validators require preferred persistent peers.

All decisions MUST be recorded in OPEN_DECISIONS.md.

⸻

97. Codex Implementation Rules

Codex MUST:

1. isolate transport behind an interface;
2. give every node a separate identity;
3. implement authenticated handshake;
4. verify genesis compatibility;
5. implement explicit message envelopes;
6. enforce size and rate limits;
7. separate message priorities;
8. implement bounded validation queues;
9. deduplicate known objects;
10. support static peers and bootstrap peers;
11. support multi-peer synchronization;
12. verify certificates independently;
13. separate P2P and administrative APIs;
14. implement structured networking errors;
15. expose networking telemetry;
16. support artificial latency and partitions;
17. implement graceful shutdown;
18. protect consensus traffic during load;
19. keep peer score outside consensus;
20. avoid central bootstrap dependence.

Codex MUST NOT:

* trust the highest advertised chain height;
* treat a bootstrap node as authoritative;
* accept state from an explorer;
* give one peer unlimited requests;
* decompress unbounded payloads;
* process expensive proofs before cheap checks;
* allow P2P peers to access admin APIs;
* use node ID as proof of unique person or device;
* treat peer geography as verified identity;
* allow local peer bans to alter ledger state;
* let telemetry failure stop consensus;
* assume one connection path is sufficient.

⸻

98. Final Networking Principle

Q1 networking MUST preserve this principle:

A network message is only a claim until it is verified.
A peer is only a transport partner, not a source of truth.
A bootstrap server may introduce nodes, but it may not define reality.
A high advertised block height proves nothing without valid certificates.
A fast connection earns convenience, not authority.
A diverse network reduces dependence, but cryptographic verification preserves truth.

Q1 succeeds at the node and networking layer when:

* nodes can find one another;
* exchange protocol objects;
* reject invalid traffic;
* recover from disconnection;
* resist partial isolation;
* and reach the same finalized ledger

without trusting any single peer, server, explorer, operator, AI model, or hardware report.