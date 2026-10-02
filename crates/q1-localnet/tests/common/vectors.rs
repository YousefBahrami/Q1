//! Shared public-fixture vector emission; assertions live outside this generator.
use super::common::*;
use q1_localnet::block::Chain;
/// Complete canonical objects and commitments for the local profile only.
pub fn vectors() -> Vec<(&'static str, Vec<u8>)> {
    let genesis = genesis();
    let mut chain = Chain::new(genesis.clone()).unwrap();
    let proposal = proposal(&chain, 0, 10);
    let block = certify(&genesis, proposal.clone());
    let mut rows = vec![
        ("genesis", genesis.encode_canonical().unwrap()),
        ("genesis_id", genesis.id().as_bytes().to_vec()),
        ("initial_state", chain.state().encode_canonical().unwrap()),
        (
            "initial_state_root",
            chain.state().root().unwrap().as_bytes().to_vec(),
        ),
        ("signed_proposal", proposal.encode_canonical().unwrap()),
        ("proposal_id", proposal.id().unwrap().as_bytes().to_vec()),
        (
            "signed_header",
            proposal.header().encode_canonical().unwrap(),
        ),
        (
            "block_id",
            proposal.header().id().unwrap().as_bytes().to_vec(),
        ),
        (
            "certificate",
            block.certificate().encode_canonical().unwrap(),
        ),
        (
            "certificate_hash",
            block.certificate().commitment().unwrap().to_vec(),
        ),
        ("certified_block", block.encode_canonical().unwrap()),
    ];
    chain.commit(&block).unwrap();
    rows.push((
        "state_after_transfer",
        chain.state().encode_canonical().unwrap(),
    ));
    rows.push((
        "state_root_after_transfer",
        chain.state().root().unwrap().as_bytes().to_vec(),
    ));
    rows
}
