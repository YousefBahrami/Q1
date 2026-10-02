//! Fixed-membership two-of-three tally, not a certificate or signature verifier.
use crate::{Error, LocalnetV0, Result};
use q1_protocol_types::types::ParticipantId;

/// Explicitly supplied committee; selection is a separate protocol decision.
///
/// Connectivity is not an input and cannot shrink membership or the threshold.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalnetCommittee {
    producer: ParticipantId,
    voters: [ParticipantId; 3],
}
impl LocalnetCommittee {
    /// Accepts exactly three distinct voters, excluding the producer.
    pub fn new(_: LocalnetV0, producer: ParticipantId, voters: [ParticipantId; 3]) -> Result<Self> {
        if voters.contains(&producer)
            || voters[0] == voters[1]
            || voters[0] == voters[2]
            || voters[1] == voters[2]
        {
            return Err(Error::InvalidCommittee);
        }
        Ok(Self { producer, voters })
    }
    /// Returns the fixed voting membership.
    #[must_use]
    pub const fn voters(&self) -> &[ParticipantId; 3] {
        &self.voters
    }
    /// Returns the non-voting producer.
    #[must_use]
    pub const fn producer(&self) -> ParticipantId {
        self.producer
    }
    /// Returns the approved threshold, independent of connectivity.
    #[must_use]
    pub const fn required_votes(&self) -> usize {
        2
    }
    /// Counts distinct member approvals already authenticated for one exact block.
    ///
    /// This helper does not verify signatures, chain, height, round, or BlockId.
    /// A caller must perform all of those checks before counting any approval.
    pub fn has_quorum(&self, authenticated_approvals: &[ParticipantId]) -> Result<bool> {
        let mut seen = [false; 3];
        for id in authenticated_approvals {
            let index = self
                .voters
                .iter()
                .position(|voter| voter == id)
                .ok_or(Error::NonMember)?;
            if seen[index] {
                return Err(Error::DuplicateVote);
            }
            seen[index] = true;
        }
        Ok(seen.into_iter().filter(|present| *present).count() >= self.required_votes())
    }
}
