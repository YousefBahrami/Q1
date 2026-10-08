//! LOCALNET full-state canonical commitment, without partial Merkle proofs.
//!
//! Snapshot bytes are exactly `[1, ChainId, GenesisId, Height, total_supply16,
//! reward_pool16, accounts, participant_registry]`. Account rows are
//! `[1, address36, balance16, nonce, account_version=1]`, sorted by raw address.
//! The entire canonical snapshot is hashed under LOCALNET_STATE, never under
//! the inactive universal state-tree profile.

use crate::{
    Error, Result,
    genesis::{Genesis, array, fixed_bytes, uint, values, version_one},
    ledger::Ledger,
};
use q1_primitives::{
    Amount, Height,
    cbor::{self, Value},
    domain::Domain,
    sha256,
};
use q1_protocol_types::{
    address::AddressEnvelope,
    block_body::BlockBodyV1,
    parent::GenesisId,
    participant::ParticipantRecordV1,
    types::{ChainId, Nonce},
};

/// Full LOCALNET state commitment; not a universal Q1 state-tree commitment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateRoot([u8; 32]);
impl StateRoot {
    /// Wraps a fixed-width state commitment; does not prove snapshot validity.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
    /// Returns immutable commitment bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// Validated full snapshot bound to one immutable LOCALNET genesis.
///
/// Decoding checks canonical structure and conserved supply, not historical
/// authorization. Recovery must also authenticate its committed block and
/// certificate or replay history before trusting an arbitrary external snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateSnapshot {
    genesis_id: GenesisId,
    ledger: Ledger,
    participant_registry: Vec<ParticipantRecordV1>,
}
impl StateSnapshot {
    /// Creates the full-state commitment for the separately bound private
    /// TESTNET_FAILOVER_V0 genesis. The caller must verify that genesis and
    /// replay certificates; this constructor never confers finality.
    pub fn for_testnet(
        _: crate::TestnetFailoverV0,
        genesis_id: GenesisId,
        ledger: Ledger,
        participant_registry: Vec<ParticipantRecordV1>,
    ) -> Result<Self> {
        if ledger.network() != q1_primitives::Network::PrivateTestnet
            || participant_registry.len() != 5
        {
            return Err(Error::InvalidState("TESTNET_FAILOVER_V0 state context"));
        }
        q1_protocol_types::participant::ParticipantSetV1::new(
            Height::ZERO,
            participant_registry.clone(),
        )?;
        ledger.check_supply()?;
        Ok(Self {
            genesis_id,
            ledger,
            participant_registry,
        })
    }
    /// Binds conserved ledger state to its chain, genesis supply and fixed registry.
    pub fn new(genesis: &Genesis, ledger: Ledger) -> Result<Self> {
        if ledger.chain_id() != genesis.chain_id()
            || ledger.total_supply() != genesis.total_supply()
        {
            return Err(Error::InvalidState("chain or genesis supply mismatch"));
        }
        ledger.check_supply()?;
        let snapshot = Self {
            genesis_id: genesis.id(),
            ledger,
            participant_registry: genesis.participants().to_vec(),
        };
        snapshot.encode_canonical()?;
        Ok(snapshot)
    }
    /// Returns immutable accounting state, including nonce, fee pool and height.
    #[must_use]
    pub const fn ledger(&self) -> &Ledger {
        &self.ledger
    }
    /// Returns the immutable genesis commitment.
    #[must_use]
    pub const fn genesis_id(&self) -> GenesisId {
        self.genesis_id
    }
    /// Hashes every canonical consensus state field with the LOCALNET state domain.
    pub fn root(&self) -> Result<StateRoot> {
        Ok(StateRoot::from_bytes(
            sha256::hash_domain(Domain::LocalnetState, &self.encode_canonical()?)?.into_bytes(),
        ))
    }
    /// Encodes all accounts, retaining zero balances and their replay state.
    pub fn encode_canonical(&self) -> Result<Vec<u8>> {
        let accounts = self
            .ledger
            .accounts()
            .map(|(address, account)| {
                Value::Array(vec![
                    Value::Unsigned(1),
                    Value::Bytes(address.as_bytes().to_vec()),
                    Value::Bytes(account.balance().to_be_bytes().to_vec()),
                    Value::Unsigned(account.nonce().get()),
                    Value::Unsigned(1),
                ])
            })
            .collect();
        let registry = self
            .participant_registry
            .iter()
            .map(|record| Ok(cbor::decode(&record.encode_canonical()?)?))
            .collect::<Result<Vec<_>>>()?;
        Ok(cbor::encode(&Value::Array(vec![
            Value::Unsigned(1),
            Value::Bytes(self.ledger.chain_id().as_bytes().to_vec()),
            Value::Bytes(self.genesis_id.as_bytes().to_vec()),
            Value::Unsigned(self.ledger.height().get()),
            Value::Bytes(self.ledger.total_supply().to_be_bytes().to_vec()),
            Value::Bytes(self.ledger.reward_pool().to_be_bytes().to_vec()),
            Value::Array(accounts),
            Value::Array(registry),
        ]))?)
    }
    /// Decodes strictly, rejecting unsorted accounts, altered registry and broken conservation.
    pub fn decode_canonical(bytes: &[u8], genesis: &Genesis) -> Result<Self> {
        let [version, chain, id, height, supply, pool, accounts, registry] =
            array(cbor::decode(bytes)?)?;
        version_one(&version)?;
        let chain = ChainId::from_bytes(fixed_bytes(&chain)?);
        let id = GenesisId::from_bytes(fixed_bytes(&id)?);
        if chain != genesis.chain_id() || id != genesis.id() {
            return Err(Error::InvalidState("chain or genesis identity mismatch"));
        }
        let registry = values(registry)?
            .iter()
            .map(|record| {
                Ok(ParticipantRecordV1::decode_canonical(&cbor::encode(
                    record,
                )?)?)
            })
            .collect::<Result<Vec<_>>>()?;
        if registry != genesis.participants() {
            return Err(Error::InvalidState("fixed participant registry mismatch"));
        }
        let mut decoded = Vec::new();
        for row in values(accounts)? {
            let [version, address, balance, nonce, account_version] = array(row)?;
            version_one(&version)?;
            version_one(&account_version)?;
            let address = AddressEnvelope::from_bytes(fixed_bytes(&address)?)?;
            if decoded
                .last()
                .is_some_and(|(previous, _, _)| *previous >= address)
            {
                return Err(Error::InvalidState("account ordering or duplicate address"));
            }
            decoded.push((
                address,
                Amount::from_be_bytes(fixed_bytes(&balance)?),
                Nonce::new(uint(&nonce)?),
            ));
        }
        let ledger = Ledger::restore(
            chain,
            Height::new(uint(&height)?),
            Amount::from_be_bytes(fixed_bytes(&supply)?),
            Amount::from_be_bytes(fixed_bytes(&pool)?),
            decoded,
        )?;
        Self::new(genesis, ledger)
    }
    /// Executes a whole ordered body on a clone; any error preserves this snapshot.
    ///
    /// This computes a candidate state only. Finality and durable commit require
    /// the proposal and certificate checks performed by the block layer.
    pub fn apply_body(&self, height: Height, body: &BlockBodyV1) -> Result<Self> {
        let mut next = self.clone();
        next.ledger.apply_block(height, body)?;
        next.encode_canonical()?;
        Ok(next)
    }
}
