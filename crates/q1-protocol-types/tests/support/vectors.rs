// TEST ONLY — NEVER USE FOR VALUE-BEARING NETWORKS. Deterministic fixture seeds are public.
use q1_primitives::{
    Address, Amount, Ed25519PrivateKey, Height, Network,
    domain::{self, Domain},
    ed25519,
};
use q1_protocol_types::{
    Result,
    address::AddressEnvelope,
    block_body::BlockBodyV1,
    chain::{ChainIdentityPreimageV1, NetworkClass},
    merkle::{ParticipantRoot, TransactionRoot},
    participant::{ParticipantIdentityBodyV1, ParticipantRecordV1, ParticipantSetV1},
    transfer::{SignedTransferV1, TransferBodyV1, TransferFields},
    types::{FeeLimit, Nonce},
};
use std::collections::BTreeMap;

pub fn vectors() -> Result<BTreeMap<&'static str, String>> {
    let mut output = BTreeMap::new();
    let mut put = |name, value: Vec<u8>| {
        output.insert(name, value.iter().map(|b| format!("{b:02x}")).collect());
    };
    let chain = ChainIdentityPreimageV1::new(NetworkClass::Localnet, [9; 32]);
    let chain_bytes = chain.encode_canonical()?;
    put("chain_frame", domain::frame(Domain::ChainId, &chain_bytes)?);
    put("chain_preimage", chain_bytes);
    put("chain_id", chain.chain_id()?.as_bytes().to_vec());
    let sender = Ed25519PrivateKey::from_seed([7; 32]);
    let recipient = Ed25519PrivateKey::from_seed([8; 32]);
    let sender_public = ed25519::public_key(&sender)?;
    let recipient_public = ed25519::public_key(&recipient)?;
    let identity = ParticipantIdentityBodyV1::new(Some(sender_public), None)?;
    put("participant_identity", identity.encode_canonical()?);
    let record = ParticipantRecordV1::new(Some(sender_public), None, Height::new(0), None)?;
    put(
        "participant_id",
        record.participant_id().as_bytes().to_vec(),
    );
    put("participant_record", record.encode_canonical()?);
    put(
        "participant_record_hash",
        record.record_hash()?.as_bytes().to_vec(),
    );
    let mut records = vec![
        record,
        ParticipantRecordV1::new(None, Some(recipient_public), Height::new(0), None)?,
    ];
    records.sort_by_key(|r| r.participant_id()); // Fixture preparation only; codec never sorts.
    let set = ParticipantSetV1::new(Height::new(1), records)?;
    put("participant_set", set.encode_canonical()?);
    put(
        "participant_root",
        ParticipantRoot::from_participant_set(&set)?
            .as_bytes()
            .to_vec(),
    );
    let fields = TransferFields {
        chain_id: chain.chain_id()?,
        sender_public_key: sender_public,
        recipient_address: AddressEnvelope::from_address(Address::from_public_key(
            Network::Localnet,
            recipient_public,
        )?),
        amount: Amount::new(123),
        fee_limit: FeeLimit::new(4),
        nonce: Nonce::new(5),
        valid_from_height: Height::new(6),
        valid_until_height: Height::new(100),
    };
    let body = TransferBodyV1::new(fields.clone())?;
    put("transfer_body", body.encode_canonical()?);
    put("transfer_signing_payload", body.signing_payload()?);
    let tx = SignedTransferV1::sign(body, &sender)?;
    put("transfer_signature", tx.signature().as_bytes().to_vec());
    put("signed_transfer", tx.encode_canonical()?);
    put("transfer_id", tx.id()?.as_bytes().to_vec());
    let mut second_fields = fields;
    second_fields.nonce = Nonce::new(6);
    let second = SignedTransferV1::sign(TransferBodyV1::new(second_fields)?, &sender)?;
    put(
        "transfer2_signature",
        second.signature().as_bytes().to_vec(),
    );
    put("transfer2_id", second.id()?.as_bytes().to_vec());
    let block = BlockBodyV1::new(vec![tx.clone(), second, tx])?;
    put("block_body", block.encode_canonical()?);
    put(
        "transaction_root",
        TransactionRoot::from_block_body(&block)?
            .as_bytes()
            .to_vec(),
    );
    let empty = BlockBodyV1::new(vec![])?;
    put("empty_block_body", empty.encode_canonical()?);
    put(
        "empty_transaction_root",
        TransactionRoot::from_block_body(&empty)?
            .as_bytes()
            .to_vec(),
    );
    let evidence = q1_protocol_types::delay::LocalnetNoneEvidence::new(NetworkClass::Localnet)?;
    put("delay_none", evidence.encode_canonical()?);
    put("delay_none_frame", evidence.commitment_frame()?);
    put(
        "delay_none_hash",
        evidence.commitment()?.as_bytes().to_vec(),
    );
    Ok(output)
}
