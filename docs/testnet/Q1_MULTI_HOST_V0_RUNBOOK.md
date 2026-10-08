# Q1 TESTNET MULTI-HOST v0 — deployment and acceptance runbook

2026-10-06. Deployment preparation over existing TESTNET_FAILOVER_V0 semantics.
**Not a public Internet service.** No independent-host result is claimed by this
runbook. Human confirmation of hosts, access and failure domains is required.
No Mainnet, monetary balances, admission or Byzantine safety is introduced.

## Architecture and trust boundary

Reuse the tested Rust ledger backend and Python durable coordination unchanged.
Keep every application listener on `127.0.0.1`. For the first independently hosted
experiment, carry peer TCP through pre-approved SSH local forwards with pinned
host keys. This adds ordinary SSH transport dependence; it is not a native Q1
peer transport protocol or a claim of production network security. Do not expose
fixture-signed RPC to LAN or Internet interfaces, enable passwordless root access,
or disable host-key checking. Fixture signing seeds are public: all participating
operators must be trusted for this crash-only experiment. An SSH tunnel cannot
turn known fixture keys into Byzantine or permissionless authentication.

Bootstrap is a deployment role distributing the pinned manifest/source hash and
running the single controller; it has no committee vote merely for being bootstrap.
Protocol voters are role IDs 0/1/2, producers A/B 3/4, optional ordinary learner 5.
If the learner is absent, its fixture identity still signs controller messages.
There must be exactly one controller journal. Bootstrap loss stops test control,
not consensus verification; relocating that controller requires preserving its
journal and ensuring the old one is stopped. No automatic takeover is provided.

Prefer separate physical hosts for all roles. Three hosts can initially hold:
A = bootstrap + producer A + voter1; B = producer B + voter2; C = voter3 + learner.
Loss of A/B combines producer and voter failure. Failure of C loses one voter
and the learner. Loss of any two voter hosts stops finalization. Two VMs on one
machine are one physical failure domain. Shared power/router/ISP remain correlated
even with three computers; record them separately. The deploy planner reports
configured common failure domains and never certifies physical independence.

## Minimum prerequisites and planning budgets

The exact smallest supported hardware is **not benchmarked**. Planning floors
below are admission-to-testing budgets, not performance guarantees.

| Role | Software prerequisite | Initial planning budget | Durable data / dependence |
|---|---|---|---|
| Bootstrap/controller | POSIX macOS/Linux, Python 3.13, compatible built q1-testnet, SSH client | 1 CPU, 1 GiB free RAM, 1 GiB free disk with binaries already built | One controller journal; manifest; no finality privilege |
| Each producer | Same runtime, working local fsync/rename/file locks; SSH client/server access | 1 CPU, 1 GiB free RAM, 1 GiB free disk | Its own journal; never share one directory across roles |
| Each voter | Same runtime and durable local storage; SSH routing to peers | 1 CPU, 1 GiB free RAM, 1 GiB free disk | Reservation must survive restart; no restored old snapshots |
| Optional learner | Same runtime | 1 CPU, 1 GiB free RAM, 1 GiB free disk | Authenticated full certificate replay |
| Source build host | Pinned Rust 1.97.1 toolchain, linker; Node 20.17.0 for reference checks | Plan 2 CPUs, 4 GiB available RAM, 10 GiB free disk | Measure actual build peaks before downsizing |

Co-located role budgets add; measure RSS, free disk and response latency before
admitting more roles. Existing research bounds are 16 blocks and 64 KiB journal/
network frames; rotate to a **new** manifest for a fresh experiment, never erase
reservations to make an existing chain proceed. No SSD/HDD brand is required.
Old macOS must pass toolchain/runtime checks before assignment. Native Windows
is unsupported by the current POSIX runner; a compatible Linux environment needs
separate confirmation and must be counted as sharing the Windows machine's failure
domain. Do not reinstall any device merely to satisfy this plan.

## Prepare without connecting or spending

From the reviewed source root:

```sh
cargo build --locked -p q1-testnet
mkdir -p .testnet-multihost
python3 research/testnet_multihost_v0/deploy.py template --output .testnet-multihost/topology.json
```

The template contains illustrative roles only, all hosts `confirmed=false`; it
must fail validation until the human inventory and SSH targets are filled in.
Create the nonce once, then distribute the identical topology privately. Hostnames,
addresses, SSH aliases and raw logs are internal operational data; generated files
are ignored. Pin the source commit, config SHA-256 and backend build on each host.
Manually verify SSH server fingerprints over an existing trusted channel, provision
only the selected user/key and restricted local forwarding, and confirm no public
application port is exposed. No password/private key belongs in the topology.

```sh
python3 research/testnet_multihost_v0/deploy.py plan --config .testnet-multihost/topology.json
```

This prints per-host `ssh -N -T -L ...` commands with strict host checking, batch
mode and forwarding-failure detection. It does not connect, allocate or edit a
firewall. Start each listed tunnel in a supervised terminal on its assigned host
only after access is confirmed. Tunnel ports must be unused; forwarding failure
is a failure, not permission to widen a bind address. Each peer receives one local
port numbered base_port + role ID. Every participating host must reach the assigned
SSH peers directly or through an explicitly reviewed existing SSH configuration.
Keep SSH keys/agent forwarding separate; the generated commands do not enable it.

## Launch, transfer, catch up and stop

On each assigned host, start each role with its own persistent directory; example
arguments below are placeholders from the three-host template:

```sh
python3 research/testnet_multihost_v0/deploy.py serve --config .testnet-multihost/topology.json --host host-a --role voter1 --directory .testnet-multihost/voter1
python3 research/testnet_multihost_v0/deploy.py serve --config .testnet-multihost/topology.json --host host-a --role producer_a --directory .testnet-multihost/producer_a
```

Use separate terminals/process supervision. Start all voters and both producers
before submitting work. On the bootstrap host, through its own tunnel map:

```sh
python3 research/testnet_multihost_v0/deploy.py status --config .testnet-multihost/topology.json --host host-a --role voter1 --directory .testnet-multihost/controller
python3 research/testnet_multihost_v0/deploy.py run --config .testnet-multihost/topology.json --host host-a --role producer_a --directory .testnet-multihost/controller --ballot 0 --amount 10
```

Height h, ballot b selects `[producer_a,producer_b][(h-1+b)%2]`. On observed failure,
first obtain and validate reachable nodes' histories; synchronize the replacement
before advancing the ballot. A timeout alone never proves a peer dead. Use the next
eligible producer and a ballot above every known promise; prepare/adoption preserves
the previously accepted value. If the old producer returns, it must catch up first.

`status` independently replays the returned history with Rust before reporting it.
Save a status result as JSON; extract its `history` array into a separate JSON file:

```sh
python3 -c 'import json; print(json.dumps(json.load(open("status.json"))["history"]))' > history.json
python3 research/testnet_multihost_v0/deploy.py sync --config .testnet-multihost/topology.json --host host-a --role voter1 --directory .testnet-multihost/controller --history history.json
```

The controller and receiver both validate chain, signatures, certificates, transfers
and StateRoot. A peer is not trusted merely because SSH connected. Conflicting local
finality fails closed. No deletion/reset of a poisoned journal is an approved recovery.
Manual sync is required in this deployment wrapper; it does not promise unattended
peer discovery or retry. Stop only the test processes/tunnels, preserving journals.

## Real multi-host failure acceptance matrix

Run baseline signed transfer first. Capture manifest/source hashes, host role map,
process/host identifiers (operational evidence only), reachability, finality height,
actual ledger root, balances/pool/supply and certificate voters before/after each
case. Save independent OS uptime/restart evidence on each host; sanitize identifiers
before publishing. Local process kills cannot substitute for physical host-loss tests.

| Case | Fault and expected acceptance | Required evidence |
|---|---|---|
| Producer host disappears | Stop isolated test host/network access; next ballot adopts chosen work with two reachable voters | Actual host loss, correct proposer, certificate + equal surviving roots |
| Voter host disappears | Two surviving fixed voters progress; membership unchanged | Certificate contains two authorized voters; no reconfiguration |
| Ordinary node disappears | Finality unaffected; learner replays on return | Before/after learner root and history |
| Separated producer + voter failure | Lose their distinct hosts; progress only if a producer and two voters remain | Role/failure-domain map, survivor quorum; otherwise explicit safe halt |
| Host restart | Controlled OS restart after durable vote/finality; recover same journal | Uptime boundary; no reservation reversal or extra fee |
| Network interruption | Stop only test tunnel(s); minority cannot finalize; reconnect and sync | Link events, no minority fee/state change, eventual common root |
| Stale return | Replay missed certificates before new proposal participation | Old-height/ballot rejected; recovered state matches |
| Conflict attempt | Two valid same-slot transfer candidates cannot replace reserved/chosen work | Rejection before/after restart; no two conflicting finalized values |
| Malformed messages | Invalid frame length/JSON/signature/body/root fails closed | Error and unchanged journal/ledger, then valid request succeeds |
| Delayed peer | Controlled delay only on test connection; two timely voters may progress | Measured RTT/timeout, late reply handling; no latency-to-vote weight |
| Untrusted catch-up peer | Offer altered/foreign/duplicate-vote/conflicting history | Rejection with unchanged state, then valid catch-up succeeds |

Repeat the existing eight crash scenarios as preflight, then perform this matrix
on confirmed hosts. Local malformed-message and certificate tests provide preflight
coverage; **all real host-loss/SSH/network-delay results remain NOT RUN** until
hardware and access exist. Collect evidence, do not infer it from configuration.

A completed run must demonstrate a signed transfer, producer failover, restored
nodes and common full StateRoot; check supply conservation and fee=1 throughout.
Before hosting more than experimental fixtures, transport/admission/key custody
and the broader failure model need separate design and authorization.
