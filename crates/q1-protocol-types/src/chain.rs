//! Approved chain identity preimage and active network-class assignments.

use crate::{Error, Result, codec, types::ChainId};
use q1_primitives::{
    Network,
    cbor::{self, Value},
    domain::Domain,
    sha256,
};

/// An active network class; reserved future-public values have no constructor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u16)]
pub enum NetworkClass {
    /// Private local development network.
    Localnet = 1,
    /// Controlled private test network.
    PrivateTestnet = 2,
    /// Research network.
    Research = 3,
}
impl NetworkClass {
    /// Returns the stable numeric registry value.
    #[must_use]
    pub const fn id(self) -> u16 {
        self as u16
    }
    /// Returns the matching address network under the approved class/HRP rule.
    #[must_use]
    pub const fn address_network(self) -> Network {
        match self {
            Self::Localnet => Network::Localnet,
            Self::PrivateTestnet => Network::PrivateTestnet,
            Self::Research => Network::Research,
        }
    }
}
impl TryFrom<u16> for NetworkClass {
    type Error = Error;
    fn try_from(value: u16) -> Result<Self> {
        match value {
            1 => Ok(Self::Localnet),
            2 => Ok(Self::PrivateTestnet),
            3 => Ok(Self::Research),
            _ => Err(Error::InvalidField("inactive or unknown network class")),
        }
    }
}

/// Exact `[1, network_class, creation_nonce:bytes32]` chain-identity preimage.
///
/// The caller must supply a public nonce generated once by an approved CSPRNG
/// and prevent reuse between network instances. A codec cannot prove entropy
/// or global uniqueness; this type does not create or authorize a network.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChainIdentityPreimageV1 {
    network_class: NetworkClass,
    creation_nonce: [u8; 32],
}
impl ChainIdentityPreimageV1 {
    /// Constructs a preimage from an active class and externally generated nonce.
    #[must_use]
    pub const fn new(network_class: NetworkClass, creation_nonce: [u8; 32]) -> Self {
        Self {
            network_class,
            creation_nonce,
        }
    }
    /// Returns the active network class.
    #[must_use]
    pub const fn network_class(&self) -> NetworkClass {
        self.network_class
    }
    /// Returns the public creation nonce.
    #[must_use]
    pub const fn creation_nonce(&self) -> &[u8; 32] {
        &self.creation_nonce
    }
    /// Derives the chain ID using registered CHAIN_ID domain 0x0010.
    pub fn chain_id(&self) -> Result<ChainId> {
        Ok(ChainId::from_bytes(
            sha256::hash_domain(Domain::ChainId, &self.encode_canonical()?)?.into_bytes(),
        ))
    }
    /// Encodes the fixed three-field canonical array.
    pub fn encode_canonical(&self) -> Result<Vec<u8>> {
        Ok(cbor::encode(&Value::Array(vec![
            Value::Unsigned(1),
            Value::Unsigned(u64::from(self.network_class.id())),
            Value::Bytes(self.creation_nonce.to_vec()),
        ]))?)
    }
    /// Rejects wrong field counts, versions, network classes, nonce widths and noncanonical CBOR.
    pub fn decode_canonical(bytes: &[u8]) -> Result<Self> {
        let [version, class, nonce] = codec::array(cbor::decode(bytes)?)?;
        codec::version(&version)?;
        let class = u16::try_from(codec::unsigned(&class)?)
            .map_err(|_| Error::InvalidField("network class width"))?;
        Ok(Self::new(
            NetworkClass::try_from(class)?,
            codec::bytes(&nonce)?,
        ))
    }
}
