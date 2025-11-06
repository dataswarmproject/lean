//! Bridge validator management

use crate::{BridgeError, Result};
use lean_crypto::Address;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Multisig threshold configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultisigThreshold {
    /// Required signatures (numerator)
    pub required: usize,
    /// Total validators (denominator)
    pub total: usize,
}

impl MultisigThreshold {
    pub fn new(required: usize, total: usize) -> Result<Self> {
        if required == 0 || required > total {
            return Err(BridgeError::InsufficientSignatures { got: required, required: total });
        }
        Ok(Self { required, total })
    }

    /// Check if signatures meet threshold
    pub fn is_met(&self, signatures: usize) -> bool {
        signatures >= self.required
    }

    /// Calculate percentage (basis points)
    pub fn percentage_bps(&self) -> u16 {
        ((self.required as u128 * 10000) / self.total as u128) as u16
    }
}

/// Bridge validator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeValidator {
    pub address: Address,
    pub stake: u128,
    pub is_active: bool,
    pub reputation: u64,
}

/// Validator set for bridge
pub struct ValidatorSet {
    validators: HashSet<Address>,
    threshold: MultisigThreshold,
    stakes: std::collections::HashMap<Address, u128>,
}

impl ValidatorSet {
    pub fn new(threshold: MultisigThreshold) -> Self {
        Self {
            validators: HashSet::new(),
            threshold,
            stakes: std::collections::HashMap::new(),
        }
    }

    /// Add validator
    pub fn add_validator(&mut self, address: Address, stake: u128) {
        self.validators.insert(address);
        self.stakes.insert(address, stake);
    }

    /// Remove validator
    pub fn remove_validator(&mut self, address: &Address) {
        self.validators.remove(address);
        self.stakes.remove(address);
    }

    /// Check if address is validator
    pub fn is_validator(&self, address: &Address) -> bool {
        self.validators.contains(address)
    }

    /// Get validator count
    pub fn count(&self) -> usize {
        self.validators.len()
    }

    /// Get required signatures
    pub fn required_signatures(&self) -> usize {
        self.threshold.required
    }

    /// Verify signatures meet threshold
    pub fn verify_threshold(&self, signatures: &[Vec<u8>]) -> Result<bool> {
        if !self.threshold.is_met(signatures.len()) {
            return Err(BridgeError::InsufficientSignatures {
                got: signatures.len(),
                required: self.threshold.required,
            });
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threshold() {
        let threshold = MultisigThreshold::new(2, 3).unwrap();
        assert!(threshold.is_met(2));
        assert!(threshold.is_met(3));
        assert!(!threshold.is_met(1));
    }

    #[test]
    fn test_validator_set() {
        let threshold = MultisigThreshold::new(2, 3).unwrap();
        let mut set = ValidatorSet::new(threshold);
        
        let addr = Address::from([1u8; 20]);
        set.add_validator(addr, 1000);
        
        assert!(set.is_validator(&addr));
        assert_eq!(set.count(), 1);
        assert_eq!(set.required_signatures(), 2);
    }
}
