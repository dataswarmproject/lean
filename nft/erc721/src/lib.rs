//! LEAN CHAIN NFT Support (ERC-721 Compatible)
//!
//! Complete NFT implementation following ERC-721 standard.

mod token;
mod metadata;
mod enumerable;
mod approval;

pub use token::{NFTContract, TokenId};
pub use metadata::{TokenMetadata, MetadataUri};
pub use enumerable::{EnumerableNFT, TokenIterator};
pub use approval::{ApprovalManager, OperatorApproval};

use thiserror::Error;

/// NFT errors
#[derive(Debug, Error)]
pub enum NFTError {
    #[error("Token not found: {0}")]
    TokenNotFound(u128),
    
    #[error("Not token owner: token {token_id}, owner {owner}, caller {caller}")]
    NotOwner {
        token_id: u128,
        owner: String,
        caller: String,
    },
    
    #[error("Not authorized: caller {0}")]
    NotAuthorized(String),
    
    #[error("Token already exists: {0}")]
    TokenAlreadyExists(u128),
    
    #[error("Invalid recipient: zero address")]
    InvalidRecipient,
    
    #[error("Transfer to self")]
    TransferToSelf,
    
    #[error("Invalid metadata URI: {0}")]
    InvalidMetadataUri(String),
    
    #[error("Arithmetic overflow")]
    ArithmeticOverflow,
}

pub type Result<T> = std::result::Result<T, NFTError>;
