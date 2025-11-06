//! Slashing conditions and penalties for validators

use lean_crypto::Address;
use serde::{Deserialize, Serialize};

/// Slashing conditions
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SlashingCondition {
    /// Double signing (signing two conflicting blocks)
    DoubleSigning {
        height: u64,
        round: u64,
    },
    
    /// Downtime (missing too many blocks)
    Downtime {
        missed_blocks: u64,
        total_blocks: u64,
    },
    
    /// Byzantine behavior (malicious activity)
    ByzantineBehavior {
        description: String,
    },
}

/// Slashing penalty
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SlashingPenalty {
    /// Validator being slashed
    pub validator: Address,
    
    /// Slashing condition
    pub condition: SlashingCondition,
    
    /// Percentage of stake to slash (0-100)
    pub slash_percentage: u8,
    
    /// Whether to jail the validator
    pub jail: bool,
    
    /// Jail duration in blocks (0 = permanent)
    pub jail_duration: u64,
}

impl SlashingPenalty {
    /// Create a penalty for double signing
    pub fn double_signing(validator: Address, height: u64, round: u64) -> Self {
        Self {
            validator,
            condition: SlashingCondition::DoubleSigning { height, round },
            slash_percentage: 5, // 5% slash
            jail: true,
            jail_duration: 0, // Permanent jail
        }
    }
    
    /// Create a penalty for downtime
    pub fn downtime(validator: Address, missed_blocks: u64, total_blocks: u64) -> Self {
        Self {
            validator,
            condition: SlashingCondition::Downtime { missed_blocks, total_blocks },
            slash_percentage: 1, // 0.1% slash
            jail: true,
            jail_duration: 10_000, // Jail for 10,000 blocks
        }
    }
    
    /// Create a penalty for Byzantine behavior
    pub fn byzantine_behavior(validator: Address, description: String) -> Self {
        Self {
            validator,
            condition: SlashingCondition::ByzantineBehavior { description },
            slash_percentage: 100, // 100% slash (complete confiscation)
            jail: true,
            jail_duration: 0, // Permanent jail
        }
    }
    
    /// Calculate the amount to slash
    pub fn calculate_slash_amount(&self, stake: u128) -> u128 {
        (stake * self.slash_percentage as u128) / 100
    }
}

/// Slashing manager
pub struct Slashing {
    /// Recorded slashing events
    events: Vec<SlashingPenalty>,
}

impl Slashing {
    /// Create a new slashing manager
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
        }
    }
    
    /// Record a slashing event
    pub fn slash(&mut self, penalty: SlashingPenalty) {
        self.events.push(penalty);
    }
    
    /// Get all slashing events
    pub fn get_events(&self) -> &[SlashingPenalty] {
        &self.events
    }
    
    /// Get slashing events for a validator
    pub fn get_validator_events(&self, validator: &Address) -> Vec<&SlashingPenalty> {
        self.events.iter()
            .filter(|e| &e.validator == validator)
            .collect()
    }
    
    /// Check if validator has been slashed
    pub fn is_slashed(&self, validator: &Address) -> bool {
        self.events.iter().any(|e| &e.validator == validator)
    }
}

impl Default for Slashing {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lean_crypto::KeyPair;
    
    #[test]
    fn test_double_signing_penalty() {
        let keypair = KeyPair::generate();
        let validator = Address::from_public_key(keypair.public_key());
        
        let penalty = SlashingPenalty::double_signing(validator, 100, 0);
        
        assert_eq!(penalty.slash_percentage, 5);
        assert!(penalty.jail);
        assert_eq!(penalty.jail_duration, 0);
    }
    
    #[test]
    fn test_slash_amount_calculation() {
        let keypair = KeyPair::generate();
        let validator = Address::from_public_key(keypair.public_key());
        
        let penalty = SlashingPenalty::double_signing(validator, 100, 0);
        let stake = 1_000_000u128;
        
        let slash_amount = penalty.calculate_slash_amount(stake);
        assert_eq!(slash_amount, 50_000); // 5% of 1,000,000
    }
    
    #[test]
    fn test_slashing_manager() {
        let mut slashing = Slashing::new();
        
        let keypair = KeyPair::generate();
        let validator = Address::from_public_key(keypair.public_key());
        
        let penalty = SlashingPenalty::double_signing(validator, 100, 0);
        slashing.slash(penalty);
        
        assert!(slashing.is_slashed(&validator));
        assert_eq!(slashing.get_events().len(), 1);
    }
}
