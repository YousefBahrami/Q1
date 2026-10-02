#!/usr/bin/env python3
"""Independent non-protocol SHA-256 and Bech32m vector reproduction."""

from __future__ import annotations

import hashlib
import platform


ALPHABET = "qpzry9x8gf2tvdw0s3jn54khce6mua7l"
GENERATORS = (0x3B6A57B2, 0x26508E6D, 0x1EA119FA, 0x3D4233DD, 0x2A1462B3)


def frame(domain_id: int, payload: bytes) -> bytes:
    return (
        b"Q1DS"
        + (1).to_bytes(2, "big")
        + domain_id.to_bytes(2, "big")
        + len(payload).to_bytes(8, "big")
        + payload
    )


def polymod(values: list[int]) -> int:
    checksum = 1
    for value in values:
        top = checksum >> 25
        checksum = ((checksum & 0x01FF_FFFF) << 5) ^ value
        for index, generator in enumerate(GENERATORS):
            if (top >> index) & 1:
                checksum ^= generator
    return checksum


def encode(hrp: str, payload: bytes) -> str:
    accumulator = int.from_bytes(payload, "big")
    padding = (-len(payload) * 8) % 5
    accumulator <<= padding
    count = (len(payload) * 8 + 4) // 5
    data = [
        (accumulator >> shift) & 31
        for shift in range((count - 1) * 5, -1, -5)
    ]
    expanded = (
        [ord(character) >> 5 for character in hrp]
        + [0]
        + [ord(character) & 31 for character in hrp]
    )
    residue = polymod(expanded + data + [0] * 6) ^ 0x2BC830A3
    checksum = [(residue >> (5 * (5 - index))) & 31 for index in range(6)]
    return f"{hrp}1{''.join(ALPHABET[value] for value in data + checksum)}"


def main() -> None:
    public_key = bytes.fromhex(
        "03a107bff3ce10be1d70dd18e74bc0996"
        "7e4d6309ba50d5f1ddc8664125531b8"
    )
    account_id = hashlib.sha256(frame(0x000F, public_key)).digest()
    assert account_id.hex() == (
        "8db5455cb537a01b61de9fccde4beee4"
        "6959a216d116be1a9d66d4d40bb5b38f"
    )
    payload = bytes((1, 1, 0, 1)) + account_id
    expected = {
        "q1l": "q1l1qyqsqqvdk4z4edfh5qdkrh5len0yhmhyd9v6y9k3z6lp48tx6n2qhddn3ujwz45w",
        "q1p": "q1p1qyqsqqvdk4z4edfh5qdkrh5len0yhmhyd9v6y9k3z6lp48tx6n2qhddn3u6q78rg",
        "q1r": "q1r1qyqsqqvdk4z4edfh5qdkrh5len0yhmhyd9v6y9k3z6lp48tx6n2qhddn3u4puyfx",
    }
    for hrp, address in expected.items():
        assert encode(hrp, payload) == address
    amount_values = (
        0,
        1,
        255,
        256,
        2**32 - 1,
        2**64 - 1,
        2**64,
        2**127,
        2**128 - 1,
    )
    expected_amounts = (
        "5000000000000000000000000000000000",
        "5000000000000000000000000000000001",
        "50000000000000000000000000000000ff",
        "5000000000000000000000000000000100",
        "50000000000000000000000000ffffffff",
        "500000000000000000ffffffffffffffff",
        "5000000000000000010000000000000000",
        "5080000000000000000000000000000000",
        "50ffffffffffffffffffffffffffffffff",
    )
    for value, vector in zip(amount_values, expected_amounts, strict=True):
        encoded = b"\x50" + value.to_bytes(16, "big")
        assert encoded.hex() == vector
        assert int.from_bytes(encoded[1:], "big") == value
    print(f"python={platform.python_version()}")
    print(f"account_id={account_id.hex()}")
    for hrp in expected:
        print(f"{hrp}={encode(hrp, payload)}")
    print(f"amount_valid={len(amount_values)}")


if __name__ == "__main__":
    main()
