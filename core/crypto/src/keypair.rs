//! Key pair management for LEAN CHAIN
//!
//! Provides Ed25519 key pair generation and management

use crate::{Signature, CryptoError};
use ed25519_dalek::{Signer, Verifier, SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::fmt;

/// A public key (Ed25519)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PublicKey([u8; 32]);

impl PublicKey {
    /// Create a PublicKey from bytes
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Create a PublicKey from a slice
    pub fn from_slice(slice: &[u8]) -> Result<Self, CryptoError> {
        if slice.len() != 32 {
            return Err(CryptoError::InvalidPublicKey(
                format!("Invalid length: {} (expected 32)", slice.len())
            ));
        }
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(slice);
        Ok(Self(bytes))
    }

    /// Get the public key as bytes
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }

    /// Parse from hex string
    pub fn from_hex(s: &str) -> Result<Self, CryptoError> {
        let bytes = hex::decode(s)
            .map_err(|e| CryptoError::InvalidPublicKey(format!("Invalid hex: {}", e)))?;
        Self::from_slice(&bytes)
    }

    /// Verify a signature
    pub fn verify(&self, message: &[u8], signature: &Signature) -> Result<(), CryptoError> {
        let verifying_key = VerifyingKey::from_bytes(&self.0)
            .map_err(|e| CryptoError::InvalidPublicKey(e.to_string()))?;

        let sig = ed25519_dalek::Signature::from_bytes(signature.as_bytes());

        verifying_key.verify(message, &sig)
            .map_err(|_| CryptoError::VerificationFailed)
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PublicKey({})", &self.to_hex()[..8])
    }
}

/// A secret key (Ed25519)
pub struct SecretKey([u8; 32]);

impl SecretKey {
    /// Create a SecretKey from bytes
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Create a SecretKey from a slice
    pub fn from_slice(slice: &[u8]) -> Result<Self, CryptoError> {
        if slice.len() != 32 {
            return Err(CryptoError::InvalidSecretKey(
                format!("Invalid length: {} (expected 32)", slice.len())
            ));
        }
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(slice);
        Ok(Self(bytes))
    }

    /// Get the secret key as bytes
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Convert to hex string (use with caution!)
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }

    /// Parse from hex string
    pub fn from_hex(s: &str) -> Result<Self, CryptoError> {
        let bytes = hex::decode(s)
            .map_err(|e| CryptoError::InvalidSecretKey(format!("Invalid hex: {}", e)))?;
        Self::from_slice(&bytes)
    }
}

impl Drop for SecretKey {
    fn drop(&mut self) {
        // Zero out the secret key on drop for security
        self.0.iter_mut().for_each(|b| *b = 0);
    }
}

impl fmt::Debug for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SecretKey([REDACTED])")
    }
}

/// A key pair (public and secret keys)
pub struct KeyPair {
    secret: SecretKey,
    public: PublicKey,
}

impl KeyPair {
    /// Generate a new random key pair
    pub fn generate() -> Self {
        let mut rng = OsRng;
        let signing_key = SigningKey::generate(&mut rng);
        let verifying_key = signing_key.verifying_key();

        Self {
            secret: SecretKey(signing_key.to_bytes()),
            public: PublicKey(verifying_key.to_bytes()),
        }
    }

    /// Create a KeyPair from a secret key
    pub fn from_secret_key(secret: SecretKey) -> Result<Self, CryptoError> {
        let signing_key = SigningKey::from_bytes(&secret.0);
        let verifying_key = signing_key.verifying_key();

        Ok(Self {
            secret,
            public: PublicKey(verifying_key.to_bytes()),
        })
    }

    /// Get the public key
    pub fn public_key(&self) -> &PublicKey {
        &self.public
    }

    /// Get the secret key
    pub fn secret_key(&self) -> &SecretKey {
        &self.secret
    }

    /// Sign a message
    pub fn sign(&self, message: &[u8]) -> Signature {
        let signing_key = SigningKey::from_bytes(&self.secret.0);
        let signature = signing_key.sign(message);
        Signature::from_bytes(signature.to_bytes())
    }

    /// Sign and verify a message (for testing)
    pub fn sign_and_verify(&self, message: &[u8]) -> Result<Signature, CryptoError> {
        let signature = self.sign(message);
        self.public.verify(message, &signature)?;
        Ok(signature)
    }
}

impl fmt::Debug for KeyPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KeyPair")
            .field("public", &self.public)
            .field("secret", &"[REDACTED]")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let keypair = KeyPair::generate();
        assert_eq!(keypair.public_key().as_bytes().len(), 32);
        assert_eq!(keypair.secret_key().as_bytes().len(), 32);
    }

    #[test]
    fn test_sign_and_verify() {
        let keypair = KeyPair::generate();
        let message = b"test message";

        let signature = keypair.sign(message);
        assert!(keypair.public_key().verify(message, &signature).is_ok());
    }

    #[test]
    fn test_verify_wrong_message() {
        let keypair = KeyPair::generate();
        let message1 = b"message 1";
        let message2 = b"message 2";

        let signature = keypair.sign(message1);
        assert!(keypair.public_key().verify(message2, &signature).is_err());
    }

    #[test]
    fn test_verify_wrong_signature() {
        let keypair1 = KeyPair::generate();
        let keypair2 = KeyPair::generate();
        let message = b"test message";

        let signature = keypair1.sign(message);
        assert!(keypair2.public_key().verify(message, &signature).is_err());
    }

    #[test]
    fn test_public_key_hex() {
        let keypair = KeyPair::generate();
        let hex = keypair.public_key().to_hex();
        let decoded = PublicKey::from_hex(&hex).unwrap();
        assert_eq!(keypair.public_key(), &decoded);
    }
}
