//! State management and transitions

use crate::{Account, AccountState, StateStorage, StateRoot};
use lean_crypto::{Address, Hash256};
use std::collections::HashMap;
use std::sync::Arc;

/// State errors
#[derive(Debug, thiserror::Error)]
pub enum StateError {
    #[error("Account not found: {0}")]
    AccountNotFound(Address),
    
    #[error("Insufficient balance: have {have}, need {need}")]
    InsufficientBalance { have: u128, need: u128 },
    
    #[error("Invalid nonce: expected {expected}, got {actual}")]
    InvalidNonce { expected: u64, actual: u64 },
    
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("State root mismatch")]
    StateRootMismatch,
}

/// Global state manager
pub struct State {
    /// State storage backend
    storage: Arc<dyn StateStorage>,
    
    /// Cache of modified accounts (not yet committed)
    cache: std::sync::RwLock<HashMap<Address, AccountState>>,
    
    /// Current state root
    root: std::sync::RwLock<StateRoot>,
}

impl State {
    /// Create a new state with storage backend
    pub fn new(storage: Arc<dyn StateStorage>) -> Result<Self, StateError> {
        let root = storage.get_state_root()
            .map_err(|e| StateError::StorageError(e.to_string()))?;
        
        Ok(Self {
            storage,
            cache: std::sync::RwLock::new(HashMap::new()),
            root: std::sync::RwLock::new(root),
        })
    }
    
    /// Get an account
    pub fn get_account(&self, address: &Address) -> Result<Account, StateError> {
        // Check cache first
        {
            let cache = self.cache.read().unwrap();
            if let Some(account_state) = cache.get(address) {
                return Ok(account_state.account.clone());
            }
        }
        
        // Load from storage
        match self.storage.get_account(address) {
            Ok(Some(account)) => Ok(account),
            Ok(None) => Ok(Account::empty()),
            Err(e) => Err(StateError::StorageError(e.to_string())),
        }
    }
    
    /// Get account state (including storage)
    pub fn get_account_state(&self, address: &Address) -> Result<AccountState, StateError> {
        // Check cache first
        {
            let cache = self.cache.read().unwrap();
            if let Some(account_state) = cache.get(address) {
                return Ok(account_state.clone());
            }
        }
        
        // Load from storage
        match self.storage.get_account_state(address) {
            Ok(Some(state)) => Ok(state),
            Ok(None) => Ok(AccountState::empty()),
            Err(e) => Err(StateError::StorageError(e.to_string())),
        }
    }
    
    /// Set account (adds to cache)
    pub fn set_account(&self, address: Address, account: Account) -> Result<(), StateError> {
        let mut cache = self.cache.write().unwrap();
        
        // Get or create account state
        let mut account_state = if let Some(existing) = cache.get(&address) {
            existing.clone()
        } else {
            match self.storage.get_account_state(&address) {
                Ok(Some(state)) => state,
                Ok(None) => AccountState::new(account.clone()),
                Err(e) => return Err(StateError::StorageError(e.to_string())),
            }
        };
        
        account_state.account = account;
        cache.insert(address, account_state);
        
        Ok(())
    }
    
    /// Get balance
    pub fn get_balance(&self, address: &Address) -> Result<u128, StateError> {
        Ok(self.get_account(address)?.balance)
    }
    
    /// Get nonce
    pub fn get_nonce(&self, address: &Address) -> Result<u64, StateError> {
        Ok(self.get_account(address)?.nonce)
    }
    
    /// Add balance
    pub fn add_balance(&self, address: Address, amount: u128) -> Result<(), StateError> {
        let mut account = self.get_account(&address)?;
        account.add_balance(amount);
        self.set_account(address, account)
    }
    
    /// Subtract balance
    pub fn sub_balance(&self, address: Address, amount: u128) -> Result<(), StateError> {
        let mut account = self.get_account(&address)?;
        account.sub_balance(amount)
            .map_err(|_| StateError::InsufficientBalance {
                have: account.balance,
                need: amount,
            })?;
        self.set_account(address, account)
    }
    
    /// Transfer balance between accounts
    pub fn transfer(&self, from: Address, to: Address, amount: u128) -> Result<(), StateError> {
        self.sub_balance(from, amount)?;
        self.add_balance(to, amount)?;
        Ok(())
    }
    
    /// Increment nonce
    pub fn increment_nonce(&self, address: Address) -> Result<(), StateError> {
        let mut account = self.get_account(&address)?;
        account.increment_nonce();
        self.set_account(address, account)
    }
    
    /// Get contract storage
    pub fn get_storage(&self, address: &Address, key: &Hash256) -> Result<Hash256, StateError> {
        let account_state = self.get_account_state(address)?;
        Ok(account_state.get_storage(key))
    }
    
    /// Set contract storage
    pub fn set_storage(&self, address: Address, key: Hash256, value: Hash256) -> Result<(), StateError> {
        let mut cache = self.cache.write().unwrap();
        
        let mut account_state = if let Some(existing) = cache.get(&address) {
            existing.clone()
        } else {
            self.storage.get_account_state(&address)
                .map_err(|e| StateError::StorageError(e.to_string()))?
                .unwrap_or_else(AccountState::empty)
        };
        
        account_state.set_storage(key, value);
        cache.insert(address, account_state);
        
        Ok(())
    }
    
    /// Commit changes to storage
    pub fn commit(&self) -> Result<StateRoot, StateError> {
        let cache = self.cache.write().unwrap();
        
        // Write all cached accounts to storage
        for (address, account_state) in cache.iter() {
            self.storage.set_account_state(address, account_state)
                .map_err(|e| StateError::StorageError(e.to_string()))?;
        }
        
        // Calculate new state root
        let new_root = self.storage.calculate_state_root()
            .map_err(|e| StateError::StorageError(e.to_string()))?;
        
        self.storage.set_state_root(new_root)
            .map_err(|e| StateError::StorageError(e.to_string()))?;
        
        // Update root
        *self.root.write().unwrap() = new_root;
        
        Ok(new_root)
    }
    
    /// Clear cache (rollback uncommitted changes)
    pub fn clear_cache(&self) {
        self.cache.write().unwrap().clear();
    }
    
    /// Get current state root
    pub fn root(&self) -> StateRoot {
        *self.root.read().unwrap()
    }
    
    /// Check if account exists
    pub fn exists(&self, address: &Address) -> bool {
        match self.get_account(address) {
            Ok(account) => !account.is_empty(),
            Err(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MemoryStateStorage;
    use lean_crypto::KeyPair;
    
    #[test]
    fn test_state_balance_operations() {
        let storage = Arc::new(MemoryStateStorage::new());
        let state = State::new(storage).unwrap();
        
        let keypair = KeyPair::generate();
        let address = Address::from_public_key(keypair.public_key());
        
        // Initial balance should be 0
        assert_eq!(state.get_balance(&address).unwrap(), 0);
        
        // Add balance
        state.add_balance(address, 1_000_000).unwrap();
        assert_eq!(state.get_balance(&address).unwrap(), 1_000_000);
        
        // Subtract balance
        state.sub_balance(address, 500_000).unwrap();
        assert_eq!(state.get_balance(&address).unwrap(), 500_000);
        
        // Try to subtract more than available
        assert!(state.sub_balance(address, 1_000_000).is_err());
    }
    
    #[test]
    fn test_state_transfer() {
        let storage = Arc::new(MemoryStateStorage::new());
        let state = State::new(storage).unwrap();
        
        let from = Address::from_public_key(&KeyPair::generate().public_key());
        let to = Address::from_public_key(&KeyPair::generate().public_key());
        
        // Give 'from' some balance
        state.add_balance(from, 1_000_000).unwrap();
        
        // Transfer
        state.transfer(from, to, 600_000).unwrap();
        
        assert_eq!(state.get_balance(&from).unwrap(), 400_000);
        assert_eq!(state.get_balance(&to).unwrap(), 600_000);
    }
    
    #[test]
    fn test_state_nonce() {
        let storage = Arc::new(MemoryStateStorage::new());
        let state = State::new(storage).unwrap();
        
        let address = Address::from_public_key(&KeyPair::generate().public_key());
        
        assert_eq!(state.get_nonce(&address).unwrap(), 0);
        
        state.increment_nonce(address).unwrap();
        assert_eq!(state.get_nonce(&address).unwrap(), 1);
        
        state.increment_nonce(address).unwrap();
        assert_eq!(state.get_nonce(&address).unwrap(), 2);
    }
    
    #[test]
    fn test_state_commit() {
        let storage = Arc::new(MemoryStateStorage::new());
        let state = State::new(storage).unwrap();
        
        let address = Address::from_public_key(&KeyPair::generate().public_key());
        
        // Make changes
        state.add_balance(address, 1_000_000).unwrap();
        state.increment_nonce(address).unwrap();
        
        // Commit
        let root = state.commit().unwrap();
        assert!(!root.is_zero());
        
        // Verify changes are persisted
        assert_eq!(state.get_balance(&address).unwrap(), 1_000_000);
        assert_eq!(state.get_nonce(&address).unwrap(), 1);
    }
}
