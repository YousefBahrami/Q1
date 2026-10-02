#!/usr/bin/env python3
"""Read-only Phase 0.1 documentation checks.

This script reports evidence. It does not rewrite specifications and does not
claim that placeholder or missing-file findings are errors without review.
"""

from __future__ import annotations

import collections
import pathlib
import re
import sys


ROOT = pathlib.Path(__file__).resolve().parent.parent
DOCS = ROOT / "docs"


def markdown_files() -> list[pathlib.Path]:
    return sorted(ROOT.rglob("*.md"))


def check_requirement_ids() -> int:
    pattern = re.compile(r"^(Q1-[A-Z]+-\d{3})(?=\s|$|—)")
    definitions: list[tuple[str, pathlib.Path, int]] = []
    for path in sorted(DOCS.glob("[0-2][0-9]_*.md")):
        for line_number, line in enumerate(path.read_text().splitlines(), 1):
            match = pattern.match(line.strip())
            if match:
                definitions.append((match.group(1), path, line_number))
    counts = collections.Counter(item[0] for item in definitions)
    duplicates = sorted(key for key, value in counts.items() if value > 1)
    print(
        f"requirement_definitions={len(definitions)} "
        f"unique={len(counts)} duplicates={len(duplicates)}"
    )
    for identifier in duplicates:
        locations = [
            f"{path.relative_to(ROOT)}:{line}"
            for value, path, line in definitions
            if value == identifier
        ]
        print(f"duplicate_requirement={identifier} locations={','.join(locations)}")
    return len(duplicates)


def check_decision_ids() -> int:
    pattern = re.compile(r"^## (DEC-Q1-\d{3})$")
    path = ROOT / "OPEN_DECISIONS.md"
    definitions = [
        (match.group(1), line_number)
        for line_number, line in enumerate(path.read_text().splitlines(), 1)
        if (match := pattern.match(line))
    ]
    counts = collections.Counter(item[0] for item in definitions)
    duplicates = sorted(key for key, value in counts.items() if value > 1)
    print(
        f"decision_definitions={len(definitions)} "
        f"unique={len(counts)} duplicates={len(duplicates)}"
    )
    for identifier in duplicates:
        lines = [str(line) for value, line in definitions if value == identifier]
        print(f"duplicate_decision={identifier} lines={','.join(lines)}")
    return len(duplicates)


def check_markdown_links() -> int:
    pattern = re.compile(r"\[[^\]]*\]\((?!https?://|mailto:|#)([^)]+)\)")
    broken: list[str] = []
    for path in markdown_files():
        for line_number, line in enumerate(path.read_text().splitlines(), 1):
            for raw_target in pattern.findall(line):
                target_text = raw_target.strip("<>").split("#", 1)[0]
                if not target_text:
                    continue
                target = (
                    pathlib.Path(target_text)
                    if target_text.startswith("/")
                    else path.parent / target_text
                )
                if not target.exists():
                    broken.append(
                        f"{path.relative_to(ROOT)}:{line_number}:{target_text}"
                    )
    print(f"broken_markdown_links={len(broken)}")
    for item in broken:
        print(f"broken_link={item}")
    return len(broken)


def check_referenced_files() -> int:
    pattern = re.compile(
        r"(?<![A-Za-z0-9_.-])"
        r"((?:docs/)?(?:[A-Za-z0-9_-]+/)*[A-Za-z0-9_-]+\.md)"
        r"(?![A-Za-z0-9_.-])"
    )
    missing: dict[str, list[str]] = collections.defaultdict(list)
    for path in markdown_files():
        for line_number, line in enumerate(path.read_text().splitlines(), 1):
            for token in pattern.findall(line):
                basename = pathlib.Path(token).name
                candidates = [
                    ROOT / token,
                    path.parent / token,
                    DOCS / token,
                    DOCS / "adr" / basename,
                    DOCS / "build" / basename,
                    DOCS / "security" / basename,
                    ROOT / "YOS" / basename,
                ]
                if not any(candidate.exists() for candidate in candidates):
                    missing[token].append(
                        f"{path.relative_to(ROOT)}:{line_number}"
                    )
    print(f"missing_referenced_file_tokens={len(missing)}")
    for token, locations in sorted(missing.items()):
        print(f"missing_reference={token} locations={','.join(locations[:5])}")
    return len(missing)


def check_legacy_paths() -> int:
    tokens = ("YOS_v1.0", "YOS_v0.3", "ADR/")
    findings: list[str] = []
    for path in markdown_files():
        for line_number, line in enumerate(path.read_text().splitlines(), 1):
            for token in tokens:
                if token in line:
                    findings.append(
                        f"{path.relative_to(ROOT)}:{line_number}:{token}"
                    )
    print(f"legacy_path_mentions={len(findings)}")
    for item in findings:
        print(f"legacy_path_mention={item}")
    return len(findings)


def check_placeholders() -> int:
    pattern = re.compile(
        r"\b(TODO|TBD|placeholder|remains open|unresolved|"
        r"not yet approved|not approved)\b",
        re.IGNORECASE,
    )
    findings: list[str] = []
    for path in markdown_files():
        for line_number, line in enumerate(path.read_text().splitlines(), 1):
            if pattern.search(line):
                findings.append(f"{path.relative_to(ROOT)}:{line_number}")
    print(f"placeholder_lines={len(findings)}")
    return len(findings)


def check_macos_metadata() -> int:
    findings = [
        path.relative_to(ROOT)
        for path in ROOT.rglob("*")
        if path.name == ".DS_Store"
        or path.name.startswith("._")
        or path.name == "__MACOSX"
    ]
    print(f"macos_metadata={len(findings)}")
    for path in findings:
        print(f"macos_metadata_path={path}")
    return len(findings)


def check_secret_patterns() -> int:
    pattern = re.compile(
        r"-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----"
        r"|AKIA[0-9A-Z]{16}"
        r"|gh[pousr]_[A-Za-z0-9_]{20,}"
        r"|sk-[A-Za-z0-9]{20,}"
        r"|AIza[0-9A-Za-z_-]{30,}"
    )
    findings: list[str] = []
    ignored_parts = {".git"}
    for path in sorted(item for item in ROOT.rglob("*") if item.is_file()):
        if ignored_parts.intersection(path.parts):
            continue
        try:
            text = path.read_text()
        except (UnicodeDecodeError, OSError):
            continue
        for line_number, line in enumerate(text.splitlines(), 1):
            if pattern.search(line):
                findings.append(f"{path.relative_to(ROOT)}:{line_number}")
    print(f"high_confidence_secret_patterns={len(findings)}")
    for item in findings:
        print(f"secret_pattern={item}")
    return len(findings)


def main() -> int:
    blocking = 0
    blocking += check_requirement_ids()
    blocking += check_decision_ids()
    blocking += check_markdown_links()
    check_referenced_files()
    check_legacy_paths()
    check_placeholders()
    blocking += check_macos_metadata()
    blocking += check_secret_patterns()
    print(f"blocking_check_failures={blocking}")
    return 1 if blocking else 0


if __name__ == "__main__":
    sys.exit(main())
