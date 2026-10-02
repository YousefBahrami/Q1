//! Explicit LOCALNET_V0 genesis and committed fixed execution policy.
//!
//! Canonical shape: `[1, "LOCALNET_V0", 1, chain_preimage, supply,
//! allocations, participants, producer, voters, policy]`.
//! Allocation rows are `[1, address36, amount16]`. Policy is
//! `[1, fee16=1, issuance=false, burn=false, rewards=false, quorum=2,
//! round=0, delay_engine=0, rotation=false]`.
//! The initial StateRoot is derived after GenesisId, avoiding a hash cycle.

use crate::{Error, LocalnetV0, Result, ledger::Ledger, state::StateSnapshot};
use q1_primitives::{
    Amount, Ed25519PublicKey, Height,
    cbor::{self, Value},
    domain::Domain,
    sha256,
};
use q1_protocol_types::{
    address::AddressEnvelope,
    chain::{ChainIdentityPreimageV1, NetworkClass},
    parent::GenesisId,
    participant::{ParticipantRecordV1, ParticipantSetV1},
    types::{ChainId, ParticipantId},
};

/// Minimal genesis for one fixed producer and three fixed voters on LOCALNET only.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Genesis {
    chain: ChainIdentityPreimageV1,
    chain_id: ChainId,
    id: GenesisId,
    supply: Amount,
    allocations: Vec<(AddressEnvelope, Amount)>,
    participants: Vec<ParticipantRecordV1>,
    producer: ParticipantId,
    voters: [ParticipantId; 3],
}
impl Genesis {
    /// Validates all sorted genesis inputs, role authorization, and exact supply.
    pub fn new(
        chain: ChainIdentityPreimageV1,
        supply: Amount,
        allocations: Vec<(AddressEnvelope, Amount)>,
        participants: Vec<ParticipantRecordV1>,
        producer: ParticipantId,
        voters: [ParticipantId; 3],
    ) -> Result<Self> {
        LocalnetV0::new(chain.network_class())?;
        if allocations.len() > cbor::MAX_ARRAY_ENTRIES
            || allocations.windows(2).any(|pair| pair[0].0 >= pair[1].0)
        {
            return Err(invalid("allocation ordering or format limit"));
        }
        if participants.len() != 4
            || voters.windows(2).any(|pair| pair[0] >= pair[1])
            || voters.contains(&producer)
        {
            return Err(Error::InvalidCommittee);
        }
        ParticipantSetV1::new(Height::ZERO, participants.clone())?;
        if participants.iter().any(|record| {
            record.activation_height() != Height::ZERO || record.deactivation_height().is_some()
        }) {
            return Err(invalid("fixed participant activation"));
        }
        let producer_record = participants
            .iter()
            .find(|record| record.participant_id() == producer)
            .ok_or(Error::InvalidCommittee)?;
        if producer_record.producer_public_key().is_none() {
            return Err(Error::InvalidCommittee);
        }
        for voter in voters {
            let record = participants
                .iter()
                .find(|record| record.participant_id() == voter)
                .ok_or(Error::InvalidCommittee)?;
            if record.validator_public_key().is_none() {
                return Err(Error::InvalidCommittee);
            }
        }
        Ledger::from_allocations(
            LocalnetV0::new(NetworkClass::Localnet)?,
            chain.chain_id()?,
            supply,
            allocations.iter().copied(),
        )?;
        let chain_id = chain.chain_id()?;
        let mut genesis = Self {
            chain,
            chain_id,
            id: GenesisId::from_bytes([0; 32]),
            supply,
            allocations,
            participants,
            producer,
            voters,
        };
        // Enforce the canonical object-size ceiling before accepting a genesis.
        let canonical = genesis.encode_canonical()?;
        genesis.id =
            GenesisId::from_bytes(sha256::hash_domain(Domain::Genesis, &canonical)?.into_bytes());
        Ok(genesis)
    }
    /// Returns the existing network identity derivation.
    #[must_use]
    pub const fn chain_id(&self) -> ChainId {
        self.chain_id
    }
    /// Returns the existing chain identity preimage.
    #[must_use]
    pub const fn chain(&self) -> &ChainIdentityPreimageV1 {
        &self.chain
    }
    /// Returns the declared conserved supply.
    #[must_use]
    pub const fn total_supply(&self) -> Amount {
        self.supply
    }
    /// Returns initial allocations in raw address order.
    #[must_use]
    pub fn allocations(&self) -> &[(AddressEnvelope, Amount)] {
        &self.allocations
    }
    /// Derives the complete local genesis identity under the existing GENESIS domain.
    #[must_use]
    pub const fn id(&self) -> GenesisId {
        self.id
    }
    /// Returns the non-voting fixed producer identity.
    #[must_use]
    pub const fn producer(&self) -> ParticipantId {
        self.producer
    }
    /// Returns the fixed sorted voter identities, independent of connectivity.
    #[must_use]
    pub const fn voters(&self) -> &[ParticipantId; 3] {
        &self.voters
    }
    /// Checks fixed voter membership without consulting connectivity.
    #[must_use]
    pub fn is_voter(&self, id: ParticipantId) -> bool {
        self.voters.contains(&id)
    }
    /// Returns the validated key for the fixed producer.
    #[must_use]
    pub fn producer_public_key(&self) -> Ed25519PublicKey {
        self.participants
            .iter()
            .find(|p| p.participant_id() == self.producer)
            .and_then(ParticipantRecordV1::producer_public_key)
            .expect("validated immutable genesis producer")
    }
    /// Resolves a fixed voter's validated signing key.
    pub fn voter_public_key(&self, id: ParticipantId) -> Result<Ed25519PublicKey> {
        if !self.is_voter(id) {
            return Err(Error::NonMember);
        }
        self.participants
            .iter()
            .find(|p| p.participant_id() == id)
            .and_then(ParticipantRecordV1::validator_public_key)
            .ok_or(Error::InvalidCommittee)
    }
    /// Returns the immutable participant registry in raw ParticipantId order.
    #[must_use]
    pub fn participants(&self) -> &[ParticipantRecordV1] {
        &self.participants
    }
    /// Builds the fixed participant-set owner at the requested height.
    pub fn participant_set(&self, height: Height) -> Result<ParticipantSetV1> {
        Ok(ParticipantSetV1::new(height, self.participants.clone())?)
    }
    /// Derives the initial full state, including GenesisId and initial StateRoot.
    pub fn initial_state(&self) -> Result<StateSnapshot> {
        let ledger = Ledger::from_allocations(
            LocalnetV0::new(NetworkClass::Localnet)?,
            self.chain_id(),
            self.supply,
            self.allocations.iter().copied(),
        )?;
        StateSnapshot::new(self, ledger)
    }
    /// Encodes all initial inputs and the exact immutable LOCALNET policy.
    pub fn encode_canonical(&self) -> Result<Vec<u8>> {
        let allocations = self
            .allocations
            .iter()
            .map(|(address, amount)| {
                Value::Array(vec![
                    Value::Unsigned(1),
                    Value::Bytes(address.as_bytes().to_vec()),
                    Value::Bytes(amount.to_be_bytes().to_vec()),
                ])
            })
            .collect();
        let participants = self
            .participants
            .iter()
            .map(|record| Ok(cbor::decode(&record.encode_canonical()?)?))
            .collect::<Result<Vec<_>>>()?;
        Ok(cbor::encode(&Value::Array(vec![
            Value::Unsigned(1),
            Value::Text("LOCALNET_V0".into()),
            Value::Unsigned(1),
            cbor::decode(&self.chain.encode_canonical()?)?,
            Value::Bytes(self.supply.to_be_bytes().to_vec()),
            Value::Array(allocations),
            Value::Array(participants),
            Value::Bytes(self.producer.as_bytes().to_vec()),
            Value::Array(
                self.voters
                    .iter()
                    .map(|id| Value::Bytes(id.as_bytes().to_vec()))
                    .collect(),
            ),
            policy(),
        ]))?)
    }
    /// Strictly parses the local profile, including its complete fixed policy.
    pub fn decode_canonical(bytes: &[u8]) -> Result<Self> {
        let [
            version,
            profile,
            protocol,
            chain,
            supply,
            allocations,
            participants,
            producer,
            voters,
            actual_policy,
        ] = array(cbor::decode(bytes)?)?;
        version_one(&version)?;
        if profile != Value::Text("LOCALNET_V0".into())
            || protocol != Value::Unsigned(1)
            || actual_policy != policy()
        {
            return Err(invalid("local profile or fixed policy"));
        }
        let chain = ChainIdentityPreimageV1::decode_canonical(&cbor::encode(&chain)?)?;
        let supply = Amount::from_be_bytes(fixed_bytes(&supply)?);
        let allocations = values(allocations)?
            .into_iter()
            .map(|row| {
                let [version, address, amount] = array(row)?;
                version_one(&version)?;
                Ok((
                    AddressEnvelope::from_bytes(fixed_bytes(&address)?)?,
                    Amount::from_be_bytes(fixed_bytes(&amount)?),
                ))
            })
            .collect::<Result<Vec<_>>>()?;
        let participants = values(participants)?
            .into_iter()
            .map(|row| Ok(ParticipantRecordV1::decode_canonical(&cbor::encode(&row)?)?))
            .collect::<Result<Vec<_>>>()?;
        let producer = ParticipantId::from_bytes(fixed_bytes(&producer)?);
        let voters = values(voters)?
            .iter()
            .map(|id| Ok(ParticipantId::from_bytes(fixed_bytes(id)?)))
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .map_err(|_| Error::InvalidCommittee)?;
        Self::new(chain, supply, allocations, participants, producer, voters)
    }
}
fn policy() -> Value {
    Value::Array(vec![
        Value::Unsigned(1),
        Value::Bytes(Amount::new(1).to_be_bytes().to_vec()),
        Value::Bool(false),
        Value::Bool(false),
        Value::Bool(false),
        Value::Unsigned(2),
        Value::Unsigned(0),
        Value::Unsigned(0),
        Value::Bool(false),
    ])
}
// Shared LOCALNET parsing checks preserve strict universal primitive CBOR rules.
pub(crate) fn invalid(field: &'static str) -> Error {
    Error::InvalidGenesis(field)
}
pub(crate) fn array<const N: usize>(value: Value) -> Result<[Value; N]> {
    values(value)?
        .try_into()
        .map_err(|_| invalid("fixed local record length"))
}
pub(crate) fn values(value: Value) -> Result<Vec<Value>> {
    if let Value::Array(values) = value {
        Ok(values)
    } else {
        Err(invalid("local array type"))
    }
}
pub(crate) fn fixed_bytes<const N: usize>(value: &Value) -> Result<[u8; N]> {
    if let Value::Bytes(bytes) = value {
        bytes
            .as_slice()
            .try_into()
            .map_err(|_| invalid("local byte width"))
    } else {
        Err(invalid("local byte string type"))
    }
}
pub(crate) fn uint(value: &Value) -> Result<u64> {
    if let Value::Unsigned(value) = value {
        Ok(*value)
    } else {
        Err(invalid("local unsigned integer type"))
    }
}
pub(crate) fn version_one(value: &Value) -> Result<()> {
    if *value == Value::Unsigned(1) {
        Ok(())
    } else {
        Err(invalid("local schema version"))
    }
}
