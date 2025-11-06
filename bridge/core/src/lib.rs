//! LEAN CHAIN Cross-Chain Bridge Framework
//!
//! Universal bridge framework for connecting to multiple blockchains.

mod transfer;
mod validator;
mod relayer;
mod state;

pub use transfer::{BridgeTransfer, TransferId, TransferStatus};
pub use validator::{BridgeValidator, ValidatorSet, MultisigThreshold};
pub use relayer::{Relayer, RelayerSet};
pub use state::{BridgeState, ChainState};

use thiserror::Error;

/// Bridge errors
#[derive(Debug, Error)]
pub enum BridgeError {
    #[error("Transfer not found: {0}")]
    TransferNotFound(String),
    
    #[error("Insufficient signatures: got {got}, need {required}")]
    InsufficientSignatures { got: usize, required: usize },
    
    #[error("Invalid signature from validator {0}")]
    InvalidSignature(String),
    
    #[error("Transfer already processed: {0}")]
    TransferAlreadyProcessed(String),
    
    #[error("Chain not supported: {0}")]
    ChainNotSupported(String),
    
    #[error("Insufficient liquidity for transfer")]
    InsufficientLiquidity,
    
    #[error("Transfer amount below minimum: {amount} < {minimum}")]
    AmountBelowMinimum { amount: u128, minimum: u128 },
    
    #[error("Transfer amount above maximum: {amount} > {maximum}")]
    AmountAboveMaximum { amount: u128, maximum: u128 },
    
    #[error("Relayer not authorized: {0}")]
    UnauthorizedRelayer(String),
    
    #[error("Invalid destination address: {0}")]
    InvalidDestination(String),
    
    #[error("Arithmetic overflow")]
    ArithmeticOverflow,
}

pub type Result<T> = std::result::Result<T, BridgeError>;

/// Supported blockchain types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChainType {
    Ethereum,
    Bitcoin,
    Cosmos,
    Polkadot,
    Solana,
}
