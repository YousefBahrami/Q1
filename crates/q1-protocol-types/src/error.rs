//! Structural object errors, separate from future execution and consensus errors.

use core::fmt;

/// Result of a protocol-object operation.
pub type Result<T> = core::result::Result<T, Error>;

/// An object cannot satisfy an approved encoding or structural invariant.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// An underlying primitive rejected the input.
    Primitive(q1_primitives::Error),
    /// A fixed-position object has the wrong number of fields.
    FieldCount {
        /// Required field count.
        expected: usize,
        /// Observed field count.
        actual: usize,
    },
    /// The object schema version is not supported.
    UnsupportedVersion(u64),
    /// A named structural field violates its invariant.
    InvalidField(&'static str),
    /// The signed chain identifier differs from the caller's expected chain.
    WrongChain,
    /// The caller's address network class differs from the expected class.
    WrongNetwork,
    /// A signing key does not match the sole sender key in the body.
    KeyMismatch,
    /// A canonical set is unsorted or contains duplicate keys.
    UnsortedOrDuplicate,
    /// An approved format ceiling was exceeded.
    ResourceLimit,
}

impl From<q1_primitives::Error> for Error {
    fn from(value: q1_primitives::Error) -> Self {
        Self::Primitive(value)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Primitive(error) => error.fmt(f),
            Self::FieldCount { expected, actual } => {
                write!(f, "expected {expected} fields, got {actual}")
            }
            Self::UnsupportedVersion(version) => write!(f, "unsupported schema version {version}"),
            Self::InvalidField(field) => write!(f, "invalid field: {field}"),
            Self::WrongChain => f.write_str("transfer belongs to another chain"),
            Self::WrongNetwork => f.write_str("address belongs to another network class"),
            Self::KeyMismatch => f.write_str("signing key does not match sender"),
            Self::UnsortedOrDuplicate => f.write_str("unsorted or duplicate canonical set key"),
            Self::ResourceLimit => f.write_str("protocol format ceiling exceeded"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Primitive(error) => Some(error),
            _ => None,
        }
    }
}
