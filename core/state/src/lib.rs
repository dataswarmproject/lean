//! State management for LEAN CHAIN
//!
//! This module provides account state management including:
//! - Account balances and nonces
//! - Smart contract storage
//! - State root calculation
//! - State transitions

pub mod account;
pub mod state;
pub mod storage;
pub mod executor;

pub use account::{Account, AccountState};
pub use state::{State, StateError};
pub use storage::{StateStorage, MemoryStateStorage, RocksDbStateStorage};
pub use executor::{StateExecutor, ExecutionResult};

use lean_crypto::{Address, Hash256};

/// Result type for state operations
pub type Result<T> = std::result::Result<T, StateError>;

/// State root type
pub type StateRoot = Hash256;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_module() {
        // Basic smoke test
        assert!(true);
    }
}
