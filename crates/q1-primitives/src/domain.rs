//! Versioned Q1 cryptographic domain framing.

use crate::{Error, Result};

/// Fixed framing magic: ASCII `Q1DS`.
pub const MAGIC: [u8; 4] = *b"Q1DS";
/// Approved domain registry framing version.
pub const REGISTRY_VERSION: u16 = 1;
/// Number of bytes preceding a framed payload.
pub const FRAME_HEADER_LENGTH: usize = 16;

/// An approved V1 cryptographic domain.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u16)]
pub enum Domain {
    /// Transaction signing payload.
    TransactionSigning = 0x0001,
    /// Complete transaction identifier.
    TransactionId = 0x0002,
    /// Unsigned block-header producer signature.
    BlockHeaderSigning = 0x0003,
    /// Complete signed block-header identifier.
    BlockId = 0x0004,
    /// Block proposal signing payload.
    ProposalSigning = 0x0005,
    /// Validator attestation signing payload.
    AttestationSigning = 0x0006,
    /// Finalization certificate commitment.
    FinalizationCertificate = 0x0007,
    /// Participant registry record.
    ParticipantRecord = 0x0008,
    /// Merkle leaf.
    MerkleLeaf = 0x0009,
    /// Merkle internal node.
    MerkleInternal = 0x000a,
    /// Delay challenge.
    DelayChallenge = 0x000b,
    /// Delay output or proof commitment.
    DelayOutput = 0x000c,
    /// Genesis object.
    Genesis = 0x000d,
    /// Snapshot manifest.
    Snapshot = 0x000e,
    /// Address/account identifier derivation.
    AddressPayload = 0x000f,
    /// Chain identity preimage approved by DEC-Q1-027 Session 1.
    ChainId = 0x0010,
    /// Complete signed LOCALNET_V0 proposal; no general/public proposal activation.
    LocalnetProposalId = 0x0011,
    /// Participant role-key identity approved by DEC-Q1-027 Session 2.
    ParticipantId = 0x0013,
    /// Complete canonical delay evidence; NONE is restricted by network policy.
    DelayEvidence = 0x0014,
    /// Complete canonical LOCALNET_V0 state, not the inactive Merkle state profile.
    LocalnetState = 0x0015,
}

impl Domain {
    /// Returns the stable unsigned registry identifier.
    #[must_use]
    pub const fn id(self) -> u16 {
        self as u16
    }
}

impl TryFrom<u16> for Domain {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self> {
        match value {
            0x0001 => Ok(Self::TransactionSigning),
            0x0002 => Ok(Self::TransactionId),
            0x0003 => Ok(Self::BlockHeaderSigning),
            0x0004 => Ok(Self::BlockId),
            0x0005 => Ok(Self::ProposalSigning),
            0x0006 => Ok(Self::AttestationSigning),
            0x0007 => Ok(Self::FinalizationCertificate),
            0x0008 => Ok(Self::ParticipantRecord),
            0x0009 => Ok(Self::MerkleLeaf),
            0x000a => Ok(Self::MerkleInternal),
            0x000b => Ok(Self::DelayChallenge),
            0x000c => Ok(Self::DelayOutput),
            0x000d => Ok(Self::Genesis),
            0x000e => Ok(Self::Snapshot),
            0x000f => Ok(Self::AddressPayload),
            0x0010 => Ok(Self::ChainId),
            0x0011 => Ok(Self::LocalnetProposalId),
            0x0013 => Ok(Self::ParticipantId),
            0x0014 => Ok(Self::DelayEvidence),
            0x0015 => Ok(Self::LocalnetState),
            _ => Err(Error::DomainUnknown(value)),
        }
    }
}

/// Builds `Q1DS || version:u16be || domain:u16be || length:u64be || payload`.
///
/// The explicit payload length prevents concatenation ambiguity, and the
/// prefix/version prevent cross-protocol or future-framing reinterpretation.
pub fn frame(domain: Domain, payload: &[u8]) -> Result<Vec<u8>> {
    let length = u64::try_from(payload.len()).map_err(|_| Error::ConversionOutOfRange)?;
    let capacity = FRAME_HEADER_LENGTH
        .checked_add(payload.len())
        .ok_or(Error::Overflow)?;
    let mut output = Vec::with_capacity(capacity);
    output.extend_from_slice(&MAGIC);
    output.extend_from_slice(&REGISTRY_VERSION.to_be_bytes());
    output.extend_from_slice(&domain.id().to_be_bytes());
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(payload);
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_has_fixed_header() {
        let framed = frame(Domain::Genesis, &[]).unwrap();
        assert_eq!(&framed[..8], &[b'Q', b'1', b'D', b'S', 0, 1, 0, 0x0d]);
        assert_eq!(framed.len(), FRAME_HEADER_LENGTH);
    }
}
