//! Approved transfer-only block body and its derived semantic count.

use q1_primitives::cbor::{self, Value};

use crate::{Result, codec, transfer::SignedTransferV1, types::TransactionCount};

/// Canonical V1 block body, preserving the supplied transfer sequence exactly.
///
/// Duplicate transfers are preserved: their runtime validity is a separate,
/// undecided block-validation rule. This type performs no transfer execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlockBodyV1 {
    transfers: Vec<SignedTransferV1>,
}

impl BlockBodyV1 {
    /// Validates the canonical format ceilings without sorting or deduplicating.
    ///
    /// These ceilings are not approved network or operational resource limits.
    pub fn new(transfers: Vec<SignedTransferV1>) -> Result<Self> {
        if transfers.len() > cbor::MAX_ARRAY_ENTRIES {
            return Err(q1_primitives::Error::CborLimitExceeded.into());
        }
        u32::try_from(transfers.len()).map_err(|_| q1_primitives::Error::ConversionOutOfRange)?;
        let body = Self { transfers };
        body.encode_canonical()?;
        Ok(body)
    }

    /// Returns transfers in their normative block-body order.
    #[must_use]
    pub fn transfers(&self) -> &[SignedTransferV1] {
        &self.transfers
    }

    /// Derives the count from the committed array; no count field is serialized.
    #[must_use]
    pub fn transaction_count(&self) -> TransactionCount {
        // Construction bounds the sequence to the format ceiling, below u32::MAX.
        TransactionCount::new(self.transfers.len() as u32)
    }

    /// Encodes the exact two-field `[1, transfers]` canonical record.
    pub fn encode_canonical(&self) -> Result<Vec<u8>> {
        Ok(cbor::encode(&Value::Array(vec![
            Value::Unsigned(1),
            Value::Array(
                self.transfers
                    .iter()
                    .map(SignedTransferV1::to_value)
                    .collect(),
            ),
        ]))?)
    }

    /// Decodes one complete canonical body, rejecting unknown or trailing fields.
    pub fn decode_canonical(bytes: &[u8]) -> Result<Self> {
        let [version, transfers] = codec::array::<2>(cbor::decode(bytes)?)?;
        codec::version(&version)?;
        let Value::Array(transfers) = transfers else {
            return Err(q1_primitives::Error::CborUnexpectedType.into());
        };
        let transfers = transfers
            .into_iter()
            .map(SignedTransferV1::from_value)
            .collect::<Result<Vec<_>>>()?;
        Self::new(transfers)
    }
}
