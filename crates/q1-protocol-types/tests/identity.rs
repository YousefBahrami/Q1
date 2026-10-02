// TEST ONLY — NEVER USE FOR VALUE-BEARING NETWORKS. Deterministic fixture seeds are public.
//! Identity conformance fixtures independently reproduced with Python hashlib.

use q1_primitives::{
    Ed25519PrivateKey, Ed25519PublicKey, Height,
    cbor::{self, Value},
    domain::{Domain, frame},
    ed25519,
};
use q1_protocol_types::{
    Error,
    chain::{ChainIdentityPreimageV1, NetworkClass},
    participant::{ParticipantIdentityBodyV1, ParticipantRecordV1, ParticipantSetV1},
    types::{CandidateIndex, ChainId, FeeLimit, Nonce, ParticipantCount, TransactionCount},
};

fn unhex(hex: &str) -> Vec<u8> {
    hex.as_bytes()
        .chunks_exact(2)
        .map(|pair| u8::from_str_radix(core::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect()
}
fn key(seed: u8) -> Ed25519PublicKey {
    ed25519::public_key(&Ed25519PrivateKey::from_seed([seed; 32])).unwrap()
}
fn fixture_key() -> Ed25519PublicKey {
    Ed25519PublicKey::try_from_slice(&unhex(
        "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a",
    ))
    .unwrap()
}
fn mutate(bytes: &[u8], index: usize, replacement: Value) -> Vec<u8> {
    let Value::Array(mut fields) = cbor::decode(bytes).unwrap() else {
        panic!()
    };
    fields[index] = replacement;
    cbor::encode(&Value::Array(fields)).unwrap()
}

#[test]
fn chain_canonical_bytes_frame_and_hash_match_independent_fixture() {
    let preimage =
        ChainIdentityPreimageV1::new(NetworkClass::Localnet, core::array::from_fn(|i| i as u8));
    let bytes = unhex("8301015820000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f");
    assert_eq!(preimage.encode_canonical().unwrap(), bytes);
    assert_eq!(
        ChainIdentityPreimageV1::decode_canonical(&bytes).unwrap(),
        preimage
    );
    assert_eq!(
        frame(Domain::ChainId, &bytes).unwrap(),
        unhex(
            "513144530001001000000000000000258301015820000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"
        )
    );
    assert_eq!(
        preimage.chain_id().unwrap().to_string(),
        "1e388dbee59dd017a042ad29141c330f00fcd178b9b09a5928bad438b947c864"
    );
    let other_class =
        ChainIdentityPreimageV1::new(NetworkClass::Research, *preimage.creation_nonce());
    assert_ne!(
        preimage.chain_id().unwrap(),
        other_class.chain_id().unwrap()
    );
    assert!("AB".repeat(32).parse::<ChainId>().is_err());
    assert!("ab".repeat(31).parse::<ChainId>().is_err());
    assert_eq!(
        preimage
            .chain_id()
            .unwrap()
            .to_string()
            .parse::<ChainId>()
            .unwrap(),
        preimage.chain_id().unwrap()
    );
}

#[test]
fn chain_rejects_reserved_classes_versions_shapes_and_noncanonical_encoding() {
    let bytes = ChainIdentityPreimageV1::new(NetworkClass::Localnet, [0; 32])
        .encode_canonical()
        .unwrap();
    for class in [0, 4, 65535, 65536, u64::MAX] {
        assert!(
            ChainIdentityPreimageV1::decode_canonical(&mutate(&bytes, 1, Value::Unsigned(class)))
                .is_err()
        );
    }
    for version in [0, 2, 65536] {
        assert_eq!(
            ChainIdentityPreimageV1::decode_canonical(&mutate(&bytes, 0, Value::Unsigned(version))),
            Err(Error::UnsupportedVersion(version))
        );
    }
    for nonce in [
        Value::Null,
        Value::Bytes(vec![0; 31]),
        Value::Bytes(vec![0; 33]),
    ] {
        assert!(ChainIdentityPreimageV1::decode_canonical(&mutate(&bytes, 2, nonce)).is_err());
    }
    for invalid in [
        vec![0x82, 1, 1],
        [bytes.clone(), vec![0]].concat(),
        [vec![0x83, 0x18, 1], bytes[2..].to_vec()].concat(),
    ] {
        assert!(ChainIdentityPreimageV1::decode_canonical(&invalid).is_err());
    }
    assert_eq!(Domain::try_from(0x10).unwrap(), Domain::ChainId);
    assert_eq!(Domain::try_from(0x13).unwrap(), Domain::ParticipantId);
    assert_eq!(Domain::try_from(0x11).unwrap(), Domain::LocalnetProposalId);
    assert!(Domain::try_from(0x12).is_err());
}

#[test]
fn participant_identity_and_record_have_distinct_fixed_commitments() {
    let identity = ParticipantIdentityBodyV1::new(Some(fixture_key()), None).unwrap();
    let identity_bytes =
        unhex("83015820d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511af6");
    assert_eq!(identity.encode_canonical().unwrap(), identity_bytes);
    assert_eq!(
        ParticipantIdentityBodyV1::decode_canonical(&identity_bytes).unwrap(),
        identity
    );
    assert_eq!(
        frame(Domain::ParticipantId, &identity_bytes).unwrap(),
        unhex(
            "5131445300010013000000000000002583015820d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511af6"
        )
    );
    assert_eq!(
        identity.participant_id().unwrap().to_string(),
        "604d8632bccd1b96f2054dd76420af91e7fd8a099437c39a7f1278c85ada3c3b"
    );
    let record = ParticipantRecordV1::new(Some(fixture_key()), None, Height::ZERO, None).unwrap();
    let record_bytes = unhex(
        "86015820604d8632bccd1b96f2054dd76420af91e7fd8a099437c39a7f1278c85ada3c3b5820d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511af600f6",
    );
    assert_eq!(record.encode_canonical().unwrap(), record_bytes);
    assert_eq!(
        ParticipantRecordV1::decode_canonical(&record_bytes).unwrap(),
        record
    );
    assert_eq!(
        record.record_hash().unwrap().to_string(),
        "79e769ac0ef7a4ec55f3b04abb8fedb7e472734e4fc2bb4707c708b23434fefb"
    );
    let scheduled = ParticipantRecordV1::new(
        Some(fixture_key()),
        None,
        Height::new(8),
        Some(Height::new(12)),
    )
    .unwrap();
    assert_eq!(record.participant_id(), scheduled.participant_id());
    assert_ne!(
        record.record_hash().unwrap(),
        scheduled.record_hash().unwrap()
    );
    let changed_role =
        ParticipantRecordV1::new(None, Some(fixture_key()), Height::ZERO, None).unwrap();
    assert_ne!(record.participant_id(), changed_role.participant_id());
    assert!(!scheduled.is_active_at(Height::new(7)));
    assert!(scheduled.is_active_at(Height::new(8)));
    assert!(scheduled.is_active_at(Height::new(11)));
    assert!(!scheduled.is_active_at(Height::new(12)));
}

#[test]
fn participant_rejects_bad_keys_identity_intervals_and_extra_fields() {
    assert!(ParticipantIdentityBodyV1::new(None, None).is_err());
    assert!(ParticipantIdentityBodyV1::new(Some(key(1)), Some(key(1))).is_err());
    for end in [0, 7, 8] {
        assert!(
            ParticipantRecordV1::new(Some(key(1)), None, Height::new(8), Some(Height::new(end)))
                .is_err()
        );
    }
    let record = ParticipantRecordV1::new(Some(key(1)), Some(key(2)), Height::ZERO, None).unwrap();
    let bytes = record.encode_canonical().unwrap();
    for (index, value) in [
        (0, Value::Unsigned(2)),
        (1, Value::Bytes(vec![0; 32])),
        (1, Value::Bytes(vec![0; 31])),
        (2, Value::Bytes(vec![0; 32])),
        (2, Value::Bytes(vec![0; 31])),
        (3, Value::Bytes(key(1).as_bytes().to_vec())),
        (4, Value::Null),
        (5, Value::Unsigned(0)),
    ] {
        assert!(ParticipantRecordV1::decode_canonical(&mutate(&bytes, index, value)).is_err());
    }
    let Value::Array(mut fields) = cbor::decode(&bytes).unwrap() else {
        panic!()
    };
    fields.push(Value::Null);
    assert!(
        ParticipantRecordV1::decode_canonical(&cbor::encode(&Value::Array(fields)).unwrap())
            .is_err()
    );
    let bytes = mutate(&mutate(&bytes, 2, Value::Null), 3, Value::Null);
    assert!(ParticipantRecordV1::decode_canonical(&bytes).is_err());
}

#[test]
fn participant_set_rejects_unsorted_duplicate_reused_keys_and_inactive_members() {
    let a = ParticipantRecordV1::new(Some(key(1)), None, Height::ZERO, None).unwrap();
    let b =
        ParticipantRecordV1::new(None, Some(key(2)), Height::ZERO, Some(Height::new(9))).unwrap();
    let mut records = vec![a.clone(), b.clone()];
    records.sort_by_key(ParticipantRecordV1::participant_id);
    let set = ParticipantSetV1::new(Height::new(8), records.clone()).unwrap();
    assert_eq!(set.count().get(), 2);
    assert_eq!(
        ParticipantSetV1::decode_canonical(&set.encode_canonical().unwrap()).unwrap(),
        set
    );
    assert!(ParticipantSetV1::new(Height::new(9), records.clone()).is_err());
    records.reverse();
    assert_eq!(
        ParticipantSetV1::new(Height::new(8), records),
        Err(Error::UnsortedOrDuplicate)
    );
    assert!(ParticipantSetV1::new(Height::ZERO, vec![]).is_err());
    assert_eq!(
        ParticipantSetV1::new(Height::ZERO, vec![a.clone(), a.clone()]),
        Err(Error::UnsortedOrDuplicate)
    );
    let reused = ParticipantRecordV1::new(None, Some(key(1)), Height::ZERO, None).unwrap();
    let mut records = vec![a, reused];
    records.sort_by_key(ParticipantRecordV1::participant_id);
    assert_eq!(
        ParticipantSetV1::new(Height::ZERO, records),
        Err(Error::UnsortedOrDuplicate)
    );
    let wire = set.encode_canonical().unwrap();
    let Value::Array(mut reverse_records) = cbor::decode(&wire).unwrap() else {
        panic!()
    };
    let Value::Array(ref mut entries) = reverse_records[2] else {
        panic!()
    };
    entries.reverse();
    assert!(
        ParticipantSetV1::decode_canonical(&cbor::encode(&Value::Array(reverse_records)).unwrap())
            .is_err()
    );
}

#[test]
fn participant_set_fixture_has_no_serialized_count() {
    let record = ParticipantRecordV1::new(Some(fixture_key()), None, Height::ZERO, None).unwrap();
    let set = ParticipantSetV1::new(Height::ZERO, vec![record]).unwrap();
    assert_eq!(
        set.encode_canonical().unwrap(),
        unhex(
            "8301008186015820604d8632bccd1b96f2054dd76420af91e7fd8a099437c39a7f1278c85ada3c3b5820d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511af600f6"
        )
    );
}

#[test]
fn semantic_types_enforce_widths_arithmetic_and_nonzero_participant_count() {
    assert_eq!(
        FeeLimit::new(1).encode_canonical().unwrap(),
        unhex("5000000000000000000000000000000001")
    );
    for value in [FeeLimit::ZERO, FeeLimit::new(1), FeeLimit::MAX] {
        assert_eq!(
            FeeLimit::decode_canonical(&value.encode_canonical().unwrap()).unwrap(),
            value
        );
    }
    for bytes in [vec![0], vec![0x50], vec![0x4f; 16], vec![0x51; 18]] {
        assert!(FeeLimit::decode_canonical(&bytes).is_err());
    }
    assert!(FeeLimit::MAX.checked_add(FeeLimit::new(1)).is_err());
    assert!(FeeLimit::ZERO.checked_sub(FeeLimit::new(1)).is_err());
    assert!(FeeLimit::MAX.checked_mul(2).is_err());
    assert!(FeeLimit::new(1).checked_div(0).is_err());
    assert_eq!(FeeLimit::new(9).checked_div(2).unwrap(), FeeLimit::new(4));
    assert_eq!(
        Nonce::new(u64::MAX).encode_canonical().unwrap(),
        vec![0x1b, 255, 255, 255, 255, 255, 255, 255, 255]
    );
    assert!(Nonce::decode_canonical(&[0x18, 0]).is_err());
    assert!(CandidateIndex::decode_canonical(&cbor::encode_unsigned(65536)).is_err());
    assert_eq!(
        CandidateIndex::new(65535).encode_canonical().unwrap(),
        vec![0x19, 255, 255]
    );
    assert!(ParticipantCount::new(0).is_err());
    assert_eq!(TransactionCount::new(0).get(), 0);
}
