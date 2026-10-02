# Contributing to Q1 LOCALNET v0

Read README.md, SECURITY.md and docs/protocol/Q1_LOCALNET_V0.md first.
Build with the pinned toolchain and run `python3 scripts/check_all.py` before
submitting a change. No personal collaboration files or private environment
are required. The source is licensed under Apache-2.0; see LICENSE.

Keep changes focused. Preserve canonical bytes, domain assignments, signature
meaning, conservation of supply and fixed local authorization. Protocol changes
need an explicit documented decision; passing existing tests alone is not
approval to change those rules. Never update frozen vectors merely to hide a
regression. Explain behavior, relevant decisions and tests actually run.

Do not commit keys, credentials, node data, logs, caches or build output.
Intentionally public test seeds must say:
TEST ONLY — NEVER USE FOR VALUE-BEARING NETWORKS.
Do not post security exploit details publicly; follow SECURITY.md.

A public repository and issue/PR endpoint have not been published yet.
Prepare reviewable patches locally until an official contribution channel exists.
No CLA, DCO workflow or maintainer contact is implied by this document.
