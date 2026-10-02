//! Small reusable traits for canonical protocol operations.

use crate::{GenericHash, Result, domain::Domain};

/// Encodes a value into its one approved canonical byte representation.
pub trait CanonicalEncode {
    /// Returns the canonical bytes or a structured validation error.
    fn encode_canonical(&self) -> Result<Vec<u8>>;
}

/// Decodes exactly one canonical value and rejects every trailing byte.
pub trait CanonicalDecode: Sized {
    /// Parses a value from canonical bytes.
    fn decode_canonical(bytes: &[u8]) -> Result<Self>;
}

/// Produces a SHA-256 digest bound to an approved Q1 domain.
pub trait DomainHash {
    /// Hashes canonical payload bytes under `domain`.
    fn domain_hash(&self, domain: Domain) -> Result<GenericHash>;
}

/// Verifies a signature against an exact message.
pub trait VerifySignature {
    /// Signature type accepted by the verifier.
    type Signature;

    /// Performs strict verification and rejects malformed inputs.
    fn verify_signature(&self, message: &[u8], signature: &Self::Signature) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Amount, traits::CanonicalDecode};

    #[test]
    fn canonical_traits_compose_without_dynamic_dispatch() {
        let encoded = Amount::from(1_u8).encode_canonical().unwrap();
        assert_eq!(Amount::decode_canonical(&encoded), Ok(Amount::from(1_u8)));
    }
}
