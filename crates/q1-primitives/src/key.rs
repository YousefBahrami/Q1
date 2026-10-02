//! Typed Ed25519 key and signature byte wrappers.

use core::fmt;

use ed25519_dalek::VerifyingKey;

use crate::{Error, Result};

/// The algorithm identifier assigned to the approved Q1 Ed25519 profile.
pub const ED25519_ALGORITHM_ID: u16 = 0x0001;

/// A validated 32-byte Ed25519 public key.
///
/// Construction rejects malformed, identity, and small-order keys so weak
/// keys cannot enter later protocol objects as apparently valid identifiers.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Ed25519PublicKey([u8; 32]);

/// An exact 64-byte Ed25519 `R || S` signature encoding.
///
/// Full scalar and point validity is checked by strict verification because
/// validity depends on both the public key and signed message.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Ed25519Signature([u8; 64]);

/// A testable private-key seed placeholder.
///
/// This type provides no storage, backup, derivation, wallet, or persistence
/// behavior. Debug output is redacted and memory is overwritten on drop.
pub struct Ed25519PrivateKey([u8; 32]);

impl Ed25519PublicKey {
    /// Parses a public key from a byte slice and rejects every non-32-byte input.
    pub fn try_from_slice(bytes: &[u8]) -> Result<Self> {
        let bytes: [u8; 32] = bytes.try_into().map_err(|_| Error::InvalidLength {
            expected: 32,
            actual: bytes.len(),
        })?;
        Self::from_bytes(bytes)
    }

    /// Parses and validates an exact Ed25519 public-key encoding.
    pub fn from_bytes(bytes: [u8; 32]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(&bytes).map_err(|_| Error::PublicKeyInvalid)?;
        if key.is_weak() || key.to_edwards().compress().to_bytes() != bytes {
            return Err(Error::PublicKeyInvalid);
        }
        Ok(Self(bytes))
    }

    /// Returns the canonical public-key bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub(crate) fn verifying_key(self) -> Result<VerifyingKey> {
        let key = VerifyingKey::from_bytes(&self.0).map_err(|_| Error::PublicKeyInvalid)?;
        if key.is_weak() || key.to_edwards().compress().to_bytes() != self.0 {
            return Err(Error::PublicKeyInvalid);
        }
        Ok(key)
    }
}

impl Ed25519Signature {
    /// Parses a signature from a byte slice and rejects every non-64-byte input.
    pub fn try_from_slice(bytes: &[u8]) -> Result<Self> {
        let bytes: [u8; 64] = bytes.try_into().map_err(|_| Error::InvalidLength {
            expected: 64,
            actual: bytes.len(),
        })?;
        Ok(Self(bytes))
    }

    /// Constructs a signature wrapper from exactly 64 bytes.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 64]) -> Self {
        Self(bytes)
    }

    /// Returns the canonical `R || S` bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 64] {
        &self.0
    }
}

impl Ed25519PrivateKey {
    /// Constructs a testable key placeholder from a 32-byte seed.
    ///
    /// Callers remain responsible for approved CSPRNG generation and custody.
    #[must_use]
    pub const fn from_seed(seed: [u8; 32]) -> Self {
        Self(seed)
    }

    pub(crate) const fn seed(&self) -> &[u8; 32] {
        &self.0
    }
}

impl Drop for Ed25519PrivateKey {
    fn drop(&mut self) {
        self.0.fill(0);
        core::hint::black_box(&mut self.0);
    }
}

impl fmt::Debug for Ed25519PublicKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("Ed25519PublicKey")
            .field(&"[32 bytes]")
            .finish()
    }
}

impl fmt::Debug for Ed25519Signature {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("Ed25519Signature")
            .field(&"[64 bytes]")
            .finish()
    }
}

impl fmt::Debug for Ed25519PrivateKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Ed25519PrivateKey([REDACTED])")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn private_debug_is_redacted() {
        let key = Ed25519PrivateKey::from_seed([42; 32]);
        assert_eq!(format!("{key:?}"), "Ed25519PrivateKey([REDACTED])");
    }
}
