# Q1 Project Entry Point

## Purpose
Entry point for AI agents and collaborators.

Read in order:
1. README.md
2. CONTRIBUTING.md and SECURITY.md
3. PROJECT.md
4. docs/00_PROJECT_CHARTER.md and docs/01_GLOSSARY.md
5. docs/protocol/Q1_LOCALNET_V0.md and current release notes

Private working checkouts additionally use AGENTS.md/YOS when present. Those
personal collaboration files are excluded from the public source snapshot;
they are not build, test, contribution or protocol dependencies.

Current target: Q1 LOCALNET v0 PUBLIC RELEASE CANDIDATE, v0.1.0-localnet.1.
The protocol milestone at 0c177fd was accepted on 2026-10-02. Current work makes
it independently reproducible; no protocol decisions are reopened. Public
network operation, monetary value, remote publication and tag pushes are excluded.
Apache-2.0 was explicitly selected for DEC-Q1-015 during this phase.

Current authority: 2026-10-01 progressive implementation instruction and
2026-10-02 explicit LOCALNET_V0 full-state, fixed-producer and minimal-schema
authorization. These supersede historical no-code/per-session gates within
this scope. Exact bytes, decisions, operational limits and deferred behavior
are recorded in `docs/protocol/Q1_LOCALNET_V0.md`.

A. DONE
- PRE-M1, M1.1 and RoundNumber correction (Gates 1–3).
- Approved chain/participant identities, transfer envelopes, parent references,
  body/set owners, transaction/participant roots and delay evidence.
- LOCALNET fee=1, explicit reward pool, no issuance/burn/distribution, whole-block
  rollback and conservation of supply.
- Full canonical state commitment, minimal local genesis/proposal/vote/certificate,
  unchanged approved HeaderBody, fixed producer plus three voters, 2-of-3 quorum.
- Atomic file persistence, replay verification, exclusive store lock and durable
  anti-equivocation reservations. Failed validation never partially commits.
- Loopback-only node daemon and executable four-process acceptance: signed
  transfers, one voter killed, continued finalization, restart and catch-up,
  equal StateRoot, insufficient quorum rejection and post-restart continuation.
- Frozen LOCALNET vectors alongside existing cross-language protocol vectors.

B. IMPLEMENTABLE NOW
- Reproduce the complete local milestone with `python3 scripts/check_all.py`.
- Review the local-only release candidate and its documented limitations.
- Further operational packaging within the same approved local profile.

C. BLOCKED BY REAL PROTOCOL DECISION (outside completed local acceptance)
- General public-network quorum/fault model, producer selection and round changes.
- Final Mainnet state/proof, compound-schema, delay and economic policies.
- Public-network transport/admission and resource policies.
- No license blocker: DEC-Q1-015 is approved as Apache-2.0.
- Remote publication still requires explicit authorization of the prepared snapshot.

D. DEFERRED TO LATER MILESTONE
- Producer failover, view changes and automatic recovery of unfinalized producer
  work; after insufficient quorum retry the exact reserved proposal.
- Partial state proofs, optimized/pruned storage and encrypted wallet custody.
- AI and HDD/hardware integration, public admission/deployment, Mainnet,
  token sale, explorer and exchange integration.

The four-node acceptance target is implemented. This is a local research
milestone, not a production/public-network release or completed final Q1 protocol.
Remote CI and public distribution have not been performed.

Historical schema approvals: DEC-Q1-027 Sessions 1–5C in docs/32–41.
RoundNumber closure: docs/44. Historical reports remain historical; this file
and the local profile are the current execution entry points.


Release delivery: README now covers clean setup, manual supervision, wallet
creation, signing/submission and voter stop/restart. `scripts/prepare_release.py`
exports committed source only, excludes personal context/Git history, and emits
checksums. `docs/releases/` records release scope and audit findings. Do not
push the original working repository's history as the public release.
