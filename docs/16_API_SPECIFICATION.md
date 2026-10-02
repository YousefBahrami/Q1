Q1 API Specification

16_API_SPECIFICATION.md

Project: Q1 Experimental Distributed Ledger
Protocol Version: 0.1
API Version: v1
Document Version: 0.1.0
Status: Draft for Engineering and Security Review
Classification: Experimental — Not for Production or Financial Use

⸻

1. Purpose

This document defines the application programming interfaces used by Q1 v0.1.

It specifies the interfaces for:

* node status;
* ledger and account queries;
* transaction submission;
* fee estimation;
* block queries;
* consensus status;
* validator and producer operations;
* peer and synchronization status;
* administrative controls;
* wallet integration;
* explorer and indexer access;
* telemetry;
* AI Observer;
* simulation;
* test orchestration;
* health checks;
* authentication;
* authorization;
* versioning;
* pagination;
* error handling;
* rate limiting;
* idempotency;
* structured data formats.

The API layer SHALL provide controlled access to Q1 components without permitting API clients to bypass deterministic protocol rules.

⸻

2. API Design Principles

Q1-API-001 — Protocol rules remain authoritative

An API request MUST NOT directly:

* edit a balance;
* create unauthorized units;
* finalize a block;
* alter a state root;
* create a validator attestation without the validator subsystem;
* bypass transaction signatures;
* bypass consensus;
* change finalized history.

APIs request actions.

The relevant protocol component determines whether those actions are valid.

⸻

Q1-API-002 — Separation of privilege

Q1 SHALL separate:

* public read APIs;
* transaction-submission APIs;
* role-specific local APIs;
* administrative APIs;
* telemetry APIs;
* research APIs.

A public client MUST NOT gain administrative access merely by reaching the node.

⸻

Q1-API-003 — Versioned interfaces

All externally visible APIs MUST be versioned.

Recommended path prefix:

/api/v1/

Observer and simulation services MAY use separate namespaces:

/observer/v1/
/simulation/v1/
/test/v1/

⸻

Q1-API-004 — Stable machine behavior

Responses and errors MUST use stable machine-readable fields.

Human-readable messages MAY improve over time without changing machine semantics.

⸻

Q1-API-005 — Exact monetary values

All monetary values in JSON MUST be encoded as decimal strings.

Example:

{
  "balance": "100000000"
}

They MUST NOT be encoded as floating-point numbers.

⸻

Q1-API-006 — Safe defaults

Administrative APIs SHOULD:

* bind to localhost by default;
* require authentication;
* use secure transport when remote;
* expose the least privilege necessary;
* reject unknown fields where security-sensitive.

⸻

3. API Domains

Q1 v0.1 SHALL define the following logical API domains:

Public Node API
Transaction API
Ledger Query API
Consensus Query API
Role API
Administrative API
Peer and Sync API
Telemetry API
Wallet Interface
Explorer API
AI Observer API
Simulation API
Test Orchestrator API
Health and Readiness API

These domains MAY be served by separate processes or listeners.

⸻

4. Transport

The initial APIs SHOULD use:

HTTP/1.1 or HTTP/2
JSON request and response bodies
UTF-8 encoding

Future versions MAY support:

* gRPC;
* WebSocket subscriptions;
* Server-Sent Events;
* binary protocols.

Consensus P2P traffic is defined separately and MUST NOT be replaced by public HTTP APIs.

⸻

5. Base URLs

Suggested local defaults:

Public Node API:
http://127.0.0.1:8170/api/v1
Administrative API:
http://127.0.0.1:8171/api/v1/admin
Telemetry:
http://127.0.0.1:8172
Explorer:
http://127.0.0.1:8180/api/v1
AI Observer:
http://127.0.0.1:8190/observer/v1
Simulator:
http://127.0.0.1:8200/simulation/v1
Test Orchestrator:
http://127.0.0.1:8210/test/v1

Ports are placeholders and MUST remain configurable.

⸻

6. Content Type

Requests with bodies MUST use:

Content-Type: application/json

Responses SHOULD use:

Content-Type: application/json; charset=utf-8

Metrics endpoints MAY use the appropriate Prometheus text format.

⸻

7. Common Response Envelope

Successful API responses SHOULD use:

{
  "api_version": "v1",
  "request_id": "req_...",
  "success": true,
  "data": {}
}

For simple health or metrics endpoints, a reduced response MAY be used.

⸻

8. Common Error Envelope

Errors MUST use:

{
  "api_version": "v1",
  "request_id": "req_...",
  "success": false,
  "error": {
    "code": "TX_INVALID_SIGNATURE",
    "message": "The transaction signature is invalid.",
    "category": "validation",
    "retryable": false,
    "details": {},
    "trace_id": "trace_..."
  }
}

Sensitive internal stack traces MUST NOT be returned to public clients.

⸻

9. Request Identifier

Clients MAY supply:

X-Request-ID

If absent, the server SHOULD generate one.

The request ID SHOULD appear in:

* response;
* logs;
* traces;
* error reports.

Request IDs are observational and have no consensus meaning.

⸻

10. Trace Identifier

The server MAY generate:

trace_id

for cross-component diagnostics.

Trace IDs MUST NOT contain secrets.

⸻

11. API Authentication Classes

Q1 SHALL recognize:

PUBLIC
LOCAL_TRUSTED
ROLE_AUTHENTICATED
ADMIN_AUTHENTICATED
SERVICE_AUTHENTICATED
TEST_ONLY

⸻

12. Public Access

Public endpoints MAY include:

* node status;
* latest finalized block;
* block lookup;
* transaction lookup;
* account query;
* fee estimation;
* transaction submission;
* health.

Public access MUST remain rate-limited.

⸻

13. Administrative Authentication

Administrative APIs SHOULD support one or more of:

* local socket access;
* bearer tokens;
* mutual TLS;
* operating-system credentials;
* signed administrative requests.

A static password in source code is prohibited.

⸻

14. Service Authentication

Service-to-service APIs, such as:

* node to indexer;
* telemetry collector to observer;
* simulator controller to workers;

SHOULD use dedicated service credentials.

They MUST NOT reuse validator, producer, wallet, or node identity private keys.

⸻

15. Authorization

Authentication proves the caller possesses accepted credentials.

Authorization determines permitted actions.

Recommended administrative roles:

READ_ONLY_ADMIN
NODE_OPERATOR
PEER_OPERATOR
SNAPSHOT_OPERATOR
ROLE_OPERATOR
SECURITY_OPERATOR
TEST_OPERATOR

One credential SHOULD NOT automatically receive every privilege.

⸻

16. Rate Limiting

Public APIs MUST enforce limits by:

* client address;
* authentication identity where available;
* endpoint;
* request cost;
* response size.

Rate-limited responses SHOULD use:

HTTP 429 Too Many Requests

and MAY include:

Retry-After

⸻

17. Request Size Limits

Configuration MUST define limits for:

* JSON body size;
* transaction body size;
* batch query size;
* pagination size;
* filter count;
* simulation scenario size;
* telemetry upload size.

Oversized requests MUST be rejected before expensive processing where possible.

⸻

18. Pagination

List endpoints SHOULD support cursor pagination.

Request example:

GET /api/v1/blocks?limit=50&cursor=...

Response:

{
  "api_version": "v1",
  "request_id": "req_123",
  "success": true,
  "data": {
    "items": [],
    "next_cursor": "cursor_...",
    "has_more": true
  }
}

Recommended default:

limit = 50

Recommended maximum:

limit = 200

⸻

19. Sorting

Endpoints supporting sorting MUST explicitly define allowed fields.

Clients MUST NOT be able to inject arbitrary database expressions.

Example:

sort=height_desc

rather than raw SQL-like values.

⸻

20. Time Representation

Operational timestamps SHALL use RFC 3339 UTC strings.

Example:

2026-07-23T12:00:00Z

Consensus height and round remain authoritative for protocol ordering.

⸻

21. Large Integer Representation

The following MUST use decimal strings in JSON:

* balances;
* amounts;
* fees;
* rewards;
* supply;
* nonces where client precision could be an issue;
* block heights where future size may exceed client integer safety;
* difficulty;
* committee weights.

⸻

22. Binary Data Representation

Hashes, keys, signatures, proofs, and canonical objects MAY use:

* lowercase hexadecimal;
* Base64URL;
* another explicitly specified encoding.

Each field MUST define its encoding.

Q1 SHOULD avoid mixing encodings without field-level clarity.

⸻

23. Public Node Status API

Endpoint

GET /api/v1/status

Purpose

Returns general node and chain status.

Response

{
  "api_version": "v1",
  "request_id": "req_001",
  "success": true,
  "data": {
    "node_id": "q1node_...",
    "node_version": "0.1.0",
    "protocol_version": "0.1",
    "chain_id": "q1-private-1",
    "genesis_hash": "hex...",
    "node_roles": [
      "FULL_NODE",
      "VALIDATOR"
    ],
    "runtime_state": "PARTICIPATING",
    "sync_state": "SYNCED",
    "safe_mode": false,
    "finalized_height": "1250",
    "finalized_block_hash": "hex...",
    "current_height": "1251",
    "current_round": "0",
    "peer_count": 12,
    "inbound_peer_count": 5,
    "outbound_peer_count": 7,
    "mempool_transaction_count": 83,
    "server_time_observational": "2026-07-23T12:00:00Z"
  }
}

The server time is observational only.

⸻

24. Health API

Endpoint

GET /api/v1/health

Response

{
  "status": "healthy",
  "components": {
    "storage": "healthy",
    "network": "healthy",
    "consensus": "healthy",
    "delay_engine": "healthy",
    "hdd_plugin": "disabled",
    "telemetry": "healthy"
  }
}

⸻

25. Readiness API

Endpoint

GET /api/v1/readiness

Purpose:

Determine whether the node is ready to serve normal requests.

Example:

{
  "ready": true,
  "sync_state": "SYNCED",
  "safe_mode": false,
  "reason": null
}

A node may be healthy but not ready, for example while synchronizing.

⸻

26. Latest Block API

Endpoint

GET /api/v1/blocks/latest

Response

{
  "api_version": "v1",
  "request_id": "req_002",
  "success": true,
  "data": {
    "block_hash": "hex...",
    "block_height": "1250",
    "parent_block_hash": "hex...",
    "round_number": "0",
    "producer_id": "participant_...",
    "transaction_count": 42,
    "block_size_bytes": 81920,
    "state_root": "hex...",
    "transaction_root": "hex...",
    "receipt_root": "hex...",
    "delay_engine_id": "q1-delay-sequential-hash-v1",
    "finalization_status": "FINALIZED",
    "finalization_certificate_hash": "hex..."
  }
}

⸻

27. Block Lookup API

Endpoints

GET /api/v1/blocks/by-height/{height}
GET /api/v1/blocks/by-hash/{block_hash}

Optional query parameters

include_transactions=true|false
include_receipts=true|false
include_attestations=true|false

Large optional expansions SHOULD be disabled by default.

⸻

28. Block List API

Endpoint

GET /api/v1/blocks

Supported filters MAY include:

from_height
to_height
producer_id
finalization_status
limit
cursor

⸻

29. Block Response Structure

{
  "block_header": {
    "protocol_version": "0.1",
    "chain_id": "q1-private-1",
    "block_height": "1250",
    "round_number": "0",
    "parent_block_hash": "hex...",
    "producer_id": "participant_...",
    "candidate_index": 0,
    "transaction_root": "hex...",
    "receipt_root": "hex...",
    "state_root": "hex...",
    "block_hash": "hex..."
  },
  "delay": {
    "engine_id": "q1-delay-sequential-hash-v1",
    "engine_version": "1",
    "difficulty": "1000000",
    "challenge_hash": "hex...",
    "output": "hex...",
    "proof_size_bytes": 128
  },
  "economics": {
    "total_fees_collected": "10000",
    "total_issuance": "1000000000",
    "total_burn": "0"
  },
  "finalization": {
    "status": "FINALIZED",
    "approving_weight": "4",
    "total_weight": "5",
    "threshold": "4",
    "certificate_hash": "hex..."
  }
}

⸻

30. Transaction Submission API

Endpoint

POST /api/v1/transactions

Request

{
  "signed_transaction": {
    "transaction_version": 1,
    "chain_id": "q1-private-1",
    "transaction_type": "TRANSFER",
    "sender_address": "q1p...",
    "sender_public_key": "hex...",
    "recipient_address": "q1p...",
    "amount": "1250000000",
    "fee_limit": "2000",
    "nonce": "7",
    "valid_from_height": "1251",
    "valid_until_height": "1351",
    "memo_hash": null,
    "signature_algorithm": "ALGORITHM_ID",
    "signature": "hex..."
  }
}

⸻

31. Transaction Submission Response

{
  "api_version": "v1",
  "request_id": "req_003",
  "success": true,
  "data": {
    "transaction_id": "hex...",
    "accepted": true,
    "mempool_status": "ACCEPTED_IN_MEMPOOL",
    "node_finalized_height": "1250",
    "already_known": false
  }
}

Acceptance into a mempool MUST NOT be represented as finalization.

⸻

32. Transaction Rejection Response

{
  "api_version": "v1",
  "request_id": "req_004",
  "success": false,
  "error": {
    "code": "TX_INVALID_SIGNATURE",
    "message": "The transaction signature is invalid.",
    "category": "transaction_validation",
    "retryable": false,
    "details": {
      "transaction_id": null
    }
  }
}

⸻

33. Idempotent Transaction Submission

Submitting the exact same signed transaction repeatedly SHOULD return the same transaction ID.

Possible response:

{
  "transaction_id": "hex...",
  "accepted": true,
  "already_known": true,
  "mempool_status": "ACCEPTED_IN_MEMPOOL"
}

⸻

34. Transaction Lookup API

Endpoint

GET /api/v1/transactions/{transaction_id}

Response

{
  "api_version": "v1",
  "request_id": "req_005",
  "success": true,
  "data": {
    "transaction_id": "hex...",
    "transaction_type": "TRANSFER",
    "sender_address": "q1p...",
    "recipient_address": "q1p...",
    "amount": "1250000000",
    "fee_limit": "2000",
    "charged_fee": "1500",
    "nonce": "7",
    "status": "FINALIZED",
    "included_block_hash": "hex...",
    "included_block_height": "1251",
    "transaction_index": 4,
    "finalization_status": "FINALIZED",
    "receipt": {
      "status": "SUCCESS",
      "sender_nonce_before": "7",
      "sender_nonce_after": "8"
    }
  }
}

⸻

35. Unknown Transaction Response

An unknown transaction SHOULD return:

HTTP 404

with:

{
  "error": {
    "code": "TX_NOT_FOUND",
    "retryable": true
  }
}

retryable may be true because another node may know the transaction.

⸻

36. Fee Estimation API

Endpoint

POST /api/v1/fees/estimate

Request

{
  "transaction_type": "TRANSFER",
  "sender_address": "q1p...",
  "recipient_address": "q1p...",
  "amount": "1250000000",
  "estimated_serialized_size_bytes": 320,
  "validity_window_blocks": "100"
}

⸻

37. Fee Estimation Response

{
  "api_version": "v1",
  "request_id": "req_006",
  "success": true,
  "data": {
    "estimate_height": "1250",
    "minimum_fee": "1000",
    "base_fee": "500",
    "size_fee": "320",
    "congestion_fee": "180",
    "discount": "0",
    "estimated_required_fee": "1000",
    "recommended_fee_limit": "1500",
    "estimate_valid_until_height": "1255",
    "economic_rule_version": "1"
  }
}

Fee estimation is not a guarantee of inclusion.

⸻

38. Account Query API

Endpoint

GET /api/v1/accounts/{address}

Response

{
  "api_version": "v1",
  "request_id": "req_007",
  "success": true,
  "data": {
    "address": "q1p...",
    "account_version": 1,
    "finalized_balance": "10000000000",
    "finalized_nonce": "8",
    "finalized_height": "1251",
    "finalized_block_hash": "hex...",
    "state_root": "hex..."
  }
}

⸻

39. Account Pending Summary API

Endpoint

GET /api/v1/accounts/{address}/pending

Response

{
  "address": "q1p...",
  "pending_outgoing_amount": "1250000000",
  "pending_incoming_amount_observed": "250000000",
  "maximum_pending_fees": "2000",
  "next_recommended_nonce": "9",
  "queued_nonce_gaps": []
}

Pending information is local node policy and not finalized consensus state.

⸻

40. Account Transaction History

Endpoint

GET /api/v1/accounts/{address}/transactions

This endpoint MAY be served by an indexer rather than a core node.

The response MUST identify whether it comes from:

CORE_NODE
INDEXER
EXPLORER

⸻

41. Supply API

Endpoint

GET /api/v1/economics/supply

Response

{
  "total_issued_supply": "1000000000000000",
  "total_burned_supply": "1000000",
  "circulating_supply": "999999999000000",
  "treasury_balance": "100000000000",
  "economic_rule_version": "1",
  "finalized_height": "1251"
}

⸻

42. Economics Summary API

Endpoint

GET /api/v1/economics/summary

Possible fields:

* fees in recent blocks;
* issuance;
* rewards by role;
* burn totals;
* treasury inflows;
* reward maturity.

This endpoint MUST not claim market price.

⸻

43. Consensus Status API

Endpoint

GET /api/v1/consensus/status

Response

{
  "chain_id": "q1-private-1",
  "target_height": "1252",
  "round_number": "0",
  "state": "PRODUCER_WINDOW_ACTIVE",
  "round_seed_hash": "hex...",
  "producer_candidates": [
    {
      "participant_id": "participant_A",
      "candidate_index": 0,
      "window_status": "ACTIVE"
    },
    {
      "participant_id": "participant_B",
      "candidate_index": 1,
      "window_status": "PENDING"
    }
  ],
  "committee_size": 5,
  "attestations_observed": 0,
  "threshold": "4",
  "safe_mode": false
}

Sensitive role information MAY be reduced on public deployments.

⸻

44. Consensus Round API

Endpoint

GET /api/v1/consensus/rounds/{height}/{round}

May return:

* candidate list;
* committee root;
* proposal hash;
* attestations;
* finalization status;
* timeout status;
* objective evidence.

⸻

45. Finalization Certificate API

Endpoint

GET /api/v1/finalization/{block_hash}

The response SHALL include sufficient data for independent verification.

⸻

46. Equivocation Evidence API

Endpoint

GET /api/v1/evidence/equivocations

Optional filters:

offender_id
offense_type
from_height
to_height

Objective signed evidence MAY be public.

Personal or operational metadata SHOULD be minimized.

⸻

47. Node Peer Summary API

Endpoint

GET /api/v1/network/summary

Response:

{
  "peer_count": 12,
  "inbound": 5,
  "outbound": 7,
  "static": 2,
  "bootstrap_connected": 1,
  "network_prefix_diversity": 6,
  "sync_peer_count": 3,
  "partition_suspected": false
}

⸻

48. Public Peer Detail Restrictions

Public APIs SHOULD NOT expose:

* full internal peer lists;
* raw IP addresses;
* ban details;
* private topology information.

Detailed peer data belongs in authenticated administration APIs.

⸻

49. Synchronization Status API

Endpoint

GET /api/v1/sync/status

Response

{
  "sync_state": "SYNCED",
  "local_finalized_height": "1251",
  "target_finalized_height": "1251",
  "headers_verified": "1251",
  "blocks_verified": "1251",
  "state_verified": true,
  "snapshot_mode": false,
  "active_sync_peers": 0
}

⸻

50. Validator Status API

Endpoint

GET /api/v1/validator/status

Authentication:

ROLE_AUTHENTICATED or LOCAL_TRUSTED

Response MAY include:

* active status;
* participant ID;
* committee membership;
* signing protection state;
* last attestation;
* missed duties;
* key availability without exposing keys.

⸻

51. Producer Status API

Endpoint

GET /api/v1/producer/status

Response MAY include:

* producer enabled;
* participant ID;
* current eligibility;
* candidate index;
* production window;
* delay state;
* last produced block;
* missed windows.

⸻

52. Producer Enable API

Endpoint

POST /api/v1/producer/enable

Authentication:

ROLE_OPERATOR

This endpoint enables local participation only.

It MUST NOT:

* alter the participant registry;
* force producer eligibility;
* bypass synchronization;
* bypass safe mode.

⸻

53. Producer Disable API

Endpoint

POST /api/v1/producer/disable

The node SHOULD safely cancel active local production work.

⸻

54. Validator Enable and Disable APIs

POST /api/v1/validator/enable
POST /api/v1/validator/disable

Disabling MUST prevent new signatures.

It MUST preserve signing history.

⸻

55. Administrative Status API

Endpoint

GET /api/v1/admin/status

Authentication required.

May return detailed component diagnostics.

⸻

56. Administrative Shutdown API

Endpoint

POST /api/v1/admin/shutdown

Request:

{
  "reason": "maintenance"
}

The node MUST perform graceful shutdown.

This endpoint MUST NOT be publicly exposed by default.

⸻

57. Configuration Reload API

Endpoint

POST /api/v1/admin/reload

Only non-consensus operational configuration MAY be reloaded without protocol activation.

Changes to consensus-critical values MUST be rejected.

⸻

58. Peer Administration API

Endpoints

GET  /api/v1/admin/peers
POST /api/v1/admin/peers/connect
POST /api/v1/admin/peers/disconnect
POST /api/v1/admin/peers/ban
POST /api/v1/admin/peers/unban

Local bans MUST NOT alter consensus state.

⸻

59. Snapshot Administration API

Endpoints

POST /api/v1/admin/snapshots
GET  /api/v1/admin/snapshots
DELETE /api/v1/admin/snapshots/{snapshot_id}

Snapshot creation MUST reference a finalized block.

Deletion MUST not remove finalized ledger data.

⸻

60. Safe Mode API

Endpoint

GET /api/v1/admin/safe-mode

Response:

{
  "safe_mode": true,
  "reason_code": "SEC_FINALITY_CONFLICT",
  "entered_at_observational": "2026-07-23T12:00:00Z",
  "evidence_refs": [
    "evidence_..."
  ],
  "production_disabled": true,
  "attestation_disabled": true,
  "finalization_disabled": true
}

⸻

61. Safe Mode Exit

Q1 v0.1 SHOULD NOT provide a simple unaudited endpoint such as:

POST /safe-mode/disable

Recovery MUST follow a documented procedure.

Any recovery API must require:

* strong authentication;
* explicit recovery package;
* evidence preservation;
* configuration versioning;
* audit logging.

⸻

62. Mempool API

Endpoints

GET /api/v1/mempool/summary
GET /api/v1/mempool/transactions

Detailed mempool access MAY require local or authenticated access to reduce information leakage and load.

⸻

63. Delay Engine Status API

Endpoint

GET /api/v1/delay/status

Response:

{
  "engine_id": "q1-delay-sequential-hash-v1",
  "engine_version": "1",
  "configured_difficulty": "1000000",
  "execution_state": "RUNNING",
  "active_challenge_hash": "hex...",
  "current_height": "1252",
  "round_number": "0",
  "candidate_index": 0,
  "generation_started_at_observational": "2026-07-23T12:00:00Z"
}

Public deployments MAY redact active operational details.

⸻

64. Delay Benchmark API

A local research endpoint MAY expose:

POST /api/v1/research/delay/benchmark

This endpoint MUST be disabled in normal public operation unless explicitly configured.

⸻

65. HDD Status API

Endpoint

GET /api/v1/hdd/status

Response MAY include:

{
  "mode": "TELEMETRY_ONLY",
  "plugin_id": "q1-hdd-lab-v1",
  "device_count": 1,
  "dataset_status": "READY",
  "active_workload": null,
  "last_result": "SUCCESS"
}

Public responses SHOULD avoid raw serial numbers and sensitive device metadata.

⸻

66. HDD Research APIs

Authenticated local endpoints MAY include:

GET  /api/v1/research/hdd/devices
POST /api/v1/research/hdd/datasets
POST /api/v1/research/hdd/workloads
POST /api/v1/research/hdd/benchmarks
POST /api/v1/research/hdd/cancel
DELETE /api/v1/research/hdd/datasets/{dataset_id}

Destructive or write-intensive operations MUST require explicit opt-in.

⸻

67. Telemetry Metrics Endpoint

Endpoint

GET /metrics

May expose Prometheus-compatible metrics.

Sensitive labels MUST be avoided.

⸻

68. Telemetry Summary API

Endpoint

GET /api/v1/telemetry/summary

Possible fields:

* CPU use;
* memory use;
* peer count;
* block propagation;
* finality duration;
* delay generation;
* verification;
* storage use;
* energy estimate.

Telemetry is observational.

⸻

69. Telemetry Event Ingestion

A separate collector MAY accept:

POST /telemetry/v1/events

Authentication:

SERVICE_AUTHENTICATED

Nodes SHOULD send batches rather than one request per event where practical.

⸻

70. Telemetry Batch Request

{
  "source_node_id": "q1node_...",
  "events": [
    {
      "event_id": "event_...",
      "event_type": "consensus.block_finalized",
      "event_version": "1",
      "chain_id": "q1-private-1",
      "payload": {}
    }
  ]
}

⸻

71. Explorer API

The explorer SHALL expose read-only indexed data.

Suggested endpoints:

GET /api/v1/explorer/blocks
GET /api/v1/explorer/blocks/{hash}
GET /api/v1/explorer/transactions/{id}
GET /api/v1/explorer/accounts/{address}
GET /api/v1/explorer/search?q=...
GET /api/v1/explorer/network
GET /api/v1/explorer/economics

Explorer responses MUST identify the indexed height.

⸻

72. Explorer Consistency Metadata

Each explorer response SHOULD include:

{
  "indexed_finalized_height": "1251",
  "indexed_finalized_block_hash": "hex...",
  "indexer_status": "SYNCED"
}

The explorer MUST not pretend stale data is current.

⸻

73. Indexer Ingestion Interface

The indexer MAY receive finalized blocks through:

* authenticated node API;
* local event stream;
* P2P finalized-block subscription;
* direct read-only database adapter.

The indexer MUST not write to consensus storage.

⸻

74. Event Subscription API

Future or optional real-time subscriptions MAY use WebSocket or Server-Sent Events.

Suggested topics:

blocks.finalized
transactions.status
consensus.round
network.status
alerts.security

Clients MUST be able to recover from missed events through normal query APIs.

⸻

75. AI Observer Status API

Endpoint

GET /observer/v1/status

Response:

{
  "observer_state": "READY",
  "service_version": "0.1.0",
  "active_models": [
    {
      "model_id": "q1-anomaly-baseline",
      "model_version": "1",
      "state": "APPROVED_FOR_OBSERVATION"
    }
  ],
  "last_ingested_event": "event_...",
  "queue_depth": 24,
  "consensus_write_access": false
}

⸻

76. AI Observer Alerts API

Endpoints

GET /observer/v1/alerts
GET /observer/v1/alerts/{alert_id}

Filters MAY include:

category
severity
status
from_time
to_time
node_id
participant_id

⸻

77. AI Alert Response

{
  "alert_id": "alert_...",
  "category": "PRODUCER_CONCENTRATION",
  "severity": "HIGH",
  "risk_score": 72,
  "evidence_class": "INFERRED",
  "affected_entities": [
    "operator_..."
  ],
  "explanation": "One operator appears to control a high share of recent production.",
  "objective_evidence": [],
  "supporting_metrics": {
    "top_1_operator_share": 0.46
  },
  "uncertainty": [
    "Operator ownership is inferred from test metadata."
  ],
  "recommended_actions": [
    "Review participant ownership assumptions.",
    "Run a Sybil concentration simulation."
  ],
  "model_id": "q1-concentration-detector",
  "model_version": "1"
}

Observer-local floating-point values MAY appear because they are non-consensus.

⸻

78. AI Alert Status Update

Endpoint

POST /observer/v1/alerts/{alert_id}/status

Authentication required.

Request:

{
  "status": "FALSE_POSITIVE",
  "review_note": "Known simulated concentration scenario."
}

This changes observer workflow only.

⸻

79. AI Analysis API

Endpoint

POST /observer/v1/analysis

May request analysis of:

* a time period;
* experiment;
* block range;
* attack run;
* participant concentration.

The observer MUST not accept commands to change consensus.

⸻

80. AI Model Registry API

GET /observer/v1/models
GET /observer/v1/models/{model_id}

Administrative model promotion MAY use a separate strongly authenticated interface.

⸻

81. Simulation Run API

Endpoint

POST /simulation/v1/runs

Request

{
  "scenario_id": "baseline-seven-node",
  "scenario_version": "1",
  "random_seed": "20260723",
  "duration": {
    "type": "BLOCKS",
    "value": "10000"
  },
  "overrides": {
    "committee_size": 5,
    "candidate_count": 3
  }
}

⸻

82. Simulation Run Response

{
  "run_id": "simrun_...",
  "status": "QUEUED",
  "scenario_hash": "hex...",
  "model_versions": {
    "consensus": "1",
    "network": "1",
    "economics": "1"
  }
}

⸻

83. Simulation Status API

GET /simulation/v1/runs/{run_id}

Response MAY include:

* status;
* progress;
* simulated time;
* events processed;
* critical invariant status;
* output artifact references.

⸻

84. Simulation Stop API

POST /simulation/v1/runs/{run_id}/stop

This affects only the simulation process.

⸻

85. Simulation Metrics API

GET /simulation/v1/runs/{run_id}/metrics

May return:

* consensus metrics;
* economics;
* energy;
* concentration;
* attack results.

⸻

86. Simulation Report API

GET /simulation/v1/runs/{run_id}/report

Reports MUST include assumptions and seed.

⸻

87. Parameter Sweep API

POST /simulation/v1/sweeps

Request example:

{
  "base_scenario_id": "committee-study",
  "parameters": {
    "committee_size": [3, 5, 7, 11],
    "byzantine_fraction": [0.0, 0.1, 0.2, 0.33]
  },
  "runs_per_combination": 100
}

Strict limits MUST prevent unbounded resource use.

⸻

88. Test Orchestrator Status API

GET /test/v1/status

May show:

* active topology;
* nodes;
* running tests;
* fault injections;
* artifact collection.

⸻

89. Test Network Creation API

POST /test/v1/networks

Request:

{
  "topology": "LOCAL_FIVE_NODE",
  "genesis_profile": "localnet",
  "deterministic_seed": "20260723",
  "enable_malicious_node": true
}

This API MUST be disabled outside controlled test infrastructure.

⸻

90. Fault Injection API

POST /test/v1/faults

Supported fault types MAY include:

NETWORK_DELAY
PACKET_LOSS
PARTITION
NODE_CRASH
CLOCK_SKEW
DISK_FULL
HDD_DISCONNECT
AI_OUTAGE
TELEMETRY_OUTAGE

The request MUST define target and duration.

⸻

91. Attack API

POST /test/v1/attacks

Attack profiles MAY include:

* double spend;
* validator equivocation;
* producer equivocation;
* invalid delay proof;
* transaction flood;
* eclipse simulation;
* HDD fraud.

Attack APIs MUST not be present in ordinary production binaries unless explicitly built for testing.

⸻

92. Test Artifact API

GET /test/v1/runs/{run_id}/artifacts

Artifacts MAY include:

* logs;
* configuration;
* genesis;
* traces;
* signed evidence;
* reports;
* metrics.

Secrets MUST be removed or protected.

⸻

93. API Error Categories

Recommended categories:

validation
authentication
authorization
not_found
conflict
rate_limit
resource_limit
protocol
consensus
storage
network
service_unavailable
internal

⸻

94. HTTP Status Mapping

Recommended mapping:

200 OK
201 Created
202 Accepted
204 No Content
400 Bad Request
401 Unauthorized
403 Forbidden
404 Not Found
409 Conflict
413 Payload Too Large
422 Unprocessable Entity
429 Too Many Requests
500 Internal Server Error
503 Service Unavailable

Consensus rejection is not always an HTTP server error.

For example, an invalid transaction SHOULD generally use 400 or 422.

⸻

95. Retry Semantics

Errors MUST identify whether retry may help.

Examples:

TX_INVALID_SIGNATURE → retryable: false
NODE_SYNCHRONIZING → retryable: true
RATE_LIMITED → retryable: true
WRONG_CHAIN → retryable: false

⸻

96. Idempotency Keys

State-changing non-consensus administrative operations MAY support:

Idempotency-Key

Transaction submission is naturally idempotent through transaction ID.

Simulation creation and snapshot creation SHOULD support idempotency keys to avoid duplicate jobs.

⸻

97. Batch APIs

Q1 MAY provide bounded batch endpoints.

Examples:

POST /api/v1/transactions/query
POST /api/v1/accounts/query

Batch size MUST be limited.

One invalid entry SHOULD have explicitly defined behavior:

* fail entire batch;
* or return per-item result.

The chosen behavior MUST be documented per endpoint.

⸻

98. API Schema Publication

Q1 SHOULD publish machine-readable schemas.

Recommended:

OpenAPI 3.1
JSON Schema

Canonical consensus serialization remains separate from JSON API schemas.

⸻

99. Client Generation

Machine-readable API definitions MAY generate:

* wallet clients;
* explorer clients;
* administration clients;
* test clients.

Generated clients MUST not hide security warnings or transaction-confirmation requirements.

⸻

100. Backward Compatibility

Within API v1, additive optional fields MAY be introduced when clients can safely ignore them.

Breaking changes require:

/api/v2/

Consensus protocol version and API version are separate.

⸻

101. Unknown Fields

Public read responses MAY include new fields.

Clients SHOULD ignore unknown non-critical response fields.

Security-sensitive requests SHOULD reject unknown fields where ambiguity could be dangerous.

⸻

102. Deprecation

Deprecated endpoints SHOULD return headers or fields indicating:

* deprecation status;
* replacement endpoint;
* removal version or date where known.

No consensus-critical behavior may depend indefinitely on a deprecated API.

⸻

103. API Audit Logging

Authenticated write actions MUST be logged.

Audit records SHOULD include:

request_id
authenticated_identity
authorization_role
endpoint
action
target
result
timestamp_observational
source_address

Secrets and full authorization tokens MUST NOT be logged.

⸻

104. Privacy

Public APIs SHOULD minimize unnecessary exposure of:

* raw peer addresses;
* device serials;
* operator metadata;
* wallet labels;
* administrative state;
* precise host information.

Finalized ledger data remains public under the v0.1 model.

⸻

105. Cross-Origin Access

Browser-facing APIs MUST define explicit Cross-Origin Resource Sharing policy.

Administrative APIs SHOULD disable cross-origin access by default.

Wildcard origin access MUST NOT be enabled casually.

⸻

106. CSRF Protection

Cookie-authenticated administrative interfaces require CSRF protection.

Bearer-token APIs SHOULD avoid browser ambient authority where practical.

⸻

107. Secure Transport

Remote authenticated APIs SHOULD use TLS.

Plain HTTP MAY be accepted for:

* localhost;
* isolated development networks;
* disposable test environments.

Wallets MUST warn before using insecure remote endpoints.

⸻

108. API Availability

Public node APIs SHOULD remain available when:

* AI Observer is offline;
* explorer is offline;
* telemetry is offline;
* HDD is disabled.

Some role-specific operations may be unavailable during:

* synchronization;
* safe mode;
* missing keys;
* protocol incompatibility.

⸻

109. API Safe-Mode Behavior

In safe mode:

Allowed MAY include:

* status;
* health;
* ledger read;
* evidence read;
* diagnostics;
* artifact export.

Prohibited MUST include:

* producer enable;
* validator signing;
* finalization control;
* unsafe recovery shortcuts.

⸻

110. API Security Tests

The test suite MUST include:

1. public endpoint access;
2. admin authentication failure;
3. admin authorization failure;
4. expired credentials;
5. token replay where relevant;
6. oversized body;
7. malformed JSON;
8. unknown critical field;
9. integer precision test;
10. transaction submission idempotency;
11. rate limiting;
12. pagination bounds;
13. injection attempts;
14. path traversal;
15. CORS misconfiguration;
16. CSRF tests where applicable;
17. insecure remote connection warning;
18. sensitive-data exposure;
19. stack-trace suppression;
20. safe-mode endpoint restrictions.

⸻

111. API Functional Tests

Required tests include:

* status;
* health;
* readiness;
* latest block;
* block by height;
* block by hash;
* transaction submission;
* transaction lookup;
* fee estimate;
* account state;
* supply;
* consensus status;
* sync status;
* producer status;
* validator status;
* peer summary;
* explorer indexing;
* observer alerts;
* simulation creation;
* test fault injection.

⸻

112. API Performance Tests

Measure:

* status latency;
* account query latency;
* transaction submission latency;
* block response size;
* pagination performance;
* concurrent clients;
* rate-limit behavior;
* telemetry ingestion throughput;
* simulation job-control latency.

API load MUST not starve consensus processing.

⸻

113. API Resource Priorities

Recommended priority:

Consensus internal traffic
Transaction validation
Finalized ledger queries
Wallet transaction submission
Synchronization
Administrative diagnostics
Explorer queries
Telemetry
Simulation and research

Simulation and heavy research APIs SHOULD run outside the node process where possible.

⸻

114. Minimum API Acceptance Criteria

The Q1 API subsystem is complete when:

1. node status is queryable;
2. health and readiness are distinct;
3. finalized blocks are queryable;
4. accounts are queryable;
5. signed transactions can be submitted;
6. invalid transactions return stable error codes;
7. transaction status can be tracked;
8. fee estimation is available;
9. monetary values use strings;
10. admin APIs require authentication;
11. public clients cannot edit balances;
12. producer and validator controls cannot bypass eligibility;
13. safe mode restricts sensitive actions;
14. explorer responses identify indexed height;
15. observer APIs remain read-only to consensus;
16. simulation APIs remain isolated from live state;
17. test APIs are disabled outside test builds;
18. request and trace IDs appear in logs;
19. rate and size limits are enforced;
20. API schemas are published.

⸻

115. Private Testnet Acceptance

Before private distributed testing, APIs MUST demonstrate:

* stable operation across multiple hosts;
* authenticated administration;
* transaction submission under load;
* correct chain and genesis reporting;
* multi-node wallet compatibility;
* no integer precision loss;
* no plaintext secret exposure;
* safe behavior during synchronization;
* safe behavior during partitions;
* observer outage independence;
* explorer lag visibility;
* bounded public-query load.

⸻

116. Known Limitations

Q1 API v0.1 does not yet guarantee:

* public internet hardening;
* production wallet compatibility;
* complete subscription reliability;
* mobile optimization;
* third-party developer stability;
* high-volume exchange integration;
* hardware-wallet integration;
* complete API backward compatibility;
* global privacy.

⸻

117. Open Decisions

The following remain unresolved:

1. exact HTTP framework;
2. HTTP/1.1 versus HTTP/2 default;
3. gRPC support;
4. WebSocket or SSE subscriptions;
5. exact authentication mechanism;
6. mutual TLS requirements;
7. administrative role model;
8. exact port assignments;
9. API gateway use;
10. public rate limits;
11. pagination cursor encoding;
12. hash and signature text encodings;
13. batch endpoint scope;
14. indexer ingestion mechanism;
15. explorer database;
16. simulation job backend;
17. telemetry transport;
18. API schema-generation tooling;
19. CORS policy for future wallet UI;
20. public testnet API hosting policy;
21. endpoint deprecation timeline;
22. audit-log retention;
23. API client SDK languages;
24. light-client proof endpoints;
25. data-availability endpoints.

All decisions MUST be recorded in OPEN_DECISIONS.md.

⸻

118. Codex Implementation Rules

Codex MUST:

1. separate public, role, administrative, observer, simulation, and test APIs;
2. version all routes;
3. encode monetary values as decimal strings;
4. use stable error codes;
5. publish OpenAPI or equivalent schemas;
6. require authentication for administrative actions;
7. bind admin APIs locally by default;
8. enforce request-size limits;
9. enforce rate limits;
10. generate request IDs;
11. suppress internal stack traces from public responses;
12. validate chain and genesis identity;
13. preserve transaction-submission idempotency;
14. keep explorer and observer write access away from consensus;
15. keep simulation isolated from live state;
16. disable test attack APIs in normal builds;
17. audit authenticated write actions;
18. distinguish health from readiness;
19. restrict sensitive operations during safe mode;
20. add API tests for every endpoint.

Codex MUST NOT:

* create a balance-edit endpoint;
* create an issuance override;
* expose private keys;
* allow an admin request to bypass consensus;
* use floating-point JSON values for money;
* expose raw device serials publicly by default;
* treat explorer responses as authoritative state;
* allow AI endpoints to penalize participants;
* expose destructive HDD actions publicly;
* permit unlimited batch size;
* allow simulation jobs to mutate a live chain;
* hide synchronization or stale-index status.

⸻

119. Final API Principle

Q1 APIs MUST preserve this principle:

An interface may expose information, request an action, or control a local service.
It may not replace cryptographic authorization, deterministic validation, or distributed finality.

The wallet asks the node to relay a transaction.
The API does not authorize the spending.

The producer operator enables local production.
The API does not grant eligibility.

The explorer displays a balance.
The API does not create that balance.

The AI Observer reports a risk.
The API does not convict or punish.

The simulator explores a future.
The API does not turn that future into reality.

Q1 succeeds at the API layer when every boundary clearly defines:

* who may call it;
* what data it accepts;
* what authority it has;
* what it can never do;
* how failure is reported;
* how abuse is limited;
* and how the deterministic protocol remains the final source of validity.