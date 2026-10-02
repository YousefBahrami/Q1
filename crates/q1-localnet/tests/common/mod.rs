// TEST ONLY — NEVER USE FOR VALUE-BEARING NETWORKS. Deterministic fixture seeds are public.
#![allow(dead_code)]
use q1_localnet::{
    block::{Attestation, Certificate, CertifiedBlock, Chain, Proposal},
    genesis::Genesis,
};
use q1_primitives::{Address, Amount, Ed25519PrivateKey, Height, Network, ed25519};
use q1_protocol_types::{
    address::AddressEnvelope,
    block_body::BlockBodyV1,
    chain::{ChainIdentityPreimageV1, NetworkClass},
    participant::ParticipantRecordV1,
    transfer::{SignedTransferV1, TransferBodyV1, TransferFields},
    types::{FeeLimit, Nonce, ParticipantId},
};
pub fn key(seed: u8) -> Ed25519PrivateKey {
    Ed25519PrivateKey::from_seed([seed; 32])
}
pub fn address(seed: u8) -> AddressEnvelope {
    AddressEnvelope::from_address(
        Address::from_public_key(Network::Localnet, ed25519::public_key(&key(seed)).unwrap())
            .unwrap(),
    )
}
pub fn voter(seed: u8) -> ParticipantId {
    ParticipantRecordV1::new(
        None,
        Some(ed25519::public_key(&key(seed)).unwrap()),
        Height::ZERO,
        None,
    )
    .unwrap()
    .participant_id()
}
pub fn genesis() -> Genesis {
    let producer = ParticipantRecordV1::new(
        Some(ed25519::public_key(&key(11)).unwrap()),
        None,
        Height::ZERO,
        None,
    )
    .unwrap();
    let mut records = vec![producer.clone()];
    let mut voters = Vec::new();
    for seed in [12, 13, 14] {
        let record = ParticipantRecordV1::new(
            None,
            Some(ed25519::public_key(&key(seed)).unwrap()),
            Height::ZERO,
            None,
        )
        .unwrap();
        voters.push(record.participant_id());
        records.push(record);
    }
    records.sort_by_key(|p| p.participant_id());
    voters.sort();
    let mut allocations = vec![(address(7), Amount::new(1000)), (address(8), Amount::ZERO)];
    allocations.sort_by_key(|(a, _)| *a);
    Genesis::new(
        ChainIdentityPreimageV1::new(NetworkClass::Localnet, [9; 32]),
        Amount::new(1000),
        allocations,
        records,
        producer.participant_id(),
        voters.try_into().unwrap(),
    )
    .unwrap()
}
pub fn transfer(genesis: &Genesis, nonce: u64, amount: u128) -> SignedTransferV1 {
    SignedTransferV1::sign(
        TransferBodyV1::new(TransferFields {
            chain_id: genesis.chain_id(),
            sender_public_key: ed25519::public_key(&key(7)).unwrap(),
            recipient_address: address(8),
            amount: Amount::new(amount),
            fee_limit: FeeLimit::new(99),
            nonce: Nonce::new(nonce),
            valid_from_height: Height::new(1),
            valid_until_height: Height::new(100),
        })
        .unwrap(),
        &key(7),
    )
    .unwrap()
}
pub fn proposal(chain: &Chain, nonce: u64, amount: u128) -> Proposal {
    chain
        .propose(
            BlockBodyV1::new(vec![transfer(chain.genesis(), nonce, amount)]).unwrap(),
            &key(11),
        )
        .unwrap()
}
pub fn certify(genesis: &Genesis, proposal: Proposal) -> CertifiedBlock {
    let mut votes = vec![
        Attestation::sign(genesis, &proposal, voter(12), &key(12)).unwrap(),
        Attestation::sign(genesis, &proposal, voter(13), &key(13)).unwrap(),
    ];
    votes.sort_by_key(|v| v.voter());
    CertifiedBlock::new(
        proposal.clone(),
        Certificate::new(genesis, &proposal, votes).unwrap(),
    )
}
pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
pub fn tempdir() -> std::path::PathBuf {
    static SERIAL: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let path = std::env::temp_dir().join(format!(
        "q1-tests-{}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos(),
        SERIAL.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ));
    std::fs::create_dir(&path).unwrap();
    path
}
