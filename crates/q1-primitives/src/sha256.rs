//! SHA-256 adapter for Q1 primitives.

use sha2::{Digest, Sha256};

use crate::{
    GenericHash, Result,
    domain::{Domain, frame},
    traits::DomainHash,
};

/// Hashes raw bytes with standard SHA-256 for internal standards-vector use.
///
/// This is deliberately not public: ADR-0003 permits only the domain-framed
/// adapter as the crate's consensus-facing hash interface.
#[must_use]
fn hash_raw(bytes: &[u8]) -> GenericHash {
    GenericHash::from_bytes(Sha256::digest(bytes).into())
}

/// Frames payload bytes under an approved domain and returns SHA-256.
pub fn hash_domain(domain: Domain, payload: &[u8]) -> Result<GenericHash> {
    Ok(hash_raw(&frame(domain, payload)?))
}

impl DomainHash for [u8] {
    fn domain_hash(&self, domain: Domain) -> Result<GenericHash> {
        hash_domain(domain, self)
    }
}

impl DomainHash for Vec<u8> {
    fn domain_hash(&self, domain: Domain) -> Result<GenericHash> {
        self.as_slice().domain_hash(domain)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_sha256_matches_standard() {
        assert_eq!(
            hash_raw(&[]).to_string(),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }
}
