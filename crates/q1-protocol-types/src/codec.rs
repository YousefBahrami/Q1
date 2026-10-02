//! Shared structural checks; no normalization or runtime policy.

use crate::{Error, Result};
use q1_primitives::cbor::Value;

pub(crate) fn array<const N: usize>(value: Value) -> Result<[Value; N]> {
    match value {
        Value::Array(values) => values
            .try_into()
            .map_err(|values: Vec<Value>| Error::FieldCount {
                expected: N,
                actual: values.len(),
            }),
        _ => Err(q1_primitives::Error::CborUnexpectedType.into()),
    }
}

pub(crate) fn unsigned(value: &Value) -> Result<u64> {
    match value {
        Value::Unsigned(value) => Ok(*value),
        _ => Err(q1_primitives::Error::CborUnexpectedType.into()),
    }
}

pub(crate) fn version(value: &Value) -> Result<()> {
    match unsigned(value)? {
        1 => Ok(()),
        version => Err(Error::UnsupportedVersion(version)),
    }
}

pub(crate) fn bytes<const N: usize>(value: &Value) -> Result<[u8; N]> {
    match value {
        Value::Bytes(value) => value.as_slice().try_into().map_err(|_| {
            q1_primitives::Error::InvalidLength {
                expected: N,
                actual: value.len(),
            }
            .into()
        }),
        _ => Err(q1_primitives::Error::CborUnexpectedType.into()),
    }
}
