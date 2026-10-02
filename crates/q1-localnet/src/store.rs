//! Atomic LOCALNET archives with authenticated replay and a durable vote journal.
//!
//! This deliberately rewrites a bounded full archive. It is not an optimized
//! database. An OS file lock protects one data directory throughout its lifetime.
//! Restore a failed store with `open`; never replace a corrupt archive with genesis.

use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use q1_primitives::cbor::{self, Value};
use q1_protocol_types::types::ParticipantId;

use crate::{
    Error, Result,
    block::{CertifiedBlock, Chain, Proposal},
    genesis::Genesis,
};

const ARCHIVE: &str = "state.cbor";
const PENDING: &str = "state.pending";
type VoteKey = (u64, u32, ParticipantId);
type VoteJournal = BTreeMap<VoteKey, Vec<u8>>;

/// A chain whose commits and signing reservations become durable before success.
///
/// Reading the in-memory chain does not bypass the persistence API: mutations are
/// exposed only through `commit` and `reserve_vote`. A post-rename I/O failure
/// poisons this instance until it is dropped and the archive is recovered.
pub struct PersistentChain {
    directory: PathBuf,
    _lock: File,
    chain: Chain,
    votes: VoteJournal,
    poisoned: bool,
}

impl PersistentChain {
    /// Initializes a new archive, refusing to overwrite an existing archive.
    pub fn create(directory: impl AsRef<Path>, genesis: &Genesis) -> Result<Self> {
        let directory = directory.as_ref().to_path_buf();
        fs::create_dir_all(&directory).map_err(io_error)?;
        let lock = lock_directory(&directory)?;
        if directory.join(ARCHIVE).try_exists().map_err(io_error)? {
            return Err(Error::InvalidBlock("persistent archive already exists"));
        }
        let mut store = Self {
            directory,
            _lock: lock,
            chain: Chain::new(genesis.clone())?,
            votes: BTreeMap::new(),
            poisoned: false,
        };
        let bytes = encode_archive(&store.chain, &store.votes)?;
        store.persist(&bytes)?;
        Ok(store)
    }

    /// Replays every certified block and verifies the exact saved state and tip.
    ///
    /// A leftover pre-rename pending file is not a committed archive and is
    /// ignored. Missing, malformed, wrong-genesis or inconsistent archives fail.
    pub fn open(directory: impl AsRef<Path>, genesis: &Genesis) -> Result<Self> {
        let directory = directory.as_ref().to_path_buf();
        let lock = lock_directory(&directory)?;
        let file = File::open(directory.join(ARCHIVE)).map_err(io_error)?;
        if file.metadata().map_err(io_error)?.len() > cbor::MAX_OBJECT_SIZE as u64 {
            return Err(q1_primitives::Error::CborLimitExceeded.into());
        }
        let mut bytes = Vec::new();
        file.take((cbor::MAX_OBJECT_SIZE as u64) + 1)
            .read_to_end(&mut bytes)
            .map_err(io_error)?;
        let (chain, votes) = decode_archive(&bytes, genesis)?;
        Ok(Self {
            directory,
            _lock: lock,
            chain,
            votes,
            poisoned: false,
        })
    }

    /// Returns the authenticated in-memory chain without exposing mutable access.
    #[must_use]
    pub const fn chain(&self) -> &Chain {
        &self.chain
    }

    /// Validates an entire certified block and atomically saves its resulting state.
    ///
    /// Failed validation or a pre-rename write failure leaves the live chain and
    /// committed archive unchanged. Ambiguous post-rename errors fail closed.
    pub fn commit(&mut self, block: &CertifiedBlock) -> Result<()> {
        self.ensure_healthy()?;
        let mut candidate = self.chain.clone();
        candidate.commit(block)?;
        let bytes = encode_archive(&candidate, &self.votes)?;
        self.persist(&bytes)?;
        self.chain = candidate;
        Ok(())
    }

    /// Durably reserves one proposal per voter, height and round before signing.
    ///
    /// The caller must invoke this successfully *before* producing a signature.
    /// Repeating the same validated reservation is harmless. A different proposal
    /// for the same key fails even after process restart. Disconnects do not alter
    /// the fixed genesis voter set.
    pub fn reserve_vote(&mut self, voter: ParticipantId, proposal: &Proposal) -> Result<()> {
        self.ensure_healthy()?;
        if !self.chain.genesis().is_voter(voter) {
            return Err(Error::NonMember);
        }
        self.chain.validate_proposal(proposal)?;
        let key = (proposal.height().get(), proposal.round().get(), voter);
        let encoded = proposal.encode_canonical()?;
        if let Some(previous) = self.votes.get(&key) {
            if Proposal::decode_canonical(previous)?.id()? != proposal.id()? {
                return Err(Error::Equivocation);
            }
            return Ok(());
        }
        let mut votes = self.votes.clone();
        votes.insert(key, encoded);
        let bytes = encode_archive(&self.chain, &votes)?;
        self.persist(&bytes)?;
        self.votes = votes;
        Ok(())
    }

    fn ensure_healthy(&self) -> Result<()> {
        if self.poisoned {
            Err(Error::StoragePoisoned)
        } else {
            Ok(())
        }
    }

    fn persist(&mut self, bytes: &[u8]) -> Result<()> {
        self.ensure_healthy()?;
        let pending = self.directory.join(PENDING);
        // Only the lock owner may discard a stale, never-committed temporary file.
        match fs::remove_file(&pending) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(io_error(error)),
        }
        let before_rename = (|| -> std::io::Result<()> {
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&pending)?;
            file.write_all(bytes)?;
            file.sync_all()?;
            fs::rename(&pending, self.directory.join(ARCHIVE))?;
            Ok(())
        })();
        if let Err(error) = before_rename {
            let _ = fs::remove_file(&pending);
            return Err(io_error(error));
        }
        // The rename may now be visible even if durability confirmation fails.
        if let Err(error) = File::open(&self.directory).and_then(|directory| directory.sync_all()) {
            self.poisoned = true;
            return Err(io_error(error));
        }
        Ok(())
    }
}

fn io_error(error: std::io::Error) -> Error {
    Error::Io(error.to_string())
}

fn lock_directory(directory: &Path) -> Result<File> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(directory.join("LOCK"))
        .map_err(io_error)?;
    match file.try_lock() {
        Ok(()) => Ok(file),
        Err(std::fs::TryLockError::WouldBlock) => Err(Error::StorageLocked),
        Err(std::fs::TryLockError::Error(error)) => Err(io_error(error)),
    }
}

fn encode_archive(chain: &Chain, votes: &VoteJournal) -> Result<Vec<u8>> {
    let history = chain
        .history()
        .iter()
        .map(|block| block.encode_canonical().map(Value::Bytes))
        .collect::<Result<Vec<_>>>()?;
    let votes = votes
        .iter()
        .map(|((_, _, voter), proposal)| {
            Value::Array(vec![
                Value::Bytes(voter.as_bytes().to_vec()),
                Value::Bytes(proposal.clone()),
            ])
        })
        .collect();
    Ok(cbor::encode(&Value::Array(vec![
        Value::Unsigned(1),
        Value::Bytes(chain.genesis().encode_canonical()?),
        Value::Array(history),
        Value::Bytes(chain.state().encode_canonical()?),
        Value::Unsigned(chain.state().ledger().height().get()),
        chain
            .tip()
            .map_or(Value::Null, |id| Value::Bytes(id.as_bytes().to_vec())),
        Value::Array(votes),
    ]))?)
}

fn fields<const N: usize>(value: Value) -> Result<[Value; N]> {
    let Value::Array(values) = value else {
        return Err(Error::InvalidBlock("archive array"));
    };
    values
        .try_into()
        .map_err(|_| Error::InvalidBlock("archive field count"))
}

fn raw_bytes(value: Value) -> Result<Vec<u8>> {
    match value {
        Value::Bytes(bytes) => Ok(bytes),
        _ => Err(Error::InvalidBlock("archive byte string")),
    }
}

fn decode_archive(bytes: &[u8], genesis: &Genesis) -> Result<(Chain, VoteJournal)> {
    let [
        version,
        saved_genesis,
        history,
        snapshot,
        height,
        tip,
        votes,
    ] = fields(cbor::decode(bytes)?)?;
    if version != Value::Unsigned(1) {
        return Err(Error::InvalidBlock("archive version"));
    }
    if raw_bytes(saved_genesis)? != genesis.encode_canonical()? {
        return Err(Error::InvalidBlock("archive genesis mismatch"));
    }
    let Value::Array(history) = history else {
        return Err(Error::InvalidBlock("archive history"));
    };
    let Value::Array(votes) = votes else {
        return Err(Error::InvalidBlock("archive vote journal"));
    };
    let mut journal = BTreeMap::new();
    let mut previous_key = None;
    for vote in votes {
        let [voter, proposal] = fields(vote)?;
        let voter = ParticipantId::from_bytes(
            raw_bytes(voter)?
                .try_into()
                .map_err(|_| Error::InvalidBlock("archive voter width"))?,
        );
        if !genesis.is_voter(voter) {
            return Err(Error::NonMember);
        }
        let encoded = raw_bytes(proposal)?;
        let proposal = Proposal::decode_canonical(&encoded)?;
        let key = (proposal.height().get(), proposal.round().get(), voter);
        if previous_key.is_some_and(|previous| previous >= key) {
            return Err(Error::InvalidBlock("archive journal order or duplicate"));
        }
        previous_key = Some(key);
        journal.insert(key, encoded);
    }
    let mut chain = Chain::new(genesis.clone())?;
    for block in history {
        validate_reservations(&chain, &journal)?;
        chain.commit(&CertifiedBlock::decode_canonical(&raw_bytes(block)?)?)?;
    }
    validate_reservations(&chain, &journal)?;
    let next_height = chain.state().ledger().height().get().checked_add(1);
    if journal
        .keys()
        .any(|(height, _, _)| *height == 0 || next_height.is_some_and(|next| *height > next))
    {
        return Err(Error::InvalidBlock("archive vote height"));
    }
    if raw_bytes(snapshot)? != chain.state().encode_canonical()? {
        return Err(Error::InvalidBlock(
            "archive snapshot disagrees with replay",
        ));
    }
    if height != Value::Unsigned(chain.state().ledger().height().get()) {
        return Err(Error::InvalidBlock("archive height disagrees with replay"));
    }
    let expected_tip = chain
        .tip()
        .map_or(Value::Null, |id| Value::Bytes(id.as_bytes().to_vec()));
    if tip != expected_tip {
        return Err(Error::InvalidBlock("archive tip disagrees with replay"));
    }
    Ok((chain, journal))
}

fn validate_reservations(chain: &Chain, votes: &VoteJournal) -> Result<()> {
    let Some(next_height) = chain.state().ledger().height().get().checked_add(1) else {
        return Ok(());
    };
    for ((height, _, _), bytes) in votes {
        if *height == next_height {
            chain.validate_proposal(&Proposal::decode_canonical(bytes)?)?;
        }
    }
    Ok(())
}
