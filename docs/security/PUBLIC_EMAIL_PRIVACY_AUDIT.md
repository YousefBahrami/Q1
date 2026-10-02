# Public email/privacy audit and maintainer commands

Date: 2026-10-03. Audited public baseline:
`21c107e8dac1f01c703579c0a044a6900ad31901`.
Result: **no personal mailbox address found in the inspected public material**.
The only email occurrences in the baseline Git objects were the GitHub noreply
author/committer identity listed below. No history rewrite was required.

## Scope and exact locations

The scan covered all 179 tracked baseline files, all 190 reachable file blobs
across both public commits, commit headers/messages, public refs and release
metadata. All three release assets were downloaded and scanned; the source
archive was inspected member by member without extracting private Git history.

| Public surface | Email findings |
|---|---|
| Commit `2f3b95375be9bb38aa8e2196759c2bb8554d68b9` | `author` and `committer`: `180775707+YousefBahrami@users.noreply.github.com`. |
| Commit `21c107e8dac1f01c703579c0a044a6900ad31901` | `author` and `committer`: the same noreply address. |
| Current tracked files and historical file blobs | No email-like address found at the audited baseline. No personal-email file/line to report. |
| README, CONTRIBUTING, SECURITY and release notes | No contact email. SECURITY explicitly says a private reporting channel is not yet configured. |
| Workspace/package metadata and historical conformance package | No email; no conflicting author contact. |
| Release title/body/API metadata | No email. Public GitHub account identifiers are not private mailboxes. |
| Custom source archive, checksum and attached release notes | No email. Downloaded hashes matched the existing published assets. |
| Release tag | Lightweight tag at the public root; no separate tagger/email object. |
| Public user profile API | `email: null`; this does not prove account-level email privacy toggles are enabled. |
| Issue/PR listing | Empty at audit time; no email found. |

This new audit document intentionally includes the public noreply address for
reproduction. The next documentation commit must use it for both author and
committer. Neither the numerical GitHub account ID nor the username is hidden
by noreply; its purpose is avoiding publication of a personal mailbox.

An Apple ID may use any email provider. No Apple account, Keychain or mailbox
was accessed to infer an association. There was no personal address in the
inspected public text to compare; **Apple ID linkage itself is not verified**.
The scan is bounded: it does not prove absence of obfuscated addresses, images,
external copies, inaccessible/deleted GitHub objects or future contributions.

## Fixes performed and limits

The public checkout already used GitHub noreply. The private working checkout
was using an inherited non-noreply Git setting. Both Q1 repositories now have
an explicit local noreply email and `user.useConfigOnly=true`. Original private
commits were neither changed nor uploaded. Global Git settings were not changed.
No personal address or private config path is copied into this public report.

No dedicated Q1 project email was identified in the approved project material.
Do not invent one or use noreply as an inbox. Public issues are appropriate for
non-sensitive technical discussion only. GitHub private vulnerability reporting
was disabled when checked; configuring a private channel remains a separate
operational decision, not an excuse to publish exploit details or personal email.

## Exact Git commands

Run these inside each Q1 checkout used for future commits:

```sh
git config --local user.email '180775707+YousefBahrami@users.noreply.github.com'
git config --local user.useConfigOnly true
git config --show-origin --get user.email
git var GIT_AUTHOR_IDENT
git var GIT_COMMITTER_IDENT
```

Review these outputs locally; do not paste unrelated private identities publicly.
Environment variables and explicit author options can override ordinary config.
The setting applies to future commits, not existing history. See
[GitHub's commit-email instructions](https://docs.github.com/en/account-and-profile/how-tos/email-preferences/setting-your-commit-email-address).

If you intentionally want this identity for **all repositories**, this optional
command has broader scope and was **not** run during the audit:

```sh
git config --global user.email '180775707+YousefBahrami@users.noreply.github.com'
```

Existing repository-local settings can override a global setting. Do not push
the original private Q1 history; use only the public-history checkout for public
publication, regardless of its current email configuration.

## GitHub privacy settings and push blocking

Open the authenticated account's email settings on macOS:

```sh
open 'https://github.com/settings/emails'
```

In **Settings → Emails**, enable:

1. **Keep my email addresses private**.
2. **Block command line pushes that expose my email**.

These are account settings, not Git config keys. Their current checkbox state
was not read or changed in this audit; public `email: null` is insufficient
evidence. No unsupported CLI command to toggle them is claimed here.

GitHub documents that its blocking check examines the **most recent commit's
author email** against private emails on the account. It is not a scan of all
earlier commits, committer fields or file contents. See the
[official push-blocking instructions](https://docs.github.com/en/account-and-profile/how-tos/email-preferences/blocking-command-line-pushes-that-expose-your-personal-email-address).

Before a maintainer push from the canonical public checkout, inspect **all**
outgoing commits and the file diff:

```sh
git fetch origin main
git log origin/main..HEAD --format='%H%nAuthor: %an <%ae>%nCommitter: %cn <%ce>%n%B'
git diff --check origin/main..HEAD
git diff origin/main..HEAD
```

Stop if an unexpected address appears. Do not push and then rely on deletion.
For other branches/tags, review their complete outgoing history and any annotated
tagger identity as well. This manual review supplements GitHub's partial block;
it is not an installed pre-push hook or automatic full-history privacy guarantee.
