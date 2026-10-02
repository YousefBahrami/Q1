#!/usr/bin/env python3
"""Foreground four-node LOCALNET and explicit local test wallet/transfer commands."""
import argparse
import json
import os
from pathlib import Path
import socket
import subprocess
import sys
import time
import uuid
from localnet_acceptance import ROOT, decode, rpc

WARNING = "TEST ONLY — NEVER USE FOR VALUE-BEARING NETWORKS"
ROLES = ["producer", "voter0", "voter1", "voter2"]


def binaries():
    target = Path(os.environ.get("CARGO_TARGET_DIR", ROOT / "target"))
    if not target.is_absolute():
        target = ROOT / target
    return target / "debug/q1-node", target / "debug/examples/fixture"


def configuration(directory):
    config = json.loads((directory / "network.json").read_text())
    ports = config["ports"]
    if len(ports) != 4 or len(set(ports)) != 4 or any(type(p) is not int or not 0 < p < 65536 for p in ports):
        raise ValueError("invalid local endpoints")
    return config, bytes.fromhex((directory / "fixture/genesis.id").read_text())


def statuses(directory):
    config, genesis = configuration(directory)
    rows = []
    for role, port in zip(ROLES, config["ports"]):
        try:
            height, tip, root, raw = decode(rpc(port, genesis, 0))
            snapshot = decode(raw)
            rows.append({"role": role, "height": height, "state_root": root.hex(),
                         "reward_pool": int.from_bytes(snapshot[5], "big"),
                         "total_supply": int.from_bytes(snapshot[4], "big"), "online": True})
        except (OSError, RuntimeError, ConnectionError) as error:
            rows.append({"role": role, "online": False, "error": str(error)})
    return rows


def wait_ready(directory, processes):
    deadline = time.monotonic() + 30
    while time.monotonic() < deadline:
        for role, process in processes.items():
            if process.poll() is not None:
                raise RuntimeError(f"{role} exited; inspect {directory / (role + '.log')}")
        rows = statuses(directory)
        if all(r["online"] for r in rows):
            return
        time.sleep(0.1)
    raise RuntimeError("node startup timed out")


def run_network(directory):
    node, fixture = binaries()
    if not node.is_file() or not fixture.is_file():
        raise RuntimeError("first run: cargo build --workspace --all-targets --locked")
    directory.mkdir(parents=True, exist_ok=True)
    # The foreground supervisor owns only children it starts. No stored PID is killed.
    import fcntl
    with (directory / "supervisor.lock").open("a+b") as lock:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        if not (directory / "network.json").exists():
            if (directory / "fixture").exists():
                raise RuntimeError("incomplete setup: use a new directory; existing evidence is retained")
            subprocess.run([str(fixture), str(directory / "fixture")], check=True)
            sockets = [socket.socket() for _ in range(4)]
            try:
                for stream in sockets:
                    stream.bind(("127.0.0.1", 0))
                ports = [s.getsockname()[1] for s in sockets]
            finally:
                for stream in sockets:
                    stream.close()
            (directory / "network.json").write_text(json.dumps({"profile": "LOCALNET_V0", "ports": ports}, indent=2) + "\n")
        config, _ = configuration(directory)
        ports = config["ports"]
        processes, logs = {}, []

        def start(role):
            if role not in ROLES:
                raise ValueError("expected producer or voter0/voter1/voter2")
            if role in processes and processes[role].poll() is None:
                raise ValueError(f"{role} already running")
            index = ROLES.index(role)
            log = (directory / f"{role}.log").open("ab")
            logs.append(log)
            source = directory / "fixture"
            processes[role] = subprocess.Popen([str(node), "--genesis", str(source / "genesis.cbor"),
                "--key-file", str(source / f"{role}.key"), "--participant", (source / f"{role}.id").read_text(),
                "--data-dir", str(directory / role), "--listen", f"127.0.0.1:{ports[index]}",
                "--producer", f"127.0.0.1:{ports[0]}", "--voters", ",".join(f"127.0.0.1:{p}" for p in ports[1:])],
                stdout=log, stderr=log)

        try:
            for role in ROLES:
                start(role)
            wait_ready(directory, processes)
            print(WARNING, flush=True)
            print("Ready. Console commands: status | stop voter2 | start voter2 | quit", flush=True)
            print("Keep this terminal open. Use scripts/localnet.py transfer/status in another terminal.", flush=True)
            for line in sys.stdin:
                command = line.strip().split()
                try:
                    if command == ["quit"]:
                        break
                    if command == ["status"]:
                        print(json.dumps(statuses(directory), indent=2), flush=True)
                    elif len(command) == 2 and command[0] == "stop" and command[1] in ROLES[1:]:
                        process = processes[command[1]]
                        if process.poll() is not None:
                            raise ValueError("voter already stopped")
                        process.kill()
                        process.wait(timeout=10)
                        print(f"Stopped {command[1]}", flush=True)
                    elif len(command) == 2 and command[0] == "start" and command[1] in ROLES[1:]:
                        start(command[1])
                        print(f"Started {command[1]}; catch-up runs automatically", flush=True)
                    else:
                        print("Commands: status | stop voter0/1/2 | start voter0/1/2 | quit", flush=True)
                except (ValueError, OSError) as error:
                    print(f"Command failed: {error}", flush=True)
        finally:
            for process in processes.values():
                if process.poll() is None:
                    process.terminate()
                    try:
                        process.wait(timeout=10)
                    except subprocess.TimeoutExpired:
                        process.kill()
                        process.wait(timeout=10)
            for log in logs:
                log.close()


def transfer(directory, recipient, amount, key_file):
    node, _ = binaries()
    config, genesis_id = configuration(directory)
    manifest = directory / "fixture/genesis.cbor"
    key_file = key_file or directory / "fixture/sender.key"
    recipient = recipient or (directory / "fixture/recipient.address").read_text()
    address = subprocess.check_output([str(node), "wallet-address", str(key_file)], text=True).strip()
    # Resolve the sender using Rust's canonical address rendering, without
    # reimplementing account identity or signature construction in Python.
    status = subprocess.check_output([str(node), "status", str(manifest), f"127.0.0.1:{config['ports'][0]}"], text=True)
    height = int(status.splitlines()[0].split("=", 1)[1])
    nonce = 0
    for line in status.splitlines():
        if line.startswith(f"account={address} "):
            nonce = int(line.rsplit("nonce=", 1)[1])
    directory.joinpath("transfers").mkdir(exist_ok=True)
    output = directory / "transfers" / f"{uuid.uuid4().hex}.cbor"
    subprocess.run([str(node), "transfer-sign", str(manifest), str(key_file), recipient, str(amount), str(nonce), str(height + 1), str(height + 100), str(output)], check=True)
    print(f"Signed transfer saved: {output}", flush=True)
    print("If submission fails, retry this SAME file with: scripts/localnet.py submit --dir DIR --file FILE", flush=True)
    submit(directory, output)


def submit(directory, path):
    node, _ = binaries()
    config, _ = configuration(directory)
    subprocess.run([str(node), "transfer-submit", str(directory / "fixture/genesis.cbor"), f"127.0.0.1:{config['ports'][0]}", str(path)], check=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    for name in ("run", "status", "transfer", "submit"):
        command = commands.add_parser(name)
        command.add_argument("--dir", type=Path, default=ROOT / ".localnet/demo")
        if name == "transfer":
            command.add_argument("--to", help="LOCALNET Bech32m address; defaults to fixture recipient")
            command.add_argument("--amount", type=int, required=True, help="positive integer base units")
            command.add_argument("--key-file", type=Path, help="defaults to publicly known fixture sender")
        if name == "submit":
            command.add_argument("--file", type=Path, required=True)
    args = parser.parse_args()
    directory = args.dir.resolve()
    if args.command == "run":
        run_network(directory)
    elif args.command == "status":
        print(json.dumps(statuses(directory), indent=2))
    elif args.command == "transfer":
        transfer(directory, args.to, args.amount, args.key_file)
    else:
        submit(directory, args.file)


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        pass
    except (OSError, ValueError, RuntimeError, subprocess.CalledProcessError) as error:
        print(f"localnet: {error}", file=sys.stderr)
        sys.exit(1)
