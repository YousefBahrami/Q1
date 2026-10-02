# Q1 checks

Run `python3 scripts/check_all.py` from any directory for the same local and CI
checks. Failures produce a nonzero exit code. Rust is located through PATH or
`~/.cargo/bin/cargo`; Node and Python must be installed.

`protocol_reference.mjs` independently signs and hashes approved public test
fixtures; `protocol_reference.py` independently checks CBOR and hashes but uses
fixture signatures. Each accepts an optional Rust TSV output path.

`localnet_reference.py` independently reconstructs canonical initial state and
hashes for the 13 frozen LOCALNET vectors. It does not verify Ed25519 signatures.
`localnet_acceptance.py` starts four real loopback subprocesses using public test
fixtures. It checks signed transfers, fee/supply accounting, voter kill,
2-of-3 progress, restart/catch-up, identical StateRoot, insufficient quorum and
durable anti-equivocation. It kills only its own subprocesses. Build with
`cargo build --workspace --all-targets --locked` first, or run `check_all.py`,
which builds and runs everything. A new `--output-dir /tmp/q1-run` retains logs,
public fixture files and `acceptance.json`; otherwise temporary data is removed.

No public deployment or external CI execution is performed by these checks.


`localnet.py run` is a foreground interactive network, with status, transfer and
retry commands from another terminal. `localnet_cli_test.py` exercises the
README wallet and manual-operation flow. `release_audit.py --history` performs
a bounded pattern/filename audit without printing credential values; it does
not certify security. `prepare_release.py` creates a deterministic committed
source snapshot with hashes, honoring the explicit personal-context exclusions
in `.gitattributes`. It does not publish or tag anything.
