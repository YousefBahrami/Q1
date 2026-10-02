//! Strongly typed block heights.

use crate::{
    Error, Result,
    cbor::{decode_unsigned, encode_unsigned},
    traits::{CanonicalDecode, CanonicalEncode},
};

/// A zero-based position in an ordered block sequence.
///
/// The type contains no clock or consensus behavior. Checked increment avoids
/// wrapping a terminal height back to genesis.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Height(u64);

impl Height {
    /// Genesis height.
    pub const ZERO: Self = Self(0);
    /// Greatest representable height.
    pub const MAX: Self = Self(u64::MAX);

    /// Constructs a height.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying integer.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Returns the next height or rejects overflow.
    pub fn checked_increment(self) -> Result<Self> {
        self.0.checked_add(1).map(Self).ok_or(Error::Overflow)
    }
}

impl CanonicalEncode for Height {
    fn encode_canonical(&self) -> Result<Vec<u8>> {
        Ok(encode_unsigned(self.0))
    }
}

impl CanonicalDecode for Height {
    fn decode_canonical(bytes: &[u8]) -> Result<Self> {
        Ok(Self(decode_unsigned(bytes)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maximum_height_does_not_wrap() {
        assert_eq!(Height::MAX.checked_increment(), Err(Error::Overflow));
    }
}
