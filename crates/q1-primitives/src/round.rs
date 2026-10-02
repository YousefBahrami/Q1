//! Deterministic round identifiers and legacy slot-bound compatibility types.

use crate::{
    Result,
    cbor::{Value, decode, encode},
    traits::{CanonicalDecode, CanonicalEncode},
};

/// A standalone protocol round number.
///
/// Round numbers begin at [`Self::ZERO`], and [`Default`] is exactly
/// [`Self::ZERO`]. The type is independent of slots, heights, clocks,
/// schedulers, and runtime behavior. Its private `u32` value protects the
/// approved protocol width while preserving numeric ordering.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RoundNumber(u32);

/// A legacy non-consensus slot data type.
///
/// Its historical encoding is retained temporarily for compatibility. It has
/// no active approved consensus role or scheduling semantics.
#[deprecated(
    note = "legacy non-consensus data type; do not use as Height or RoundNumber, and do not introduce new uses; removal requires a later Gate"
)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Slot(u64);

/// A legacy composite slot-bound round.
///
/// Its historical encoding is retained temporarily for compatibility. It is
/// not the approved consensus round primitive; new code must use
/// [`RoundNumber`].
#[deprecated(
    note = "legacy composite Slot-bound type; not the approved consensus round primitive; new code must use RoundNumber; removal requires a later Gate"
)]
#[allow(deprecated)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Round {
    slot: Slot,
    number: u32,
}

impl RoundNumber {
    /// The initial protocol round.
    pub const ZERO: Self = Self(0);

    /// Constructs an independent round number.
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the underlying `u32` value.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

#[allow(deprecated)]
impl Slot {
    /// Constructs a slot identifier.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying identifier.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

#[allow(deprecated)]
impl Round {
    /// Constructs a round identifier.
    #[must_use]
    pub const fn new(slot: Slot, number: u32) -> Self {
        Self { slot, number }
    }

    /// Returns the containing slot.
    #[must_use]
    pub const fn slot(self) -> Slot {
        self.slot
    }

    /// Returns the round number inside the slot.
    #[must_use]
    pub const fn number(self) -> u32 {
        self.number
    }
}

impl CanonicalEncode for RoundNumber {
    fn encode_canonical(&self) -> Result<Vec<u8>> {
        encode(&Value::Unsigned(u64::from(self.0)))
    }
}

impl CanonicalDecode for RoundNumber {
    fn decode_canonical(bytes: &[u8]) -> Result<Self> {
        match decode(bytes)? {
            Value::Unsigned(value) => Ok(Self(
                u32::try_from(value).map_err(|_| crate::Error::ConversionOutOfRange)?,
            )),
            _ => Err(crate::Error::CborUnexpectedType),
        }
    }
}

#[allow(deprecated)]
impl CanonicalEncode for Slot {
    fn encode_canonical(&self) -> Result<Vec<u8>> {
        encode(&Value::Unsigned(self.0))
    }
}

#[allow(deprecated)]
impl CanonicalDecode for Slot {
    fn decode_canonical(bytes: &[u8]) -> Result<Self> {
        match decode(bytes)? {
            Value::Unsigned(value) => Ok(Self(value)),
            _ => Err(crate::Error::CborUnexpectedType),
        }
    }
}

#[allow(deprecated)]
impl CanonicalEncode for Round {
    fn encode_canonical(&self) -> Result<Vec<u8>> {
        encode(&Value::Array(vec![
            Value::Unsigned(1),
            Value::Unsigned(self.slot.0),
            Value::Unsigned(u64::from(self.number)),
        ]))
    }
}

#[allow(deprecated)]
impl CanonicalDecode for Round {
    fn decode_canonical(bytes: &[u8]) -> Result<Self> {
        match decode(bytes)? {
            Value::Array(values) if values.len() == 3 => match values.as_slice() {
                [
                    Value::Unsigned(1),
                    Value::Unsigned(slot),
                    Value::Unsigned(number),
                ] => Ok(Self::new(
                    Slot(*slot),
                    u32::try_from(*number).map_err(|_| crate::Error::ConversionOutOfRange)?,
                )),
                _ => Err(crate::Error::CborUnexpectedType),
            },
            _ => Err(crate::Error::CborUnexpectedType),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_number_zero_is_the_default() {
        assert_eq!(RoundNumber::ZERO, RoundNumber::new(0));
        assert_eq!(RoundNumber::default(), RoundNumber::ZERO);
    }

    #[test]
    #[allow(deprecated)]
    fn legacy_round_is_only_typed_data() {
        let value = Round::new(Slot::new(7), 2);
        assert_eq!(value.slot(), Slot::new(7));
        assert_eq!(value.number(), 2);
    }
}
