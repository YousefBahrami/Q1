#!/usr/bin/env python3
"""Independently reconstruct local genesis/state framing and compare frozen bytes.

Python validates canonical bytes and SHA-256 commitments; Ed25519 verification
is covered by Rust tests, not claimed by this dependency-free Python script.
"""
from pathlib import Path
import sys
from localnet_acceptance import decode, encode, digest

ROOT = Path(__file__).resolve().parent.parent
FROZEN = ROOT / "vectors/localnet/v0/approved.tsv"


def check(path):
    assert path.read_bytes() == FROZEN.read_bytes(), "Rust reference bytes changed"
    rows = dict(line.split("\t") for line in path.read_text().splitlines())
    rows = {key: bytes.fromhex(value) for key, value in rows.items()}
    assert len(rows) == 13
    for name, value in rows.items():
        if len(value) != 32:
            assert encode(decode(value)) == value, name
    for source, target, domain in [
        ("genesis", "genesis_id", 0x0d),
        ("initial_state", "initial_state_root", 0x15),
        ("signed_proposal", "proposal_id", 0x11),
        ("signed_header", "block_id", 0x04),
        ("certificate", "certificate_hash", 0x07),
        ("state_after_transfer", "state_root_after_transfer", 0x15),
    ]:
        assert digest(domain, rows[source]) == rows[target], target
    genesis = decode(rows["genesis"])
    assert genesis[:3] == [1, "LOCALNET_V0", 1]
    assert genesis[9] == [1, (1).to_bytes(16, "big"), False, False, False, 2, 0, 0, False]
    chain = digest(0x10, encode(genesis[3]))
    reconstructed = [1, chain, rows["genesis_id"], 0, genesis[4], bytes(16),
                     [[1, row[1], row[2], 0, 1] for row in genesis[5]], genesis[6]]
    assert encode(reconstructed) == rows["initial_state"]
    proposal = decode(rows["signed_proposal"])
    header = decode(rows["signed_header"])
    certificate = decode(rows["certificate"])
    state = decode(rows["state_after_transfer"])
    assert proposal[0][3] == header
    assert header[0][9] == rows["state_root_after_transfer"]
    assert certificate[3] == rows["proposal_id"]
    for attestation in certificate[4]:
        assert attestation[0][6] == rows["proposal_id"]
        assert attestation[0][7] == rows["block_id"]
    assert decode(rows["certified_block"]) == [1, proposal, certificate]
    assert state[:3] == reconstructed[:3] and state[3] == 1
    assert int.from_bytes(state[5], "big") == 1
    assert sorted(int.from_bytes(row[2], "big") for row in state[6]) == [10, 989]
    assert sorted(row[3] for row in state[6]) == [0, 1]
    assert sum(int.from_bytes(row[2], "big") for row in state[6]) + 1 == int.from_bytes(state[4], "big") == 1000
    print("PASS: 13 frozen LOCALNET vectors; independent Python canonical bytes, initial state reconstruction and domain hashes")


if __name__ == "__main__":
    check(Path(sys.argv[1]) if len(sys.argv) > 1 else FROZEN)
