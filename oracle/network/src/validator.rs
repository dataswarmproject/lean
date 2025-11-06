//! Oracle validator set management

use crate::{PriceData, OracleError, Result};
use lean_crypto::Address;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Oracle validator information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleValidator {
    pub address: Address,
    pub stake: u128,
    pub reputation: u64,
    pub is_active: bool,
}

/// Validator set for oracles
pub struct ValidatorSet {
    validators: HashMap<Address, OracleValidator>,
    min_stake: u128,
}

impl ValidatorSet {
    pub fn new(min_stake: u128) -> Self {
        Self {
            validators: HashMap::new(),
            min_stake,
        }
    }

    /// Add oracle validator
    pub fn add_validator(&mut self, validator: OracleValidator) -> Result<()> {
        if validator.stake < self.min_stake {
            return Err(OracleError::UnauthorizedOracle(
                "Insufficient stake".to_string(),
            ));
        }
        self.validators.insert(validator.address, validator);
        Ok(())
    }

    /// Check if oracle is authorized
    pub fn is_authorized(&self, oracle: &Address) -> bool {
        self.validators
            .get(oracle)
            .map(|v| v.is_active && v.stake >= self.min_stake)
            .unwrap_or(false)
    }

    /// Verify price data signature
    pub fn verify_price(&self, price_data: &PriceData) -> Result<bool> {
        if !self.is_authorized(&price_data.oracle) {
            return Err(OracleError::UnauthorizedOracle(
                hex::encode(price_data.oracle.as_bytes()),
            ));
        }

        // In production, verify signature over (asset, price, timestamp)
        // For now, just check authorization
        Ok(true)
    }

    /// Filter valid prices from authorized oracles
    pub fn filter_valid<'a>(
        &self,
        prices: &'a [PriceData],
    ) -> Vec<&'a PriceData> {
        prices
            .iter()
            .filter(|p| self.is_authorized(&p.oracle))
            .collect()
    }

    /// Update validator reputation
    pub fn update_reputation(&mut self, oracle: &Address, change: i64) {
        if let Some(validator) = self.validators.get_mut(oracle) {
            if change > 0 {
                validator.reputation = validator.reputation.saturating_add(change as u64);
            } else {
                validator.reputation = validator.reputation.saturating_sub((-change) as u64);
            }
        }
    }

    /// Slash validator stake
    pub fn slash(&mut self, oracle: &Address, amount: u128) -> Result<u128> {
        let validator = self
            .validators
            .get_mut(oracle)
            .ok_or_else(|| OracleError::UnauthorizedOracle(hex::encode(oracle.as_bytes())))?;

        let slashed = amount.min(validator.stake);
        validator.stake -= slashed;

        if validator.stake < self.min_stake {
            validator.is_active = false;
        }

        Ok(slashed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_address(n: u8) -> Address {
        Address::from([n; 20])
    }

    #[test]
    fn test_validator_set() {
        let mut set = ValidatorSet::new(1000);
        
        let validator = OracleValidator {
            address: test_address(1),
            stake: 5000,
            reputation: 100,
            is_active: true,
        };

        set.add_validator(validator).unwrap();
        assert!(set.is_authorized(&test_address(1)));
        assert!(!set.is_authorized(&test_address(2)));
    }

    #[test]
    fn test_insufficient_stake() {
        let mut set = ValidatorSet::new(1000);
        
        let validator = OracleValidator {
            address: test_address(1),
            stake: 500, // Below minimum
            reputation: 100,
            is_active: true,
        };

        assert!(set.add_validator(validator).is_err());
    }

    #[test]
    fn test_slash() {
        let mut set = ValidatorSet::new(1000);
        
        let validator = OracleValidator {
            address: test_address(1),
            stake: 5000,
            reputation: 100,
            is_active: true,
        };

        set.add_validator(validator).unwrap();
        
        let slashed = set.slash(&test_address(1), 1000).unwrap();
        assert_eq!(slashed, 1000);
        
        let val = set.validators.get(&test_address(1)).unwrap();
        assert_eq!(val.stake, 4000);
    }
}
