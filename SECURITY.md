# Q1 LOCALNET v0 security

Q1 LOCALNET v0 is experimental software. It has not undergone a complete
independent security audit and should not secure real-world value. It is not
Mainnet. Consensus, economics and NONE delay are explicitly local test rules.
The source is public at [YousefBahrami/Q1](https://github.com/YousefBahrami/Q1),
maintained by Yousef Bahrami. Source publication and passing remote CI do not
establish public-network readiness or replace security review.

## Supported scope and known limits

- v0.1.0-localnet.1 is a published experimental LOCALNET source release, not production-ready.
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
channel to the project maintainer, send a minimal reproduction there. Otherwise,
retain details locally until an official private reporting channel is published;
do not post exploit details or keys in a public issue. An official intake must
be configured before operating any external network or holding real value.

Do not include credentials, wallet seeds or unrelated personal/infrastructure
data in reports. Testing is authorized only on environments you own or have
explicit permission to test.

Commit identity and contact privacy are documented in the
[public email audit](docs/security/PUBLIC_EMAIL_PRIVACY_AUDIT.md). GitHub noreply
is a commit identity, not a vulnerability-reporting mailbox. Verify project
origin through the [canonical channels](BRAND.md) before trusting a release;
a third-party fork or matching checksum alone does not establish endorsement.

## References

The implemented [LOCALNET profile](docs/protocol/Q1_LOCALNET_V0.md) takes
precedence for this release's scope. Broader requirements and open threats are
recorded in [security model](docs/12_SECURITY_MODEL.md),
[adversarial scenarios](docs/13_HOW_TO_BREAK_Q1.md) and
[open decisions](OPEN_DECISIONS.md).

## Unpublished research candidate boundary

The research candidate adds TESTNET_FAILOVER_V0 and multi-host deployment tooling.
These are explicit private crash-only experiments using known fixture keys.
LOCALNET retains its existing wire profile and limits. SSH forwarding does not
make fixture keys secret, establish permissionless admission or confer Byzantine
safety. Keep application RPC loopback-only; use only confirmed private hosts and
pinned SSH host identities. No production key custody or public Internet service.
Resource proofs demonstrate tested access, not physical uniqueness or mining yield.

## Native research candidate

The separate TESTNET adapter uses permissioned mutual TLS and explicit private
addresses. It does not make the published LOCALNET RPC encrypted. Local crash,
recovery and basic resource-limit tests do not establish public-network safety.
Do not expose either profile publicly. Native real-host acceptance, rolling upgrade,
key lifecycle rehearsal and public admission remain gated. No production mining,
monetary issuance, public custody or completed independent audit is claimed.
