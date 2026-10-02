//! Typed parent references approved in DEC-Q1-027 Session 5A.
//!
//! Parsing establishes a tagged identity, not its provenance or finality. The
//! caller supplies authenticated chain context to [`ParentReferenceV1::validate_for_height`].

use q1_primitives::{
    Height,
    cbor::{self, Value},
};

use crate::{Error, Result, codec};

/// Identity of an independently encoded genesis manifest.
///
/// Wrapping bytes does not calculate or validate a GenesisManifest commitment.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GenesisId([u8; 32]);

impl GenesisId {
    /// Wraps a genesis identifier supplied by a trusted context or typed decoder.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Returns the identifier's fixed-width bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// Identity of a complete signed block header, distinct from a genesis identity.
///
/// Wrapping bytes does not calculate or validate a signed-header commitment.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BlockId([u8; 32]);

impl BlockId {
    /// Wraps a block identifier supplied by a trusted context or typed decoder.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Returns the identifier's fixed-width bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// Exact three-field `[1, parent_kind, parent_id:bytes32]` record.
///
/// No null or untagged all-zero sentinel represents genesis. Both variants
/// always carry a typed identity; zero digest bytes have no special meaning.
/// The approved structural rules do not reserve a forbidden digest value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParentReferenceV1 {
    /// Parent of the first signed block, governed by an independent genesis manifest.
    Genesis(GenesisId),
    /// Parent of a later signed block, identified by its complete signed header.
    Block(BlockId),
}

impl ParentReferenceV1 {
    /// Encodes the exact record, including the explicit GENESIS=1 or BLOCK=2 tag.
    pub fn encode_canonical(&self) -> Result<Vec<u8>> {
        let (kind, id) = match self {
            Self::Genesis(id) => (1, id.as_bytes()),
            Self::Block(id) => (2, id.as_bytes()),
        };
        Ok(cbor::encode(&Value::Array(vec![
            Value::Unsigned(1),
            Value::Unsigned(kind),
            Value::Bytes(id.to_vec()),
        ]))?)
    }

    /// Rejects unknown versions/kinds, missing/trailing fields and malformed CBOR.
    ///
    /// Decoding does not prove a hash's domain or that the referenced object exists.
    pub fn decode_canonical(bytes: &[u8]) -> Result<Self> {
        let [version, kind, id] = codec::array::<3>(cbor::decode(bytes)?)?;
        codec::version(&version)?;
        let id = codec::bytes::<32>(&id)?;
        match codec::unsigned(&kind)? {
            1 => Ok(Self::Genesis(GenesisId::from_bytes(id))),
            2 => Ok(Self::Block(BlockId::from_bytes(id))),
            _ => Err(Error::InvalidField("parent kind")),
        }
    }

    /// Checks the exact signed-block height boundary and expected parent identity.
    ///
    /// `governing_genesis` must come from the caller's authenticated chain context.
    /// Above height one, `previous_block` must identify the authenticated immediate
    /// predecessor; this helper does not authenticate history or prove finality.
    /// At height one, `previous_block` is unused because the parent is genesis.
    pub fn validate_for_height(
        &self,
        height: Height,
        governing_genesis: GenesisId,
        previous_block: Option<(Height, BlockId)>,
    ) -> Result<()> {
        match (height.get(), self) {
            (0, _) => Err(Error::InvalidField("signed block height is zero")),
            (1, Self::Genesis(id)) if *id == governing_genesis => Ok(()),
            (1, _) => Err(Error::InvalidField("governing genesis parent")),
            (_, Self::Genesis(_)) => Err(Error::InvalidField("genesis parent above height one")),
            (height, Self::Block(id)) => match previous_block {
                Some((previous_height, expected_id))
                    if previous_height.get() == height - 1 && *id == expected_id =>
                {
                    Ok(())
                }
                _ => Err(Error::InvalidField("immediate block parent")),
            },
        }
    }
}
