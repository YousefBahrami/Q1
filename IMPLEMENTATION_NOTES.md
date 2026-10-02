IMPLEMENTATION_NOTES.md

Framework: YOS (Yousef Operating System)
Version: 1.0 (Draft)
Status: Active Engineering Notes

⸻

Purpose

This document records implementation knowledge discovered during engineering work.

These notes are not specifications.

They are not architecture decisions.

They are not protocol rules.

They exist to preserve practical engineering knowledge that would otherwise be lost.

⸻

Why This File Exists

During implementation, engineers constantly discover useful information.

Examples include:

* implementation constraints;
* library limitations;
* debugging observations;
* performance findings;
* tooling behavior;
* unexpected interactions;
* failed experiments;
* practical recommendations.

Most of these discoveries never belong in the official specification.

But they should never disappear.

⸻

What Belongs Here

Typical entries include:

* implementation tips;
* repository conventions;
* build observations;
* compiler behavior;
* dependency notes;
* debugging techniques;
* testing lessons;
* performance measurements;
* integration advice;
* migration notes;
* platform-specific behavior;
* developer warnings.

⸻

What Does NOT Belong Here

This document must not become:

* the specification;
* architecture documentation;
* ADR records;
* roadmap planning;
* protocol rules;
* security policy;
* product requirements.

Those belong in their own documents.

⸻

Guiding Principle

Implementation Notes explain:

what engineers learned while building.

They do not define:

what the system must do.

⸻

Note Categories

Every note should belong to one category.

Build
Compiler
Repository
Architecture
Performance
Networking
Database
API
Testing
Debugging
Security Observation
Tooling
Developer Experience
Deployment
Migration
Other

⸻

Note Template

## NOTE-0001
Title:
Category:
Date:
Author:
Related Documents:
Related ADR:
Summary:
Observation:
Recommendation:
Future Review:

⸻

Example

NOTE-0001

Title:

Deterministic serialization required before hashing

Category:

Architecture

Summary:

Several hash mismatches were caused by inconsistent serialization order.

Observation:

Hash functions behaved correctly.

Serialization order was inconsistent.

Recommendation:

Never optimize serialization before deterministic behavior is fully tested.

Future Review:

Remove this note once deterministic vectors are permanently verified.

⸻

Temporary Knowledge

Many implementation notes are temporary.

Once knowledge becomes:

* part of the specification;
* an ADR;
* permanent documentation;
* or no longer relevant,

the note should either:

* reference the new document;
* or be archived.

⸻

Engineering Observations

Observations should describe facts rather than opinions.

Prefer:

* measured behavior;
* reproducible findings;
* documented limitations.

Avoid:

* personal preferences;
* undocumented assumptions;
* unsupported conclusions.

⸻

Relationship with Specifications

If implementation contradicts the specification:

Do not modify the specification silently.

Instead:

1. record the observation;
2. identify the affected specification;
3. create or reference an Open Decision if necessary;
4. request architectural review.

⸻

Relationship with ADRs

Implementation Notes explain what happened.

ADRs explain why an important architectural decision was made.

If a note changes architectural direction, create or update an ADR.

⸻

Relationship with OPEN_DECISIONS

If an implementation note reveals an unresolved architectural question:

Create or reference the corresponding Decision ID.

Do not resolve it here.

⸻

Repository Rule

Every project using YOS should maintain its own Implementation Notes.

Project-specific knowledge remains inside the project repository.

YOS implementation notes remain inside YOS.

Knowledge should not leak between unrelated repositories without explicit review.

⸻

Review Policy

Implementation Notes should be reviewed:

* after every milestone;
* before major refactoring;
* before release;
* after significant debugging sessions.

Obsolete notes should be archived rather than deleted.

⸻

Writing Style

Keep notes:

* factual;
* concise;
* reproducible;
* technically useful.

Future engineers should immediately understand the observation without reading old conversations.

⸻

Final Principle

Specifications describe the intended system.

Implementation Notes preserve the reality encountered while building it.

Strong engineering requires both.

## 2026-10-02 — LOCALNET implementation and verification boundary

The two genuine local blockers were missing state commitment semantics and
producer/committee selection. Historical schema approval gates also held up
compound objects. The latest explicit human instructions resolved those
locally and authorized minimal compound schemas; execution has now advanced
through four real processes. Earlier agent runs stopped at usage limits with
partial files; those files required integration, compilation and verification,
not merely a completion report.

Implementation choices: use a full-state hash because the approved Merkle
state profile has no defined proof/key semantics; derive initial StateRoot
after GenesisId to avoid a cycle; bind fixed authority through genesis; preserve
HeaderBody fields exactly. Persist full certified history plus snapshot and
vote reservations with atomic rename and directory sync; replay before trusting
the snapshot. These choices are local and deliberately unoptimized.

The first full workspace run found an outdated negative test for domain 0x0011
following its authorized local registration. The assertion was updated alongside
the registry and whole-range collision coverage. Early compiler/linter failures
in new fixture imports and nested conditionals were resolved before final checks.
See the local profile for runtime bounds, in-flight producer limitations and
reproducible verification commands. No public network, source publication or
remote CI execution is implied by passing local acceptance.
