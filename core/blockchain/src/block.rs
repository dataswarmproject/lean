//! Block structure and validation

use lean_crypto::{Address, Hash256};
use lean_transaction::Transaction;
use serde::{Deserialize, Serialize};
use std::fmt;

/// A block in the blockchain
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    /// Block header
    pub header: BlockHeader,

    /// Transactions in this block
    pub transactions: Vec<Transaction>,
}

impl Block {
    /// Create a new block
    pub fn new(
        height: u64,
        previous_hash: Hash256,
        state_root: Hash256,
        validator: Address,
        transactions: Vec<Transaction>,
    ) -> Self {
        let transactions_root = crate::merkle::MerkleTree::from_transactions(&transactions).root();
        let timestamp = chrono::Utc::now().timestamp();

        let mut header = BlockHeader {
            version: 1,
            height,
            previous_hash,
            transactions_root,
            state_root,
            timestamp,
            validator,
            signature: lean_crypto::Signature::from_bytes([0u8; 64]),
            total_fees: 0,
            gas_used: 0,
            gas_limit: 10_000_000, // 10M gas per block
        };

        // Calculate total fees and gas used
        let (total_fees, gas_used) = transactions.iter().fold((0u128, 0u64), |(fees, gas), tx| {
            (fees + tx.cost(), gas + tx.gas_limit)
        });

        header.total_fees = total_fees;
        header.gas_used = gas_used;

        Self {
            header,
            transactions,
        }
    }

    /// Create the genesis block
    pub fn genesis(state_root: Hash256) -> Self {
        let genesis_validator = Address::zero();
        let header = BlockHeader {
            version: 1,
            height: 0,
            previous_hash: Hash256::zero(),
            transactions_root: Hash256::zero(),
            state_root,
            timestamp: 0,
            validator: genesis_validator,
            signature: lean_crypto::Signature::from_bytes([0u8; 64]),
            total_fees: 0,
            gas_used: 0,
            gas_limit: 10_000_000,
        };

        Self {
            header,
            transactions: Vec::new(),
        }
    }

    /// Sign the block
    pub fn sign(&mut self, keypair: &lean_crypto::KeyPair) {
        let message = self.header.signing_message();
        self.header.signature = keypair.sign(&message);
    }

    /// Get block hash
    pub fn hash(&self) -> Hash256 {
        self.header.hash()
    }

    /// Verify block signature
    pub fn verify_signature(&self, public_key: &lean_crypto::PublicKey) -> Result<(), BlockError> {
        let message = self.header.signing_message();
        public_key
            .verify(&message, &self.header.signature)
            .map_err(|_| BlockError::InvalidSignature)
    }

    /// Validate block structure
    pub fn validate(&self) -> Result<(), BlockError> {
        // Verify transactions root
        let computed_root = crate::merkle::MerkleTree::from_transactions(&self.transactions).root();
        if computed_root != self.header.transactions_root {
            return Err(BlockError::InvalidTransactionsRoot);
        }

        // Verify total fees
        let (total_fees, gas_used) = self.transactions.iter().fold((0u128, 0u64), |(fees, gas), tx| {
            (fees + tx.cost(), gas + tx.gas_limit)
        });

        if total_fees != self.header.total_fees {
            return Err(BlockError::InvalidTotalFees {
                expected: total_fees,
                actual: self.header.total_fees,
            });
        }

        if gas_used != self.header.gas_used {
            return Err(BlockError::InvalidGasUsed {
                expected: gas_used,
                actual: self.header.gas_used,
            });
        }

        // Check gas limit
        if gas_used > self.header.gas_limit {
            return Err(BlockError::GasLimitExceeded {
                limit: self.header.gas_limit,
                used: gas_used,
            });
        }

        Ok(())
    }

    /// Get block size in bytes
    pub fn size(&self) -> usize {
        bincode::serialize(self).unwrap_or_default().len()
    }

    /// Check if this is the genesis block
    pub fn is_genesis(&self) -> bool {
        self.header.height == 0 && self.header.previous_hash.is_zero()
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Block(height: {}, txs: {}, hash: {})",
            self.header.height,
            self.transactions.len(),
            self.hash()
        )
    }
}

/// Block header containing metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockHeader {
    /// Protocol version
    pub version: u32,

    /// Block height (number)
    pub height: u64,

    /// Hash of the previous block
    pub previous_hash: Hash256,

    /// Merkle root of transactions
    pub transactions_root: Hash256,

    /// State root (Merkle Patricia Trie)
    pub state_root: Hash256,

    /// Block timestamp (Unix timestamp)
    pub timestamp: i64,

    /// Validator address
    pub validator: Address,

    /// Validator signature
    pub signature: lean_crypto::Signature,

    /// Total transaction fees in block
    pub total_fees: u128,

    /// Total gas used
    pub gas_used: u64,

    /// Gas limit for block
    pub gas_limit: u64,
}

impl BlockHeader {
    /// Get the signing message for the header
    pub fn signing_message(&self) -> Vec<u8> {
        let mut message = Vec::new();
        message.extend_from_slice(&self.version.to_le_bytes());
        message.extend_from_slice(&self.height.to_le_bytes());
        message.extend_from_slice(self.previous_hash.as_bytes());
        message.extend_from_slice(self.transactions_root.as_bytes());
        message.extend_from_slice(self.state_root.as_bytes());
        message.extend_from_slice(&self.timestamp.to_le_bytes());
        message.extend_from_slice(self.validator.as_bytes());
        message.extend_from_slice(&self.total_fees.to_le_bytes());
        message.extend_from_slice(&self.gas_used.to_le_bytes());
        message.extend_from_slice(&self.gas_limit.to_le_bytes());
        message
    }

    /// Compute block header hash
    pub fn hash(&self) -> Hash256 {
        let mut hasher = lean_crypto::Hasher::new();
        hasher.update(&self.signing_message());
        hasher.update(self.signature.as_bytes());
        hasher.finalize()
    }
}

/// Block errors
#[derive(Debug, thiserror::Error)]
pub enum BlockError {
    #[error("Invalid block signature")]
    InvalidSignature,

    #[error("Invalid transactions root")]
    InvalidTransactionsRoot,

    #[error("Invalid total fees: expected {expected}, got {actual}")]
    InvalidTotalFees { expected: u128, actual: u128 },

    #[error("Invalid gas used: expected {expected}, got {actual}")]
    InvalidGasUsed { expected: u64, actual: u64 },

    #[error("Gas limit exceeded: limit {limit}, used {used}")]
    GasLimitExceeded { limit: u64, used: u64 },

    #[error("Invalid previous hash")]
    InvalidPreviousHash,

    #[error("Invalid timestamp")]
    InvalidTimestamp,

    #[error("Block too large: {size} bytes (max {max})")]
    BlockTooLarge { size: usize, max: usize },
}

#[cfg(test)]
mod tests {
    use super::*;
    use lean_crypto::KeyPair;

    #[test]
    fn test_genesis_block() {
        let genesis = Block::genesis(Hash256::zero());
        assert_eq!(genesis.header.height, 0);
        assert!(genesis.header.previous_hash.is_zero());
        assert!(genesis.is_genesis());
        assert_eq!(genesis.transactions.len(), 0);
    }

    #[test]
    fn test_block_creation() {
        let keypair = KeyPair::generate();
        let validator = Address::from_public_key(keypair.public_key());

        let mut block = Block::new(
            1,
            Hash256::hash(b"previous"),
            Hash256::zero(),
            validator,
            Vec::new(),
        );

        block.sign(&keypair);
        assert_eq!(block.header.height, 1);
        assert!(!block.is_genesis());
    }

    #[test]
    fn test_block_validation() {
        let keypair = KeyPair::generate();
        let validator = Address::from_public_key(keypair.public_key());

        let mut block = Block::new(
            1,
            Hash256::hash(b"previous"),
            Hash256::zero(),
            validator,
            Vec::new(),
        );

        block.sign(&keypair);
        assert!(block.validate().is_ok());
    }

    #[test]
    fn test_block_signature_verification() {
        let keypair = KeyPair::generate();
        let validator = Address::from_public_key(keypair.public_key());

        let mut block = Block::new(
            1,
            Hash256::hash(b"previous"),
            Hash256::zero(),
            validator,
            Vec::new(),
        );

        block.sign(&keypair);
        assert!(block.verify_signature(keypair.public_key()).is_ok());
    }
}
