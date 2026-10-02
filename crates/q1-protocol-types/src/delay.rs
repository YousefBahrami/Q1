//! Exact NONE witness with explicit local-network activation checks.

use crate::{Error, Result, chain::NetworkClass, codec};
use q1_primitives::{
    cbor::{self, Value},
    domain::{self, Domain},
    sha256,
};

/// Typed commitment to complete canonical evidence, never a DELAY_OUTPUT hash.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DelayEvidenceHash([u8; 32]);
impl DelayEvidenceHash {
    /// Parses a claimed wire commitment; verification must recompute it from evidence.
    #[must_use]
    pub const fn from_claimed_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
    /// Returns the exact commitment bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// The sole currently active delay evidence form, explicitly LOCALNET v0 only.
///
/// There is no public default or production configuration that enables NONE.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalnetNoneEvidence(());
impl LocalnetNoneEvidence {
    /// Requires an explicit local network class; rejects every other class.
    pub fn new(network: NetworkClass) -> Result<Self> {
        if network != NetworkClass::Localnet {
            return Err(Error::WrongNetwork);
        }
        Ok(Self(()))
    }
    /// Encodes the approved exact `[1,0,0,h'',h'']` witness.
    pub fn encode_canonical(&self) -> Result<Vec<u8>> {
        Ok(cbor::encode(&Value::Array(vec![
            Value::Unsigned(1),
            Value::Unsigned(0),
            Value::Unsigned(0),
            Value::Bytes(vec![]),
            Value::Bytes(vec![]),
        ]))?)
    }
    /// Rejects noncanonical bytes, other engines, difficulty, output or proof.
    pub fn decode_for_network(network: NetworkClass, bytes: &[u8]) -> Result<Self> {
        let evidence = Self::new(network)?;
        let [version, engine, difficulty, output, proof] = codec::array(cbor::decode(bytes)?)?;
        codec::version(&version)?;
        if codec::unsigned(&engine)? != 0
            || codec::unsigned(&difficulty)? != 0
            || output != Value::Bytes(vec![])
            || proof != Value::Bytes(vec![])
        {
            return Err(Error::InvalidField("localnet NONE evidence"));
        }
        Ok(evidence)
    }
    /// Independently enforces activation at the verification boundary.
    pub fn verify_for_network(&self, network: NetworkClass) -> Result<()> {
        Self::new(network).map(|_| ())
    }
    /// Returns the complete domain frame, useful for independent vectors.
    pub fn commitment_frame(&self) -> Result<Vec<u8>> {
        Ok(domain::frame(
            Domain::DelayEvidence,
            &self.encode_canonical()?,
        )?)
    }
    /// Commits under DELAY_EVIDENCE=0x0014; the old delay domain is unchanged.
    pub fn commitment(&self) -> Result<DelayEvidenceHash> {
        Ok(DelayEvidenceHash(
            sha256::hash_domain(Domain::DelayEvidence, &self.encode_canonical()?)?.into_bytes(),
        ))
    }
}
