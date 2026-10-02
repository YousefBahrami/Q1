Q1 HDD Laboratory Module Specification

07_HDD_LAB_MODULE.md

Project: Q1 Experimental Distributed Ledger
Protocol Version: 0.1
Document Version: 0.1.0
Status: Draft for Engineering Review
Classification: Experimental — Not for Production or Financial Use

⸻

1. Purpose

This document defines the Q1 HDD Laboratory Module.

The HDD Laboratory Module is an experimental subsystem designed to test whether widely available mechanical hard drives can contribute measurable and useful properties to Q1 block production without becoming a false source of trust, an environmental burden, or a barrier to public participation.

The module SHALL investigate whether HDD-based work can provide any practical benefit through:

* challenge-bound read and write operations;
* measurable physical latency;
* limited resource commitment;
* storage contribution;
* anti-precomputation behavior;
* reduced advantage from massive parallel computation;
* energy characteristics different from unrestricted proof-of-work;
* accessible participation using common consumer hardware.

This document does not assume that HDD participation is secure or useful.

Its purpose is to define how that assumption will be tested.

⸻

2. Research Question

The central research question is:

Can an ordinary mechanical HDD contribute a reproducible, challenge-bound, economically meaningful cost to block production without becoming easy to simulate, outsource, cache, replay, or centralize?

The module MUST be designed to allow an honest answer of:

* yes;
* partially;
* no;
* or only under restricted conditions.

No architectural decision may force HDD to remain part of Q1 if experiments do not justify it.

⸻

3. Non-Authority Principle

The HDD module is not a source of final truth.

For Q1 v0.1:

* HDD telemetry MUST NOT independently validate a block;
* HDD output MUST NOT independently create producer eligibility;
* HDD model or serial number MUST NOT be treated as identity proof;
* operating-system disk reports MUST NOT be trusted as physical truth;
* AI analysis MUST NOT convert HDD telemetry into consensus validity;
* a forged HDD report MUST NOT be sufficient to finalize a block;
* the base network MUST function with the HDD module disabled.

Recommended initial mode:

HDD_MODE = TELEMETRY_ONLY

⸻

4. Experimental Objectives

The HDD module SHALL measure:

1. whether challenge-bound disk operations are reproducible;
2. whether previous results can be replayed;
3. whether cached data defeats the intended delay;
4. whether SSD, RAM disk, virtual disk, or remote storage can imitate HDD behavior;
5. whether ordinary consumer HDDs can participate reliably;
6. how device age and fragmentation affect performance;
7. energy use per workload;
8. thermal and mechanical effects;
9. read and write amplification;
10. expected device wear;
11. producer concentration caused by hardware differences;
12. whether HDD contribution adds security beyond mathematical delay;
13. whether the workload creates useful storage;
14. whether the workload merely creates waste;
15. whether verification is cheaper than generation;
16. whether the module can be safely removed.

⸻

5. Module Architecture

The HDD Laboratory Module SHALL contain the following components:

Q1 HDD Lab Module
├── Device Discovery
├── Device Qualification
├── Dataset Manager
├── Challenge Generator
├── Workload Planner
├── Workload Executor
├── Commitment Builder
├── Evidence Collector
├── Result Verifier
├── Telemetry Collector
├── Energy Estimator
├── Wear Estimator
├── Simulator
├── Adversarial Adapter
└── Benchmark Reporter

Each component MUST be independently testable.

⸻

6. Plugin Interface

The HDD module MUST implement a stable plugin interface.

HDDPlugin {
    plugin_id()
    plugin_version()
    capabilities()
    discover_devices()
    inspect_device(device_id)
    qualify_device(device_id, policy)
    prepare_dataset(device_id, dataset_config)
    derive_workload(challenge, dataset_metadata, workload_config)
    execute_workload(device_id, workload)
    build_commitment(challenge, workload_result)
    verify_commitment(challenge, commitment, evidence, policy)
    collect_metrics()
    estimate_energy()
    estimate_wear()
    cancel()
    cleanup()
}

The Q1 node MUST communicate with the HDD module only through this interface.

⸻

7. Operating Modes

The module SHALL support distinct operating modes.

7.1 Disabled

HDD_MODE = DISABLED

Behavior:

* no HDD discovery;
* no HDD workload;
* no HDD commitment;
* no HDD telemetry requirement;
* consensus remains fully operational.

⸻

7.2 Telemetry Only

HDD_MODE = TELEMETRY_ONLY

Behavior:

* HDD workload may be executed;
* results are recorded;
* block validity does not depend on HDD success;
* no consensus reward is granted solely from HDD output;
* failures do not invalidate otherwise valid blocks.

This SHALL be the initial recommended mode.

⸻

7.3 Delay Input

HDD_MODE = DELAY_INPUT

Behavior:

* HDD commitment contributes to the mathematical delay input;
* a producer configured for this mode must produce valid HDD evidence;
* mathematical delay proof remains mandatory;
* HDD evidence alone remains insufficient.

This mode is experimental and MUST NOT be enabled before telemetry-only results are reviewed.

⸻

7.4 Reward Modifier

HDD_MODE = REWARD_MODIFIER

Behavior:

* valid HDD participation may affect experimental reward;
* reward effect must be capped;
* invalid or absent HDD work cannot invalidate base consensus unless separately configured;
* reward rules must be deterministic.

⸻

7.5 Eligibility Modifier

HDD_MODE = ELIGIBILITY_MODIFIER

Behavior:

* HDD contribution affects producer eligibility weight;
* influence must be bounded;
* Sybil and capacity-centralization effects must be measured;
* this mode requires separate protocol approval.

⸻

8. Device Classes

The module SHALL distinguish at least:

MECHANICAL_HDD
SOLID_STATE_DRIVE
RAM_DISK
VIRTUAL_DISK
NETWORK_BLOCK_DEVICE
UNKNOWN_DEVICE

Device classification is observational and potentially forgeable.

It MUST NOT be treated as cryptographic proof of physical hardware.

⸻

9. Supported Initial Hardware

The first physical test phase SHOULD support:

* SATA mechanical HDD;
* USB mechanical HDD;
* internal laptop HDD;
* external consumer HDD;
* desktop 3.5-inch HDD;
* 2.5-inch HDD.

The test suite SHOULD also include:

* SSD;
* NVMe SSD;
* RAM disk;
* virtual disk image;
* network-attached storage;
* cloud block storage.

These alternatives are required to determine whether the workload actually distinguishes mechanical behavior.

⸻

10. Device Discovery

The module MAY collect:

DeviceMetadata {
    operating_system_identifier
    reported_vendor
    reported_model
    reported_serial_optional
    reported_capacity
    interface_type
    rotational_flag
    reported_rpm_optional
    logical_sector_size
    physical_sector_size
    filesystem_type
    mount_point
    removable_flag
    system_disk_flag
    health_data_optional
}

All fields are untrusted telemetry.

A producer MUST NOT gain consensus authority merely by reporting them.

⸻

11. Device Safety

The HDD module MUST default to safe operation.

It MUST NOT:

* overwrite arbitrary user files;
* write outside the configured test directory;
* format a disk;
* modify partition tables;
* bypass operating-system permissions;
* use raw-device access without explicit operator configuration;
* run destructive workloads without warning;
* consume all available storage;
* write indefinitely;
* conceal expected wear.

The test directory MUST be explicit.

Recommended:

q1-hdd-lab/

⸻

12. Device Qualification

Before use, the module SHOULD evaluate:

* writable test directory;
* available capacity;
* basic read and write success;
* minimum free space;
* filesystem compatibility;
* configured safety limits;
* measured baseline latency;
* measured baseline throughput;
* device health warnings where available;
* whether the device hosts critical system data.

A device failing safety checks MUST be rejected or restricted to read-only tests.

⸻

13. Dataset Model

The HDD module MAY create a deterministic local dataset used for challenge-bound access.

HDDDataset {
    dataset_version
    dataset_id
    owner_participant_id
    device_id_local
    dataset_size
    chunk_size
    chunk_count
    generation_seed
    dataset_root
    creation_height
    expiration_height_optional
}

⸻

14. Dataset Requirements

Q1-HDD-001 — Deterministic generation

Dataset contents MUST be deterministically generated from:

* dataset generation seed;
* participant identity;
* chunk index;
* protocol domain separator.

Conceptually:

chunk_i =
    EXPAND(
        HASH(
            DOMAIN_Q1_HDD_DATASET
            || generation_seed
            || participant_id
            || encode_u64(i)
        ),
        chunk_size
    )

⸻

Q1-HDD-002 — Dataset commitment

The module MUST calculate a deterministic dataset commitment.

Recommended:

dataset_root = MerkleRoot(chunk_hashes)

or another authenticated commitment.

⸻

Q1-HDD-003 — No secret dataset

Security MUST NOT depend on dataset contents remaining secret.

⸻

Q1-HDD-004 — Configurable size

Dataset size MUST be configurable.

Recommended initial experiments:

1 GB
4 GB
16 GB
64 GB

Smaller sizes MAY be used for automated tests.

⸻

Q1-HDD-005 — Capacity fairness

Raw capacity MUST NOT automatically translate into unlimited consensus influence.

⸻

15. Dataset Lifecycle

A dataset MAY pass through:

NOT_CREATED
CREATING
READY
VERIFYING
ACTIVE
DEGRADED
CORRUPTED
EXPIRED
DELETING
DELETED

Dataset corruption MUST be detected through commitment checks.

⸻

16. Challenge Binding

Every HDD workload MUST derive from the current consensus challenge.

Conceptually:

HDDChallenge =
    HASH(
        DOMAIN_Q1_HDD_CHALLENGE
        || delay_challenge
        || participant_id
        || dataset_id
        || workload_version
        || workload_parameters_hash
    )

A workload result MUST NOT be valid for another:

* chain;
* height;
* round;
* producer;
* dataset;
* workload version;
* challenge.

⸻

17. Workload Types

The module SHALL support multiple workload types for comparison.

17.1 Sequential Read

Reads a challenge-selected contiguous region.

Purpose:

* throughput measurement;
* caching analysis;
* energy measurement.

⸻

17.2 Random Read

Reads challenge-selected chunks from non-contiguous offsets.

Purpose:

* seek-latency measurement;
* mechanical behavior analysis;
* SSD and RAM-disk comparison.

⸻

17.3 Sequential Write

Writes challenge-derived data into a dedicated temporary region.

Purpose:

* write-energy measurement;
* persistence testing;
* wear analysis.

This workload MUST be bounded and disabled by default.

⸻

17.4 Random Write

Writes challenge-derived small blocks to selected offsets within a dedicated disposable dataset.

Purpose:

* seek behavior;
* write amplification;
* wear measurement.

This workload MUST require explicit operator opt-in.

⸻

17.5 Read-Transform-Commit

Reads selected chunks, combines them with the challenge, and computes a commitment.

Conceptually:

response_i =
    HASH(
        challenge
        || chunk_index_i
        || chunk_bytes_i
    )

Then:

commitment =
    MerkleRoot(response_hashes)

⸻

17.6 Seek Pattern

Executes a challenge-derived sequence of offsets intended to create mechanical seek operations.

Purpose:

* physical latency study;
* simulation resistance study.

⸻

17.7 Mixed Workload

Combines:

* random reads;
* sequential reads;
* limited writes;
* mathematical transformation.

This MAY become the primary experimental workload after benchmarking.

⸻

18. Initial Recommended Workload

The first non-destructive physical experiment SHOULD use:

RANDOM_READ_TRANSFORM_COMMIT_V1

Properties:

* read-only;
* challenge-bound;
* uses an existing deterministic dataset;
* selects many pseudo-random chunk positions;
* reads full chunks;
* hashes chunk content with challenge and index;
* commits responses into a deterministic root;
* records latency and throughput;
* avoids intentional write wear.

⸻

19. Initial Workload Definition

Given:

dataset_chunk_count = C
sample_count = S
HDDChallenge = Q

For each sample index j:

selector_j =
    HASH(
        DOMAIN_Q1_HDD_SELECTOR
        || Q
        || encode_u64(j)
    )

Selected chunk:

chunk_index_j =
    selector_j mod C

Response:

response_j =
    HASH(
        DOMAIN_Q1_HDD_RESPONSE
        || Q
        || encode_u64(j)
        || encode_u64(chunk_index_j)
        || chunk_bytes
    )

Final commitment:

HDDCommitment =
    MerkleRoot(response_0 ... response_(S-1))

⸻

20. Duplicate Chunk Selection

If the same chunk index is selected more than once, the protocol MUST define whether:

* duplicates are permitted;
* or selection continues until unique indices are obtained.

Recommended first implementation:

duplicates are permitted

Rationale:

* simpler deterministic behavior;
* avoids variable loop complexity;
* preserves exact challenge mapping.

Later versions MAY test unique selection.

⸻

21. Sample Count and Chunk Size

Initial experimental values MAY include:

chunk_size:
    64 KB
    256 KB
    1 MB
    4 MB
sample_count:
    32
    128
    512
    2048

The test suite MUST evaluate:

* workload duration;
* proof size;
* cache effects;
* device variance;
* energy;
* mechanical wear;
* simulation advantage.

⸻

22. Cache Resistance

Operating systems and devices may cache disk data.

The module MUST investigate:

* warm-cache performance;
* cold-cache performance;
* repeated challenge behavior;
* filesystem cache effects;
* controller cache effects;
* application-level cache;
* dataset size relative to RAM;
* direct I/O where supported;
* cache flushing where safe and permitted.

The module MUST NOT claim physical disk access merely because an application issued a read call.

⸻

23. Cache Experiment Modes

The benchmark harness SHOULD support:

CACHE_MODE = DEFAULT_OS
CACHE_MODE = COLD_START
CACHE_MODE = DROP_APPLICATION_CACHE
CACHE_MODE = DIRECT_IO_IF_SUPPORTED
CACHE_MODE = REPEATED_CHALLENGE
CACHE_MODE = UNIQUE_CHALLENGE

Some operating systems may restrict cache control.

All limitations MUST be recorded.

⸻

24. Commitment Structure

HDDCommitmentRecord {
    commitment_version
    plugin_id
    plugin_version
    workload_id
    workload_version
    chain_id
    block_height
    round_number
    producer_id
    dataset_id
    dataset_root
    challenge_hash
    chunk_size
    sample_count
    response_root
    evidence_hash_optional
}

⸻

25. Evidence Structure

Evidence MAY include:

HDDEvidence {
    selected_chunk_indices
    selected_chunk_hashes_optional
    response_hashes_optional
    Merkle authentication paths
    dataset proofs
    workload_parameters
}

The exact evidence mode depends on verification strategy.

⸻

26. Verification Strategies

The module SHALL test several verification strategies.

26.1 Full Data Verification

The verifier independently holds the full dataset and recomputes all selected reads.

Advantage:

* strong deterministic verification.

Disadvantage:

* every validator must store the dataset;
* network duplication;
* poor scalability.

⸻

26.2 Proof of Dataset Membership

The producer provides selected chunk data and Merkle proofs against the dataset root.

Verifier checks:

* chunk membership;
* challenge-derived selection;
* response hashes;
* final commitment.

Advantage:

* verifier need not store full dataset.

Disadvantage:

* does not prove data came from HDD at challenge time;
* producer may keep data in faster media;
* proof size may be large.

⸻

26.3 Remote Challenge Response

The validator requests challenge-selected data after round start.

Advantage:

* freshness.

Disadvantage:

* network timing dependence;
* potential centralization;
* still does not prove mechanical storage.

⸻

26.4 Sampled Audit

Validators verify a subset of responses.

Advantage:

* lower verification cost.

Disadvantage:

* probabilistic;
* incomplete;
* potentially gameable.

Sampled audits MUST NOT be misrepresented as full physical proof.

⸻

27. Physical Truth Limitation

A valid commitment can prove only that the producer knew or obtained challenge-relevant data.

It does not prove that:

* the data was read from a mechanical HDD;
* the data was stored locally;
* the disk had a particular model;
* the producer did not use RAM or SSD;
* the producer did not outsource the operation;
* the reported latency is genuine;
* the operating system report is honest.

This limitation is fundamental and MUST remain explicit.

⸻

28. Simulator

The HDD module MUST include a simulator capable of producing syntactically valid workload results without physical HDD use.

Simulation modes SHOULD include:

SIMULATED_HDD
SSD_SUBSTITUTION
RAM_DISK_SUBSTITUTION
VIRTUAL_DISK_SUBSTITUTION
REMOTE_STORAGE_SUBSTITUTION
CACHED_DATASET
PRECOMPUTED_DATASET
FAKE_TELEMETRY
MANIPULATED_LATENCY

The simulator is necessary to determine what the verifier can and cannot distinguish.

⸻

29. Adversarial Adapter

The adversarial adapter SHALL allow controlled corruption of:

* device metadata;
* workload duration;
* selected indices;
* response data;
* commitment root;
* dataset root;
* evidence paths;
* energy estimates;
* reported bytes;
* reported errors;
* timestamps;
* device classification.

Every corrupted result MUST be tested against verification rules.

⸻

30. HDD Result States

An HDD execution MAY have:

NOT_STARTED
DATASET_NOT_READY
CHALLENGE_DERIVED
RUNNING
READING
WRITING
HASHING
COMMITTING
COMPLETED
SELF_VERIFIED
FAILED
CANCELLED
EXPIRED
DEVICE_LOST
DATASET_CORRUPTED

⸻

31. Cancellation

The module MUST support cancellation when:

* another block finalizes;
* producer window expires;
* round advances;
* node enters safe mode;
* device disconnects;
* operator stops the node;
* workload exceeds safety limits.

Cancelled work MUST NOT produce a valid commitment.

⸻

32. Device Loss

If the device becomes unavailable:

* telemetry-only mode MUST allow base consensus to continue;
* delay-input mode MUST cause the producer attempt to fail safely;
* partial results MUST not be accepted;
* the failure reason MUST be logged;
* filesystem integrity SHOULD be checked on restart.

⸻

33. Dataset Corruption

The module MUST detect dataset corruption through:

* chunk hash verification;
* dataset-root verification;
* periodic audit;
* read failure;
* inconsistent metadata.

A corrupted dataset MUST NOT be used to create a valid commitment.

⸻

34. Remote Storage

The protocol MUST assume that a participant may use remote or cloud storage.

Remote storage tests MUST measure:

* latency;
* bandwidth;
* centralization risk;
* outage risk;
* outsourcing economics;
* evidence indistinguishability.

Q1 MUST NOT claim local ownership merely from successful response.

⸻

35. Multiple Devices

A participant MAY connect multiple devices for testing.

The module MUST record each device separately.

Raw device count MUST NOT automatically equal consensus weight.

Experiments MAY compare:

* one large drive;
* several small drives;
* mixed HDD and SSD;
* RAID;
* JBOD;
* network storage;
* sharded datasets.

⸻

36. RAID and Aggregation

The module MUST test whether RAID or striping creates disproportionate advantage.

Test configurations SHOULD include:

single HDD
RAID 0
RAID 1
RAID 5 or equivalent
multiple independent HDDs

The protocol MUST not assume one logical device equals one physical device.

⸻

37. Dataset Duplication

A participant may copy one dataset to multiple devices.

The module MUST determine whether duplicated datasets:

* increase throughput;
* allow parallel challenge response;
* create Sybil-like amplification;
* improve reliability;
* undermine intended fairness.

Dataset duplication MUST NOT be treated as unique contribution unless protocol rules explicitly justify it.

⸻

38. Energy Measurement

The module MUST measure or estimate:

* idle device power;
* spin-up energy;
* active read power;
* active write power;
* seek-intensive power;
* energy per workload;
* energy per byte read;
* energy per commitment;
* duplicated candidate energy;
* host CPU energy used for hashing;
* external enclosure energy where measurable.

Measurement method MUST be recorded.

⸻

39. Wear Measurement

The module MUST estimate:

* bytes read;
* bytes written;
* workload count;
* seek count where estimable;
* spin-up cycles;
* operating duration;
* temperature;
* health indicators where available;
* projected workload per day;
* projected annual write volume.

Q1 MUST not claim “green” behavior while hiding device wear.

⸻

40. Environmental Assessment

The research report SHOULD compare:

* energy consumption;
* hardware lifetime;
* e-waste implications;
* reuse of existing drives;
* requirement for new device purchases;
* cooling requirements;
* storage usefulness;
* discarded work.

The evaluation MUST consider total system cost, not electricity alone.

⸻

41. Accessibility Assessment

The module MUST examine whether ordinary users can participate using:

* old laptop HDDs;
* external USB drives;
* common desktop drives;
* low-capacity drives;
* low-bandwidth internet;
* low-end computers.

The study MUST record:

* setup complexity;
* minimum usable capacity;
* operating-system restrictions;
* workload duration;
* failure rate;
* user safety risks.

⸻

42. Fairness Metrics

The module SHOULD calculate:

* commitments per device class;
* energy per successful commitment;
* cost per successful commitment;
* success rate by device class;
* performance concentration;
* reward concentration under simulated rules;
* capacity concentration;
* operator concentration;
* speed advantage of SSD and RAM;
* advantage from multiple devices;
* advantage from cloud storage.

⸻

43. Security Benefit Metrics

The HDD module may be considered useful only if it contributes measurable benefit such as:

* higher attack cost;
* reduced computation centralization;
* reduced wasted energy;
* improved storage availability;
* improved challenge diversity;
* increased difficulty of precomputation;
* increased resilience through common hardware.

Perceived physicality alone is not a security benefit.

⸻

44. Useful Storage Question

The module MUST distinguish:

Artificial test data

Data created only to prove storage or access.

Network-useful data

Data that contributes to:

* ledger history;
* snapshots;
* block availability;
* archival redundancy;
* recovery;
* public data distribution.

Later experiments SHOULD test whether HDD work can be redirected toward useful network storage.

Q1 v0.1 does not assume that its initial dataset is useful.

⸻

45. Potential Useful Storage Modes

Future modes MAY include storing:

* finalized block ranges;
* state snapshots;
* erasure-coded ledger fragments;
* historical transaction archives;
* testnet recovery data;
* public protocol files.

Any useful-storage design MUST preserve:

* deterministic verification;
* privacy boundaries;
* data-availability guarantees;
* resistance to selective storage;
* versioning.

⸻

46. Reward Eligibility

In telemetry-only mode:

HDD reward = 0

unless a separate test-reward configuration is explicitly enabled.

In later reward experiments, a valid HDD reward MUST depend on:

* accepted workload;
* accepted commitment;
* configured reward mode;
* finalized source block;
* bounded per-participant limits.

Reward MUST NOT be granted solely for reporting a device.

⸻

47. Anti-Centralization Limits

If HDD affects reward or eligibility, the protocol MUST test bounded influence mechanisms such as:

* square-root weighting;
* capped capacity contribution;
* diminishing returns;
* per-participant saturation;
* lottery among qualified devices;
* rotating workload eligibility;
* cooldowns;
* useful-data diversity.

No mechanism is approved in this document.

⸻

48. HDD and Producer Selection

The initial consensus implementation SHOULD ignore HDD in producer selection.

Recommended:

ExperimentalContributionFactor = 1

for all participants regardless of HDD.

Only after testing may an HDD-derived bounded factor be introduced.

⸻

49. HDD and Delay Engine

In telemetry-only mode:

DelayInput = mathematical challenge only

In delay-input mode:

DelayInput =
    HASH(
        delay_challenge
        || accepted_hdd_commitment
    )

The delay proof MUST still be independently valid.

⸻

50. Telemetry Fields

The module SHOULD record:

plugin_id
plugin_version
workload_id
workload_version
node_id
participant_id
device_class
reported_device_metadata
dataset_id
dataset_size
chunk_size
sample_count
challenge_hash
start_time_local
completion_time_local
duration
bytes_requested
bytes_read
bytes_written
read_errors
write_errors
average_latency
median_latency
p95_latency
p99_latency
throughput
cpu_usage
memory_usage
estimated_energy
temperature_optional
cache_mode
filesystem
operating_system
commitment_hash
verification_result
failure_code

Private or identifying fields SHOULD be minimized in exported research data.

⸻

51. Stable Failure Codes

Recommended:

HDD_DISABLED
HDD_DEVICE_NOT_FOUND
HDD_DEVICE_UNSAFE
HDD_PERMISSION_DENIED
HDD_INSUFFICIENT_SPACE
HDD_UNSUPPORTED_FILESYSTEM
HDD_DATASET_NOT_FOUND
HDD_DATASET_CORRUPTED
HDD_DATASET_ROOT_MISMATCH
HDD_INVALID_CHALLENGE
HDD_WORKLOAD_UNSUPPORTED
HDD_READ_FAILED
HDD_WRITE_FAILED
HDD_TIMEOUT
HDD_CANCELLED
HDD_DEVICE_DISCONNECTED
HDD_COMMITMENT_MISMATCH
HDD_EVIDENCE_INVALID
HDD_PROOF_TOO_LARGE
HDD_RESOURCE_LIMIT
HDD_CACHE_POLICY_UNAVAILABLE
HDD_TELEMETRY_INCOMPLETE
HDD_INTERNAL_ERROR

⸻

52. Resource Limits

Configuration MUST define:

maximum_dataset_size
minimum_free_space
maximum_bytes_read_per_workload
maximum_bytes_written_per_workload
maximum_sample_count
maximum_chunk_size
maximum_workload_duration
maximum_evidence_size
maximum_concurrent_workloads
minimum_device_health_policy_optional

The module MUST reject unsafe parameters.

⸻

53. Write Safety Limits

Write workloads MUST be disabled by default.

When enabled, configuration MUST include:

maximum_bytes_written_per_round
maximum_bytes_written_per_day
maximum_test_directory_size
allow_raw_device_access = false
allow_system_disk = false

The module MUST provide a prominent wear warning.

⸻

54. Dataset Cleanup

The module MUST support:

* safe dataset deletion;
* temporary-file cleanup;
* interrupted-operation recovery;
* verification before deletion;
* configurable retention;
* disk-space reporting.

Cleanup MUST be restricted to the configured Q1 test directory.

⸻

55. Cross-Platform Requirements

The initial implementation SHOULD support:

* Linux;
* macOS;
* Windows where practical.

Platform-specific code MUST be isolated behind adapters.

The core workload derivation and commitment calculation MUST remain identical across platforms.

⸻

56. Benchmark Tool

Suggested commands:

q1-hdd discover
q1-hdd inspect
q1-hdd qualify
q1-hdd dataset create
q1-hdd dataset verify
q1-hdd workload run
q1-hdd workload verify
q1-hdd benchmark
q1-hdd compare
q1-hdd simulate
q1-hdd attack
q1-hdd report
q1-hdd cleanup

⸻

57. Deterministic Test Vectors

The repository MUST include deterministic vectors for:

* dataset generation;
* chunk generation;
* dataset root;
* challenge derivation;
* selected chunk indices;
* response hashes;
* commitment root;
* evidence verification.

Physical timing is not deterministic and SHALL not be part of consensus test vectors.

⸻

58. Required Functional Tests

The automated suite MUST include:

1. device discovery;
2. no device found;
3. safe directory validation;
4. dataset creation;
5. dataset verification;
6. corrupted dataset;
7. deterministic challenge;
8. deterministic chunk selection;
9. valid random-read workload;
10. wrong selected index;
11. altered chunk data;
12. wrong dataset root;
13. wrong challenge;
14. replayed commitment;
15. wrong producer;
16. wrong round;
17. wrong workload version;
18. commitment mismatch;
19. evidence truncation;
20. excessive evidence size;
21. cancellation;
22. device disconnect;
23. insufficient storage;
24. read permission failure;
25. HDD disabled;
26. telemetry-only behavior;
27. delay-input behavior;
28. simulator behavior;
29. fake telemetry;
30. cache-mode comparison;
31. SSD substitution;
32. RAM-disk substitution;
33. virtual-disk substitution;
34. remote-storage substitution;
35. repeated challenge;
36. unique challenge;
37. multiple-device test;
38. dataset duplication;
39. RAID benchmark;
40. write-limit enforcement.

⸻

59. Required Adversarial Tests

The test team MUST attempt:

* precomputing responses;
* caching all dataset chunks in RAM;
* serving data from SSD;
* falsifying device model;
* falsifying rotational status;
* falsifying latency;
* using a virtual disk;
* replaying old commitments;
* outsourcing challenge response;
* duplicating one dataset across many identities;
* using many virtual devices;
* manipulating filesystem cache;
* truncating evidence;
* forging Merkle paths;
* changing dataset after commitment;
* using stale dataset roots;
* returning only selected data without persistent storage;
* delaying responses strategically;
* claiming device failure to manipulate fallback;
* overwhelming validators with proof size;
* using HDD results from another chain;
* using one workload result for multiple candidates.

⸻

60. Minimum Prototype Acceptance

The HDD module prototype is complete when:

1. it discovers and inspects a physical HDD;
2. it creates a deterministic dataset;
3. it calculates a reproducible dataset root;
4. it derives a challenge-bound random-read workload;
5. it executes the workload safely;
6. it produces a deterministic commitment;
7. another process verifies the commitment structure;
8. changing challenge invalidates the commitment;
9. replay across rounds fails;
10. the same workload runs on SSD and RAM disk for comparison;
11. telemetry captures timing, bytes, and resource use;
12. the node operates normally with HDD disabled;
13. HDD failure does not corrupt the ledger;
14. simulator results can be injected;
15. write workloads remain disabled by default;
16. all files remain inside the test directory;
17. cancellation works;
18. dataset corruption is detected;
19. benchmark results are exportable;
20. no claim of physical authenticity is made.

⸻

61. Criteria for Retaining HDD

HDD MAY remain in Q1 after the laboratory phase only if evidence shows at least one substantial benefit.

Possible retention criteria:

* meaningful reduction in total energy per unit of security;
* useful network data storage;
* bounded and understandable participation;
* measurable attack-cost increase;
* acceptable device wear;
* low user-entry cost;
* limited centralization advantage;
* compatibility with deterministic verification;
* no dependence on unverifiable telemetry.

At least one benefit MUST outweigh added complexity and risk.

⸻

62. Criteria for Restricting HDD

HDD MAY remain only as an optional research or archival component if:

* it provides useful storage but weak consensus security;
* it improves telemetry but not validity;
* it assists archival nodes;
* it supports network recovery;
* it provides educational or benchmarking value.

In this case it MUST NOT affect block validity or monetary issuance.

⸻

63. Criteria for Removing HDD

The HDD module SHOULD be removed from consensus design if experiments show:

* SSD or RAM simulation defeats the intended property;
* physical authenticity cannot be meaningfully verified;
* energy or wear costs exceed benefits;
* large operators gain overwhelming advantage;
* the workload creates only waste;
* proof verification is too expensive;
* user safety is unacceptable;
* device behavior is too inconsistent;
* remote outsourcing defeats fairness;
* complexity increases attack surface without measurable security gain.

Removal is a valid and successful research outcome.

⸻

64. Research Report Requirements

Every major HDD experiment SHOULD produce:

experiment_id
hypothesis
hardware configuration
software version
dataset configuration
workload configuration
cache mode
network mode
measurement method
raw results
derived metrics
observed limitations
attack attempts
comparison devices
conclusion
recommended next action

The report MUST distinguish measurement from inference.

⸻

65. Known Limitations

Q1 HDD Lab v0.1 cannot prove:

* mechanical rotation;
* physical location;
* unique physical device identity;
* local ownership;
* absence of caching;
* absence of faster storage;
* honest latency reporting;
* honest energy reporting;
* permanent storage over time;
* resistance to specialized storage systems.

The module is an experiment in measurable storage work, not a trusted hardware oracle.

⸻

66. Open Decisions

The following remain unresolved:

1. exact initial dataset size;
2. exact chunk size;
3. exact sample count;
4. exact dataset commitment tree;
5. exact workload selected for milestone one;
6. whether direct I/O will be required;
7. how cache effects will be normalized;
8. whether write workloads are necessary;
9. whether dataset storage can become useful;
10. whether validators store datasets;
11. whether evidence includes chunk contents;
12. acceptable proof size;
13. whether HDD affects delay input;
14. whether HDD affects rewards;
15. whether HDD affects eligibility;
16. how capacity influence is capped;
17. how duplicate datasets are detected;
18. how remote execution affects fairness;
19. how energy is measured reliably;
20. how wear is estimated;
21. whether RAID receives separate treatment;
22. whether HDD testing continues after formal VDF adoption;
23. whether archival storage becomes a separate protocol role.

All decisions MUST be recorded in OPEN_DECISIONS.md.

⸻

67. Codex Implementation Rules

Codex MUST:

1. implement HDD as a replaceable plugin;
2. default to TELEMETRY_ONLY;
3. keep base consensus operational without HDD;
4. restrict all file operations to an explicit test directory;
5. disable destructive write workloads by default;
6. implement deterministic dataset generation;
7. implement deterministic challenge selection;
8. bind commitments to chain, height, round, producer, and dataset;
9. expose simulator and adversarial modes;
10. distinguish device metadata from trusted evidence;
11. record cache mode;
12. enforce dataset and workload limits;
13. use checked arithmetic;
14. support cancellation;
15. detect dataset corruption;
16. export structured benchmark results;
17. compare HDD, SSD, RAM disk, and virtual disk;
18. document every platform-specific behavior;
19. provide stable failure codes;
20. preserve the ability to remove the entire module.

Codex MUST NOT:

* treat an operating-system disk label as proof;
* write outside the configured directory;
* format or repartition devices;
* use raw-device access by default;
* grant reward merely for reporting hardware;
* let HDD telemetry finalize blocks;
* hide wear or energy costs;
* assume one logical disk equals one physical disk;
* assume one device equals one independent participant;
* claim that successful commitment proves mechanical access;
* make ledger correctness depend on AI classification of HDD data.

⸻

68. Final HDD Principle

The Q1 HDD Laboratory Module MUST preserve this principle:

Physical machinery may create cost, latency, and measurable behavior, but it does not automatically create truth.
A hard drive may contribute data, storage, or work; it may not demand trust merely because it spins.
Every claimed benefit must survive comparison with caches, SSDs, RAM, virtual disks, outsourcing, and fraud.
If HDD adds measurable value, Q1 may keep it.
If it adds only complexity, heat, wear, or illusion, Q1 must remove it.

Q1 succeeds in the HDD laboratory when it can explain, with evidence, not belief:

* what the drive contributed;
* what could be verified;
* what could be forged;
* what energy was consumed;
* what wear was created;
* who gained advantage;
* and whether the network is better with the module than without it.