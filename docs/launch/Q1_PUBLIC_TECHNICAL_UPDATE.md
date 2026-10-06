# Q1 technical development update — October 2026

Q1 LOCALNET v0 is experimental distributed-ledger software for technical
reproduction and evaluation. It is not Mainnet, does not secure real-world
value, and has not undergone a complete independent security audit. No monetary
offer, distribution or reward program is live.

## What you can reproduce today

The [published LOCALNET release](https://github.com/YousefBahrami/Q1/releases/tag/v0.1.0-localnet.1)
runs one producer and three voters on one machine. Two authorized votes finalize
a signed transfer. The accepted four-node sequence includes stopping a voter,
continued finalization, restart and catch-up to identical StateRoots.

Its test scenario ends at height 4 with sender 956, recipient 40, nonce 4,
reward pool 4 and unchanged supply 1000. These are test balances, with no monetary
claim. The one-unit fee, quorum and disabled delay mechanism are local rules;
they do not establish Mainnet economics or a public fault model. Follow the
[repository README](../../README.md) and the
[LOCALNET milestone](../reports/milestones/LOCALNET_V0.md) for reproducible instructions.

## Resource experiment v0: useful negative findings

Maintainer-run research found that its initial evidence could be regenerated
after file deletion. Several experimental reward formulas also allowed a single
resource owner to gain by splitting identities. Those outcomes invalidate a
claim that the first experiment proved retained physical storage or solved
Sybil-resistant rewards. They are reasons to redesign the mechanism.

## Resource experiment v1: access, with clear limits

Subsequent controlled trials committed data before fresh challenges. Retained
files, warmed files and RAM copies each passed 30/30 trials. Deleted-file,
generated-substitute and wrong-file readers each passed 0/30. A 99%-retained
reader passed 10/30 sampled challenges but failed complete recovery.

This measures challenge-based access under tested controls. It does not prove
physical HDD ownership, continuous availability or permissionless uniqueness.
Zero identity-splitting gain in a separate accounting simulation relied on a
trusted resource mapping; it is not a permissionless security result. The next
resource direction is useful immutable content with explicit duplication and
reconstruction attacks. Resource ownership must not automatically grant votes.

## Testnet preparation

Research began with a separate signed-log failover experiment. A subsequent
maintainer-run private TESTNET profile now reexecutes real Q1 signed transfers
and full state commitments through producer crash/recovery scenarios. This
integration remains a bounded loopback experiment with fixed participants,
honest-voter assumptions and controller-observed failure. It is not a deployed
multi-machine or public Testnet and is not part of the published LOCALNET release.

The resource and failover research summarized here has been run in the development
workspace. Its newer source/evidence bundle is not shipped by this documentation
update. Independent reproduction of those research results awaits its separate
reviewed source publication; the existing LOCALNET release is available now.
Next testnet work concerns independent hosts, transport, partitions and recovery.

## Participate now

[Review the code and follow development](https://github.com/YousefBahrami/Q1),
reproduce LOCALNET on your own computer, and submit non-sensitive technical
findings through [repository issues](https://github.com/YousefBahrami/Q1/issues).
You may register interest in future node, validator or resource research there;
this is not admission, allocation or a promise of rewards. Do not expose local
node ports to the Internet or submit keys, identity documents or private addresses.

No dedicated project mailbox, newsletter or external community signup is announced.
Use the existing repository for public technical discussion. For sensitive
findings follow [SECURITY](../../SECURITY.md); public issues are not private intake.
