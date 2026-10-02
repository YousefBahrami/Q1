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

Current phase: post-release review of Q1 LOCALNET v0, v0.1.0-localnet.1.
The [public source release](https://github.com/YousefBahrami/Q1/releases/tag/v0.1.0-localnet.1)
and remote CI are complete. This is public source distribution, not a public
network or Mainnet. Q1 is an open-source protocol project initiated by Yousef
Bahrami, who maintains the project. Apache-2.0 is approved under DEC-Q1-015.

The current human instruction authorizes status/attribution corrections,
an evidence-based differentiation audit and a PUBLIC TESTNET v0 plan only.
Testnet implementation requires explicit approval of the thesis and milestone.
No consensus, economics, delay or admission decision is made by these documents.

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
- Public repository initialized from the clean reviewed snapshot without private
  history; release tag/assets published after successful remote CI.
- Anonymous fresh public clone build, complete checks, signed transfers and
  four-process recovery reproduced the accepted StateRoot.

B. IMPLEMENTABLE NOW
- Reproduce the complete local milestone with `python3 scripts/check_all.py`.
- Review the [technical thesis](docs/research/Q1_DIFFERENTIATION_AUDIT.md) and
  [proposed testnet gates](docs/research/Q1_PUBLIC_TESTNET_V0_PLAN.md).
- Maintain public status and attribution without changing protocol behavior.

C. BLOCKED BY REAL PROTOCOL DECISION (outside completed local acceptance)
- General public-network quorum/fault model, producer selection and round changes.
- Final Mainnet state/proof, compound-schema, delay and economic policies.
- Public-network transport/admission and resource policies.
- No license blocker: DEC-Q1-015 is approved as Apache-2.0.
- No source-publication blocker: the explicitly authorized release is public.
- PUBLIC TESTNET implementation and deployment are not authorized by this phase.

D. DEFERRED TO LATER MILESTONE
- Producer failover, view changes and automatic recovery of unfinalized producer
  work; after insufficient quorum retry the exact reserved proposal.
- Partial state proofs, optimized/pruned storage and encrypted wallet custody.
- AI and HDD/hardware integration, public admission/deployment, Mainnet,
  token sale, explorer and exchange integration.

The four-node acceptance target is implemented. This is a local research
milestone, not a production/public-network release or completed final Q1 protocol.
Remote CI and public source distribution passed; public-network operation has
not been performed. [Post-release review](docs/releases/POST_RELEASE_REVIEW.md)
records the publication evidence and status corrections.

Historical schema approvals: DEC-Q1-027 Sessions 1–5C in docs/32–41.
RoundNumber closure: docs/44. Historical reports remain historical; this file
and the local profile are the current execution entry points.


Release delivery: README now covers clean setup, manual supervision, wallet
creation, signing/submission and voter stop/restart. `scripts/prepare_release.py`
exports committed source only, excludes personal context/Git history, and emits
checksums. `docs/releases/` records release scope and audit findings. Do not
push the original working repository's history as the public release.
