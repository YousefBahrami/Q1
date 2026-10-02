#!/usr/bin/env python3
"""Exercise the documented foreground supervisor, wallet, signing and transfer CLI."""
import os
from pathlib import Path
import stat
import subprocess
import sys
import tempfile
import time
from localnet import ROOT, binaries, statuses


def run():
    node, _ = binaries()
    with tempfile.TemporaryDirectory(prefix="q1-cli-") as temp:
        directory = Path(temp) / "network with spaces"
        log = Path(temp) / "console.log"
        with log.open("w") as output:
            process = subprocess.Popen([sys.executable, str(ROOT / "scripts/localnet.py"), "run", "--dir", str(directory)], stdin=subprocess.PIPE, stdout=output, stderr=output, text=True)
            def command(*args, success=True):
                result = subprocess.run(list(map(str,args)), capture_output=True, text=True)
                if (result.returncode == 0) != success:
                    raise AssertionError(result.stdout + result.stderr)
                return result.stdout.strip()
            def wait(height, online=4):
                deadline = time.monotonic() + 30
                while time.monotonic() < deadline:
                    assert process.poll() is None, log.read_text()
                    try:
                        rows = statuses(directory)
                        active = [r for r in rows if r["online"]]
                        if len(active) == online and all(r["height"] == height for r in active):
                            assert len({r["state_root"] for r in active}) == 1
                            assert all(r["reward_pool"] == height and r["total_supply"] == 1000 for r in active)
                            return rows
                    except FileNotFoundError:
                        pass
                    time.sleep(0.1)
                raise AssertionError(log.read_text())
            try:
                wait(0)
                wallet = directory / "alice.key"
                address = command(node, "wallet-new", wallet)
                assert address == command(node,"wallet-address",wallet)
                assert len(wallet.read_bytes()) == 32
                original = wallet.read_bytes()
                command(node,"wallet-new",wallet,success=False)
                assert wallet.read_bytes() == original
                if os.name == "posix":
                    assert stat.S_IMODE(wallet.stat().st_mode) == 0o600
                    wallet.chmod(0o644)
                    command(node,"wallet-address",wallet,success=False)
                    wallet.chmod(0o600)
                script = ROOT / "scripts/localnet.py"
                command(sys.executable,script,"transfer","--dir",directory,"--amount",10,"--to",address)
                wait(1)
                process.stdin.write("stop voter2\n"); process.stdin.flush()
                wait(1,3)
                recipient=(directory / "fixture/recipient.address").read_text()
                command(sys.executable,script,"transfer","--dir",directory,"--amount",3,"--key-file",wallet,"--to",recipient)
                wait(2,3)
                process.stdin.write("start voter2\n");process.stdin.flush()
                final=wait(2)
                command(sys.executable,script,"status","--dir",directory)
                # Replaying a saved signed transfer must not charge or advance state.
                signed=next((directory / "transfers").glob("*.cbor"))
                command(sys.executable,script,"submit","--dir",directory,"--file",signed,success=False)
                assert statuses(directory)==final
                process.stdin.write("quit\n");process.stdin.flush()
                process.wait(timeout=10)
                assert process.returncode==0
            finally:
                if process.poll() is None:
                    process.stdin.close()
                    try:
                        process.wait(timeout=10)
                    except subprocess.TimeoutExpired:
                        process.kill();process.wait(timeout=10)
            assert all(not row["online"] for row in statuses(directory))
    print("PASS: README CLI wallet creation/permissions/no overwrite, funded transfer, wallet spend, voter stop/start/catch-up, replay rejection, owned-process cleanup")


if __name__ == "__main__":
    run()
