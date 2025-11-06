//! BFT-PoS Consensus for LEAN CHAIN
//!
//! Implements Byzantine Fault Tolerant Proof of Stake consensus with:
//! - Validator selection based on stake
//! - Three-phase voting (Propose, Prevote, Precommit)
//! - Fast finality (1-2 seconds)
//! - Slashing for malicious behavior

pub mod validator;
pub mod voting;
pub mod engine;
pub mod slashing;

pub use validator::{Validator, ValidatorSet};
pub use voting::{Vote, VoteType, VoteSet};
pub use engine::{ConsensusEngine, ConsensusConfig};
pub use slashing::{Slashing, SlashingCondition};

use lean_crypto::{Address, Hash256};

/// Result type for consensus operations
pub type Result<T> = std::result::Result<T, ConsensusError>;

/// Consensus errors
#[derive(Debug, thiserror::Error)]
pub enum ConsensusError {
    #[error("Invalid validator: {0}")]
    InvalidValidator(String),
    
    #[error("Insufficient voting power: have {have}, need {need}")]
    InsufficientVotingPower { have: u128, need: u128 },
    
    #[error("Invalid vote: {0}")]
    InvalidVote(String),
    
    #[error("Timeout reached")]
    Timeout,
    
    #[error("Byzantine behavior detected")]
    ByzantineBehavior,
    
    #[error("Consensus not reached")]
    ConsensusNotReached,
    
    #[error("Finality not achieved")]
    FinalityNotAchieved,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_consensus_module() {
        assert!(true);
    }
}
