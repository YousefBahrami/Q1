# TESTNET_FAILOVER_V0 integration map — before implementation

Date: 2026-10-05. Human-authorized minimum TESTNET integration. No Mainnet rule.

| Surface | Prior isolated experiment | Existing Q1 LOCALNET | Minimum integrated TESTNET |
|---|---|---|---|
| Value | Signed synthetic text | Signed canonical transfers in block bodies | Immutable signed wrapper containing actual Q1 BlockBodyV1 and candidate StateRoot |
| Execution | No ledger or fee | Rust Ledger whole-block execution and StateSnapshot | Reuse the same Rust execution, signatures, nonce checks, conservation and full-state commitment |
| Authority | Two producers, three voters, fixed fixture keys | One fixed producer, three voters, round zero | Explicit TESTNET_FAILOVER_V0 profile; two eligible non-voting producers, three fixed voters |
| Network/profile | Research-only string/loopback | NetworkClass::Localnet, strict NONE guard | NetworkClass::PrivateTestnet, separate genesis/profile identity, loopback transport; no NONE witness and no delay-security claim |
| State/genesis | Research log hash | Full canonical accounts, fee pool and registry | Full StateSnapshot with five-member role registry and profile-bound genesis; shared state codec/hash, never an old LOCALNET state |
| Reservation | Durable increasing prepare/accept ballot | Exact (height, round, voter) proposal reservation | Keep isolated prepare/adoption rules in a new integration runner; reserve durable immutable Q1 value before emitting vote |
| Finality | Signed majority of arbitrary value hashes | LOCALNET proposal/header/certificate schemas | TESTNET-only wrapper/certificate verified by Rust before applying ledger state; do not relabel these as unchanged LOCALNET wire objects |
| Recovery | Synthetic history replay | Authenticated ledger replay | Every voter/producer/learner reexecutes signed transfers and checks claimed StateRoot when replaying/adopting |

The existing approved LOCALNET header, proposal, genesis, certificate, NONE guard,
fee/supply behavior and golden vectors remain unchanged. TESTNET wrappers have
an explicit string namespace and are not Mainnet schemas. No new numeric domain
is allocated: new coordination/outer-block framing stays in its named TESTNET
namespace; the shared full-state commitment remains bound to a different
profile-specific GenesisId and chain identity. State serialization contains all
accounts and all five role records. This scoped reuse extends the full-state
codec to the controlled test profile, not to the inactive general state tree.

The shared ledger gains an explicit guarded private-testnet initialization path
and address-network field; its existing LOCALNET constructor/restore retains the
old behavior. No monetary issuance, reward distribution, mining weight or new
fee formula. Test balances remain arbitrary test units: supply 1000, fee 1.
Resource access research remains outside consensus authority.

Proposal origin/creation ballot and current coordination ballot are distinct.
A recovered value keeps original producer signature/body/StateRoot; the new
producer cannot replace it merely because it now coordinates. Majority prepare
reports preserve already accepted values, including chosen-but-unannounced work.
A local observed process failure triggers the next deterministic coordination
ballot. This bounded trigger does not establish a production failure detector,
partition-liveness result or permissionless network.

Acceptance must exercise all eight human-requested failure points, genuine
signed transfers, exact state/fee accounting, rejection without state change,
restart of replacement and voter, stale producer, conflicts and replay. Persist
journal changes before vote delivery; replay all certificates and compare roots.
The crash-only model assumes authorized voters are honest. Equivocation detection
is not one-Byzantine-member tolerance. Full Internet hardening remains separate.
