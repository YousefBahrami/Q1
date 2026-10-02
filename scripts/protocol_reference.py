#!/usr/bin/env python3
"""Independent M1.2 CBOR/hash checks using Python's standard library.

Public keys are fixed PUBLIC TEST values for seeds [7;32] and [8;32].
Signatures are read from the fixture: Python does NOT independently implement or
verify Ed25519. Node/OpenSSL and Rust perform independent signing/verification.
This script is not wallet key generation or runtime protocol validation.
"""
import hashlib
from pathlib import Path
import sys


FIXTURE = Path(__file__).resolve().parent.parent / "vectors/protocol_objects/v1/approved.tsv"


def be(value, width):
    return value.to_bytes(width, "big")


def head(major, value):
    if value < 24:
        return bytes([major * 32 + value])
    for width, tag in ((1, 24), (2, 25), (4, 26), (8, 27)):
        if value < 1 << (width * 8):
            return bytes([major * 32 + tag]) + be(value, width)
    raise ValueError("CBOR integer exceeds u64")


def cbor(value):
    if value is None:
        return b"\xf6"
    if isinstance(value, bytes):
        return head(2, len(value)) + value
    if isinstance(value, list):
        return head(4, len(value)) + b"".join(map(cbor, value))
    if type(value) is int and value >= 0:
        return head(0, value)
    raise ValueError("unsupported fixture value")


def decode(data):
    position = 0

    def take(count):
        nonlocal position
        if position + count > len(data):
            raise ValueError("truncated CBOR")
        value = data[position:position + count]
        position += count
        return value

    def item(depth):
        if depth > 16:
            raise ValueError("nesting ceiling")
        initial = take(1)[0]
        if initial == 246:
            return None
        major, extra = initial >> 5, initial & 31
        if major not in (0, 2, 4):
            raise ValueError("prohibited type")
        argument = extra
        if extra >= 24:
            if extra > 27:
                raise ValueError("indefinite/reserved encoding")
            argument = int.from_bytes(take(1 << (extra - 24)), "big")
            if argument < (24, 256, 65536, 4294967296)[extra - 24]:
                raise ValueError("nonminimal encoding")
        if major == 0:
            return argument
        if argument > (65535 if major == 4 else 16777216):
            raise ValueError("format ceiling")
        if major == 2:
            return take(argument)
        return [item(depth + 1) for _ in range(argument)]

    if len(data) > 16777216:
        raise ValueError("object ceiling")
    value = item(1)
    if position != len(data):
        raise ValueError("trailing bytes")
    return value


def frame(domain, payload):
    return b"Q1DS" + be(1, 2) + be(domain, 2) + be(len(payload), 8) + payload


def digest(domain, payload):
    return hashlib.sha256(frame(domain, payload)).digest()


def merkle(profile, items):
    if not items:
        return digest(9, be(profile, 2) + be(0, 8))
    level = [digest(9, be(profile, 2) + be(i, 8) + be(len(value), 8) + value)
             for i, value in enumerate(items)]
    while len(level) > 1:
        level = [digest(10, be(profile, 2) + level[i] + level[i + 1])
                 if i + 1 < len(level) else level[i]
                 for i in range(0, len(level), 2)]
    return level[0]


def read_vectors(path):
    values = {}
    for line in Path(path).read_text().splitlines():
        name, value = line.split("\t")
        if name in values or bytes.fromhex(value).hex() != value:
            raise ValueError(f"duplicate name or noncanonical hex: {name}")
        values[name] = bytes.fromhex(value)
    return values


def main():
    fixture = read_vectors(FIXTURE)
    sender = bytes.fromhex("ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c")
    recipient = bytes.fromhex("1398f62c6d1a457c51ba6a4b5f3dbd2f69fca93216218dc8997e416bd17d93ca")
    chain = [1, 1, bytes([9]) * 32]
    chain_id = digest(16, cbor(chain))
    identity = [1, sender, None]
    participant_id = digest(19, cbor(identity))
    participant = [1, participant_id, sender, None, 0, None]
    other_id = digest(19, cbor([1, None, recipient]))
    participants = sorted([participant, [1, other_id, None, recipient, 0, None]], key=lambda record: record[1])
    address = bytes([1, 1, 0, 1]) + digest(15, recipient)

    def transfer(nonce, signature):
        if len(signature) != 64:
            raise ValueError("wrong fixture signature width")
        body = [1, chain_id, sender, address, be(123, 16), be(4, 16), nonce, 6, 100]
        return body, [body, 1, signature]

    body, signed = transfer(5, fixture["transfer_signature"])
    _, signed2 = transfer(6, fixture["transfer2_signature"])
    transfers = [signed, signed2, signed]
    results = {
        "chain_preimage": cbor(chain), "chain_frame": frame(16, cbor(chain)), "chain_id": chain_id,
        "participant_identity": cbor(identity), "participant_id": participant_id,
        "participant_record": cbor(participant), "participant_record_hash": digest(8, cbor(participant)),
        "participant_set": cbor([1, 1, participants]), "participant_root": merkle(3, list(map(cbor, participants))),
        "transfer_body": cbor(body), "transfer_signing_payload": frame(1, cbor(body)),
        "transfer_signature": fixture["transfer_signature"], "signed_transfer": cbor(signed),
        "transfer_id": digest(2, cbor(signed)), "transfer2_signature": fixture["transfer2_signature"],
        "transfer2_id": digest(2, cbor(signed2)), "block_body": cbor([1, transfers]),
        "transaction_root": merkle(1, list(map(cbor, transfers))),
        "empty_block_body": cbor([1, []]), "empty_transaction_root": merkle(1, []),
        "delay_none": cbor([1, 0, 0, b"", b""]),
        "delay_none_frame": frame(20, cbor([1, 0, 0, b"", b""])),
        "delay_none_hash": digest(20, cbor([1, 0, 0, b"", b""])),
    }
    for name in ("chain_preimage", "participant_identity", "participant_record", "participant_set",
                 "transfer_body", "signed_transfer", "block_body", "empty_block_body"):
        if cbor(decode(results[name])) != results[name]:
            raise ValueError(f"round-trip mismatch: {name}")
    for value in ("1801", "9fff", "82018000", "59000100", "82", "a0", "d800", "830119ffff"):
        try:
            decode(bytes.fromhex(value))
        except ValueError:
            continue
        raise ValueError(f"accepted malformed CBOR: {value}")
    if len(sys.argv) > 2:
        raise ValueError("usage: python3 scripts/protocol_reference.py [rust-output.tsv]")
    output = "".join(f"{name}\t{results[name].hex()}\n" for name in sorted(results))
    for path in [FIXTURE, *sys.argv[1:]]:
        if Path(path).read_text() != output:
            raise ValueError(f"reference mismatch: {path}")
    print(f"Python: {len(results)} protocol CBOR/hash vectors and canonical rejection checks passed; "
          "Ed25519 signatures are fixture inputs, not independently verified")


if __name__ == "__main__":
    main()
