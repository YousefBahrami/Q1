// TEST ONLY — NEVER USE FOR VALUE-BEARING NETWORKS. Deterministic fixture seeds are public.
//! Independent preimage, sequence, and canonical-owner conformance checks.

use q1_primitives::{
    Address, Ed25519PrivateKey, Network,
    cbor::{self, Value},
    ed25519,
};
use q1_protocol_types::{
    block_body::BlockBodyV1,
    merkle::{ParticipantRoot, TransactionRoot},
    participant::{ParticipantRecordV1, ParticipantSetV1},
    transfer::SignedTransferV1,
};
use sha2::{Digest, Sha256};

// These helpers write the approved preimages independently of Q1's domain and
// Merkle adapters. Tree shapes below are explicit, rather than a second loop.
fn frame(domain: u16, payload: &[u8]) -> Vec<u8> {
    let mut bytes = b"Q1DS\x00\x01".to_vec();
    bytes.extend(domain.to_be_bytes());
    bytes.extend((payload.len() as u64).to_be_bytes());
    bytes.extend(payload);
    bytes
}

fn hash(domain: u16, payload: &[u8]) -> [u8; 32] {
    Sha256::digest(frame(domain, payload)).into()
}

fn leaf(profile: u16, index: u64, bytes: &[u8]) -> [u8; 32] {
    let mut payload = profile.to_be_bytes().to_vec();
    payload.extend(index.to_be_bytes());
    payload.extend((bytes.len() as u64).to_be_bytes());
    payload.extend(bytes);
    hash(9, &payload)
}

fn pair(profile: u16, left: [u8; 32], right: [u8; 32]) -> [u8; 32] {
    let mut payload = profile.to_be_bytes().to_vec();
    payload.extend(left);
    payload.extend(right);
    hash(10, &payload)
}

fn transfer(nonce: u64) -> SignedTransferV1 {
    let key = Ed25519PrivateKey::from_seed([41; 32]);
    let public = ed25519::public_key(&key).unwrap();
    let recipient = Address::from_public_key(Network::Localnet, public).unwrap();
    let body = Value::Array(vec![
        Value::Unsigned(1),
        Value::Bytes(vec![7; 32]),
        Value::Bytes(public.as_bytes().to_vec()),
        Value::Bytes(recipient.to_payload().to_vec()),
        Value::Bytes(100_u128.to_be_bytes().to_vec()),
        Value::Bytes(0_u128.to_be_bytes().to_vec()),
        Value::Unsigned(nonce),
        Value::Unsigned(0),
        Value::Unsigned(100),
    ]);
    let payload = frame(1, &cbor::encode(&body).unwrap());
    let signature = ed25519::sign(&key, &payload);
    let signed = Value::Array(vec![
        body,
        Value::Unsigned(1),
        Value::Bytes(signature.as_bytes().to_vec()),
    ]);
    SignedTransferV1::decode_canonical(&cbor::encode(&signed).unwrap()).unwrap()
}

fn root(transfers: Vec<SignedTransferV1>) -> TransactionRoot {
    TransactionRoot::from_block_body(&BlockBodyV1::new(transfers).unwrap()).unwrap()
}

#[test]
fn empty_body_has_exact_bytes_and_typed_empty_root() {
    let body = BlockBodyV1::new(vec![]).unwrap();
    assert_eq!(body.encode_canonical().unwrap(), [0x82, 0x01, 0x80]);
    assert_eq!(body.transaction_count().get(), 0);
    let empty = hash(9, &[0, 1, 0, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(
        TransactionRoot::from_block_body(&body).unwrap().as_bytes(),
        &empty
    );
    assert_ne!(empty, leaf(1, 0, &[]));
    assert_ne!(empty, hash(9, &[0, 3, 0, 0, 0, 0, 0, 0, 0, 0]));
}

#[test]
fn one_two_three_and_five_leaves_match_explicit_independent_tree_shapes() {
    let items: Vec<_> = (0..5).map(transfer).collect();
    let leaves: Vec<_> = items
        .iter()
        .enumerate()
        .map(|(index, item)| leaf(1, index as u64, &item.encode_canonical().unwrap()))
        .collect();
    let ab = pair(1, leaves[0], leaves[1]);
    let cd = pair(1, leaves[2], leaves[3]);
    let abc = pair(1, ab, leaves[2]);
    let abcde = pair(1, pair(1, ab, cd), leaves[4]);
    for (count, expected) in [(1, leaves[0]), (2, ab), (3, abc), (5, abcde)] {
        assert_eq!(root(items[..count].to_vec()).as_bytes(), &expected);
    }
    // Duplicate-last would produce this different three-leaf root.
    assert_ne!(abc, pair(1, ab, pair(1, leaves[2], leaves[2])));
}

#[test]
fn body_preserves_order_duplicates_and_derived_count() {
    let a = transfer(0);
    let b = transfer(1);
    let body = BlockBodyV1::new(vec![a.clone(), b.clone(), a.clone()]).unwrap();
    let encoded = body.encode_canonical().unwrap();
    assert_eq!(BlockBodyV1::decode_canonical(&encoded).unwrap(), body);
    assert_eq!(body.transfers(), &[a.clone(), b.clone(), a.clone()]);
    assert_eq!(body.transaction_count().get(), 3);
    assert_ne!(root(vec![a.clone(), b.clone()]), root(vec![b, a.clone()]));
    assert_ne!(root(vec![a.clone()]), root(vec![a.clone(), a.clone()]));
    let bytes = a.encode_canonical().unwrap();
    assert_ne!(leaf(1, 0, &bytes), leaf(1, 1, &bytes));
    assert_ne!(leaf(1, 0, &bytes), leaf(3, 0, &bytes));
}

#[test]
fn body_rejects_invalid_shape_version_and_noncanonical_bytes() {
    for invalid in [
        vec![0x81, 1],                   // Missing transfers.
        vec![0x83, 1, 0x80, 0xf6],       // Unknown third field.
        vec![0x82, 0, 0x80],             // Version zero.
        vec![0x82, 2, 0x80],             // Future version.
        vec![0x82, 1, 0xf6],             // Null transfers.
        vec![0x82, 1, 0x81, 0x80],       // Malformed transfer.
        vec![0x82, 0x18, 1, 0x80],       // Nonminimal version.
        vec![0x82, 1, 0x9f, 0xff],       // Indefinite array.
        vec![0x82, 1, 0x80, 0],          // Trailing bytes.
        vec![0x82, 1, 0x99, 0xff, 0xff], // Truncated declared array.
    ] {
        assert!(
            BlockBodyV1::decode_canonical(&invalid).is_err(),
            "{invalid:x?}"
        );
    }
}

fn participant(seed: u8) -> ParticipantRecordV1 {
    let key = ed25519::public_key(&Ed25519PrivateKey::from_seed([seed; 32])).unwrap();
    let identity = Value::Array(vec![
        Value::Unsigned(1),
        Value::Bytes(key.as_bytes().to_vec()),
        Value::Null,
    ]);
    let id = hash(0x13, &cbor::encode(&identity).unwrap());
    let record = Value::Array(vec![
        Value::Unsigned(1),
        Value::Bytes(id.to_vec()),
        Value::Bytes(key.as_bytes().to_vec()),
        Value::Null,
        Value::Unsigned(0),
        Value::Null,
    ]);
    ParticipantRecordV1::decode_canonical(&cbor::encode(&record).unwrap()).unwrap()
}

#[test]
fn participant_root_commits_record_bytes_and_preserves_owning_set_context() {
    use q1_primitives::Height;
    let mut records = vec![participant(51), participant(52), participant(53)];
    records.sort_by_key(|record| record.participant_id());
    let leaves: Vec<_> = records
        .iter()
        .enumerate()
        .map(|(index, record)| leaf(3, index as u64, &record.encode_canonical().unwrap()))
        .collect();
    let set = ParticipantSetV1::new(Height::new(1), records.clone()).unwrap();
    assert_eq!(set.count().get(), 3);
    let actual = ParticipantRoot::from_participant_set(&set).unwrap();
    assert_eq!(
        actual.as_bytes(),
        &pair(3, pair(3, leaves[0], leaves[1]), leaves[2])
    );
    // reference_height is owner context, not an extra Merkle leaf or payload.
    let later = ParticipantSetV1::new(Height::new(2), records.clone()).unwrap();
    assert_eq!(
        actual,
        ParticipantRoot::from_participant_set(&later).unwrap()
    );
    assert_ne!(
        set.encode_canonical().unwrap(),
        later.encode_canonical().unwrap()
    );
    assert_eq!(
        ParticipantSetV1::decode_canonical(&set.encode_canonical().unwrap()).unwrap(),
        set
    );
    let bytes = records[0].encode_canonical().unwrap();
    let hashed_record = hash(8, &bytes);
    let single = ParticipantSetV1::new(Height::new(1), vec![records[0].clone()]).unwrap();
    assert_eq!(
        ParticipantRoot::from_participant_set(&single)
            .unwrap()
            .as_bytes(),
        &leaves[0]
    );
    assert_ne!(leaves[0], leaf(3, 0, &hashed_record));
    assert!(ParticipantSetV1::new(Height::new(1), vec![]).is_err());
    assert!(
        ParticipantSetV1::new(Height::new(1), vec![records[0].clone(), records[0].clone()])
            .is_err()
    );
    records.reverse();
    assert!(ParticipantSetV1::new(Height::new(1), records).is_err());
}
