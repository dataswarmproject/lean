//! Account state representation

use lean_crypto::{Address, Hash256};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// An account in the state
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Account {
    /// Account balance (in smallest unit)
    pub balance: u128,
    
    /// Transaction nonce (for replay protection)
    pub nonce: u64,
    
    /// Contract code hash (if this is a contract account)
    pub code_hash: Option<Hash256>,
    
    /// Storage root (for contract storage)
    pub storage_root: Hash256,
}

impl Account {
    /// Create a new account with initial balance
    pub fn new(balance: u128) -> Self {
        Self {
            balance,
            nonce: 0,
            code_hash: None,
            storage_root: Hash256::zero(),
        }
    }
    
    /// Create an empty account
    pub fn empty() -> Self {
        Self::new(0)
    }
    
    /// Check if account is empty
    pub fn is_empty(&self) -> bool {
        self.balance == 0 && self.nonce == 0 && self.code_hash.is_none()
    }
    
    /// Check if account is a contract
    pub fn is_contract(&self) -> bool {
        self.code_hash.is_some()
    }
    
    /// Increment nonce
    pub fn increment_nonce(&mut self) {
        self.nonce += 1;
    }
    
    /// Add to balance
    pub fn add_balance(&mut self, amount: u128) {
        self.balance = self.balance.saturating_add(amount);
    }
    
    /// Subtract from balance (returns error if insufficient)
    pub fn sub_balance(&mut self, amount: u128) -> Result<(), String> {
        if self.balance < amount {
            return Err(format!("Insufficient balance: have {}, need {}", self.balance, amount));
        }
        self.balance -= amount;
        Ok(())
    }
}

impl Default for Account {
    fn default() -> Self {
        Self::empty()
    }
}

/// Complete account state including storage
#[derive(Clone, Debug)]
pub struct AccountState {
    /// The account
    pub account: Account,
    
    /// Contract storage (key-value pairs)
    pub storage: HashMap<Hash256, Hash256>,
    
    /// Contract code (if any)
    pub code: Option<Vec<u8>>,
}

impl AccountState {
    /// Create a new account state
    pub fn new(account: Account) -> Self {
        Self {
            account,
            storage: HashMap::new(),
            code: None,
        }
    }
    
    /// Create an empty account state
    pub fn empty() -> Self {
        Self::new(Account::empty())
    }
    
    /// Get storage value
    pub fn get_storage(&self, key: &Hash256) -> Hash256 {
        self.storage.get(key).copied().unwrap_or(Hash256::zero())
    }
    
    /// Set storage value
    pub fn set_storage(&mut self, key: Hash256, value: Hash256) {
        if value.is_zero() {
            self.storage.remove(&key);
        } else {
            self.storage.insert(key, value);
        }
    }
    
    /// Set contract code
    pub fn set_code(&mut self, code: Vec<u8>) {
        let code_hash = Hash256::hash(&code);
        self.account.code_hash = Some(code_hash);
        self.code = Some(code);
    }
    
    /// Calculate storage root (simplified - just hash all storage)
    pub fn calculate_storage_root(&self) -> Hash256 {
        if self.storage.is_empty() {
            return Hash256::zero();
        }
        
        let mut hasher = lean_crypto::Hasher::new();
        let mut keys: Vec<_> = self.storage.keys().collect();
        keys.sort();
        
        for key in keys {
            hasher.update(key.as_bytes());
            hasher.update(self.storage[key].as_bytes());
        }
        
        hasher.finalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_account_creation() {
        let account = Account::new(1_000_000);
        assert_eq!(account.balance, 1_000_000);
        assert_eq!(account.nonce, 0);
        assert!(!account.is_contract());
    }
    
    #[test]
    fn test_balance_operations() {
        let mut account = Account::new(1_000);
        
        account.add_balance(500);
        assert_eq!(account.balance, 1_500);
        
        assert!(account.sub_balance(200).is_ok());
        assert_eq!(account.balance, 1_300);
        
        assert!(account.sub_balance(2_000).is_err());
    }
    
    #[test]
    fn test_nonce_increment() {
        let mut account = Account::new(0);
        assert_eq!(account.nonce, 0);
        
        account.increment_nonce();
        assert_eq!(account.nonce, 1);
        
        account.increment_nonce();
        assert_eq!(account.nonce, 2);
    }
    
    #[test]
    fn test_account_state_storage() {
        let mut state = AccountState::empty();
        
        let key = Hash256::hash(b"test_key");
        let value = Hash256::hash(b"test_value");
        
        state.set_storage(key, value);
        assert_eq!(state.get_storage(&key), value);
        
        // Test storage root changes
        let root1 = state.calculate_storage_root();
        assert!(!root1.is_zero());
        
        let key2 = Hash256::hash(b"another_key");
        let value2 = Hash256::hash(b"another_value");
        state.set_storage(key2, value2);
        
        let root2 = state.calculate_storage_root();
        assert_ne!(root1, root2);
    }
    
    #[test]
    fn test_contract_code() {
        let mut state = AccountState::empty();
        assert!(!state.account.is_contract());
        
        let code = vec![0x60, 0x80, 0x60, 0x40]; // Some bytecode
        state.set_code(code.clone());
        
        assert!(state.account.is_contract());
        assert_eq!(state.code, Some(code));
    }
}
