//! Hashing functions for LEAN CHAIN
//!
//! Provides SHA-3 (Keccak-256) and BLAKE3 hashing algorithms

use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use std::fmt;

/// A 256-bit hash value
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Hash256([u8; 32]);

impl Hash256 {
    /// Create a new Hash256 from a byte array
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Create a Hash256 from a slice (panics if not 32 bytes)
    pub fn from_slice(slice: &[u8]) -> Result<Self, String> {
        if slice.len() != 32 {
            return Err(format!("Invalid hash length: {} (expected 32)", slice.len()));
        }
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(slice);
        Ok(Self(bytes))
    }

    /// Compute SHA-3 (Keccak-256) hash of data
    pub fn hash(data: &[u8]) -> Self {
        let mut hasher = Keccak256::new();
        hasher.update(data);
        let result = hasher.finalize();
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&result);
        Self(bytes)
    }

    /// Compute BLAKE3 hash of data (faster for large data)
    pub fn blake3(data: &[u8]) -> Self {
        let hash = blake3::hash(data);
        Self(*hash.as_bytes())
    }

    /// Get the hash as a byte slice
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }

    /// Parse from hex string
    pub fn from_hex(s: &str) -> Result<Self, String> {
        let bytes = hex::decode(s).map_err(|e| format!("Invalid hex: {}", e))?;
        Self::from_slice(&bytes)
    }

    /// Create a zero hash
    pub fn zero() -> Self {
        Self([0u8; 32])
    }

    /// Check if hash is zero
    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|&b| b == 0)
    }
}

impl Default for Hash256 {
    fn default() -> Self {
        Self::zero()
    }
}

impl fmt::Display for Hash256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl fmt::Debug for Hash256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hash256({})", &self.to_hex()[..8])
    }
}

impl AsRef<[u8]> for Hash256 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<[u8; 32]> for Hash256 {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

/// Hasher for incremental hashing
pub struct Hasher {
    inner: Keccak256,
}

impl Hasher {
    /// Create a new hasher
    pub fn new() -> Self {
        Self {
            inner: Keccak256::new(),
        }
    }

    /// Update the hasher with data
    pub fn update(&mut self, data: &[u8]) {
        self.inner.update(data);
    }

    /// Finalize and return the hash
    pub fn finalize(self) -> Hash256 {
        let result = self.inner.finalize();
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&result);
        Hash256(bytes)
    }
}

impl Default for Hasher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_consistency() {
        let data = b"test data";
        let hash1 = Hash256::hash(data);
        let hash2 = Hash256::hash(data);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_hash_difference() {
        let hash1 = Hash256::hash(b"data1");
        let hash2 = Hash256::hash(b"data2");
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_hex_conversion() {
        let hash = Hash256::hash(b"test");
        let hex = hash.to_hex();
        let decoded = Hash256::from_hex(&hex).unwrap();
        assert_eq!(hash, decoded);
    }

    #[test]
    fn test_incremental_hashing() {
        let data1 = b"hello";
        let data2 = b"world";

        let mut hasher = Hasher::new();
        hasher.update(data1);
        hasher.update(data2);
        let hash1 = hasher.finalize();

        let hash2 = Hash256::hash(b"helloworld");

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_blake3_hash() {
        let data = b"test data";
        let hash = Hash256::blake3(data);
        assert!(!hash.is_zero());
    }
}
