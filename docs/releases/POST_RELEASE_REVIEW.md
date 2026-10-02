# Post-release review — 2026-10-02

## Actual public status

Q1 has a [public source repository](https://github.com/YousefBahrami/Q1) and
[experimental source release v0.1.0-localnet.1](https://github.com/YousefBahrami/Q1/releases/tag/v0.1.0-localnet.1).
It does not have a launched public testnet or Mainnet. No complete independent
security audit has been performed. Source availability and passing CI do not
change those boundaries.

The public repository began at root commit
`2f3b95375be9bb38aa8e2196759c2bb8554d68b9`, containing the 175 reviewed snapshot
files. No original private Git history, personal collaboration directory or
private repository metadata was transferred. Later documentation commits extend
only that public history; the release tag and attached artifacts are preserved.

Publication evidence:

- [Initial main CI](https://github.com/YousefBahrami/Q1/actions/runs/36986600792)
  and [tag CI](https://github.com/YousefBahrami/Q1/actions/runs/37035748177) passed
  on the public root, including actual four-process acceptance and wallet/CLI.
- After publication an anonymous HTTPS clone with an initially absent target
  directory matched all 175 per-file hashes. README build and full checks passed:
  86 workspace Rust tests, 4 historical tests, 23 protocol and 13 local vectors,
  four-process acceptance, wallet/CLI, fmt, Clippy, Rustdoc and blocking docs checks.
- The fresh clone shared the installed toolchain and normal Cargo download cache.
  This was not an empty-machine toolchain-install test or an independent auditor's run.
- Final accepted state: height=4, sender=956, recipient=40, nonce=4, pool=4,
  supply=1000. All four nodes recovered the common root
  `b7ec7d47f4cf37d029e74bab02d8bc2aab191f5a5733f92ac4310d919183b087`.
- All three custom release assets were downloaded anonymously and matched the
  reviewed files byte-for-byte. Source archive SHA-256:
  `813628168e253326bac770db7d0808bbc7eb0b1de4007482514694ae78cce7b0`.
  This is the named `q1-v0.1.0-localnet.1-source.tar.gz` asset, not GitHub's
  automatically generated source archive.

## Public-facing status audit

| Surface | Finding and correction |
|---|---|
| README | Replaced pre-publication claims with actual clone/release/CI links; distinguishes source distribution from network operation. |
| PROJECT and OPEN_DECISIONS | Publication authorization and completion recorded; general protocol decisions stay open. Current scope is review/planning only. |
| CONTRIBUTING | Actual public issue/PR routes replace the unavailable-channel statement; sensitive reports still require a private route. |
| SECURITY | Published experimental release status and maintainer terminology; local-only limits and lack of a configured private intake remain explicit. |
| Current release notes and CHANGELOG | Public release and CI now recorded; old implementation entries are explicitly historical. |
| Local profile and milestone report | Updated publication/license status only; approved protocol bytes, rules and test observations unchanged. |
| Preparation report and REPRODUCIBILITY.json | Preparation report explicitly marked historical. Original machine-readable reproduction evidence retained unchanged. |
| Older numbered decision/review documents | Dated historical approvals are not rewritten as current authorization. Current PROJECT and later human decisions govern; original evidence is preserved. |
| Release page, tag and assets | Release page already states published experimental scope; project attribution added. Immutable tag/assets preserve original reviewed bytes, including historical preparation wording. |
| Repository description | Minimal initiated-by attribution plus experimental LOCALNET scope; no invented organization or public-network claim. |

## Minimum public attribution

**Q1 is an open-source protocol project initiated by Yousef Bahrami.**
Yousef Bahrami is the project maintainer. Use those roles consistently in README,
release notes, contribution/security guidance and the repository description.
No biographical information is needed. This review does not add an exclusive
original-designer claim, imply ownership of a network, or invent a foundation,
company, issuer, team or community.

LICENSE remains the unmodified Apache-2.0 text. Its appendix's bracketed example
copyright line is part of the standard license, not an AI or organization
attribution. The reviewed snapshot had no separate NOTICE file or conflicting
project copyright headers; none is fabricated by this update. Existing third-party
notices/licenses remain untouched. No new legal assignment or conclusion about
copyright ownership is made.

All four workspace packages inherit Apache-2.0 and use `publish = false`; the
historical research package also declares Apache-2.0. Cargo metadata has no
conflicting author claim. Leaving it unchanged is the minimum correction; an
`authors` field is unnecessary for identifying the initiator and maintainer in
project documentation. No crate publication is authorized here.

AI tools assisted development/review. Promea, OpenAI and AI tools are not presented
as legal owners, issuers or copyright holders. No legal personhood follows from
acknowledging development assistance.

## Deliverables and scope

- [Differentiation audit](../research/Q1_DIFFERENTIATION_AUDIT.md): separates
  standard engineering, Q1 choices, possible novelty and unproven research.
- [PUBLIC TESTNET v0 plan](../research/Q1_PUBLIC_TESTNET_V0_PLAN.md): proposed
  minimum topology, explicit decision dependencies and testable launch gates.
- [Pre-offer boundary](PRE_OFFER_BOUNDARY.md): factual engineering scope only.

No consensus/runtime code, domain assignment, frozen vector, economic rule or
network exposure is changed by this review. Human approval of the technical
thesis and milestone is required before testnet implementation. A later launch
also needs explicit approval of the concrete implementation and review evidence.

## Validation of this documentation update

The complete `python3 scripts/check_all.py` passed locally in the separate
public-history checkout with this review's documentation: 86 workspace Rust
tests, 4 historical Rust tests, 23 protocol vectors, 13 LOCALNET vectors, actual
four-process acceptance, wallet/CLI, formatting, Clippy, build, Rustdoc and
blocking documentation checks. The accepted StateRoot was unchanged. Only
Markdown files changed; source code, Cargo metadata/lockfiles, vectors and LICENSE
were compared against the published root and remain byte-identical.

Historical placeholder/missing-reference advisories remain visible; blocking
documentation failures were zero. Checks and a bounded release-hygiene scan
are not a complete security audit. For remote validation of this later update,
inspect the commit-specific result in [GitHub Actions](https://github.com/YousefBahrami/Q1/actions);
the publication CI links above attest only to the original release root.
