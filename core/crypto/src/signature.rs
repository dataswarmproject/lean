//! Digital signatures for LEAN CHAIN
//!
//! Provides Ed25519 signature creation and verification

use serde::{Deserialize, Serialize};
use std::fmt;

/// An Ed25519 signature
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Signature([u8; 64]);

impl Signature {
    /// Create a Signature from bytes
    pub fn from_bytes(bytes: [u8; 64]) -> Self {
        Self(bytes)
    }

    /// Create a Signature from a slice
    pub fn from_slice(slice: &[u8]) -> Result<Self, SignatureError> {
        if slice.len() != 64 {
            return Err(SignatureError::InvalidLength(slice.len()));
        }
        let mut bytes = [0u8; 64];
        bytes.copy_from_slice(slice);
        Ok(Self(bytes))
    }

    /// Get the signature as bytes
    pub fn as_bytes(&self) -> &[u8; 64] {
        &self.0
    }

    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }

    /// Parse from hex string
    pub fn from_hex(s: &str) -> Result<Self, SignatureError> {
        let bytes = hex::decode(s)
            .map_err(|e| SignatureError::InvalidHex(e.to_string()))?;
        Self::from_slice(&bytes)
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Signature({}...)", &self.to_hex()[..16])
    }
}

impl AsRef<[u8]> for Signature {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

/// Errors that can occur with signatures
#[derive(Debug, thiserror::Error)]
pub enum SignatureError {
    #[error("Invalid signature length: {0} (expected 64)")]
    InvalidLength(usize),

    #[error("Invalid hex encoding: {0}")]
    InvalidHex(String),

    #[error("Signature verification failed")]
    VerificationFailed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature_hex() {
        let sig_bytes = [42u8; 64];
        let sig = Signature::from_bytes(sig_bytes);
        let hex = sig.to_hex();
        let decoded = Signature::from_hex(&hex).unwrap();
        assert_eq!(sig, decoded);
    }

    #[test]
    fn test_signature_invalid_length() {
        let bytes = [0u8; 32];
        assert!(Signature::from_slice(&bytes).is_err());
    }
}
