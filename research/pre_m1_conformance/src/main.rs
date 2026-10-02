//! Non-protocol pre-M1 conformance experiment.

use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use sha2::{Digest, Sha256};

const SEED: [u8; 32] = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
];
const PAYLOAD: [u8; 5] = [0x82, 0x01, 0x42, 0x00, 0xff];
const EXPECTED_PUBLIC: &str = "03a107bff3ce10be1d70dd18e74bc09967e4d6309ba50d5f1ddc8664125531b8";
const EXPECTED_SIGNATURE: &str = concat!(
    "355e9ab16419b61d545e9d0e61402352d85ea87d0335eafbf9ec0c1c9a9bcb9e",
    "7d22645a93042a8632973d83e8fb5a83945dd53f09e0b288723513bdcb32e908"
);
const EXPECTED_ACCOUNT: &str = "8db5455cb537a01b61de9fccde4beee46959a216d116be1a9d66d4d40bb5b38f";

const VALID_CBOR: &[&str] = &[
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
const INVALID_CBOR: &[&str] = &[
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
const VALID_AMOUNT: &[(u128, &str)] = &[
    (0, "5000000000000000000000000000000000"),
    (1, "5000000000000000000000000000000001"),
    (255, "50000000000000000000000000000000ff"),
    (256, "5000000000000000000000000000000100"),
    (u32::MAX as u128, "50000000000000000000000000ffffffff"),
    (u64::MAX as u128, "500000000000000000ffffffffffffffff"),
    (u64::MAX as u128 + 1, "5000000000000000010000000000000000"),
    (1_u128 << 127, "5080000000000000000000000000000000"),
    (u128::MAX, "50ffffffffffffffffffffffffffffffff"),
];
const INVALID_AMOUNT: &[&str] = &[
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

fn domain_frame(domain: u16, payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(16 + payload.len());
    out.extend_from_slice(b"Q1DS");
    out.extend_from_slice(&1_u16.to_be_bytes());
    out.extend_from_slice(&domain.to_be_bytes());
    out.extend_from_slice(&(payload.len() as u64).to_be_bytes());
    out.extend_from_slice(payload);
    out
}

fn sha256(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}

fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(DIGITS[(byte >> 4) as usize] as char);
        out.push(DIGITS[(byte & 0x0f) as usize] as char);
    }
    out
}

fn unhex(value: &str) -> Vec<u8> {
    assert!(value.len().is_multiple_of(2));
    value
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let nibble = |byte: u8| match byte {
                b'0'..=b'9' => byte - b'0',
                b'a'..=b'f' => byte - b'a' + 10,
                _ => panic!("invalid research hex"),
            };
            (nibble(pair[0]) << 4) | nibble(pair[1])
        })
        .collect()
}

fn convert_8_to_5(bytes: &[u8]) -> Vec<u8> {
    let mut acc = 0_u32;
    let mut bits = 0_u8;
    let mut out = Vec::new();
    for byte in bytes {
        acc = (acc << 8) | u32::from(*byte);
        bits += 8;
        while bits >= 5 {
            bits -= 5;
            out.push(((acc >> bits) & 31) as u8);
        }
    }
    if bits != 0 {
        out.push(((acc << (5 - bits)) & 31) as u8);
    }
    out
}

fn polymod(values: impl IntoIterator<Item = u8>) -> u32 {
    const GENERATORS: [u32; 5] = [0x3b6a57b2, 0x26508e6d, 0x1ea119fa, 0x3d4233dd, 0x2a1462b3];
    let mut check = 1_u32;
    for value in values {
        let top = check >> 25;
        check = ((check & 0x01ff_ffff) << 5) ^ u32::from(value);
        for (index, generator) in GENERATORS.iter().enumerate() {
            if ((top >> index) & 1) != 0 {
                check ^= generator;
            }
        }
    }
    check
}

fn bech32m(hrp: &str, payload: &[u8]) -> String {
    const ALPHABET: &[u8; 32] = b"qpzry9x8gf2tvdw0s3jn54khce6mua7l";
    let data = convert_8_to_5(payload);
    let mut expanded: Vec<u8> = hrp.bytes().map(|byte| byte >> 5).collect();
    expanded.push(0);
    expanded.extend(hrp.bytes().map(|byte| byte & 31));
    expanded.extend_from_slice(&data);
    expanded.extend_from_slice(&[0; 6]);
    let checksum = polymod(expanded) ^ 0x2bc8_30a3;
    let mut combined = data;
    for index in 0..6 {
        combined.push(((checksum >> (5 * (5 - index))) & 31) as u8);
    }
    let mut out = format!("{hrp}1");
    out.extend(
        combined
            .into_iter()
            .map(|value| ALPHABET[value as usize] as char),
    );
    out
}

fn read_argument(input: &[u8], cursor: &mut usize, additional: u8) -> Result<u64, &'static str> {
    let width = match additional {
        0..=23 => return Ok(u64::from(additional)),
        24 => 1,
        25 => 2,
        26 => 4,
        27 => 8,
        _ => return Err("indefinite/reserved argument"),
    };
    let end = cursor.checked_add(width).ok_or("cursor overflow")?;
    let bytes = input.get(*cursor..end).ok_or("truncated argument")?;
    *cursor = end;
    let mut value = 0_u64;
    for byte in bytes {
        value = (value << 8) | u64::from(*byte);
    }
    let minimum = match width {
        1 => 24,
        2 => 256,
        4 => 65_536,
        8 => 4_294_967_296,
        _ => unreachable!(),
    };
    if value < minimum {
        return Err("non-shortest argument");
    }
    Ok(value)
}

fn parse_item(input: &[u8], cursor: &mut usize, depth: u8) -> Result<(), &'static str> {
    if depth > 16 {
        return Err("nesting depth");
    }
    let initial = *input.get(*cursor).ok_or("truncated item")?;
    *cursor += 1;
    let major = initial >> 5;
    let additional = initial & 31;
    match major {
        0 => {
            read_argument(input, cursor, additional)?;
        }
        2 | 3 => {
            let length = read_argument(input, cursor, additional)?;
            let limit = if major == 2 { 16_777_216 } else { 128 };
            if length > limit {
                return Err("string limit");
            }
            let end = cursor
                .checked_add(length as usize)
                .ok_or("string cursor overflow")?;
            let bytes = input.get(*cursor..end).ok_or("truncated string")?;
            *cursor = end;
            if major == 3 && !bytes.iter().all(|byte| (0x21..=0x7e).contains(byte)) {
                return Err("text policy");
            }
        }
        4 => {
            let length = read_argument(input, cursor, additional)?;
            if length > 65_535 {
                return Err("array limit");
            }
            for _ in 0..length {
                parse_item(input, cursor, depth + 1)?;
            }
        }
        7 if matches!(additional, 20..=22) => {}
        1 => return Err("negative integer prohibited"),
        5 => return Err("map prohibited"),
        6 => return Err("tag prohibited"),
        7 => return Err("simple/float prohibited"),
        _ => return Err("unknown major type"),
    }
    Ok(())
}

fn valid_profile_item(input: &[u8]) -> bool {
    if input.len() > 16_777_216 {
        return false;
    }
    let mut cursor = 0;
    parse_item(input, &mut cursor, 1).is_ok() && cursor == input.len()
}

fn encode_amount(value: u128) -> Vec<u8> {
    let mut output = Vec::with_capacity(17);
    output.push(0x50);
    output.extend_from_slice(&value.to_be_bytes());
    output
}

fn decode_amount(bytes: &[u8]) -> Option<u128> {
    if bytes.len() != 17 || bytes[0] != 0x50 {
        return None;
    }
    Some(u128::from_be_bytes(bytes[1..].try_into().ok()?))
}

fn derive() -> (Vec<u8>, VerifyingKey, Signature, [u8; 32], [u8; 36]) {
    let message = domain_frame(0x0001, &PAYLOAD);
    let signing_key = SigningKey::from_bytes(&SEED);
    let verifying_key = signing_key.verifying_key();
    let signature = signing_key.sign(&message);
    let account_id = sha256(&domain_frame(0x000f, verifying_key.as_bytes()));
    let mut address_payload = [0_u8; 36];
    address_payload[..4].copy_from_slice(&[1, 1, 0, 1]);
    address_payload[4..].copy_from_slice(&account_id);
    (
        message,
        verifying_key,
        signature,
        account_id,
        address_payload,
    )
}

fn main() {
    let (message, verifying_key, signature, account_id, address_payload) = derive();
    assert_eq!(hex(verifying_key.as_bytes()), EXPECTED_PUBLIC);
    assert_eq!(hex(&signature.to_bytes()), EXPECTED_SIGNATURE);
    assert_eq!(hex(&account_id), EXPECTED_ACCOUNT);
    for vector in VALID_CBOR {
        assert!(valid_profile_item(&unhex(vector)));
    }
    for vector in INVALID_CBOR {
        assert!(!valid_profile_item(&unhex(vector)));
    }
    for (value, vector) in VALID_AMOUNT {
        assert_eq!(hex(&encode_amount(*value)), *vector);
        assert_eq!(decode_amount(&unhex(vector)), Some(*value));
    }
    for vector in INVALID_AMOUNT {
        assert_eq!(decode_amount(&unhex(vector)), None);
    }
    println!("rust_impl=ed25519-dalek 3.0.0");
    println!("message={}", hex(&message));
    println!("public_key={}", hex(verifying_key.as_bytes()));
    println!("signature={}", hex(&signature.to_bytes()));
    println!("account_id={}", hex(&account_id));
    for hrp in ["q1l", "q1p", "q1r"] {
        println!("{hrp}={}", bech32m(hrp, &address_payload));
    }
    println!("cbor_valid={}", VALID_CBOR.len());
    println!("cbor_rejected={}", INVALID_CBOR.len());
    println!("amount_valid={}", VALID_AMOUNT.len());
    println!("amount_rejected={}", INVALID_AMOUNT.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_ed25519_vector_and_mutations() {
        let (message, verifying_key, signature, _, _) = derive();
        assert_eq!(hex(verifying_key.as_bytes()), EXPECTED_PUBLIC);
        assert_eq!(hex(&signature.to_bytes()), EXPECTED_SIGNATURE);
        assert!(verifying_key.verify_strict(&message, &signature).is_ok());

        let mut mutated_message = message.clone();
        let last = mutated_message.len() - 1;
        mutated_message[last] ^= 1;
        assert!(
            verifying_key
                .verify_strict(&mutated_message, &signature)
                .is_err()
        );

        let mut mutated_signature = signature.to_bytes();
        mutated_signature[0] ^= 1;
        let mutated_signature = Signature::from_bytes(&mutated_signature);
        assert!(
            verifying_key
                .verify_strict(&message, &mutated_signature)
                .is_err()
        );
    }

    #[test]
    fn domain_and_address_vectors() {
        let (_, _, _, account_id, address_payload) = derive();
        assert_eq!(hex(&account_id), EXPECTED_ACCOUNT);
        assert_eq!(
            bech32m("q1l", &address_payload),
            "q1l1qyqsqqvdk4z4edfh5qdkrh5len0yhmhyd9v6y9k3z6lp48tx6n2qhddn3ujwz45w"
        );
        assert_eq!(
            bech32m("q1p", &address_payload),
            "q1p1qyqsqqvdk4z4edfh5qdkrh5len0yhmhyd9v6y9k3z6lp48tx6n2qhddn3u6q78rg"
        );
        assert_eq!(
            bech32m("q1r", &address_payload),
            "q1r1qyqsqqvdk4z4edfh5qdkrh5len0yhmhyd9v6y9k3z6lp48tx6n2qhddn3u4puyfx"
        );
    }

    #[test]
    fn cbor_profile_corpus() {
        for vector in VALID_CBOR {
            assert!(valid_profile_item(&unhex(vector)), "valid {vector}");
        }
        for vector in INVALID_CBOR {
            assert!(!valid_profile_item(&unhex(vector)), "invalid {vector}");
        }
    }

    #[test]
    fn amount_profile_corpus() {
        for (value, vector) in VALID_AMOUNT {
            assert_eq!(hex(&encode_amount(*value)), *vector);
            assert_eq!(decode_amount(&unhex(vector)), Some(*value));
        }
        for vector in INVALID_AMOUNT {
            assert_eq!(decode_amount(&unhex(vector)), None);
        }
    }
}
