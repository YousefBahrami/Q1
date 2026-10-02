# DEC-Q1-027 Sessions 1–5C Checkpoint Review

Status: Final documentation review complete

Review date: 2026-07-25

Baseline commit: `4b3355a3d504e320252036c824037c260b36db38`

## 1. Scope Reviewed

This review covers every pending repository change since the accepted M1.1
checkpoint and the accumulated documentation decisions from DEC-Q1-027
Sessions 1, 2, 3, 4, 5A, 5B, and 5C.

The review is documentation-only. It does not authorize or create
RoundNumber, protocol-object, Merkle, StateRoot, DelayEvidence, BlockHeader,
ledger, consensus, network, wallet, or runtime implementation.

## 2. Decisions Covered

The reviewed documentation consistently records:

- the three-field ChainIdentityPreimageV1, network-class registry, and
  separation of ChainId, network name, GenesisId, HRP, and network class;
- distinct schema and protocol versions;
- append-only enum rules, strict fixed field counts, and fixed-position
  explicit null only where a schema declares an optional field;
- the three-field ParticipantIdentityBodyV1 and six-field
  ParticipantRecordV1;
- the nine-field TransferBodyV1 and three-field SignedTransferV1;
- indexed Merkle leaves, typed empty roots, and promotion of an unpaired odd
  node without duplication;
- the two-field BlockBodyV1 and three-field ParticipantSetV1;
- the three-field ParentReferenceV1 and its GENESIS/BLOCK distinction;
- the three-field SignedBlockHeaderV1 and complete-envelope BlockId;
- the exact eleven-field BlockHeaderBodyV1;
- the exact five-field DelayEvidenceV1 base shape and conditional NONE form;
- the inactive StateRoot construction profile and implementation blocker;
- omission of ReceiptRoot, CandidateIndex, serialized counts, memo,
  transaction type, sender address, and generic body commitment where the
  approved V1 schemas require their absence.

No approved field order, domain payload, or protocol rule was changed by this
checkpoint review.

## 3. Documentation Consistency Findings

The first checkpoint pass found two stale statements in the main decision
package. The authorized correction changed only:

1. the package header to record documentation approval through Sessions 1–5C
   while separately retaining blocked implementation and runtime activation;
2. the executive-summary domain statement to remove an obsolete numeric count
   and state that additional assignments and registrations remain required.

The follow-up search found no remaining direct statement that only Sessions
1–4 and 5A are recorded, no remaining fixed count for pending domains, and no
statement that Header schema approval authorizes implementation or activation.
Historical session reports retain the boundary that was true at the close of
their respective sessions and are not current-status contradictions.

No further protocol inconsistency or stop condition was found.

## 4. Approval-Field Findings

- SCHEMA-COMMON-001–022, SCHEMA-GATE-001–003, SCHEMA-PART-001–005,
  SCHEMA-TX-001–006, SCHEMA-ECON-001, SCHEMA-MERKLE-001–005, and
  SCHEMA-BLOCK-001–006 carry their reviewed final statuses.
- The Session 5C DelayEvidence architecture, base shape, separate-domain
  requirement, and conditional NONE form are recorded with their actual
  approval and blocker qualifications.
- Genesis, Proposal, Attestation, Certificate, Snapshot, unreviewed economics,
  and SCHEMA-DOMAIN-001 rows remain blank and non-normative.
- CHAIN_ID and PARTICIPANT_ID remain approved pending normative registration.
- PROPOSAL_ID and ATTESTATION_ID remain proposed and unapproved.
- DELAY_EVIDENCE is approved in purpose but has no numeric assignment and
  remains pending normative registration.
- The normative domain registry was not modified.
- Schema approval is consistently separated from implementation and runtime
  activation authorization.

No unreviewed decision was found marked approved.

## 5. Blockers Preserved

The repository still explicitly preserves:

1. the separately authorized M1.1 RoundNumber primitive correction;
2. normative registration of CHAIN_ID;
3. normative registration of PARTICIPANT_ID;
4. numeric assignment and normative registration of DELAY_EVIDENCE;
5. exact DelayEvidence engine profiles and resource bounds;
6. network-profile authorization for NONE;
7. the complete StateRoot/state-model decision;
8. StateRoot profile activation;
9. independent golden and rejection vectors;
10. explicit implementation authorization.

None is closed or weakened.

## 6. Repository Findings

Comparison with the baseline commit found:

- no changed source file under `crates/`;
- no change to the workspace or research Cargo manifests or lockfiles;
- no generated artifact included in Git;
- root `target/` ignored by the root `.gitignore`;
- `research/pre_m1_conformance/target/` ignored by its local `.gitignore`;
- no scratch, temporary, editor, or macOS metadata file;
- no high-confidence secret;
- no change to `docs/protocol/Q1_CRYPTOGRAPHIC_DOMAIN_REGISTRY_V1.md`;
- all intended Session reports and decision briefs present;
- engineering briefs explicitly labeled documentation/engineering review only
  with no human decision recorded.

No Git remote is configured.

## 7. Proposed Checkpoint File Set

Modified tracked files:

- `CHANGELOG.md`
- `OPEN_DECISIONS.md`
- `PROJECT.md`
- `README.md`
- `docs/01_GLOSSARY.md`
- `docs/03_ARCHITECTURE.md`
- `docs/04_LEDGER_AND_TRANSACTIONS.md`
- `docs/05_CONSENSUS.md`
- `docs/06_DELAY_ENGINE.md`
- `docs/09_WALLET.md`
- `docs/14_TEST_PLAN.md`
- `docs/22_REQUIREMENTS_TRACEABILITY_SKELETON.md`

Untracked documentation proposed for the checkpoint:

- `docs/31_M1_2_PROTOCOL_OBJECT_SCHEMAS_REPORT.md`
- `docs/32_DEC_Q1_027_SCHEMA_DECISION_PACKAGE.md`
- `docs/33_DEC_Q1_027_SESSION_1_COMMON_RULES_REPORT.md`
- `docs/34_DEC_Q1_027_SESSION_2_PARTICIPANT_AND_TYPES_REPORT.md`
- `docs/35_DEC_Q1_027_SESSION_3_TRANSFER_REPORT.md`
- `docs/36_DEC_Q1_027_SESSION_4_MERKLE_REPORT.md`
- `docs/37_DEC_Q1_027_SESSION_5A_BLOCK_ID_AND_PARENT_REPORT.md`
- `docs/37_DEC_Q1_027_SESSION_5_BLOCKHEADER_DECISION_BRIEF.md`
- `docs/38_DEC_Q1_027_SESSION_5A_BLOCK_ID_AND_PARENT_REVIEW.md`
- `docs/38_DEC_Q1_027_SESSION_5B_ROOTS_AND_BODY_BRIEF.md`
- `docs/39_DEC_Q1_027_SESSION_5B_ROOTS_AND_BODY_REPORT.md`
- `docs/40_DEC_Q1_027_SESSION_5C_FINAL_HEADER_BRIEF.md`
- `docs/41_DEC_Q1_027_SESSION_5C_FINAL_HEADER_REPORT.md`
- `docs/42_DEC_Q1_027_SESSIONS_1_TO_5C_CHECKPOINT_REVIEW.md`

The proposed checkpoint therefore contains 26 documentation files: 12
modified tracked files and 14 new documentation files.

## 8. Validation Results

All blocking checks passed:

| Check | Exact result |
|---|---|
| `cargo fmt --all -- --check` | passed |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | passed |
| `cargo test --workspace --all-targets --all-features` | 33 passed; 0 failed |
| `cargo doc --workspace --no-deps` | passed; q1-primitives documentation generated |
| `cargo audit` | 1,169 advisories loaded; 57 dependencies scanned; no vulnerability reported |
| `git diff --check` | passed |
| documentation checker | passed; 0 blocking failures |
| requirement-ID scan | 392 definitions; 392 unique; 0 duplicates |
| decision-ID scan | 27 definitions; 27 unique; 0 duplicates |
| broken Markdown links | 0 |
| high-confidence secret scan | 0 findings |
| macOS metadata scan | 0 findings |
| Rust conformance tests | 4 passed; 0 failed |
| Rust conformance executable | 10 CBOR valid, 14 rejected; 9 Amount valid, 11 rejected |
| Node/OpenSSL conformance | passed; same vectors and rejection counts as Rust |
| Python conformance | passed; three addresses and 9 Amount vectors reproduced |
| conformance `cargo fmt --check` | passed |
| conformance clippy with `-D warnings` | passed |

The documentation checker also reports 27 known future-file references, 10
legacy-path mentions, and 133 placeholder lines. These are non-blocking,
pre-existing planning/governance references and are outside this checkpoint's
bounded correction.

## 9. Unresolved Risks

The blockers in section 5 remain consensus and implementation risks. In
addition, later object schemas are deliberately unapproved, resource ceilings
are not runtime allocation limits, and the existing conformance corpus does
not constitute the future independent protocol-object vectors required before
implementation.

The checkpoint records decisions; it does not make the protocol executable.

## 10. Recommendation

**APPROVE DEC-Q1-027 SESSIONS 1–5C CHECKPOINT**

The working tree is ready for one documentation-only checkpoint containing
the exact file set above. Staging and commit require separate explicit human
authorization. Session 6 and all implementation remain unauthorized.
