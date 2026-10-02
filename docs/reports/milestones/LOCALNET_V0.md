# LOCALNET v0 milestone verification — 2026-10-02

## COMPLETED

Implemented full canonical state commitment, local genesis and signed compound
block schemas, fixed producer/three voters with 2-of-3 certificates, whole-block
rollback, atomic persistence/replay, durable vote reservations, loopback node
and executable four-process acceptance. Scope is LOCALNET_V0 ONLY.

## TESTED

`python3 scripts/check_all.py` passed locally: 86 workspace Rust tests, 4
historical conformance tests, 23 existing Rust/Node/Python protocol vectors,
13 frozen LOCALNET vectors checked by Rust and independent Python canonical
reconstruction/hashing, actual four-process acceptance, format, Clippy with
warnings denied, workspace build, Rustdoc with warnings denied and blocking
documentation checks. Remote CI was not executed.

The old negative test treating 0x0011 as unregistered initially failed after
its authorized local registration. It was updated with the domain registry;
the final full check passed. Documentation checks still report historical
placeholder/missing-reference advisories; blocking failures are zero.

## OBSERVED BEHAVIOR

The four processes agreed after each signed transfer. Killing a voter left
exactly two authorized votes in the next certificate and finalization continued.
The restarted voter caught up, then successfully submitted another transfer.
With two voters offline, one vote could not finalize and no fee/state changed.
Killing/restarting that one voter preserved its reservation: a conflicting
valid producer proposal was rejected. Retrying the original proposal after
voters returned succeeded. All four processes then recovered from disk.

Final height: 4. Sender balance: 956. Recipient balance: 40. Sender nonce: 4.
Reward pool: 4. Total supply: 1000, unchanged.
All four final StateRoots:
`b7ec7d47f4cf37d029e74bab02d8bc2aab191f5a5733f92ac4310d919183b087`.

Machine-readable evidence: [LOCALNET_V0_ACCEPTANCE.json](LOCALNET_V0_ACCEPTANCE.json).
The process ID is run evidence, never a consensus input.

## NOW WORKING

This local milestone is ready for review and reproduction. No background
network is left running by the acceptance harness.

## NEXT

Review the local-only release candidate and settle DEC-Q1-015 before any public
source distribution. Packaging/publication and public-network readiness are
separate work; they have not been performed by this milestone.

## REAL BLOCKERS

No blocker remains for this four-node local acceptance target. Final public
quorum, fault model, delay, economics, admission and license remain open.
Producer failover, automatic unfinalized-work recovery, partial proofs and
production wallet custody are deferred. Full bounded archives and loopback TCP
are deliberate local limits; see the current execution profile.

The earlier implementation was incomplete because state/selection and compound
schema semantics were undecided, and interrupted agent work left integration
and tests unfinished. The explicit human local decisions now supply authority;
this milestone adds executable evidence rather than treating past approval as
proof that code existed.

## FOUR-NODE TARGET REMAINING

None for the approved sequence: four nodes → signed transfer → fee=1 →
common finalized state → voter stop → 2-of-3 progress → restart → catch-up →
identical StateRoot. This result does not claim Mainnet or a published release.
