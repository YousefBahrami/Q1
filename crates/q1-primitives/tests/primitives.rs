// TEST ONLY — NEVER USE FOR VALUE-BEARING NETWORKS. Deterministic fixture seeds are public.
//! Published vectors, malformed inputs, and boundary tests for M1.1.

use core::str::FromStr;

use q1_primitives::{
    Address, Amount, BlockHash, Ed25519PrivateKey, Ed25519PublicKey, Ed25519Signature, Error,
    Height, Network, RoundNumber, TransactionHash,
    cbor::{self, Value},
    domain::{self, Domain},
    ed25519, sha256,
    traits::{CanonicalDecode, CanonicalEncode},
};

fn decode_hex(value: &str) -> Vec<u8> {
    value
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let nibble = |value: u8| match value {
                b'0'..=b'9' => value - b'0',
                b'a'..=b'f' => value - b'a' + 10,
                _ => panic!("test vector contains invalid hex"),
            };
            (nibble(pair[0]) << 4) | nibble(pair[1])
        })
        .collect()
}

#[test]
fn amount_arithmetic_boundaries() {
    assert_eq!(
        Amount::new(2).checked_add(Amount::new(3)),
        Ok(Amount::new(5))
    );
    assert_eq!(
        Amount::MAX.checked_add(Amount::new(1)),
        Err(Error::Overflow)
    );
    assert_eq!(
        Amount::ZERO.checked_sub(Amount::new(1)),
        Err(Error::Underflow)
    );
    assert_eq!(Amount::new(7).checked_mul(6), Ok(Amount::new(42)));
    assert_eq!(Amount::MAX.checked_mul(2), Err(Error::Overflow));
    assert_eq!(Amount::new(42).checked_div(6), Ok(Amount::new(7)));
    assert_eq!(Amount::new(42).checked_div(0), Err(Error::DivisionByZero));
    assert_eq!(
        u64::try_from(Amount::new(u128::from(u64::MAX))),
        Ok(u64::MAX)
    );
    assert_eq!(
        u64::try_from(Amount::new(u128::from(u64::MAX) + 1)),
        Err(Error::ConversionOutOfRange)
    );
}

#[test]
fn amount_normative_vectors() {
    let vectors = [
        (0_u128, "5000000000000000000000000000000000"),
        (1, "5000000000000000000000000000000001"),
        (255, "50000000000000000000000000000000ff"),
        (256, "5000000000000000000000000000000100"),
        (u128::from(u32::MAX), "50000000000000000000000000ffffffff"),
        (u128::from(u64::MAX), "500000000000000000ffffffffffffffff"),
        (
            u128::from(u64::MAX) + 1,
            "5000000000000000010000000000000000",
        ),
        (1_u128 << 127, "5080000000000000000000000000000000"),
        (u128::MAX, "50ffffffffffffffffffffffffffffffff"),
    ];
    for (value, expected_hex) in vectors {
        let amount = Amount::new(value);
        assert_eq!(amount.encode_canonical().unwrap(), decode_hex(expected_hex));
        assert_eq!(
            Amount::decode_canonical(&decode_hex(expected_hex)).unwrap(),
            amount
        );
    }
}

#[test]
fn amount_rejects_every_alternate_representation() {
    let invalid = [
        "40",
        "4100",
        "480000000000000000",
        "4f000000000000000000000000000000",
        "510000000000000000000000000000000000",
        "00",
        "1bffffffffffffffff",
        "5f5000000000000000000000000000000000ff",
        "d84050000000000000000000000000000000",
        "20",
        "f90000",
    ];
    for vector in invalid {
        assert!(
            Amount::decode_canonical(&decode_hex(vector)).is_err(),
            "{vector}"
        );
    }
}

#[test]
fn height_is_typed_and_canonical() {
    assert_eq!(Height::new(41).checked_increment(), Ok(Height::new(42)));
    assert_eq!(Height::MAX.checked_increment(), Err(Error::Overflow));
    let encoded = Height::new(256).encode_canonical().unwrap();
    assert_eq!(encoded, decode_hex("190100"));
    assert_eq!(Height::decode_canonical(&encoded), Ok(Height::new(256)));
}

#[test]
fn round_number_values_order_and_encode_canonically() {
    assert_eq!(RoundNumber::ZERO, RoundNumber::new(0));
    assert_eq!(RoundNumber::default(), RoundNumber::ZERO);
    assert_eq!(RoundNumber::new(1).get(), 1);
    assert_eq!(RoundNumber::new(u32::MAX).get(), u32::MAX);
    assert_eq!(RoundNumber::new(7), RoundNumber::new(7));
    assert_ne!(RoundNumber::new(7), RoundNumber::new(8));
    assert!(RoundNumber::ZERO < RoundNumber::new(1));
    assert!(RoundNumber::new(u32::MAX) > RoundNumber::new(1));

    let vectors = [
        (0, "00"),
        (1, "01"),
        (23, "17"),
        (24, "1818"),
        (255, "18ff"),
        (256, "190100"),
        (u32::MAX, "1affffffff"),
    ];
    for (value, expected_hex) in vectors {
        let round_number = RoundNumber::new(value);
        let bytes = decode_hex(expected_hex);
        assert_eq!(round_number.encode_canonical().unwrap(), bytes);
        assert_eq!(RoundNumber::decode_canonical(&bytes).unwrap(), round_number);
    }
}

#[test]
fn round_number_rejects_out_of_domain_and_non_canonical_inputs() {
    let invalid = [
        "1b0000000100000000",
        "20",
        "f90000",
        "8100",
        "a0",
        "c000",
        "1800",
        "1a0000",
        "0000",
    ];
    for vector in invalid {
        assert!(
            RoundNumber::decode_canonical(&decode_hex(vector)).is_err(),
            "{vector}"
        );
    }
}

#[test]
#[allow(deprecated)]
fn legacy_slot_and_round_keep_their_historical_canonical_vector() {
    use q1_primitives::{Round, Slot};

    let slot = Slot::new(9);
    let round = Round::new(slot, 3);
    let round_bytes = round.encode_canonical().unwrap();
    assert_eq!(round_bytes, decode_hex("83010903"));
    assert_eq!(Round::decode_canonical(&round_bytes), Ok(round));
    assert!(Round::decode_canonical(&decode_hex("8401090300")).is_err());
}

#[test]
fn hash_wrappers_do_not_interchange_implicitly() {
    let bytes = [0xab; 32];
    let block = BlockHash::from_bytes(bytes);
    let transaction = TransactionHash::from_bytes(bytes);
    assert_eq!(
        block.to_string(),
        "abababababababababababababababababababababababababababababababab"
    );
    assert_eq!(BlockHash::from_str(&block.to_string()), Ok(block));
    assert_eq!(transaction.into_bytes(), bytes);
    assert!(BlockHash::from_str("ABAB").is_err());
    assert!(BlockHash::from_str(&"A".repeat(64)).is_err());
}

#[test]
fn restricted_cbor_accepts_published_vectors() {
    let valid = [
        "00",
        "17",
        "1818",
        "190100",
        "40",
        "4200ff",
        "60",
        "627131",
        "82014200ff",
        "8301f4f6",
    ];
    for vector in valid {
        let bytes = decode_hex(vector);
        let value = cbor::decode(&bytes).unwrap();
        assert_eq!(cbor::encode(&value).unwrap(), bytes);
    }
}

#[test]
fn restricted_cbor_rejects_published_vectors() {
    let invalid = [
        "1801",
        "1817",
        "190018",
        "5f4100ff",
        "7f6171ff",
        "9f01ff",
        "a0",
        "a201000101",
        "c001",
        "f90000",
        "8201",
        "0102",
        "63713100",
        "ff",
    ];
    for vector in invalid {
        assert!(cbor::decode(&decode_hex(vector)).is_err(), "{vector}");
    }
    assert_eq!(
        cbor::encode(&Value::Text("has space".to_owned())),
        Err(Error::CborInvalidText)
    );
}

#[test]
fn domain_frame_and_sha_vectors_match() {
    let payload = decode_hex("82014200ff");
    assert_eq!(
        domain::frame(Domain::TransactionSigning, &payload).unwrap(),
        decode_hex("5131445300010001000000000000000582014200ff")
    );
    assert_eq!(
        sha256::hash_domain(Domain::TransactionId, &[])
            .unwrap()
            .to_string(),
        "0b368d1bc8790bd7aa6bbf27d608e25da78de7758440a70935433b08cdd6e7cf"
    );
    assert!(Domain::try_from(0).is_err());
    assert!(Domain::try_from(0xfffe).is_err());
}

#[test]
fn ed25519_two_implementation_project_vector_matches() {
    let seed: [u8; 32] =
        decode_hex("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f")
            .try_into()
            .unwrap();
    let private = Ed25519PrivateKey::from_seed(seed);
    let public = ed25519::public_key(&private).unwrap();
    assert_eq!(
        public.as_bytes(),
        &decode_hex("03a107bff3ce10be1d70dd18e74bc09967e4d6309ba50d5f1ddc8664125531b8").as_slice()
    );
    let payload = decode_hex("82014200ff");
    let signature = ed25519::sign_domain(&private, Domain::TransactionSigning, &payload).unwrap();
    assert_eq!(
        signature.as_bytes(),
        &decode_hex(concat!(
            "355e9ab16419b61d545e9d0e61402352d85ea87d0335eafbf9ec0c1c9a9bcb9e",
            "7d22645a93042a8632973d83e8fb5a83945dd53f09e0b288723513bdcb32e908"
        ))
        .as_slice()
    );
    assert!(
        ed25519::verify_domain_strict(public, Domain::TransactionSigning, &payload, signature)
            .is_ok()
    );

    let mut mutated_payload = payload.clone();
    mutated_payload[0] ^= 1;
    assert!(
        ed25519::verify_domain_strict(
            public,
            Domain::TransactionSigning,
            &mutated_payload,
            signature
        )
        .is_err()
    );
    let mut mutated_signature = *signature.as_bytes();
    mutated_signature[0] ^= 1;
    assert!(
        ed25519::verify_domain_strict(
            public,
            Domain::TransactionSigning,
            &payload,
            Ed25519Signature::from_bytes(mutated_signature)
        )
        .is_err()
    );
    assert!(
        ed25519::verify_domain_strict(public, Domain::TransactionId, &payload, signature).is_err()
    );

    let mut mutated_public_key = *public.as_bytes();
    mutated_public_key[0] ^= 1;
    if let Ok(mutated_public_key) = Ed25519PublicKey::from_bytes(mutated_public_key) {
        assert!(
            ed25519::verify_domain_strict(
                mutated_public_key,
                Domain::TransactionSigning,
                &payload,
                signature
            )
            .is_err()
        );
    }
}

#[test]
fn ed25519_rejects_malformed_lengths_weak_keys_and_scalars() {
    for length in [0, 31, 33, 64] {
        assert!(matches!(
            Ed25519PublicKey::try_from_slice(&vec![0; length]),
            Err(Error::InvalidLength { .. })
        ));
    }
    for invalid_point in [
        [0; 32],
        {
            let mut identity = [0; 32];
            identity[0] = 1;
            identity
        },
        [0xff; 32],
    ] {
        assert!(Ed25519PublicKey::from_bytes(invalid_point).is_err());
    }
    for length in [0, 63, 65] {
        assert!(matches!(
            Ed25519Signature::try_from_slice(&vec![0; length]),
            Err(Error::InvalidLength { .. })
        ));
    }

    let private = Ed25519PrivateKey::from_seed([7; 32]);
    let public = ed25519::public_key(&private).unwrap();
    let malformed = Ed25519Signature::from_bytes([0xff; 64]);
    assert!(ed25519::verify_strict(public, b"message", malformed).is_err());
}

#[test]
fn address_vectors_and_failures_match_profile() {
    let public = Ed25519PublicKey::from_bytes(
        decode_hex("03a107bff3ce10be1d70dd18e74bc09967e4d6309ba50d5f1ddc8664125531b8")
            .try_into()
            .unwrap(),
    )
    .unwrap();
    let expected = [
        (
            Network::Localnet,
            "q1l1qyqsqqvdk4z4edfh5qdkrh5len0yhmhyd9v6y9k3z6lp48tx6n2qhddn3ujwz45w",
        ),
        (
            Network::PrivateTestnet,
            "q1p1qyqsqqvdk4z4edfh5qdkrh5len0yhmhyd9v6y9k3z6lp48tx6n2qhddn3u6q78rg",
        ),
        (
            Network::Research,
            "q1r1qyqsqqvdk4z4edfh5qdkrh5len0yhmhyd9v6y9k3z6lp48tx6n2qhddn3u4puyfx",
        ),
    ];
    for (network, text) in expected {
        let address = Address::from_public_key(network, public).unwrap();
        assert_eq!(address.to_string(), text);
        assert_eq!(Address::parse_for_network(network, text), Ok(address));
    }

    let local = expected[0].1;
    assert_eq!(
        Address::parse_for_network(Network::PrivateTestnet, local),
        Err(Error::AddressWrongNetwork)
    );
    let invalid = [
        "q1l1qyqsqqvdk4z4edfh5qdkrh5len0yhmhyd9v6y9k3z6lp48tx6n2qhddn3ujwz45q",
        "Q1l1qyqsqqvdk4z4edfh5qdkrh5len0yhmhyd9v6y9k3z6lp48tx6n2qhddn3ujwz45w",
        "q1l1qyqsqqvdk4z4edfh5qdkrh5len0yhmhyd9v6y9k3z6lp48tx6n2qhddn3upmp2c55",
        "q1l1qgqsqqvdk4z4edfh5qdkrh5len0yhmhyd9v6y9k3z6lp48tx6n2qhddn3u04nv77",
        "q1l1qyqsqq5dk4z4edfh5qdkrh5len0yhmhyd9v6y9k3z6lp48tx6n2qhddn3uu78978",
        "q1l1qyqsqqvdk4z4edfh5qdkrh5len0yhmhyd9v6y9k3z6lp48tx6n2qhddn0jr2vf",
    ];
    for text in invalid {
        assert!(
            Address::parse_for_network(Network::Localnet, text).is_err(),
            "{text}"
        );
    }
    assert!(Address::parse_for_network(Network::Localnet, &format!(" {local}")).is_err());
    assert!(Address::parse_for_network(Network::Localnet, "q11qqqqqq").is_err());
}
