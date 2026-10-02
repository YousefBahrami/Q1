# Q1 LOCALNET v0 security

Q1 LOCALNET v0 is experimental software. It has not undergone a complete
independent security audit and should not secure real-world value. It is not
Mainnet. Consensus, economics and NONE delay are explicitly local test rules.

## Supported scope and known limits

- v0.1.0-localnet.1 is a local research release candidate, not production-ready.
- Nodes are loopback-only. Do not forward ports or expose RPC to the Internet.
  Transport is unencrypted and not hardened for hostile public peers.
- One fixed producer and three fixed voters use 2-of-3 certificates. This is
  the approved local liveness profile, not a settled public Byzantine fault model.
- No producer rotation/failover or automatic unfinalized-proposal recovery.
- Wallet files are unencrypted local test seeds, without recovery/mnemonics,
  hardware signing, key rotation or production custody guarantees.
- Full bounded archives use atomic rename and replay. There is no claim of
  testing every filesystem/power-loss configuration or resisting disk rollback.
- Status output is an observation from a local endpoint, not a light-client proof.
- Historical code/dependency reviews and passing tests are not an independent audit.

**TEST ONLY — NEVER USE FOR VALUE-BEARING NETWORKS.** All deterministic seeds
in Rust test/example fixtures and reference tooling are intentionally public.
Never reuse their identities, or local demo wallets, for valuable networks.

## Reporting a vulnerability

No approved public security address or private intake service exists yet.
No contact address is invented by this release. If you already have a private
channel to the project owner, send a minimal reproduction there. Otherwise,
retain details locally until an official private reporting channel is published;
do not post exploit details or keys in a public issue. An official intake must
be configured before operating any external network or holding real value.

Do not include credentials, wallet seeds or unrelated personal/infrastructure
data in reports. Testing is authorized only on environments you own or have
explicit permission to test.

## References

The implemented [LOCALNET profile](docs/protocol/Q1_LOCALNET_V0.md) takes
precedence for this release's scope. Broader requirements and open threats are
recorded in [security model](docs/12_SECURITY_MODEL.md),
[adversarial scenarios](docs/13_HOW_TO_BREAK_Q1.md) and
[open decisions](OPEN_DECISIONS.md).
