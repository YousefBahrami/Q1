//! Fixed-byte vectors and structural/contextual rejection checks for parent references.

use q1_primitives::{
    Height,
    cbor::{self, Value},
};
use q1_protocol_types::parent::{BlockId, GenesisId, ParentReferenceV1};

#[test]
fn fixed_canonical_parent_vectors() {
    // Independently written CBOR: array(3), version 1, kind, bytes(32).
    let expected = [
        0x83, 0x01, 0x01, 0x58, 0x20, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
        0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19,
        0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
    ];
    let id = core::array::from_fn(|i| (i + 1) as u8);
    let genesis = ParentReferenceV1::Genesis(GenesisId::from_bytes(id));
    assert_eq!(genesis.encode_canonical().unwrap(), expected);
    assert_eq!(
        ParentReferenceV1::decode_canonical(&expected).unwrap(),
        genesis
    );
    let mut block_expected = expected;
    block_expected[2] = 2;
    let block = ParentReferenceV1::Block(BlockId::from_bytes(id));
    assert_eq!(block.encode_canonical().unwrap(), block_expected);
    assert_eq!(
        ParentReferenceV1::decode_canonical(&block_expected).unwrap(),
        block
    );
    assert_ne!(genesis, block);
}

#[test]
fn validates_governing_genesis_and_exact_immediate_parent() {
    let genesis_id = GenesisId::from_bytes([1; 32]);
    let genesis = ParentReferenceV1::Genesis(genesis_id);
    let block_id = BlockId::from_bytes([2; 32]);
    let block = ParentReferenceV1::Block(block_id);
    assert!(
        genesis
            .validate_for_height(Height::new(1), genesis_id, None)
            .is_ok()
    );
    assert!(
        genesis
            .validate_for_height(Height::ZERO, genesis_id, None)
            .is_err()
    );
    assert!(
        genesis
            .validate_for_height(Height::new(1), GenesisId::from_bytes([3; 32]), None)
            .is_err()
    );
    assert!(
        genesis
            .validate_for_height(Height::new(2), genesis_id, None)
            .is_err()
    );
    assert!(
        block
            .validate_for_height(Height::new(1), genesis_id, Some((Height::ZERO, block_id)))
            .is_err()
    );
    assert!(
        block
            .validate_for_height(Height::new(2), genesis_id, Some((Height::new(1), block_id)))
            .is_ok()
    );
    assert!(
        block
            .validate_for_height(Height::new(2), genesis_id, None)
            .is_err()
    );
    assert!(
        block
            .validate_for_height(Height::new(2), genesis_id, Some((Height::ZERO, block_id)))
            .is_err()
    );
    assert!(
        block
            .validate_for_height(Height::new(2), genesis_id, Some((Height::new(2), block_id)))
            .is_err()
    );
    assert!(
        block
            .validate_for_height(
                Height::new(2),
                genesis_id,
                Some((Height::new(1), BlockId::from_bytes([3; 32])))
            )
            .is_err()
    );
    assert!(
        block
            .validate_for_height(
                Height::MAX,
                genesis_id,
                Some((Height::new(u64::MAX - 1), block_id))
            )
            .is_ok()
    );
    assert!(
        block
            .validate_for_height(Height::new(2), genesis_id, Some((Height::MAX, block_id)))
            .is_err()
    );
}

#[test]
fn rejects_malformed_records_and_noncanonical_cbor() {
    let record = || {
        vec![
            Value::Unsigned(1),
            Value::Unsigned(1),
            Value::Bytes(vec![1; 32]),
        ]
    };
    let mut rejected = vec![Value::Null, Value::Bytes(vec![0; 32]), Value::Array(vec![])];
    for version in [0, 2, 65536] {
        let mut fields = record();
        fields[0] = Value::Unsigned(version);
        rejected.push(Value::Array(fields));
    }
    for kind in [0, 3, 65536, u64::MAX] {
        let mut fields = record();
        fields[1] = Value::Unsigned(kind);
        rejected.push(Value::Array(fields));
    }
    for id in [
        Value::Null,
        Value::Unsigned(0),
        Value::Bytes(vec![1; 31]),
        Value::Bytes(vec![1; 33]),
    ] {
        let mut fields = record();
        fields[2] = id;
        rejected.push(Value::Array(fields));
    }
    for index in 0..2 {
        let mut fields = record();
        fields[index] = Value::Null;
        rejected.push(Value::Array(fields));
    }
    let mut short = record();
    short.pop();
    rejected.push(Value::Array(short));
    let mut long = record();
    long.push(Value::Null);
    rejected.push(Value::Array(long));
    for value in rejected {
        assert!(ParentReferenceV1::decode_canonical(&cbor::encode(&value).unwrap()).is_err());
    }
    let valid = cbor::encode(&Value::Array(record())).unwrap();
    for length in 0..valid.len() {
        assert!(ParentReferenceV1::decode_canonical(&valid[..length]).is_err());
    }
    let mut trailing = valid.clone();
    trailing.push(0);
    assert!(ParentReferenceV1::decode_canonical(&trailing).is_err());
    let mut nonminimal = vec![0x83, 0x18, 0x01];
    nonminimal.extend_from_slice(&valid[2..]);
    assert!(ParentReferenceV1::decode_canonical(&nonminimal).is_err());
}

#[test]
fn zero_bytes_never_act_as_a_genesis_sentinel() {
    let block = ParentReferenceV1::Block(BlockId::from_bytes([0; 32]));
    // Zero is not a special hash value: the tag remains BLOCK, including at height one.
    let decoded = ParentReferenceV1::decode_canonical(&block.encode_canonical().unwrap()).unwrap();
    assert!(
        decoded
            .validate_for_height(Height::new(1), GenesisId::from_bytes([1; 32]), None)
            .is_err()
    );
    assert!(
        decoded
            .validate_for_height(
                Height::new(2),
                GenesisId::from_bytes([1; 32]),
                Some((Height::new(1), BlockId::from_bytes([2; 32])))
            )
            .is_err()
    );
    assert!(ParentReferenceV1::decode_canonical(&[0; 32]).is_err());
}
