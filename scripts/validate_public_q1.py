#!/usr/bin/env python3
"""Finite LOCALNET reproduction; no installation, remote nodes or uploads."""
import argparse
import json
import os
from pathlib import Path
import platform
import shutil
import signal
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parent.parent
TOOLCHAIN = "1.97.1"
STATE_ROOT = "b7ec7d47f4cf37d029e74bab02d8bc2aab191f5a5733f92ac4310d919183b087"
STAGES = ("build", "rust_tests", "localnet")


def capture(command):
    return subprocess.check_output(command, cwd=ROOT, text=True,
                                   stderr=subprocess.STDOUT, timeout=30).strip()


def check_evidence(evidence):
    if not isinstance(evidence, dict):
        raise ValueError("Acceptance evidence must be an object")
    expected = {"profile": "LOCALNET_V0", "nodes": 4, "quorum": "2-of-3",
                "final_height": 4, "final_state_root": STATE_ROOT,
                "reward_pool": 4, "total_supply": 1000,
                "restart_catch_up": True, "post_restart_transfer": True,
                "all_processes_recovered": True,
                "restart_equivocation_rejected": True,
                "one_of_three_rejected_without_state_change": True}
    for key, value in expected.items():
        if type(evidence.get(key)) is not type(value) or evidence[key] != value:
            raise ValueError("Acceptance evidence mismatch: " + key)
    progress = evidence.get("offline_progress", {})
    if not isinstance(progress, dict) or progress.get("height") != 2 or progress.get("votes") != 2:
        raise ValueError("Missing two-vote offline progress")


def run_stage(command, log, timeout=1800):
    # A separate group lets interruption/timeout stop only our subprocess tree.
    with log.open("wb") as stream:
        child = subprocess.Popen(command, cwd=ROOT, stdout=stream,
                                 stderr=subprocess.STDOUT, start_new_session=True)
        try:
            return child.wait(timeout=timeout)
        except (KeyboardInterrupt, subprocess.TimeoutExpired):
            try:
                os.killpg(child.pid, signal.SIGINT)
            except ProcessLookupError:
                pass
            try:
                child.wait(timeout=15)
            except subprocess.TimeoutExpired:
                pass
            finally:
                # Also reap descendants if the parent exited before its cleanup.
                try:
                    os.killpg(child.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                child.wait()
            raise


def validate(output):
    report = {"schema": 1, "scope": "LOCALNET build, Rust tests, four-process acceptance",
              "result": "UNSUPPORTED", "failure_stage": "preflight",
              "environment": {"os": platform.system(), "os_version": platform.release(),
                              "architecture": platform.machine(), "python": platform.python_version()},
              "q1_commit": None, "working_tree_changed": None,
              "stages": {name: "SKIPPED" for name in STAGES},
              "localnet_completed": False, "restart_catch_up": False,
              "not_tested": ["full check_all.py", "native transport", "multi-host", "Public Testnet"],
              "independence": "Not determined by this script; maintainer/CI runs are not independent evidence"}
    exit_code = 2
    try:
        if os.name != "posix" or platform.system() not in ("Linux", "Darwin"):
            raise ValueError("UNSUPPORTED: use macOS or Linux; native Windows is not supported")
        if sys.version_info < (3, 11) or sys.flags.optimize:
            raise ValueError("UNSUPPORTED: Python >=3.11 required, with assertions enabled (no -O/PYTHONOPTIMIZE)")
        if not shutil.which("git"):
            raise ValueError("UNSUPPORTED: install Git manually first")
        report["q1_commit"] = capture(["git", "rev-parse", "HEAD"])
        report["working_tree_changed"] = bool(capture(["git", "status", "--porcelain", "--untracked-files=normal"]))
        rustup = shutil.which("rustup") or str(Path.home() / ".cargo/bin/rustup")
        # rustup run does not install a missing toolchain. No automatic installer.
        rust = capture([rustup, "run", TOOLCHAIN, "rustc", "--version"])
        report["environment"]["rust"] = rust
        if not rust.startswith("rustc " + TOOLCHAIN + " "):
            raise ValueError("UNSUPPORTED: wrong Rust toolchain")
        cargo = [rustup, "run", TOOLCHAIN, "cargo"]
        commands = {
            "build": cargo + ["build", "--workspace", "--all-targets", "--locked"],
            "rust_tests": cargo + ["test", "--workspace", "--all-targets", "--all-features", "--locked"],
            "localnet": [sys.executable, "scripts/localnet_acceptance.py", "--output-dir", str(output / "localnet")],
        }
        report["result"] = "FAIL"
        exit_code = 1
        for stage in STAGES:
            report["failure_stage"] = stage
            report["stages"][stage] = "FAIL"
            print("RUN:", stage, "(log:", output / (stage + ".log"), ")", flush=True)
            code = run_stage(commands[stage], output / (stage + ".log"))
            if code != 0:
                report["error"] = "Stage exited with status " + str(code)
                return exit_code
            if stage == "localnet":
                evidence = json.loads((output / "localnet/acceptance.json").read_text())
                check_evidence(evidence)
                report["localnet_completed"] = True
                report["restart_catch_up"] = True
                report["final_state_root"] = evidence["final_state_root"]
            report["stages"][stage] = "PASS"
        report["result"] = "PASS"
        report["failure_stage"] = None
        exit_code = 0
    except KeyboardInterrupt:
        report["result"] = "FAIL"
        report["error"] = "Interrupted by operator; unfinished stages are not passes"
        exit_code = 130
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        report["error"] = str(error)
    finally:
        (output / "summary.json").write_text(json.dumps(report, indent=2) + "\n")
        print(report["result"], json.dumps(report["stages"]), flush=True)
        print("Evidence:", output, "— review/redact before sharing; no upload performed", flush=True)
    return exit_code


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", type=Path, help="new directory; never overwrite existing evidence")
    args = parser.parse_args()
    try:
        if args.output_dir:
            output = args.output_dir.resolve()
            output.mkdir(mode=0o700)
        else:
            output = Path(tempfile.mkdtemp(prefix="q1-public-validation-"))
    except OSError as error:
        print("FAIL: cannot create fresh evidence directory:", error, file=sys.stderr)
        return 1
    return validate(output)


if __name__ == "__main__":
    sys.exit(main())
