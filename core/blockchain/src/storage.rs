//! Block storage implementations

use crate::Block;
use lean_crypto::Hash256;
use std::collections::HashMap;
use std::sync::RwLock;

/// Block storage errors
#[derive(Debug, thiserror::Error)]
pub enum BlockStorageError {
    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Block not found: {0}")]
    BlockNotFound(Hash256),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Block storage trait
pub trait BlockStorage: Send + Sync {
    /// Store a block
    fn store_block(&self, block: &Block) -> Result<(), BlockStorageError>;

    /// Get a block by hash
    fn get_block(&self, hash: &Hash256) -> Result<Option<Block>, BlockStorageError>;

    /// Get a block by height
    fn get_block_by_height(&self, height: u64) -> Result<Option<Block>, BlockStorageError>;

    /// Set the best block (chain tip)
    fn set_best_block(&self, hash: Hash256) -> Result<(), BlockStorageError>;

    /// Get the best block hash
    fn get_best_block(&self) -> Result<Hash256, BlockStorageError>;

    /// Delete a block (for reorganization)
    fn delete_block(&self, hash: &Hash256) -> Result<(), BlockStorageError>;
}

/// In-memory block storage (for testing)
pub struct MemoryBlockStorage {
    /// Blocks by hash
    blocks: RwLock<HashMap<Hash256, Block>>,

    /// Block hashes by height
    heights: RwLock<HashMap<u64, Hash256>>,

    /// Best block hash
    best_block: RwLock<Option<Hash256>>,
}

impl MemoryBlockStorage {
    /// Create a new memory block storage
    pub fn new() -> Self {
        Self {
            blocks: RwLock::new(HashMap::new()),
            heights: RwLock::new(HashMap::new()),
            best_block: RwLock::new(None),
        }
    }

    /// Get the number of blocks stored
    pub fn len(&self) -> usize {
        self.blocks.read().unwrap().len()
    }

    /// Check if storage is empty
    pub fn is_empty(&self) -> bool {
        self.blocks.read().unwrap().is_empty()
    }
}

impl Default for MemoryBlockStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockStorage for MemoryBlockStorage {
    fn store_block(&self, block: &Block) -> Result<(), BlockStorageError> {
        let hash = block.hash();
        let height = block.header.height;

        let mut blocks = self.blocks.write().unwrap();
        let mut heights = self.heights.write().unwrap();

        blocks.insert(hash, block.clone());
        heights.insert(height, hash);

        Ok(())
    }

    fn get_block(&self, hash: &Hash256) -> Result<Option<Block>, BlockStorageError> {
        let blocks = self.blocks.read().unwrap();
        Ok(blocks.get(hash).cloned())
    }

    fn get_block_by_height(&self, height: u64) -> Result<Option<Block>, BlockStorageError> {
        let heights = self.heights.read().unwrap();
        if let Some(hash) = heights.get(&height) {
            self.get_block(hash)
        } else {
            Ok(None)
        }
    }

    fn set_best_block(&self, hash: Hash256) -> Result<(), BlockStorageError> {
        let mut best_block = self.best_block.write().unwrap();
        *best_block = Some(hash);
        Ok(())
    }

    fn get_best_block(&self) -> Result<Hash256, BlockStorageError> {
        let best_block = self.best_block.read().unwrap();
        best_block.ok_or_else(|| BlockStorageError::DatabaseError("No best block set".to_string()))
    }

    fn delete_block(&self, hash: &Hash256) -> Result<(), BlockStorageError> {
        let mut blocks = self.blocks.write().unwrap();

        if let Some(block) = blocks.remove(hash) {
            let mut heights = self.heights.write().unwrap();
            heights.remove(&block.header.height);
        }

        Ok(())
    }
}

/// RocksDB-based block storage
pub struct RocksDbBlockStorage {
    db: rocksdb::DB,
}

impl RocksDbBlockStorage {
    /// Open or create a RocksDB block storage
    pub fn open(path: &str) -> Result<Self, BlockStorageError> {
        let mut opts = rocksdb::Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);

        let db = rocksdb::DB::open(&opts, path)
            .map_err(|e| BlockStorageError::DatabaseError(e.to_string()))?;

        Ok(Self { db })
    }

    /// Key prefix for blocks
    fn block_key(hash: &Hash256) -> Vec<u8> {
        let mut key = b"block:".to_vec();
        key.extend_from_slice(hash.as_bytes());
        key
    }

    /// Key prefix for height index
    fn height_key(height: u64) -> Vec<u8> {
        let mut key = b"height:".to_vec();
        key.extend_from_slice(&height.to_le_bytes());
        key
    }

    /// Key for best block
    fn best_block_key() -> &'static [u8] {
        b"best_block"
    }
}

impl BlockStorage for RocksDbBlockStorage {
    fn store_block(&self, block: &Block) -> Result<(), BlockStorageError> {
        let hash = block.hash();
        let height = block.header.height;

        // Serialize block
        let block_bytes = bincode::serialize(block)
            .map_err(|e| BlockStorageError::SerializationError(e.to_string()))?;

        // Store block by hash
        self.db
            .put(Self::block_key(&hash), block_bytes)
            .map_err(|e| BlockStorageError::DatabaseError(e.to_string()))?;

        // Store height index
        self.db
            .put(Self::height_key(height), hash.as_bytes())
            .map_err(|e| BlockStorageError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    fn get_block(&self, hash: &Hash256) -> Result<Option<Block>, BlockStorageError> {
        match self.db.get(Self::block_key(hash)) {
            Ok(Some(bytes)) => {
                let block = bincode::deserialize(&bytes)
                    .map_err(|e| BlockStorageError::SerializationError(e.to_string()))?;
                Ok(Some(block))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(BlockStorageError::DatabaseError(e.to_string())),
        }
    }

    fn get_block_by_height(&self, height: u64) -> Result<Option<Block>, BlockStorageError> {
        match self.db.get(Self::height_key(height)) {
            Ok(Some(hash_bytes)) => {
                let hash = Hash256::from_slice(&hash_bytes)
                    .map_err(|e| BlockStorageError::SerializationError(e))?;
                self.get_block(&hash)
            }
            Ok(None) => Ok(None),
            Err(e) => Err(BlockStorageError::DatabaseError(e.to_string())),
        }
    }

    fn set_best_block(&self, hash: Hash256) -> Result<(), BlockStorageError> {
        self.db
            .put(Self::best_block_key(), hash.as_bytes())
            .map_err(|e| BlockStorageError::DatabaseError(e.to_string()))
    }

    fn get_best_block(&self) -> Result<Hash256, BlockStorageError> {
        match self.db.get(Self::best_block_key()) {
            Ok(Some(bytes)) => Hash256::from_slice(&bytes)
                .map_err(|e| BlockStorageError::SerializationError(e)),
            Ok(None) => Err(BlockStorageError::DatabaseError("No best block set".to_string())),
            Err(e) => Err(BlockStorageError::DatabaseError(e.to_string())),
        }
    }

    fn delete_block(&self, hash: &Hash256) -> Result<(), BlockStorageError> {
        // Get the block to find its height
        if let Some(block) = self.get_block(hash)? {
            // Delete height index
            self.db
                .delete(Self::height_key(block.header.height))
                .map_err(|e| BlockStorageError::DatabaseError(e.to_string()))?;
        }

        // Delete block
        self.db
            .delete(Self::block_key(hash))
            .map_err(|e| BlockStorageError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lean_crypto::{KeyPair, Address};

    #[test]
    fn test_memory_storage() {
        let storage = MemoryBlockStorage::new();
        let genesis = Block::genesis(Hash256::zero());
        let hash = genesis.hash();

        storage.store_block(&genesis).unwrap();
        let retrieved = storage.get_block(&hash).unwrap().unwrap();

        assert_eq!(retrieved.header.height, genesis.header.height);
    }

    #[test]
    fn test_height_index() {
        let storage = MemoryBlockStorage::new();
        let genesis = Block::genesis(Hash256::zero());

        storage.store_block(&genesis).unwrap();
        let retrieved = storage.get_block_by_height(0).unwrap().unwrap();

        assert_eq!(retrieved.header.height, 0);
    }

    #[test]
    fn test_best_block() {
        let storage = MemoryBlockStorage::new();
        let genesis = Block::genesis(Hash256::zero());
        let hash = genesis.hash();

        storage.store_block(&genesis).unwrap();
        storage.set_best_block(hash).unwrap();

        let best = storage.get_best_block().unwrap();
        assert_eq!(best, hash);
    }
}
