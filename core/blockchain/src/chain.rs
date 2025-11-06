//! Blockchain chain management

use crate::{Block, BlockStorage};
use lean_crypto::Hash256;
use std::sync::{Arc, RwLock};

/// Chain configuration
#[derive(Clone, Debug)]
pub struct ChainConfig {
    /// Chain ID (for replay protection)
    pub chain_id: u64,

    /// Genesis block hash
    pub genesis_hash: Hash256,

    /// Maximum block size in bytes
    pub max_block_size: usize,

    /// Target block time in seconds
    pub target_block_time: u64,

    /// Gas limit per block
    pub gas_limit: u64,
}

impl Default for ChainConfig {
    fn default() -> Self {
        Self {
            chain_id: 1,
            genesis_hash: Hash256::zero(),
            max_block_size: 10_485_760, // 10 MB
            target_block_time: 2,        // 2 seconds
            gas_limit: 10_000_000,       // 10M gas
        }
    }
}

/// The blockchain
pub struct Chain {
    /// Chain configuration
    config: ChainConfig,

    /// Block storage
    storage: Arc<dyn BlockStorage>,

    /// Current chain tip (best block hash)
    tip: RwLock<Hash256>,

    /// Current height
    height: RwLock<u64>,
}

impl Chain {
    /// Create a new chain with genesis block
    pub fn new(config: ChainConfig, storage: Arc<dyn BlockStorage>, genesis: Block) -> Result<Self, ChainError> {
        // Validate genesis block
        if !genesis.is_genesis() {
            return Err(ChainError::InvalidGenesisBlock);
        }

        let genesis_hash = genesis.hash();

        // Store genesis block
        storage.store_block(&genesis)?;
        storage.set_best_block(genesis_hash)?;

        Ok(Self {
            config,
            storage,
            tip: RwLock::new(genesis_hash),
            height: RwLock::new(0),
        })
    }

    /// Load existing chain from storage
    pub fn load(config: ChainConfig, storage: Arc<dyn BlockStorage>) -> Result<Self, ChainError> {
        let tip = storage.get_best_block()?;
        let block = storage.get_block(&tip)?
            .ok_or(ChainError::BlockNotFound(tip))?;

        Ok(Self {
            config,
            storage,
            tip: RwLock::new(tip),
            height: RwLock::new(block.header.height),
        })
    }

    /// Add a block to the chain
    pub fn add_block(&self, block: Block) -> Result<(), ChainError> {
        // Validate block structure
        block.validate()?;

        // Check if parent exists
        let parent = self.storage.get_block(&block.header.previous_hash)?
            .ok_or(ChainError::BlockNotFound(block.header.previous_hash))?;

        // Validate height
        if block.header.height != parent.header.height + 1 {
            return Err(ChainError::InvalidHeight {
                expected: parent.header.height + 1,
                actual: block.header.height,
            });
        }

        // Store block
        let block_hash = block.hash();
        self.storage.store_block(&block)?;

        // Update tip if this extends the best chain
        let current_height = *self.height.read().unwrap();
        if block.header.height > current_height {
            self.storage.set_best_block(block_hash)?;
            *self.tip.write().unwrap() = block_hash;
            *self.height.write().unwrap() = block.header.height;
        }

        Ok(())
    }

    /// Get the current chain tip
    pub fn tip(&self) -> Hash256 {
        *self.tip.read().unwrap()
    }

    /// Get the current height
    pub fn height(&self) -> u64 {
        *self.height.read().unwrap()
    }

    /// Get a block by hash
    pub fn get_block(&self, hash: &Hash256) -> Result<Option<Block>, ChainError> {
        Ok(self.storage.get_block(hash)?)
    }

    /// Get a block by height
    pub fn get_block_by_height(&self, height: u64) -> Result<Option<Block>, ChainError> {
        Ok(self.storage.get_block_by_height(height)?)
    }

    /// Get the genesis block
    pub fn get_genesis(&self) -> Result<Block, ChainError> {
        self.get_block_by_height(0)?
            .ok_or(ChainError::GenesisNotFound)
    }

    /// Get chain configuration
    pub fn config(&self) -> &ChainConfig {
        &self.config
    }
}

/// Chain errors
#[derive(Debug, thiserror::Error)]
pub enum ChainError {
    #[error("Invalid genesis block")]
    InvalidGenesisBlock,

    #[error("Genesis block not found")]
    GenesisNotFound,

    #[error("Block not found: {0}")]
    BlockNotFound(Hash256),

    #[error("Invalid height: expected {expected}, got {actual}")]
    InvalidHeight { expected: u64, actual: u64 },

    #[error("Block error: {0}")]
    BlockError(#[from] crate::BlockError),

    #[error("Storage error: {0}")]
    StorageError(#[from] crate::BlockStorageError),

    #[error("Fork detected")]
    ForkDetected,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::MemoryBlockStorage;
    use lean_crypto::{KeyPair, Address};

    #[test]
    fn test_chain_creation() {
        let config = ChainConfig::default();
        let storage = Arc::new(MemoryBlockStorage::new());
        let genesis = Block::genesis(Hash256::zero());

        let chain = Chain::new(config, storage, genesis).unwrap();
        assert_eq!(chain.height(), 0);
    }

    #[test]
    fn test_add_block() {
        let config = ChainConfig::default();
        let storage = Arc::new(MemoryBlockStorage::new());
        let genesis = Block::genesis(Hash256::zero());
        let genesis_hash = genesis.hash();

        let chain = Chain::new(config, storage, genesis).unwrap();

        let keypair = KeyPair::generate();
        let validator = Address::from_public_key(keypair.public_key());

        let mut block = Block::new(
            1,
            genesis_hash,
            Hash256::zero(),
            validator,
            Vec::new(),
        );
        block.sign(&keypair);

        chain.add_block(block).unwrap();
        assert_eq!(chain.height(), 1);
    }
}
