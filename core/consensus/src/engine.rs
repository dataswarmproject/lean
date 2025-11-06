//! Consensus engine orchestrating the BFT-PoS protocol

use crate::{ValidatorSet, Vote, VoteType, VoteSet, ConsensusError, Result};
use lean_blockchain::Block;
use lean_crypto::Hash256;
use std::time::Duration;

/// Consensus configuration
#[derive(Clone, Debug)]
pub struct ConsensusConfig {
    /// Target block time in seconds
    pub target_block_time: u64,
    
    /// Timeout for propose phase (milliseconds)
    pub propose_timeout: u64,
    
    /// Timeout for prevote phase (milliseconds)
    pub prevote_timeout: u64,
    
    /// Timeout for precommit phase (milliseconds)
    pub precommit_timeout: u64,
    
    /// Maximum rounds before giving up
    pub max_rounds: u64,
}

impl Default for ConsensusConfig {
    fn default() -> Self {
        Self {
            target_block_time: 2,    // 2 seconds
            propose_timeout: 500,     // 500ms
            prevote_timeout: 500,     // 500ms
            precommit_timeout: 500,   // 500ms
            max_rounds: 10,
        }
    }
}

/// Current consensus state
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConsensusState {
    /// Waiting for new round to start
    NewRound,
    /// Propose phase - waiting for block proposal
    Propose,
    /// Prevote phase - validators vote on proposal
    Prevote,
    /// Precommit phase - validators commit to block
    Precommit,
    /// Commit phase - block is finalized
    Commit,
}

/// Consensus engine managing the BFT-PoS protocol
pub struct ConsensusEngine {
    /// Configuration
    config: ConsensusConfig,
    
    /// Validator set
    validator_set: ValidatorSet,
    
    /// Current block height
    height: u64,
    
    /// Current round
    round: u64,
    
    /// Current state
    state: ConsensusState,
    
    /// Prevote set for current round
    prevotes: Option<VoteSet>,
    
    /// Precommit set for current round
    precommits: Option<VoteSet>,
    
    /// Proposed block for current round
    proposed_block: Option<Block>,
}

impl ConsensusEngine {
    /// Create a new consensus engine
    pub fn new(config: ConsensusConfig, validator_set: ValidatorSet) -> Self {
        Self {
            config,
            validator_set,
            height: 0,
            round: 0,
            state: ConsensusState::NewRound,
            prevotes: None,
            precommits: None,
            proposed_block: None,
        }
    }
    
    /// Start a new height
    pub fn new_height(&mut self, height: u64) {
        self.height = height;
        self.round = 0;
        self.state = ConsensusState::NewRound;
        self.prevotes = None;
        self.precommits = None;
        self.proposed_block = None;
    }
    
    /// Start a new round
    pub fn new_round(&mut self) {
        self.state = ConsensusState::Propose;
        
        let total_power = self.validator_set.total_voting_power();
        self.prevotes = Some(VoteSet::new(total_power));
        self.precommits = Some(VoteSet::new(total_power));
    }
    
    /// Get current proposer for this round
    pub fn get_proposer(&self) -> Option<lean_crypto::Address> {
        self.validator_set.select_proposer(self.round, self.height)
    }
    
    /// Receive a block proposal
    pub fn receive_proposal(&mut self, block: Block) -> Result<()> {
        if self.state != ConsensusState::Propose {
            return Err(ConsensusError::InvalidVote("Not in propose phase".to_string()));
        }
        
        // Verify proposer
        let proposer = self.get_proposer()
            .ok_or(ConsensusError::InvalidValidator("No proposer selected".to_string()))?;
        
        if block.header.validator != proposer {
            return Err(ConsensusError::InvalidValidator("Wrong proposer".to_string()));
        }
        
        // Verify block height
        if block.header.height != self.height {
            return Err(ConsensusError::InvalidVote(
                format!("Wrong height: expected {}, got {}", self.height, block.header.height)
            ));
        }
        
        // Store proposed block
        self.proposed_block = Some(block);
        
        // Move to prevote phase
        self.state = ConsensusState::Prevote;
        
        Ok(())
    }
    
    /// Add a prevote
    pub fn add_prevote(&mut self, vote: Vote) -> Result<()> {
        if self.state != ConsensusState::Prevote {
            return Err(ConsensusError::InvalidVote("Not in prevote phase".to_string()));
        }
        
        if vote.vote_type != VoteType::Prevote {
            return Err(ConsensusError::InvalidVote("Wrong vote type".to_string()));
        }
        
        // Get validator power
        let validator = self.validator_set.get_validator(&vote.validator)
            .ok_or(ConsensusError::InvalidValidator("Unknown validator".to_string()))?;
        
        let power = validator.voting_power();
        
        // Add to prevote set
        if let Some(prevotes) = &mut self.prevotes {
            prevotes.add_vote(vote, power)
                .map_err(|e| ConsensusError::InvalidVote(e))?;
            
            // Check if we have 2/3+ prevotes
            if let Some(block_hash) = self.proposed_block.as_ref().map(|b| b.hash()) {
                if prevotes.has_majority(&block_hash) {
                    // Move to precommit phase
                    self.state = ConsensusState::Precommit;
                }
            }
        }
        
        Ok(())
    }
    
    /// Add a precommit
    pub fn add_precommit(&mut self, vote: Vote) -> Result<()> {
        if self.state != ConsensusState::Precommit {
            return Err(ConsensusError::InvalidVote("Not in precommit phase".to_string()));
        }
        
        if vote.vote_type != VoteType::Precommit {
            return Err(ConsensusError::InvalidVote("Wrong vote type".to_string()));
        }
        
        // Get validator power
        let validator = self.validator_set.get_validator(&vote.validator)
            .ok_or(ConsensusError::InvalidValidator("Unknown validator".to_string()))?;
        
        let power = validator.voting_power();
        
        // Add to precommit set
        if let Some(precommits) = &mut self.precommits {
            precommits.add_vote(vote, power)
                .map_err(|e| ConsensusError::InvalidVote(e))?;
            
            // Check if we have 2/3+ precommits
            if let Some(block_hash) = self.proposed_block.as_ref().map(|b| b.hash()) {
                if precommits.has_majority(&block_hash) {
                    // Move to commit phase - consensus reached!
                    self.state = ConsensusState::Commit;
                }
            }
        }
        
        Ok(())
    }
    
    /// Check if consensus is reached
    pub fn is_consensus_reached(&self) -> bool {
        self.state == ConsensusState::Commit
    }
    
    /// Get the committed block
    pub fn get_committed_block(&self) -> Option<&Block> {
        if self.is_consensus_reached() {
            self.proposed_block.as_ref()
        } else {
            None
        }
    }
    
    /// Handle timeout - move to next round
    pub fn handle_timeout(&mut self) -> Result<()> {
        if self.round >= self.config.max_rounds {
            return Err(ConsensusError::ConsensusNotReached);
        }
        
        self.round += 1;
        self.new_round();
        
        Ok(())
    }
    
    /// Get current state
    pub fn state(&self) -> &ConsensusState {
        &self.state
    }
    
    /// Get current height
    pub fn height(&self) -> u64 {
        self.height
    }
    
    /// Get current round
    pub fn round(&self) -> u64 {
        self.round
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Validator;
    use lean_crypto::KeyPair;
    
    #[test]
    fn test_consensus_engine_creation() {
        let config = ConsensusConfig::default();
        let validator_set = ValidatorSet::new();
        
        let engine = ConsensusEngine::new(config, validator_set);
        assert_eq!(engine.height(), 0);
        assert_eq!(engine.round(), 0);
    }
    
    #[test]
    fn test_new_height() {
        let config = ConsensusConfig::default();
        let validator_set = ValidatorSet::new();
        
        let mut engine = ConsensusEngine::new(config, validator_set);
        engine.new_height(10);
        
        assert_eq!(engine.height(), 10);
        assert_eq!(engine.round(), 0);
    }
}
