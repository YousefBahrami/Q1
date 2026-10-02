//! Strict pure-Ed25519 adapter.

use ed25519_dalek::{Signature, Signer, SigningKey};

use crate::{
    Error, Result,
    domain::{Domain, frame},
    key::{Ed25519PrivateKey, Ed25519PublicKey, Ed25519Signature},
    traits::VerifySignature,
};

/// Derives the validated public key corresponding to a private seed.
pub fn public_key(private_key: &Ed25519PrivateKey) -> Result<Ed25519PublicKey> {
    let key = SigningKey::from_bytes(private_key.seed());
    Ed25519PublicKey::from_bytes(key.verifying_key().to_bytes())
}

/// Signs exact message bytes with pure Ed25519.
///
/// Protocol objects should normally use [`sign_domain`] to prevent unframed
/// signing. This exact-message function exists for RFC 8032 vector testing.
#[must_use]
pub fn sign(private_key: &Ed25519PrivateKey, message: &[u8]) -> Ed25519Signature {
    let key = SigningKey::from_bytes(private_key.seed());
    Ed25519Signature::from_bytes(key.sign(message).to_bytes())
}

/// Frames canonical payload bytes and signs the frame directly.
pub fn sign_domain(
    private_key: &Ed25519PrivateKey,
    domain: Domain,
    canonical_payload: &[u8],
) -> Result<Ed25519Signature> {
    Ok(sign(private_key, &frame(domain, canonical_payload)?))
}

/// Verifies according to the approved strict Q1 profile.
///
/// This intentionally aliases strict verification. Q1 does not expose the
/// dalek permissive verifier as a protocol-validity API.
pub fn verify(
    public_key: Ed25519PublicKey,
    message: &[u8],
    signature: Ed25519Signature,
) -> Result<()> {
    verify_strict(public_key, message, signature)
}

/// Strictly verifies canonical points, rejects weak keys and enforces `S < L`.
pub fn verify_strict(
    public_key: Ed25519PublicKey,
    message: &[u8],
    signature: Ed25519Signature,
) -> Result<()> {
    let key = public_key.verifying_key()?;
    let signature = Signature::from_bytes(signature.as_bytes());
    key.verify_strict(message, &signature)
        .map_err(|_| Error::VerificationFailed)
}

/// Strictly verifies a signature over an approved domain frame.
pub fn verify_domain_strict(
    public_key: Ed25519PublicKey,
    domain: Domain,
    canonical_payload: &[u8],
    signature: Ed25519Signature,
) -> Result<()> {
    verify_strict(public_key, &frame(domain, canonical_payload)?, signature)
}

impl VerifySignature for Ed25519PublicKey {
    type Signature = Ed25519Signature;

    fn verify_signature(&self, message: &[u8], signature: &Self::Signature) -> Result<()> {
        verify_strict(*self, message, *signature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_signature_and_mutation_diverge() {
        let private = Ed25519PrivateKey::from_seed([7; 32]);
        let public = public_key(&private).unwrap();
        let signature = sign(&private, b"message");
        assert!(verify_strict(public, b"message", signature).is_ok());
        assert!(verify_strict(public, b"mutated", signature).is_err());
    }
}
