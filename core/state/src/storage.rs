//! State storage implementations

use crate::{Account, AccountState, StateRoot};
use lean_crypto::{Address, Hash256};
use std::collections::HashMap;
use std::sync::RwLock;

/// State storage errors
#[derive(Debug, thiserror::Error)]
pub enum StateStorageError {
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Account not found: {0}")]
    AccountNotFound(Address),
}

/// State storage trait
pub trait StateStorage: Send + Sync {
    /// Get an account
    fn get_account(&self, address: &Address) -> Result<Option<Account>, StateStorageError>;
    
    /// Get account state (including storage and code)
    fn get_account_state(&self, address: &Address) -> Result<Option<AccountState>, StateStorageError>;
    
    /// Set account state
    fn set_account_state(&self, address: &Address, state: &AccountState) -> Result<(), StateStorageError>;
    
    /// Delete account
    fn delete_account(&self, address: &Address) -> Result<(), StateStorageError>;
    
    /// Get state root
    fn get_state_root(&self) -> Result<StateRoot, StateStorageError>;
    
    /// Set state root
    fn set_state_root(&self, root: StateRoot) -> Result<(), StateStorageError>;
    
    /// Calculate state root from all accounts
    fn calculate_state_root(&self) -> Result<StateRoot, StateStorageError>;
}

/// In-memory state storage (for testing)
pub struct MemoryStateStorage {
    /// Accounts by address
    accounts: RwLock<HashMap<Address, AccountState>>,
    
    /// Current state root
    state_root: RwLock<StateRoot>,
}

impl MemoryStateStorage {
    /// Create a new memory state storage
    pub fn new() -> Self {
        Self {
            accounts: RwLock::new(HashMap::new()),
            state_root: RwLock::new(Hash256::zero()),
        }
    }
    
    /// Get the number of accounts
    pub fn len(&self) -> usize {
        self.accounts.read().unwrap().len()
    }
    
    /// Check if storage is empty
    pub fn is_empty(&self) -> bool {
        self.accounts.read().unwrap().is_empty()
    }
}

impl Default for MemoryStateStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl StateStorage for MemoryStateStorage {
    fn get_account(&self, address: &Address) -> Result<Option<Account>, StateStorageError> {
        let accounts = self.accounts.read().unwrap();
        Ok(accounts.get(address).map(|state| state.account.clone()))
    }
    
    fn get_account_state(&self, address: &Address) -> Result<Option<AccountState>, StateStorageError> {
        let accounts = self.accounts.read().unwrap();
        Ok(accounts.get(address).cloned())
    }
    
    fn set_account_state(&self, address: &Address, state: &AccountState) -> Result<(), StateStorageError> {
        let mut accounts = self.accounts.write().unwrap();
        accounts.insert(*address, state.clone());
        Ok(())
    }
    
    fn delete_account(&self, address: &Address) -> Result<(), StateStorageError> {
        let mut accounts = self.accounts.write().unwrap();
        accounts.remove(address);
        Ok(())
    }
    
    fn get_state_root(&self) -> Result<StateRoot, StateStorageError> {
        Ok(*self.state_root.read().unwrap())
    }
    
    fn set_state_root(&self, root: StateRoot) -> Result<(), StateStorageError> {
        *self.state_root.write().unwrap() = root;
        Ok(())
    }
    
    fn calculate_state_root(&self) -> Result<StateRoot, StateStorageError> {
        let accounts = self.accounts.read().unwrap();
        
        if accounts.is_empty() {
            return Ok(Hash256::zero());
        }
        
        // Simple state root calculation: hash all accounts
        let mut hasher = lean_crypto::Hasher::new();
        let mut addresses: Vec<_> = accounts.keys().collect();
        addresses.sort();
        
        for address in addresses {
            hasher.update(address.as_bytes());
            let account_state = &accounts[address];
            
            // Hash account data
            hasher.update(&account_state.account.balance.to_le_bytes());
            hasher.update(&account_state.account.nonce.to_le_bytes());
            
            if let Some(code_hash) = &account_state.account.code_hash {
                hasher.update(code_hash.as_bytes());
            }
            
            // Hash storage root
            let storage_root = account_state.calculate_storage_root();
            hasher.update(storage_root.as_bytes());
        }
        
        Ok(hasher.finalize())
    }
}

/// RocksDB-based state storage
pub struct RocksDbStateStorage {
    db: rocksdb::DB,
}

impl RocksDbStateStorage {
    /// Open or create a RocksDB state storage
    pub fn open(path: &str) -> Result<Self, StateStorageError> {
        let mut opts = rocksdb::Options::default();
        opts.create_if_missing(true);
        
        let db = rocksdb::DB::open(&opts, path)
            .map_err(|e| StateStorageError::DatabaseError(e.to_string()))?;
        
        Ok(Self { db })
    }
    
    /// Key for account
    fn account_key(address: &Address) -> Vec<u8> {
        let mut key = b"account:".to_vec();
        key.extend_from_slice(address.as_bytes());
        key
    }
    
    /// Key for state root
    fn state_root_key() -> &'static [u8] {
        b"state_root"
    }
}

impl StateStorage for RocksDbStateStorage {
    fn get_account(&self, address: &Address) -> Result<Option<Account>, StateStorageError> {
        match self.get_account_state(address)? {
            Some(state) => Ok(Some(state.account)),
            None => Ok(None),
        }
    }
    
    fn get_account_state(&self, address: &Address) -> Result<Option<AccountState>, StateStorageError> {
        match self.db.get(Self::account_key(address)) {
            Ok(Some(bytes)) => {
                let state = bincode::deserialize(&bytes)
                    .map_err(|e| StateStorageError::SerializationError(e.to_string()))?;
                Ok(Some(state))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(StateStorageError::DatabaseError(e.to_string())),
        }
    }
    
    fn set_account_state(&self, address: &Address, state: &AccountState) -> Result<(), StateStorageError> {
        let bytes = bincode::serialize(state)
            .map_err(|e| StateStorageError::SerializationError(e.to_string()))?;
        
        self.db.put(Self::account_key(address), bytes)
            .map_err(|e| StateStorageError::DatabaseError(e.to_string()))
    }
    
    fn delete_account(&self, address: &Address) -> Result<(), StateStorageError> {
        self.db.delete(Self::account_key(address))
            .map_err(|e| StateStorageError::DatabaseError(e.to_string()))
    }
    
    fn get_state_root(&self) -> Result<StateRoot, StateStorageError> {
        match self.db.get(Self::state_root_key()) {
            Ok(Some(bytes)) => {
                Hash256::from_slice(&bytes)
                    .map_err(|e| StateStorageError::SerializationError(e))
            }
            Ok(None) => Ok(Hash256::zero()),
            Err(e) => Err(StateStorageError::DatabaseError(e.to_string())),
        }
    }
    
    fn set_state_root(&self, root: StateRoot) -> Result<(), StateStorageError> {
        self.db.put(Self::state_root_key(), root.as_bytes())
            .map_err(|e| StateStorageError::DatabaseError(e.to_string()))
    }
    
    fn calculate_state_root(&self) -> Result<StateRoot, StateStorageError> {
        // In a real implementation, this would traverse a Merkle Patricia Trie
        // For now, we'll compute a simple hash of all accounts
        let mut hasher = lean_crypto::Hasher::new();
        
        let iter = self.db.prefix_iterator(b"account:");
        for item in iter {
            let (key, value) = item.map_err(|e| StateStorageError::DatabaseError(e.to_string()))?;
            hasher.update(&key);
            hasher.update(&value);
        }
        
        Ok(hasher.finalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lean_crypto::KeyPair;
    
    #[test]
    fn test_memory_storage() {
        let storage = MemoryStateStorage::new();
        let address = Address::from_public_key(&KeyPair::generate().public_key());
        
        let account = Account::new(1_000_000);
        let account_state = AccountState::new(account);
        
        storage.set_account_state(&address, &account_state).unwrap();
        let retrieved = storage.get_account_state(&address).unwrap().unwrap();
        
        assert_eq!(retrieved.account.balance, 1_000_000);
    }
    
    #[test]
    fn test_state_root_calculation() {
        let storage = MemoryStateStorage::new();
        
        // Empty storage should have zero root
        let root1 = storage.calculate_state_root().unwrap();
        assert_eq!(root1, Hash256::zero());
        
        // Add an account
        let address = Address::from_public_key(&KeyPair::generate().public_key());
        let account = Account::new(1_000_000);
        let account_state = AccountState::new(account);
        storage.set_account_state(&address, &account_state).unwrap();
        
        let root2 = storage.calculate_state_root().unwrap();
        assert_ne!(root2, Hash256::zero());
        
        // Add another account - root should change
        let address2 = Address::from_public_key(&KeyPair::generate().public_key());
        let account2 = Account::new(2_000_000);
        let account_state2 = AccountState::new(account2);
        storage.set_account_state(&address2, &account_state2).unwrap();
        
        let root3 = storage.calculate_state_root().unwrap();
        assert_ne!(root3, root2);
    }
}
