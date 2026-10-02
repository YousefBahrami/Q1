//! Signed LOCALNET proposals, exact-proposal votes and atomic certified commits.
//!
//! The approved eleven-field Q1 header is retained. Local proposal/vote wrappers
//! are distinct from future public schemas; every contextual check binds genesis.

use crate::{
    Error, Result, codec,
    genesis::Genesis,
    state::{StateRoot, StateSnapshot},
};
use q1_primitives::{
    Ed25519PrivateKey, Ed25519Signature, Height, RoundNumber,
    cbor::{self, Value},
    domain::Domain,
    ed25519, sha256,
};
use q1_protocol_types::{
    block_body::BlockBodyV1,
    chain::NetworkClass,
    delay::{DelayEvidenceHash, LocalnetNoneEvidence},
    merkle::{ParticipantRoot, TransactionRoot},
    parent::{BlockId, GenesisId, ParentReferenceV1},
    types::{ChainId, ParticipantId},
};

/// Identifier of a complete signed LOCALNET_V0 proposal, distinct from BlockId.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ProposalId([u8; 32]);
impl ProposalId {
    /// Wraps a claimed ID; verification recomputes the complete signed proposal.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
    /// Returns the identifier bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// Approved eleven-field header body with claimed typed commitments.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HeaderBody {
    chain_id: ChainId,
    height: Height,
    round: RoundNumber,
    parent: ParentReferenceV1,
    producer: ParticipantId,
    transaction_root: TransactionRoot,
    participant_root: ParticipantRoot,
    state_root: StateRoot,
    delay_hash: DelayEvidenceHash,
}
impl HeaderBody {
    /// Returns the height committed by the producer.
    #[must_use]
    pub const fn height(&self) -> Height {
        self.height
    }
    /// Returns the round, which the LOCALNET profile requires to equal zero.
    #[must_use]
    pub const fn round(&self) -> RoundNumber {
        self.round
    }
    /// Returns the resulting-state commitment claimed by the producer.
    #[must_use]
    pub const fn state_root(&self) -> StateRoot {
        self.state_root
    }
    /// Returns the transaction commitment claimed by the producer.
    #[must_use]
    pub const fn transaction_root(&self) -> TransactionRoot {
        self.transaction_root
    }
    /// Returns the explicit genesis/block parent reference.
    #[must_use]
    pub const fn parent(&self) -> ParentReferenceV1 {
        self.parent
    }
    fn to_value(&self) -> Result<Value> {
        Ok(Value::Array(vec![
            Value::Unsigned(1),
            Value::Unsigned(1),
            Value::Bytes(self.chain_id.as_bytes().to_vec()),
            Value::Unsigned(self.height.get()),
            Value::Unsigned(u64::from(self.round.get())),
            codec::value(&self.parent.encode_canonical()?)?,
            Value::Bytes(self.producer.as_bytes().to_vec()),
            Value::Bytes(self.transaction_root.as_bytes().to_vec()),
            Value::Bytes(self.participant_root.as_bytes().to_vec()),
            Value::Bytes(self.state_root.as_bytes().to_vec()),
            Value::Bytes(self.delay_hash.as_bytes().to_vec()),
        ]))
    }
    fn from_value(value: Value) -> Result<Self> {
        let [
            version,
            protocol,
            chain,
            height,
            round,
            parent,
            producer,
            tx,
            participants,
            state,
            delay,
        ] = codec::array(value)?;
        codec::version(&version)?;
        codec::version(&protocol)?;
        let height = Height::new(codec::unsigned(&height)?);
        if height.get() == 0 {
            return Err(Error::InvalidBlock("signed height zero"));
        }
        let round = RoundNumber::new(
            u32::try_from(codec::unsigned(&round)?)
                .map_err(|_| Error::InvalidBlock("round width"))?,
        );
        Ok(Self {
            chain_id: ChainId::from_bytes(codec::bytes(&chain)?),
            height,
            round,
            parent: ParentReferenceV1::decode_canonical(&cbor::encode(&parent)?)?,
            producer: ParticipantId::from_bytes(codec::bytes(&producer)?),
            transaction_root: TransactionRoot::from_claimed_bytes(codec::bytes(&tx)?),
            participant_root: ParticipantRoot::from_claimed_bytes(codec::bytes(&participants)?),
            state_root: StateRoot::from_bytes(codec::bytes(&state)?),
            delay_hash: DelayEvidenceHash::from_claimed_bytes(codec::bytes(&delay)?),
        })
    }
}

/// Complete signed header retaining Q1's approved producer-signature and BlockId rules.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignedBlockHeader {
    body: HeaderBody,
    signature: Ed25519Signature,
}
impl SignedBlockHeader {
    /// Returns the immutable header body.
    #[must_use]
    pub const fn body(&self) -> &HeaderBody {
        &self.body
    }
    /// Computes BlockId from the complete canonical signed header.
    pub fn id(&self) -> Result<BlockId> {
        Ok(BlockId::from_bytes(
            sha256::hash_domain(Domain::BlockId, &self.encode_canonical()?)?.into_bytes(),
        ))
    }
    /// Encodes exact `[header_body, algorithm=1, producer_signature]`.
    pub fn encode_canonical(&self) -> Result<Vec<u8>> {
        Ok(cbor::encode(&self.to_value()?)?)
    }
    /// Parses structure; validity and authority require contextual verification.
    pub fn decode_canonical(bytes: &[u8]) -> Result<Self> {
        Self::from_value(cbor::decode(bytes)?)
    }
    fn to_value(&self) -> Result<Value> {
        Ok(Value::Array(vec![
            self.body.to_value()?,
            Value::Unsigned(1),
            Value::Bytes(self.signature.as_bytes().to_vec()),
        ]))
    }
    fn from_value(value: Value) -> Result<Self> {
        let [body, algorithm, signature] = codec::array(value)?;
        codec::version(&algorithm)?;
        Ok(Self {
            body: HeaderBody::from_value(body)?,
            signature: Ed25519Signature::from_bytes(codec::bytes(&signature)?),
        })
    }
}

/// Signed LOCALNET_V0 proposal containing the exact header, ordered body and evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Proposal {
    genesis_id: GenesisId,
    header: SignedBlockHeader,
    body: BlockBodyV1,
    evidence: LocalnetNoneEvidence,
    signature: Ed25519Signature,
}
impl Proposal {
    /// Returns the immutable signed header.
    #[must_use]
    pub const fn header(&self) -> &SignedBlockHeader {
        &self.header
    }
    /// Returns the exact ordered transfers.
    #[must_use]
    pub const fn body(&self) -> &BlockBodyV1 {
        &self.body
    }
    /// Returns its signed block height.
    #[must_use]
    pub const fn height(&self) -> Height {
        self.header.body.height
    }
    /// Returns its signed round.
    #[must_use]
    pub const fn round(&self) -> RoundNumber {
        self.header.body.round
    }
    /// Computes the ID of every consensus-critical proposal byte including signatures.
    pub fn id(&self) -> Result<ProposalId> {
        Ok(ProposalId::from_bytes(
            sha256::hash_domain(Domain::LocalnetProposalId, &self.encode_canonical()?)?
                .into_bytes(),
        ))
    }
    fn signing_body(&self) -> Result<Value> {
        Ok(Value::Array(vec![
            Value::Unsigned(1),
            Value::Text("LOCALNET_V0".into()),
            Value::Bytes(self.genesis_id.as_bytes().to_vec()),
            self.header.to_value()?,
            codec::value(&self.body.encode_canonical()?)?,
            codec::value(&self.evidence.encode_canonical()?)?,
        ]))
    }
    /// Encodes a local signed envelope, not the unapproved public proposal schema.
    pub fn encode_canonical(&self) -> Result<Vec<u8>> {
        Ok(cbor::encode(&Value::Array(vec![
            self.signing_body()?,
            Value::Unsigned(1),
            Value::Bytes(self.signature.as_bytes().to_vec()),
        ]))?)
    }
    /// Strictly parses the local envelope; does not grant block validity.
    pub fn decode_canonical(bytes: &[u8]) -> Result<Self> {
        let [body, algorithm, signature] = codec::array(cbor::decode(bytes)?)?;
        codec::version(&algorithm)?;
        let [version, profile, genesis, header, body, evidence] = codec::array(body)?;
        codec::version(&version)?;
        if profile != Value::Text("LOCALNET_V0".into()) {
            return Err(Error::InvalidBlock("proposal profile"));
        }
        Ok(Self {
            genesis_id: GenesisId::from_bytes(codec::bytes(&genesis)?),
            header: SignedBlockHeader::from_value(header)?,
            body: BlockBodyV1::decode_canonical(&cbor::encode(&body)?)?,
            evidence: LocalnetNoneEvidence::decode_for_network(
                NetworkClass::Localnet,
                &cbor::encode(&evidence)?,
            )?,
            signature: Ed25519Signature::from_bytes(codec::bytes(&signature)?),
        })
    }
    /// Verifies immutable genesis authorization, both producer signatures and all static commitments.
    /// Execution, height continuity and resulting StateRoot require `Chain::validate_proposal`.
    pub fn verify_authorization(&self, genesis: &Genesis) -> Result<()> {
        let header = &self.header.body;
        if self.genesis_id != genesis.id() || header.chain_id != genesis.chain_id() {
            return Err(Error::InvalidBlock("genesis or chain mismatch"));
        }
        if header.producer != genesis.producer() || header.round != RoundNumber::ZERO {
            return Err(Error::InvalidBlock("producer or round"));
        }
        let public = genesis.producer_public_key();
        ed25519::verify_domain_strict(
            public,
            Domain::BlockHeaderSigning,
            &cbor::encode(&header.to_value()?)?,
            self.header.signature,
        )?;
        ed25519::verify_domain_strict(
            public,
            Domain::ProposalSigning,
            &cbor::encode(&self.signing_body()?)?,
            self.signature,
        )?;
        self.evidence.verify_for_network(NetworkClass::Localnet)?;
        if self.evidence.commitment()? != header.delay_hash {
            return Err(Error::InvalidBlock("delay commitment"));
        }
        if TransactionRoot::from_block_body(&self.body)? != header.transaction_root {
            return Err(Error::InvalidBlock("transaction root"));
        }
        if ParticipantRoot::from_participant_set(&genesis.participant_set(header.height)?)?
            != header.participant_root
        {
            return Err(Error::InvalidBlock("participant root"));
        }
        Ok(())
    }
}

/// A strict Ed25519 vote for one exact local proposal and its signed block identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attestation {
    genesis_id: GenesisId,
    chain_id: ChainId,
    height: Height,
    round: RoundNumber,
    proposal_id: ProposalId,
    block_id: BlockId,
    voter: ParticipantId,
    signature: Ed25519Signature,
}
impl Attestation {
    /// Signs an authorized proposal; node callers MUST durably reserve this vote first.
    /// This pure cryptographic operation alone cannot prevent restart double-signing.
    pub fn sign(
        genesis: &Genesis,
        proposal: &Proposal,
        voter: ParticipantId,
        key: &Ed25519PrivateKey,
    ) -> Result<Self> {
        proposal.verify_authorization(genesis)?;
        let public = genesis.voter_public_key(voter)?;
        if ed25519::public_key(key)? != public {
            return Err(Error::InvalidBlock("voter key mismatch"));
        }
        let mut vote = Self {
            genesis_id: genesis.id(),
            chain_id: genesis.chain_id(),
            height: proposal.height(),
            round: proposal.round(),
            proposal_id: proposal.id()?,
            block_id: proposal.header.id()?,
            voter,
            signature: Ed25519Signature::from_bytes([0; 64]),
        };
        vote.signature = ed25519::sign_domain(
            key,
            Domain::AttestationSigning,
            &cbor::encode(&vote.signing_body())?,
        )?;
        Ok(vote)
    }
    /// Returns the claimed voter; signatures and membership are verified separately.
    #[must_use]
    pub const fn voter(&self) -> ParticipantId {
        self.voter
    }
    fn signing_body(&self) -> Value {
        Value::Array(vec![
            Value::Unsigned(1),
            Value::Text("LOCALNET_V0".into()),
            Value::Bytes(self.genesis_id.as_bytes().to_vec()),
            Value::Bytes(self.chain_id.as_bytes().to_vec()),
            Value::Unsigned(self.height.get()),
            Value::Unsigned(u64::from(self.round.get())),
            Value::Bytes(self.proposal_id.as_bytes().to_vec()),
            Value::Bytes(self.block_id.as_bytes().to_vec()),
            Value::Bytes(self.voter.as_bytes().to_vec()),
            Value::Unsigned(1),
        ])
    }
    /// Verifies the full context, exact proposal, signature and fixed voter membership.
    pub fn verify(&self, genesis: &Genesis, proposal: &Proposal) -> Result<()> {
        if self.genesis_id != genesis.id()
            || self.chain_id != genesis.chain_id()
            || self.height != proposal.height()
            || self.round != proposal.round()
            || self.proposal_id != proposal.id()?
            || self.block_id != proposal.header.id()?
        {
            return Err(Error::InvalidBlock("vote context or proposal mismatch"));
        }
        let key = genesis.voter_public_key(self.voter)?;
        ed25519::verify_domain_strict(
            key,
            Domain::AttestationSigning,
            &cbor::encode(&self.signing_body())?,
            self.signature,
        )?;
        Ok(())
    }
    /// Encodes the local signed vote envelope.
    pub fn encode_canonical(&self) -> Result<Vec<u8>> {
        Ok(cbor::encode(&Value::Array(vec![
            self.signing_body(),
            Value::Unsigned(1),
            Value::Bytes(self.signature.as_bytes().to_vec()),
        ]))?)
    }
    /// Parses exact fields; context/signature checks remain mandatory.
    pub fn decode_canonical(bytes: &[u8]) -> Result<Self> {
        let [body, algorithm, signature] = codec::array(cbor::decode(bytes)?)?;
        codec::version(&algorithm)?;
        let [
            version,
            profile,
            genesis,
            chain,
            height,
            round,
            proposal,
            block,
            voter,
            kind,
        ] = codec::array(body)?;
        codec::version(&version)?;
        codec::version(&kind)?;
        if profile != Value::Text("LOCALNET_V0".into()) {
            return Err(Error::InvalidBlock("vote profile"));
        }
        Ok(Self {
            genesis_id: GenesisId::from_bytes(codec::bytes(&genesis)?),
            chain_id: ChainId::from_bytes(codec::bytes(&chain)?),
            height: Height::new(codec::unsigned(&height)?),
            round: RoundNumber::new(
                u32::try_from(codec::unsigned(&round)?)
                    .map_err(|_| Error::InvalidBlock("round width"))?,
            ),
            proposal_id: ProposalId::from_bytes(codec::bytes(&proposal)?),
            block_id: BlockId::from_bytes(codec::bytes(&block)?),
            voter: ParticipantId::from_bytes(codec::bytes(&voter)?),
            signature: Ed25519Signature::from_bytes(codec::bytes(&signature)?),
        })
    }
}

/// Canonical sorted evidence from two or three distinct fixed authorized voters.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Certificate {
    genesis_id: GenesisId,
    proposal_id: ProposalId,
    attestations: Vec<Attestation>,
}
impl Certificate {
    /// Requires already sorted evidence; never silently deduplicates or sorts votes.
    pub fn new(
        genesis: &Genesis,
        proposal: &Proposal,
        attestations: Vec<Attestation>,
    ) -> Result<Self> {
        let certificate = Self {
            genesis_id: genesis.id(),
            proposal_id: proposal.id()?,
            attestations,
        };
        certificate.verify(genesis, proposal)?;
        Ok(certificate)
    }
    /// Returns the complete immutable signed evidence.
    #[must_use]
    pub fn attestations(&self) -> &[Attestation] {
        &self.attestations
    }
    /// Verifies exact-proposal agreement, fixed membership, distinctness and all signatures.
    pub fn verify(&self, genesis: &Genesis, proposal: &Proposal) -> Result<()> {
        proposal.verify_authorization(genesis)?;
        if self.genesis_id != genesis.id() || self.proposal_id != proposal.id()? {
            return Err(Error::InvalidBlock("certificate context"));
        }
        if !(2..=3).contains(&self.attestations.len()) {
            return Err(Error::InvalidBlock("two-of-three quorum required"));
        }
        let mut previous = None;
        for vote in &self.attestations {
            if previous.is_some_and(|id| id >= vote.voter) {
                return Err(Error::DuplicateVote);
            }
            vote.verify(genesis, proposal)?;
            previous = Some(vote.voter);
        }
        Ok(())
    }
    /// Commits to complete local evidence under the existing certificate domain.
    pub fn commitment(&self) -> Result<[u8; 32]> {
        Ok(
            sha256::hash_domain(Domain::FinalizationCertificate, &self.encode_canonical()?)?
                .into_bytes(),
        )
    }
    /// Encodes the local certificate and complete attestations.
    pub fn encode_canonical(&self) -> Result<Vec<u8>> {
        let votes = self
            .attestations
            .iter()
            .map(|vote| codec::value(&vote.encode_canonical()?))
            .collect::<Result<Vec<_>>>()?;
        Ok(cbor::encode(&Value::Array(vec![
            Value::Unsigned(1),
            Value::Text("LOCALNET_V0".into()),
            Value::Bytes(self.genesis_id.as_bytes().to_vec()),
            Value::Bytes(self.proposal_id.as_bytes().to_vec()),
            Value::Array(votes),
        ]))?)
    }
    /// Decodes structural evidence; callers must verify it against the exact proposal.
    pub fn decode_canonical(bytes: &[u8]) -> Result<Self> {
        let [version, profile, genesis, proposal, votes] = codec::array(cbor::decode(bytes)?)?;
        codec::version(&version)?;
        if profile != Value::Text("LOCALNET_V0".into()) {
            return Err(Error::InvalidBlock("certificate profile"));
        }
        let Value::Array(votes) = votes else {
            return Err(Error::InvalidBlock("attestation list"));
        };
        if !(2..=3).contains(&votes.len()) {
            return Err(Error::InvalidBlock("certificate length"));
        }
        let attestations = votes
            .into_iter()
            .map(|vote| Attestation::decode_canonical(&cbor::encode(&vote)?))
            .collect::<Result<Vec<_>>>()?;
        if attestations.windows(2).any(|v| v[0].voter >= v[1].voter) {
            return Err(Error::DuplicateVote);
        }
        Ok(Self {
            genesis_id: GenesisId::from_bytes(codec::bytes(&genesis)?),
            proposal_id: ProposalId::from_bytes(codec::bytes(&proposal)?),
            attestations,
        })
    }
}

/// A proposal and its complete certificate; structural construction is not acceptance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CertifiedBlock {
    proposal: Proposal,
    certificate: Certificate,
}
impl CertifiedBlock {
    /// Pairs the evidence; `Chain::commit` performs all checks before mutation.
    #[must_use]
    pub const fn new(proposal: Proposal, certificate: Certificate) -> Self {
        Self {
            proposal,
            certificate,
        }
    }
    /// Returns the proposed block and transitions.
    #[must_use]
    pub const fn proposal(&self) -> &Proposal {
        &self.proposal
    }
    /// Returns the full certificate.
    #[must_use]
    pub const fn certificate(&self) -> &Certificate {
        &self.certificate
    }
    /// Encodes the local transport/persistence wrapper.
    pub fn encode_canonical(&self) -> Result<Vec<u8>> {
        Ok(cbor::encode(&Value::Array(vec![
            Value::Unsigned(1),
            codec::value(&self.proposal.encode_canonical()?)?,
            codec::value(&self.certificate.encode_canonical()?)?,
        ]))?)
    }
    /// Parses the wrapper without treating it as authorized history.
    pub fn decode_canonical(bytes: &[u8]) -> Result<Self> {
        let [version, proposal, certificate] = codec::array(cbor::decode(bytes)?)?;
        codec::version(&version)?;
        Ok(Self::new(
            Proposal::decode_canonical(&cbor::encode(&proposal)?)?,
            Certificate::decode_canonical(&cbor::encode(&certificate)?)?,
        ))
    }
}

/// Validated local chain state with fixed genesis and full certified history.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Chain {
    genesis: Genesis,
    state: StateSnapshot,
    tip: Option<BlockId>,
    history: Vec<CertifiedBlock>,
}
impl Chain {
    /// Derives exact initial state from a validated local genesis.
    pub fn new(genesis: Genesis) -> Result<Self> {
        let state = genesis.initial_state()?;
        Ok(Self {
            genesis,
            state,
            tip: None,
            history: vec![],
        })
    }
    /// Returns the immutable authority/configuration of this local chain.
    #[must_use]
    pub const fn genesis(&self) -> &Genesis {
        &self.genesis
    }
    /// Returns only the current successfully committed state.
    #[must_use]
    pub const fn state(&self) -> &StateSnapshot {
        &self.state
    }
    /// Returns the latest signed-header BlockId, or None at genesis.
    #[must_use]
    pub const fn tip(&self) -> Option<BlockId> {
        self.tip
    }
    /// Returns complete validated certificates in height order.
    #[must_use]
    pub fn history(&self) -> &[CertifiedBlock] {
        &self.history
    }
    /// Builds and signs the next deterministic proposal using the fixed producer key.
    pub fn propose(&self, body: BlockBodyV1, key: &Ed25519PrivateKey) -> Result<Proposal> {
        if ed25519::public_key(key)? != self.genesis.producer_public_key() {
            return Err(Error::InvalidBlock("producer key mismatch"));
        }
        let height = self.state.ledger().height().checked_increment()?;
        let next = self.state.apply_body(height, &body)?;
        let evidence = LocalnetNoneEvidence::new(NetworkClass::Localnet)?;
        let header_body = HeaderBody {
            chain_id: self.genesis.chain_id(),
            height,
            round: RoundNumber::ZERO,
            parent: self.tip.map_or(
                ParentReferenceV1::Genesis(self.genesis.id()),
                ParentReferenceV1::Block,
            ),
            producer: self.genesis.producer(),
            transaction_root: TransactionRoot::from_block_body(&body)?,
            participant_root: ParticipantRoot::from_participant_set(
                &self.genesis.participant_set(height)?,
            )?,
            state_root: next.root()?,
            delay_hash: evidence.commitment()?,
        };
        let signature = ed25519::sign_domain(
            key,
            Domain::BlockHeaderSigning,
            &cbor::encode(&header_body.to_value()?)?,
        )?;
        let header = SignedBlockHeader {
            body: header_body,
            signature,
        };
        let mut proposal = Proposal {
            genesis_id: self.genesis.id(),
            header,
            body,
            evidence,
            signature: Ed25519Signature::from_bytes([0; 64]),
        };
        proposal.signature = ed25519::sign_domain(
            key,
            Domain::ProposalSigning,
            &cbor::encode(&proposal.signing_body()?)?,
        )?;
        Ok(proposal)
    }
    /// Independently reexecutes a proposal and returns a candidate without mutating the chain.
    pub fn validate_proposal(&self, proposal: &Proposal) -> Result<StateSnapshot> {
        proposal.verify_authorization(&self.genesis)?;
        if proposal.height() != self.state.ledger().height().checked_increment()? {
            return Err(Error::WrongHeight);
        }
        proposal.header.body.parent.validate_for_height(
            proposal.height(),
            self.genesis.id(),
            self.tip.map(|id| (self.state.ledger().height(), id)),
        )?;
        let next = self.state.apply_body(proposal.height(), &proposal.body)?;
        if next.root()? != proposal.header.body.state_root {
            return Err(Error::InvalidBlock("resulting state root"));
        }
        Ok(next)
    }
    /// Commits only after structure, authority, execution, both roots and quorum verify.
    /// Every failure leaves state, tip and history unchanged.
    pub fn commit(&mut self, block: &CertifiedBlock) -> Result<()> {
        let next = self.validate_proposal(&block.proposal)?;
        block.certificate.verify(&self.genesis, &block.proposal)?;
        let tip = block.proposal.header.id()?;
        self.state = next;
        self.tip = Some(tip);
        self.history.push(block.clone());
        Ok(())
    }
}
