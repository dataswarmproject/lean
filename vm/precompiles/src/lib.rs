//! Precompiled contracts for LEAN CHAIN EVM
//!
//! Provides optimized implementations of common cryptographic operations

use std::collections::HashMap;

pub mod ecrecover;
pub mod sha256;
pub mod ripemd160;
pub mod identity;
pub mod modexp;

/// Precompiled contract result
pub type Result<T> = std::result::Result<T, PrecompileError>;

/// Precompiled contract errors
#[derive(Debug, thiserror::Error)]
pub enum PrecompileError {
    #[error("Invalid input length: expected {expected}, got {actual}")]
    InvalidInputLength { expected: usize, actual: usize },
    
    #[error("Invalid input data: {0}")]
    InvalidInput(String),
    
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
    
    #[error("Out of gas")]
    OutOfGas,
}

/// Precompiled contract trait
pub trait Precompile: Send + Sync {
    /// Execute the precompiled contract
    fn execute(&self, input: &[u8], gas_limit: u64) -> Result<(Vec<u8>, u64)>;
    
    /// Get the gas cost for the input
    fn gas_cost(&self, input: &[u8]) -> u64;
}

/// Precompiled contract registry
pub struct PrecompileRegistry {
    contracts: HashMap<u64, Box<dyn Precompile>>,
}

impl PrecompileRegistry {
    /// Create a new registry with default precompiles
    pub fn new() -> Self {
        let mut registry = Self {
            contracts: HashMap::new(),
        };
        
        // Register default precompiles at their standard addresses
        registry.register(0x01, Box::new(ecrecover::EcRecover));
        registry.register(0x02, Box::new(sha256::Sha256));
        registry.register(0x03, Box::new(ripemd160::Ripemd160));
        registry.register(0x04, Box::new(identity::Identity));
        registry.register(0x05, Box::new(modexp::ModExp));
        
        registry
    }
    
    /// Register a precompiled contract at an address
    pub fn register(&mut self, address: u64, contract: Box<dyn Precompile>) {
        self.contracts.insert(address, contract);
    }
    
    /// Check if an address is a precompiled contract
    pub fn is_precompile(&self, address: u64) -> bool {
        self.contracts.contains_key(&address)
    }
    
    /// Execute a precompiled contract
    pub fn execute(&self, address: u64, input: &[u8], gas_limit: u64) -> Result<(Vec<u8>, u64)> {
        match self.contracts.get(&address) {
            Some(contract) => contract.execute(input, gas_limit),
            None => Err(PrecompileError::ExecutionFailed(
                format!("No precompile at address {:#x}", address)
            )),
        }
    }
    
    /// Get gas cost for a precompiled contract call
    pub fn gas_cost(&self, address: u64, input: &[u8]) -> Option<u64> {
        self.contracts.get(&address).map(|c| c.gas_cost(input))
    }
}

impl Default for PrecompileRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_registry() {
        let registry = PrecompileRegistry::new();
        
        // Check standard precompiles are registered
        assert!(registry.is_precompile(0x01)); // ECRECOVER
        assert!(registry.is_precompile(0x02)); // SHA256
        assert!(registry.is_precompile(0x03)); // RIPEMD160
        assert!(registry.is_precompile(0x04)); // IDENTITY
        assert!(registry.is_precompile(0x05)); // MODEXP
        
        // Check non-existent address
        assert!(!registry.is_precompile(0xFF));
    }
}
