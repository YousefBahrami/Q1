# Q1 Phase 0.1 Normalization Report

Report Version: 0.1.0
Date: 2026-07-24
Status: Complete for Human Review
Current Gate Before Work: Phase 0.1 — REPEAT

## 1. Outcome

The authorized normalization work was completed without implementing protocol
code or approving any Proposed ADR.

Recommended next gate:

**ADVANCE WITH RESTRICTIONS TO PHASE 0.2**

Restrictions:

1. human review of the requirement-ID migration remains required;
2. ADR-0001 through ADR-0005 remain Proposed and unapproved;
3. no M1 or consensus-critical implementation may begin;
4. the initial Git checkpoint remains blocked until the human configures a Git
   identity;
5. unnamed normative clauses still require inventory before traceability can
   be called complete.

## 2. Preserved Historical Findings

The original findings remain in:

- `docs/21_SPECIFICATION_CONSISTENCY_REVIEW.md`
- `docs/22_REQUIREMENTS_TRACEABILITY_SKELETON.md`
- `docs/23_INITIAL_REPOSITORY_STRUCTURE.md`
- `docs/24_M0_IMPLEMENTATION_PLAN.md`

An approved-resolution record was appended to the consistency review rather
than deleting the original findings.

## 3. Path Normalization

Approved paths applied:

- `YOS_v1.0/` → `YOS/`
- root `ADR/README.md` → `docs/adr/README.md`
- created `docs/build/`
- created `docs/security/`
- created `docs/reports/consistency/`
- created `docs/reports/milestones/`
- created `docs/reports/security/`
- created `docs/reports/benchmarks/`
- created M0 placeholder directories `configs/`, `genesis/`, `scripts/`, and
  `tests/`

The scratch files `Plain text` and `Plain text.md` were preserved because no
deletion decision was issued.

The legacy path strings still found by the scan occur in the preserved
historical review, the historical context of DEC-Q1-011, and the ADR README's
statement that the old path is retired. They are not active path instructions.

## 4. Requirement Identifier Migration

Policy applied:

`Q1-<DOCUMENT_DOMAIN>-<NUMBER>`

Migration record:

`docs/reports/consistency/REQUIREMENT_ID_MIGRATION.md`

Statistics:

- pre-existing definitions mapped: 389;
- definitions renamed: 238;
- definitions retaining already-canonical identifiers: 151;
- new approved system requirements added after migration: 2;
- current normative definitions scanned: 391;
- unique current identifiers: 391;
- duplicate current identifiers: 0.

Migration method:

- each definition occurrence was parsed in its source document;
- system-requirement definitions received sequential `Q1-SYS-*` identifiers;
- specialized component documents retained their existing canonical domain and
  number where possible;
- AI Observer definitions moved from `Q1-AI-*` to `Q1-AIO-*`;
- old identifiers, source sections, meanings, and new identifiers were written
  to the migration table before document rewriting;
- no repository-wide blind substitution was used.

The migration changed identifiers only. It did not intentionally change the
meaning of migrated requirements.

## 5. Approved Specification Resolutions Applied

### Genesis

Updated:

- `docs/02_SYSTEM_REQUIREMENTS.md`
- `docs/04_LEDGER_AND_TRANSACTIONS.md`
- `docs/10_TOKENOMICS.md`
- `docs/14_TEST_PLAN.md`
- traceability skeleton

Approved invariant:

`sum(all explicit genesis allocations) = declared genesis supply`

Any treasury, faucet, research, or reserve amount must be an explicit
allocation. No allocation values were approved.

### Initial private-testnet participant model

Updated system and consensus specifications with the canonical label:

`PERMISSIONED_PRIVATE_TESTNET_REGISTRY`

The model is genesis-defined, permissioned, temporary, private-testnet only,
and equal-weighted initially. Public admission and Sybil resistance remain
open.

### Initial economic narrowing

Updated tokenomics to record:

- integer division remainder → Protocol Treasury;
- initial private-testnet fee burn → disabled;
- all initial fees → security reward pool unless a later approved scenario
  changes allocation.

Final fee arithmetic, issuance values/policy, and reward weights remain open.

## 6. Decision Register

`OPEN_DECISIONS.md` now contains:

- an index with milestone blocking information;
- 26 unique Q1 decision records;
- resolved statuses for namespace, paths, genesis, private registry, initial
  remainder, and initial fee burn;
- narrowed but still-open safe-mode behavior;
- separate records for randomness, round change, fork choice, fees, issuance,
  rewards, public admission, formal VDF, and public governance.

Mechanical result:

- decision definitions: 26;
- unique decision IDs: 26;
- duplicates: 0.

## 7. ADR Drafts

Created with `Status: Proposed`:

- `docs/adr/ADR-0001-primary-language.md`
- `docs/adr/ADR-0002-canonical-serialization.md`
- `docs/adr/ADR-0003-hash-function.md`
- `docs/adr/ADR-0004-signature-algorithm.md`
- `docs/adr/ADR-0005-address-encoding.md`

No ADR contains an approved selection. The drafts compare options, list
required evidence, and retain open questions.

## 8. Security Drafts

Created:

- `SECURITY.md`
- `docs/security/THREAT_REGISTER.md`
- `docs/security/INCIDENT_RESPONSE.md`
- `docs/security/KEY_MANAGEMENT.md`
- `docs/security/RELEASE_SECURITY.md`
- `docs/security/DEPENDENCY_POLICY.md`

The threat register contains 18 initial threats extracted from the existing
security and adversarial specifications. Controls are recorded as specified,
not as implemented.

The incident-response draft records the approved safe-mode principle:
automatic entry may be allowed, but exit requires an auditable recovery
package. Exact authority and validation remain open.

## 9. Repository Foundation Files

Created or normalized:

- Q1-specific root `README.md`;
- `PROJECT.md` reading order and authorization boundary;
- `.gitignore`;
- `CHANGELOG.md`;
- `CONTRIBUTING.md`;
- `LICENSE_DECISION.md`;
- M0 directory README placeholders;
- `scripts/check_documentation.py`.

`LICENSE_DECISION.md` grants no rights and only records DEC-Q1-015 as open.

## 10. Mechanical Checks Executed

Command:

`python3 scripts/check_documentation.py`

Final result before this report:

| Check | Result |
|---|---|
| Normative identifier definitions | 391 |
| Unique normative identifiers | 391 |
| Duplicate normative identifiers | 0 |
| Decision definitions | 26 |
| Unique decision identifiers | 26 |
| Duplicate decision identifiers | 0 |
| Broken Markdown links | 0 |
| macOS metadata | 0 |
| High-confidence secret patterns | 0 |
| Placeholder lines | 95 |
| Missing referenced file tokens | 27 |
| Blocking check failures | 0 |

Placeholder lines are expected because Proposed ADRs, open decisions, future
artifacts, and explicit M0 placeholders remain visible.

Missing references were not silently created. They consist primarily of:

- later ADRs such as storage and P2P transport;
- future milestone reports/templates;
- future governance/QIP documents;
- independent-review artifacts;
- explicitly unauthorized future mainnet specifications;
- one scanner tokenization artifact caused by the filename `Plain text.md`.

These findings remain visible. They do not establish that the referenced
future documents exist.

Additional manual/high-confidence scan performed:

- private-key PEM headers;
- common AWS, GitHub, OpenAI-style, and Google API key prefixes;
- macOS metadata names.

No high-confidence secret was found. This is not a proof that no sensitive
information exists.

## 11. Git Status

`git init` completed successfully.

Repository state:

- branch: `main`;
- commits: none;
- remote: none created;
- all current project files are untracked pending the initial checkpoint.

The configured Git user name and email are both absent. In accordance with the
authorization, no identity was invented and no commit was attempted.

Checkpoint status:

**BLOCKED ON HUMAN GIT IDENTITY CONFIGURATION**

After identity configuration, the authorized initial commit message is:

```text
Initial Q1 specification repository

Q1 begins as an engineering specification, not as source code.
The architecture precedes implementation.
The specification governs the code, not the other way around.
```

No remote, push, or public publication is authorized.

## 12. Remaining Open or Blocking Work

Still blocks M1:

- primary language;
- canonical serialization;
- hash function;
- signature algorithm;
- address encoding.

Still blocks later consensus/economic milestones:

- randomness construction;
- round-change protocol;
- complete fork-choice behavior;
- safe-mode recovery validation and authority;
- exact congestion formula;
- final issuance policy;
- reward weights and duplicate-role handling;
- public participant admission;
- formal VDF path;
- public governance.

Specification-readiness limitation:

Many MUST/SHALL clauses remain section-scoped without individual requirement
IDs. They must be inventoried and linked before complete traceability can be
claimed.

## 13. Gate Recommendation

Recommendation:

**ADVANCE WITH RESTRICTIONS TO PHASE 0.2**

Evidence supporting progression:

- current normative identifiers are unique;
- canonical paths are applied;
- blocking decisions are visible;
- ADR-0001 through ADR-0005 drafts are ready for review;
- referenced M0 security documents exist as controlled drafts;
- approved semantic resolutions are explicit and traceable;
- no protocol implementation was created.

Restrictions before any further gate:

- review and approve or correct the migration table;
- configure Git identity and create the authorized checkpoint;
- review ADR drafts without treating them as selected;
- continue reporting missing future artifacts and unnamed normative clauses;
- do not begin M1.
