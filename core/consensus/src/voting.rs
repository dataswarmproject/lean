//! Voting mechanism for BFT consensus

use lean_crypto::{Address, Hash256, Signature};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Vote type in the consensus process
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum VoteType {
    /// Prevote phase
    Prevote,
    /// Precommit phase
    Precommit,
}

/// A vote from a validator
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vote {
    /// Vote type
    pub vote_type: VoteType,
    
    /// Block height
    pub height: u64,
    
    /// Round number
    pub round: u64,
    
    /// Block hash being voted for
    pub block_hash: Hash256,
    
    /// Validator address
    pub validator: Address,
    
    /// Vote signature
    pub signature: Signature,
}

impl Vote {
    /// Create a new vote
    pub fn new(
        vote_type: VoteType,
        height: u64,
        round: u64,
        block_hash: Hash256,
        validator: Address,
    ) -> Self {
        Self {
            vote_type,
            height,
            round,
            block_hash,
            validator,
            signature: Signature::from_bytes([0u8; 64]),
        }
    }
    
    /// Sign the vote
    pub fn sign(&mut self, keypair: &lean_crypto::KeyPair) {
        let message = self.signing_message();
        self.signature = keypair.sign(&message);
    }
    
    /// Get signing message
    fn signing_message(&self) -> Vec<u8> {
        let mut message = Vec::new();
        message.push(match self.vote_type {
            VoteType::Prevote => 1,
            VoteType::Precommit => 2,
        });
        message.extend_from_slice(&self.height.to_le_bytes());
        message.extend_from_slice(&self.round.to_le_bytes());
        message.extend_from_slice(self.block_hash.as_bytes());
        message.extend_from_slice(self.validator.as_bytes());
        message
    }
    
    /// Verify vote signature
    pub fn verify(&self, public_key: &lean_crypto::PublicKey) -> bool {
        let message = self.signing_message();
        public_key.verify(&message, &self.signature).is_ok()
    }
}

/// Collection of votes
#[derive(Clone, Debug)]
pub struct VoteSet {
    /// Votes by validator address
    votes: HashMap<Address, Vote>,
    
    /// Voting power by block hash
    voting_power: HashMap<Hash256, u128>,
    
    /// Total voting power available
    total_power: u128,
}

impl VoteSet {
    /// Create a new vote set
    pub fn new(total_power: u128) -> Self {
        Self {
            votes: HashMap::new(),
            voting_power: HashMap::new(),
            total_power,
        }
    }
    
    /// Add a vote
    pub fn add_vote(&mut self, vote: Vote, power: u128) -> Result<(), String> {
        // Check if validator already voted
        if self.votes.contains_key(&vote.validator) {
            return Err("Validator already voted".to_string());
        }
        
        // Add voting power for this block
        *self.voting_power.entry(vote.block_hash).or_insert(0) += power;
        
        // Store vote
        self.votes.insert(vote.validator, vote);
        
        Ok(())
    }
    
    /// Get voting power for a block hash
    pub fn get_voting_power(&self, block_hash: &Hash256) -> u128 {
        self.voting_power.get(block_hash).copied().unwrap_or(0)
    }
    
    /// Check if block hash has 2/3+ majority
    pub fn has_majority(&self, block_hash: &Hash256) -> bool {
        let power = self.get_voting_power(block_hash);
        let threshold = (self.total_power * 2) / 3 + 1;
        power >= threshold
    }
    
    /// Get block hash with most votes
    pub fn get_majority_block(&self) -> Option<Hash256> {
        self.voting_power
            .iter()
            .max_by_key(|(_, &power)| power)
            .map(|(hash, _)| *hash)
    }
    
    /// Get all votes
    pub fn votes(&self) -> Vec<&Vote> {
        self.votes.values().collect()
    }
    
    /// Get vote count
    pub fn len(&self) -> usize {
        self.votes.len()
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.votes.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lean_crypto::KeyPair;
    
    #[test]
    fn test_vote_creation() {
        let keypair = KeyPair::generate();
        let validator = Address::from_public_key(keypair.public_key());
        let block_hash = Hash256::hash(b"test block");
        
        let mut vote = Vote::new(VoteType::Prevote, 1, 0, block_hash, validator);
        vote.sign(&keypair);
        
        assert!(vote.verify(keypair.public_key()));
    }
    
    #[test]
    fn test_vote_set_majority() {
        let total_power = 1_000_000u128;
        let mut vote_set = VoteSet::new(total_power);
        
        let block_hash = Hash256::hash(b"block");
        
        // Add votes totaling 2/3+
        for i in 0..3 {
            let keypair = KeyPair::generate();
            let validator = Address::from_public_key(keypair.public_key());
            
            let vote = Vote::new(VoteType::Prevote, 1, 0, block_hash, validator);
            vote_set.add_vote(vote, 350_000).unwrap();
        }
        
        assert!(vote_set.has_majority(&block_hash));
    }
    
    #[test]
    fn test_vote_set_no_majority() {
        let total_power = 1_000_000u128;
        let mut vote_set = VoteSet::new(total_power);
        
        let block_hash = Hash256::hash(b"block");
        
        // Add votes totaling less than 2/3
        for i in 0..2 {
            let keypair = KeyPair::generate();
            let validator = Address::from_public_key(keypair.public_key());
            
            let vote = Vote::new(VoteType::Prevote, 1, 0, block_hash, validator);
            vote_set.add_vote(vote, 300_000).unwrap();
        }
        
        assert!(!vote_set.has_majority(&block_hash));
    }
}
