// TEST ONLY — NEVER USE FOR VALUE-BEARING NETWORKS. Deterministic fixture seeds are public.
//! Property tests for deterministic primitive round trips.

use proptest::prelude::*;
use q1_primitives::{
    Address, Amount, Ed25519PrivateKey, Height, Network, RoundNumber,
    cbor::{self, Value},
    ed25519,
    traits::{CanonicalDecode, CanonicalEncode},
};

proptest! {
    #[test]
    fn amount_round_trips_every_u128(value in any::<u128>()) {
        let amount = Amount::new(value);
        let encoded = amount.encode_canonical().unwrap();
        prop_assert_eq!(encoded.len(), 17);
        prop_assert_eq!(encoded[0], 0x50);
        prop_assert_eq!(Amount::decode_canonical(&encoded).unwrap(), amount);
    }

    #[test]
    fn amount_checked_add_matches_u128(left in any::<u128>(), right in any::<u128>()) {
        let actual = Amount::new(left).checked_add(Amount::new(right));
        match left.checked_add(right) {
            Some(expected) => prop_assert_eq!(actual.unwrap(), Amount::new(expected)),
            None => prop_assert!(actual.is_err()),
        }
    }

    #[test]
    fn height_round_trips(value in any::<u64>()) {
        let height = Height::new(value);
        let encoded = height.encode_canonical().unwrap();
        prop_assert_eq!(Height::decode_canonical(&encoded).unwrap(), height);
    }

    #[test]
    fn round_number_round_trips_every_u32(value in any::<u32>()) {
        let round_number = RoundNumber::new(value);
        let encoded = round_number.encode_canonical().unwrap();
        prop_assert_eq!(
            RoundNumber::decode_canonical(&encoded).unwrap(),
            round_number
        );
    }

    #[test]
    fn cbor_unsigned_round_trips(value in any::<u64>()) {
        let value = Value::Unsigned(value);
        let encoded = cbor::encode(&value).unwrap();
        prop_assert_eq!(cbor::decode(&encoded).unwrap(), value);
    }

    #[test]
    fn cbor_decoder_never_panics_on_arbitrary_input(bytes in proptest::collection::vec(any::<u8>(), 0..4096)) {
        let _ = cbor::decode(&bytes);
    }

    #[test]
    fn address_round_trips_for_test_seeds(seed in any::<[u8; 32]>(), selector in 0_u8..3) {
        let private = Ed25519PrivateKey::from_seed(seed);
        let public = ed25519::public_key(&private).unwrap();
        let network = match selector {
            0 => Network::Localnet,
            1 => Network::PrivateTestnet,
            _ => Network::Research,
        };
        let address = Address::from_public_key(network, public).unwrap();
        let text = address.to_string();
        prop_assert_eq!(Address::parse_for_network(network, &text).unwrap(), address);
    }
}
