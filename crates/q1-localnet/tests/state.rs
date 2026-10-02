//! Full-state commitment field sensitivity, canonical order and reload tests.
mod common;
use common::*;
use q1_localnet::{LocalnetV0, genesis::Genesis, ledger::Ledger, state::StateSnapshot};
use q1_primitives::{
    Amount,
    cbor::{self, Value},
};
use q1_protocol_types::chain::NetworkClass;

#[test]
fn same_state_and_different_insertion_order_have_identical_bytes_and_root() {
    let genesis = genesis();
    let initial = genesis.initial_state().unwrap();
    let ledger = Ledger::from_allocations(
        LocalnetV0::new(NetworkClass::Localnet).unwrap(),
        genesis.chain_id(),
        genesis.total_supply(),
        genesis.allocations().iter().copied().rev(),
    )
    .unwrap();
    let reversed = StateSnapshot::new(&genesis, ledger).unwrap();
    assert_eq!(initial, reversed);
    assert_eq!(initial.root().unwrap(), reversed.root().unwrap());
    let bytes = initial.encode_canonical().unwrap();
    let decoded = StateSnapshot::decode_canonical(
        &bytes,
        &Genesis::decode_canonical(&genesis.encode_canonical().unwrap()).unwrap(),
    )
    .unwrap();
    assert_eq!(decoded.root().unwrap(), initial.root().unwrap());
    assert_eq!(decoded.encode_canonical().unwrap(), bytes);
}

#[test]
fn balance_pool_nonce_and_height_are_committed() {
    let genesis = genesis();
    let snapshot = genesis.initial_state().unwrap();
    let root = snapshot.root().unwrap();
    for change in 0..4 {
        let Value::Array(mut fields) = cbor::decode(&snapshot.encode_canonical().unwrap()).unwrap()
        else {
            panic!()
        };
        let Value::Array(accounts) = &mut fields[6] else {
            panic!()
        };
        let sender=accounts.iter().position(|r|matches!(r,Value::Array(row) if row[1]==Value::Bytes(address(7).as_bytes().to_vec()))).unwrap();
        let recipient = 1 - sender;
        if change == 0 {
            let Value::Array(row) = &mut accounts[sender] else {
                panic!()
            };
            row[2] = Value::Bytes(Amount::new(999).to_be_bytes().to_vec());
            let Value::Array(row) = &mut accounts[recipient] else {
                panic!()
            };
            row[2] = Value::Bytes(Amount::new(1).to_be_bytes().to_vec());
        } else if change == 1 {
            let Value::Array(row) = &mut accounts[sender] else {
                panic!()
            };
            row[2] = Value::Bytes(Amount::new(999).to_be_bytes().to_vec());
            fields[5] = Value::Bytes(Amount::new(1).to_be_bytes().to_vec());
        } else if change == 2 {
            let Value::Array(row) = &mut accounts[sender] else {
                panic!()
            };
            row[3] = Value::Unsigned(1);
        } else {
            fields[3] = Value::Unsigned(1);
        }
        let changed = StateSnapshot::decode_canonical(
            &cbor::encode(&Value::Array(fields)).unwrap(),
            &genesis,
        )
        .unwrap();
        assert_ne!(changed.root().unwrap(), root, "field mutation {change}");
    }
}

#[test]
fn malformed_snapshot_and_altered_authority_fail_closed() {
    let genesis = genesis();
    let snapshot = genesis.initial_state().unwrap();
    for index in [0, 1, 2, 4, 7] {
        let Value::Array(mut fields) = cbor::decode(&snapshot.encode_canonical().unwrap()).unwrap()
        else {
            panic!()
        };
        fields[index] = Value::Unsigned(99);
        assert!(
            StateSnapshot::decode_canonical(
                &cbor::encode(&Value::Array(fields)).unwrap(),
                &genesis
            )
            .is_err()
        );
    }
    let Value::Array(mut fields) = cbor::decode(&snapshot.encode_canonical().unwrap()).unwrap()
    else {
        panic!()
    };
    let Value::Array(ref mut accounts) = fields[6] else {
        panic!()
    };
    accounts.reverse();
    assert!(
        StateSnapshot::decode_canonical(&cbor::encode(&Value::Array(fields)).unwrap(), &genesis)
            .is_err()
    );
    let original = genesis.encode_canonical().unwrap();
    for position in [0, 1, 2, 8, 9] {
        let Value::Array(mut fields) = cbor::decode(&original).unwrap() else {
            panic!()
        };
        fields[position] = Value::Unsigned(0);
        assert!(Genesis::decode_canonical(&cbor::encode(&Value::Array(fields)).unwrap()).is_err());
    }
}

#[path = "common/vectors.rs"]
mod vectors;
#[test]
fn frozen_localnet_reference_vectors_match() {
    let generated: String = vectors::vectors()
        .into_iter()
        .map(|(name, bytes)| format!("{name}\t{}\n", hex(&bytes)))
        .collect();
    assert_eq!(
        generated,
        include_str!("../../../vectors/localnet/v0/approved.tsv")
    );
}
