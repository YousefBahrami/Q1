# LOCALNET public release candidate preparation — 2026-10-02

Version: v0.1.0-localnet.1. Protocol milestone 0c177fd is accepted; canonical
consensus bytes and fixed local rules are unchanged. This report records
preparation, not publication, an independent security audit or a public network.

## Release audit

A read-only review of tracked files and reachable Git blobs found no real
credential using the bounded key/token/private-key patterns in
`scripts/release_audit.py`. Manual review distinguished deliberate public
Ed25519 fixture seeds from runtime wallets. All fixture material is explicitly
labeled TEST ONLY — NEVER USE FOR VALUE-BEARING NETWORKS. A pattern scan cannot
prove absence of every possible secret; it is not an independent security audit.

Five historical documentation files contained absolute author-home paths.
Their working-tree references now use portable home/relative forms, preserving
test observations and protocol evidence. The old blobs remain in Git history;
no history rewrite was performed. Temporary smoke-test locations in historical
reports are evidence, not current setup dependencies. Current commands require
no iCloud directory, attachment or private file.

YOS contains personal biography/collaboration context. `.gitattributes` excludes
YOS and the private AGENTS instructions from the source archive. The original
files stay available privately. Recorded decision-owner names in protocol
records remain as attribution/authority provenance. The archive excludes all
Git metadata, including author email metadata; original history is not the
public distribution candidate. Nothing has been uploaded.

No generated binaries, node databases, raw key files, caches or logs were tracked.
`.gitignore` now also covers nested research build output, localnet demo/evidence,
release output, editor files, raw wallet keys and runtime state. Frozen canonical
CBOR vectors and milestone evidence remain versioned; no blanket CBOR exclusion
was introduced. New documentation does not depend on private collaboration files.

## Size analysis

At the beginning of this release audit, 177 tracked files totaled 1,736,457
logical bytes (about 1.66 MiB). The largest tracked file was about 71 KiB; no
large tracked binary existed. Documentation accounted for about 1.2 MiB.
Git object storage was about 3.57 MiB allocated; logical `.git` file sizes were
about 2.09 MiB. The slight difference is filesystem block allocation.

Build output dominates the working directory: before further RC builds, root
`target/` held about 1.17 GiB of logical data and the historical research target
about 90.6 MiB. After additional builds, `du -sk` reported roughly 1.33 GiB for
root target, 89.5 MiB for research target, 3.7 MiB for Git and 1.36 MiB for docs.
Debug dependencies, incremental artifacts, examples/tests and generated Rustdoc
explain why the directory can grow past the previously observed ~500 MB while
source stays small. The precise earlier 500 MB snapshot is unavailable; its
historical byte breakdown is not claimed. No cache was deleted merely to make
these numbers look smaller, and no Git history was rewritten.

To reclaim rebuildable output when desired, `cargo clean` clears the workspace
target; `cargo clean --manifest-path research/pre_m1_conformance/Cargo.toml`
clears its independent target. These cleanup commands were not run against the
original working directory during release preparation. Local databases and
wallets are not build caches and must not be treated as disposable output.

## License and publication boundary

The user explicitly approved Apache-2.0 for DEC-Q1-015. LICENSE is the official
Apache license text; workspace packages carry Apache-2.0 metadata. The earlier
placeholder is superseded, with the four-option comparison retained in
LICENSE_DECISION.md. Third-party dependencies retain their own licenses.

The deliverable is a source-only tar.gz plus a SHA-256 sidecar and per-file
manifest. It contains all protocol source, tests, fixtures and reference evidence.
No vendored third-party binaries are redistributed. First builds use the pinned
Cargo dependencies/toolchain and require those downloads or an existing cache.
This is reproducible behavior/source packaging, not a claim of bit-identical
native binaries across machines.

No technical or license blocker is known for the reviewed snapshot after its
checks pass. Publishing the original repository history would still disclose
personal context/old local paths and is not approved. Remote publication and
remote tag/release creation require a separate explicit instruction. Public
security intake is not configured; SECURITY.md states this without inventing
contact details. The release makes no operated-network or custody promise.

## Validation record

The working-tree complete check passed with 86 workspace Rust tests and four
historical conformance tests, all existing and local reference vectors, real
four-process acceptance, and the new README wallet/CLI integration. The latter
checks random wallet creation, mode 0600, overwrite refusal, funding/spending,
voter stop/start/catch-up, replay rejection and process cleanup. Clippy,
formatting, build, Rustdoc and blocking documentation checks passed.

Clean source-snapshot and fresh-clone verification results follow below. Remote CI has not been run. Historical
placeholder/reference advisories remain advisory and are not hidden.


## Completion update

The full check passed again from a fresh `git clone --no-hardlinks` of release
implementation commit `69dde45`, using an initially absent target directory.
It also passed from the actual extracted public source archive, without `.git`,
YOS, AGENTS, private files or a preexisting target directory. The two runs used
independent build directories; they shared the installed toolchain and normal
Cargo dependency-download cache. This was not an empty-machine/toolchain-install
or remote Linux CI test. In the archive, only the Git-diff check was skipped
because no Git metadata exists; all build, protocol, wallet/CLI, runtime and
blocking documentation checks ran successfully.

Per-file SHA-256 manifest entries matched the extracted source. Running the
packager twice for the same committed tree produced the same compressed hash.
The initial tested archive had 174 files / 1,770,883 uncompressed source bytes
and 520,528 compressed bytes. The final documentation-only closure adds this
record and machine-readable evidence; the final sidecar/manifest identify its
new source commit and archive hash. Code and protocol-vector bytes match the
fully tested implementation commit.

[REPRODUCIBILITY.json](REPRODUCIBILITY.json) records both clean reproductions.
The final snapshot's documentation and manifest are checked after adding this
record. No remote tag, remote repository, release or running external network
was created. The reviewed snapshot has no known remaining technical/license
blocker; it awaits explicit publication authorization. Original Git history
remains private and is not a publication candidate.

During preparation, the CLI integration test caught a Python file-mode assertion
typo, and the full check caught a misplaced shebang after adding a fixture
warning. Both were corrected and the final working-tree, clean-clone and
source-archive checks passed. No protocol vector or consensus rule was changed
to resolve either failure.
