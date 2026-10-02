//! Distinct immutable semantic types approved by DEC-Q1-027.

use crate::{Error, Result, codec};
use core::{fmt, str::FromStr};
use q1_primitives::{GenericHash, cbor};

macro_rules! bytes32_type {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name([u8; 32]);
        impl $name {
            /// Wraps exactly 32 bytes; derivation/provenance is checked by the owning object.
            #[must_use]
            pub const fn from_bytes(bytes: [u8; 32]) -> Self {
                Self(bytes)
            }
            /// Returns immutable binary bytes.
            #[must_use]
            pub const fn as_bytes(&self) -> &[u8; 32] {
                &self.0
            }
            /// Returns owned binary bytes.
            #[must_use]
            pub const fn into_bytes(self) -> [u8; 32] {
                self.0
            }
            /// Encodes the exact 32-byte CBOR string.
            pub fn encode_canonical(&self) -> Result<Vec<u8>> {
                Ok(cbor::encode_bytes(&self.0)?)
            }
            /// Decodes the exact 32-byte CBOR string.
            pub fn decode_canonical(bytes: &[u8]) -> Result<Self> {
                Ok(Self(codec::bytes(&cbor::decode(bytes)?)?))
            }
        }
        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                for byte in self.0 {
                    write!(formatter, "{byte:02x}")?;
                }
                Ok(())
            }
        }
        impl FromStr for $name {
            type Err = Error;
            fn from_str(value: &str) -> Result<Self> {
                Ok(Self(value.parse::<GenericHash>()?.into_bytes()))
            }
        }
    };
}
bytes32_type!(
    ChainId,
    "An identity derived from the canonical chain identity preimage."
);
bytes32_type!(
    ParticipantId,
    "An identity derived from participant role keys, excluding activation schedule."
);
bytes32_type!(
    ParticipantRecordHash,
    "A commitment to a complete participant record, including activation schedule."
);
bytes32_type!(
    TransferId,
    "The identifier of a complete signed transfer, including its signature."
);

macro_rules! unsigned_type {
    ($name:ident, $inner:ty, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name($inner);
        impl $name {
            /// Constructs the semantic value explicitly.
            #[must_use]
            pub const fn new(value: $inner) -> Self {
                Self(value)
            }
            /// Returns the underlying integer explicitly.
            #[must_use]
            pub const fn get(self) -> $inner {
                self.0
            }
            /// Encodes the shortest CBOR unsigned representation.
            pub fn encode_canonical(&self) -> Result<Vec<u8>> {
                Ok(cbor::encode_unsigned(u64::from(self.0)))
            }
            /// Rejects noncanonical encodings and values outside the semantic width.
            pub fn decode_canonical(bytes: &[u8]) -> Result<Self> {
                let value = cbor::decode_unsigned(bytes)?;
                Ok(Self(
                    value
                        .try_into()
                        .map_err(|_| Error::InvalidField(stringify!($name)))?,
                ))
            }
        }
    };
}
unsigned_type!(
    Weight,
    u64,
    "A role weight; participant records do not serialize it."
);
unsigned_type!(
    Nonce,
    u64,
    "An account nonce field; execution and replay rules are separate decisions."
);
unsigned_type!(ByteCount, u64, "A semantic byte count.");
unsigned_type!(CandidateIndex, u16, "A semantic candidate index.");

/// A count derived from the canonical block body's transfer sequence, never serialized separately.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TransactionCount(u32);
impl TransactionCount {
    /// Constructs the count, including the valid empty sequence count.
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }
    /// Returns the count explicitly.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

/// A nonzero count derived from the canonical participant set, never serialized separately.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ParticipantCount(u32);
impl ParticipantCount {
    /// Rejects the invalid empty participant count.
    pub fn new(value: u32) -> Result<Self> {
        if value == 0 {
            return Err(Error::InvalidField("participant count is zero"));
        }
        Ok(Self(value))
    }
    /// Returns the count explicitly.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

/// A fee limit with fixed 16-byte unsigned encoding and no implicit Amount conversion.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FeeLimit(u128);
impl FeeLimit {
    /// Zero fee limit.
    pub const ZERO: Self = Self(0);
    /// Maximum representable fee limit.
    pub const MAX: Self = Self(u128::MAX);
    /// Constructs a fee limit; no fee formula or charging behavior is implied.
    #[must_use]
    pub const fn new(value: u128) -> Self {
        Self(value)
    }
    /// Returns its unsigned value explicitly.
    #[must_use]
    pub const fn get(self) -> u128 {
        self.0
    }
    /// Returns the fixed-width big-endian payload.
    #[must_use]
    pub const fn to_be_bytes(self) -> [u8; 16] {
        self.0.to_be_bytes()
    }
    /// Constructs from the fixed-width big-endian payload.
    #[must_use]
    pub const fn from_be_bytes(bytes: [u8; 16]) -> Self {
        Self(u128::from_be_bytes(bytes))
    }
    /// Adds fee limits without wrapping.
    pub fn checked_add(self, rhs: Self) -> Result<Self> {
        Ok(Self(
            self.0
                .checked_add(rhs.0)
                .ok_or(q1_primitives::Error::Overflow)?,
        ))
    }
    /// Subtracts fee limits without underflow.
    pub fn checked_sub(self, rhs: Self) -> Result<Self> {
        Ok(Self(
            self.0
                .checked_sub(rhs.0)
                .ok_or(q1_primitives::Error::Underflow)?,
        ))
    }
    /// Multiplies without wrapping.
    pub fn checked_mul(self, rhs: u128) -> Result<Self> {
        Ok(Self(
            self.0
                .checked_mul(rhs)
                .ok_or(q1_primitives::Error::Overflow)?,
        ))
    }
    /// Divides by a nonzero unsigned value.
    pub fn checked_div(self, rhs: u128) -> Result<Self> {
        Ok(Self(
            self.0
                .checked_div(rhs)
                .ok_or(q1_primitives::Error::DivisionByZero)?,
        ))
    }
    /// Encodes only as a definite 16-byte CBOR byte string.
    pub fn encode_canonical(&self) -> Result<Vec<u8>> {
        Ok(cbor::encode_bytes(&self.to_be_bytes())?)
    }
    /// Rejects integers and byte strings with any other width.
    pub fn decode_canonical(bytes: &[u8]) -> Result<Self> {
        Ok(Self::from_be_bytes(codec::bytes(&cbor::decode(bytes)?)?))
    }
}
