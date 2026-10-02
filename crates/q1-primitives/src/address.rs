//! Strict Q1-specific Bech32m address envelopes.

use core::fmt;

use crate::{
    Error, Result,
    domain::Domain,
    key::{ED25519_ALGORITHM_ID, Ed25519PublicKey},
    sha256,
};

const BECH32M_CONSTANT: u32 = 0x2bc8_30a3;
const ALPHABET: &[u8; 32] = b"qpzry9x8gf2tvdw0s3jn54khce6mua7l";
const ENVELOPE_VERSION: u8 = 0x01;
const BINARY_LENGTH: usize = 36;
const MAX_TEXT_LENGTH: usize = 90;

/// A currently enabled Q1 experimental network.
///
/// The future `q1` public HRP is intentionally absent, so parsing it cannot
/// imply mainnet or public-network authorization.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Network {
    /// Local engineering network, HRP `q1l`.
    Localnet,
    /// Controlled private testnet, HRP `q1p`.
    PrivateTestnet,
    /// Isolated research network, HRP `q1r`.
    Research,
}

/// Semantics of the account identifier carried by an address.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum AddressType {
    /// A single-account identifier derived from one Ed25519 public key.
    SingleEd25519 = 0x01,
}

/// A validated Q1 address envelope.
///
/// The account identifier is the full domain-separated SHA-256 digest of the
/// public key. Keeping network, envelope version, type, and algorithm explicit
/// prevents inference from payload length or presentation context.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Address {
    network: Network,
    address_type: AddressType,
    algorithm_id: u16,
    account_id: [u8; 32],
}

impl Network {
    /// Returns the only canonical lowercase HRP for this network.
    #[must_use]
    pub const fn hrp(self) -> &'static str {
        match self {
            Self::Localnet => "q1l",
            Self::PrivateTestnet => "q1p",
            Self::Research => "q1r",
        }
    }

    fn from_hrp(hrp: &str) -> Result<Self> {
        match hrp {
            "q1l" => Ok(Self::Localnet),
            "q1p" => Ok(Self::PrivateTestnet),
            "q1r" => Ok(Self::Research),
            _ => Err(Error::AddressTextInvalid),
        }
    }
}

impl Address {
    /// Derives a V1 single-key account address from a validated public key.
    pub fn from_public_key(network: Network, public_key: Ed25519PublicKey) -> Result<Self> {
        let account_id =
            sha256::hash_domain(Domain::AddressPayload, public_key.as_bytes())?.into_bytes();
        Ok(Self {
            network,
            address_type: AddressType::SingleEd25519,
            algorithm_id: ED25519_ALGORITHM_ID,
            account_id,
        })
    }

    /// Parses an address and rejects a valid address for another network.
    pub fn parse_for_network(expected: Network, text: &str) -> Result<Self> {
        validate_text(text)?;
        let separator = text.rfind('1').ok_or(Error::AddressTextInvalid)?;
        if separator == 0 || separator + 7 > text.len() {
            return Err(Error::AddressTextInvalid);
        }
        let hrp = &text[..separator];
        let actual_network = Network::from_hrp(hrp)?;
        if actual_network != expected {
            return Err(Error::AddressWrongNetwork);
        }
        let data = decode_alphabet(&text.as_bytes()[separator + 1..])?;
        if polymod(hrp_expand(hrp).into_iter().chain(data.iter().copied())) != BECH32M_CONSTANT {
            return Err(Error::AddressChecksumInvalid);
        }
        let payload = convert_5_to_8(&data[..data.len() - 6])?;
        Self::from_payload(actual_network, &payload)
    }

    /// Validates a binary envelope with an explicitly supplied display network.
    ///
    /// The 36 consensus bytes contain no network class. Chain binding must be
    /// checked separately by the containing signed protocol object.
    pub fn from_payload(network: Network, payload: &[u8]) -> Result<Self> {
        if payload.len() != BINARY_LENGTH {
            return Err(Error::InvalidLength {
                expected: BINARY_LENGTH,
                actual: payload.len(),
            });
        }
        if payload[0] != ENVELOPE_VERSION {
            return Err(Error::AddressVersionUnsupported(payload[0]));
        }
        let address_type = match payload[1] {
            0x01 => AddressType::SingleEd25519,
            value => return Err(Error::AddressTypeUnsupported(value)),
        };
        let algorithm_id = u16::from_be_bytes([payload[2], payload[3]]);
        if algorithm_id != ED25519_ALGORITHM_ID {
            return Err(Error::AlgorithmUnsupported(algorithm_id));
        }
        let mut account_id = [0_u8; 32];
        account_id.copy_from_slice(&payload[4..]);
        Ok(Self {
            network,
            address_type,
            algorithm_id,
            account_id,
        })
    }

    /// Returns the network encoded by the HRP.
    #[must_use]
    pub const fn network(self) -> Network {
        self.network
    }

    /// Returns the versioned account type.
    #[must_use]
    pub const fn address_type(self) -> AddressType {
        self.address_type
    }

    /// Returns the explicit cryptographic algorithm identifier.
    #[must_use]
    pub const fn algorithm_id(self) -> u16 {
        self.algorithm_id
    }

    /// Returns the immutable full 32-byte account identifier.
    #[must_use]
    pub const fn account_id(&self) -> &[u8; 32] {
        &self.account_id
    }

    /// Returns the exact 36-byte V1 binary envelope.
    #[must_use]
    pub fn to_payload(self) -> [u8; BINARY_LENGTH] {
        let mut output = [0_u8; BINARY_LENGTH];
        output[0] = ENVELOPE_VERSION;
        output[1] = self.address_type as u8;
        output[2..4].copy_from_slice(&self.algorithm_id.to_be_bytes());
        output[4..].copy_from_slice(&self.account_id);
        output
    }

    /// Encodes the canonical lowercase Bech32m text.
    #[must_use]
    pub fn encode(self) -> String {
        encode_bech32m(self.network.hrp(), &self.to_payload())
    }
}

impl fmt::Display for Address {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.encode())
    }
}

fn validate_text(text: &str) -> Result<()> {
    if text.is_empty()
        || text.len() > MAX_TEXT_LENGTH
        || text.bytes().any(|byte| !(0x21..=0x7e).contains(&byte))
        || text.bytes().any(|byte| byte.is_ascii_uppercase())
    {
        return Err(Error::AddressTextInvalid);
    }
    Ok(())
}

fn encode_bech32m(hrp: &str, payload: &[u8]) -> String {
    let data = convert_8_to_5(payload);
    let mut checksum_input = hrp_expand(hrp);
    checksum_input.extend_from_slice(&data);
    checksum_input.extend_from_slice(&[0; 6]);
    let residue = polymod(checksum_input) ^ BECH32M_CONSTANT;
    let mut output = String::with_capacity(hrp.len() + 1 + data.len() + 6);
    output.push_str(hrp);
    output.push('1');
    for value in data {
        output.push(ALPHABET[value as usize] as char);
    }
    for index in 0..6 {
        let value = ((residue >> (5 * (5 - index))) & 31) as usize;
        output.push(ALPHABET[value] as char);
    }
    output
}

fn decode_alphabet(data: &[u8]) -> Result<Vec<u8>> {
    if data.len() < 6 {
        return Err(Error::AddressTextInvalid);
    }
    data.iter()
        .map(|character| {
            ALPHABET
                .iter()
                .position(|candidate| candidate == character)
                .and_then(|value| u8::try_from(value).ok())
                .ok_or(Error::AddressTextInvalid)
        })
        .collect()
}

fn hrp_expand(hrp: &str) -> Vec<u8> {
    let mut output = Vec::with_capacity(hrp.len() * 2 + 1);
    output.extend(hrp.bytes().map(|byte| byte >> 5));
    output.push(0);
    output.extend(hrp.bytes().map(|byte| byte & 31));
    output
}

fn polymod(values: impl IntoIterator<Item = u8>) -> u32 {
    const GENERATORS: [u32; 5] = [
        0x3b6a_57b2,
        0x2650_8e6d,
        0x1ea1_19fa,
        0x3d42_33dd,
        0x2a14_62b3,
    ];
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

fn convert_8_to_5(bytes: &[u8]) -> Vec<u8> {
    let mut accumulator = 0_u32;
    let mut bits = 0_u8;
    let mut output = Vec::new();
    for byte in bytes {
        accumulator = (accumulator << 8) | u32::from(*byte);
        bits += 8;
        while bits >= 5 {
            bits -= 5;
            output.push(((accumulator >> bits) & 31) as u8);
        }
    }
    if bits != 0 {
        output.push(((accumulator << (5 - bits)) & 31) as u8);
    }
    output
}

fn convert_5_to_8(values: &[u8]) -> Result<Vec<u8>> {
    let mut accumulator = 0_u32;
    let mut bits = 0_u8;
    let mut output = Vec::new();
    for value in values {
        if *value > 31 {
            return Err(Error::AddressPaddingInvalid);
        }
        accumulator = (accumulator << 5) | u32::from(*value);
        bits += 5;
        while bits >= 8 {
            bits -= 8;
            output.push(((accumulator >> bits) & 0xff) as u8);
        }
    }
    if bits > 4 || (bits != 0 && ((accumulator << (8 - bits)) & 0xff) != 0) {
        return Err(Error::AddressPaddingInvalid);
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encode_with_checksum_constant(hrp: &str, payload: &[u8], constant: u32) -> String {
        let data = convert_8_to_5(payload);
        let mut checksum_input = hrp_expand(hrp);
        checksum_input.extend_from_slice(&data);
        checksum_input.extend_from_slice(&[0; 6]);
        let residue = polymod(checksum_input) ^ constant;
        let mut output = format!("{hrp}1");
        for value in data {
            output.push(ALPHABET[value as usize] as char);
        }
        for index in 0..6 {
            output.push(ALPHABET[((residue >> (5 * (5 - index))) & 31) as usize] as char);
        }
        output
    }

    #[test]
    fn reserved_public_hrp_is_not_a_network() {
        assert_eq!(Network::from_hrp("q1"), Err(Error::AddressTextInvalid));

        let payload = [0_u8; BINARY_LENGTH];
        let reserved = encode_bech32m("q1", &payload);
        assert_eq!(
            Address::parse_for_network(Network::Localnet, &reserved),
            Err(Error::AddressTextInvalid)
        );
    }

    #[test]
    fn envelope_fields_and_lengths_are_checked_after_valid_checksum() {
        let mut payload = [0_u8; BINARY_LENGTH];
        payload[0] = ENVELOPE_VERSION;
        payload[1] = AddressType::SingleEd25519 as u8;
        payload[2..4].copy_from_slice(&ED25519_ALGORITHM_ID.to_be_bytes());

        let mut unsupported_version = payload;
        unsupported_version[0] = 2;
        assert_eq!(
            Address::parse_for_network(
                Network::Localnet,
                &encode_bech32m("q1l", &unsupported_version)
            ),
            Err(Error::AddressVersionUnsupported(2))
        );

        let mut unsupported_type = payload;
        unsupported_type[1] = 2;
        assert_eq!(
            Address::parse_for_network(
                Network::Localnet,
                &encode_bech32m("q1l", &unsupported_type)
            ),
            Err(Error::AddressTypeUnsupported(2))
        );

        let mut unsupported_algorithm = payload;
        unsupported_algorithm[2..4].copy_from_slice(&2_u16.to_be_bytes());
        assert_eq!(
            Address::parse_for_network(
                Network::Localnet,
                &encode_bech32m("q1l", &unsupported_algorithm)
            ),
            Err(Error::AlgorithmUnsupported(2))
        );

        for invalid_length in [35, 37] {
            let text = encode_bech32m("q1l", &vec![0_u8; invalid_length]);
            assert_eq!(
                Address::parse_for_network(Network::Localnet, &text),
                Err(Error::InvalidLength {
                    expected: BINARY_LENGTH,
                    actual: invalid_length,
                })
            );
        }
    }

    #[test]
    fn bech32_checksum_is_not_accepted_as_bech32m() {
        let mut payload = [0_u8; BINARY_LENGTH];
        payload[0] = ENVELOPE_VERSION;
        payload[1] = AddressType::SingleEd25519 as u8;
        payload[2..4].copy_from_slice(&ED25519_ALGORITHM_ID.to_be_bytes());
        let bech32 = encode_with_checksum_constant("q1l", &payload, 1);
        assert_eq!(
            Address::parse_for_network(Network::Localnet, &bech32),
            Err(Error::AddressChecksumInvalid)
        );
    }
}
