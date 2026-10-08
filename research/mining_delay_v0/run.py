#!/usr/bin/env python3
"""Safe finite benchmark/reward runner and independent evidence verification."""
import argparse
from datetime import datetime, timezone
import gc
import hashlib
import json
import platform
import resource
import shutil
import statistics
import sys
import tempfile
import time
from pathlib import Path

from resource_lab import (CHUNK_BYTES, LabError, canonical, chunk, commitment,
                          create_dataset, digest, prove, read_json, verify)
from rewards import simulate

HERE = Path(__file__).resolve().parent
SIZES = {"small": 1, "medium": 4, "large": 16}


def write_json(path, value):
    with path.open("x", encoding="utf-8") as stream:
        json.dump(value, stream, sort_keys=True, indent=2, allow_nan=False)
        stream.write("\n")


def benchmark(output: Path, temporary_parent: Path | None) -> dict:
    sizes = []
    minimum_free = 512 * 1024 * 1024
    scratch_parent = temporary_parent or Path(tempfile.gettempdir())
    if shutil.disk_usage(scratch_parent).free < minimum_free:
        raise LabError("FREE_SPACE_FLOOR")
    for name, mib in SIZES.items():
        with tempfile.TemporaryDirectory(prefix="q1-resource-lab-", dir=scratch_parent) as temp:
            data_path = Path(temp) / "dataset.bin"
            start = time.perf_counter_ns()
            manifest, levels = create_dataset(data_path, mib * 1024**2,
                                             digest("fixture-seed"), digest("fixture-participant"))
            generation_ns = time.perf_counter_ns() - start
            context = dict(profile=manifest["profile"], chain="q1-resource-lab-no-network",
                           height=1, round=0, producer=manifest["participant"],
                           nonce=digest("fixture-challenge", name.encode()).hex(),
                           dataset=commitment(manifest))
            measured = []
            canonical_proofs = set()

            def measure(mode, reader, setup_bytes):
                for trial in range(3):
                    before = resource.getrusage(resource.RUSAGE_SELF)
                    cpu = time.process_time_ns()
                    wall = time.perf_counter_ns()
                    evidence = prove(manifest, levels, context, reader)
                    encoded = canonical(evidence)
                    proof_ns = time.perf_counter_ns() - wall
                    cpu_ns = time.process_time_ns() - cpu
                    after = resource.getrusage(resource.RUSAGE_SELF)
                    wall = time.perf_counter_ns()
                    verified_root = verify(encoded, manifest, context)
                    verify_ns = time.perf_counter_ns() - wall
                    evidence_hash = hashlib.sha256(encoded).hexdigest()
                    canonical_proofs.add(evidence_hash)
                    measured.append(dict(mode=mode, trial=trial, proof_ns=proof_ns,
                                         proof_cpu_ns=cpu_ns, verify_ns=verify_ns,
                                         logical_sample_bytes=64 * CHUNK_BYTES,
                                         setup_read_bytes=setup_bytes if trial == 0 else 0,
                                         sample_file_read_calls=64 if mode == "buffered_file" else 0,
                                         physical_io_bytes=None,
                                         os_input_block_ops_delta=after.ru_inblock - before.ru_inblock,
                                         os_output_block_ops_delta=after.ru_oublock - before.ru_oublock,
                                         evidence_bytes=len(encoded), evidence_sha256=evidence_hash,
                                         response_root=verified_root,
                                         unique_chunks=len({r["index"] for r in evidence["rows"]})))
                    if name == "small" and mode == "buffered_file" and trial == 0:
                        write_json(output / "manifest.json", manifest)
                        write_json(output / "context.json", context)
                        with (output / "evidence.json").open("xb") as stream:
                            stream.write(encoded)

            with data_path.open("rb", buffering=0) as stream:
                def file_read(index):
                    stream.seek(index * CHUNK_BYTES)
                    return stream.read(CHUNK_BYTES)
                measure("buffered_file", file_read, 0)
            ram = data_path.read_bytes()
            measure("ram_substitution", lambda i: ram[i * CHUNK_BYTES:(i + 1) * CHUNK_BYTES], len(ram))
            del ram
            gc.collect()
            data_path.unlink()  # Remove only this run's newly created temporary file.
            seed = bytes.fromhex(manifest["seed"])
            participant = bytes.fromhex(manifest["participant"])
            measure("regenerate_no_dataset_file", lambda i: chunk(seed, participant, i), 0)
            if len(canonical_proofs) != 1:
                raise LabError("SUBSTITUTION_MISMATCH")
            medians = {mode: {key: statistics.median(r[key] for r in measured if r["mode"] == mode)
                              for key in ("proof_ns", "verify_ns", "proof_cpu_ns")}
                       for mode in ("buffered_file", "ram_substitution", "regenerate_no_dataset_file")}
            sizes.append(dict(size=name, dataset_bytes=mib * 1024**2,
                              generated_write_bytes=mib * 1024**2,
                              retained_merkle_digest_bytes=sum(len(row) for row in levels) * 32,
                              python_metadata_overhead_bytes=None,
                              generation_ns=generation_ns, manifest=manifest, context=context,
                              medians=medians, trials=measured,
                              all_substitutions_identical_and_verified=True,
                              dataset_file_removed_before_regeneration=True))
    return dict(sizes=sizes, total_generated_write_bytes=sum(s["dataset_bytes"] for s in sizes),
                minimum_free_bytes=minimum_free,
                cache_policy="buffered after generation; OS cache not evicted; NOT a cold-device benchmark",
                device_type="not independently established; no physical-HDD claim",
                physical_io_energy_wear="not measured; OS block counters are observations, not disk-byte evidence",
                security_limit="public deterministic seed and Merkle metadata permit on-demand regeneration",
                nonce_policy="deterministic fixtures for reproduction; no unpredictable live challenge claim")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    run = commands.add_parser("run")
    run.add_argument("--output", required=True, type=Path, help="new report directory; never overwritten")
    run.add_argument("--scratch-parent", type=Path, help="existing ordinary directory on chosen test volume")
    check = commands.add_parser("verify")
    check.add_argument("--manifest", required=True, type=Path)
    check.add_argument("--context", required=True, type=Path)
    check.add_argument("--evidence", required=True, type=Path)
    args = parser.parse_args()
    if args.command == "verify":
        # Bounded file read, followed by independently supplied expected context.
        from resource_lab import MAX_EVIDENCE_BYTES
        with args.evidence.open("rb") as stream:
            raw = stream.read(MAX_EVIDENCE_BYTES + 1)
        print(verify(raw, read_json(args.manifest), read_json(args.context)))
        return
    args.output.mkdir(mode=0o700, parents=False, exist_ok=False)
    source_hashes = {name: hashlib.sha256((HERE / name).read_bytes()).hexdigest()
                     for name in ("resource_lab.py", "rewards.py", "run.py", "test_lab.py")}
    report = dict(profile="RESEARCH_ONLY_NO_CONSENSUS_OR_MONETARY_EFFECT",
                  recorded_utc=datetime.now(timezone.utc).isoformat(),
                  environment=dict(os=platform.system(), release=platform.release(),
                                   architecture=platform.machine(), python=platform.python_version()),
                  source_sha256=source_hashes, benchmark=benchmark(args.output, args.scratch_parent),
                  simulation=simulate())
    usage = resource.getrusage(resource.RUSAGE_SELF)
    report["peak_rss_bytes_process_lifetime"] = usage.ru_maxrss * (1 if sys.platform == "darwin" else 1024)
    write_json(args.output / "results.json", report)
    names = ("manifest.json", "context.json", "evidence.json", "results.json")
    with (args.output / "SHA256SUMS").open("x") as stream:
        for name in names:
            stream.write(f"{hashlib.sha256((args.output / name).read_bytes()).hexdigest()}  {name}\n")
    print(json.dumps(dict(status="completed", sizes=list(SIZES),
                          total_dataset_write_bytes=report["benchmark"]["total_generated_write_bytes"],
                          trials=27, simulation_scenarios=len(report["simulation"]["scenarios"]))))


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("CANCELLED: temporary datasets cleaned; partial reports are not success", file=sys.stderr)
        sys.exit(130)
    except (LabError, OSError, ValueError) as error:
        print(f"LAB_FAILED: {type(error).__name__}", file=sys.stderr)
        if isinstance(error, LabError):
            print(str(error), file=sys.stderr)
        sys.exit(1)
