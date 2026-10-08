# Reproduce Q1 LOCALNET

A finite local experiment, not Mainnet, mining or a public network. No Q1 account,
private infrastructure or real funds are needed. Success and failure are both useful.

## Supported environment / prerequisites

Use a normal macOS or Linux development environment with loopback TCP and POSIX
process/file-lock support. Existing baseline evidence is macOS x86_64 and GitHub
Ubuntu x86_64. Other architectures are unverified attempts; native Windows is
unsupported. Container restrictions can cause a genuine failed/unsupported attempt.

Install **Git, a C linker/development tools, Python 3.13.3 and Rust via rustup**.
Python >=3.11 is accepted by the helper; 3.13.3 is the reference tested version.
Use your OS's development-tool instructions and [official Rust installer](https://rust-lang.org/tools/install/).
After you choose to install rustup, install the pinned compiler explicitly:

```sh
rustup toolchain install 1.97.1 --profile minimal --component rustfmt --component clippy
```

Allow several GB of disk and internet for Cargo dependencies. No pip/npm setup,
Node or OpenSSL is needed for this small LOCALNET path. Initial compilation can
take minutes; the helper allows 30 minutes per stage. It installs no system packages,
changes no network settings, uploads nothing and starts no persistent service.
Do not run Python with `-O` or `PYTHONOPTIMIZE`; acceptance assertions must execute.

## Clone / pinned commit / one command

```sh
git clone https://github.com/YousefBahrami/Q1.git
cd Q1
git checkout --detach 997ee2aa5496726eca8f92021ae115d689d150b5
python3 scripts/validate_public_q1.py
```

The pinned commit contains this helper and the unchanged published ledger; later
status documentation may be on main. The helper prints its evidence directory and
runs the three stages below, stopping at the first failure. A dirty checkout is
recorded in `summary.json`; report modifications rather than calling it an exact
reproduction. Each attempt uses a fresh output directory, never an old success file.

## Build / test / LOCALNET run

Equivalent manual commands (after the same checkout):

```sh
cargo build --workspace --all-targets --locked
cargo test --workspace --all-targets --all-features --locked
python3 scripts/localnet_acceptance.py --output-dir .localnet-evidence
```

Use a new output directory. The acceptance harness starts four actual loopback
processes, submits fixture **signed transfers**, kills one voter, checks continued
2-of-3 finalization, restarts it, verifies catch-up, checks insufficient-quorum and
durable conflicting-vote rejection, then restarts all four processes from disk.
It stops its processes on exit. The keys and balances are publicly known test fixtures.
No manual wallet setup is needed for this first attempt.

## Expected result / success evidence

The helper must exit **0**, print **PASS**, and report all three stages PASS with
`localnet_completed=true` and `restart_catch_up=true`. Its `localnet/acceptance.json`
must show height **4**, pool **4**, supply **1000**, restart/catch-up and full-process
recovery true. The harness also checks sender **956**, recipient **40**, nonce **4**,
fee **1** per transfer and equal final state on all four nodes. Expected StateRoot:

```text
b7ec7d47f4cf37d029e74bab02d8bc2aab191f5a5733f92ac4310d919183b087
```

Exit **1** means FAIL; **2** means UNSUPPORTED/missing preflight prerequisites;
**130** means interrupted. Later unexecuted stages say **SKIPPED**, never PASS.
A LOCALNET pass does not imply all native research checks passed. For optional full
research/CI reproduction, add Node 20.17.0 and OpenSSL 3.x and use
`python3 scripts/check_all.py` as described in the [native scope](../releases/Q1_NATIVE_CANDIDATE_CI_SCOPE.md).
This still does not prove native multi-host operation or public-network safety.

## How to report success or failure

Open a [reproduction report](https://github.com/YousefBahrami/Q1/issues/new?template=reproduction.md).
Return OS/version, architecture, Rust/Python versions, exact Q1 commit and command,
result/failure stage, whether LOCALNET and restart/catch-up completed, and:

- `summary.json` and, if produced, `localnet/acceptance.json`;
- only the relevant excerpt from `build.log`, `rust_tests.log` or `localnet.log`;
- if requested, a sanitized excerpt from `localnet/producer.log` or voter logs.

Read and redact before posting: local paths, usernames, private addresses and
unrelated environment details may occur in compiler/node logs. Do not upload the
whole directory, fixture/key files or account information. No identity, email or
employer is needed. Security-sensitive findings follow [SECURITY.md](../../SECURITY.md).

On failure, keep the original report/log, stop at the failed stage and say what you
actually tried. Missing tools, unclear instructions and negative results are welcome.
Retry only after recording a specific change; use a new directory. Do not disable
tests, reset state to disguise a failure or expose a port to make it work.
