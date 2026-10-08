# Q1 LOCALNET v0

**Experimental software — LOCALNET only. Not Mainnet. No claim of production
security and no public monetary value implied.** Consensus, fee, reward and
delay settings in this release are local test-profile rules, not final Q1 rules.

Published source release: [v0.1.0-localnet.1](https://github.com/YousefBahrami/Q1/releases/tag/v0.1.0-localnet.1),
licensed under [Apache-2.0](LICENSE). **Public source release does not mean
public network:** this release runs only on local loopback addresses. No public
testnet or Mainnet has been launched. No complete independent security audit
has been performed.

Q1 is an open-source protocol project initiated by Yousef Bahrami.
Yousef Bahrami is the project maintainer. These are project roles, not ownership
of a network. No foundation, company or separate governing organization is implied.

Official source: [YousefBahrami/Q1](https://github.com/YousefBahrami/Q1).
Official releases are designated in its [release index](https://github.com/YousefBahrami/Q1/releases).
See [canonical identity and fork guidance](BRAND.md). Third-party implementations
may describe their Q1 origin/compatibility without becoming official releases.

Latest [technical development update](docs/launch/Q1_PUBLIC_TECHNICAL_UPDATE.md):
LOCALNET status, resource research findings and ways to participate.

## What Q1 is

Q1 is a Rust distributed-ledger research implementation with deterministic
execution and independently checked protocol vectors. LOCALNET runs one fixed
producer and three fixed voters on your own computer. Two distinct signed votes
finalize a block; a disconnected voter remains a committee member and catches
up after restart. Each successful transfer moves exactly one base unit into an
explicit reward pool. No issuance, burn or reward distribution occurs.

The implemented [local profile](docs/protocol/Q1_LOCALNET_V0.md) defines exact
bytes and rules. The [milestone report](docs/reports/milestones/LOCALNET_V0.md)
and [machine-readable evidence](docs/reports/milestones/LOCALNET_V0_ACCEPTANCE.json)
record the accepted scenario. [Release notes](docs/releases/v0.1.0-localnet.1.md)
describe this release and its limits. The
[differentiation audit](docs/research/Q1_DIFFERENTIATION_AUDIT.md) assesses what
the evidence supports; the [PUBLIC TESTNET v0 plan](docs/research/Q1_PUBLIC_TESTNET_V0_PLAN.md)
is a proposal awaiting human approval, not an implemented network.

## Research source candidate — not yet published or publicly validated

This checkout adds bounded research after the immutable LOCALNET release. It is
an experimental source candidate; the release tag/archive remain unchanged.

- [Resource v0](research/mining_delay_v0/README.md) and
  [v1](research/mining_delay_v1/README.md) preserve negative/access evidence.
- [Useful resource v2](research/useful_resource_v2/README.md) tests retrievable
  static content, reconstruction and shared storage.
- [Resource uniqueness v0](research/resource_uniqueness_v0/README.md) measures
  identity/root-alias counterexamples without a physical-resource oracle.
- [Integrated TESTNET_FAILOVER_V0](research/testnet_ledger_v0/README.md) executes
  real Q1 signed transfers and full StateRoots through bounded producer failures.
- [Multi-host preparation](docs/testnet/Q1_MULTI_HOST_V0_RUNBOOK.md) provides
  validated private SSH routing/deployment commands. Local CLI preflight is not
  independent-host or SSH-network acceptance.

`python3 scripts/check_all.py` includes these research tests and frozen proof
replays. The new `q1-testnet` binary is a fixture-only ledger backend, not Mainnet
or the existing LOCALNET node CLI. The new implementations use public fixture
keys and fixed honest-voter assumptions. Resource rewards, service payments and
consensus authority remain separate. No monetary issuance or public network.
Remote CI for this exact code candidate has not run; only local results may be
claimed until publication is authorized and its public CI succeeds.

## Native research source candidate

This checkout is a proposed source candidate for technical review, not a new
release or public network. Native TLS authentication, replay/recovery and bounded
sessions are implemented and tested locally. Real native two-host networking is
pending. Read the [current technical update](docs/launch/Q1_EXTERNAL_VALIDATION_UPDATE.md)
and [CI/reproduction scope](docs/releases/Q1_NATIVE_CANDIDATE_CI_SCOPE.md).
GitHub Issues is the public technical contact. Candidate availability must not be
inferred from the existing release tag; test the exact reviewed candidate revision.
OpenSSL **3.x** is required on PATH for fresh test certificate generation.

## Architecture

```text
local wallet/CLI → signed transfer → fixed producer → 3 fixed voters
                                       ↓             2 valid votes
                                certified block → durable commit
                                       ↓
                              voter replay / catch-up
```

- `q1-primitives`: canonical CBOR, hashes, domain framing, Ed25519 and addresses.
- `q1-protocol-types`: approved identities, transfers and transaction/participant roots.
- `q1-localnet`: full StateRoot, genesis, signed proposals/votes/certificates,
  atomic execution and durable replay/anti-equivocation storage.
- `q1-node`: loopback TCP daemon and explicit local wallet/transaction commands.
- Python scripts supervise processes and check evidence; they do not sign or
  define ledger execution. Rust validates every submitted transfer and block.

## Requirements

Start in the root of the received checkout or extracted source archive.
No private files, iCloud location, account credentials or prior project history
are needed. To obtain the published version:

```sh
git clone https://github.com/YousefBahrami/Q1.git
cd Q1
git checkout v0.1.0-localnet.1
```

The release tag and attached archive preserve the reviewed snapshot. Its
pre-publication status text is historical; current status is documented on
`main` and the release page. Stay on `main` to read the post-release review.

- macOS or Linux with a C linker/toolchain and Git. The foreground harness uses
  POSIX file locks; wallet creation uses OS randomness from `/dev/urandom`.
- Rust via rustup: `rust-toolchain.toml` pins **1.97.1**, rustfmt and Clippy.
- Python **3.13.3** and Node.js **20.17.0** are the tested reference versions.
  Python scripts use only the standard library; no pip/npm setup is needed.
- Internet access for the first Rust toolchain/dependency download. Cargo.lock
  pins dependencies; subsequent builds may use Cargo's cache.
- Allow several GB of local disk space for debug builds, tests and Rustdoc.
  Generated artifacts are ignored by Git and excluded from release archives.

The recorded local verification host is macOS x86_64. Ubuntu remote CI passed
for the [published tag](https://github.com/YousefBahrami/Q1/actions/runs/37035748177),
including actual four-process acceptance and wallet/CLI integration.
See [current CI runs](https://github.com/YousefBahrami/Q1/actions) for later commits.

## Build

```sh
cargo build --workspace --all-targets --locked
target/debug/q1-node --version
```

Expected version: `q1-node 0.1.0-localnet.1 (LOCALNET_V0 ONLY)`.
If Cargo is not on PATH, add your rustup installation's `bin` directory first.
The commands below use the default Cargo target directory; custom
`CARGO_TARGET_DIR` is supported by the Python scripts.

## Test and reproduce the milestone

```sh
python3 scripts/check_all.py
```

This runs formatting, Clippy, Rust tests, build, Rustdoc, independent protocol
and state-vector checks, four actual node processes, wallet/CLI integration,
and documentation checks. It exits nonzero on a blocking failure. Historical
documentation advisories are printed separately; they are not runtime setup steps.

To run only the accepted four-process scenario after building:

```sh
python3 scripts/localnet_acceptance.py
```

To keep process logs and `acceptance.json`, pass a **new** directory:

```sh
python3 scripts/localnet_acceptance.py --output-dir .localnet-evidence
```

The harness terminates its own nodes on exit. It is a finite test, not a daemon.
Final height is 4; sender=956, recipient=40, reward_pool=4, total_supply=1000.
All four nodes have this final StateRoot:

```text
b7ec7d47f4cf37d029e74bab02d8bc2aab191f5a5733f92ac4310d919183b087
```

## Run an interactive four-node localnet

In terminal 1:

```sh
python3 scripts/localnet.py run --dir .localnet/demo
```

Wait for `Ready`. Leave this foreground supervisor running. It selects four
available loopback ports and records them in `.localnet/demo/network.json`.
It generates the documented fixture genesis and test identities on first run;
subsequent runs recover the same data. Use a different directory for a fresh network.

**TEST ONLY — NEVER USE FOR VALUE-BEARING NETWORKS.** Fixture private seeds are
publicly known. They are written with restrictive file permissions to avoid
teaching unsafe file handling, not to make known keys secret.

In terminal 2, observe all four nodes and make a transfer:

```sh
python3 scripts/localnet.py status --dir .localnet/demo
python3 scripts/localnet.py transfer --dir .localnet/demo --amount 10
python3 scripts/localnet.py status --dir .localnet/demo
```

This spends from the explicitly documented fixture sender to the fixture
recipient. After this first transfer: height=1, sender=989, recipient=10,
reward_pool=1 and total_supply=1000. Status reports each node's height and
StateRoot; allow a short catch-up interval for all four to agree.

## Create and use a local wallet

In terminal 2 while the supervisor is running:

```sh
target/debug/q1-node wallet-new .localnet/demo/alice.key
target/debug/q1-node wallet-address .localnet/demo/alice.key
python3 scripts/localnet.py transfer --dir .localnet/demo --amount 10 --to "$(target/debug/q1-node wallet-address .localnet/demo/alice.key)"
python3 scripts/localnet.py transfer --dir .localnet/demo --amount 3 --key-file .localnet/demo/alice.key
```

A newly created wallet starts unfunded. The first command using `--to` funds it
from the public fixture sender. The next spends from Alice to the default
fixture recipient. Wallet files contain an **unencrypted** 32-byte seed, use
OS randomness, are created without overwriting an existing file, and require
mode 0600 on Unix. This is a local test tool, not production wallet custody.

The transfer wrapper reads the nonce/height, calls Rust to sign, saves the
canonical signed transfer under `.localnet/demo/transfers/`, then submits it.
Amount is a positive integer in base units; the signed fee limit is 1.
For explicit offline signing and raw submission syntax:

```sh
target/debug/q1-node --help
```

If a submission fails after voters reserved it, retry the **same saved file**
using `python3 scripts/localnet.py submit --dir .localnet/demo --file PATH`
with the path printed by the wrapper. Do not substitute a different transaction
at that height. Inspect status if a response was lost; replay is rejected and
there is no exactly-once RPC guarantee. Concurrent submitters are not queued.

## Stop and restart a voter

In terminal 1's console, enter:

```text
stop voter2
```

In terminal 2:

```sh
python3 scripts/localnet.py transfer --dir .localnet/demo --amount 10
python3 scripts/localnet.py status --dir .localnet/demo
```

The remaining two voters still finalize. The stopped voter is reported offline;
committee membership does not change. Then in terminal 1:

```text
start voter2
status
```

The voter loads its archive and catches up automatically. Repeat `status` until
all four heights and StateRoots agree. Enter `quit` or press Ctrl-C to stop the
supervisor's processes; data remains for restart. Producer failure may halt progress.
The automated acceptance additionally checks insufficient quorum, process-kill
recovery and persistent rejection of a conflicting vote after restart.

## Repository structure

`crates/` contains Rust code and tests; `scripts/` contains checks and local
supervision; `vectors/` contains frozen reference fixtures; `docs/protocol/`
contains implemented profiles; `docs/reports/milestones/` contains evidence;
`research/pre_m1_conformance/` retains independently runnable earlier vectors.
`docs/releases/` contains release notes and preparation findings. Older numbered
specifications include proposed future work and historical decisions; they do
not override the current LOCALNET profile. `PROJECT.md` is the project status.

## Security and release status

Read [SECURITY.md](SECURITY.md) before testing. There has been no complete
independent security audit. Do not secure real-world value. Nodes bind only to
loopback; do not forward their ports or expose this RPC on the Internet.
Full archives are bounded at 16 MiB and rewritten on commit; this release makes
no performance, scaling, public-network fault-tolerance or decentralization claim.
State hashes are full commitments, without partial/light-client proofs.
The status CLI is a local observation tool, not a trustless light client.

`python3 scripts/prepare_release.py` prepares a deterministic source archive and
checksums from a clean committed tree. It performs no upload or tagging. The
private working repository retains personal collaboration context and historical
machine paths; `.gitattributes` excludes that context from the public snapshot.
**Do not publish its original Git history.** The public repository was initialized
from the reviewed snapshot with new root commit `2f3b95375be9bb38aa8e2196759c2bb8554d68b9`.
The original release assets and tag remain unchanged by later documentation updates.

## Roadmap

Before a public testnet: decide and verify the broader fault model, producer
rotation/failover, admission, network hardening, resource policy and recovery
of unfinalized work. Before Mainnet: finalize economics and delay mechanisms,
production custody, governance and independent security review. No dates or
Mainnet readiness are promised. See [open decisions](OPEN_DECISIONS.md) and
[contribution guidance](CONTRIBUTING.md). Public-value activity is outside this
milestone; see [pre-offer boundary](docs/releases/PRE_OFFER_BOUNDARY.md).
The [open-source strategy](docs/research/Q1_OPEN_SOURCE_STRATEGY.md) and
[early revenue comparison](docs/research/Q1_EARLY_REVENUE_OPTIONS.md) are planning
documents; no payment channel, token offering or hosted network is launched.
