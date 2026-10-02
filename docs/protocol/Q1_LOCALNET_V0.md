# Q1 LOCALNET v0 execution profile

Scope: **LOCALNET v0 ONLY**. Human decisions recorded on 2026-10-01 and
2026-10-02. Authority: Yousef Bahrami's explicit continuation, three local
policy decisions, and subsequent full-state/fixed-producer/schema authorization.
This is the current bounded execution profile; public-network decisions remain open.

## Approved policy

- Each successful transfer pays exactly 1 base unit. FeeLimit is a ceiling,
  never the debit: debit = amount + 1. FeeLimit < 1 is rejected.
- Issuance = 0; burn = 0; reward distribution = 0. Explicit reward_pool
  accumulates fees. Sum(account balances) + reward_pool = declared supply.
- A rejected transfer has no state or fee effect. Any invalid transfer rejects
  the entire block, including earlier candidate changes and fees.
- One fixed producer, three fixed voters, quorum two distinct authorized votes,
  round zero, no rotation. The producer is excluded from the voter set.
  Disconnect never changes membership; a reconnecting voter catches up.
- NONE delay is exactly canonical `[1,0,0,h'',h'']` (`850100004040`). Construction,
  decoding and verification reject every non-Localnet network class. There is
  no production mode or implicit fallback that can activate NONE.
- Full canonical state commitment, with no partial proofs or light-client API.
- Existing docs/04 rules remain: positive amount, exact next nonce starting at
  zero, signed chain and inclusive height window, strict signatures, checked
  arithmetic, sequential heights. Self-transfers are rejected per docs/04
  section 9.4 and docs/09. Used zero-balance accounts retain their nonce.

These decisions do not close general DEC-Q1-021/022/023, resolve the general
"two-thirds" versus "strictly greater than two-thirds" contradiction, select
Mainnet's fault model, or replace the final delay protocol. Producer failure
may halt progress. No view change, leader election, admission or automatic
committee reconfiguration is implemented.

## Canonical local schemas

All arrays use the existing strict Q1 deterministic CBOR. Hashes and IDs are
bytes32; signatures bytes64; monetary values bytes16 unsigned big-endian;
addresses use the existing bytes36 AddressEnvelope. Integers use shortest CBOR.
Embedded objects are arrays unless explicitly called byte strings. There is
no wall-clock field in any consensus preimage.

Genesis is `[1,"LOCALNET_V0",protocol=1,ChainIdentityPreimageV1,total_supply,
allocations,participant_registry,producer_id,voter_ids,policy]`.
Allocation rows `[1,address,amount]` are strictly sorted by raw address, without
duplicates. The four ParticipantRecordV1 records and three voter IDs are strictly
sorted by ParticipantId. All records activate at height zero without deactivation;
role keys and identity checks preserve the approved ParticipantSetV1 rules.
Policy is `[1,fee=1,issuance=false,burn=false,reward_distribution=false,quorum=2,
round=0,delay_engine=0,rotation=false]`. Decoding rejects different policy values.
GenesisId hashes the complete manifest under existing GENESIS (0x000d).
Initial StateRoot is derived from this manifest after GenesisId; serializing
it inside the manifest would create a circular hash dependency.

StateSnapshot is `[1,ChainId,GenesisId,height,total_supply,reward_pool,accounts,
participant_registry]`. Accounts are strictly sorted by raw address with rows
`[1,address,balance,nonce,account_version=1]`. No zero-account pruning occurs;
an absent account reads as balance/nonce zero. Explicit rows remain committed.
The immutable registry is sorted by ParticipantId. GenesisId transitively binds
all fixed authority and policy. StateRoot is Q1HashV1(LOCALNET_STATE=0x0015,
complete canonical snapshot). The shared state-tree Merkle profile 0x0005
remains inactive: its state-key/proof semantics have not been defined. Reusing
transaction or participant tree profiles for state would violate those profiles.
Tip identity is validated and persisted separately to avoid a block/state hash cycle.

HeaderBody preserves all eleven Session 5C fields exactly:
`[schema=1,protocol=1,ChainId,height,round,ParentReferenceV1,producer_id,TxRoot,
ParticipantRoot,StateRoot,DelayEvidenceHash]`.
SignedBlockHeader is `[HeaderBody,algorithm=1,producer_signature]`.
The signature signs the BLOCK_HEADER_SIGNING frame directly. BlockId hashes
this complete envelope under BLOCK_ID, preserving Session 5A identity rules.
TxRoot and ParticipantRoot use the approved indexed Merkle profiles and owner
objects; both are recomputed. The height-specific participant set remains fixed.

Proposal signing body is `[1,"LOCALNET_V0",GenesisId,SignedBlockHeader,
BlockBodyV1,DelayEvidenceV1]`; envelope `[body,algorithm=1,producer_signature]`.
Its signature uses PROPOSAL_SIGNING. ProposalId hashes the **complete envelope**
under LOCALNET_PROPOSAL_ID=0x0011. Both producer signatures are verified.

Attestation signing body is `[1,"LOCALNET_V0",GenesisId,ChainId,height,round,
ProposalId,BlockId,voter_id,attestation_type=1]`; envelope
`[body,algorithm=1,voter_signature]`, signed under ATTESTATION_SIGNING.
Genesis/chain and BlockId explicitly prevent cross-context reuse. Type 1 is the
only local finalization vote. Before signing, the runtime independently executes
and validates the proposal, then durably reserves its complete bytes at
`(height,round,voter)`. Different proposals at that key are rejected after restart.
The pure cryptographic signing helper alone does not provide this persistence.

Certificate is `[1,"LOCALNET_V0",GenesisId,ProposalId,attestations]`, containing
two or three full signed attestations in strictly increasing voter-ID order.
Unknown, duplicate, unsorted, mixed-context, bad-signature and insufficient
votes are rejected. Its optional commitment uses FINALIZATION_CERTIFICATE.
CertifiedBlock is `[1,Proposal,Certificate]`.

The local byte layouts fill previously unapproved compound schemas using the
explicit human authorization. They do not approve the generic blank decision
rows in docs/32. The approved reusable schemas and HeaderBody are preserved.
The registry records local allocations after checking all existing assignments;
0x0012 remains unregistered. Frozen vectors are in
`vectors/localnet/v0/approved.tsv`; 13 objects/commitments are pinned by Rust,
with independent Python framing, initial-state reconstruction and hashing.

## Commit and recovery

Chain validates structure, producer authorization, parent/height, ordered
transaction execution, TxRoot, ParticipantRoot, StateRoot, delay commitment and
certificate before changing state, tip or history. A failure rolls back all
candidate balances, nonces and fees.

PersistentChain holds an exclusive OS file lock for its data directory. Archive
shape is `[1,genesis_bytes,[certified_block_bytes],snapshot_bytes,height,
tip_bytes_or_null,[[voter_id,proposal_bytes],...]]`. Vote rows are sorted by
height, round, voter. All bytes fields contain complete canonical objects.
It writes a new file, syncs the file, atomically renames it to `state.cbor`, then
syncs the directory. Pre-rename failure preserves the old commit. A failure to
confirm directory durability after rename poisons further writes/signing until
reopen. A leftover `state.pending` is never accepted as committed data.

Open replays every certified block from the configured genesis and compares
exact saved snapshot, height and tip. Vote reservations are validated against
their historical parent state. Missing or corrupt archives fail closed; the
node never silently reinitializes an existing data directory. Tests cover
corruption, pre-rename failure, competing opens, durable reservations, reload,
and post-restart continuation. Power-loss behavior on every filesystem and
hardware combination has not been tested.

## Loopback node and resource bounds

`q1-node` requires an explicit local genesis, role/key, new or valid data directory,
loopback listen address, fixed producer endpoint and three distinct voter
endpoints. The key must match the authorized genesis role. Unix key files must
exclude group/other access. No key is silently created or selected. The fixture
example intentionally writes public deterministic seeds, unsuitable for custody.

TCP transport is an operational local harness, not the unresolved public-network
transport. Frame: u32 big-endian length then canonical
`[1,GenesisId,opcode,payload_bytes]`. Opcodes: status=0, submit=1, propose=2,
commit=3, sync=4, error=255. Submit contains one complete signed transfer;
propose/commit contain complete canonical objects; sync requests the next
height and returns one certified block or empty bytes. Status returns
`[height,tip_or_null,StateRoot,snapshot_bytes]`. Voters poll producer for catch-up;
all received blocks undergo the same full commit checks.

Only loopback IPs are admitted. Frames and archives are bounded by the existing
16 MiB canonical limit; CBOR depth is 16 and array length at most 65,535.
There are at most 16 request workers, 15-second socket timeouts and one producer
submission in flight. State locks are released before outbound requests.
These are local implementation limits, not approved public-network parameters.
Full history and votes are rewritten on every durable mutation; reaching the
archive ceiling fails without a partial commit. No performance/scalability claim
is made. Local transport is unencrypted and offers no public peer identity,
rate-abuse or denial-of-service guarantee.

If a submission collects too few votes, its valid votes remain durably reserved.
Retry the exact same signed transfer/proposal when voters return. A competing
proposal can be rejected for equivocation. There is no producer pending-work
queue or automatic recovery of an unfinalized proposal from peers; producer
crash/failover recovery for such in-flight work is deferred. Already committed
history recovers and continues. Clients that lose the submission reply should
inspect chain state before retrying; there is no exactly-once RPC guarantee.

## Run and verify

```sh
cargo build --workspace --all-targets --locked
python3 scripts/localnet_acceptance.py
python3 scripts/check_all.py
```

To retain public fixtures, process logs and machine-readable evidence, supply a
new directory to `python3 scripts/localnet_acceptance.py --output-dir /tmp/q1-run`.
The harness starts four actual independent processes and cleans up its own
processes on success/failure. It verifies signed transfers, fee=1 and conserved
supply, common finalized state, bad-signature/replay rejection, voter SIGKILL,
continued 2-of-3 progress, restart/catch-up, subsequent transfer, 1-of-3 rejection,
restart anti-equivocation, exact-proposal retry and recovery of all four processes.
It is finite and suitable for the existing CI job; it does not deploy a network.

Release scope remains a local research implementation. Encrypted wallet
custody, general state proofs, producer rotation,
public admission, AI/HDD integration and Mainnet remain outside this milestone.


Publication update (2026-10-02): milestone 0c177fd accepted; public source release
v0.1.0-localnet.1 adds operational CLI/supervision without changing
these canonical protocol bytes. DEC-Q1-015 is now approved as Apache-2.0.
README.md documents wallet/sign/submit/status commands and the foreground
supervisor. These are local test tools, not production custody. The separately
authorized [source release](https://github.com/YousefBahrami/Q1/releases/tag/v0.1.0-localnet.1)
and remote CI are complete. No public network has been launched and none of
the local protocol rules above has been promoted to a public-network rule.
