//! Cryptographic primitives for LEAN CHAIN
//!
//! This module provides core cryptographic functionality including:
//! - Ed25519 signature generation and verification
//! - SHA-3 (Keccak-256) hashing
//! - BLAKE3 for fast hashing
//! - Address generation and validation
//! - Key pair management

pub mod hash;
pub mod keypair;
pub mod signature;
pub mod address;

pub use hash::{Hash256, Hasher};
pub use keypair::{KeyPair, PublicKey, SecretKey};
pub use signature::{Signature, SignatureError};
pub use address::{Address, AddressError};

/// Result type for cryptographic operations
pub type Result<T> = std::result::Result<T, CryptoError>;

/// Errors that can occur in cryptographic operations
#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("Invalid signature: {0}")]
    InvalidSignature(String),

    #[error("Invalid public key: {0}")]
    InvalidPublicKey(String),

    #[error("Invalid secret key: {0}")]
    InvalidSecretKey(String),

    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Verification failed")]
    VerificationFailed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_and_verify() {
        let keypair = KeyPair::generate();
        let message = b"Hello, LEAN CHAIN!";

        let signature = keypair.sign(message);
        assert!(keypair.public_key().verify(message, &signature).is_ok());
    }

    #[test]
    fn test_invalid_signature() {
        let keypair1 = KeyPair::generate();
        let keypair2 = KeyPair::generate();
        let message = b"Test message";

        let signature = keypair1.sign(message);
        assert!(keypair2.public_key().verify(message, &signature).is_err());
    }

    #[test]
    fn test_address_generation() {
        let keypair = KeyPair::generate();
        let address = Address::from_public_key(keypair.public_key());

        assert_eq!(address.as_bytes().len(), 20);
    }

    #[test]
    fn test_hash_consistency() {
        let data = b"test data";
        let hash1 = Hash256::hash(data);
        let hash2 = Hash256::hash(data);

        assert_eq!(hash1, hash2);
    }
}
