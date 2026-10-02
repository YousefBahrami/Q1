//! Approved indexed sequence commitments for canonical transfer and participant owners.
//!
//! State, receipt, and snapshot profiles remain inactive. No public generic root
//! constructor permits exchanging their semantic identities or supplying raw leaves.

use q1_primitives::{domain::Domain, sha256::hash_domain};

use crate::{Result, block_body::BlockBodyV1, participant::ParticipantSetV1};

/// Profile 0x0001 commitment to the exact canonical block-body transfer sequence.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TransactionRoot([u8; 32]);

impl TransactionRoot {
    /// Parses a claimed wire commitment; block validation must recompute it.
    #[must_use]
    pub const fn from_claimed_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
    /// Commits to canonical signed transfers with indexes and derived body count.
    pub fn from_block_body(body: &BlockBodyV1) -> Result<Self> {
        Ok(Self(sequence_root(
            0x0001,
            body.transfers()
                .iter()
                .map(|transfer| transfer.encode_canonical()),
        )?))
    }

    /// Returns the immutable commitment bytes without changing its semantic type.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// Profile 0x0003 commitment to the records in a canonical active participant set.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ParticipantRoot([u8; 32]);

impl ParticipantRoot {
    /// Parses a claimed wire commitment; contextual validation must recompute it.
    #[must_use]
    pub const fn from_claimed_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
    /// Commits to the owning set's already validated, strictly ordered records.
    ///
    /// The owning set rejects empty, duplicate, unsorted, and inactive records.
    /// Reconstructing the set from finalized pre-height state is a runtime concern.
    pub fn from_participant_set(set: &ParticipantSetV1) -> Result<Self> {
        Ok(Self(sequence_root(
            0x0003,
            set.participants()
                .iter()
                .map(|record| record.encode_canonical()),
        )?))
    }

    /// Returns the immutable commitment bytes without changing its semantic type.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

fn sequence_root(profile: u16, items: impl Iterator<Item = Result<Vec<u8>>>) -> Result<[u8; 32]> {
    let mut level = Vec::new();
    for (index, item) in items.enumerate() {
        let item = item?;
        let index = u64::try_from(index).map_err(|_| q1_primitives::Error::ConversionOutOfRange)?;
        let length =
            u64::try_from(item.len()).map_err(|_| q1_primitives::Error::ConversionOutOfRange)?;
        let mut payload = Vec::new();
        payload.extend_from_slice(&profile.to_be_bytes());
        payload.extend_from_slice(&index.to_be_bytes());
        payload.extend_from_slice(&length.to_be_bytes());
        payload.extend_from_slice(&item);
        level.push(hash_domain(Domain::MerkleLeaf, &payload)?.into_bytes());
    }

    if level.is_empty() {
        let mut payload = [0_u8; 10];
        payload[..2].copy_from_slice(&profile.to_be_bytes());
        return Ok(hash_domain(Domain::MerkleLeaf, &payload)?.into_bytes());
    }

    while level.len() > 1 {
        // Compact in place. Each destination precedes the unread source pair.
        let next_len = level.len().div_ceil(2);
        for destination in 0..next_len {
            let left = 2 * destination;
            level[destination] = if left + 1 < level.len() {
                let mut payload = [0_u8; 66];
                payload[..2].copy_from_slice(&profile.to_be_bytes());
                payload[2..34].copy_from_slice(&level[left]);
                payload[34..].copy_from_slice(&level[left + 1]);
                hash_domain(Domain::MerkleInternal, &payload)?.into_bytes()
            } else {
                level[left]
            };
        }
        level.truncate(next_len);
    }
    Ok(level[0])
}
