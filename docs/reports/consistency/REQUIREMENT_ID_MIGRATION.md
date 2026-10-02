Q1 Requirement Identifier Migration Table

Version: 0.1.0
Status: Applied migration record — requires human review
Date: 2026-07-24

This table records every pre-normalization identifier definition. Historical identifiers remain searchable here. The migration changed identifiers, not requirement meaning.

| Old Identifier | Source Document | Section | Old Meaning | New Identifier | Conflict Status | Notes |
|---|---|---|---|---|---|---|
| Q1-PR-001 | 02_SYSTEM_REQUIREMENTS.md | 4. Core Design Principles | Deterministic validity | Q1-SYS-001 | CONFLICT | Definition occurrence mapped independently |
| Q1-PR-002 | 02_SYSTEM_REQUIREMENTS.md | 4. Core Design Principles | Modular architecture | Q1-SYS-002 | CONFLICT | Definition occurrence mapped independently |
| Q1-PR-003 | 02_SYSTEM_REQUIREMENTS.md | 4. Core Design Principles | Public verifiability | Q1-SYS-003 | CONFLICT | Definition occurrence mapped independently |
| Q1-PR-004 | 02_SYSTEM_REQUIREMENTS.md | 4. Core Design Principles | Low entry barrier | Q1-SYS-004 | CONFLICT | Definition occurrence mapped independently |
| Q1-PR-005 | 02_SYSTEM_REQUIREMENTS.md | 4. Core Design Principles | Measurability | Q1-SYS-005 | CONFLICT | Definition occurrence mapped independently |
| Q1-PR-006 | 02_SYSTEM_REQUIREMENTS.md | 4. Core Design Principles | No financial promises | Q1-SYS-006 | CONFLICT | Definition occurrence mapped independently |
| Q1-MODE-001 | 02_SYSTEM_REQUIREMENTS.md | 8. Operating Modes | Single-node development mode | Q1-SYS-007 | CONFLICT | Definition occurrence mapped independently |
| Q1-MODE-002 | 02_SYSTEM_REQUIREMENTS.md | 8. Operating Modes | Local multi-node mode | Q1-SYS-008 | CONFLICT | Definition occurrence mapped independently |
| Q1-MODE-003 | 02_SYSTEM_REQUIREMENTS.md | 8. Operating Modes | Private distributed testnet | Q1-SYS-009 | CONFLICT | Definition occurrence mapped independently |
| Q1-MODE-004 | 02_SYSTEM_REQUIREMENTS.md | 8. Operating Modes | Public experimental testnet | Q1-SYS-010 | CONFLICT | Definition occurrence mapped independently |
| Q1-ID-001 | 02_SYSTEM_REQUIREMENTS.md | 9. Cryptographic Identity Requirements | Each wallet MUST have: | Q1-SYS-011 | CONFLICT | Definition occurrence mapped independently |
| Q1-ID-002 | 02_SYSTEM_REQUIREMENTS.md | 9. Cryptographic Identity Requirements | Private keys MUST NOT be transmitted to any node. | Q1-SYS-012 | CONFLICT | Definition occurrence mapped independently |
| Q1-ID-003 | 02_SYSTEM_REQUIREMENTS.md | 9. Cryptographic Identity Requirements | Private keys MUST NOT be stored in plaintext by default. | Q1-SYS-013 | CONFLICT | Definition occurrence mapped independently |
| Q1-ID-004 | 02_SYSTEM_REQUIREMENTS.md | 9. Cryptographic Identity Requirements | The first implementation SHOULD use a widely reviewed digital signature algorithm. | Q1-SYS-014 | CONFLICT | Definition occurrence mapped independently |
| Q1-ID-005 | 02_SYSTEM_REQUIREMENTS.md | 9. Cryptographic Identity Requirements | Every transaction MUST contain a valid digital signature. | Q1-SYS-015 | CONFLICT | Definition occurrence mapped independently |
| Q1-ID-006 | 02_SYSTEM_REQUIREMENTS.md | 9. Cryptographic Identity Requirements | Every candidate block MUST be signed by its producer. | Q1-SYS-016 | CONFLICT | Definition occurrence mapped independently |
| Q1-ID-007 | 02_SYSTEM_REQUIREMENTS.md | 9. Cryptographic Identity Requirements | Every validator attestation MUST be individually signed. | Q1-SYS-017 | CONFLICT | Definition occurrence mapped independently |
| Q1-ID-008 | 02_SYSTEM_REQUIREMENTS.md | 9. Cryptographic Identity Requirements | The protocol MUST reject malformed, invalid, or replayed signatures. | Q1-SYS-018 | CONFLICT | Definition occurrence mapped independently |
| Q1-LED-001 | 02_SYSTEM_REQUIREMENTS.md | 10. Ledger Model Requirements | Q1 v0.1 MUST use one clearly defined ledger model. | Q1-SYS-019 | CONFLICT | Definition occurrence mapped independently |
| Q1-LED-002 | 02_SYSTEM_REQUIREMENTS.md | 10. Ledger Model Requirements | The ledger MUST support a native unit. | Q1-SYS-020 | CONFLICT | Definition occurrence mapped independently |
| Q1-LED-003 | 02_SYSTEM_REQUIREMENTS.md | 10. Ledger Model Requirements | Balances MUST be represented as integers. | Q1-SYS-021 | CONFLICT | Definition occurrence mapped independently |
| Q1-LED-004 | 02_SYSTEM_REQUIREMENTS.md | 10. Ledger Model Requirements | The smallest unit MUST be fixed in the protocol configuration. | Q1-SYS-022 | CONFLICT | Definition occurrence mapped independently |
| Q1-LED-005 | 02_SYSTEM_REQUIREMENTS.md | 10. Ledger Model Requirements | State transitions MUST be deterministic. | Q1-SYS-023 | CONFLICT | Definition occurrence mapped independently |
| Q1-LED-006 | 02_SYSTEM_REQUIREMENTS.md | 10. Ledger Model Requirements | A transaction MUST NOT create a negative balance. | Q1-SYS-024 | CONFLICT | Definition occurrence mapped independently |
| Q1-LED-007 | 02_SYSTEM_REQUIREMENTS.md | 10. Ledger Model Requirements | The ledger MUST reject duplicate transaction identifiers. | Q1-SYS-025 | CONFLICT | Definition occurrence mapped independently |
| Q1-LED-008 | 02_SYSTEM_REQUIREMENTS.md | 10. Ledger Model Requirements | The ledger MUST support reconstructing current state from the genesis block. | Q1-SYS-026 | CONFLICT | Definition occurrence mapped independently |
| Q1-LED-009 | 02_SYSTEM_REQUIREMENTS.md | 10. Ledger Model Requirements | The ledger SHOULD support state snapshots for faster synchronization. | Q1-SYS-027 | CONFLICT | Definition occurrence mapped independently |
| Q1-GEN-001 | 02_SYSTEM_REQUIREMENTS.md | 11. Genesis Requirements | Every network instance MUST begin from a genesis configuration. | Q1-SYS-028 | CONFLICT | Definition occurrence mapped independently |
| Q1-GEN-002 | 02_SYSTEM_REQUIREMENTS.md | 11. Genesis Requirements | The genesis configuration MUST contain: | Q1-SYS-029 | CONFLICT | Definition occurrence mapped independently |
| Q1-GEN-003 | 02_SYSTEM_REQUIREMENTS.md | 11. Genesis Requirements | The genesis file MUST be human-readable. | Q1-SYS-030 | CONFLICT | Definition occurrence mapped independently |
| Q1-GEN-004 | 02_SYSTEM_REQUIREMENTS.md | 11. Genesis Requirements | All nodes on the same network MUST use the same genesis hash. | Q1-SYS-031 | CONFLICT | Definition occurrence mapped independently |
| Q1-TX-001 | 02_SYSTEM_REQUIREMENTS.md | 12. Transaction Requirements | A transaction MUST be uniquely identifiable by a deterministic hash. | Q1-SYS-032 | CONFLICT | Definition occurrence mapped independently |
| Q1-TX-002 | 02_SYSTEM_REQUIREMENTS.md | 12. Transaction Requirements | A transaction MUST be rejected if: | Q1-SYS-033 | CONFLICT | Definition occurrence mapped independently |
| Q1-TX-003 | 02_SYSTEM_REQUIREMENTS.md | 12. Transaction Requirements | The system MUST support at least the following transaction types: | Q1-SYS-034 | CONFLICT | Definition occurrence mapped independently |
| Q1-TX-004 | 02_SYSTEM_REQUIREMENTS.md | 3. genesis allocation. | Protocol reward transactions MUST NOT be manually created by normal users. | Q1-SYS-035 | CONFLICT | Definition occurrence mapped independently |
| Q1-TX-005 | 02_SYSTEM_REQUIREMENTS.md | 3. genesis allocation. | A user MUST be able to estimate the fee before signing. | Q1-SYS-036 | CONFLICT | Definition occurrence mapped independently |
| Q1-TX-006 | 02_SYSTEM_REQUIREMENTS.md | 3. genesis allocation. | The transaction format MUST be versioned. | Q1-SYS-037 | CONFLICT | Definition occurrence mapped independently |
| Q1-TX-007 | 02_SYSTEM_REQUIREMENTS.md | 3. genesis allocation. | The transaction serialization format MUST be canonical. | Q1-SYS-038 | CONFLICT | Definition occurrence mapped independently |
| Q1-MEM-001 | 02_SYSTEM_REQUIREMENTS.md | 13. Transaction Pool Requirements | Each full node MUST maintain a local transaction pool. | Q1-SYS-039 | CONFLICT | Definition occurrence mapped independently |
| Q1-MEM-002 | 02_SYSTEM_REQUIREMENTS.md | 13. Transaction Pool Requirements | Only transactions passing preliminary validation MAY enter the pool. | Q1-SYS-040 | CONFLICT | Definition occurrence mapped independently |
| Q1-MEM-003 | 02_SYSTEM_REQUIREMENTS.md | 13. Transaction Pool Requirements | The pool MUST prevent duplicate entries. | Q1-SYS-041 | CONFLICT | Definition occurrence mapped independently |
| Q1-MEM-004 | 02_SYSTEM_REQUIREMENTS.md | 13. Transaction Pool Requirements | The pool MUST have configurable limits for: | Q1-SYS-042 | CONFLICT | Definition occurrence mapped independently |
| Q1-MEM-005 | 02_SYSTEM_REQUIREMENTS.md | 13. Transaction Pool Requirements | The pool MUST include spam resistance. | Q1-SYS-043 | CONFLICT | Definition occurrence mapped independently |
| Q1-MEM-006 | 02_SYSTEM_REQUIREMENTS.md | 13. Transaction Pool Requirements | Transactions MUST be removed from the pool when: | Q1-SYS-044 | CONFLICT | Definition occurrence mapped independently |
| Q1-BLK-001 | 02_SYSTEM_REQUIREMENTS.md | 14. Block Requirements | The block hash MUST be deterministically derived from the canonical block header. | Q1-SYS-045 | CONFLICT | Definition occurrence mapped independently |
| Q1-BLK-002 | 02_SYSTEM_REQUIREMENTS.md | 14. Block Requirements | A block MUST reference exactly one parent block. | Q1-SYS-046 | CONFLICT | Definition occurrence mapped independently |
| Q1-BLK-003 | 02_SYSTEM_REQUIREMENTS.md | 14. Block Requirements | A block MUST be rejected if: | Q1-SYS-047 | CONFLICT | Definition occurrence mapped independently |
| Q1-BLK-004 | 02_SYSTEM_REQUIREMENTS.md | 14. Block Requirements | Maximum block size MUST be configurable in testnet mode. | Q1-SYS-048 | CONFLICT | Definition occurrence mapped independently |
| Q1-BLK-005 | 02_SYSTEM_REQUIREMENTS.md | 14. Block Requirements | Maximum transaction count per block MUST be configurable. | Q1-SYS-049 | CONFLICT | Definition occurrence mapped independently |
| Q1-BLK-006 | 02_SYSTEM_REQUIREMENTS.md | 14. Block Requirements | The protocol MUST define a target block interval. | Q1-SYS-050 | CONFLICT | Definition occurrence mapped independently |
| Q1-SEL-001 | 02_SYSTEM_REQUIREMENTS.md | 15. Block Producer Selection Requirements | Block producers MUST be selected through a deterministic and publicly verifiable procedure. | Q1-SYS-051 | CONFLICT | Definition occurrence mapped independently |
| Q1-SEL-002 | 02_SYSTEM_REQUIREMENTS.md | 15. Block Producer Selection Requirements | The selection result MUST derive from data unavailable before the previous block was finalized. | Q1-SYS-052 | CONFLICT | Definition occurrence mapped independently |
| Q1-SEL-003 | 02_SYSTEM_REQUIREMENTS.md | 15. Block Producer Selection Requirements | The selection process MUST prevent a producer from choosing its own favorable challenge. | Q1-SYS-053 | CONFLICT | Definition occurrence mapped independently |
| Q1-SEL-004 | 02_SYSTEM_REQUIREMENTS.md | 15. Block Producer Selection Requirements | The first implementation MAY use a weighted lottery. | Q1-SYS-054 | CONFLICT | Definition occurrence mapped independently |
| Q1-SEL-005 | 02_SYSTEM_REQUIREMENTS.md | 15. Block Producer Selection Requirements | The weight formula MUST be configurable. | Q1-SYS-055 | CONFLICT | Definition occurrence mapped independently |
| Q1-SEL-006 | 02_SYSTEM_REQUIREMENTS.md | 15. Block Producer Selection Requirements | Raw wealth MUST NOT provide unlimited linear control over selection probability. | Q1-SYS-056 | CONFLICT | Definition occurrence mapped independently |
| Q1-SEL-007 | 02_SYSTEM_REQUIREMENTS.md | 15. Block Producer Selection Requirements | The system MUST include a mechanism reducing repeated selection of the same producer. | Q1-SYS-057 | CONFLICT | Definition occurrence mapped independently |
| Q1-SEL-008 | 02_SYSTEM_REQUIREMENTS.md | 15. Block Producer Selection Requirements | The selected producer list MUST be independently computable by all full nodes. | Q1-SYS-058 | CONFLICT | Definition occurrence mapped independently |
| Q1-SEL-009 | 02_SYSTEM_REQUIREMENTS.md | 15. Block Producer Selection Requirements | More than one candidate SHOULD be selected per block round to provide fallback. | Q1-SYS-059 | CONFLICT | Definition occurrence mapped independently |
| Q1-SEL-010 | 02_SYSTEM_REQUIREMENTS.md | 15. Block Producer Selection Requirements | If the first candidate fails to publish before the deadline, the next candidate MUST become eligible according to deterministic rules. | Q1-SYS-060 | CONFLICT | Definition occurrence mapped independently |
| Q1-DLY-001 | 02_SYSTEM_REQUIREMENTS.md | 16. Delay Engine Requirements | The delay engine MUST receive a challenge derived from finalized network data. | Q1-SYS-061 | CONFLICT | Definition occurrence mapped independently |
| Q1-DLY-002 | 02_SYSTEM_REQUIREMENTS.md | 16. Delay Engine Requirements | The challenge MUST include at least: | Q1-SYS-062 | CONFLICT | Definition occurrence mapped independently |
| Q1-DLY-003 | 02_SYSTEM_REQUIREMENTS.md | 16. Delay Engine Requirements | The delay function MUST require sequential work. | Q1-SYS-063 | CONFLICT | Definition occurrence mapped independently |
| Q1-DLY-004 | 02_SYSTEM_REQUIREMENTS.md | 16. Delay Engine Requirements | The delay output MUST be difficult to compute substantially faster through ordinary parallelization. | Q1-SYS-064 | CONFLICT | Definition occurrence mapped independently |
| Q1-DLY-005 | 02_SYSTEM_REQUIREMENTS.md | 16. Delay Engine Requirements | The delay proof MUST be significantly cheaper to verify than to produce. | Q1-SYS-065 | CONFLICT | Definition occurrence mapped independently |
| Q1-DLY-006 | 02_SYSTEM_REQUIREMENTS.md | 16. Delay Engine Requirements | Verification MUST be deterministic. | Q1-SYS-066 | CONFLICT | Definition occurrence mapped independently |
| Q1-DLY-007 | 02_SYSTEM_REQUIREMENTS.md | 16. Delay Engine Requirements | Delay difficulty MUST be configurable. | Q1-SYS-067 | CONFLICT | Definition occurrence mapped independently |
| Q1-DLY-008 | 02_SYSTEM_REQUIREMENTS.md | 16. Delay Engine Requirements | The system MUST record: | Q1-SYS-068 | CONFLICT | Definition occurrence mapped independently |
| Q1-DLY-009 | 02_SYSTEM_REQUIREMENTS.md | 16. Delay Engine Requirements | The delay module MUST be replaceable. | Q1-SYS-069 | CONFLICT | Definition occurrence mapped independently |
| Q1-DLY-010 | 02_SYSTEM_REQUIREMENTS.md | 16. Delay Engine Requirements | A simplified delay implementation MUST be clearly labeled: | Q1-SYS-070 | CONFLICT | Definition occurrence mapped independently |
| Q1-HDD-001 | 02_SYSTEM_REQUIREMENTS.md | 17. HDD Laboratory Module Requirements | The HDD module MUST be implemented as an experimental plugin. | Q1-SYS-071 | CONFLICT | Definition occurrence mapped independently |
| Q1-HDD-002 | 02_SYSTEM_REQUIREMENTS.md | 17. HDD Laboratory Module Requirements | The core network MUST remain functional when the HDD module is disabled. | Q1-SYS-072 | CONFLICT | Definition occurrence mapped independently |
| Q1-HDD-003 | 02_SYSTEM_REQUIREMENTS.md | 17. HDD Laboratory Module Requirements | The HDD module MUST accept a fresh unpredictable challenge. | Q1-SYS-073 | CONFLICT | Definition occurrence mapped independently |
| Q1-HDD-004 | 02_SYSTEM_REQUIREMENTS.md | 17. HDD Laboratory Module Requirements | The module MAY perform combinations of: | Q1-SYS-074 | CONFLICT | Definition occurrence mapped independently |
| Q1-HDD-005 | 02_SYSTEM_REQUIREMENTS.md | 17. HDD Laboratory Module Requirements | The HDD response MUST be bound to: | Q1-SYS-075 | CONFLICT | Definition occurrence mapped independently |
| Q1-HDD-006 | 02_SYSTEM_REQUIREMENTS.md | 17. HDD Laboratory Module Requirements | Previously recorded HDD output MUST NOT be valid for a new challenge. | Q1-SYS-076 | CONFLICT | Definition occurrence mapped independently |
| Q1-HDD-007 | 02_SYSTEM_REQUIREMENTS.md | 17. HDD Laboratory Module Requirements | The module MUST record: | Q1-SYS-077 | CONFLICT | Definition occurrence mapped independently |
| Q1-HDD-008 | 02_SYSTEM_REQUIREMENTS.md | 17. HDD Laboratory Module Requirements | The protocol MUST NOT assume that HDD telemetry proves physical truth. | Q1-SYS-078 | CONFLICT | Definition occurrence mapped independently |
| Q1-HDD-009 | 02_SYSTEM_REQUIREMENTS.md | 17. HDD Laboratory Module Requirements | HDD results MUST initially be treated as one of the following: | Q1-SYS-079 | CONFLICT | Definition occurrence mapped independently |
| Q1-HDD-010 | 02_SYSTEM_REQUIREMENTS.md | 17. HDD Laboratory Module Requirements | The system MUST support simulated HDD participants so that physical and virtual results can be compared. | Q1-SYS-080 | CONFLICT | Definition occurrence mapped independently |
| Q1-HDD-011 | 02_SYSTEM_REQUIREMENTS.md | 17. HDD Laboratory Module Requirements | The test suite MUST attempt: | Q1-SYS-081 | CONFLICT | Definition occurrence mapped independently |
| Q1-VAL-001 | 02_SYSTEM_REQUIREMENTS.md | 18. Validator Committee Requirements | Each round MUST have a deterministically selected validator committee. | Q1-SYS-082 | CONFLICT | Definition occurrence mapped independently |
| Q1-VAL-002 | 02_SYSTEM_REQUIREMENTS.md | 18. Validator Committee Requirements | The committee size MUST be configurable. | Q1-SYS-083 | CONFLICT | Definition occurrence mapped independently |
| Q1-VAL-003 | 02_SYSTEM_REQUIREMENTS.md | 18. Validator Committee Requirements | A validator MUST independently verify the full candidate block before attesting. | Q1-SYS-084 | CONFLICT | Definition occurrence mapped independently |
| Q1-VAL-004 | 02_SYSTEM_REQUIREMENTS.md | 18. Validator Committee Requirements | A validator MUST NOT attest to two conflicting blocks at the same height and round. | Q1-SYS-085 | CONFLICT | Definition occurrence mapped independently |
| Q1-VAL-005 | 02_SYSTEM_REQUIREMENTS.md | 18. Validator Committee Requirements | Conflicting signed attestations MUST be detectable and stored as evidence. | Q1-SYS-086 | CONFLICT | Definition occurrence mapped independently |
| Q1-VAL-006 | 02_SYSTEM_REQUIREMENTS.md | 18. Validator Committee Requirements | Initial finalization SHOULD require at least two-thirds of committee weight. | Q1-SYS-087 | CONFLICT | Definition occurrence mapped independently |
| Q1-VAL-007 | 02_SYSTEM_REQUIREMENTS.md | 18. Validator Committee Requirements | Validator weight MUST NOT rely only on physical network proximity. | Q1-SYS-088 | CONFLICT | Definition occurrence mapped independently |
| Q1-VAL-008 | 02_SYSTEM_REQUIREMENTS.md | 18. Validator Committee Requirements | The protocol SHOULD attempt committee diversity across: | Q1-SYS-089 | CONFLICT | Definition occurrence mapped independently |
| Q1-CON-001 | 02_SYSTEM_REQUIREMENTS.md | 19. Consensus and Finalization Requirements | The protocol MUST define explicit states for a block: | Q1-SYS-090 | CONFLICT | Definition occurrence mapped independently |
| Q1-CON-002 | 02_SYSTEM_REQUIREMENTS.md | 19. Consensus and Finalization Requirements | A proposed block MUST NOT immediately become final. | Q1-SYS-091 | CONFLICT | Definition occurrence mapped independently |
| Q1-CON-003 | 02_SYSTEM_REQUIREMENTS.md | 19. Consensus and Finalization Requirements | Finalization MUST require a valid finalization certificate. | Q1-SYS-092 | CONFLICT | Definition occurrence mapped independently |
| Q1-CON-004 | 02_SYSTEM_REQUIREMENTS.md | 19. Consensus and Finalization Requirements | The finalization certificate MUST contain sufficient signed attestations to prove the required threshold. | Q1-SYS-093 | CONFLICT | Definition occurrence mapped independently |
| Q1-CON-005 | 02_SYSTEM_REQUIREMENTS.md | 19. Consensus and Finalization Requirements | Once a block is finalized, honest nodes MUST NOT reorganize the chain below that block under normal protocol operation. | Q1-SYS-094 | CONFLICT | Definition occurrence mapped independently |
| Q1-CON-006 | 02_SYSTEM_REQUIREMENTS.md | 19. Consensus and Finalization Requirements | Conflicting finalized blocks MUST be treated as a critical consensus failure. | Q1-SYS-095 | CONFLICT | Definition occurrence mapped independently |
| Q1-CON-007 | 02_SYSTEM_REQUIREMENTS.md | 19. Consensus and Finalization Requirements | The system MUST halt or enter safe mode if conflicting finalization certificates are detected. | Q1-SYS-096 | CONFLICT | Definition occurrence mapped independently |
| Q1-CON-008 | 02_SYSTEM_REQUIREMENTS.md | 19. Consensus and Finalization Requirements | Fork-choice rules MUST be deterministic. | Q1-SYS-097 | CONFLICT | Definition occurrence mapped independently |
| Q1-CON-009 | 02_SYSTEM_REQUIREMENTS.md | 19. Consensus and Finalization Requirements | The protocol MUST define behavior when: | Q1-SYS-098 | CONFLICT | Definition occurrence mapped independently |
| Q1-CON-010 | 02_SYSTEM_REQUIREMENTS.md | 19. Consensus and Finalization Requirements | Liveness MUST NOT override safety. | Q1-SYS-099 | CONFLICT | Definition occurrence mapped independently |
| Q1-TIM-001 | 02_SYSTEM_REQUIREMENTS.md | 20. Time Requirements | Local system clocks MUST NOT be treated as the sole source of truth. | Q1-SYS-100 | CONFLICT | Definition occurrence mapped independently |
| Q1-TIM-002 | 02_SYSTEM_REQUIREMENTS.md | 20. Time Requirements | Block timestamps MUST be checked against protocol-defined tolerance. | Q1-SYS-101 | CONFLICT | Definition occurrence mapped independently |
| Q1-TIM-003 | 02_SYSTEM_REQUIREMENTS.md | 20. Time Requirements | A node with a severely incorrect clock MUST be warned and MAY be prevented from producing blocks. | Q1-SYS-102 | CONFLICT | Definition occurrence mapped independently |
| Q1-TIM-004 | 02_SYSTEM_REQUIREMENTS.md | 20. Time Requirements | Round progression SHOULD depend primarily on: | Q1-SYS-103 | CONFLICT | Definition occurrence mapped independently |
| Q1-TIM-005 | 02_SYSTEM_REQUIREMENTS.md | 20. Time Requirements | The system MUST simulate clock manipulation attacks. | Q1-SYS-104 | CONFLICT | Definition occurrence mapped independently |
| Q1-FEE-001 | 02_SYSTEM_REQUIREMENTS.md | 21. Fee Requirements | Every normal transfer MUST pay a non-negative fee. | Q1-SYS-105 | CONFLICT | Definition occurrence mapped independently |
| Q1-FEE-002 | 02_SYSTEM_REQUIREMENTS.md | 21. Fee Requirements | Anonymous transactions MUST NOT receive a net negative fee. | Q1-SYS-106 | CONFLICT | Definition occurrence mapped independently |
| Q1-FEE-003 | 02_SYSTEM_REQUIREMENTS.md | 21. Fee Requirements | The initial fee model MUST be understandable to users. | Q1-SYS-107 | CONFLICT | Definition occurrence mapped independently |
| Q1-FEE-004 | 02_SYSTEM_REQUIREMENTS.md | 21. Fee Requirements | The fee estimator MUST display the expected fee before signing. | Q1-SYS-108 | CONFLICT | Definition occurrence mapped independently |
| Q1-FEE-005 | 02_SYSTEM_REQUIREMENTS.md | 21. Fee Requirements | The fee algorithm MUST be deterministic from public network data. | Q1-SYS-109 | CONFLICT | Definition occurrence mapped independently |
| Q1-FEE-006 | 02_SYSTEM_REQUIREMENTS.md | 21. Fee Requirements | The fee model MUST include anti-spam protection. | Q1-SYS-110 | CONFLICT | Definition occurrence mapped independently |
| Q1-FEE-007 | 02_SYSTEM_REQUIREMENTS.md | 21. Fee Requirements | Fee parameters MUST be configurable in testnet mode. | Q1-SYS-111 | CONFLICT | Definition occurrence mapped independently |
| Q1-FEE-008 | 02_SYSTEM_REQUIREMENTS.md | 21. Fee Requirements | The system MUST test behavior under: | Q1-SYS-112 | CONFLICT | Definition occurrence mapped independently |
| Q1-RWD-001 | 02_SYSTEM_REQUIREMENTS.md | 22. Reward Requirements | Block rewards MUST be generated only according to protocol rules. | Q1-SYS-113 | CONFLICT | Definition occurrence mapped independently |
| Q1-RWD-002 | 02_SYSTEM_REQUIREMENTS.md | 22. Reward Requirements | Reward allocation MUST be deterministic and independently verifiable. | Q1-SYS-114 | CONFLICT | Definition occurrence mapped independently |
| Q1-RWD-003 | 02_SYSTEM_REQUIREMENTS.md | 22. Reward Requirements | The initial reward may be divided among: | Q1-SYS-115 | CONFLICT | Definition occurrence mapped independently |
| Q1-RWD-004 | 02_SYSTEM_REQUIREMENTS.md | 22. Reward Requirements | The exact percentages MUST remain configurable during simulation. | Q1-SYS-116 | CONFLICT | Definition occurrence mapped independently |
| Q1-RWD-005 | 02_SYSTEM_REQUIREMENTS.md | 22. Reward Requirements | No actor MUST receive a reward for invalid, late, duplicate, or conflicting work. | Q1-SYS-117 | CONFLICT | Definition occurrence mapped independently |
| Q1-RWD-006 | 02_SYSTEM_REQUIREMENTS.md | 22. Reward Requirements | A producer whose block is rejected MUST NOT receive the producer reward. | Q1-SYS-118 | CONFLICT | Definition occurrence mapped independently |
| Q1-RWD-007 | 02_SYSTEM_REQUIREMENTS.md | 22. Reward Requirements | A validator signing conflicting blocks MUST be penalizable. | Q1-SYS-119 | CONFLICT | Definition occurrence mapped independently |
| Q1-RWD-008 | 02_SYSTEM_REQUIREMENTS.md | 22. Reward Requirements | The prototype MAY implement virtual penalties before implementing locked collateral. | Q1-SYS-120 | CONFLICT | Definition occurrence mapped independently |
| Q1-RWD-009 | 02_SYSTEM_REQUIREMENTS.md | 22. Reward Requirements | Reward issuance MUST be included in supply accounting. | Q1-SYS-121 | CONFLICT | Definition occurrence mapped independently |
| Q1-SUP-001 | 02_SYSTEM_REQUIREMENTS.md | 23. Supply Requirements | Q1 v0.1 MUST define a test supply model. | Q1-SYS-122 | CONFLICT | Definition occurrence mapped independently |
| Q1-SUP-002 | 02_SYSTEM_REQUIREMENTS.md | 23. Supply Requirements | The test supply model MUST record: | Q1-SYS-123 | CONFLICT | Definition occurrence mapped independently |
| Q1-SUP-003 | 02_SYSTEM_REQUIREMENTS.md | 23. Supply Requirements | The supply model MUST use integer arithmetic. | Q1-SYS-124 | CONFLICT | Definition occurrence mapped independently |
| Q1-SUP-004 | 02_SYSTEM_REQUIREMENTS.md | 23. Supply Requirements | No final monetary policy is approved at this stage. | Q1-SYS-125 | CONFLICT | Definition occurrence mapped independently |
| Q1-SUP-005 | 02_SYSTEM_REQUIREMENTS.md | 23. Supply Requirements | Alternative supply models MUST be testable through configuration. | Q1-SYS-126 | CONFLICT | Definition occurrence mapped independently |
| Q1-NET-001 | 02_SYSTEM_REQUIREMENTS.md | 24. Networking Requirements | Nodes MUST communicate over authenticated or cryptographically signed messages where appropriate. | Q1-SYS-127 | CONFLICT | Definition occurrence mapped independently |
| Q1-NET-002 | 02_SYSTEM_REQUIREMENTS.md | 24. Networking Requirements | The network MUST support peer discovery. | Q1-SYS-128 | CONFLICT | Definition occurrence mapped independently |
| Q1-NET-003 | 02_SYSTEM_REQUIREMENTS.md | 24. Networking Requirements | The network MUST support manually configured peers for private testnets. | Q1-SYS-129 | CONFLICT | Definition occurrence mapped independently |
| Q1-NET-004 | 02_SYSTEM_REQUIREMENTS.md | 24. Networking Requirements | A node MUST maintain configurable limits for: | Q1-SYS-130 | CONFLICT | Definition occurrence mapped independently |
| Q1-NET-005 | 02_SYSTEM_REQUIREMENTS.md | 24. Networking Requirements | The network MUST detect malformed messages. | Q1-SYS-131 | CONFLICT | Definition occurrence mapped independently |
| Q1-NET-006 | 02_SYSTEM_REQUIREMENTS.md | 24. Networking Requirements | Nodes MUST be able to temporarily disconnect abusive peers. | Q1-SYS-132 | CONFLICT | Definition occurrence mapped independently |
| Q1-NET-007 | 02_SYSTEM_REQUIREMENTS.md | 24. Networking Requirements | Peer penalties MUST NOT directly change ledger state. | Q1-SYS-133 | CONFLICT | Definition occurrence mapped independently |
| Q1-NET-008 | 02_SYSTEM_REQUIREMENTS.md | 24. Networking Requirements | The network MUST support chain synchronization from genesis. | Q1-SYS-134 | CONFLICT | Definition occurrence mapped independently |
| Q1-NET-009 | 02_SYSTEM_REQUIREMENTS.md | 24. Networking Requirements | The network SHOULD support snapshot-assisted synchronization. | Q1-SYS-135 | CONFLICT | Definition occurrence mapped independently |
| Q1-NET-010 | 02_SYSTEM_REQUIREMENTS.md | 24. Networking Requirements | The system MUST test: | Q1-SYS-136 | CONFLICT | Definition occurrence mapped independently |
| Q1-NOD-001 | 02_SYSTEM_REQUIREMENTS.md | 25. Node Requirements | A node MUST have a unique node key separate from wallet spending keys. | Q1-SYS-137 | CONFLICT | Definition occurrence mapped independently |
| Q1-NOD-002 | 02_SYSTEM_REQUIREMENTS.md | 25. Node Requirements | A node MUST store its data in a configurable directory. | Q1-SYS-138 | CONFLICT | Definition occurrence mapped independently |
| Q1-NOD-003 | 02_SYSTEM_REQUIREMENTS.md | 25. Node Requirements | A node MUST support graceful shutdown. | Q1-SYS-139 | CONFLICT | Definition occurrence mapped independently |
| Q1-NOD-004 | 02_SYSTEM_REQUIREMENTS.md | 25. Node Requirements | A node MUST recover from ordinary restart without corrupting the ledger. | Q1-SYS-140 | CONFLICT | Definition occurrence mapped independently |
| Q1-NOD-005 | 02_SYSTEM_REQUIREMENTS.md | 25. Node Requirements | A node MUST detect corrupted local data. | Q1-SYS-141 | CONFLICT | Definition occurrence mapped independently |
| Q1-NOD-006 | 02_SYSTEM_REQUIREMENTS.md | 25. Node Requirements | A node MUST expose health information. | Q1-SYS-142 | CONFLICT | Definition occurrence mapped independently |
| Q1-NOD-007 | 02_SYSTEM_REQUIREMENTS.md | 25. Node Requirements | A node MUST provide structured logs. | Q1-SYS-143 | CONFLICT | Definition occurrence mapped independently |
| Q1-NOD-008 | 02_SYSTEM_REQUIREMENTS.md | 25. Node Requirements | A node MUST support at least: | Q1-SYS-144 | CONFLICT | Definition occurrence mapped independently |
| Q1-WAL-001 | 02_SYSTEM_REQUIREMENTS.md | 26. Wallet Requirements | The first wallet MUST be a command-line wallet. | Q1-SYS-145 | CONFLICT | Definition occurrence mapped independently |
| Q1-WAL-002 | 02_SYSTEM_REQUIREMENTS.md | 26. Wallet Requirements | The wallet MUST support: | Q1-SYS-146 | CONFLICT | Definition occurrence mapped independently |
| Q1-WAL-003 | 02_SYSTEM_REQUIREMENTS.md | 26. Wallet Requirements | The wallet MUST display the network name and chain identifier. | Q1-SYS-147 | CONFLICT | Definition occurrence mapped independently |
| Q1-WAL-004 | 02_SYSTEM_REQUIREMENTS.md | 26. Wallet Requirements | The wallet MUST warn users that Q1 v0.1 units have no guaranteed financial value. | Q1-SYS-148 | CONFLICT | Definition occurrence mapped independently |
| Q1-WAL-005 | 02_SYSTEM_REQUIREMENTS.md | 26. Wallet Requirements | The wallet MUST NOT automatically upload private keys. | Q1-SYS-149 | CONFLICT | Definition occurrence mapped independently |
| Q1-WAL-006 | 02_SYSTEM_REQUIREMENTS.md | 26. Wallet Requirements | The wallet SHOULD support encrypted local key storage. | Q1-SYS-150 | CONFLICT | Definition occurrence mapped independently |
| Q1-WAL-007 | 02_SYSTEM_REQUIREMENTS.md | 26. Wallet Requirements | The wallet SHOULD support offline transaction signing. | Q1-SYS-151 | CONFLICT | Definition occurrence mapped independently |
| Q1-WAL-008 | 02_SYSTEM_REQUIREMENTS.md | 26. Wallet Requirements | The wallet MUST clearly distinguish: | Q1-SYS-152 | CONFLICT | Definition occurrence mapped independently |
| Q1-EXP-001 | 02_SYSTEM_REQUIREMENTS.md | 27. Explorer Requirements | The explorer MUST display: | Q1-SYS-153 | CONFLICT | Definition occurrence mapped independently |
| Q1-EXP-002 | 02_SYSTEM_REQUIREMENTS.md | 27. Explorer Requirements | The explorer MUST allow lookup by: | Q1-SYS-154 | CONFLICT | Definition occurrence mapped independently |
| Q1-EXP-003 | 02_SYSTEM_REQUIREMENTS.md | 27. Explorer Requirements | The explorer MUST display a clear experimental-network warning. | Q1-SYS-155 | CONFLICT | Definition occurrence mapped independently |
| Q1-EXP-004 | 02_SYSTEM_REQUIREMENTS.md | 27. Explorer Requirements | The first explorer MAY be local and minimal. | Q1-SYS-156 | CONFLICT | Definition occurrence mapped independently |
| Q1-AI-001 | 02_SYSTEM_REQUIREMENTS.md | 28. AI Observer Requirements | The AI observer MUST operate outside the deterministic consensus path. | Q1-SYS-157 | CONFLICT | Definition occurrence mapped independently |
| Q1-AI-002 | 02_SYSTEM_REQUIREMENTS.md | 28. AI Observer Requirements | Consensus MUST remain functional if the AI observer is offline. | Q1-SYS-158 | CONFLICT | Definition occurrence mapped independently |
| Q1-AI-003 | 02_SYSTEM_REQUIREMENTS.md | 28. AI Observer Requirements | The AI observer MAY analyze: | Q1-SYS-159 | CONFLICT | Definition occurrence mapped independently |
| Q1-AI-004 | 02_SYSTEM_REQUIREMENTS.md | 28. AI Observer Requirements | Every AI-generated alert MUST include: | Q1-SYS-160 | CONFLICT | Definition occurrence mapped independently |
| Q1-AI-005 | 02_SYSTEM_REQUIREMENTS.md | 28. AI Observer Requirements | AI findings MUST be labeled as: | Q1-SYS-161 | CONFLICT | Definition occurrence mapped independently |
| Q1-AI-006 | 02_SYSTEM_REQUIREMENTS.md | 28. AI Observer Requirements | An AI alert MUST NOT automatically confiscate funds, reject blocks, or alter consensus. | Q1-SYS-162 | CONFLICT | Definition occurrence mapped independently |
| Q1-AI-007 | 02_SYSTEM_REQUIREMENTS.md | 28. AI Observer Requirements | The AI observer MUST support false-positive and false-negative evaluation. | Q1-SYS-163 | CONFLICT | Definition occurrence mapped independently |
| Q1-AI-008 | 02_SYSTEM_REQUIREMENTS.md | 28. AI Observer Requirements | AI model changes MUST be versioned. | Q1-SYS-164 | CONFLICT | Definition occurrence mapped independently |
| Q1-TEL-001 | 02_SYSTEM_REQUIREMENTS.md | 29. Telemetry Requirements | Telemetry MUST be enabled by default in private experimental networks. | Q1-SYS-165 | CONFLICT | Definition occurrence mapped independently |
| Q1-TEL-002 | 02_SYSTEM_REQUIREMENTS.md | 29. Telemetry Requirements | Telemetry MUST be configurable. | Q1-SYS-166 | CONFLICT | Definition occurrence mapped independently |
| Q1-TEL-003 | 02_SYSTEM_REQUIREMENTS.md | 29. Telemetry Requirements | Telemetry MUST NOT collect wallet private keys or seed phrases. | Q1-SYS-167 | CONFLICT | Definition occurrence mapped independently |
| Q1-TEL-004 | 02_SYSTEM_REQUIREMENTS.md | 29. Telemetry Requirements | Telemetry MUST record at least: | Q1-SYS-168 | CONFLICT | Definition occurrence mapped independently |
| Q1-TEL-005 | 02_SYSTEM_REQUIREMENTS.md | 29. Telemetry Requirements | Telemetry data SHOULD use structured formats. | Q1-SYS-169 | CONFLICT | Definition occurrence mapped independently |
| Q1-TEL-006 | 02_SYSTEM_REQUIREMENTS.md | 29. Telemetry Requirements | The test environment SHOULD support visualization dashboards. | Q1-SYS-170 | CONFLICT | Definition occurrence mapped independently |
| Q1-ENE-001 | 02_SYSTEM_REQUIREMENTS.md | 30. Energy Measurement Requirements | Q1 MUST NOT claim energy superiority without measurement. | Q1-SYS-171 | CONFLICT | Definition occurrence mapped independently |
| Q1-ENE-002 | 02_SYSTEM_REQUIREMENTS.md | 30. Energy Measurement Requirements | The prototype MUST estimate or measure energy usage for: | Q1-SYS-172 | CONFLICT | Definition occurrence mapped independently |
| Q1-ENE-003 | 02_SYSTEM_REQUIREMENTS.md | 30. Energy Measurement Requirements | Energy reporting MUST include the measurement method. | Q1-SYS-173 | CONFLICT | Definition occurrence mapped independently |
| Q1-ENE-004 | 02_SYSTEM_REQUIREMENTS.md | 30. Energy Measurement Requirements | The system SHOULD calculate: | Q1-SYS-174 | CONFLICT | Definition occurrence mapped independently |
| Q1-ENE-005 | 02_SYSTEM_REQUIREMENTS.md | 30. Energy Measurement Requirements | The system MUST compare multiple configurations, including: | Q1-SYS-175 | CONFLICT | Definition occurrence mapped independently |
| Q1-SEC-001 | 02_SYSTEM_REQUIREMENTS.md | 31. Security Requirements | The system MUST reject unauthorized balance creation. | Q1-SYS-176 | CONFLICT | Definition occurrence mapped independently |
| Q1-SEC-002 | 02_SYSTEM_REQUIREMENTS.md | 31. Security Requirements | The system MUST reject double spending. | Q1-SYS-177 | CONFLICT | Definition occurrence mapped independently |
| Q1-SEC-003 | 02_SYSTEM_REQUIREMENTS.md | 31. Security Requirements | The system MUST prevent transaction replay across different chain identifiers. | Q1-SYS-178 | CONFLICT | Definition occurrence mapped independently |
| Q1-SEC-004 | 02_SYSTEM_REQUIREMENTS.md | 31. Security Requirements | The system MUST detect conflicting validator signatures. | Q1-SYS-179 | CONFLICT | Definition occurrence mapped independently |
| Q1-SEC-005 | 02_SYSTEM_REQUIREMENTS.md | 31. Security Requirements | The system MUST test malicious producers. | Q1-SYS-180 | CONFLICT | Definition occurrence mapped independently |
| Q1-SEC-006 | 02_SYSTEM_REQUIREMENTS.md | 31. Security Requirements | The system MUST test malicious validators. | Q1-SYS-181 | CONFLICT | Definition occurrence mapped independently |
| Q1-SEC-007 | 02_SYSTEM_REQUIREMENTS.md | 31. Security Requirements | The system MUST test Sybil-node creation. | Q1-SYS-182 | CONFLICT | Definition occurrence mapped independently |
| Q1-SEC-008 | 02_SYSTEM_REQUIREMENTS.md | 31. Security Requirements | The system MUST test network partitions. | Q1-SYS-183 | CONFLICT | Definition occurrence mapped independently |
| Q1-SEC-009 | 02_SYSTEM_REQUIREMENTS.md | 31. Security Requirements | The system MUST test manipulated local clocks. | Q1-SYS-184 | CONFLICT | Definition occurrence mapped independently |
| Q1-SEC-010 | 02_SYSTEM_REQUIREMENTS.md | 31. Security Requirements | The system MUST test forged HDD telemetry. | Q1-SYS-185 | CONFLICT | Definition occurrence mapped independently |
| Q1-SEC-011 | 02_SYSTEM_REQUIREMENTS.md | 31. Security Requirements | The system MUST test invalid delay proofs. | Q1-SYS-186 | CONFLICT | Definition occurrence mapped independently |
| Q1-SEC-012 | 02_SYSTEM_REQUIREMENTS.md | 31. Security Requirements | The system MUST test long-range and alternate-chain attempts where applicable. | Q1-SYS-187 | CONFLICT | Definition occurrence mapped independently |
| Q1-SEC-013 | 02_SYSTEM_REQUIREMENTS.md | 31. Security Requirements | Critical cryptographic failures MUST cause safe rejection, not silent acceptance. | Q1-SYS-188 | CONFLICT | Definition occurrence mapped independently |
| Q1-SEC-014 | 02_SYSTEM_REQUIREMENTS.md | 31. Security Requirements | Private keys, passwords, and secrets MUST NOT appear in normal logs. | Q1-SYS-189 | CONFLICT | Definition occurrence mapped independently |
| Q1-PRI-001 | 02_SYSTEM_REQUIREMENTS.md | 32. Privacy Requirements | Q1 v0.1 is not a privacy blockchain. | Q1-SYS-190 | CONFLICT | Definition occurrence mapped independently |
| Q1-PRI-002 | 02_SYSTEM_REQUIREMENTS.md | 32. Privacy Requirements | Users MUST be informed that addresses, balances, and transactions may be publicly visible. | Q1-SYS-191 | CONFLICT | Definition occurrence mapped independently |
| Q1-PRI-003 | 02_SYSTEM_REQUIREMENTS.md | 32. Privacy Requirements | Telemetry MUST avoid unnecessary personal data. | Q1-SYS-192 | CONFLICT | Definition occurrence mapped independently |
| Q1-PRI-004 | 02_SYSTEM_REQUIREMENTS.md | 32. Privacy Requirements | IP addresses and geographic data SHOULD be anonymized or minimized in exported research reports. | Q1-SYS-193 | CONFLICT | Definition occurrence mapped independently |
| Q1-PER-001 | 02_SYSTEM_REQUIREMENTS.md | 33. Performance Requirements | A normal transaction SHOULD be validated locally within: | Q1-SYS-194 | CONFLICT | Definition occurrence mapped independently |
| Q1-PER-002 | 02_SYSTEM_REQUIREMENTS.md | 33. Performance Requirements | A delay proof SHOULD be verified significantly faster than it is generated. | Q1-SYS-195 | CONFLICT | Definition occurrence mapped independently |
| Q1-PER-003 | 02_SYSTEM_REQUIREMENTS.md | 33. Performance Requirements | A finalized block SHOULD propagate to most reachable testnet nodes within: | Q1-SYS-196 | CONFLICT | Definition occurrence mapped independently |
| Q1-PER-004 | 02_SYSTEM_REQUIREMENTS.md | 33. Performance Requirements | The network SHOULD maintain operation when up to one-third of validator weight is unavailable or malicious, subject to the final consensus design. | Q1-SYS-197 | CONFLICT | Definition occurrence mapped independently |
| Q1-PER-005 | 02_SYSTEM_REQUIREMENTS.md | 33. Performance Requirements | The node MUST remain stable under at least: | Q1-SYS-198 | CONFLICT | Definition occurrence mapped independently |
| Q1-REL-001 | 02_SYSTEM_REQUIREMENTS.md | 34. Reliability Requirements | A node MUST restart without losing finalized ledger data. | Q1-SYS-199 | CONFLICT | Definition occurrence mapped independently |
| Q1-REL-002 | 02_SYSTEM_REQUIREMENTS.md | 34. Reliability Requirements | Temporary network disconnection MUST NOT corrupt local state. | Q1-SYS-200 | CONFLICT | Definition occurrence mapped independently |
| Q1-REL-003 | 02_SYSTEM_REQUIREMENTS.md | 34. Reliability Requirements | A recovering node MUST be able to synchronize with honest peers. | Q1-SYS-201 | CONFLICT | Definition occurrence mapped independently |
| Q1-REL-004 | 02_SYSTEM_REQUIREMENTS.md | 34. Reliability Requirements | Malformed peer messages MUST NOT crash the node. | Q1-SYS-202 | CONFLICT | Definition occurrence mapped independently |
| Q1-REL-005 | 02_SYSTEM_REQUIREMENTS.md | 34. Reliability Requirements | An HDD-module failure MUST NOT automatically corrupt the core ledger. | Q1-SYS-203 | CONFLICT | Definition occurrence mapped independently |
| Q1-REL-006 | 02_SYSTEM_REQUIREMENTS.md | 34. Reliability Requirements | An AI-observer failure MUST NOT stop consensus. | Q1-SYS-204 | CONFLICT | Definition occurrence mapped independently |
| Q1-CFG-001 | 02_SYSTEM_REQUIREMENTS.md | 35. Configuration Requirements | Experimental parameters MUST be configurable without changing source code. | Q1-SYS-205 | CONFLICT | Definition occurrence mapped independently |
| Q1-CFG-002 | 02_SYSTEM_REQUIREMENTS.md | 35. Configuration Requirements | Configurable parameters MUST include: | Q1-SYS-206 | CONFLICT | Definition occurrence mapped independently |
| Q1-CFG-003 | 02_SYSTEM_REQUIREMENTS.md | 35. Configuration Requirements | Consensus-critical configuration MUST be part of genesis or protocol versioning. | Q1-SYS-207 | CONFLICT | Definition occurrence mapped independently |
| Q1-CFG-004 | 02_SYSTEM_REQUIREMENTS.md | 35. Configuration Requirements | Nodes with incompatible consensus configuration MUST refuse participation. | Q1-SYS-208 | CONFLICT | Definition occurrence mapped independently |
| Q1-LOG-001 | 02_SYSTEM_REQUIREMENTS.md | 36. Logging Requirements | All components MUST use structured logging. | Q1-SYS-209 | CONFLICT | Definition occurrence mapped independently |
| Q1-LOG-002 | 02_SYSTEM_REQUIREMENTS.md | 36. Logging Requirements | Logs MUST support at least: | Q1-SYS-210 | CONFLICT | Definition occurrence mapped independently |
| Q1-LOG-003 | 02_SYSTEM_REQUIREMENTS.md | 36. Logging Requirements | Consensus-critical events MUST have stable event identifiers. | Q1-SYS-211 | CONFLICT | Definition occurrence mapped independently |
| Q1-LOG-004 | 02_SYSTEM_REQUIREMENTS.md | 36. Logging Requirements | Each block round MUST have a traceable round identifier. | Q1-SYS-212 | CONFLICT | Definition occurrence mapped independently |
| Q1-LOG-005 | 02_SYSTEM_REQUIREMENTS.md | 36. Logging Requirements | Logs MUST allow reconstruction of why a block or transaction was rejected. | Q1-SYS-213 | CONFLICT | Definition occurrence mapped independently |
| Q1-API-001 | 02_SYSTEM_REQUIREMENTS.md | 37. API Requirements | The node MUST expose an API for: | Q1-SYS-214 | CONFLICT | Definition occurrence mapped independently |
| Q1-API-002 | 02_SYSTEM_REQUIREMENTS.md | 37. API Requirements | Administrative APIs MUST be separated from public read APIs. | Q1-SYS-215 | CONFLICT | Definition occurrence mapped independently |
| Q1-API-003 | 02_SYSTEM_REQUIREMENTS.md | 37. API Requirements | Administrative APIs MUST require authentication. | Q1-SYS-216 | CONFLICT | Definition occurrence mapped independently |
| Q1-API-004 | 02_SYSTEM_REQUIREMENTS.md | 37. API Requirements | The API MUST be versioned. | Q1-SYS-217 | CONFLICT | Definition occurrence mapped independently |
| Q1-DEV-001 | 02_SYSTEM_REQUIREMENTS.md | 38. Development Requirements | The codebase MUST be organized into independent modules. | Q1-SYS-218 | CONFLICT | Definition occurrence mapped independently |
| Q1-DEV-002 | 02_SYSTEM_REQUIREMENTS.md | 38. Development Requirements | Consensus logic MUST be isolated from user-interface code. | Q1-SYS-219 | CONFLICT | Definition occurrence mapped independently |
| Q1-DEV-003 | 02_SYSTEM_REQUIREMENTS.md | 38. Development Requirements | Cryptographic operations MUST be isolated behind clear interfaces. | Q1-SYS-220 | CONFLICT | Definition occurrence mapped independently |
| Q1-DEV-004 | 02_SYSTEM_REQUIREMENTS.md | 38. Development Requirements | The HDD module MUST be isolated behind a plugin interface. | Q1-SYS-221 | CONFLICT | Definition occurrence mapped independently |
| Q1-DEV-005 | 02_SYSTEM_REQUIREMENTS.md | 38. Development Requirements | The AI observer MUST be a separate service or process. | Q1-SYS-222 | CONFLICT | Definition occurrence mapped independently |
| Q1-DEV-006 | 02_SYSTEM_REQUIREMENTS.md | 38. Development Requirements | Every mandatory requirement SHOULD map to: | Q1-SYS-223 | CONFLICT | Definition occurrence mapped independently |
| Q1-DEV-007 | 02_SYSTEM_REQUIREMENTS.md | 38. Development Requirements | The project MUST support reproducible local setup. | Q1-SYS-224 | CONFLICT | Definition occurrence mapped independently |
| Q1-DEV-008 | 02_SYSTEM_REQUIREMENTS.md | 38. Development Requirements | The project MUST include: | Q1-SYS-225 | CONFLICT | Definition occurrence mapped independently |
| Q1-TST-001 | 02_SYSTEM_REQUIREMENTS.md | 39. Testing Requirements | Every deterministic protocol rule MUST have automated tests. | Q1-SYS-226 | CONFLICT | Definition occurrence mapped independently |
| Q1-TST-002 | 02_SYSTEM_REQUIREMENTS.md | 39. Testing Requirements | Tests MUST include: | Q1-SYS-227 | CONFLICT | Definition occurrence mapped independently |
| Q1-TST-003 | 02_SYSTEM_REQUIREMENTS.md | 39. Testing Requirements | The system MUST support deterministic test seeds. | Q1-SYS-228 | CONFLICT | Definition occurrence mapped independently |
| Q1-TST-004 | 02_SYSTEM_REQUIREMENTS.md | 39. Testing Requirements | A failed consensus test MUST block release. | Q1-SYS-229 | CONFLICT | Definition occurrence mapped independently |
| Q1-TST-005 | 02_SYSTEM_REQUIREMENTS.md | 39. Testing Requirements | The test environment MUST be able to create malicious nodes. | Q1-SYS-230 | CONFLICT | Definition occurrence mapped independently |
| Q1-TST-006 | 02_SYSTEM_REQUIREMENTS.md | 39. Testing Requirements | A malicious node MUST be configurable to: | Q1-SYS-231 | CONFLICT | Definition occurrence mapped independently |
| Q1-ARC-001 | 03_ARCHITECTURE.md | 2. Architectural Objectives | Deterministic core | Q1-ARC-001 | UNCHANGED | Definition occurrence mapped independently |
| Q1-ARC-002 | 03_ARCHITECTURE.md | 2. Architectural Objectives | Modular replacement | Q1-ARC-002 | UNCHANGED | Definition occurrence mapped independently |
| Q1-ARC-003 | 03_ARCHITECTURE.md | 2. Architectural Objectives | Safety isolation | Q1-ARC-003 | UNCHANGED | Definition occurrence mapped independently |
| Q1-ARC-004 | 03_ARCHITECTURE.md | 2. Architectural Objectives | Observable execution | Q1-ARC-004 | UNCHANGED | Definition occurrence mapped independently |
| Q1-ARC-005 | 03_ARCHITECTURE.md | 2. Architectural Objectives | Adversarial testability | Q1-ARC-005 | UNCHANGED | Definition occurrence mapped independently |
| Q1-ARC-006 | 03_ARCHITECTURE.md | 2. Architectural Objectives | Progressive decentralization | Q1-ARC-006 | UNCHANGED | Definition occurrence mapped independently |
| Q1-LTX-001 | 04_LEDGER_AND_TRANSACTIONS.md | 2. Ledger Model | Account-based ledger | Q1-LTX-001 | UNCHANGED | Definition occurrence mapped independently |
| Q1-LTX-002 | 04_LEDGER_AND_TRANSACTIONS.md | 2. Ledger Model | Rationale | Q1-LTX-002 | UNCHANGED | Definition occurrence mapped independently |
| Q1-LTX-003 | 04_LEDGER_AND_TRANSACTIONS.md | 3. Native Test Asset | Temporary asset identity | Q1-LTX-003 | UNCHANGED | Definition occurrence mapped independently |
| Q1-LTX-004 | 04_LEDGER_AND_TRANSACTIONS.md | 3. Native Test Asset | Integer representation | Q1-LTX-004 | UNCHANGED | Definition occurrence mapped independently |
| Q1-LTX-005 | 04_LEDGER_AND_TRANSACTIONS.md | 3. Native Test Asset | Maximum numeric range | Q1-LTX-005 | UNCHANGED | Definition occurrence mapped independently |
| Q1-LTX-006 | 04_LEDGER_AND_TRANSACTIONS.md | 4. Account State | Balance | Q1-LTX-006 | UNCHANGED | Definition occurrence mapped independently |
| Q1-LTX-007 | 04_LEDGER_AND_TRANSACTIONS.md | 4. Account State | Account nonce | Q1-LTX-007 | UNCHANGED | Definition occurrence mapped independently |
| Q1-LTX-008 | 04_LEDGER_AND_TRANSACTIONS.md | 4. Account State | Account creation | Q1-LTX-008 | UNCHANGED | Definition occurrence mapped independently |
| Q1-LTX-009 | 04_LEDGER_AND_TRANSACTIONS.md | 4. Account State | Empty accounts | Q1-LTX-009 | UNCHANGED | Definition occurrence mapped independently |
| Q1-LTX-010 | 04_LEDGER_AND_TRANSACTIONS.md | 5. Address Model | Address derivation | Q1-LTX-010 | UNCHANGED | Definition occurrence mapped independently |
| Q1-LTX-011 | 04_LEDGER_AND_TRANSACTIONS.md | 5. Address Model | Network separation | Q1-LTX-011 | UNCHANGED | Definition occurrence mapped independently |
| Q1-LTX-012 | 04_LEDGER_AND_TRANSACTIONS.md | 5. Address Model | Address encoding | Q1-LTX-012 | UNCHANGED | Definition occurrence mapped independently |
| Q1-LTX-013 | 04_LEDGER_AND_TRANSACTIONS.md | 5. Address Model | Public key disclosure | Q1-LTX-013 | UNCHANGED | Definition occurrence mapped independently |
| Q1-LTX-014 | 04_LEDGER_AND_TRANSACTIONS.md | 11. Canonical Serialization | Deterministic bytes | Q1-LTX-014 | UNCHANGED | Definition occurrence mapped independently |
| Q1-LTX-015 | 04_LEDGER_AND_TRANSACTIONS.md | 11. Canonical Serialization | No ambiguous encodings | Q1-LTX-015 | UNCHANGED | Definition occurrence mapped independently |
| Q1-LTX-016 | 04_LEDGER_AND_TRANSACTIONS.md | 11. Canonical Serialization | Serialization format decision | Q1-LTX-016 | UNCHANGED | Definition occurrence mapped independently |
| Q1-LTX-017 | 04_LEDGER_AND_TRANSACTIONS.md | 15. Mempool Nonce Rules | Stale nonce | Q1-LTX-017 | UNCHANGED | Definition occurrence mapped independently |
| Q1-LTX-018 | 04_LEDGER_AND_TRANSACTIONS.md | 15. Mempool Nonce Rules | Future nonce | Q1-LTX-018 | UNCHANGED | Definition occurrence mapped independently |
| Q1-LTX-019 | 04_LEDGER_AND_TRANSACTIONS.md | 15. Mempool Nonce Rules | Maximum nonce gap | Q1-LTX-019 | UNCHANGED | Definition occurrence mapped independently |
| Q1-LTX-020 | 04_LEDGER_AND_TRANSACTIONS.md | 18. Transaction Ordering | Deterministic execution order | Q1-LTX-020 | UNCHANGED | Definition occurrence mapped independently |
| Q1-LTX-021 | 04_LEDGER_AND_TRANSACTIONS.md | 18. Transaction Ordering | Same-sender ordering | Q1-LTX-021 | UNCHANGED | Definition occurrence mapped independently |
| Q1-LTX-022 | 04_LEDGER_AND_TRANSACTIONS.md | 18. Transaction Ordering | Candidate ordering policy | Q1-LTX-022 | UNCHANGED | Definition occurrence mapped independently |
| Q1-LTX-023 | 04_LEDGER_AND_TRANSACTIONS.md | 22. Transaction Receipts | Receipt commitment | Q1-LTX-023 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-001 | 05_CONSENSUS.md | 7. Block Height and Round | Height | Q1-CON-001 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-002 | 05_CONSENSUS.md | 7. Block Height and Round | Round | Q1-CON-002 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-003 | 05_CONSENSUS.md | 7. Block Height and Round | Round uniqueness | Q1-CON-003 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-004 | 05_CONSENSUS.md | 10. Consensus Randomness | Round seed | Q1-CON-004 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-005 | 05_CONSENSUS.md | 10. Consensus Randomness | No producer-controlled seed | Q1-CON-005 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-006 | 05_CONSENSUS.md | 10. Consensus Randomness | Deterministic derivation | Q1-CON-006 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-007 | 05_CONSENSUS.md | 10. Consensus Randomness | Bias analysis | Q1-CON-007 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-008 | 05_CONSENSUS.md | 11. Producer Selection | Ordered candidate list | Q1-CON-008 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-009 | 05_CONSENSUS.md | 11. Producer Selection | Deterministic selection | Q1-CON-009 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-010 | 05_CONSENSUS.md | 11. Producer Selection | Selection interface | Q1-CON-010 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-011 | 05_CONSENSUS.md | 11. Producer Selection | Eligibility filter | Q1-CON-011 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-012 | 05_CONSENSUS.md | 11. Producer Selection | Initial weighted lottery | Q1-CON-012 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-013 | 05_CONSENSUS.md | 11. Producer Selection | No unlimited wealth control | Q1-CON-013 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-014 | 05_CONSENSUS.md | 11. Producer Selection | Cooldown | Q1-CON-014 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-015 | 05_CONSENSUS.md | 11. Producer Selection | Duplicate candidate prevention | Q1-CON-015 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-016 | 05_CONSENSUS.md | 13. Validator Committee Selection | Temporary committee | Q1-CON-016 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-017 | 05_CONSENSUS.md | 13. Validator Committee Selection | Deterministic selection | Q1-CON-017 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-018 | 05_CONSENSUS.md | 13. Validator Committee Selection | Initial committee size | Q1-CON-018 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-019 | 05_CONSENSUS.md | 13. Validator Committee Selection | Producer exclusion | Q1-CON-019 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-020 | 05_CONSENSUS.md | 13. Validator Committee Selection | Candidate independence | Q1-CON-020 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-021 | 05_CONSENSUS.md | 13. Validator Committee Selection | Committee diversity | Q1-CON-021 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-022 | 05_CONSENSUS.md | 15. Quorum Threshold | Supermajority | Q1-CON-022 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-023 | 05_CONSENSUS.md | 15. Quorum Threshold | Threshold examples | Q1-CON-023 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-024 | 05_CONSENSUS.md | 15. Quorum Threshold | No rounding ambiguity | Q1-CON-024 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-025 | 05_CONSENSUS.md | 17. Production Windows | Deterministic windows | Q1-CON-025 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-026 | 05_CONSENSUS.md | 17. Production Windows | Window duration | Q1-CON-026 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-027 | 05_CONSENSUS.md | 17. Production Windows | Early fallback prohibition | Q1-CON-027 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-028 | 05_CONSENSUS.md | 17. Production Windows | Late proposal handling | Q1-CON-028 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-029 | 05_CONSENSUS.md | 23. One Vote per Height and Round | No double attestation | Q1-CON-029 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-030 | 05_CONSENSUS.md | 23. One Vote per Height and Round | Persistent signing protection | Q1-CON-030 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-031 | 05_CONSENSUS.md | 27. Finality | Immediate protocol finality | Q1-CON-031 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-032 | 05_CONSENSUS.md | 27. Finality | No ordinary reorganization below finality | Q1-CON-032 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-033 | 05_CONSENSUS.md | 27. Finality | Conflicting finality | Q1-CON-033 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-034 | 05_CONSENSUS.md | 30. Producer Timeout and Fallback | Timeout | Q1-CON-034 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-035 | 05_CONSENSUS.md | 30. Producer Timeout and Fallback | Fallback activation | Q1-CON-035 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-036 | 05_CONSENSUS.md | 30. Producer Timeout and Fallback | No centralized timeout authority | Q1-CON-036 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-037 | 05_CONSENSUS.md | 30. Producer Timeout and Fallback | Simplified prototype fallback | Q1-CON-037 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CON-038 | 05_CONSENSUS.md | 36. Equivocation Evidence | Objective verification | Q1-CON-038 | UNCHANGED | Definition occurrence mapped independently |
| Q1-DLY-001 | 06_DELAY_ENGINE.md | 7. Challenge Construction | Canonical challenge | Q1-DLY-001 | UNCHANGED | Definition occurrence mapped independently |
| Q1-DLY-002 | 06_DELAY_ENGINE.md | 7. Challenge Construction | Freshness | Q1-DLY-002 | UNCHANGED | Definition occurrence mapped independently |
| Q1-DLY-003 | 06_DELAY_ENGINE.md | 7. Challenge Construction | Precomputation resistance | Q1-DLY-003 | UNCHANGED | Definition occurrence mapped independently |
| Q1-DLY-004 | 06_DELAY_ENGINE.md | 7. Challenge Construction | Producer binding | Q1-DLY-004 | UNCHANGED | Definition occurrence mapped independently |
| Q1-DLY-005 | 06_DELAY_ENGINE.md | 7. Challenge Construction | Round binding | Q1-DLY-005 | UNCHANGED | Definition occurrence mapped independently |
| Q1-DLY-006 | 06_DELAY_ENGINE.md | 7. Challenge Construction | Engine binding | Q1-DLY-006 | UNCHANGED | Definition occurrence mapped independently |
| Q1-DLY-007 | 06_DELAY_ENGINE.md | 11. Formal VDF Requirements | Sequentiality claim | Q1-DLY-007 | UNCHANGED | Definition occurrence mapped independently |
| Q1-DLY-008 | 06_DELAY_ENGINE.md | 11. Formal VDF Requirements | Efficient verification | Q1-DLY-008 | UNCHANGED | Definition occurrence mapped independently |
| Q1-DLY-009 | 06_DELAY_ENGINE.md | 11. Formal VDF Requirements | Compact proof | Q1-DLY-009 | UNCHANGED | Definition occurrence mapped independently |
| Q1-DLY-010 | 06_DELAY_ENGINE.md | 11. Formal VDF Requirements | Public construction | Q1-DLY-010 | UNCHANGED | Definition occurrence mapped independently |
| Q1-DLY-011 | 06_DELAY_ENGINE.md | 11. Formal VDF Requirements | No custom cryptography by convenience | Q1-DLY-011 | UNCHANGED | Definition occurrence mapped independently |
| Q1-DLY-012 | 06_DELAY_ENGINE.md | 11. Formal VDF Requirements | Trusted setup disclosure | Q1-DLY-012 | UNCHANGED | Definition occurrence mapped independently |
| Q1-DLY-013 | 06_DELAY_ENGINE.md | 11. Formal VDF Requirements | Parameter transparency | Q1-DLY-013 | UNCHANGED | Definition occurrence mapped independently |
| Q1-DLY-014 | 06_DELAY_ENGINE.md | 12. Difficulty Model | Integer difficulty | Q1-DLY-014 | UNCHANGED | Definition occurrence mapped independently |
| Q1-DLY-015 | 06_DELAY_ENGINE.md | 12. Difficulty Model | Bounds | Q1-DLY-015 | UNCHANGED | Definition occurrence mapped independently |
| Q1-DLY-016 | 06_DELAY_ENGINE.md | 12. Difficulty Model | Consensus value | Q1-DLY-016 | UNCHANGED | Definition occurrence mapped independently |
| Q1-DLY-017 | 06_DELAY_ENGINE.md | 13. Difficulty Adjustment | Bounded adjustment | Q1-DLY-017 | UNCHANGED | Definition occurrence mapped independently |
| Q1-DLY-018 | 06_DELAY_ENGINE.md | 13. Difficulty Adjustment | Adjustment interval | Q1-DLY-018 | UNCHANGED | Definition occurrence mapped independently |
| Q1-DLY-019 | 06_DELAY_ENGINE.md | 13. Difficulty Adjustment | Robust statistic | Q1-DLY-019 | UNCHANGED | Definition occurrence mapped independently |
| Q1-DLY-020 | 06_DELAY_ENGINE.md | 13. Difficulty Adjustment | Telemetry versus consensus | Q1-DLY-020 | UNCHANGED | Definition occurrence mapped independently |
| Q1-DLY-021 | 06_DELAY_ENGINE.md | 27. HDD Composite Mode | No HDD sole authority | Q1-DLY-021 | UNCHANGED | Definition occurrence mapped independently |
| Q1-DLY-022 | 06_DELAY_ENGINE.md | 27. HDD Composite Mode | Replay resistance | Q1-DLY-022 | UNCHANGED | Definition occurrence mapped independently |
| Q1-DLY-023 | 06_DELAY_ENGINE.md | 27. HDD Composite Mode | Simulated hardware acceptance | Q1-DLY-023 | UNCHANGED | Definition occurrence mapped independently |
| Q1-DLY-024 | 06_DELAY_ENGINE.md | 27. HDD Composite Mode | Mode declaration | Q1-DLY-024 | UNCHANGED | Definition occurrence mapped independently |
| Q1-HDD-001 | 07_HDD_LAB_MODULE.md | 14. Dataset Requirements | Deterministic generation | Q1-HDD-001 | UNCHANGED | Definition occurrence mapped independently |
| Q1-HDD-002 | 07_HDD_LAB_MODULE.md | 14. Dataset Requirements | Dataset commitment | Q1-HDD-002 | UNCHANGED | Definition occurrence mapped independently |
| Q1-HDD-003 | 07_HDD_LAB_MODULE.md | 14. Dataset Requirements | No secret dataset | Q1-HDD-003 | UNCHANGED | Definition occurrence mapped independently |
| Q1-HDD-004 | 07_HDD_LAB_MODULE.md | 14. Dataset Requirements | Configurable size | Q1-HDD-004 | UNCHANGED | Definition occurrence mapped independently |
| Q1-HDD-005 | 07_HDD_LAB_MODULE.md | 14. Dataset Requirements | Capacity fairness | Q1-HDD-005 | UNCHANGED | Definition occurrence mapped independently |
| Q1-NET-001 | 08_NODE_AND_NETWORKING.md | 4. Node Identity | Separate keys | Q1-NET-001 | UNCHANGED | Definition occurrence mapped independently |
| Q1-NET-002 | 08_NODE_AND_NETWORKING.md | 4. Node Identity | Identity persistence | Q1-NET-002 | UNCHANGED | Definition occurrence mapped independently |
| Q1-NET-003 | 08_NODE_AND_NETWORKING.md | 4. Node Identity | Identity rotation | Q1-NET-003 | UNCHANGED | Definition occurrence mapped independently |
| Q1-NET-004 | 08_NODE_AND_NETWORKING.md | 4. Node Identity | No proof of uniqueness | Q1-NET-004 | UNCHANGED | Definition occurrence mapped independently |
| Q1-WAL-001 | 09_WALLET.md | 2. Wallet Design Principles | User control | Q1-WAL-001 | UNCHANGED | Definition occurrence mapped independently |
| Q1-WAL-002 | 09_WALLET.md | 2. Wallet Design Principles | Local signing | Q1-WAL-002 | UNCHANGED | Definition occurrence mapped independently |
| Q1-WAL-003 | 09_WALLET.md | 2. Wallet Design Principles | Explicit finality | Q1-WAL-003 | UNCHANGED | Definition occurrence mapped independently |
| Q1-WAL-004 | 09_WALLET.md | 2. Wallet Design Principles | Network clarity | Q1-WAL-004 | UNCHANGED | Definition occurrence mapped independently |
| Q1-WAL-005 | 09_WALLET.md | 2. Wallet Design Principles | Secure defaults | Q1-WAL-005 | UNCHANGED | Definition occurrence mapped independently |
| Q1-WAL-006 | 09_WALLET.md | 2. Wallet Design Principles | Simplicity before feature expansion | Q1-WAL-006 | UNCHANGED | Definition occurrence mapped independently |
| Q1-WAL-007 | 09_WALLET.md | 9. Key Model | Private key isolation | Q1-WAL-007 | UNCHANGED | Definition occurrence mapped independently |
| Q1-WAL-008 | 09_WALLET.md | 9. Key Model | No plaintext persistence | Q1-WAL-008 | UNCHANGED | Definition occurrence mapped independently |
| Q1-WAL-009 | 09_WALLET.md | 9. Key Model | Memory handling | Q1-WAL-009 | UNCHANGED | Definition occurrence mapped independently |
| Q1-WAL-010 | 09_WALLET.md | 9. Key Model | Key reuse | Q1-WAL-010 | UNCHANGED | Definition occurrence mapped independently |
| Q1-WAL-011 | 09_WALLET.md | 11. Recovery Material | No false recovery promise | Q1-WAL-011 | UNCHANGED | Definition occurrence mapped independently |
| Q1-WAL-012 | 09_WALLET.md | 11. Recovery Material | Recovery verification | Q1-WAL-012 | UNCHANGED | Definition occurrence mapped independently |
| Q1-WAL-013 | 09_WALLET.md | 19. Network Profiles | Trusted configuration | Q1-WAL-013 | UNCHANGED | Definition occurrence mapped independently |
| Q1-WAL-014 | 09_WALLET.md | 19. Network Profiles | Network switch | Q1-WAL-014 | UNCHANGED | Definition occurrence mapped independently |
| Q1-TKN-001 | 10_TOKENOMICS.md | 9. Genesis Allocation | Transparent genesis | Q1-TKN-001 | UNCHANGED | Definition occurrence mapped independently |
| Q1-TKN-002 | 10_TOKENOMICS.md | 9. Genesis Allocation | Deterministic total | Q1-TKN-002 | UNCHANGED | Definition occurrence mapped independently |
| Q1-TKN-003 | 10_TOKENOMICS.md | 9. Genesis Allocation | No hidden reserve | Q1-TKN-003 | UNCHANGED | Definition occurrence mapped independently |
| Q1-TKN-004 | 10_TOKENOMICS.md | 9. Genesis Allocation | Testnet reset freedom | Q1-TKN-004 | UNCHANGED | Definition occurrence mapped independently |
| Q1-AI-001 | 11_AI_OBSERVER.md | 4. Core Principles | Non-authority | Q1-AIO-001 | RENAMED | Definition occurrence mapped independently |
| Q1-AI-002 | 11_AI_OBSERVER.md | 4. Core Principles | Consensus independence | Q1-AIO-002 | RENAMED | Definition occurrence mapped independently |
| Q1-AI-003 | 11_AI_OBSERVER.md | 4. Core Principles | Read-only design | Q1-AIO-003 | RENAMED | Definition occurrence mapped independently |
| Q1-AI-004 | 11_AI_OBSERVER.md | 4. Core Principles | Explainability | Q1-AIO-004 | RENAMED | Definition occurrence mapped independently |
| Q1-AI-005 | 11_AI_OBSERVER.md | 4. Core Principles | Versioning | Q1-AIO-005 | RENAMED | Definition occurrence mapped independently |
| Q1-AI-006 | 11_AI_OBSERVER.md | 4. Core Principles | Reproducibility | Q1-AIO-006 | RENAMED | Definition occurrence mapped independently |
| Q1-AI-007 | 11_AI_OBSERVER.md | 4. Core Principles | Human review | Q1-AIO-007 | RENAMED | Definition occurrence mapped independently |
| Q1-SEC-001 | 12_SECURITY_MODEL.md | 9. Security Assumptions | Cryptographic assumptions | Q1-SEC-001 | UNCHANGED | Definition occurrence mapped independently |
| Q1-SEC-002 | 12_SECURITY_MODEL.md | 9. Security Assumptions | Key secrecy | Q1-SEC-002 | UNCHANGED | Definition occurrence mapped independently |
| Q1-SEC-003 | 12_SECURITY_MODEL.md | 9. Security Assumptions | Byzantine threshold | Q1-SEC-003 | UNCHANGED | Definition occurrence mapped independently |
| Q1-SEC-004 | 12_SECURITY_MODEL.md | 9. Security Assumptions | Deterministic execution | Q1-SEC-004 | UNCHANGED | Definition occurrence mapped independently |
| Q1-SEC-005 | 12_SECURITY_MODEL.md | 9. Security Assumptions | Network availability | Q1-SEC-005 | UNCHANGED | Definition occurrence mapped independently |
| Q1-SEC-006 | 12_SECURITY_MODEL.md | 9. Security Assumptions | Finalized participant registry | Q1-SEC-006 | UNCHANGED | Definition occurrence mapped independently |
| Q1-SEC-007 | 12_SECURITY_MODEL.md | 9. Security Assumptions | Delay verification | Q1-SEC-007 | UNCHANGED | Definition occurrence mapped independently |
| Q1-SEC-008 | 12_SECURITY_MODEL.md | 9. Security Assumptions | Software integrity | Q1-SEC-008 | UNCHANGED | Definition occurrence mapped independently |
| Q1-SEC-009 | 12_SECURITY_MODEL.md | 9. Security Assumptions | Prototype admission limitation | Q1-SEC-009 | UNCHANGED | Definition occurrence mapped independently |
| Q1-TST-001 | 14_TEST_PLAN.md | 2. Testing Philosophy | Test behavior, not intention | Q1-TST-001 | UNCHANGED | Definition occurrence mapped independently |
| Q1-TST-002 | 14_TEST_PLAN.md | 2. Testing Philosophy | Determinism before scale | Q1-TST-002 | UNCHANGED | Definition occurrence mapped independently |
| Q1-TST-003 | 14_TEST_PLAN.md | 2. Testing Philosophy | Safety before liveness | Q1-TST-003 | UNCHANGED | Definition occurrence mapped independently |
| Q1-TST-004 | 14_TEST_PLAN.md | 2. Testing Philosophy | Failure is valid output | Q1-TST-004 | UNCHANGED | Definition occurrence mapped independently |
| Q1-TST-005 | 14_TEST_PLAN.md | 2. Testing Philosophy | Reproducibility | Q1-TST-005 | UNCHANGED | Definition occurrence mapped independently |
| Q1-TST-006 | 14_TEST_PLAN.md | 2. Testing Philosophy | Isolation | Q1-TST-006 | UNCHANGED | Definition occurrence mapped independently |
| Q1-TST-007 | 14_TEST_PLAN.md | 2. Testing Philosophy | Requirement traceability | Q1-TST-007 | UNCHANGED | Definition occurrence mapped independently |
| Q1-SIM-001 | 15_SIMULATION_PLAN.md | 2. Simulation Philosophy | Simulation is not reality | Q1-SIM-001 | UNCHANGED | Definition occurrence mapped independently |
| Q1-SIM-002 | 15_SIMULATION_PLAN.md | 2. Simulation Philosophy | Assumptions must be explicit | Q1-SIM-002 | UNCHANGED | Definition occurrence mapped independently |
| Q1-SIM-003 | 15_SIMULATION_PLAN.md | 2. Simulation Philosophy | Multiple models | Q1-SIM-003 | UNCHANGED | Definition occurrence mapped independently |
| Q1-SIM-004 | 15_SIMULATION_PLAN.md | 2. Simulation Philosophy | Adversarial agents | Q1-SIM-004 | UNCHANGED | Definition occurrence mapped independently |
| Q1-SIM-005 | 15_SIMULATION_PLAN.md | 2. Simulation Philosophy | Reproducibility | Q1-SIM-005 | UNCHANGED | Definition occurrence mapped independently |
| Q1-SIM-006 | 15_SIMULATION_PLAN.md | 2. Simulation Philosophy | Sensitivity over certainty | Q1-SIM-006 | UNCHANGED | Definition occurrence mapped independently |
| Q1-API-001 | 16_API_SPECIFICATION.md | 2. API Design Principles | Protocol rules remain authoritative | Q1-API-001 | UNCHANGED | Definition occurrence mapped independently |
| Q1-API-002 | 16_API_SPECIFICATION.md | 2. API Design Principles | Separation of privilege | Q1-API-002 | UNCHANGED | Definition occurrence mapped independently |
| Q1-API-003 | 16_API_SPECIFICATION.md | 2. API Design Principles | Versioned interfaces | Q1-API-003 | UNCHANGED | Definition occurrence mapped independently |
| Q1-API-004 | 16_API_SPECIFICATION.md | 2. API Design Principles | Stable machine behavior | Q1-API-004 | UNCHANGED | Definition occurrence mapped independently |
| Q1-API-005 | 16_API_SPECIFICATION.md | 2. API Design Principles | Exact monetary values | Q1-API-005 | UNCHANGED | Definition occurrence mapped independently |
| Q1-API-006 | 16_API_SPECIFICATION.md | 2. API Design Principles | Safe defaults | Q1-API-006 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CBX-001 | 17_CODEX_BUILD_INSTRUCTIONS.md | 4. Build Philosophy | Correctness before complexity | Q1-CBX-001 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CBX-002 | 17_CODEX_BUILD_INSTRUCTIONS.md | 4. Build Philosophy | Small complete milestones | Q1-CBX-002 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CBX-003 | 17_CODEX_BUILD_INSTRUCTIONS.md | 4. Build Philosophy | Test-first protocol development | Q1-CBX-003 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CBX-004 | 17_CODEX_BUILD_INSTRUCTIONS.md | 7. Documentation | No hidden assumptions | Q1-CBX-004 | UNCHANGED | Definition occurrence mapped independently |
| Q1-CBX-005 | 17_CODEX_BUILD_INSTRUCTIONS.md | 7. Documentation | Experimental honesty | Q1-CBX-005 | UNCHANGED | Definition occurrence mapped independently |

Definitions mapped: 389
