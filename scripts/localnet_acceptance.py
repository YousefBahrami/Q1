#!/usr/bin/env python3
"""Real four-process LOCALNET v0 acceptance. Only public fixture keys, loopback TCP.

Run after cargo build --workspace --all-targets; no third-party Python packages.
Creates isolated temporary data and kills only the subprocesses it started.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import socket
import struct
import subprocess
import tempfile
import time

ROOT = Path(__file__).resolve().parent.parent
LIMIT = 16 * 1024 * 1024


def encode(value):
    def head(major, size):
        if size < 24:
            return bytes([major * 32 + size])
        for extra, length in [(24, 1), (25, 2), (26, 4), (27, 8)]:
            if size < 1 << (8 * length):
                return bytes([major * 32 + extra]) + size.to_bytes(length, "big")
        raise ValueError("integer too large")
    if value is None:
        return b"\xf6"
    if isinstance(value, bool):
        return b"\xf5" if value else b"\xf4"
    if isinstance(value, int):
        return head(0, value)
    if isinstance(value, bytes):
        return head(2, len(value)) + value
    if isinstance(value, str):
        raw = value.encode()
        return head(3, len(raw)) + raw
    if isinstance(value, list):
        return head(4, len(value)) + b"".join(encode(v) for v in value)
    raise ValueError("unsupported CBOR")


def decode(data):
    def read(offset):
        first = data[offset]
        offset += 1
        if first in (244, 245, 246):
            return {244: False, 245: True, 246: None}[first], offset
        major, size = first >> 5, first & 31
        if size >= 24:
            length = {24: 1, 25: 2, 26: 4, 27: 8}[size]
            size = int.from_bytes(data[offset:offset + length], "big")
            offset += length
        if major == 0:
            return size, offset
        if major in (2, 3):
            raw = data[offset:offset + size]
            assert len(raw) == size
            return (raw if major == 2 else raw.decode()), offset + size
        if major == 4:
            values = []
            for _ in range(size):
                value, offset = read(offset)
                values.append(value)
            return values, offset
        raise ValueError("unsupported CBOR")
    value, end = read(0)
    assert end == len(data) and encode(value) == data, "noncanonical CBOR"
    return value


def digest(domain, payload):
    return hashlib.sha256(b"Q1DS" + b"\x00\x01" + domain.to_bytes(2, "big") + len(payload).to_bytes(8, "big") + payload).digest()


def rpc(port, genesis, op, payload=b""):
    def read_exact(stream, length):
        parts = bytearray()
        while len(parts) < length:
            chunk = stream.recv(length - len(parts))
            if not chunk:
                raise ConnectionError("early EOF")
            parts.extend(chunk)
        return bytes(parts)
    with socket.create_connection(("127.0.0.1", port), timeout=20) as stream:
        raw = encode([1, genesis, op, payload])
        stream.sendall(struct.pack(">I", len(raw)) + raw)
        size = struct.unpack(">I", read_exact(stream, 4))[0]
        assert size <= LIMIT
        version, response_genesis, code, body = decode(read_exact(stream, size))
        assert version == 1 and response_genesis == genesis
        if code == 255:
            raise RuntimeError(body.decode())
        assert code == op
        return body


def run(directory, binary, fixture_binary):
    fixture = directory / "fixture"
    subprocess.run([str(fixture_binary), str(fixture)], check=True)
    genesis = bytes.fromhex((fixture / "genesis.id").read_text())
    # Hold all four sockets together to choose distinct ephemeral loopback ports.
    sockets = [socket.socket() for _ in range(4)]
    for stream in sockets:
        stream.bind(("127.0.0.1", 0))
    ports = [s.getsockname()[1] for s in sockets]
    for stream in sockets:
        stream.close()
    roles = ["producer", "voter0", "voter1", "voter2"]
    processes = {}
    logs = []
    evidence = {"profile": "LOCALNET_V0", "nodes": 4, "quorum": "2-of-3", "genesis": genesis.hex()}

    def start(index):
        role = roles[index]
        log = (directory / f"{role}.log").open("ab")
        logs.append(log)
        command = [str(binary), "--genesis", str(fixture / "genesis.cbor"),
                   "--key-file", str(fixture / f"{role}.key"),
                   "--participant", (fixture / f"{role}.id").read_text(),
                   "--data-dir", str(directory / role), "--listen", f"127.0.0.1:{ports[index]}",
                   "--producer", f"127.0.0.1:{ports[0]}", "--voters",
                   ",".join(f"127.0.0.1:{p}" for p in ports[1:])]
        processes[index] = subprocess.Popen(command, stdout=log, stderr=log)

    def status(index):
        process = processes[index]
        assert process.poll() is None, f"{roles[index]} exited {process.returncode}"
        return decode(rpc(ports[index], genesis, 0))

    def await_height(indices, height):
        deadline = time.monotonic() + 40
        last = None
        while time.monotonic() < deadline:
            try:
                states = [status(i) for i in indices]
                if all(s[0] == height for s in states):
                    assert all(s == states[0] for s in states), "nodes disagree on tip/root/state"
                    return states[0]
            except (ConnectionError, OSError) as error:
                last = error
            time.sleep(0.1)
        raise AssertionError(f"height {height} timeout: {last}")

    def assert_state(state, height):
        assert state[0] == height
        expected = (fixture / f"state{height}.cbor").read_bytes()
        assert state[3] == expected, "process state differs from independent execution"
        assert state[2].hex() == (fixture / f"root{height}.hex").read_text()
        assert state[2] == digest(0x15, state[3]), "independent Python StateRoot differs"
        snapshot = decode(state[3])
        balances = sorted(int.from_bytes(row[2], "big") for row in snapshot[6])
        nonces = sorted(row[3] for row in snapshot[6])
        pool, supply = int.from_bytes(snapshot[5], "big"), int.from_bytes(snapshot[4], "big")
        assert pool == height and supply == 1000
        assert balances == sorted([1000 - height * 11, height * 10])
        assert nonces == [0, height]
        assert sum(balances) + pool == supply

    def reject(index, op, payload):
        try:
            rpc(ports[index], genesis, op, payload)
        except RuntimeError:
            return
        raise AssertionError("invalid operation accepted")

    try:
        for index in range(4):
            start(index)
        initial = await_height(range(4), 0)
        # Invalid signature must not reserve a vote, charge a fee, or change state.
        invalid = bytearray((fixture / "tx0.cbor").read_bytes())
        invalid[-1] ^= 1
        reject(1, 1, bytes(invalid))
        assert await_height(range(4), 0) == initial
        rpc(ports[1], genesis, 1, (fixture / "tx0.cbor").read_bytes())
        first = await_height(range(4), 1)
        assert_state(first, 1)
        reject(0, 1, (fixture / "tx0.cbor").read_bytes())
        assert await_height(range(4), 1) == first
        old_pid = processes[3].pid
        processes[3].kill()
        processes[3].wait(timeout=10)
        assert processes[3].returncode != 0
        rpc(ports[2], genesis, 1, (fixture / "tx1.cbor").read_bytes())
        second = await_height(range(3), 2)
        assert_state(second, 2)
        # Inspect actual signed certificate: exactly the two online authorized voters.
        block = decode(rpc(ports[0], genesis, 4, encode(2)))
        votes = block[2][4]
        assert len(votes) == 2
        offline_id = bytes.fromhex((fixture / "voter2.id").read_text())
        assert offline_id not in [v[0][8] for v in votes]
        evidence["offline_progress"] = {"height": 2, "votes": 2, "stopped_pid": old_pid}
        start(3)
        assert processes[3].pid != old_pid
        assert await_height(range(4), 2) == second
        rpc(ports[3], genesis, 1, (fixture / "tx2.cbor").read_bytes())
        third = await_height(range(4), 3)
        assert_state(third, 3)
        # One online voter cannot finalize: no state or fee changes.
        for index in (2, 3):
            processes[index].kill()
            processes[index].wait(timeout=10)
        reject(0, 1, (fixture / "tx3.cbor").read_bytes())
        assert await_height(range(2), 3) == third
        # The only voter reserved this unfinalized proposal. Kill it to prove
        # durable anti-equivocation, then send a different valid signed proposal.
        processes[1].kill()
        processes[1].wait(timeout=10)
        start(1)
        assert await_height(range(2), 3) == third
        reject(1, 2, (fixture / "conflict3.proposal").read_bytes())
        assert await_height(range(2), 3) == third
        evidence["restart_equivocation_rejected"] = True
        for index in (2, 3):
            start(index)
        assert await_height(range(4), 3) == third
        # Retry the exact proposal reserved by the single voter; no equivocation.
        rpc(ports[3], genesis, 1, (fixture / "tx3.cbor").read_bytes())
        final = await_height(range(4), 4)
        assert_state(final, 4)
        evidence["one_of_three_rejected_without_state_change"] = True
        evidence["final_height"] = 4
        evidence["final_state_root"] = final[2].hex()
        evidence["reward_pool"] = 4
        evidence["total_supply"] = 1000
        evidence["restart_catch_up"] = True
        evidence["post_restart_transfer"] = True
        # Recover all processes from disk, including the producer, at committed height.
        for process in processes.values():
            if process.poll() is None:
                process.kill()
                process.wait(timeout=10)
        for index in range(4):
            start(index)
        assert await_height(range(4), 4) == final
        evidence["all_processes_recovered"] = True
        (directory / "acceptance.json").write_text(json.dumps(evidence, indent=2) + "\n")
        print(json.dumps(evidence, indent=2), flush=True)
        print("PASS: four processes, signed transfers, fee=1, atomic rejection, voter kill, 2-of-3 progress, restart, catch-up, identical StateRoot", flush=True)
    except BaseException:
        for role in roles:
            path = directory / f"{role}.log"
            if path.exists():
                print(f"{role} log tail:\n{path.read_text()[-5000:]}", flush=True)
        raise
    finally:
        for process in processes.values():
            if process.poll() is None:
                process.kill()
                process.wait(timeout=10)
        for log in logs:
            log.close()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", type=Path, help="new directory to retain logs, public fixtures and evidence")
    arguments = parser.parse_args()
    target = Path(os.environ.get("CARGO_TARGET_DIR", ROOT / "target")) / "debug"
    if arguments.output_dir:
        arguments.output_dir.mkdir()
        run(arguments.output_dir, target / "q1-node", target / "examples/fixture")
    else:
        with tempfile.TemporaryDirectory(prefix="q1-four-node-") as temporary:
            run(Path(temporary), target / "q1-node", target / "examples/fixture")


if __name__ == "__main__":
    main()
