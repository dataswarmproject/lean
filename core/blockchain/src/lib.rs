//! LEAN CHAIN Blockchain Core
//!
//! This module provides the core blockchain functionality including:
//! - Block structure and validation
//! - Chain management and storage
//! - Merkle tree for transaction verification
//! - Block reorganization handling

pub mod block;
pub mod chain;
pub mod merkle;
pub mod storage;

pub use block::{Block, BlockHeader, BlockError};
pub use chain::{Chain, ChainConfig, ChainError};
pub use merkle::MerkleTree;
pub use storage::{BlockStorage, BlockStorageError};

use lean_crypto::Hash256;

/// Result type for blockchain operations
pub type Result<T> = std::result::Result<T, BlockchainError>;

/// Blockchain errors
#[derive(Debug, thiserror::Error)]
pub enum BlockchainError {
    #[error("Block error: {0}")]
    BlockError(#[from] BlockError),

    #[error("Chain error: {0}")]
    ChainError(#[from] ChainError),

    #[error("Storage error: {0}")]
    StorageError(#[from] BlockStorageError),

    #[error("Invalid genesis block")]
    InvalidGenesis,

    #[error("Block not found: {0}")]
    BlockNotFound(Hash256),

    #[error("Transaction not found: {0}")]
    TransactionNotFound(Hash256),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain_error_display() {
        let err = BlockchainError::InvalidGenesis;
        assert_eq!(err.to_string(), "Invalid genesis block");
    }
}
