#!/usr/bin/env python3
"""Read-only release hygiene scan; findings omit secret values. Not a security audit."""
import argparse
import json
from pathlib import Path
import re
import subprocess

ROOT=Path(__file__).resolve().parent.parent
# Deliberately bounded signatures. A clean scan is not proof of secret absence.
PATTERNS={
    "credential_marker":re.compile(rb"-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY|\b(?:ghp_|github_pat_|AKIA|sk_live_|sk-proj-)[A-Za-z0-9_-]{12,}"),
    "credential_assignment":re.compile(rb'''(?i)(?:api[_-]?key|access[_-]?token|password|client[_-]?secret)\s*[:=]\s*["'][A-Za-z0-9+/=_-]{20,}["']'''),
    "absolute_user_path":re.compile(rb"/(?:Users|home)/[A-Za-z0-9_.-]+/|[A-Z]:\\Users\\[A-Za-z0-9_.-]+\\|com[~]apple[~]CloudDocs"),
}


def findings(name,raw):
    result=[]
    for kind,pattern in PATTERNS.items():
        lines=[i for i,line in enumerate(raw.splitlines(),1) if pattern.search(line)]
        if lines:result.append({"path":name,"kind":kind,"lines":lines})
    return result


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--history",action="store_true",help="also inspect all reachable Git blobs; never rewrite history")
    args=parser.parse_args()
    paths=subprocess.check_output(["git","ls-files","-z"],cwd=ROOT).decode().split("\0")[:-1]
    current=[]
    for name in paths:
        path=ROOT / name
        if path.is_file():
            current.extend(findings(name,path.read_bytes()))
            if path.suffix in {".key",".pem",".p12",".pfx",".log",".pyc"} or any(part in {"target","__pycache__",".localnet"} for part in path.relative_to(ROOT).parts):
                current.append({"path":name,"kind":"tracked_generated_or_key_file"})
    history=[]
    if args.history:
        objects=subprocess.check_output(["git","rev-list","--objects","--all"],cwd=ROOT,text=True).splitlines()
        for row in objects:
            oid,_,name=row.partition(" ")
            if not name:continue
            kind=subprocess.check_output(["git","cat-file","-t",oid],cwd=ROOT,text=True).strip()
            if kind=="blob":
                history.extend(dict(item,object=oid) for item in findings(name,subprocess.check_output(["git","cat-file","blob",oid],cwd=ROOT)))
    personal=[p for p in paths if p.startswith("YOS/")]
    result={"tracked_files":len(paths),"tracked_bytes":sum((ROOT / p).stat().st_size for p in paths if (ROOT / p).is_file()),
        "current_findings":current,"history_findings":history,"personal_context_excluded_from_export":personal,
        "scope":"pattern scan plus manual review required; fixture seeds are deliberately public; do not publish original Git history"}
    print(json.dumps(result,indent=2))
    serious=any(f["kind"]!="absolute_user_path" for f in current+history)
    if serious or current:
        raise SystemExit(1)


if __name__=="__main__":
    main()
