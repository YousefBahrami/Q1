//! Atomic native-transfer accounting using the approved LOCALNET fee profile.

use crate::{Error, LocalnetV0, Result};
use q1_primitives::{Amount, Height, Network};
use q1_protocol_types::{
    address::AddressEnvelope,
    block_body::BlockBodyV1,
    transfer::SignedTransferV1,
    types::{ChainId, Nonce, TransferId},
};
use std::collections::BTreeMap;

/// Internal account state, not a newly approved canonical wire schema.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Account {
    balance: Amount,
    nonce: Nonce,
}
impl Account {
    /// Returns controlled base units.
    #[must_use]
    pub const fn balance(self) -> Amount {
        self.balance
    }
    /// Returns the nonce required by the next outgoing transfer.
    #[must_use]
    pub const fn nonce(self) -> Nonce {
        self.nonce
    }
}

/// Observable execution result; not a canonical receipt or consensus certificate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AppliedTransfer {
    /// Complete signed transfer identity.
    pub transfer_id: TransferId,
    /// Actual charged fee, always one base unit for a successful local transfer.
    pub actual_fee: Amount,
}

/// In-memory local accounting, with an explicit fee pool and immutable supply.
///
/// No persistence or StateRoot encoding is implied by this internal representation.
/// Accounts are never pruned, so spending to zero cannot erase replay protection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ledger {
    chain_id: ChainId,
    height: Height,
    total_supply: Amount,
    reward_pool: Amount,
    accounts: BTreeMap<AddressEnvelope, Account>,
}
impl Ledger {
    /// Restores a structurally checked snapshot; callers must still authenticate history.
    pub(crate) fn restore(
        chain_id: ChainId,
        height: Height,
        total_supply: Amount,
        reward_pool: Amount,
        accounts: Vec<(AddressEnvelope, Amount, Nonce)>,
    ) -> Result<Self> {
        let mut restored = BTreeMap::new();
        for (address, balance, nonce) in accounts {
            if restored
                .insert(address, Account { balance, nonce })
                .is_some()
            {
                return Err(Error::InvalidState("duplicate account"));
            }
        }
        let ledger = Self {
            chain_id,
            height,
            total_supply,
            reward_pool,
            accounts: restored,
        };
        ledger.check_supply()?;
        Ok(ledger)
    }
    /// Builds internal initial state from explicit allocations and an exact supply.
    ///
    /// This is not a GenesisManifest parser or a genesis authorization mechanism.
    pub fn from_allocations(
        _: LocalnetV0,
        chain_id: ChainId,
        declared_supply: Amount,
        allocations: impl IntoIterator<Item = (AddressEnvelope, Amount)>,
    ) -> Result<Self> {
        let mut accounts = BTreeMap::new();
        let mut sum = Amount::ZERO;
        for (address, balance) in allocations {
            if accounts
                .insert(
                    address,
                    Account {
                        balance,
                        nonce: Nonce::new(0),
                    },
                )
                .is_some()
            {
                return Err(Error::DuplicateAllocation);
            }
            sum = sum.checked_add(balance)?;
        }
        if sum != declared_supply {
            return Err(Error::SupplyMismatch);
        }
        Ok(Self {
            chain_id,
            height: Height::new(0),
            total_supply: declared_supply,
            reward_pool: Amount::ZERO,
            accounts,
        })
    }
    /// Returns the state height after successful whole-block execution.
    #[must_use]
    pub const fn height(&self) -> Height {
        self.height
    }
    /// Returns the network instance to which transfers must be signed.
    #[must_use]
    pub const fn chain_id(&self) -> ChainId {
        self.chain_id
    }
    /// Returns the immutable declared supply.
    #[must_use]
    pub const fn total_supply(&self) -> Amount {
        self.total_supply
    }
    /// Returns fees accumulated without issuance, burning, or distribution.
    #[must_use]
    pub const fn reward_pool(&self) -> Amount {
        self.reward_pool
    }
    /// Reads an existing account or the approved zero-balance, zero-nonce default.
    #[must_use]
    pub fn account(&self, address: AddressEnvelope) -> Account {
        self.accounts.get(&address).copied().unwrap_or(Account {
            balance: Amount::ZERO,
            nonce: Nonce::new(0),
        })
    }
    /// Reads stored accounts in raw-address order; does not prune zero balances.
    pub fn accounts(&self) -> impl Iterator<Item = (&AddressEnvelope, &Account)> {
        self.accounts.iter()
    }
    /// Checks exact conservation including the protocol reward pool.
    pub fn check_supply(&self) -> Result<()> {
        let sum = self
            .accounts
            .values()
            .try_fold(self.reward_pool, |sum, account| {
                sum.checked_add(account.balance)
            })?;
        if sum != self.total_supply {
            return Err(Error::SupplyMismatch);
        }
        Ok(())
    }
    /// Executes an ordered block atomically after caller-established consensus validity.
    ///
    /// Any rejected transfer rejects the entire candidate, including all its fees.
    /// Success is an accounting result, not a finality claim.
    pub fn apply_block(
        &mut self,
        height: Height,
        body: &BlockBodyV1,
    ) -> Result<Vec<AppliedTransfer>> {
        if height != self.height.checked_increment()? {
            return Err(Error::WrongHeight);
        }
        let mut candidate = self.clone();
        let mut results = Vec::new();
        for transfer in body.transfers() {
            results.push(candidate.apply_transfer(height, transfer)?);
        }
        candidate.height = height;
        candidate.check_supply()?;
        *self = candidate;
        Ok(results)
    }
    fn apply_transfer(
        &mut self,
        height: Height,
        transfer: &SignedTransferV1,
    ) -> Result<AppliedTransfer> {
        transfer.verify_for_chain(self.chain_id)?;
        let fields = transfer.body().fields();
        if fields.amount == Amount::ZERO {
            return Err(Error::ZeroAmount);
        }
        if fields.fee_limit.get() < 1 {
            return Err(Error::FeeLimit);
        }
        if height < fields.valid_from_height || height > fields.valid_until_height {
            return Err(Error::ExpiredOrPremature);
        }
        let sender_address =
            AddressEnvelope::from_address(transfer.body().sender_address(Network::Localnet)?);
        if sender_address == fields.recipient_address {
            return Err(Error::SelfTransfer);
        }
        let sender = self.account(sender_address);
        let recipient = self.account(fields.recipient_address);
        if fields.nonce != sender.nonce {
            return Err(Error::NonceMismatch);
        }
        let fee = Amount::new(1);
        let debit = fields.amount.checked_add(fee)?;
        if sender.balance < debit {
            return Err(Error::InsufficientBalance);
        }
        let sender_after = Account {
            balance: sender.balance.checked_sub(debit)?,
            nonce: Nonce::new(
                sender
                    .nonce
                    .get()
                    .checked_add(1)
                    .ok_or(q1_primitives::Error::Overflow)?,
            ),
        };
        let recipient_after = Account {
            balance: recipient.balance.checked_add(fields.amount)?,
            nonce: recipient.nonce,
        };
        let pool_after = self.reward_pool.checked_add(fee)?;
        let transfer_id = transfer.id()?;
        self.accounts.insert(sender_address, sender_after);
        self.accounts
            .insert(fields.recipient_address, recipient_after);
        self.reward_pool = pool_after;
        Ok(AppliedTransfer {
            transfer_id,
            actual_fee: fee,
        })
    }
}
