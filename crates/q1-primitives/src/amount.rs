//! Strongly typed Q1 monetary amounts.

use crate::{
    Error, Result,
    cbor::{decode_bytes, encode_bytes},
    traits::{CanonicalDecode, CanonicalEncode},
};

/// An unsigned Q1 monetary amount.
///
/// The `u128` bound makes overflow explicit. Canonical CBOR is always a
/// definite 16-byte big-endian byte string, even when the value fits in
/// `u64`; this prevents one amount from having multiple protocol encodings.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Amount(u128);

impl Amount {
    /// The additive identity.
    pub const ZERO: Self = Self(0);
    /// The greatest representable protocol amount.
    pub const MAX: Self = Self(u128::MAX);

    /// Constructs an amount from its exact unsigned value.
    #[must_use]
    pub const fn new(value: u128) -> Self {
        Self(value)
    }

    /// Returns the wrapped unsigned value.
    #[must_use]
    pub const fn get(self) -> u128 {
        self.0
    }

    /// Adds two amounts, rejecting overflow rather than wrapping.
    pub fn checked_add(self, rhs: Self) -> Result<Self> {
        self.0.checked_add(rhs.0).map(Self).ok_or(Error::Overflow)
    }

    /// Subtracts an amount, rejecting a negative result.
    pub fn checked_sub(self, rhs: Self) -> Result<Self> {
        self.0.checked_sub(rhs.0).map(Self).ok_or(Error::Underflow)
    }

    /// Multiplies by an unsigned factor, rejecting overflow.
    pub fn checked_mul(self, rhs: u128) -> Result<Self> {
        self.0.checked_mul(rhs).map(Self).ok_or(Error::Overflow)
    }

    /// Divides by a non-zero unsigned divisor.
    ///
    /// Division by zero returns a machine-readable conversion error; no panic
    /// or floating-point fallback is permitted.
    pub fn checked_div(self, rhs: u128) -> Result<Self> {
        self.0
            .checked_div(rhs)
            .map(Self)
            .ok_or(Error::DivisionByZero)
    }

    /// Returns the exact 16-byte big-endian payload used by canonical CBOR.
    #[must_use]
    pub const fn to_be_bytes(self) -> [u8; 16] {
        self.0.to_be_bytes()
    }

    /// Constructs an amount from its exact 16-byte big-endian payload.
    #[must_use]
    pub const fn from_be_bytes(bytes: [u8; 16]) -> Self {
        Self(u128::from_be_bytes(bytes))
    }
}

impl From<u8> for Amount {
    fn from(value: u8) -> Self {
        Self(u128::from(value))
    }
}

impl From<u16> for Amount {
    fn from(value: u16) -> Self {
        Self(u128::from(value))
    }
}

impl From<u32> for Amount {
    fn from(value: u32) -> Self {
        Self(u128::from(value))
    }
}

impl From<u64> for Amount {
    fn from(value: u64) -> Self {
        Self(u128::from(value))
    }
}

impl TryFrom<Amount> for u64 {
    type Error = Error;

    fn try_from(value: Amount) -> Result<Self> {
        Self::try_from(value.0).map_err(|_| Error::ConversionOutOfRange)
    }
}

impl CanonicalEncode for Amount {
    fn encode_canonical(&self) -> Result<Vec<u8>> {
        encode_bytes(&self.to_be_bytes())
    }
}

impl CanonicalDecode for Amount {
    fn decode_canonical(bytes: &[u8]) -> Result<Self> {
        let payload = decode_bytes(bytes)?;
        let payload: [u8; 16] =
            payload
                .try_into()
                .map_err(|value: Vec<u8>| Error::InvalidLength {
                    expected: 16,
                    actual: value.len(),
                })?;
        Ok(Self::from_be_bytes(payload))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checked_arithmetic_never_wraps() {
        assert_eq!(
            Amount::MAX.checked_add(Amount::from(1_u8)),
            Err(Error::Overflow)
        );
        assert_eq!(
            Amount::ZERO.checked_sub(Amount::from(1_u8)),
            Err(Error::Underflow)
        );
    }
}
