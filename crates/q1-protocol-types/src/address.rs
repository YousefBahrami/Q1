//! Binary recipient addresses; network presentation is deliberately separate.

use crate::{Result, codec};
use q1_primitives::{Address, Network, cbor};

/// A validated 36-byte V1 address envelope without an HRP or network class.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AddressEnvelope([u8; 36]);

impl AddressEnvelope {
    /// Checks version, type, algorithm and exact binary width.
    pub fn from_bytes(bytes: [u8; 36]) -> Result<Self> {
        // Network is an external display context, never part of these bytes.
        Address::from_payload(Network::Localnet, &bytes)?;
        Ok(Self(bytes))
    }

    /// Extracts the validated binary envelope from an address.
    #[must_use]
    pub fn from_address(address: Address) -> Self {
        Self(address.to_payload())
    }

    /// Returns the canonical binary bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 36] {
        &self.0
    }

    /// Attaches an explicit display network; it does not establish chain validity.
    pub fn to_address(self, network: Network) -> Result<Address> {
        Ok(Address::from_payload(network, &self.0)?)
    }

    /// Encodes the exact 36-byte CBOR byte string.
    pub fn encode_canonical(&self) -> Result<Vec<u8>> {
        Ok(cbor::encode_bytes(&self.0)?)
    }

    /// Decodes one canonical byte string and validates the binary envelope.
    pub fn decode_canonical(bytes: &[u8]) -> Result<Self> {
        Self::from_bytes(codec::bytes(&cbor::decode(bytes)?)?)
    }
}
