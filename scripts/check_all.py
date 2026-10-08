#!/usr/bin/env python3
"""Run the same deterministic checks locally and in CI; stop on first failure."""
from pathlib import Path
import os
import shutil
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parent.parent


def main():
    cargo = shutil.which("cargo") or str(Path.home() / ".cargo/bin/cargo")
    environment = os.environ.copy()
    environment["RUSTDOCFLAGS"] = "-D warnings"
    environment.setdefault("PROPTEST_RNG_SEED", "20261001")

    def run(*command, cwd=ROOT, stdout=None):
        print("CHECK:", " ".join(map(str, command)), flush=True)
        subprocess.run(command, cwd=cwd, env=environment, stdout=stdout, check=True)

    run(cargo, "fmt", "--all", "--", "--check")
    run(cargo, "clippy", "--workspace", "--all-targets", "--all-features", "--locked", "--", "-D", "warnings")
    run(cargo, "test", "--workspace", "--all-targets", "--all-features", "--locked")
    run(cargo, "build", "--workspace", "--all-targets", "--locked")
    run(cargo, "run", "--quiet", "--locked", "-p", "q1-localnet", "--example", "transfer")
    run(cargo, "doc", "--workspace", "--no-deps", "--locked")
    with tempfile.TemporaryDirectory(prefix="q1-vectors-") as temporary:
        output = Path(temporary) / "rust.tsv"
        with output.open("w") as stream:
            run(cargo, "run", "--quiet", "--locked", "-p", "q1-protocol-types", "--example", "protocol_vectors", stdout=stream)
        run("node", "scripts/protocol_reference.mjs", str(output))
        run(sys.executable, "scripts/protocol_reference.py", str(output))
        local_output = Path(temporary) / "localnet.tsv"
        with local_output.open("w") as stream:
            run(cargo, "run", "--quiet", "--locked", "-p", "q1-localnet", "--example", "state_vectors", stdout=stream)
        run(sys.executable, "scripts/localnet_reference.py", str(local_output))
    run(sys.executable, "scripts/localnet_acceptance.py")
    run(sys.executable, "scripts/localnet_cli_test.py")
    research = ROOT / "research/pre_m1_conformance"
    # Separate build output prevents stale historical research artifacts from
    # masking a fresh compilation (source and lockfile are still the originals).
    research_target = ROOT / "target/pre-m1-checks"
    run(cargo, "test", "--locked", "--target-dir", str(research_target), cwd=research)
    run(cargo, "run", "--locked", "--target-dir", str(research_target), cwd=research)
    run("node", str(research / "node_conformance.js"))
    run(sys.executable, str(research / "python_vectors.py"))
    # Isolated resource-lab tests: no mining benchmark or consensus activation.
    run(sys.executable, "-m", "unittest", "discover", "-s", "research/mining_delay_v0", "-p", "test_*.py")
    run(sys.executable, "-m", "unittest", "discover", "-s", "research/mining_delay_v1", "-p", "test_*.py")
    run(sys.executable, "-m", "unittest", "discover", "-s", "research/testnet_failover_v0", "-p", "test_*.py")
    with tempfile.TemporaryDirectory(prefix="q1-testnet-crash-check-") as temporary:
        run(sys.executable, "research/testnet_failover_v0/run.py", "--output", str(Path(temporary) / "evidence"))
    run(sys.executable, "research/mining_delay_v1/replay.py", "docs/reports/research/mining-delay-v1-2026-10-05")
    run(sys.executable, "-m", "unittest", "discover", "-s", "research/useful_resource_v2", "-p", "test_*.py")
    run(sys.executable, "-m", "unittest", "discover", "-s", "research/testnet_ledger_v0", "-p", "test_*.py")
    with tempfile.TemporaryDirectory(prefix="q1-ledger-crash-check-") as temporary:
        evidence = Path(temporary) / "evidence"
        run(sys.executable, "research/testnet_ledger_v0/run.py", "--output", str(evidence))
        run(sys.executable, "research/testnet_ledger_v0/run.py", "--replay", str(evidence / "acceptance.json"))
    run(sys.executable, "-m", "unittest", "discover", "-s", "research/resource_uniqueness_v0", "-p", "test_*.py")
    run(sys.executable, "research/resource_uniqueness_v0/replay.py", "docs/reports/research/resource-uniqueness-v0-2026-10-06")
    run(sys.executable, "research/resource_uniqueness_v0/concurrent_audits.py", "--replay", "docs/reports/research/resource-concurrent-v0-2026-10-06")
    run(sys.executable, "-m", "unittest", "discover", "-s", "research/testnet_multihost_v0", "-p", "test_*.py")
    run(sys.executable, "research/testnet_multihost_v0/smoke.py")
    run(sys.executable, "research/testnet_native_v0/vectors.py")
    run(sys.executable, "-m", "unittest", "discover", "-s", "research/testnet_native_v0", "-p", "test_*.py")
    with tempfile.TemporaryDirectory(prefix="q1-native-check-") as temporary:
        run(sys.executable, "research/testnet_native_v0/run.py", "--output", str(Path(temporary) / "evidence"))
    run(sys.executable, "scripts/check_documentation.py")
    if (ROOT / ".git").exists():
        run("git", "diff", "--check")
    else:
        print("Source archive: Git diff check skipped; all source/build/runtime checks executed.", flush=True)
    print("All local checks passed, including real four-node LOCALNET v0 acceptance.", flush=True)


if __name__ == "__main__":
    try:
        main()
    except (subprocess.CalledProcessError, OSError) as error:
        print(f"CHECK FAILED: {error}", file=sys.stderr)
        sys.exit(1)
