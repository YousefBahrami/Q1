# Q1 tests

Implemented Rust tests live with their crates under `crates/*/tests` and unit
modules. `research/pre_m1_conformance` is the isolated historical reference
harness. Approved object vectors live in `vectors/protocol_objects/v1`.

Run `python3 scripts/check_all.py` for format, lint, tests, build, Rustdoc,
independent references and documentation checks. Local accounting and voting
policy tests do not establish network finality; four-node acceptance is pending.

All deterministic key seeds in tests, examples and reference tooling are public fixtures.
TEST ONLY — NEVER USE FOR VALUE-BEARING NETWORKS.
