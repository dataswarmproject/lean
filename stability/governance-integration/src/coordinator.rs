use crate::types::{GovernanceConfig, GovernanceAction, Proposal, ActionStatus, GovernanceError, Result};
use crate::parameters::ParameterController;
use crate::emergency::EmergencyMultisig;
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;

/// Main governance coordinator for stability system
pub struct StabilityGovernance {
    config: GovernanceConfig,
    parameter_controller: ParameterController,
    emergency_multisig: EmergencyMultisig,
    proposals: HashMap<u64, Proposal>,
    next_proposal_id: u64,
    total_voting_power: u128,
}

impl StabilityGovernance {
    pub fn new(config: GovernanceConfig, signers: Vec<String>, total_voting_power: u128) -> Self {
        Self {
            parameter_controller: ParameterController::new(),
            emergency_multisig: EmergencyMultisig::new(config.clone(), signers),
            config,
            proposals: HashMap::new(),
            next_proposal_id: 1,
            total_voting_power,
        }
    }

    pub fn create_proposal(&mut self, action: GovernanceAction, proposer: String) -> u64 {
        let id = self.next_proposal_id;
        self.next_proposal_id += 1;

        let proposal = Proposal {
            id,
            action,
            proposer,
            created_at: Utc::now(),
            execute_after: None,
            status: ActionStatus::Pending,
            votes_for: 0,
            votes_against: 0,
        };

        self.proposals.insert(id, proposal);
        id
    }

    pub fn vote(&mut self, proposal_id: u64, voter_power: u128, vote_for: bool) -> Result<()> {
        let proposal = self.proposals.get_mut(&proposal_id)
            .ok_or(GovernanceError::ProposalNotFound(proposal_id))?;

        if proposal.status != ActionStatus::Pending {
            return Err(GovernanceError::InvalidStatus {
                expected: ActionStatus::Pending,
                actual: proposal.status,
            });
        }

        if vote_for {
            proposal.votes_for += voter_power;
        } else {
            proposal.votes_against += voter_power;
        }

        Ok(())
    }

    pub fn finalize_voting(&mut self, proposal_id: u64) -> Result<()> {
        let proposal = self.proposals.get_mut(&proposal_id)
            .ok_or(GovernanceError::ProposalNotFound(proposal_id))?;

        if proposal.status != ActionStatus::Pending {
            return Err(GovernanceError::InvalidStatus {
                expected: ActionStatus::Pending,
                actual: proposal.status,
            });
        }

        // Check quorum
        let total_votes = proposal.votes_for + proposal.votes_against;
        let quorum = (self.total_voting_power * self.config.quorum_bps as u128) / 10000;

        if total_votes < quorum {
            proposal.status = ActionStatus::Rejected;
            return Err(GovernanceError::InsufficientQuorum {
                votes: total_votes,
                quorum,
            });
        }

        // Check approval threshold
        let approval_needed = (total_votes * self.config.approval_threshold_bps as u128) / 10000;

        if proposal.votes_for >= approval_needed {
            proposal.status = ActionStatus::Approved;

            // Set timelock
            let execute_after = Utc::now() + Duration::hours(self.config.timelock_hours);
            proposal.execute_after = Some(execute_after);
        } else {
            proposal.status = ActionStatus::Rejected;
            return Err(GovernanceError::NotApproved {
                votes_for: proposal.votes_for,
                total_votes,
            });
        }

        Ok(())
    }

    pub fn execute_proposal(&mut self, proposal_id: u64) -> Result<()> {
        let proposal = self.proposals.get_mut(&proposal_id)
            .ok_or(GovernanceError::ProposalNotFound(proposal_id))?;

        if proposal.status != ActionStatus::Approved {
            return Err(GovernanceError::InvalidStatus {
                expected: ActionStatus::Approved,
                actual: proposal.status,
            });
        }

        // Check timelock
        if let Some(execute_after) = proposal.execute_after {
            if Utc::now() < execute_after {
                return Err(GovernanceError::TimeLockActive { until: execute_after });
            }
        }

        // Execute action based on type
        match &proposal.action {
            GovernanceAction::UpdateParameter { name, value } => {
                self.parameter_controller.update_parameter(
                    name.clone(),
                    value.clone(),
                    proposal.proposer.clone()
                )?;
            }
            GovernanceAction::EmergencyPause { .. } |
            GovernanceAction::EmergencyResume |
            GovernanceAction::UpdateCircuitBreaker { .. } |
            GovernanceAction::UpdateTreasuryAllocation { .. } |
            GovernanceAction::AdjustSupplyMechanism { .. } => {
                // These would integrate with respective systems
                // For now, just mark as executed
            }
        }

        proposal.status = ActionStatus::Executed;
        Ok(())
    }

    pub fn get_proposal(&self, proposal_id: u64) -> Option<&Proposal> {
        self.proposals.get(&proposal_id)
    }

    pub fn get_parameter(&self, name: &str) -> Option<String> {
        self.parameter_controller.get_parameter(name)
    }

    pub fn get_parameter_controller(&self) -> &ParameterController {
        &self.parameter_controller
    }

    pub fn get_emergency_multisig(&self) -> &EmergencyMultisig {
        &self.emergency_multisig
    }

    pub fn get_emergency_multisig_mut(&mut self) -> &mut EmergencyMultisig {
        &mut self.emergency_multisig
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_governance() -> StabilityGovernance {
        let config = GovernanceConfig::default();
        let signers = (1..=9).map(|i| format!("signer{}", i)).collect();
        StabilityGovernance::new(config, signers, 1_000_000_000_000)
    }

    #[test]
    fn test_governance_creation() {
        let governance = create_test_governance();
        assert!(governance.get_parameter("max_daily_adjustment_bps").is_some());
    }

    #[test]
    fn test_create_proposal() {
        let mut governance = create_test_governance();
        let action = GovernanceAction::UpdateParameter {
            name: "max_daily_adjustment_bps".to_string(),
            value: "600".to_string(),
        };

        let proposal_id = governance.create_proposal(action, "proposer1".to_string());
        assert_eq!(proposal_id, 1);
        assert!(governance.get_proposal(proposal_id).is_some());
    }

    #[test]
    fn test_voting() {
        let mut governance = create_test_governance();
        let action = GovernanceAction::UpdateParameter {
            name: "max_daily_adjustment_bps".to_string(),
            value: "600".to_string(),
        };

        let proposal_id = governance.create_proposal(action, "proposer1".to_string());

        // Vote with 20% of voting power
        assert!(governance.vote(proposal_id, 200_000_000_000, true).is_ok());

        let proposal = governance.get_proposal(proposal_id).unwrap();
        assert_eq!(proposal.votes_for, 200_000_000_000);
    }

    #[test]
    fn test_finalize_voting_success() {
        let mut governance = create_test_governance();
        let action = GovernanceAction::UpdateParameter {
            name: "max_daily_adjustment_bps".to_string(),
            value: "600".to_string(),
        };

        let proposal_id = governance.create_proposal(action, "proposer1".to_string());

        // Vote with 70% voting power (exceeds 60% approval threshold)
        governance.vote(proposal_id, 700_000_000_000, true).unwrap();

        // Finalize should succeed
        assert!(governance.finalize_voting(proposal_id).is_ok());

        let proposal = governance.get_proposal(proposal_id).unwrap();
        assert_eq!(proposal.status, ActionStatus::Approved);
    }

    #[test]
    fn test_finalize_voting_insufficient_quorum() {
        let mut governance = create_test_governance();
        let action = GovernanceAction::UpdateParameter {
            name: "max_daily_adjustment_bps".to_string(),
            value: "600".to_string(),
        };

        let proposal_id = governance.create_proposal(action, "proposer1".to_string());

        // Vote with only 5% voting power (below 10% quorum)
        governance.vote(proposal_id, 50_000_000_000, true).unwrap();

        // Finalize should fail due to quorum
        let result = governance.finalize_voting(proposal_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_execute_proposal() {
        let mut governance = create_test_governance();
        let action = GovernanceAction::UpdateParameter {
            name: "max_daily_adjustment_bps".to_string(),
            value: "600".to_string(),
        };

        let proposal_id = governance.create_proposal(action, "proposer1".to_string());

        // Vote and approve
        governance.vote(proposal_id, 700_000_000_000, true).unwrap();
        governance.finalize_voting(proposal_id).unwrap();

        // Manually set execute_after to past for testing
        if let Some(proposal) = governance.proposals.get_mut(&proposal_id) {
            proposal.execute_after = Some(Utc::now() - Duration::hours(1));
        }

        // Execute should succeed
        assert!(governance.execute_proposal(proposal_id).is_ok());

        let proposal = governance.get_proposal(proposal_id).unwrap();
        assert_eq!(proposal.status, ActionStatus::Executed);

        // Parameter should be updated
        assert_eq!(governance.get_parameter("max_daily_adjustment_bps").unwrap(), "600");
    }
}
