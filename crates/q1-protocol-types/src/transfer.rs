//! Session 3 transfer bodies, signing payloads and complete-envelope identities.

use crate::{
    Error, Result,
    address::AddressEnvelope,
    codec,
    types::{ChainId, FeeLimit, Nonce, TransferId},
};
use q1_primitives::{
    Address, Amount, Ed25519PrivateKey, Ed25519PublicKey, Ed25519Signature, Height, Network,
    cbor::{self, Value},
    domain::{self, Domain},
    ed25519, sha256,
};

/// Input fields for the approved nine-position transfer body.
///
/// The schema version is fixed by the type and is not caller-selectable.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferFields {
    /// Signed network-instance identifier.
    pub chain_id: ChainId,
    /// Sole source of sender identity.
    pub sender_public_key: Ed25519PublicKey,
    /// Binary recipient envelope without HRP.
    pub recipient_address: AddressEnvelope,
    /// Requested transfer amount; zero policy belongs to execution.
    pub amount: Amount,
    /// Signed fee ceiling; no fee formula is implied.
    pub fee_limit: FeeLimit,
    /// Signed replay counter; account-state validation is separate.
    pub nonce: Nonce,
    /// Inclusive lower validity height.
    pub valid_from_height: Height,
    /// Inclusive upper validity height; no infinite sentinel.
    pub valid_until_height: Height,
}

/// An immutable structurally valid transfer body, not an executed transaction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferBodyV1(TransferFields);

/// A structurally valid signed envelope; call `verify` to verify its signature.
///
/// Decoding deliberately preserves invalid signature bytes so parsing cannot
/// masquerade as account-state, fee, replay, or admission validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignedTransferV1 {
    body: TransferBodyV1,
    signature: Ed25519Signature,
}

impl TransferBodyV1 {
    /// Validates the approved structural height-window constraint.
    pub fn new(fields: TransferFields) -> Result<Self> {
        if fields.valid_from_height > fields.valid_until_height {
            return Err(Error::InvalidField("validity_window"));
        }
        Ok(Self(fields))
    }

    /// Returns the immutable signed fields.
    #[must_use]
    pub const fn fields(&self) -> &TransferFields {
        &self.0
    }

    /// Derives the sender address in an explicit display network.
    pub fn sender_address(&self, network: Network) -> Result<Address> {
        Ok(Address::from_public_key(network, self.0.sender_public_key)?)
    }

    /// Encodes exactly the approved nine fields.
    pub fn encode_canonical(&self) -> Result<Vec<u8>> {
        Ok(cbor::encode(&self.to_value())?)
    }

    /// Decodes a canonical body without inventing execution rules.
    pub fn decode_canonical(bytes: &[u8]) -> Result<Self> {
        Self::from_value(cbor::decode(bytes)?)
    }

    /// Frames the canonical body under TRANSACTION_SIGNING.
    pub fn signing_payload(&self) -> Result<Vec<u8>> {
        Ok(domain::frame(
            Domain::TransactionSigning,
            &self.encode_canonical()?,
        )?)
    }

    pub(crate) fn to_value(&self) -> Value {
        let fields = &self.0;
        Value::Array(vec![
            Value::Unsigned(1),
            Value::Bytes(fields.chain_id.as_bytes().to_vec()),
            Value::Bytes(fields.sender_public_key.as_bytes().to_vec()),
            Value::Bytes(fields.recipient_address.as_bytes().to_vec()),
            Value::Bytes(fields.amount.to_be_bytes().to_vec()),
            Value::Bytes(fields.fee_limit.to_be_bytes().to_vec()),
            Value::Unsigned(fields.nonce.get()),
            Value::Unsigned(fields.valid_from_height.get()),
            Value::Unsigned(fields.valid_until_height.get()),
        ])
    }

    pub(crate) fn from_value(value: Value) -> Result<Self> {
        let [
            version,
            chain,
            sender,
            recipient,
            amount,
            fee,
            nonce,
            from,
            until,
        ] = codec::array(value)?;
        codec::version(&version)?;
        Self::new(TransferFields {
            chain_id: ChainId::from_bytes(codec::bytes(&chain)?),
            sender_public_key: Ed25519PublicKey::from_bytes(codec::bytes(&sender)?)?,
            recipient_address: AddressEnvelope::from_bytes(codec::bytes(&recipient)?)?,
            amount: Amount::from_be_bytes(codec::bytes(&amount)?),
            fee_limit: FeeLimit::from_be_bytes(codec::bytes(&fee)?),
            nonce: Nonce::new(codec::unsigned(&nonce)?),
            valid_from_height: Height::new(codec::unsigned(&from)?),
            valid_until_height: Height::new(codec::unsigned(&until)?),
        })
    }
}

impl SignedTransferV1 {
    /// Signs the exact body, rejecting a private key unrelated to its sender.
    pub fn sign(body: TransferBodyV1, key: &Ed25519PrivateKey) -> Result<Self> {
        if ed25519::public_key(key)? != body.0.sender_public_key {
            return Err(Error::KeyMismatch);
        }
        let signature =
            ed25519::sign_domain(key, Domain::TransactionSigning, &body.encode_canonical()?)?;
        Ok(Self { body, signature })
    }

    /// Attaches an externally produced signature after strict verification.
    pub fn from_signature(body: TransferBodyV1, signature: Ed25519Signature) -> Result<Self> {
        let transfer = Self { body, signature };
        transfer.verify()?;
        Ok(transfer)
    }

    /// Returns the signed body.
    #[must_use]
    pub const fn body(&self) -> &TransferBodyV1 {
        &self.body
    }

    /// Returns the exact signature included in TransferId.
    #[must_use]
    pub const fn signature(&self) -> Ed25519Signature {
        self.signature
    }

    /// Verifies the approved strict Ed25519 signature; does not execute anything.
    pub fn verify(&self) -> Result<()> {
        Ok(ed25519::verify_domain_strict(
            self.body.0.sender_public_key,
            Domain::TransactionSigning,
            &self.body.encode_canonical()?,
            self.signature,
        )?)
    }

    /// Checks chain binding as well as the signature; replay/fees remain separate.
    pub fn verify_for_chain(&self, expected: ChainId) -> Result<()> {
        if self.body.0.chain_id != expected {
            return Err(Error::WrongChain);
        }
        self.verify()
    }

    /// Hashes the entire canonical signed envelope under TRANSACTION_ID.
    pub fn id(&self) -> Result<TransferId> {
        Ok(TransferId::from_bytes(
            sha256::hash_domain(Domain::TransactionId, &self.encode_canonical()?)?.into_bytes(),
        ))
    }

    /// Encodes the three-field signed envelope without a self-ID.
    pub fn encode_canonical(&self) -> Result<Vec<u8>> {
        Ok(cbor::encode(&self.to_value())?)
    }

    /// Parses structural bytes; callers must separately verify signature and state.
    pub fn decode_canonical(bytes: &[u8]) -> Result<Self> {
        Self::from_value(cbor::decode(bytes)?)
    }

    pub(crate) fn to_value(&self) -> Value {
        Value::Array(vec![
            self.body.to_value(),
            Value::Unsigned(1),
            Value::Bytes(self.signature.as_bytes().to_vec()),
        ])
    }

    pub(crate) fn from_value(value: Value) -> Result<Self> {
        let [body, algorithm, signature] = codec::array(value)?;
        let algorithm = codec::unsigned(&algorithm)?;
        if algorithm != 1 {
            return Err(Error::InvalidField("signature_algorithm"));
        }
        Ok(Self {
            body: TransferBodyV1::from_value(body)?,
            signature: Ed25519Signature::from_bytes(codec::bytes(&signature)?),
        })
    }
}
