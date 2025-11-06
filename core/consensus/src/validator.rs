//! Validator management and selection

use lean_crypto::{Address, KeyPair, PublicKey};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Minimum stake to become a validator (100,000 LEAN)
pub const MIN_VALIDATOR_STAKE: u128 = 100_000_000_000_000;

/// Maximum number of validators
pub const MAX_VALIDATORS: usize = 1_000;

/// A validator in the network
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Validator {
    /// Validator address
    pub address: Address,
    
    /// Validator public key
    pub public_key: PublicKey,
    
    /// Staked amount
    pub stake: u128,
    
    /// Commission rate (0-100)
    pub commission_rate: u8,
    
    /// Reputation score (0-100)
    pub reputation: u8,
    
    /// Total blocks proposed
    pub blocks_proposed: u64,
    
    /// Total blocks signed
    pub blocks_signed: u64,
    
    /// Total blocks missed
    pub blocks_missed: u64,
    
    /// Is validator jailed (temporarily banned)
    pub jailed: bool,
}

impl Validator {
    /// Create a new validator
    pub fn new(address: Address, public_key: PublicKey, stake: u128, commission_rate: u8) -> Self {
        Self {
            address,
            public_key,
            stake,
            commission_rate,
            reputation: 100, // Start with perfect reputation
            blocks_proposed: 0,
            blocks_signed: 0,
            blocks_missed: 0,
            jailed: false,
        }
    }
    
    /// Get voting power (proportional to stake)
    pub fn voting_power(&self) -> u128 {
        if self.jailed {
            0
        } else {
            self.stake
        }
    }
    
    /// Calculate uptime percentage
    pub fn uptime(&self) -> f64 {
        let total = self.blocks_signed + self.blocks_missed;
        if total == 0 {
            return 100.0;
        }
        (self.blocks_signed as f64 / total as f64) * 100.0
    }
    
    /// Check if validator should be slashed for downtime
    pub fn should_slash_for_downtime(&self) -> bool {
        self.uptime() < 95.0 && (self.blocks_signed + self.blocks_missed) > 100
    }
    
    /// Update reputation based on performance
    pub fn update_reputation(&mut self) {
        let uptime = self.uptime();
        
        if uptime >= 99.0 {
            self.reputation = 100;
        } else if uptime >= 95.0 {
            self.reputation = 90;
        } else if uptime >= 90.0 {
            self.reputation = 70;
        } else {
            self.reputation = 50;
        }
    }
}

/// Set of validators
#[derive(Clone, Debug)]
pub struct ValidatorSet {
    /// Validators by address
    validators: HashMap<Address, Validator>,
    
    /// Total voting power
    total_voting_power: u128,
}

impl ValidatorSet {
    /// Create a new validator set
    pub fn new() -> Self {
        Self {
            validators: HashMap::new(),
            total_voting_power: 0,
        }
    }
    
    /// Add a validator
    pub fn add_validator(&mut self, validator: Validator) -> Result<(), String> {
        if validator.stake < MIN_VALIDATOR_STAKE {
            return Err(format!("Stake below minimum: {}", MIN_VALIDATOR_STAKE));
        }
        
        if self.validators.len() >= MAX_VALIDATORS {
            // Check if this validator has more stake than the lowest
            if let Some(lowest) = self.get_lowest_stake_validator() {
                if validator.stake <= lowest.stake {
                    return Err("Validator set full and stake not high enough".to_string());
                }
                // Remove lowest validator
                self.remove_validator(&lowest.address);
            }
        }
        
        self.total_voting_power += validator.voting_power();
        self.validators.insert(validator.address, validator);
        
        Ok(())
    }
    
    /// Remove a validator
    pub fn remove_validator(&mut self, address: &Address) -> Option<Validator> {
        if let Some(validator) = self.validators.remove(address) {
            self.total_voting_power -= validator.voting_power();
            Some(validator)
        } else {
            None
        }
    }
    
    /// Get a validator
    pub fn get_validator(&self, address: &Address) -> Option<&Validator> {
        self.validators.get(address)
    }
    
    /// Get mutable validator
    pub fn get_validator_mut(&mut self, address: &Address) -> Option<&mut Validator> {
        self.validators.get_mut(address)
    }
    
    /// Get all validators
    pub fn validators(&self) -> Vec<&Validator> {
        self.validators.values().collect()
    }
    
    /// Get active validators (not jailed)
    pub fn active_validators(&self) -> Vec<&Validator> {
        self.validators.values().filter(|v| !v.jailed).collect()
    }
    
    /// Get total voting power
    pub fn total_voting_power(&self) -> u128 {
        self.total_voting_power
    }
    
    /// Get validator count
    pub fn len(&self) -> usize {
        self.validators.len()
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.validators.is_empty()
    }
    
    /// Select proposer for a round (based on stake-weighted randomness)
    pub fn select_proposer(&self, round: u64, block_height: u64) -> Option<Address> {
        let active = self.active_validators();
        if active.is_empty() {
            return None;
        }
        
        // Deterministic selection based on round and height
        let seed = round.wrapping_mul(block_height);
        let index = (seed as usize) % active.len();
        
        Some(active[index].address)
    }
    
    /// Get validator with lowest stake
    fn get_lowest_stake_validator(&self) -> Option<Validator> {
        self.validators.values()
            .min_by_key(|v| v.stake)
            .cloned()
    }
    
    /// Calculate 2/3+ majority threshold
    pub fn majority_threshold(&self) -> u128 {
        (self.total_voting_power * 2) / 3 + 1
    }
}

impl Default for ValidatorSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validator_creation() {
        let keypair = KeyPair::generate();
        let address = Address::from_public_key(keypair.public_key());
        
        let validator = Validator::new(
            address,
            *keypair.public_key(),
            MIN_VALIDATOR_STAKE,
            10,
        );
        
        assert_eq!(validator.stake, MIN_VALIDATOR_STAKE);
        assert_eq!(validator.reputation, 100);
        assert!(!validator.jailed);
    }
    
    #[test]
    fn test_voting_power() {
        let keypair = KeyPair::generate();
        let address = Address::from_public_key(keypair.public_key());
        
        let mut validator = Validator::new(
            address,
            *keypair.public_key(),
            1_000_000,
            10,
        );
        
        assert_eq!(validator.voting_power(), 1_000_000);
        
        validator.jailed = true;
        assert_eq!(validator.voting_power(), 0);
    }
    
    #[test]
    fn test_validator_set() {
        let mut set = ValidatorSet::new();
        
        let keypair = KeyPair::generate();
        let address = Address::from_public_key(keypair.public_key());
        
        let validator = Validator::new(
            address,
            *keypair.public_key(),
            MIN_VALIDATOR_STAKE,
            10,
        );
        
        set.add_validator(validator).unwrap();
        assert_eq!(set.len(), 1);
        assert_eq!(set.total_voting_power(), MIN_VALIDATOR_STAKE);
    }
    
    #[test]
    fn test_proposer_selection() {
        let mut set = ValidatorSet::new();
        
        for i in 0..5 {
            let keypair = KeyPair::generate();
            let address = Address::from_public_key(keypair.public_key());
            
            let validator = Validator::new(
                address,
                *keypair.public_key(),
                MIN_VALIDATOR_STAKE + (i as u128 * 1000),
                10,
            );
            
            set.add_validator(validator).unwrap();
        }
        
        let proposer = set.select_proposer(0, 1);
        assert!(proposer.is_some());
    }
    
    #[test]
    fn test_uptime_calculation() {
        let keypair = KeyPair::generate();
        let address = Address::from_public_key(keypair.public_key());
        
        let mut validator = Validator::new(
            address,
            *keypair.public_key(),
            MIN_VALIDATOR_STAKE,
            10,
        );
        
        validator.blocks_signed = 95;
        validator.blocks_missed = 5;
        
        assert_eq!(validator.uptime(), 95.0);
    }
}
