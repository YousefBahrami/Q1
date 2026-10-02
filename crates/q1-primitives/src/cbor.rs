//! Q1's restricted deterministic CBOR profile.

use crate::{Error, Result};

/// Maximum permitted nesting, counting the top-level item.
pub const MAX_NESTING_DEPTH: u8 = 16;
/// Maximum permitted byte-string length.
pub const MAX_BYTE_STRING_LENGTH: usize = 16 * 1024 * 1024;
/// Maximum permitted consensus-critical text length.
pub const MAX_TEXT_STRING_LENGTH: usize = 128;
/// Maximum permitted array item count.
pub const MAX_ARRAY_ENTRIES: usize = 65_535;
/// Maximum permitted canonical object length.
pub const MAX_OBJECT_SIZE: usize = 16 * 1024 * 1024;

/// A value in the deliberately small Q1 CBOR data model.
///
/// Maps, tags, negative integers, floats, and generic simple values have no
/// variants, preventing an encoder from emitting prohibited constructions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    /// An RFC 8949 unsigned integer, bounded to CBOR's native `u64`.
    Unsigned(u64),
    /// A definite-length byte string.
    Bytes(Vec<u8>),
    /// A schema-authorized printable-ASCII text string.
    Text(String),
    /// A definite-length ordered collection.
    Array(Vec<Self>),
    /// A schema-authorized boolean.
    Bool(bool),
    /// A schema-authorized explicit null.
    Null,
}

/// Encodes one value according to the restricted deterministic profile.
pub fn encode(value: &Value) -> Result<Vec<u8>> {
    let mut output = Vec::new();
    encode_into(value, &mut output, 1)?;
    if output.len() > MAX_OBJECT_SIZE {
        return Err(Error::CborLimitExceeded);
    }
    Ok(output)
}

/// Decodes exactly one restricted deterministic CBOR item.
pub fn decode(bytes: &[u8]) -> Result<Value> {
    if bytes.len() > MAX_OBJECT_SIZE {
        return Err(Error::CborLimitExceeded);
    }
    let mut cursor = 0;
    let value = decode_item(bytes, &mut cursor, 1)?;
    if cursor != bytes.len() {
        return Err(Error::CborTrailingBytes);
    }
    Ok(value)
}

/// Encodes an unsigned integer using its shortest RFC 8949 representation.
pub fn encode_unsigned(value: u64) -> Vec<u8> {
    let mut output = Vec::with_capacity(9);
    encode_head(0, value, &mut output);
    output
}

/// Decodes one top-level unsigned integer.
pub fn decode_unsigned(bytes: &[u8]) -> Result<u64> {
    match decode(bytes)? {
        Value::Unsigned(value) => Ok(value),
        _ => Err(Error::CborUnexpectedType),
    }
}

/// Encodes a byte string with a definite shortest-form length.
pub fn encode_bytes(value: &[u8]) -> Result<Vec<u8>> {
    if value.len() > MAX_BYTE_STRING_LENGTH {
        return Err(Error::CborLimitExceeded);
    }
    let mut output = Vec::with_capacity(9 + value.len());
    encode_head(2, value.len() as u64, &mut output);
    output.extend_from_slice(value);
    Ok(output)
}

/// Decodes one top-level byte string.
pub fn decode_bytes(bytes: &[u8]) -> Result<Vec<u8>> {
    match decode(bytes)? {
        Value::Bytes(value) => Ok(value),
        _ => Err(Error::CborUnexpectedType),
    }
}

fn encode_into(value: &Value, output: &mut Vec<u8>, depth: u8) -> Result<()> {
    if depth > MAX_NESTING_DEPTH {
        return Err(Error::CborLimitExceeded);
    }
    match value {
        Value::Unsigned(value) => encode_head(0, *value, output),
        Value::Bytes(value) => {
            if value.len() > MAX_BYTE_STRING_LENGTH {
                return Err(Error::CborLimitExceeded);
            }
            encode_head(2, value.len() as u64, output);
            output.extend_from_slice(value);
        }
        Value::Text(value) => {
            validate_text(value.as_bytes())?;
            encode_head(3, value.len() as u64, output);
            output.extend_from_slice(value.as_bytes());
        }
        Value::Array(values) => {
            if values.len() > MAX_ARRAY_ENTRIES {
                return Err(Error::CborLimitExceeded);
            }
            encode_head(4, values.len() as u64, output);
            for value in values {
                encode_into(value, output, depth + 1)?;
            }
        }
        Value::Bool(false) => output.push(0xf4),
        Value::Bool(true) => output.push(0xf5),
        Value::Null => output.push(0xf6),
    }
    if output.len() > MAX_OBJECT_SIZE {
        return Err(Error::CborLimitExceeded);
    }
    Ok(())
}

fn encode_head(major: u8, value: u64, output: &mut Vec<u8>) {
    let major = major << 5;
    match value {
        0..=23 => output.push(major | value as u8),
        24..=0xff => output.extend_from_slice(&[major | 24, value as u8]),
        0x100..=0xffff => {
            output.push(major | 25);
            output.extend_from_slice(&(value as u16).to_be_bytes());
        }
        0x1_0000..=0xffff_ffff => {
            output.push(major | 26);
            output.extend_from_slice(&(value as u32).to_be_bytes());
        }
        _ => {
            output.push(major | 27);
            output.extend_from_slice(&value.to_be_bytes());
        }
    }
}

fn decode_item(bytes: &[u8], cursor: &mut usize, depth: u8) -> Result<Value> {
    if depth > MAX_NESTING_DEPTH {
        return Err(Error::CborLimitExceeded);
    }
    let initial = take_byte(bytes, cursor)?;
    let major = initial >> 5;
    let additional = initial & 31;
    match major {
        0 => Ok(Value::Unsigned(read_argument(bytes, cursor, additional)?)),
        2 => {
            let length = length_to_usize(read_argument(bytes, cursor, additional)?)?;
            if length > MAX_BYTE_STRING_LENGTH {
                return Err(Error::CborLimitExceeded);
            }
            Ok(Value::Bytes(take(bytes, cursor, length)?.to_vec()))
        }
        3 => {
            let length = length_to_usize(read_argument(bytes, cursor, additional)?)?;
            if length > MAX_TEXT_STRING_LENGTH {
                return Err(Error::CborLimitExceeded);
            }
            let text = take(bytes, cursor, length)?;
            validate_text(text)?;
            Ok(Value::Text(
                core::str::from_utf8(text)
                    .map_err(|_| Error::CborInvalidText)?
                    .to_owned(),
            ))
        }
        4 => {
            let length = length_to_usize(read_argument(bytes, cursor, additional)?)?;
            if length > MAX_ARRAY_ENTRIES {
                return Err(Error::CborLimitExceeded);
            }
            // Every item needs at least one byte. Reject impossible declarations
            // before allocating, then grow from parsed input rather than a count
            // supplied by an untrusted sender.
            if length > bytes.len().saturating_sub(*cursor) {
                return Err(Error::CborTruncated);
            }
            let mut values = Vec::new();
            for _ in 0..length {
                values.push(decode_item(bytes, cursor, depth + 1)?);
            }
            Ok(Value::Array(values))
        }
        7 if additional == 20 => Ok(Value::Bool(false)),
        7 if additional == 21 => Ok(Value::Bool(true)),
        7 if additional == 22 => Ok(Value::Null),
        _ => Err(Error::CborTypeProhibited(major)),
    }
}

fn read_argument(bytes: &[u8], cursor: &mut usize, additional: u8) -> Result<u64> {
    let width = match additional {
        0..=23 => return Ok(u64::from(additional)),
        24 => 1,
        25 => 2,
        26 => 4,
        27 => 8,
        _ => return Err(Error::CborNonCanonical),
    };
    let raw = take(bytes, cursor, width)?;
    let mut value = 0_u64;
    for byte in raw {
        value = (value << 8) | u64::from(*byte);
    }
    let minimum = match width {
        1 => 24,
        2 => 0x100,
        4 => 0x1_0000,
        8 => 0x1_0000_0000,
        _ => unreachable!(),
    };
    if value < minimum {
        return Err(Error::CborNonCanonical);
    }
    Ok(value)
}

fn validate_text(bytes: &[u8]) -> Result<()> {
    if bytes.len() > MAX_TEXT_STRING_LENGTH {
        return Err(Error::CborLimitExceeded);
    }
    if bytes.iter().all(|byte| (0x21..=0x7e).contains(byte)) {
        Ok(())
    } else {
        Err(Error::CborInvalidText)
    }
}

fn length_to_usize(value: u64) -> Result<usize> {
    usize::try_from(value).map_err(|_| Error::CborLimitExceeded)
}

fn take_byte(bytes: &[u8], cursor: &mut usize) -> Result<u8> {
    let value = *bytes.get(*cursor).ok_or(Error::CborTruncated)?;
    *cursor += 1;
    Ok(value)
}

fn take<'a>(bytes: &'a [u8], cursor: &mut usize, length: usize) -> Result<&'a [u8]> {
    let end = cursor.checked_add(length).ok_or(Error::CborLimitExceeded)?;
    let value = bytes.get(*cursor..end).ok_or(Error::CborTruncated)?;
    *cursor = end;
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shortest_integer_is_enforced() {
        assert_eq!(encode_unsigned(23), vec![0x17]);
        assert_eq!(decode(&[0x18, 0x17]), Err(Error::CborNonCanonical));
    }

    #[test]
    fn declared_limits_are_rejected_before_payload_allocation() {
        let oversized_bytes = [0x5a, 0x01, 0x00, 0x00, 0x01];
        assert_eq!(decode(&oversized_bytes), Err(Error::CborLimitExceeded));

        let oversized_array = [0x9a, 0x00, 0x01, 0x00, 0x00];
        assert_eq!(decode(&oversized_array), Err(Error::CborLimitExceeded));
    }

    #[test]
    fn nesting_limit_is_enforced() {
        let mut bytes = vec![0x81; usize::from(MAX_NESTING_DEPTH)];
        bytes.push(0x00);
        assert_eq!(decode(&bytes), Err(Error::CborLimitExceeded));
    }

    #[test]
    fn truncated_array_count_does_not_drive_allocation() {
        assert_eq!(decode(&[0x99, 0xff, 0xff]), Err(Error::CborTruncated));
        assert_eq!(decode(&[0x82, 0x00]), Err(Error::CborTruncated));
    }
}
