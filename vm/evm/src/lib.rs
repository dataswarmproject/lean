//! EVM (Ethereum Virtual Machine) implementation for LEAN CHAIN
//!
//! Provides full EVM compatibility for executing Solidity smart contracts

pub mod opcode;
pub mod stack;
pub mod memory;
pub mod gas;
pub mod interpreter;
pub mod context;

pub use opcode::{OpCode, OPCODES};
pub use stack::{Stack, StackError};
pub use memory::{Memory, MemoryError};
pub use gas::{Gas, GasCost};
pub use interpreter::{Interpreter, InterpreterResult};
pub use context::{ExecutionContext, CallContext};

/// Result type for EVM operations
pub type Result<T> = std::result::Result<T, EvmError>;

/// EVM errors
#[derive(Debug, thiserror::Error)]
pub enum EvmError {
    #[error("Stack error: {0}")]
    StackError(#[from] StackError),
    
    #[error("Memory error: {0}")]
    MemoryError(#[from] MemoryError),
    
    #[error("Out of gas")]
    OutOfGas,
    
    #[error("Invalid opcode: {0:#x}")]
    InvalidOpcode(u8),
    
    #[error("Invalid jump destination")]
    InvalidJump,
    
    #[error("Execution reverted: {0}")]
    Revert(String),
    
    #[error("Contract creation failed")]
    ContractCreationFailed,
    
    #[error("Call failed")]
    CallFailed,
    
    #[error("Invalid contract code")]
    InvalidCode,
}

/// EVM version
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvmVersion {
    /// Frontier (first version)
    Frontier,
    /// Homestead
    Homestead,
    /// Byzantium
    Byzantium,
    /// Constantinople
    Constantinople,
    /// Istanbul
    Istanbul,
    /// Berlin
    Berlin,
    /// London (current default)
    London,
    /// Shanghai
    Shanghai,
}

impl Default for EvmVersion {
    fn default() -> Self {
        Self::London
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_evm_version() {
        let version = EvmVersion::default();
        assert_eq!(version, EvmVersion::London);
    }
}
