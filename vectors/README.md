# Q1 reference vectors

These files pin canonical protocol bytes and commitments. Test/reference keys
are deliberately public; they are not wallet or validator provisioning material.

TEST ONLY — NEVER USE FOR VALUE-BEARING NETWORKS.

Do not change expected vectors to conceal a protocol regression. The protocol
objects are independently checked in Rust, Node and Python; LOCALNET objects
are checked in Rust and Python. Python does not verify Ed25519 signatures.
