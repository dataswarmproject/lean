//! Address generation and validation for LEAN CHAIN
//!
//! Addresses are derived from public keys using SHA-3 hashing

use crate::{Hash256, PublicKey};
use serde::{Deserialize, Serialize};
use std::fmt;

/// A LEAN CHAIN address (20 bytes, Ethereum-compatible)
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Address([u8; 20]);

impl Address {
    /// Create an Address from bytes
    pub fn from_bytes(bytes: [u8; 20]) -> Self {
        Self(bytes)
    }

    /// Create an Address from a slice
    pub fn from_slice(slice: &[u8]) -> Result<Self, AddressError> {
        if slice.len() != 20 {
            return Err(AddressError::InvalidLength(slice.len()));
        }
        let mut bytes = [0u8; 20];
        bytes.copy_from_slice(slice);
        Ok(Self(bytes))
    }

    /// Derive an address from a public key
    pub fn from_public_key(public_key: &PublicKey) -> Self {
        let hash = Hash256::hash(public_key.as_bytes());
        let hash_bytes = hash.as_bytes();
        let mut address_bytes = [0u8; 20];
        address_bytes.copy_from_slice(&hash_bytes[12..32]); // Take last 20 bytes
        Self(address_bytes)
    }

    /// Get the address as bytes
    pub fn as_bytes(&self) -> &[u8; 20] {
        &self.0
    }

    /// Convert to hex string with 0x prefix
    pub fn to_hex(&self) -> String {
        format!("0x{}", hex::encode(self.0))
    }

    /// Convert to hex string without 0x prefix
    pub fn to_hex_no_prefix(&self) -> String {
        hex::encode(self.0)
    }

    /// Parse from hex string (with or without 0x prefix)
    pub fn from_hex(s: &str) -> Result<Self, AddressError> {
        let s = s.strip_prefix("0x").unwrap_or(s);
        let bytes = hex::decode(s)
            .map_err(|e| AddressError::InvalidHex(e.to_string()))?;
        Self::from_slice(&bytes)
    }

    /// Convert to Base58 encoding (for readability)
    pub fn to_base58(&self) -> String {
        bs58::encode(self.0).into_string()
    }

    /// Parse from Base58 string
    pub fn from_base58(s: &str) -> Result<Self, AddressError> {
        let bytes = bs58::decode(s)
            .into_vec()
            .map_err(|e| AddressError::InvalidBase58(e.to_string()))?;
        Self::from_slice(&bytes)
    }

    /// Create a zero address
    pub fn zero() -> Self {
        Self([0u8; 20])
    }

    /// Check if address is zero
    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|&b| b == 0)
    }

    /// Checksum address (EIP-55 compatible)
    pub fn checksum(&self) -> String {
        let hex = self.to_hex_no_prefix();
        let hash = Hash256::hash(hex.as_bytes());
        let hash_hex = hash.to_hex();

        let mut result = String::from("0x");
        for (i, ch) in hex.chars().enumerate() {
            if ch.is_ascii_digit() {
                result.push(ch);
            } else {
                let hash_char = hash_hex.chars().nth(i).unwrap();
                if hash_char >= '8' {
                    result.push(ch.to_ascii_uppercase());
                } else {
                    result.push(ch);
                }
            }
        }
        result
    }
}

impl Default for Address {
    fn default() -> Self {
        Self::zero()
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Address({})", self.to_hex())
    }
}

impl AsRef<[u8]> for Address {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<[u8; 20]> for Address {
    fn from(bytes: [u8; 20]) -> Self {
        Self(bytes)
    }
}

/// Errors that can occur with addresses
#[derive(Debug, thiserror::Error)]
pub enum AddressError {
    #[error("Invalid address length: {0} (expected 20)")]
    InvalidLength(usize),

    #[error("Invalid hex encoding: {0}")]
    InvalidHex(String),

    #[error("Invalid Base58 encoding: {0}")]
    InvalidBase58(String),

    #[error("Checksum mismatch")]
    ChecksumMismatch,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::KeyPair;

    #[test]
    fn test_address_from_public_key() {
        let keypair = KeyPair::generate();
        let address = Address::from_public_key(keypair.public_key());
        assert_eq!(address.as_bytes().len(), 20);
    }

    #[test]
    fn test_address_hex() {
        let keypair = KeyPair::generate();
        let address = Address::from_public_key(keypair.public_key());
        let hex = address.to_hex();
        let decoded = Address::from_hex(&hex).unwrap();
        assert_eq!(address, decoded);
    }

    #[test]
    fn test_address_hex_no_prefix() {
        let address_bytes = [42u8; 20];
        let address = Address::from_bytes(address_bytes);
        let hex = address.to_hex_no_prefix();
        let decoded = Address::from_hex(&hex).unwrap();
        assert_eq!(address, decoded);
    }

    #[test]
    fn test_address_base58() {
        let keypair = KeyPair::generate();
        let address = Address::from_public_key(keypair.public_key());
        let base58 = address.to_base58();
        let decoded = Address::from_base58(&base58).unwrap();
        assert_eq!(address, decoded);
    }

    #[test]
    fn test_zero_address() {
        let address = Address::zero();
        assert!(address.is_zero());
        assert_eq!(address.to_hex(), "0x0000000000000000000000000000000000000000");
    }

    #[test]
    fn test_checksum() {
        let keypair = KeyPair::generate();
        let address = Address::from_public_key(keypair.public_key());
        let checksum = address.checksum();
        assert!(checksum.starts_with("0x"));
    }
}
