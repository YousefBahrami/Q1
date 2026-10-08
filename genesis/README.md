# Q1 Genesis

The approved LOCALNET-only genesis is implemented in
[`Genesis`](../crates/q1-localnet/src/genesis.rs); the
[`fixture`](../crates/q1-localnet/examples/fixture.rs) uses explicitly public
test keys and a 1,000-base-unit test allocation. See the
[execution profile](../docs/protocol/Q1_LOCALNET_V0.md).

Approved invariant:

`sum(all explicit genesis allocations) = declared genesis supply`

No production allocation values are approved.

LOCALNET balances are test state, not Mainnet Q1 or a monetary claim. The
fixture supply does not determine Mainnet supply. The
[issuance research](../docs/research/Q1_MAINNET_ISSUANCE_AND_MINING_THESIS.md)
traces initialization and the decisions required for a future economic profile.
