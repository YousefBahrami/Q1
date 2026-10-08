#!/usr/bin/env python3
"""Create a deterministic source-only LOCALNET release candidate; never publish/tag.

Uses committed files and export-ignore. Preserves the private repository/history.
Requires a clean tree and approved LICENSE. No runtime databases or keys included.
"""
import argparse
import gzip
import hashlib
import io
import json
from pathlib import Path
import subprocess
import tarfile
import tomllib

ROOT = Path(__file__).resolve().parent.parent


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", type=Path, default=ROOT / "dist")
    args = parser.parse_args()
    if (ROOT / "docs/releases/Q1_PUBLICATION_MANIFEST.json").exists():
        raise SystemExit(
            "Mixed private research checkout: use the reviewed PUBLIC manifest "
            "in a clean public checkout. Do not bulk-export internal strategy "
            "or overwrite the existing LOCALNET release assets."
        )
    if subprocess.check_output(["git","status","--porcelain","--untracked-files=all"],cwd=ROOT).strip():
        raise SystemExit("Refusing release from a dirty tree; commit and verify changes first")
    if not (ROOT / "LICENSE").is_file():
        raise SystemExit("Missing approved LICENSE")
    version=tomllib.loads((ROOT / "crates/q1-node/Cargo.toml").read_text())["package"]["version"]
    prefix=f"q1-v{version}"
    commit=subprocess.check_output(["git","rev-parse","HEAD"],cwd=ROOT,text=True).strip()
    archive=subprocess.check_output(["git","archive","--format=tar",f"--prefix={prefix}/",commit],cwd=ROOT)
    files={}
    with tarfile.open(fileobj=io.BytesIO(archive),mode="r:") as source:
        for member in source:
            path=Path(member.name)
            relative=Path(*path.parts[1:])
            if any(part in {"YOS","AGENTS.md",".git","target",".localnet","__pycache__"} for part in relative.parts):
                raise SystemExit(f"Excluded private/generated path in archive: {relative}")
            if member.isdir():
                continue
            if not member.isfile() or relative.suffix in {".key",".pem",".p12",".pfx",".log",".pyc"} or relative.name in {"state.cbor","state.pending"}:
                raise SystemExit(f"Unexpected release artifact: {relative}")
            raw=source.extractfile(member).read()
            files[str(relative)]={"bytes":len(raw),"sha256":hashlib.sha256(raw).hexdigest()}
    if "LICENSE" not in files or "vectors/localnet/v0/approved.tsv" not in files:
        raise SystemExit("Required license/evidence missing")
    compressed=gzip.compress(archive,mtime=0)
    args.output_dir.mkdir(parents=True,exist_ok=True)
    name=f"{prefix}-source.tar.gz"
    (args.output_dir / name).write_bytes(compressed)
    digest=hashlib.sha256(compressed).hexdigest()
    (args.output_dir / f"{name}.sha256").write_text(f"{digest}  {name}\n")
    manifest={"version":f"v{version}","source_commit":commit,"scope":"LOCALNET_V0 ONLY", "license":"Apache-2.0",
              "archive":name,"archive_sha256":digest,"files":files,"excluded":["YOS/","AGENTS.md","Git history/metadata","all untracked/ignored files"]}
    (args.output_dir / f"{prefix}-manifest.json").write_text(json.dumps(manifest,indent=2,sort_keys=True)+"\n")
    print(f"Prepared {name}: {len(files)} files, {len(compressed)} compressed bytes; SHA256 {digest}")
    print("No tag, upload, remote repository or release created. Publish the reviewed snapshot only after authorization.")


if __name__=="__main__":
    main()
