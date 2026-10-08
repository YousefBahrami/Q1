"""Bounded standalone data-access experiment. Never a Q1 consensus proof.

Research instantiation of docs/07_HDD_LAB_MODULE.md sections 14/19/26.
Hash labels, JSON and SHAKE expansion are lab-only, not protocol allocations.
"""
from __future__ import annotations

import hashlib
import json
from pathlib import Path
from typing import Callable

PROFILE = "Q1_RESOURCE_LAB_V0_NOT_CONSENSUS"
CHUNK_BYTES = 4096
MAX_DATASET_BYTES = 64 * 1024 * 1024
MAX_SAMPLES = 64
MAX_EVIDENCE_BYTES = 2 * 1024 * 1024


class LabError(ValueError):
    """A stable error code, without private paths or data in the message."""


def require(condition: bool, code: str) -> None:
    if not condition:
        raise LabError(code)


def canonical(value: object) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":"),
                      ensure_ascii=True, allow_nan=False).encode("ascii")


def digest(label: str, *parts: bytes) -> bytes:
    h = hashlib.sha256()
    for part in (PROFILE.encode(), label.encode(), *parts):
        h.update(len(part).to_bytes(8, "big"))
        h.update(part)
    return h.digest()


def integer(value: object, low: int, high: int) -> int:
    require(type(value) is int and low <= value <= high, "INTEGER_RANGE")
    return value


def hex_bytes(value: object, size: int) -> bytes:
    require(type(value) is str and len(value) == 2 * size, "HEX_LENGTH")
    try:
        result = bytes.fromhex(value)
    except ValueError:
        raise LabError("HEX_ENCODING") from None
    require(result.hex() == value, "HEX_ENCODING")
    return result


def fields(obj: object, names: str) -> None:
    require(type(obj) is dict and set(obj) == set(names.split()), "SCHEMA")


def chunk(seed: bytes, participant: bytes, index: int) -> bytes:
    base = digest("dataset", seed, participant, index.to_bytes(8, "big"))
    return hashlib.shake_256(base).digest(CHUNK_BYTES)


def leaf(index: int, data: bytes) -> bytes:
    return digest("dataset-leaf", index.to_bytes(8, "big"), data)


def tree(leaves: list[bytes], label: str) -> list[list[bytes]]:
    require(bool(leaves) and len(leaves) & (len(leaves) - 1) == 0, "TREE_SIZE")
    levels = [leaves]
    while len(levels[-1]) > 1:
        row = levels[-1]
        levels.append([digest(label, row[i], row[i + 1])
                       for i in range(0, len(row), 2)])
    return levels


def path_for(levels: list[list[bytes]], index: int) -> list[str]:
    proof = []
    for row in levels[:-1]:
        proof.append(row[index ^ 1].hex())
        index //= 2
    return proof


def validate_manifest(manifest: dict) -> None:
    fields(manifest, "profile participant seed chunk_bytes chunk_count root")
    require(manifest["profile"] == PROFILE, "PROFILE")
    hex_bytes(manifest["participant"], 32)
    hex_bytes(manifest["seed"], 32)
    require(type(manifest["chunk_bytes"]) is int and
            manifest["chunk_bytes"] == CHUNK_BYTES, "CHUNK_SIZE")
    count = integer(manifest["chunk_count"], 1, MAX_DATASET_BYTES // CHUNK_BYTES)
    require(count & (count - 1) == 0, "DATASET_SIZE")
    hex_bytes(manifest["root"], 32)


def commitment(manifest: dict) -> str:
    validate_manifest(manifest)
    return digest("manifest", canonical(manifest)).hex()


def create_dataset(path: Path, size_bytes: int, seed: bytes,
                   participant: bytes) -> tuple[dict, list[list[bytes]]]:
    integer(size_bytes, CHUNK_BYTES, MAX_DATASET_BYTES)
    count, remainder = divmod(size_bytes, CHUNK_BYTES)
    require(not remainder and count & (count - 1) == 0, "DATASET_SIZE")
    require(len(seed) == len(participant) == 32, "IDENTITY_SIZE")
    leaves = []
    # Callers supply a newly created dedicated temporary directory. Never
    # overwrite a file/symlink or touch raw devices, existing datasets or keys.
    with path.open("xb", buffering=0) as stream:
        for i in range(count):
            data = chunk(seed, participant, i)
            require(stream.write(data) == CHUNK_BYTES, "SHORT_WRITE")
            leaves.append(leaf(i, data))
    levels = tree(leaves, "dataset-node")
    manifest = dict(profile=PROFILE, participant=participant.hex(), seed=seed.hex(),
                    chunk_bytes=CHUNK_BYTES, chunk_count=count,
                    root=levels[-1][0].hex())
    return manifest, levels


def validate_context(context: dict, manifest: dict) -> None:
    fields(context, "profile chain height round producer nonce dataset")
    require(context["profile"] == PROFILE and
            context["chain"] == "q1-resource-lab-no-network", "CONTEXT_PROFILE")
    integer(context["height"], 1, 2**64 - 1)
    integer(context["round"], 0, 2**64 - 1)
    hex_bytes(context["nonce"], 32)
    require(context["producer"] == manifest["participant"], "PRODUCER")
    require(context["dataset"] == commitment(manifest), "DATASET_BINDING")


def challenge(context: dict, manifest: dict) -> bytes:
    validate_context(context, manifest)
    return digest("challenge", canonical(context))


def indices(q: bytes, count: int, samples: int) -> list[int]:
    integer(samples, 1, MAX_SAMPLES)
    require(samples & (samples - 1) == 0, "SAMPLE_COUNT")
    return [int.from_bytes(digest("selector", q, j.to_bytes(8, "big")), "big") % count
            for j in range(samples)]


def response(q: bytes, j: int, index: int, data: bytes) -> bytes:
    return digest("response", q, j.to_bytes(8, "big"),
                  index.to_bytes(8, "big"), data)


def prove(manifest: dict, levels: list[list[bytes]], context: dict,
          read_chunk: Callable[[int], bytes], samples: int = 64) -> dict:
    q = challenge(context, manifest)
    rows, responses = [], []
    for j, index in enumerate(indices(q, manifest["chunk_count"], samples)):
        data = read_chunk(index)
        require(len(data) == CHUNK_BYTES, "SHORT_READ")
        require(leaf(index, data) == levels[0][index], "DATA_CORRUPTION")
        r = response(q, j, index, data)
        rows.append(dict(index=index, data=data.hex(),
                         path=path_for(levels, index), response=r.hex()))
        responses.append(r)
    return dict(profile=PROFILE, dataset=commitment(manifest),
                challenge=q.hex(), samples=samples, rows=rows,
                response_root=tree(responses, "response-node")[-1][0].hex())


def decode(raw: bytes) -> dict:
    require(type(raw) is bytes and len(raw) <= MAX_EVIDENCE_BYTES, "ENCODED_SIZE")

    def unique(pairs):
        result = {}
        for key, value in pairs:
            require(key not in result, "DUPLICATE_FIELD")
            result[key] = value
        return result

    try:
        obj = json.loads(raw, object_pairs_hook=unique)
    except (UnicodeError, json.JSONDecodeError, RecursionError):
        raise LabError("JSON_ENCODING") from None
    require(type(obj) is dict, "SCHEMA")
    return obj


def verify(raw: bytes, manifest: dict, context: dict) -> str:
    """Verify against independently supplied manifest AND fresh context.

    The caller pins these expected inputs; an evidence-supplied challenge/root
    is never its own authority. No device type, timing or persistent storage
    claim is verified. Even valid membership only samples the dataset.
    """
    q = challenge(context, manifest)
    evidence = decode(raw)
    fields(evidence, "profile dataset challenge samples rows response_root")
    require(evidence["profile"] == PROFILE, "PROFILE")
    require(evidence["dataset"] == commitment(manifest), "DATASET_BINDING")
    require(evidence["challenge"] == q.hex(), "CHALLENGE_BINDING")
    selected = indices(q, manifest["chunk_count"], evidence["samples"])
    require(type(evidence["rows"]) is list and
            len(evidence["rows"]) == len(selected), "ROW_COUNT")
    depth = manifest["chunk_count"].bit_length() - 1
    responses = []
    for j, (row, index) in enumerate(zip(evidence["rows"], selected)):
        fields(row, "index data path response")
        require(type(row["index"]) is int and row["index"] == index, "SELECTION")
        data = hex_bytes(row["data"], CHUNK_BYTES)
        require(data == chunk(bytes.fromhex(manifest["seed"]),
                              bytes.fromhex(manifest["participant"]), index),
                "DATASET_GENERATION")
        require(type(row["path"]) is list and len(row["path"]) == depth, "PATH_LENGTH")
        node, cursor = leaf(index, data), index
        for sibling_hex in row["path"]:
            sibling = hex_bytes(sibling_hex, 32)
            node = (digest("dataset-node", sibling, node) if cursor & 1
                    else digest("dataset-node", node, sibling))
            cursor //= 2
        require(node.hex() == manifest["root"], "MEMBERSHIP")
        r = response(q, j, index, data)
        require(row["response"] == r.hex(), "RESPONSE")
        responses.append(r)
    root = tree(responses, "response-node")[-1][0].hex()
    require(evidence["response_root"] == root, "RESPONSE_ROOT")
    return root


def read_json(path: Path) -> dict:
    with path.open("rb") as stream:
        return decode(stream.read(MAX_EVIDENCE_BYTES + 1))
