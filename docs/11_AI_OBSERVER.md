Q1 AI Observer Specification

11_AI_OBSERVER.md

Project: Q1 Experimental Distributed Ledger
Protocol Version: 0.1
Document Version: 0.1.0
Status: Draft for Engineering and Security Review
Classification: Experimental — Non-Consensus — Not for Production or Financial Use

⸻

1. Purpose

This document defines the Q1 AI Observer.

The AI Observer is a non-consensus analytical subsystem designed to:

* ingest telemetry;
* identify anomalies;
* detect suspicious patterns;
* compare experiments;
* assist security analysis;
* explain operational risks;
* generate research reports;
* recommend additional tests;
* help human operators understand complex network behavior.

The AI Observer is not a source of protocol truth.

It MUST NOT:

* determine transaction validity;
* determine block validity;
* cast validator attestations;
* create finalization certificates;
* hold user private keys;
* modify balances;
* change consensus parameters;
* penalize participants automatically;
* freeze accounts;
* blacklist nodes by itself;
* replace deterministic protocol verification.

The guiding principle is:

AI may observe uncertainty.
Consensus must resolve validity through deterministic rules.

⸻

2. Role of AI in Q1

Q1 uses AI for assistance in areas where:

* data volume is large;
* patterns may be difficult for humans to detect;
* several weak indicators must be considered together;
* behavior changes over time;
* attack strategies may be novel;
* operational explanations are valuable.

AI SHALL be used for:

* observation;
* classification;
* prioritization;
* correlation;
* explanation;
* hypothesis generation;
* test generation;
* report generation.

AI SHALL NOT be used as a hidden consensus oracle.

⸻

3. Architectural Position

The AI Observer SHALL operate outside the deterministic protocol path.

Q1 Nodes
   │
   ├── Structured Logs
   ├── Metrics
   ├── Consensus Events
   ├── Network Events
   ├── Delay Metrics
   ├── HDD Metrics
   └── Economic Metrics
          │
          ▼
    Telemetry Ingestion
          │
          ▼
    Feature Extraction
          │
          ▼
 ┌─────────────────────────────┐
 │     Q1 AI Observer          │
 │                             │
 │ Rule-Based Detection        │
 │ Statistical Detection       │
 │ AI Model Analysis           │
 │ Risk Scoring                │
 │ Explanation                 │
 │ Report Generation           │
 └─────────────┬───────────────┘
               │
               ▼
       Alerts and Reports
No direct write path to:
- ledger state
- consensus votes
- wallet keys
- participant penalties

⸻

4. Core Principles

Q1-AIO-001 — Non-authority

AI output MUST be advisory.

No AI result may independently change consensus state.

⸻

Q1-AIO-002 — Consensus independence

The network MUST continue to:

* validate transactions;
* validate blocks;
* select producers;
* select committees;
* finalize blocks;
* synchronize nodes

when the AI Observer is offline.

⸻

Q1-AIO-003 — Read-only design

The AI Observer SHOULD use read-only access to:

* telemetry;
* indexed finalized data;
* experiment datasets;
* public network status.

It MUST NOT have direct write access to:

* ledger databases;
* keystores;
* consensus state;
* participant registry;
* treasury accounts;
* wallet files.

⸻

Q1-AIO-004 — Explainability

Every material AI alert SHOULD include:

* what was detected;
* why it was considered unusual;
* what data supported the conclusion;
* what uncertainty remains;
* what action is recommended;
* whether the alert came from rules, statistics, or an AI model.

⸻

Q1-AIO-005 — Versioning

Every model, rule set, feature schema, and scoring configuration MUST be versioned.

⸻

Q1-AIO-006 — Reproducibility

Where possible, an alert SHOULD be reproducible from:

* the same telemetry;
* the same feature pipeline;
* the same model version;
* the same configuration.

⸻

Q1-AIO-007 — Human review

High-impact operational recommendations SHOULD require human review.

⸻

5. AI Observer Scope

The AI Observer MAY analyze:

* block timing anomalies;
* producer concentration;
* validator concentration;
* repeated fallback activation;
* invalid block patterns;
* invalid delay proofs;
* consensus round failures;
* validator equivocation indicators;
* producer equivocation indicators;
* suspicious peer clustering;
* eclipse indicators;
* network partitions;
* transaction spam;
* fee manipulation;
* mempool anomalies;
* abnormal reward concentration;
* HDD telemetry anomalies;
* delay-engine performance anomalies;
* energy anomalies;
* unusual software-version clusters;
* repeated node crashes;
* abnormal recovery behavior;
* testnet attack outcomes.

⸻

6. Explicit Non-Scope

The AI Observer MUST NOT:

* classify a legal person as guilty;
* infer legal ownership of funds;
* identify real-world users without lawful basis;
* determine whether a transaction is lawful;
* determine sanctions compliance;
* determine criminal intent;
* decide account recovery;
* reverse finalized transactions;
* override cryptographic verification;
* infer physical HDD truth with certainty;
* treat IP address as proof of identity;
* treat geography as proof of independence;
* promise attack detection completeness.

⸻

7. AI Observer Components

The AI Observer SHOULD contain:

Q1 AI Observer
├── Telemetry Ingestor
├── Data Normalizer
├── Feature Extractor
├── Rule Engine
├── Statistical Detector
├── AI Model Adapter
├── Risk Scoring Engine
├── Correlation Engine
├── Alert Manager
├── Explanation Generator
├── Incident Timeline Builder
├── Experiment Comparator
├── Report Generator
├── Model Registry
├── Evaluation Harness
└── Privacy Filter

⸻

8. Telemetry Ingestor

The Telemetry Ingestor SHALL accept structured data from:

* node runtime;
* consensus engine;
* networking layer;
* ledger engine;
* delay engine;
* HDD laboratory;
* wallet telemetry where enabled;
* economic simulator;
* test orchestrator;
* host resource monitors.

It SHOULD support:

* JSON Lines;
* message queue;
* metrics endpoint;
* time-series database;
* batch file import.

⸻

9. Input Trust Model

All telemetry MUST be treated as potentially incomplete or manipulated.

The AI Observer SHALL distinguish:

CRYPTOGRAPHICALLY_VERIFIED
PROTOCOL_DERIVED
MULTI_NODE_OBSERVED
LOCAL_MEASUREMENT
SELF_REPORTED
INFERRED
SIMULATED

These evidence classes MUST not be treated equally.

⸻

10. Evidence Reliability

Recommended conceptual reliability order:

1. Cryptographically verified protocol evidence
2. Deterministically derived finalized-chain data
3. Consistent multi-node observations
4. Independent external measurements
5. Local node measurements
6. Self-reported hardware telemetry
7. AI inference

This ordering is guidance for risk scoring.

It is not a consensus rule.

⸻

11. Event Envelope

The AI Observer SHOULD consume events using a common envelope.

ObserverEvent {
    event_id
    event_type
    event_version
    source_node_id
    source_component
    chain_id
    block_height_optional
    round_id_optional
    local_timestamp
    evidence_class
    payload
    integrity_hash_optional
}

⸻

12. Data Normalization

The normalization layer MUST:

* validate schemas;
* reject malformed records;
* standardize units;
* standardize field names;
* preserve source identity;
* preserve evidence class;
* record missing fields;
* prevent silent type conversion;
* preserve raw source references.

⸻

13. Feature Extraction

Features MAY include:

Consensus features

* round duration;
* number of fallback activations;
* attestation participation;
* quorum delay;
* invalid proposal count;
* repeated producer selection;
* repeated validator overlap;
* equivocation count;
* safe-mode events.

Network features

* peer churn;
* IP-prefix concentration;
* message latency;
* message loss;
* peer-score distribution;
* invalid-message rates;
* connection diversity;
* bootstrap dependence.

Delay features

* generation duration;
* verification duration;
* generation-to-verification ratio;
* failure rate;
* producer speed concentration;
* hardware-profile variance;
* repeated identical outputs.

HDD features

* workload duration;
* read latency distribution;
* cache-mode effect;
* device-class claim;
* SSD/HDD comparison;
* repeated commitment patterns;
* suspiciously stable latency;
* impossible device behavior;
* missing evidence.

Economic features

* reward concentration;
* fee spikes;
* transaction splitting;
* circular transfers;
* subsidy extraction;
* low-value spam;
* treasury accumulation;
* participant profitability estimates.

⸻

14. Rule-Based Detection

The Rule Engine SHALL support deterministic alerts.

Examples:

IF validator signs two conflicting blocks
THEN alert = OBJECTIVE_EQUIVOCATION
IF two conflicting finalization certificates exist
THEN alert = CRITICAL_FINALITY_CONFLICT
IF one producer creates more than X% of recent blocks
THEN alert = PRODUCER_CONCENTRATION

Rules MUST be:

* versioned;
* inspectable;
* testable;
* explainable.

⸻

15. Statistical Detection

The Statistical Detector MAY identify:

* outliers;
* trend shifts;
* unusual correlations;
* distribution changes;
* repeated temporal patterns;
* sudden concentration;
* abnormal latency;
* energy deviations.

Methods MAY include:

* moving averages;
* median absolute deviation;
* z-scores;
* quantile thresholds;
* change-point detection;
* clustering;
* time-series anomaly detection.

Statistical output remains advisory.

⸻

16. AI Model Adapter

The AI Model Adapter SHALL allow one or more models to analyze prepared features.

Possible model classes:

* classifier;
* anomaly detector;
* sequence model;
* graph model;
* large language model for explanation;
* ensemble.

The adapter MUST isolate model-specific code from the rest of the observer.

⸻

17. Model Input Controls

Before data reaches a model, the system SHOULD:

* remove secrets;
* minimize personal data;
* enforce size limits;
* validate feature schema;
* label simulated data;
* preserve model-input hashes;
* prevent prompt or payload injection where language models are used.

⸻

18. Model Output Structure

AIModelResult {
    model_id
    model_version
    feature_schema_version
    analysis_id
    category
    confidence
    risk_score
    supporting_feature_ids[]
    explanation
    limitations[]
    recommended_actions[]
}

⸻

19. Risk Score

The AI Observer MAY produce a bounded risk score.

Recommended range:

0 to 100

Interpretation:

0–19   Informational
20–39  Low concern
40–59  Suspicious
60–79  High risk
80–100 Critical

Risk score MUST NOT directly change consensus.

⸻

20. Multi-Signal Scoring

A risk score SHOULD combine:

* evidence reliability;
* severity;
* frequency;
* persistence;
* number of affected nodes;
* objective evidence;
* model confidence;
* possible impact.

Conceptual:

RiskScore =
    bounded(
        SeverityWeight
        × EvidenceReliability
        × PersistenceFactor
        × ScopeFactor
        × ConfidenceFactor
    )

This formula is non-consensus and MAY use floating-point arithmetic.

⸻

21. Alert Categories

Recommended categories:

CONSENSUS_ANOMALY
FINALITY_CONFLICT
PRODUCER_EQUIVOCATION
VALIDATOR_EQUIVOCATION
PRODUCER_CONCENTRATION
VALIDATOR_CONCENTRATION
ROUND_LIVENESS_FAILURE
NETWORK_PARTITION
ECLIPSE_SUSPECTED
PEER_FLOOD
TRANSACTION_SPAM
FEE_MANIPULATION
REWARD_FARMING
SYBIL_PATTERN
DELAY_PROOF_ANOMALY
HDD_TELEMETRY_ANOMALY
ENERGY_ANOMALY
SOFTWARE_VERSION_RISK
NODE_CRASH_CLUSTER
DATA_INTEGRITY_RISK
MODEL_UNCERTAINTY

⸻

22. Alert Structure

ObserverAlert {
    alert_id
    alert_version
    category
    severity
    risk_score
    chain_id
    affected_entities[]
    first_observed
    last_observed
    evidence_refs[]
    supporting_metrics[]
    model_or_rule_source
    explanation
    uncertainty
    recommended_actions[]
    status
}

⸻

23. Alert Status

An alert MAY be:

OPEN
ACKNOWLEDGED
INVESTIGATING
CONFIRMED
FALSE_POSITIVE
RESOLVED
ARCHIVED

These are operational states.

They do not alter consensus.

⸻

24. Objective Evidence Alerts

Where objective cryptographic evidence exists, the alert MUST identify it.

Examples:

* two conflicting signed attestations;
* two conflicting producer proposals;
* invalid finalization certificate;
* mismatched state roots;
* signature failure.

The AI Observer SHOULD distinguish:

OBJECTIVE_PROTOCOL_VIOLATION

from:

BEHAVIORAL_SUSPICION

⸻

25. Consensus Anomaly Detection

The AI Observer SHOULD detect:

* repeated round timeouts;
* unusual quorum delays;
* sudden validator absence;
* repeated fallback use;
* one producer dominating successful rounds;
* one committee composition recurring excessively;
* correlation between specific validators and failed rounds;
* abnormal safe-mode activation.

⸻

26. Network Anomaly Detection

The observer SHOULD analyze:

* peer concentration;
* sudden topology changes;
* simultaneous peer loss;
* unusual inbound floods;
* many identities from one network range;
* false high-tip claims;
* delayed block propagation;
* unusual isolation of validators;
* dependency on one bootstrap node.

⸻

27. Delay Anomaly Detection

The observer MAY flag:

* delay completion far faster than baseline;
* delay completion far slower than baseline;
* identical proof patterns across unrelated participants;
* repeated proof failures from one implementation;
* verification cost approaching generation cost;
* unusual performance jumps after software updates;
* possible specialized hardware dominance.

A fast delay result is not proof of cheating.

It is a reason for analysis.

⸻

28. HDD Anomaly Detection

The observer MAY flag:

* claimed HDD behavior matching RAM-disk performance;
* suspiciously constant random-read latency;
* device metadata changes without dataset changes;
* repeated commitment reuse;
* cache effects inconsistent with declared mode;
* identical telemetry across many identities;
* impossible throughput for claimed hardware;
* missing evidence;
* inconsistent dataset roots.

AI MUST NOT claim certainty that a mechanical drive was or was not used.

⸻

29. Economic Anomaly Detection

The observer SHOULD detect:

* circular transaction volume;
* self-funded fee farming;
* transaction splitting;
* subsidy farming;
* repeated low-value spam;
* reward concentration;
* treasury anomalies;
* sudden fee spikes;
* collusive inclusion patterns;
* rewards disconnected from measured useful work.

⸻

30. Correlation Engine

The observer SHOULD correlate signals across domains.

Example:

Producer concentration
+
unusual delay speed
+
same network prefix
+
same software build
=
higher combined risk

Correlation does not prove common ownership.

The explanation MUST state this limitation.

⸻

31. Incident Timeline

For serious events, the observer SHOULD build a timeline containing:

* precursor events;
* first anomaly;
* affected nodes;
* consensus impact;
* network impact;
* economic impact;
* recovery;
* unresolved questions.

⸻

32. Experiment Comparison

The AI Observer SHOULD compare protocol configurations.

Examples:

* HDD enabled versus disabled;
* one producer candidate versus three;
* equal validator weight versus bounded weight;
* different delay difficulties;
* different fee models;
* different network topologies.

Reports MUST distinguish correlation from causation.

⸻

33. Test Generation

The observer MAY recommend or generate adversarial tests.

Examples:

* increase clock skew;
* isolate one validator;
* flood delay proofs;
* simulate HDD cache substitution;
* increase producer concentration;
* reduce committee availability;
* manipulate fee demand.

Generated tests MUST require human or test-orchestrator approval before execution.

⸻

34. Automatic Response Boundaries

The AI Observer MAY automatically:

* create alerts;
* increase logging detail;
* preserve telemetry;
* request additional read-only metrics;
* generate reports;
* recommend safe-mode review.

It MUST NOT automatically:

* ban consensus participants;
* slash rewards;
* remove validators;
* change protocol parameters;
* enter all nodes into safe mode;
* reject valid blocks;
* freeze wallets.

⸻

35. Safe Operational Automation

A local operator MAY configure non-consensus responses such as:

* send notification;
* open incident ticket;
* preserve logs;
* increase snapshot frequency;
* start additional diagnostics.

These actions MUST remain outside consensus.

⸻

36. Model Registry

The Model Registry SHALL store:

ModelRecord {
    model_id
    model_version
    model_type
    training_data_summary
    feature_schema_version
    creation_date
    evaluator_version
    known_limitations[]
    approved_usage[]
    prohibited_usage[]
    artifact_hash
}

⸻

37. Model Promotion

A model MAY move through:

DEVELOPMENT
SHADOW
EVALUATION
APPROVED_FOR_OBSERVATION
DEPRECATED
RETIRED

No model may be used for high-severity operational alerts without evaluation.

⸻

38. Shadow Mode

New models SHOULD first run in shadow mode.

In shadow mode:

* outputs are recorded;
* operators are not required to act;
* false positives are measured;
* comparison with existing rules occurs;
* model drift is evaluated.

⸻

39. Model Evaluation

The evaluation harness MUST measure:

* precision;
* recall;
* false-positive rate;
* false-negative rate;
* detection latency;
* calibration;
* robustness;
* explanation quality;
* performance across scenarios;
* sensitivity to manipulated telemetry.

⸻

40. Ground Truth

Ground truth MAY come from:

* deterministic injected attacks;
* objective protocol evidence;
* labeled simulations;
* known network partitions;
* deliberate malicious-node profiles;
* human-reviewed incidents.

Unlabeled real behavior MUST not be casually treated as malicious ground truth.

⸻

41. False Positives

Every detector MUST be evaluated for false positives.

False-positive handling SHOULD include:

* review workflow;
* threshold adjustment;
* evidence inspection;
* detector version tracking;
* alert suppression with expiration;
* explanation improvement.

⸻

42. False Negatives

The system MUST acknowledge that attacks may go undetected.

The AI Observer SHALL NOT advertise complete attack detection.

The test suite MUST include attacks intentionally designed to bypass current models.

⸻

43. Model Drift

The observer MUST monitor whether:

* network behavior changes;
* hardware changes;
* software versions change;
* fee patterns change;
* attack methods evolve;
* model accuracy degrades.

Model drift SHOULD trigger reevaluation.

⸻

44. Adversarial AI Risks

The observer MUST consider:

* telemetry poisoning;
* feature manipulation;
* prompt injection;
* model extraction;
* model evasion;
* coordinated false alarms;
* alert flooding;
* training-data contamination;
* explanation manipulation;
* compromised model artifacts.

⸻

45. Telemetry Poisoning

Because nodes may submit false telemetry, the observer SHOULD:

* track data source;
* assign evidence reliability;
* compare multiple nodes;
* detect identical fabricated patterns;
* avoid trusting one self-reported source;
* prioritize cryptographic evidence.

⸻

46. Prompt Injection Protection

If language models process logs or text, the system MUST treat log content as untrusted data.

It SHOULD:

* isolate instructions from data;
* sanitize tool calls;
* prevent direct execution;
* restrict model permissions;
* require structured output;
* disallow model access to secrets.

⸻

47. Privacy

The observer MUST minimize personal data.

It SHOULD avoid unnecessary collection of:

* names;
* email addresses;
* precise physical locations;
* wallet labels;
* personal device identifiers;
* unrelated file-system information.

⸻

48. IP Address Handling

IP addresses MAY be operationally useful but are sensitive metadata.

Research exports SHOULD:

* hash or truncate addresses;
* aggregate prefixes;
* remove unnecessary raw records;
* apply retention limits.

⸻

49. Data Retention

Retention policies SHOULD be configurable for:

* raw telemetry;
* normalized features;
* alerts;
* model inputs;
* model outputs;
* incident reports.

Consensus evidence may require longer retention than ordinary telemetry.

⸻

50. Secret Exclusion

The AI Observer MUST NOT ingest:

* wallet private keys;
* recovery phrases;
* validator private keys;
* producer private keys;
* keystore passwords;
* administrative secrets.

Input filters SHOULD scan for accidental secret patterns.

⸻

51. Observer API

The observer MAY expose:

GET  /observer/v1/status
GET  /observer/v1/alerts
GET  /observer/v1/alerts/{id}
GET  /observer/v1/models
GET  /observer/v1/reports
POST /observer/v1/analysis
POST /observer/v1/evaluation

Write operations MUST affect only observer data.

⸻

52. Access Control

Observer administration MUST require authentication.

Public read-only reports MAY be exposed separately.

The observer MUST NOT share credentials with node administration.

⸻

53. Report Types

The observer SHOULD generate:

DAILY_NETWORK_REPORT
CONSENSUS_HEALTH_REPORT
SECURITY_INCIDENT_REPORT
HDD_EXPERIMENT_REPORT
DELAY_ENGINE_REPORT
ECONOMIC_CONCENTRATION_REPORT
NETWORK_DIVERSITY_REPORT
MODEL_EVALUATION_REPORT
ATTACK_SIMULATION_REPORT
PROTOCOL_COMPARISON_REPORT

⸻

54. Report Structure

ObserverReport {
    report_id
    report_type
    report_version
    period
    data_sources[]
    model_versions[]
    executive_summary
    key_findings[]
    evidence[]
    uncertainties[]
    recommended_actions[]
    appendices[]
}

⸻

55. Explanation Requirements

A high-severity alert SHOULD answer:

* What happened?
* When did it begin?
* Which nodes or roles were affected?
* What evidence is objective?
* What evidence is inferred?
* What protocol risk exists?
* What benign explanations remain possible?
* What test or review should occur next?

⸻

56. Confidence Language

The observer SHOULD use calibrated language.

Recommended:

Observed
Verified
Strongly indicated
Likely
Possible
Uncertain
Insufficient evidence

It SHOULD avoid unsupported certainty.

⸻

57. AI Observer States

STARTING
LOADING_MODELS
INGESTING
READY
DEGRADED
EVALUATING
REPORTING
PAUSED
FAILED
STOPPED

The node network MUST remain functional in every observer state.

⸻

58. Failure Behavior

If the observer fails:

* consensus continues;
* nodes continue operating;
* telemetry MAY queue within limits;
* operators receive health warnings;
* no block becomes invalid;
* no reward changes;
* no participant is penalized.

⸻

59. Backpressure

The observer MUST not overload nodes.

Telemetry ingestion SHOULD use:

* bounded queues;
* batch processing;
* rate limits;
* sampling;
* retention limits.

Nodes SHOULD drop optional observer telemetry before affecting consensus performance.

⸻

60. Performance Requirements

The observer SHOULD support:

* near-real-time critical alerts;
* batch analysis;
* historical analysis;
* multi-node correlation;
* experiment comparison.

Initial targets MAY include:

critical rule-based alert latency < 5 seconds
standard analysis latency < 60 seconds
batch report generation < configurable limit

These are operational targets only.

⸻

61. Deterministic Rule Tests

Rule-based detectors MUST have deterministic tests.

Example:

Given:
two valid conflicting attestations from one validator
Expected:
VALIDATOR_EQUIVOCATION alert
severity = critical or high
objective evidence = true

⸻

62. Model Test Datasets

The repository SHOULD contain:

* normal network traces;
* partition traces;
* eclipse simulations;
* spam attacks;
* producer concentration scenarios;
* validator collusion scenarios;
* HDD substitution scenarios;
* delay anomaly scenarios;
* fee manipulation scenarios;
* reward farming scenarios.

Sensitive data MUST be removed.

⸻

63. Required Observer Tests

The automated suite MUST include:

1. telemetry ingestion;
2. malformed event rejection;
3. missing field handling;
4. evidence-class preservation;
5. rule-based equivocation detection;
6. finality conflict detection;
7. producer concentration detection;
8. validator concentration detection;
9. partition detection;
10. eclipse suspicion;
11. transaction spam detection;
12. fee spike detection;
13. reward farming detection;
14. delay anomaly detection;
15. HDD anomaly detection;
16. energy anomaly detection;
17. model unavailable;
18. model timeout;
19. model malformed output;
20. model version mismatch;
21. alert deduplication;
22. alert escalation;
23. false-positive review;
24. report generation;
25. privacy filtering;
26. secret-pattern filtering;
27. telemetry poisoning;
28. prompt injection attempt;
29. alert flooding;
30. observer shutdown;
31. observer restart;
32. node operation while observer is offline;
33. model drift evaluation;
34. shadow-mode comparison;
35. simulated attack test recommendation.

⸻

64. Security Tests

The test team MUST attempt:

* inserting fake telemetry;
* forging device claims;
* submitting impossible metrics;
* creating coordinated false alerts;
* suppressing telemetry;
* overwhelming alert queues;
* injecting instructions into logs;
* manipulating feature distributions;
* causing model disagreement;
* replacing model files;
* using stale models;
* accessing observer APIs without authorization;
* extracting sensitive telemetry;
* tricking the observer into recommending consensus changes.

⸻

65. Minimum Prototype Acceptance

The AI Observer prototype is complete when:

1. it ingests structured telemetry from multiple nodes;
2. it preserves evidence classes;
3. it detects objective validator equivocation;
4. it detects objective producer equivocation;
5. it alerts on conflicting finalization certificates;
6. it calculates producer concentration;
7. it calculates validator concentration;
8. it detects a simulated partition;
9. it flags abnormal delay behavior;
10. it flags suspicious HDD telemetry;
11. it produces a structured risk score;
12. it explains supporting evidence;
13. it records model and rule versions;
14. it generates an incident report;
15. it remains read-only;
16. it has no access to private keys;
17. disabling it does not affect consensus;
18. model failure does not affect ledger state;
19. false positives can be marked and reviewed;
20. simulated data is clearly labeled.

⸻

66. Private Testnet Acceptance

Before the observer is treated as operationally useful, it MUST demonstrate:

* ingestion from at least seven nodes;
* stable operation through 1,000 finalized blocks;
* successful detection of injected objective violations;
* measured false-positive rate;
* measured false-negative rate on labeled attacks;
* correct distinction between objective and inferred evidence;
* no consensus impact when offline;
* privacy-filter validation;
* model-version reproducibility;
* report usefulness in post-incident review.

⸻

67. Criteria for Expanding AI Role

AI MAY receive broader observational responsibility only if:

* detection quality is measured;
* false positives are manageable;
* explanations are useful;
* privacy is protected;
* model drift is monitored;
* permission boundaries remain strict;
* human review remains available.

Broader observation does not imply consensus authority.

⸻

68. Criteria for Restricting AI Role

The AI role SHOULD be reduced if:

* alerts are mostly noise;
* explanations are unreliable;
* telemetry costs harm node performance;
* models are easy to manipulate;
* privacy costs exceed value;
* rule-based methods perform better;
* operational teams over-trust model output.

⸻

69. Criteria for Removing AI Components

An AI model or subsystem SHOULD be removed if:

* it provides no measurable value;
* it repeatedly causes harmful false alarms;
* it cannot be evaluated;
* it requires sensitive data without justification;
* it becomes a hidden source of authority;
* it creates unacceptable attack surface.

Removing an AI component is a valid engineering outcome.

⸻

70. Known Limitations

Q1 AI Observer v0.1 cannot guarantee:

* detection of all attacks;
* correct attribution;
* proof of physical hardware use;
* proof of common ownership;
* legal intent;
* absence of false positives;
* absence of false negatives;
* unbiased models;
* permanent model accuracy;
* autonomous safe governance.

The observer is an analytical assistant, not an infallible guardian.

⸻

71. Open Decisions

The following remain unresolved:

1. initial model architecture;
2. exact feature schema;
3. alert thresholds;
4. risk-score formula;
5. telemetry backend;
6. model hosting method;
7. local versus remote inference;
8. privacy-retention periods;
9. IP anonymization method;
10. model evaluation datasets;
11. whether language models are used for explanations;
12. exact prompt-injection protections;
13. alert escalation workflow;
14. model approval process;
15. report cadence;
16. operator notification channels;
17. whether observer recommendations can trigger automated diagnostics;
18. long-term model governance;
19. whether public testnet telemetry is opt-in;
20. whether multiple independent observer implementations are encouraged.

All decisions MUST be recorded in OPEN_DECISIONS.md.

⸻

72. Codex Implementation Rules

Codex MUST:

1. implement the AI Observer as a separate service or process;
2. use read-only data access;
3. keep consensus functional without the observer;
4. version models, features, rules, and alerts;
5. preserve evidence classes;
6. distinguish objective evidence from inference;
7. provide structured risk scores;
8. provide explanations and limitations;
9. implement privacy filtering;
10. scan for accidental secrets;
11. use bounded queues and rate limits;
12. provide deterministic tests for rule-based detectors;
13. support shadow mode;
14. support model evaluation;
15. record false positives and false negatives;
16. label simulated data;
17. isolate model adapters;
18. restrict observer API permissions;
19. keep all recommendations non-binding;
20. expose health and model status.

Codex MUST NOT:

* give the observer ledger write access;
* give the observer consensus signing keys;
* let AI cast validator votes;
* let AI reject mathematically valid blocks;
* let AI create rewards or penalties;
* let AI modify participant weight automatically;
* feed private keys into models;
* treat IP address as proof of identity;
* treat HDD telemetry as physical truth;
* hide model uncertainty;
* present AI output as guaranteed fact;
* stop consensus because model inference failed.

⸻

73. Final AI Principle

The Q1 AI Observer MUST preserve this principle:

Mathematics determines validity.
Signatures establish authorization.
Consensus establishes finality.
AI observes behavior around those rules.

AI may notice what humans miss.
It may connect signals that appear unrelated.
It may warn before a weakness becomes a failure.
It may explain, compare, and recommend.

But it must never quietly become the owner of truth.

Q1 succeeds at the AI layer when the observer can say:

* what it observed;
* what it verified;
* what it inferred;
* how confident it is;
* what uncertainty remains;
* and what should be investigated next

without holding keys, changing balances, controlling votes, or replacing deterministic consensus.