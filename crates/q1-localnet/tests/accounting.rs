// TEST ONLY — NEVER USE FOR VALUE-BEARING NETWORKS. Deterministic fixture seeds are public.
//! LOCALNET v0 accounting conservation and atomic rejection tests.
use proptest::prelude::*;
use q1_localnet::{Error, LocalnetV0, ledger::Ledger};
use q1_primitives::{Address, Amount, Ed25519PrivateKey, Height, Network, ed25519};
use q1_protocol_types::{
    address::AddressEnvelope,
    block_body::BlockBodyV1,
    chain::NetworkClass,
    transfer::{SignedTransferV1, TransferBodyV1, TransferFields},
    types::{ChainId, FeeLimit, Nonce},
};

fn address(seed: u8) -> AddressEnvelope {
    AddressEnvelope::from_address(
        Address::from_public_key(
            Network::Localnet,
            ed25519::public_key(&Ed25519PrivateKey::from_seed([seed; 32])).unwrap(),
        )
        .unwrap(),
    )
}
fn ledger(balance: u128) -> Ledger {
    Ledger::from_allocations(
        LocalnetV0::new(NetworkClass::Localnet).unwrap(),
        ChainId::from_bytes([1; 32]),
        Amount::new(balance),
        [(address(7), Amount::new(balance))],
    )
    .unwrap()
}
fn fields(amount: u128, nonce: u64) -> TransferFields {
    TransferFields {
        chain_id: ChainId::from_bytes([1; 32]),
        sender_public_key: ed25519::public_key(&Ed25519PrivateKey::from_seed([7; 32])).unwrap(),
        recipient_address: address(8),
        amount: Amount::new(amount),
        fee_limit: FeeLimit::new(100),
        nonce: Nonce::new(nonce),
        valid_from_height: Height::new(1),
        valid_until_height: Height::new(10),
    }
}
fn sign(fields: TransferFields) -> SignedTransferV1 {
    SignedTransferV1::sign(
        TransferBodyV1::new(fields).unwrap(),
        &Ed25519PrivateKey::from_seed([7; 32]),
    )
    .unwrap()
}
fn body(transfers: Vec<SignedTransferV1>) -> BlockBodyV1 {
    BlockBodyV1::new(transfers).unwrap()
}

#[test]
fn actual_fee_not_limit_is_charged_and_supply_is_conserved() {
    let mut state = ledger(100);
    let receipts = state
        .apply_block(Height::new(1), &body(vec![sign(fields(20, 0))]))
        .unwrap();
    assert_eq!(state.account(address(7)).balance(), Amount::new(79));
    assert_eq!(state.account(address(8)).balance(), Amount::new(20));
    assert_eq!(state.account(address(7)).nonce(), Nonce::new(1));
    assert_eq!(state.reward_pool(), Amount::new(1));
    assert_eq!(state.total_supply(), Amount::new(100));
    assert_eq!(receipts[0].actual_fee, Amount::new(1));
    state.check_supply().unwrap();
}

#[test]
fn every_rejection_is_atomic_and_charges_nothing() {
    for case in 0..8 {
        let mut state = ledger(100);
        let before = state.clone();
        let mut invalid = fields(10, 0);
        match case {
            0 => invalid.amount = Amount::ZERO,
            1 => invalid.amount = Amount::new(100),
            2 => invalid.fee_limit = FeeLimit::ZERO,
            3 => invalid.nonce = Nonce::new(1),
            4 => invalid.valid_from_height = Height::new(2),
            5 => {
                invalid.valid_from_height = Height::new(0);
                invalid.valid_until_height = Height::new(0);
            }
            6 => invalid.chain_id = ChainId::from_bytes([2; 32]),
            7 => invalid.recipient_address = address(7),
            _ => unreachable!(),
        }
        assert!(
            state
                .apply_block(Height::new(1), &body(vec![sign(invalid)]))
                .is_err()
        );
        assert_eq!(state, before, "case {case}");
    }
    let mut state = ledger(100);
    let before = state.clone();
    let mut bad = sign(fields(10, 0)).encode_canonical().unwrap();
    *bad.last_mut().unwrap() ^= 1;
    let invalid = SignedTransferV1::decode_canonical(&bad).unwrap();
    assert!(
        state
            .apply_block(Height::new(1), &body(vec![invalid]))
            .is_err()
    );
    assert_eq!(state, before);
}

#[test]
fn a_late_block_failure_rolls_back_earlier_transfers_and_fees() {
    let mut state = ledger(100);
    let before = state.clone();
    let first = sign(fields(20, 0));
    assert_eq!(
        state.apply_block(Height::new(1), &body(vec![first.clone(), first])),
        Err(Error::NonceMismatch)
    );
    assert_eq!(state, before);
}

#[test]
fn zero_balance_does_not_erase_nonce_or_enable_replay() {
    let mut state = ledger(10);
    let transfer = sign(fields(9, 0));
    state
        .apply_block(Height::new(1), &body(vec![transfer.clone()]))
        .unwrap();
    assert_eq!(state.account(address(7)).balance(), Amount::ZERO);
    assert_eq!(state.account(address(7)).nonce(), Nonce::new(1));
    assert_eq!(state.accounts().count(), 2);
    let before = state.clone();
    assert_eq!(
        state.apply_block(Height::new(2), &body(vec![transfer])),
        Err(Error::NonceMismatch)
    );
    assert_eq!(state, before);
}

#[test]
fn repeated_execution_is_deterministic_and_block_height_is_sequential() {
    let mut a = ledger(100);
    let mut b = a.clone();
    let block = body(vec![sign(fields(20, 0)), sign(fields(30, 1))]);
    let result_a = a.apply_block(Height::new(1), &block).unwrap();
    assert_eq!(result_a, b.apply_block(Height::new(1), &block).unwrap());
    assert_eq!(a, b);
    assert_eq!(a.reward_pool(), Amount::new(2));
    assert_eq!(
        a.apply_block(Height::new(1), &block),
        Err(Error::WrongHeight)
    );
    assert_eq!(a, b);
}

#[test]
fn allocation_and_arithmetic_boundaries_cannot_create_supply() {
    let mode = LocalnetV0::new(NetworkClass::Localnet).unwrap();
    let chain = ChainId::from_bytes([1; 32]);
    assert_eq!(
        Ledger::from_allocations(mode, chain, Amount::new(2), [(address(7), Amount::new(1))]),
        Err(Error::SupplyMismatch)
    );
    assert_eq!(
        Ledger::from_allocations(
            mode,
            chain,
            Amount::new(2),
            [(address(7), Amount::new(1)), (address(7), Amount::new(1))]
        ),
        Err(Error::DuplicateAllocation)
    );
    assert!(
        Ledger::from_allocations(
            mode,
            chain,
            Amount::MAX,
            [(address(7), Amount::MAX), (address(8), Amount::new(1))]
        )
        .is_err()
    );
    let mut state = ledger(u128::MAX);
    let before = state.clone();
    assert!(
        state
            .apply_block(Height::new(1), &body(vec![sign(fields(u128::MAX, 0))]))
            .is_err()
    );
    assert_eq!(state, before);
}

proptest! {
    #[test]
    fn successful_transfer_preserves_supply_for_generated_balances(balance in 2_u128..u128::MAX, fraction in 1_u128..1000, limit in 1_u128..u128::MAX) {
        let amount = 1 + (balance - 2) / fraction;
        let mut request = fields(amount, 0);
        request.fee_limit = FeeLimit::new(limit);
        let mut state = ledger(balance);
        state.apply_block(Height::new(1), &body(vec![sign(request)])).unwrap();
        prop_assert_eq!(state.account(address(7)).balance().get(), balance - amount - 1);
        prop_assert_eq!(state.account(address(8)).balance().get(), amount);
        prop_assert_eq!(state.reward_pool(), Amount::new(1));
        prop_assert_eq!(state.total_supply().get(), balance);
        prop_assert!(state.check_supply().is_ok());
    }
}
