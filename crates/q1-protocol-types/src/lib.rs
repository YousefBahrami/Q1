//! Approved Q1 V1 protocol objects, canonical bytes, and typed commitments.
//!
//! Structural decoding is not ledger, network admission, or finality validation.
//! Object-specific runtime limits and the state/consensus rules remain separate
//! decisions. Never treat a decoded object as an executed or finalized object.

pub mod address;
pub mod block_body;
pub mod chain;
mod codec;
pub mod delay;
pub mod error;
pub mod merkle;
pub mod parent;
pub mod participant;
pub mod transfer;
pub mod types;

pub use error::{Error, Result};
