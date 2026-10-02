//! Machine-readable errors for protocol primitive operations.

use core::fmt;

/// Result alias used throughout `q1-primitives`.
pub type Result<T> = core::result::Result<T, Error>;

/// A machine-readable failure produced by a protocol primitive.
///
/// Variants intentionally carry structured data rather than free-form strings
/// so callers cannot accidentally make protocol decisions by parsing text.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// Checked arithmetic exceeded the type's upper bound.
    Overflow,
    /// Checked arithmetic would produce a negative unsigned value.
    Underflow,
    /// A conversion cannot represent the source value safely.
    ConversionOutOfRange,
    /// Checked integer division received a zero divisor.
    DivisionByZero,
    /// Input length differs from the invariant required by the type.
    InvalidLength {
        /// Exact required byte or character count.
        expected: usize,
        /// Observed byte or character count.
        actual: usize,
    },
    /// A hexadecimal string is malformed or non-canonical.
    InvalidHex,
    /// A CBOR item uses a prohibited or unsupported type.
    CborTypeProhibited(u8),
    /// A CBOR argument is not encoded in its shortest form.
    CborNonCanonical,
    /// A CBOR item is truncated.
    CborTruncated,
    /// Bytes remain after the single expected CBOR item.
    CborTrailingBytes,
    /// A configured CBOR resource ceiling was exceeded.
    CborLimitExceeded,
    /// A CBOR text string violates the approved ASCII policy.
    CborInvalidText,
    /// The caller expected a different CBOR value kind.
    CborUnexpectedType,
    /// An address contains non-canonical casing, whitespace, or characters.
    AddressTextInvalid,
    /// An address checksum is invalid or uses Bech32 rather than Bech32m.
    AddressChecksumInvalid,
    /// An address uses a network other than the expected network.
    AddressWrongNetwork,
    /// An address envelope version is unsupported.
    AddressVersionUnsupported(u8),
    /// An address type is unsupported.
    AddressTypeUnsupported(u8),
    /// A cryptographic algorithm identifier is unsupported.
    AlgorithmUnsupported(u16),
    /// Bech32m data has non-zero or excessive conversion padding.
    AddressPaddingInvalid,
    /// A cryptographic domain identifier is not registered for protocol use.
    DomainUnknown(u16),
    /// An Ed25519 public key is malformed or weak.
    PublicKeyInvalid,
    /// An Ed25519 signature is malformed.
    SignatureInvalid,
    /// Signature verification failed.
    VerificationFailed,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Overflow => formatter.write_str("arithmetic overflow"),
            Self::Underflow => formatter.write_str("arithmetic underflow"),
            Self::ConversionOutOfRange => formatter.write_str("conversion out of range"),
            Self::DivisionByZero => formatter.write_str("division by zero"),
            Self::InvalidLength { expected, actual } => {
                write!(
                    formatter,
                    "invalid length: expected {expected}, got {actual}"
                )
            }
            Self::InvalidHex => formatter.write_str("invalid canonical hexadecimal input"),
            Self::CborTypeProhibited(major) => {
                write!(formatter, "prohibited CBOR major type {major}")
            }
            Self::CborNonCanonical => formatter.write_str("non-canonical CBOR"),
            Self::CborTruncated => formatter.write_str("truncated CBOR"),
            Self::CborTrailingBytes => formatter.write_str("trailing CBOR bytes"),
            Self::CborLimitExceeded => formatter.write_str("CBOR resource limit exceeded"),
            Self::CborInvalidText => formatter.write_str("CBOR text violates the Q1 policy"),
            Self::CborUnexpectedType => formatter.write_str("unexpected CBOR value type"),
            Self::AddressTextInvalid => formatter.write_str("invalid canonical address text"),
            Self::AddressChecksumInvalid => formatter.write_str("invalid Bech32m checksum"),
            Self::AddressWrongNetwork => formatter.write_str("address belongs to another network"),
            Self::AddressVersionUnsupported(version) => {
                write!(formatter, "unsupported address version {version}")
            }
            Self::AddressTypeUnsupported(kind) => {
                write!(formatter, "unsupported address type {kind}")
            }
            Self::AlgorithmUnsupported(algorithm) => {
                write!(formatter, "unsupported algorithm identifier {algorithm}")
            }
            Self::AddressPaddingInvalid => {
                formatter.write_str("invalid Bech32m conversion padding")
            }
            Self::DomainUnknown(domain) => write!(formatter, "unknown domain identifier {domain}"),
            Self::PublicKeyInvalid => formatter.write_str("invalid Ed25519 public key"),
            Self::SignatureInvalid => formatter.write_str("invalid Ed25519 signature"),
            Self::VerificationFailed => formatter.write_str("signature verification failed"),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn errors_remain_machine_readable() {
        let error = Error::InvalidLength {
            expected: 16,
            actual: 15,
        };
        assert!(matches!(
            error,
            Error::InvalidLength {
                expected: 16,
                actual: 15
            }
        ));
    }
}
