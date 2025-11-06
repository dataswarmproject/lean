//! Transaction processing for LEAN CHAIN
//!
//! This module handles all transaction types including:
//! - Payment transactions (P2P transfers)
//! - Smart contract deployment and execution
//! - Validator staking operations
//! - Governance proposals and votes
//! - Cross-chain bridge operations

use lean_crypto::{Address, Hash256, PublicKey, Signature};
use serde::{Deserialize, Serialize};
use std::fmt;

pub mod pool;
pub mod types;
pub mod validation;

pub use pool::TransactionPool;
pub use types::*;
pub use validation::*;

/// Result type for transaction operations
pub type Result<T> = std::result::Result<T, TransactionError>;

/// Transaction errors
#[derive(Debug, thiserror::Error)]
pub enum TransactionError {
    #[error("Invalid signature")]
    InvalidSignature,

    #[error("Invalid nonce: expected {expected}, got {actual}")]
    InvalidNonce { expected: u64, actual: u64 },

    #[error("Insufficient balance: have {have}, need {need}")]
    InsufficientBalance { have: u128, need: u128 },

    #[error("Gas limit too low: minimum {minimum}, provided {provided}")]
    GasLimitTooLow { minimum: u64, provided: u64 },

    #[error("Gas price too low: minimum {minimum}, provided {provided}")]
    GasPriceTooLow { minimum: u64, provided: u64 },

    #[error("Transaction too large: {size} bytes (max {max})")]
    TransactionTooLarge { size: usize, max: usize },

    #[error("Invalid transaction type: {0}")]
    InvalidType(u8),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Replay attack detected")]
    ReplayAttack,

    #[error("Contract deployment failed: {0}")]
    DeploymentFailed(String),

    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
}

/// A signed transaction
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Transaction {
    /// Transaction nonce (for replay protection)
    pub nonce: u64,

    /// Sender address
    pub from: Address,

    /// Transaction type and payload
    pub tx_type: TransactionType,

    /// Gas limit for execution
    pub gas_limit: u64,

    /// Gas price (in smallest unit)
    pub gas_price: u64,

    /// Transaction signature
    pub signature: Signature,

    /// Transaction hash (computed)
    #[serde(skip)]
    hash: Option<Hash256>,
}

impl Transaction {
    /// Create a new transaction (unsigned)
    pub fn new(
        nonce: u64,
        from: Address,
        tx_type: TransactionType,
        gas_limit: u64,
        gas_price: u64,
    ) -> Self {
        Self {
            nonce,
            from,
            tx_type,
            gas_limit,
            gas_price,
            signature: Signature::from_bytes([0u8; 64]),
            hash: None,
        }
    }

    /// Sign the transaction
    pub fn sign(&mut self, keypair: &lean_crypto::KeyPair) {
        let message = self.signing_message();
        self.signature = keypair.sign(&message);
        self.hash = Some(self.compute_hash());
    }

    /// Get the signing message (all fields except signature)
    pub fn signing_message(&self) -> Vec<u8> {
        let mut message = Vec::new();
        message.extend_from_slice(&self.nonce.to_le_bytes());
        message.extend_from_slice(self.from.as_bytes());
        message.extend_from_slice(&bincode::serialize(&self.tx_type).unwrap());
        message.extend_from_slice(&self.gas_limit.to_le_bytes());
        message.extend_from_slice(&self.gas_price.to_le_bytes());
        message
    }

    /// Compute transaction hash
    pub fn compute_hash(&self) -> Hash256 {
        let mut hasher = lean_crypto::Hasher::new();
        hasher.update(&self.signing_message());
        hasher.update(self.signature.as_bytes());
        hasher.finalize()
    }

    /// Get transaction hash (computes if not cached)
    pub fn hash(&mut self) -> Hash256 {
        if let Some(hash) = self.hash {
            hash
        } else {
            let hash = self.compute_hash();
            self.hash = Some(hash);
            hash
        }
    }

    /// Verify transaction signature
    pub fn verify_signature(&self) -> Result<()> {
        let message = self.signing_message();
        let public_key = self.recover_public_key()?;
        public_key
            .verify(&message, &self.signature)
            .map_err(|_| TransactionError::InvalidSignature)
    }

    /// Recover public key from signature (simplified - assumes public key derivation)
    fn recover_public_key(&self) -> Result<PublicKey> {
        // In a real implementation, we would recover the public key from the signature
        // For now, we'll need to store or pass the public key separately
        // This is a placeholder that should be implemented with proper ECDSA recovery
        Err(TransactionError::InvalidSignature)
    }

    /// Get transaction size in bytes
    pub fn size(&self) -> usize {
        bincode::serialize(self).unwrap_or_default().len()
    }

    /// Calculate transaction cost (gas_limit * gas_price)
    pub fn cost(&self) -> u128 {
        self.gas_limit as u128 * self.gas_price as u128
    }

    /// Get the value being transferred (if applicable)
    pub fn value(&self) -> u128 {
        match &self.tx_type {
            TransactionType::Payment(payment) => payment.amount,
            TransactionType::ContractCall(call) => call.value,
            _ => 0,
        }
    }

    /// Check if this is a contract creation transaction
    pub fn is_contract_creation(&self) -> bool {
        matches!(self.tx_type, TransactionType::ContractDeploy(_))
    }

    /// Serialize to bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        bincode::serialize(self)
            .map_err(|e| TransactionError::SerializationError(e.to_string()))
    }

    /// Deserialize from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        bincode::deserialize(bytes)
            .map_err(|e| TransactionError::SerializationError(e.to_string()))
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Transaction(from: {}, type: {:?}, nonce: {}, gas: {})",
            self.from, self.tx_type, self.nonce, self.gas_limit
        )
    }
}

/// Transaction receipt (after execution)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionReceipt {
    /// Transaction hash
    pub tx_hash: Hash256,

    /// Block number where transaction was included
    pub block_number: u64,

    /// Block hash
    pub block_hash: Hash256,

    /// Transaction index in block
    pub transaction_index: u32,

    /// Sender address
    pub from: Address,

    /// Recipient address (if applicable)
    pub to: Option<Address>,

    /// Contract address (if contract creation)
    pub contract_address: Option<Address>,

    /// Gas used
    pub gas_used: u64,

    /// Transaction success status
    pub status: bool,

    /// Logs emitted
    pub logs: Vec<Log>,

    /// Output data
    pub output: Vec<u8>,
}

/// Event log emitted by smart contracts
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Log {
    /// Contract address that emitted the log
    pub address: Address,

    /// Log topics (indexed parameters)
    pub topics: Vec<Hash256>,

    /// Log data (non-indexed parameters)
    pub data: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use lean_crypto::KeyPair;

    #[test]
    fn test_transaction_creation() {
        let keypair = KeyPair::generate();
        let from = Address::from_public_key(keypair.public_key());
        let to = Address::from_public_key(&KeyPair::generate().public_key());

        let payment = PaymentTransaction {
            to,
            amount: 1_000_000,
        };

        let mut tx = Transaction::new(
            0,
            from,
            TransactionType::Payment(payment),
            21_000,
            1,
        );

        tx.sign(&keypair);
        assert_ne!(tx.hash(), Hash256::zero());
    }

    #[test]
    fn test_transaction_serialization() {
        let keypair = KeyPair::generate();
        let from = Address::from_public_key(keypair.public_key());
        let to = Address::from_public_key(&KeyPair::generate().public_key());

        let payment = PaymentTransaction {
            to,
            amount: 1_000_000,
        };

        let mut tx = Transaction::new(
            0,
            from,
            TransactionType::Payment(payment),
            21_000,
            1,
        );

        tx.sign(&keypair);

        let bytes = tx.to_bytes().unwrap();
        let decoded = Transaction::from_bytes(&bytes).unwrap();

        assert_eq!(tx, decoded);
    }

    #[test]
    fn test_transaction_cost() {
        let from = Address::zero();
        let to = Address::zero();

        let payment = PaymentTransaction {
            to,
            amount: 1_000_000,
        };

        let tx = Transaction::new(
            0,
            from,
            TransactionType::Payment(payment),
            21_000,
            10,
        );

        assert_eq!(tx.cost(), 210_000);
    }
}
