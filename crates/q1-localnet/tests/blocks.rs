//! Certified local block validation and all-or-nothing state updates.
mod common;
use common::*;
use q1_localnet::block::{Attestation, Certificate, CertifiedBlock, Chain, Proposal};
use q1_primitives::{
    cbor::{self, Value},
    domain::Domain,
    ed25519,
};

fn resign_proposal(mut envelope: Value) -> Proposal {
    let Value::Array(ref mut outer) = envelope else {
        panic!()
    };
    let Value::Array(ref mut body) = outer[0] else {
        panic!()
    };
    let Value::Array(ref mut header) = body[3] else {
        panic!()
    };
    header[2] = Value::Bytes(
        ed25519::sign_domain(
            &key(11),
            Domain::BlockHeaderSigning,
            &cbor::encode(&header[0]).unwrap(),
        )
        .unwrap()
        .as_bytes()
        .to_vec(),
    );
    outer[2] = Value::Bytes(
        ed25519::sign_domain(
            &key(11),
            Domain::ProposalSigning,
            &cbor::encode(&outer[0]).unwrap(),
        )
        .unwrap()
        .as_bytes()
        .to_vec(),
    );
    Proposal::decode_canonical(&cbor::encode(&envelope).unwrap()).unwrap()
}
#[test]
fn two_valid_voters_finalize_the_same_exact_state() {
    let genesis = genesis();
    let mut a = Chain::new(genesis.clone()).unwrap();
    let mut b = a.clone();
    let block = certify(&genesis, proposal(&a, 0, 20));
    a.commit(&block).unwrap();
    let decoded = CertifiedBlock::decode_canonical(&block.encode_canonical().unwrap()).unwrap();
    b.commit(&decoded).unwrap();
    assert_eq!(a, b);
    assert_eq!(a.state().ledger().height().get(), 1);
    assert_eq!(a.state().ledger().reward_pool().get(), 1);
    assert_eq!(a.state().ledger().account(address(7)).balance().get(), 979);
    assert_eq!(a.state().ledger().account(address(8)).balance().get(), 20);
    assert_eq!(
        a.state().root().unwrap(),
        block.proposal().header().body().state_root()
    );
}
#[test]
fn certificate_rejects_one_vote_duplicates_unknown_wrong_proposal_or_signature() {
    let genesis = genesis();
    let chain = Chain::new(genesis.clone()).unwrap();
    let p = proposal(&chain, 0, 20);
    let other = proposal(&chain, 0, 21);
    let vote = Attestation::sign(&genesis, &p, voter(12), &key(12)).unwrap();
    assert!(Certificate::new(&genesis, &p, vec![vote.clone()]).is_err());
    assert!(Certificate::new(&genesis, &p, vec![vote.clone(), vote.clone()]).is_err());
    let second = Attestation::sign(&genesis, &other, voter(13), &key(13)).unwrap();
    let mut mixed = vec![vote.clone(), second];
    mixed.sort_by_key(|v| v.voter());
    assert!(Certificate::new(&genesis, &p, mixed).is_err());
    let original = vote.encode_canonical().unwrap();
    for field in [2, 3, 4, 5, 6, 7, 8] {
        let Value::Array(mut envelope) = cbor::decode(&original).unwrap() else {
            panic!()
        };
        let Value::Array(ref mut body) = envelope[0] else {
            panic!()
        };
        body[field] = match field {
            4 | 5 => Value::Unsigned(9),
            _ => Value::Bytes(vec![77; 32]),
        };
        let tampered =
            Attestation::decode_canonical(&cbor::encode(&Value::Array(envelope)).unwrap()).unwrap();
        assert!(tampered.verify(&genesis, &p).is_err());
    }
    let mut bad = original;
    *bad.last_mut().unwrap() ^= 1;
    assert!(
        Attestation::decode_canonical(&bad)
            .unwrap()
            .verify(&genesis, &p)
            .is_err()
    );
    assert!(Attestation::sign(&genesis, &p, genesis.producer(), &key(11)).is_err());
}
#[test]
fn wrong_roots_parent_round_and_invalid_execution_leave_entire_chain_unchanged() {
    let genesis = genesis();
    let mut chain = Chain::new(genesis.clone()).unwrap();
    let before = chain.clone();
    let valid = proposal(&chain, 0, 20);
    for field in [4, 5, 7, 8, 9, 10] {
        let mut envelope = cbor::decode(&valid.encode_canonical().unwrap()).unwrap();
        let Value::Array(ref mut outer) = envelope else {
            panic!()
        };
        let Value::Array(ref mut body) = outer[0] else {
            panic!()
        };
        let Value::Array(ref mut header) = body[3] else {
            panic!()
        };
        let Value::Array(ref mut fields) = header[0] else {
            panic!()
        };
        fields[field] = match field {
            4 => Value::Unsigned(1),
            5 => Value::Array(vec![
                Value::Unsigned(1),
                Value::Unsigned(1),
                Value::Bytes(vec![33; 32]),
            ]),
            _ => Value::Bytes(vec![33; 32]),
        };
        let forged = resign_proposal(envelope);
        assert!(chain.validate_proposal(&forged).is_err());
        // Pair with real evidence to ensure commit, not just construction, is fail-closed.
        let block = CertifiedBlock::new(
            forged,
            certify(&genesis, valid.clone()).certificate().clone(),
        );
        assert!(chain.commit(&block).is_err());
        assert_eq!(chain, before);
    }
    let mut envelope = cbor::decode(&valid.encode_canonical().unwrap()).unwrap();
    let Value::Array(ref mut outer) = envelope else {
        panic!()
    };
    let Value::Array(ref mut body) = outer[0] else {
        panic!()
    };
    let Value::Array(ref mut block_body) = body[4] else {
        panic!()
    };
    let Value::Array(ref mut transfers) = block_body[1] else {
        panic!()
    };
    transfers.push(transfers[0].clone());
    let invalid = resign_proposal(envelope);
    assert!(chain.validate_proposal(&invalid).is_err());
    assert_eq!(chain, before);
}
#[test]
fn transaction_order_is_committed_and_replay_does_not_advance_tip() {
    let genesis = genesis();
    let mut chain = Chain::new(genesis.clone()).unwrap();
    let body = q1_protocol_types::block_body::BlockBodyV1::new(vec![
        transfer(&genesis, 0, 10),
        transfer(&genesis, 1, 15),
    ])
    .unwrap();
    let p = chain.propose(body, &key(11)).unwrap();
    let block = certify(&genesis, p.clone());
    chain.commit(&block).unwrap();
    let before = chain.clone();
    assert!(chain.commit(&block).is_err());
    assert_eq!(chain, before);
    let original = Chain::new(genesis).unwrap();
    let mut envelope = cbor::decode(&p.encode_canonical().unwrap()).unwrap();
    let Value::Array(ref mut outer) = envelope else {
        panic!()
    };
    let Value::Array(ref mut body) = outer[0] else {
        panic!()
    };
    let Value::Array(ref mut block_body) = body[4] else {
        panic!()
    };
    let Value::Array(ref mut transfers) = block_body[1] else {
        panic!()
    };
    transfers.reverse();
    let reversed = resign_proposal(envelope);
    assert!(original.validate_proposal(&reversed).is_err());
}
