// TEST ONLY — NEVER USE FOR VALUE-BEARING NETWORKS. Deterministic fixture seeds are public.
//! Observable accounting demo using PUBLIC TEST seeds, not wallet custody or finality.
use q1_localnet::{LocalnetV0, ledger::Ledger, quorum::LocalnetCommittee};
use q1_primitives::{Address, Amount, Ed25519PrivateKey, Height, Network, ed25519};
use q1_protocol_types::{
    address::AddressEnvelope,
    block_body::BlockBodyV1,
    chain::{ChainIdentityPreimageV1, NetworkClass},
    delay::LocalnetNoneEvidence,
    participant::ParticipantRecordV1,
    transfer::{SignedTransferV1, TransferBodyV1, TransferFields},
    types::{FeeLimit, Nonce},
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mode = LocalnetV0::new(NetworkClass::Localnet)?;
    let chain = ChainIdentityPreimageV1::new(NetworkClass::Localnet, [9; 32]).chain_id()?;
    let sender_key = Ed25519PrivateKey::from_seed([7; 32]);
    let sender_public = ed25519::public_key(&sender_key)?;
    let recipient_public = ed25519::public_key(&Ed25519PrivateKey::from_seed([8; 32]))?;
    let sender =
        AddressEnvelope::from_address(Address::from_public_key(Network::Localnet, sender_public)?);
    let recipient = AddressEnvelope::from_address(Address::from_public_key(
        Network::Localnet,
        recipient_public,
    )?);
    let mut ledger =
        Ledger::from_allocations(mode, chain, Amount::new(100), [(sender, Amount::new(100))])?;
    let transaction = SignedTransferV1::sign(
        TransferBodyV1::new(TransferFields {
            chain_id: chain,
            sender_public_key: sender_public,
            recipient_address: recipient,
            amount: Amount::new(20),
            fee_limit: FeeLimit::new(50),
            nonce: Nonce::new(0),
            valid_from_height: Height::new(1),
            valid_until_height: Height::new(1),
        })?,
        &sender_key,
    )?;
    let block = BlockBodyV1::new(vec![transaction.clone()])?;
    let result = ledger.apply_block(Height::new(1), &block)?;
    ledger.check_supply()?;
    assert_eq!(ledger.account(sender).balance(), Amount::new(79));
    assert_eq!(ledger.account(recipient).balance(), Amount::new(20));
    assert_eq!(ledger.reward_pool(), Amount::new(1));
    let after = ledger.clone();
    assert!(ledger.apply_block(Height::new(2), &block).is_err());
    assert_eq!(ledger, after);
    let producer =
        ParticipantRecordV1::new(Some(sender_public), None, Height::new(0), None)?.participant_id();
    let mut voters = Vec::new();
    for seed in [10, 11, 12] {
        let key = ed25519::public_key(&Ed25519PrivateKey::from_seed([seed; 32]))?;
        voters.push(
            ParticipantRecordV1::new(None, Some(key), Height::new(0), None)?.participant_id(),
        );
    }
    let committee = LocalnetCommittee::new(mode, producer, voters.clone().try_into().unwrap())?;
    assert!(committee.has_quorum(&voters[..2])?);
    assert!(!committee.has_quorum(&voters[..1])?);
    let evidence = LocalnetNoneEvidence::new(NetworkClass::Localnet)?;
    evidence.verify_for_network(NetworkClass::Localnet)?;
    println!("LOCALNET v0 accounting demo; public test fixtures; no network or finality claim");
    println!("transfer_id={}", result[0].transfer_id);
    println!("sender=79 recipient=20 reward_pool=1 total_supply=100 actual_fee=1 FeeLimit=50");
    println!("rejected_replay_preserves_state=true authenticated_vote_tally=2/3 NONE=850100004040");
    Ok(())
}
