use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use thiserror::Error;

/// Governance actions that can be performed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GovernanceAction {
    /// Update stability parameter
    UpdateParameter { name: String, value: String },
    /// Trigger emergency pause
    EmergencyPause { reason: String },
    /// Resume from emergency
    EmergencyResume,
    /// Update treasury allocation
    UpdateTreasuryAllocation { asset: String, allocation_bps: u16 },
    /// Adjust supply mechanism
    AdjustSupplyMechanism { mechanism: String, rate_bps: u16 },
    /// Update circuit breaker threshold
    UpdateCircuitBreaker { level: String, threshold_bps: u16 },
}

/// Status of a governance action
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionStatus {
    /// Pending approval
    Pending,
    /// Approved, pending timelock
    Approved,
    /// In timelock period
    TimeLocked { until: i64 },
    /// Executed successfully
    Executed,
    /// Rejected by governance
    Rejected,
    /// Cancelled before execution
    Cancelled,
}

/// Governance proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: u64,
    pub action: GovernanceAction,
    pub proposer: String,
    pub created_at: DateTime<Utc>,
    pub execute_after: Option<DateTime<Utc>>,
    pub status: ActionStatus,
    pub votes_for: u128,
    pub votes_against: u128,
}

/// Governance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceConfig {
    /// Timelock duration for non-emergency actions (hours)
    pub timelock_hours: i64,
    /// Quorum required (basis points)
    pub quorum_bps: u16,
    /// Approval threshold (basis points)
    pub approval_threshold_bps: u16,
    /// Emergency multisig threshold (n of m)
    pub emergency_threshold: (u8, u8),
}

impl Default for GovernanceConfig {
    fn default() -> Self {
        Self {
            timelock_hours: 48,           // 48 hour timelock
            quorum_bps: 1000,             // 10% quorum
            approval_threshold_bps: 6000, // 60% approval
            emergency_threshold: (5, 9),  // 5-of-9 multisig
        }
    }
}

/// Errors related to governance operations
#[derive(Error, Debug)]
pub enum GovernanceError {
    #[error("Proposal not found: {0}")]
    ProposalNotFound(u64),

    #[error("Insufficient votes: {votes} < quorum {quorum}")]
    InsufficientQuorum {
        votes: u128,
        quorum: u128,
    },

    #[error("Proposal not approved: {votes_for} / {total_votes}")]
    NotApproved {
        votes_for: u128,
        total_votes: u128,
    },

    #[error("Timelock active until {until}")]
    TimeLockActive {
        until: DateTime<Utc>,
    },

    #[error("Invalid status: expected {expected:?}, got {actual:?}")]
    InvalidStatus {
        expected: ActionStatus,
        actual: ActionStatus,
    },

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Emergency action requires {required} signatures, got {provided}")]
    InsufficientSignatures {
        required: u8,
        provided: u8,
    },
}

pub type Result<T> = std::result::Result<T, GovernanceError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_governance_config() {
        let config = GovernanceConfig::default();
        assert_eq!(config.timelock_hours, 48);
        assert_eq!(config.quorum_bps, 1000);
        assert_eq!(config.approval_threshold_bps, 6000);
    }

    #[test]
    fn test_action_status() {
        assert_eq!(ActionStatus::Pending, ActionStatus::Pending);
        assert_ne!(ActionStatus::Pending, ActionStatus::Executed);
    }
}
