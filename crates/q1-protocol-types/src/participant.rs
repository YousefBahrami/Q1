//! Approved participant identity, record, and structural participant-set codecs.

use crate::{
    Error, Result, codec,
    types::{ParticipantCount, ParticipantId, ParticipantRecordHash},
};
use q1_primitives::{
    Ed25519PublicKey, Height,
    cbor::{self, Value},
    domain::Domain,
    sha256,
};
use std::collections::BTreeSet;

fn key_value(key: Option<Ed25519PublicKey>) -> Value {
    key.map_or(Value::Null, |key| Value::Bytes(key.as_bytes().to_vec()))
}
fn parse_key(value: &Value) -> Result<Option<Ed25519PublicKey>> {
    if *value == Value::Null {
        return Ok(None);
    }
    Ok(Some(Ed25519PublicKey::from_bytes(codec::bytes(value)?)?))
}

/// The exact three-field participant identity body; key presence determines roles.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParticipantIdentityBodyV1 {
    producer_public_key: Option<Ed25519PublicKey>,
    validator_public_key: Option<Ed25519PublicKey>,
}
impl ParticipantIdentityBodyV1 {
    /// Requires at least one validated key and distinct keys when both roles exist.
    pub fn new(
        producer_public_key: Option<Ed25519PublicKey>,
        validator_public_key: Option<Ed25519PublicKey>,
    ) -> Result<Self> {
        if producer_public_key.is_none() && validator_public_key.is_none() {
            return Err(Error::InvalidField("participant requires a role key"));
        }
        if producer_public_key.is_some() && producer_public_key == validator_public_key {
            return Err(Error::InvalidField("participant role keys must differ"));
        }
        Ok(Self {
            producer_public_key,
            validator_public_key,
        })
    }
    /// Returns the producer role key, when present.
    #[must_use]
    pub const fn producer_public_key(&self) -> Option<Ed25519PublicKey> {
        self.producer_public_key
    }
    /// Returns the validator role key, when present.
    #[must_use]
    pub const fn validator_public_key(&self) -> Option<Ed25519PublicKey> {
        self.validator_public_key
    }
    /// Derives identity from role keys using registered PARTICIPANT_ID domain 0x0013.
    pub fn participant_id(&self) -> Result<ParticipantId> {
        Ok(ParticipantId::from_bytes(
            sha256::hash_domain(Domain::ParticipantId, &self.encode_canonical()?)?.into_bytes(),
        ))
    }
    /// Encodes the fixed three-field identity body, including explicit nulls.
    pub fn encode_canonical(&self) -> Result<Vec<u8>> {
        Ok(cbor::encode(&Value::Array(vec![
            Value::Unsigned(1),
            key_value(self.producer_public_key),
            key_value(self.validator_public_key),
        ]))?)
    }
    /// Parses the canonical body and validates both optional key slots.
    pub fn decode_canonical(bytes: &[u8]) -> Result<Self> {
        let [version, producer, validator] = codec::array(cbor::decode(bytes)?)?;
        codec::version(&version)?;
        Self::new(parse_key(&producer)?, parse_key(&validator)?)
    }
}

/// Exact six-field participant record with derived identity and half-open activity interval.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParticipantRecordV1 {
    identity: ParticipantIdentityBodyV1,
    participant_id: ParticipantId,
    activation_height: Height,
    deactivation_height: Option<Height>,
}
impl ParticipantRecordV1 {
    /// Derives the ID and validates role keys and the optional exclusive deactivation bound.
    pub fn new(
        producer_public_key: Option<Ed25519PublicKey>,
        validator_public_key: Option<Ed25519PublicKey>,
        activation_height: Height,
        deactivation_height: Option<Height>,
    ) -> Result<Self> {
        if deactivation_height.is_some_and(|height| height <= activation_height) {
            return Err(Error::InvalidField("deactivation must follow activation"));
        }
        let identity = ParticipantIdentityBodyV1::new(producer_public_key, validator_public_key)?;
        Ok(Self {
            participant_id: identity.participant_id()?,
            identity,
            activation_height,
            deactivation_height,
        })
    }
    /// Returns the role-key identity, independent of activation scheduling.
    #[must_use]
    pub const fn participant_id(&self) -> ParticipantId {
        self.participant_id
    }
    /// Returns the producer role key, when present.
    #[must_use]
    pub const fn producer_public_key(&self) -> Option<Ed25519PublicKey> {
        self.identity.producer_public_key()
    }
    /// Returns the validator role key, when present.
    #[must_use]
    pub const fn validator_public_key(&self) -> Option<Ed25519PublicKey> {
        self.identity.validator_public_key()
    }
    /// Returns the inclusive activation bound.
    #[must_use]
    pub const fn activation_height(&self) -> Height {
        self.activation_height
    }
    /// Returns the optional exclusive deactivation bound.
    #[must_use]
    pub const fn deactivation_height(&self) -> Option<Height> {
        self.deactivation_height
    }
    /// Checks only the approved half-open interval; no registry provenance is implied.
    #[must_use]
    pub fn is_active_at(&self, height: Height) -> bool {
        self.activation_height <= height && self.deactivation_height.is_none_or(|end| height < end)
    }
    /// Commits to the complete canonical record, including activation scheduling.
    pub fn record_hash(&self) -> Result<ParticipantRecordHash> {
        Ok(ParticipantRecordHash::from_bytes(
            sha256::hash_domain(Domain::ParticipantRecord, &self.encode_canonical()?)?.into_bytes(),
        ))
    }
    /// Encodes the fixed six-field canonical record.
    pub fn encode_canonical(&self) -> Result<Vec<u8>> {
        Ok(cbor::encode(&self.to_value())?)
    }
    /// Recomputes identity and rejects malformed keys, intervals, or canonical structure.
    pub fn decode_canonical(bytes: &[u8]) -> Result<Self> {
        Self::from_value(cbor::decode(bytes)?)
    }
    pub(crate) fn to_value(&self) -> Value {
        Value::Array(vec![
            Value::Unsigned(1),
            Value::Bytes(self.participant_id.as_bytes().to_vec()),
            key_value(self.producer_public_key()),
            key_value(self.validator_public_key()),
            Value::Unsigned(self.activation_height.get()),
            self.deactivation_height
                .map_or(Value::Null, |height| Value::Unsigned(height.get())),
        ])
    }
    pub(crate) fn from_value(value: Value) -> Result<Self> {
        let [version, id, producer, validator, activation, deactivation] = codec::array(value)?;
        codec::version(&version)?;
        let id = ParticipantId::from_bytes(codec::bytes(&id)?);
        let deactivation = if deactivation == Value::Null {
            None
        } else {
            Some(Height::new(codec::unsigned(&deactivation)?))
        };
        let record = Self::new(
            parse_key(&producer)?,
            parse_key(&validator)?,
            Height::new(codec::unsigned(&activation)?),
            deactivation,
        )?;
        if record.participant_id != id {
            return Err(Error::InvalidField("participant identity mismatch"));
        }
        Ok(record)
    }
}

/// Canonical structural owner of the active participant sequence for a reference height.
///
/// This validates the approved record/ordering/activity rules. It cannot prove
/// that the supplied sequence was derived from finalized state before that
/// height; the future ledger validator must establish that provenance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParticipantSetV1 {
    reference_height: Height,
    participants: Vec<ParticipantRecordV1>,
    count: ParticipantCount,
}
impl ParticipantSetV1 {
    /// Validates a nonempty, already sorted sequence without sorting or deduplicating it.
    pub fn new(reference_height: Height, participants: Vec<ParticipantRecordV1>) -> Result<Self> {
        if participants.len() > cbor::MAX_ARRAY_ENTRIES {
            return Err(Error::ResourceLimit);
        }
        let count = ParticipantCount::new(
            u32::try_from(participants.len()).map_err(|_| Error::ResourceLimit)?,
        )?;
        let mut previous_id = None;
        let mut role_keys = BTreeSet::new();
        for participant in &participants {
            let id = participant.participant_id();
            if previous_id.is_some_and(|previous| previous >= id) {
                return Err(Error::UnsortedOrDuplicate);
            }
            previous_id = Some(id);
            if !participant.is_active_at(reference_height) {
                return Err(Error::InvalidField(
                    "participant inactive at reference height",
                ));
            }
            for key in [
                participant.producer_public_key(),
                participant.validator_public_key(),
            ]
            .into_iter()
            .flatten()
            {
                if !role_keys.insert(*key.as_bytes()) {
                    return Err(Error::UnsortedOrDuplicate);
                }
            }
        }
        Ok(Self {
            reference_height,
            participants,
            count,
        })
    }
    /// Returns the height whose validation the supplied set governs.
    #[must_use]
    pub const fn reference_height(&self) -> Height {
        self.reference_height
    }
    /// Returns the immutable records in canonical order.
    #[must_use]
    pub fn participants(&self) -> &[ParticipantRecordV1] {
        &self.participants
    }
    /// Returns the nonserialized semantic count derived from the record array.
    #[must_use]
    pub const fn count(&self) -> ParticipantCount {
        self.count
    }
    /// Encodes the exact three-field set; no count or set identifier is serialized.
    pub fn encode_canonical(&self) -> Result<Vec<u8>> {
        Ok(cbor::encode(&Value::Array(vec![
            Value::Unsigned(1),
            Value::Unsigned(self.reference_height.get()),
            Value::Array(
                self.participants
                    .iter()
                    .map(ParticipantRecordV1::to_value)
                    .collect(),
            ),
        ]))?)
    }
    /// Rejects noncanonical structure, ordering, duplicate role keys and inactive records.
    pub fn decode_canonical(bytes: &[u8]) -> Result<Self> {
        let [version, height, records] = codec::array(cbor::decode(bytes)?)?;
        codec::version(&version)?;
        let Value::Array(records) = records else {
            return Err(Error::InvalidField("participant array"));
        };
        let records = records
            .into_iter()
            .map(ParticipantRecordV1::from_value)
            .collect::<Result<Vec<_>>>()?;
        Self::new(Height::new(codec::unsigned(&height)?), records)
    }
}
