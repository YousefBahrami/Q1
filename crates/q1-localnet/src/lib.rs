//! Explicit LOCALNET v0 policy and atomic accounting. Not production consensus.
//!
//! Full-state commitments, signed blocks, fixed two-of-three certificates and
//! durable recovery implement the explicit LOCALNET_V0 human decisions. The pure
//! ledger/tally APIs alone do not establish finality; use the validated chain and
//! persistent store. No public-network policy or final state tree is implied.

pub mod block;
mod codec;
pub mod genesis;
pub mod ledger;
pub mod quorum;
pub mod state;
pub mod store;

use core::fmt;
use q1_protocol_types::chain::NetworkClass;

/// Proof that the caller selected the explicitly permitted LOCALNET v0 mode.
#[derive(Clone, Copy, Debug)]
pub struct LocalnetV0(());

/// Explicit controlled-private TESTNET_FAILOVER_V0 execution capability.
/// This does not activate a public network, delay witness or Mainnet profile.
#[derive(Clone, Copy, Debug)]
pub struct TestnetFailoverV0(());
impl TestnetFailoverV0 {
    /// Accepts only the already registered controlled private-testnet class.
    pub fn new(network: NetworkClass) -> Result<Self> {
        if network != NetworkClass::PrivateTestnet {
            return Err(Error::InvalidGenesis("TESTNET_FAILOVER_V0 network class"));
        }
        Ok(Self(()))
    }
}
impl LocalnetV0 {
    /// Rejects every non-local network; no production or implicit default exists.
    pub fn new(network: NetworkClass) -> Result<Self> {
        if network != NetworkClass::Localnet {
            return Err(Error::NonLocalNetwork);
        }
        Ok(Self(()))
    }
}

/// LOCALNET operation result.
pub type Result<T> = core::result::Result<T, Error>;
/// Machine-readable failures; a failed transition leaves the original state intact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// This policy cannot activate on any other network class.
    NonLocalNetwork,
    /// Primitive validation or checked arithmetic failed.
    Primitive(q1_primitives::Error),
    /// Protocol structure, signature or chain binding failed.
    Protocol(q1_protocol_types::Error),
    /// Duplicate initial allocation address.
    DuplicateAllocation,
    /// Explicit allocations do not equal the declared supply.
    SupplyMismatch,
    /// The next block must follow the current state height exactly.
    WrongHeight,
    /// A transfer has zero amount.
    ZeroAmount,
    /// Self-transfers are excluded from the initial ledger profile.
    SelfTransfer,
    /// The signed fee ceiling cannot pay the approved one-unit fee.
    FeeLimit,
    /// Block height is outside the inclusive signed interval.
    ExpiredOrPremature,
    /// The signed nonce is stale or skips the next required value.
    NonceMismatch,
    /// Sender cannot pay amount plus actual fee.
    InsufficientBalance,
    /// Committee has duplicate members or includes its non-voting producer.
    InvalidCommittee,
    /// An approval claims a voter outside the fixed committee.
    NonMember,
    /// A voter is counted twice.
    DuplicateVote,
    /// A LOCALNET genesis shape, role, or policy invariant was violated.
    InvalidGenesis(&'static str),
    /// A complete local state is malformed or inconsistent.
    InvalidState(&'static str),
    /// A block, proposal, or certificate failed contextual validation.
    InvalidBlock(&'static str),
    /// A voter attempted to reserve a second proposal in the same context.
    Equivocation,
    /// The durable outcome is ambiguous; reopen and recover before continuing.
    StoragePoisoned,
    /// Another process holds the exclusive data-directory lock.
    StorageLocked,
    /// A filesystem operation failed; contains no key material.
    Io(String),
}
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error.to_string())
    }
}
impl From<q1_primitives::Error> for Error {
    fn from(e: q1_primitives::Error) -> Self {
        Self::Primitive(e)
    }
}
impl From<q1_protocol_types::Error> for Error {
    fn from(e: q1_protocol_types::Error) -> Self {
        Self::Protocol(e)
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Primitive(e) => e.fmt(f),
            Self::Protocol(e) => e.fmt(f),
            other => write!(f, "LOCALNET v0: {other:?}"),
        }
    }
}
impl std::error::Error for Error {}
