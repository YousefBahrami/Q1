// TEST ONLY — NEVER USE FOR VALUE-BEARING NETWORKS. Deterministic fixture seeds are public.
//! Transfer schema, signature-binding, and malformed-input regression tests.

use proptest::prelude::*;
use q1_primitives::{
    Address, Amount, Ed25519PrivateKey, Height, Network,
    cbor::{self, Value},
    ed25519,
};
use q1_protocol_types::{
    Error,
    address::AddressEnvelope,
    transfer::{SignedTransferV1, TransferBodyV1, TransferFields},
    types::{ChainId, FeeLimit, Nonce},
};

fn fields() -> TransferFields {
    let sender = Ed25519PrivateKey::from_seed([7; 32]);
    let recipient = ed25519::public_key(&Ed25519PrivateKey::from_seed([8; 32])).unwrap();
    TransferFields {
        chain_id: ChainId::from_bytes([9; 32]),
        sender_public_key: ed25519::public_key(&sender).unwrap(),
        recipient_address: AddressEnvelope::from_address(
            Address::from_public_key(Network::Localnet, recipient).unwrap(),
        ),
        amount: Amount::new(123),
        fee_limit: FeeLimit::new(4),
        nonce: Nonce::new(5),
        valid_from_height: Height::new(6),
        valid_until_height: Height::new(100),
    }
}
fn signed() -> SignedTransferV1 {
    SignedTransferV1::sign(
        TransferBodyV1::new(fields()).unwrap(),
        &Ed25519PrivateKey::from_seed([7; 32]),
    )
    .unwrap()
}
fn mutate_body(index: usize, value: Value) -> Vec<u8> {
    let Value::Array(mut values) = cbor::decode(
        &TransferBodyV1::new(fields())
            .unwrap()
            .encode_canonical()
            .unwrap(),
    )
    .unwrap() else {
        panic!()
    };
    values[index] = value;
    cbor::encode(&Value::Array(values)).unwrap()
}

#[test]
fn signature_binds_every_body_field_and_the_chain() {
    let tx = signed();
    tx.verify_for_chain(fields().chain_id).unwrap();
    assert_eq!(
        tx.verify_for_chain(ChainId::from_bytes([10; 32])),
        Err(Error::WrongChain)
    );
    let original_id = tx.id().unwrap();
    for index in 1..9 {
        let mut changed = fields();
        match index {
            1 => changed.chain_id = ChainId::from_bytes([10; 32]),
            2 => {
                changed.sender_public_key =
                    ed25519::public_key(&Ed25519PrivateKey::from_seed([11; 32])).unwrap()
            }
            3 => {
                changed.recipient_address = AddressEnvelope::from_address(
                    Address::from_public_key(Network::Localnet, changed.sender_public_key).unwrap(),
                )
            }
            4 => changed.amount = Amount::new(124),
            5 => changed.fee_limit = FeeLimit::new(5),
            6 => changed.nonce = Nonce::new(6),
            7 => changed.valid_from_height = Height::new(7),
            8 => changed.valid_until_height = Height::new(101),
            _ => unreachable!(),
        }
        let body = TransferBodyV1::new(changed).unwrap();
        let envelope = cbor::encode(&Value::Array(vec![
            cbor::decode(&body.encode_canonical().unwrap()).unwrap(),
            Value::Unsigned(1),
            Value::Bytes(tx.signature().as_bytes().to_vec()),
        ]))
        .unwrap();
        let mutated = SignedTransferV1::decode_canonical(&envelope).unwrap();
        assert!(mutated.verify().is_err(), "unbound field {index}");
        assert_ne!(mutated.id().unwrap(), original_id);
    }
}

#[test]
fn envelope_signature_and_algorithm_are_not_ignored() {
    let tx = signed();
    let original = tx.encode_canonical().unwrap();
    let Value::Array(mut envelope) = cbor::decode(&original).unwrap() else {
        panic!()
    };
    let Value::Bytes(signature) = &mut envelope[2] else {
        panic!()
    };
    signature[0] ^= 1;
    let mutated =
        SignedTransferV1::decode_canonical(&cbor::encode(&Value::Array(envelope.clone())).unwrap())
            .unwrap();
    assert!(mutated.verify().is_err());
    assert_ne!(mutated.id().unwrap(), tx.id().unwrap());
    envelope[1] = Value::Unsigned(2);
    assert!(
        SignedTransferV1::decode_canonical(&cbor::encode(&Value::Array(envelope)).unwrap())
            .is_err()
    );
    let wrong_key = Ed25519PrivateKey::from_seed([8; 32]);
    assert_eq!(
        SignedTransferV1::sign(tx.body().clone(), &wrong_key),
        Err(Error::KeyMismatch)
    );
    assert_eq!(SignedTransferV1::decode_canonical(&original).unwrap(), tx);
    for length in 0..original.len() {
        assert!(SignedTransferV1::decode_canonical(&original[..length]).is_err());
    }
    let mut trailing = original;
    trailing.push(0);
    assert!(SignedTransferV1::decode_canonical(&trailing).is_err());
}

#[test]
fn exact_schema_rejects_alternatives_but_does_not_invent_execution_policy() {
    for (index, value) in [
        (0, Value::Unsigned(0)),
        (0, Value::Unsigned(2)),
        (1, Value::Bytes(vec![0; 31])),
        (2, Value::Bytes(vec![0; 32])),
        (3, Value::Text("q1l".into())),
        (4, Value::Unsigned(123)),
        (5, Value::Bytes(vec![0; 15])),
        (6, Value::Null),
        (7, Value::Unsigned(101)),
    ] {
        assert!(TransferBodyV1::decode_canonical(&mutate_body(index, value)).is_err());
    }
    let mut zero = fields();
    zero.amount = Amount::ZERO;
    zero.fee_limit = FeeLimit::ZERO;
    zero.nonce = Nonce::new(0);
    zero.valid_until_height = zero.valid_from_height;
    assert!(TransferBodyV1::new(zero).is_ok()); // Structural validity, not execution validity.
    let encoded = TransferBodyV1::new(fields())
        .unwrap()
        .encode_canonical()
        .unwrap();
    let Value::Array(mut values) = cbor::decode(&encoded).unwrap() else {
        panic!()
    };
    values.push(Value::Null);
    assert!(
        TransferBodyV1::decode_canonical(&cbor::encode(&Value::Array(values.clone())).unwrap())
            .is_err()
    );
    values.truncate(8);
    assert!(
        TransferBodyV1::decode_canonical(&cbor::encode(&Value::Array(values)).unwrap()).is_err()
    );
    let mut overlong_version = encoded;
    overlong_version.splice(1..2, [0x18, 0x01]);
    assert!(TransferBodyV1::decode_canonical(&overlong_version).is_err());
}

#[test]
fn binary_address_has_no_invented_network_bytes() {
    let envelope = fields().recipient_address;
    let local = envelope.to_address(Network::Localnet).unwrap();
    let private = envelope.to_address(Network::PrivateTestnet).unwrap();
    assert_ne!(local.encode(), private.encode());
    assert_eq!(local.to_payload(), private.to_payload());
    for index in 0..4 {
        let mut invalid = *envelope.as_bytes();
        invalid[index] = 9;
        assert!(AddressEnvelope::from_bytes(invalid).is_err());
    }
    assert_eq!(
        AddressEnvelope::decode_canonical(&envelope.encode_canonical().unwrap()).unwrap(),
        envelope
    );
}

proptest! {
    #[test]
    fn body_round_trips_full_width_fields(amount in any::<u128>(), fee in any::<u128>(), nonce in any::<u64>(), height in any::<u64>()) {
        let mut fields = fields();
        fields.amount = Amount::new(amount);
        fields.fee_limit = FeeLimit::new(fee);
        fields.nonce = Nonce::new(nonce);
        fields.valid_from_height = Height::new(height);
        fields.valid_until_height = Height::new(height);
        let body = TransferBodyV1::new(fields).unwrap();
        prop_assert_eq!(TransferBodyV1::decode_canonical(&body.encode_canonical().unwrap()).unwrap(), body);
    }
    #[test]
    fn object_decoders_never_panic(bytes in prop::collection::vec(any::<u8>(), 0..4096)) {
        let _ = TransferBodyV1::decode_canonical(&bytes);
        let _ = SignedTransferV1::decode_canonical(&bytes);
    }
}
