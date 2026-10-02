//! Immutable 32-byte hash wrapper types.

use core::{fmt, str::FromStr};

use crate::{Error, Result};

macro_rules! hash_type {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name([u8; 32]);

        impl $name {
            /// Constructs the typed hash from exactly 32 bytes.
            #[must_use]
            pub const fn from_bytes(bytes: [u8; 32]) -> Self {
                Self(bytes)
            }

            /// Returns a reference to the immutable bytes.
            #[must_use]
            pub const fn as_bytes(&self) -> &[u8; 32] {
                &self.0
            }

            /// Consumes the wrapper and returns its bytes.
            #[must_use]
            pub const fn into_bytes(self) -> [u8; 32] {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write_lower_hex(&self.0, formatter)
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "{}(", stringify!($name))?;
                write_lower_hex(&self.0, formatter)?;
                formatter.write_str(")")
            }
        }

        impl FromStr for $name {
            type Err = Error;

            fn from_str(value: &str) -> Result<Self> {
                Ok(Self(parse_lower_hex_32(value)?))
            }
        }

        impl From<[u8; 32]> for $name {
            fn from(value: [u8; 32]) -> Self {
                Self(value)
            }
        }

        impl From<$name> for [u8; 32] {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

hash_type!(
    /// A hash that identifies a block under its approved domain.
    BlockHash
);
hash_type!(
    /// A hash that identifies a complete transaction under its approved domain.
    TransactionHash
);
hash_type!(
    /// A typed commitment to protocol state.
    StateHash
);
hash_type!(
    /// A typed Merkle leaf or internal-node hash.
    MerkleHash
);
hash_type!(
    /// A 32-byte SHA-256 result without a more specific semantic type.
    GenericHash
);

fn parse_lower_hex_32(value: &str) -> Result<[u8; 32]> {
    if value.len() != 64 {
        return Err(Error::InvalidLength {
            expected: 64,
            actual: value.len(),
        });
    }
    let mut output = [0_u8; 32];
    for (index, pair) in value.as_bytes().chunks_exact(2).enumerate() {
        let high = lower_hex_nibble(pair[0])?;
        let low = lower_hex_nibble(pair[1])?;
        output[index] = (high << 4) | low;
    }
    Ok(output)
}

fn lower_hex_nibble(value: u8) -> Result<u8> {
    match value {
        b'0'..=b'9' => Ok(value - b'0'),
        b'a'..=b'f' => Ok(value - b'a' + 10),
        _ => Err(Error::InvalidHex),
    }
}

fn write_lower_hex(bytes: &[u8], formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    for byte in bytes {
        formatter.write_str(
            core::str::from_utf8(&[DIGITS[(byte >> 4) as usize], DIGITS[(byte & 0x0f) as usize]])
                .map_err(|_| fmt::Error)?,
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formatting_is_lowercase_and_exact() {
        let hash = GenericHash::from_bytes([0xab; 32]);
        assert_eq!(hash.to_string(), "ab".repeat(32));
        assert_eq!(hash.to_string().parse(), Ok(hash));
    }
}
