# Q1 open-source strategy

Date: 2026-10-03. Status: project strategy and engineering boundaries.
This is a practical summary of the published license and proposed maintenance
work, not a legal opinion or a change to [LICENSE](../../LICENSE).

Q1 is an open-source protocol project initiated and maintained by Yousef Bahrami.
The canonical repository is [YousefBahrami/Q1](https://github.com/YousefBahrami/Q1).
The [identity notice](../../BRAND.md) distinguishes upstream releases from forks
without claiming a registered trademark, exclusive protocol ownership or an
existing foundation/company. The public release is experimental LOCALNET only.

## What third parties may do and must preserve

Apache-2.0 permits use, copying, modification and redistribution, including
commercial derivatives. Distribution conditions include a license copy,
prominent notices of changed files, applicable source copyright/patent/trademark/
attribution notices, and relevant NOTICE attributions if a NOTICE was supplied.
Q1's reviewed release contains LICENSE and no separate NOTICE file.

The contributor patent grant is limited to the claims described in section 3
and has a patent-litigation termination condition; it is not a blanket patent
clearance. Section 6 does not grant general trademark rights and preserves
limited customary origin/NOTICE uses. Sections 7–9 address warranty, liability
and separately offered support. See the authoritative
[Apache-2.0 text](https://www.apache.org/licenses/LICENSE-2.0).

Forks need not send improvements upstream merely because they use Apache-2.0.
They can sell a derivative or services and may keep modifications private within
the license's conditions; the [ASF FAQ](https://www.apache.org/foundation/license-faq)
explains this permissive model. Their independent contracts and other applicable
obligations are separate questions. Neither a paid offering nor a different
license for modifications erases obligations attached to the original work.

## What a fork does not automatically gain

Copying the code does not grant access to this repository/account, release
publishing credentials, future private keys, maintainer endorsement, a support
contract or an existing customer relationship. It does not create a public Q1
network, establish protocol compatibility or appoint an issuer.

Technical compatibility is tested against a named profile/version, canonical
bytes, execution rules and rejection behavior. Matching selected vectors alone
does not establish complete equivalence. Forks can deliberately diverge; document
the differences rather than calling all derivatives fraudulent or unauthorized.

Canonical authority here means the maintainer can decide what this repository
merges and designates as an official release. It does not mean control over
independent operators, users, all uses of the name or all software descendants.
Brand rights and name availability require separate jurisdiction-specific review;
this project has not completed trademark clearance or registration.

## Competition and differentiation

| Area | What official Q1 can maintain | What cannot be assumed / next evidence |
|---|---|---|
| Canonical repository | Stable URL, reviewable history, explicit maintainer decisions and issue triage. | A fork can copy all public content. Keep origin links in current documents and record channel changes. |
| Brand and attribution | Consistent factual identity and release wording; no invented organization. | No blanket exclusive right to the short name Q1 is established. Verify relevant name/register conflicts before spending on a brand. |
| Release governance | Select official versions; CI before releases; immutable-by-policy historical assets; publish hashes and compatibility notes. | Current lightweight tag/checksum is not an independently signed release attestation. Account security, recovery and stronger provenance are future operational work. |
| Genesis/network identity | Publish approved genesis, profile, role keys and activation rules for any future official experiment. | LOCALNET fixture keys are public. Anyone can copy identifiers; trust requires independently authenticated configuration. No public network exists now. |
| Protocol compatibility | Maintain precise specs, frozen vectors, rejection cases and migration rules. | Compatibility is a measurable claim, not endorsement or a right to call a fork official. No formal certification program exists. |
| Tooling | Improve reproducibility, debugging, wallet/CLI ergonomics and independent checks. | Competing projects can reuse improvements. Quality and responsiveness must be earned repeatedly. |
| Community | Provide useful public reviews, newcomer instructions and clear contribution boundaries. | No established community, team size or market demand is claimed. Begin with actual contributors and independently reproduced results. |
| Security/reputation | Publish limitations, negative findings, incident follow-up and externally reviewable evidence. | Passing tests does not establish security; a private reporting route and independent review remain open work. |
| Commercial services | Offer optional scoped expertise, integration and support when eligible to contract and receive payment. | The license does not reserve those services to the maintainer or require royalties from competitors. |

Current differentiation is the executable research baseline, not a demonstrated
unique cryptocurrency. See the [technical audit](Q1_DIFFERENTIATION_AUDIT.md).
The practical strategy is to become a reliable upstream through reproducibility,
review quality and transparent decisions, while measuring whether a new protocol
idea actually improves on existing alternatives.

## Low-cost protection and release discipline

Already present: public-only Git history, consistent initiator/maintainer
attribution, Apache-2.0, explicit local scope, source checksums and successful CI.
This phase adds canonical-origin wording and a documented email/privacy audit.
It does not change runtime authority or the published release archive.

Recommended next operational work, not claimed complete: verify account 2FA and
recovery, choose a private project contact, document reviewer/release authority,
evaluate branch protection and artifact signing, and preserve notices during
packaging. Do not create an issuer or restrict existing license permissions as
a shortcut to these controls. No confidential project key should be embedded in
code, genesis fixtures, release descriptions or support examples.

For forks, recommended wording is a distinct distribution name with upstream
version and modifications, linked origin and an explicit independent maintainer.
This is origin/compatibility guidance; [BRAND.md](../../BRAND.md) does not impose
an additional code-license restriction or assert unverified trademark rights.

For funding, prefer scoped services or development sponsorship with no token
entitlement; evaluate the [early revenue options](Q1_EARLY_REVENUE_OPTIONS.md).
Any public-value distribution remains behind the
[pre-offer approval boundary](../releases/PRE_OFFER_BOUNDARY.md).
