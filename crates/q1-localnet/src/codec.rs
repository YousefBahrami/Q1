//! Exact structural helpers for the explicitly local schemas.
use crate::{Error, Result};
use q1_primitives::cbor::Value;
pub(crate) fn array<const N: usize>(value: Value) -> Result<[Value; N]> {
    let Value::Array(values) = value else {
        return Err(Error::InvalidBlock("expected array"));
    };
    values
        .try_into()
        .map_err(|_| Error::InvalidBlock("field count"))
}
pub(crate) fn unsigned(value: &Value) -> Result<u64> {
    if let Value::Unsigned(value) = value {
        Ok(*value)
    } else {
        Err(Error::InvalidBlock("expected unsigned"))
    }
}
pub(crate) fn version(value: &Value) -> Result<()> {
    if unsigned(value)? == 1 {
        Ok(())
    } else {
        Err(Error::InvalidBlock("unsupported local schema version"))
    }
}
pub(crate) fn bytes<const N: usize>(value: &Value) -> Result<[u8; N]> {
    let Value::Bytes(bytes) = value else {
        return Err(Error::InvalidBlock("expected bytes"));
    };
    bytes
        .as_slice()
        .try_into()
        .map_err(|_| Error::InvalidBlock("byte width"))
}
pub(crate) fn value(bytes: &[u8]) -> Result<Value> {
    Ok(q1_primitives::cbor::decode(bytes)?)
}
