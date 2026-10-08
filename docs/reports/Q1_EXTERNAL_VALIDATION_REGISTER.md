# Q1 external validation evidence register

2026-10-08. Public-safe register. The milestone sequence is:
**internal tests → public code → public CI → independent reproduction → Public Testnet candidate**.
These are distinct stages. Public Testnet remains BLOCKED.

## Independent participant evidence

No independent participant result has been recorded yet. Do not create a successful
row for the maintainer, an automated CI runner, a hypothetical user or a drafted
report. An unsuccessful independent reproduction is also a milestone worth recording.

| DATE | PARTICIPANT / ANONYMOUS LABEL | Q1 COMMIT | ENVIRONMENT | WHAT THEY ATTEMPTED | RESULT | ISSUE / PR LINK | REPRODUCED? | NEW DEFECT FOUND? | RESOLVED? |
|---|---|---|---|---|---|---|---|---|---|

Result values: PASS / PARTIAL / FAIL / BLOCKED / UNVERIFIED. REPRODUCED means a second
reported reproduction of the observation, not automatically consensus with its
interpretation. Use UNKNOWN where evidence is missing; resolution needs an exact
fix commit or a documented explanation. Preserve original failures and corrections.

Use a participant's chosen public/anonymous label and linked public issue with consent.
Record only necessary OS/architecture/toolchain details. No email, real name, home
location, private hostname/IP, employer/customer identity or credentials are needed.
Redact logs before linking; security vulnerabilities follow SECURITY.md privately.
Maintainers assess claims without changing a negative report into a pass by assumption.

## Automated public evidence — not independent human reproduction

| Date | Q1 commit | Environment | Attempt | Result | Evidence |
|---|---|---|---|---|---|
| 2026-10-08 | 05c60fec048a67e4b25560deca10c556072aa8d8 | GitHub-hosted Ubuntu | Full check_all.py, stopped at native rate workload test | FAIL; test portability corrected in 735b3fe | [Initial CI](https://github.com/YousefBahrami/Q1/actions/runs/37720489659) |
| 2026-10-08 | 735b3fe1bc73a73bb40083ff2860b0d6e8dcc19c | GitHub-hosted Ubuntu, pinned toolchain | Full check_all.py: build/lint/Rust/Python/vectors/loopback acceptance/docs | PASS | [Source CI](https://github.com/YousefBahrami/Q1/actions/runs/37835065229) |

This automation cannot prove the real private native route, physical-host independence,
public-network safety, Byzantine tolerance or permissionless resource uniqueness.
See the [current status and participation instructions](../launch/Q1_EXTERNAL_VALIDATION_UPDATE.md).

## External validation levels

Current independently established level: **LEVEL 0 — public CI only**. No new
participant evidence is created by preparing this guide or running it ourselves.

| Level | Evidence required | Current status |
|---|---|---|
| 0 — Public CI | Linked public commit and hosted CI result | Reached for the published baseline; future candidate commits require their own CI |
| 1 — Independent build | Outside participant, exact commit/environment/command, successful build log | No report |
| 2 — Independent LOCALNET | Participant runs signed-transfer local acceptance and reports matching state/supply | No report |
| 3 — Independent recovery | Participant reproduces voter interruption, two-vote progress, restart/catch-up and full restart with evidence | No report |
| 4 — Reproducible defect or ambiguity | Independent finding, exact counterexample or conflicting specification references, independently checkable reproduction/analysis | No report |
| 5 — Independent multi-host | Actual separate machines under an independent operator, declared transport/fault scenario and ledger/state evidence | No report |

Levels describe types of evidence, not a score to inflate. Level 4 may precede a
successful level 2/3 and remains valuable; it does not imply lower levels passed.
An unsupported or failed attempt is recorded even when no level is reached.
Level 5 must identify native versus SSH-carried transport; it cannot silently close
the prescribed native acceptance or Public Testnet gate. No public port/VPN change
is requested by this invitation. Review scope before attempting a multi-host test.

For each new participant row, add `level(s) evidenced`, relationship/sponsorship
when voluntarily relevant, and link the actual public report. Reconfirm the
commit/environment and retain failures alongside fixes. Maintainer reruns or
project-controlled machines are not independent. No unnecessary personal data.
