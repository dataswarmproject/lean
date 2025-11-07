use crate::types::{GovernanceConfig, GovernanceError, Result};
use std::collections::HashSet;

/// Emergency multisig for critical actions
pub struct EmergencyMultisig {
    config: GovernanceConfig,
    signers: HashSet<String>,
    pending_actions: Vec<EmergencyAction>,
}

#[derive(Debug, Clone)]
struct EmergencyAction {
    id: u64,
    action_type: EmergencyActionType,
    signatures: Vec<String>,
    executed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EmergencyActionType {
    Pause,
    Resume,
    CircuitBreakerOverride,
    EmergencyWithdrawal { amount: u128 },
}

impl EmergencyMultisig {
    pub fn new(config: GovernanceConfig, signers: Vec<String>) -> Self {
        Self {
            config,
            signers: signers.into_iter().collect(),
            pending_actions: Vec::new(),
        }
    }

    pub fn propose_emergency_action(&mut self, action_type: EmergencyActionType, proposer: &str) -> Result<u64> {
        if !self.signers.contains(proposer) {
            return Err(GovernanceError::Unauthorized(
                format!("Proposer {} is not a signer", proposer)
            ));
        }

        let id = self.pending_actions.len() as u64;
        self.pending_actions.push(EmergencyAction {
            id,
            action_type,
            signatures: vec![proposer.to_string()],
            executed: false,
        });

        Ok(id)
    }

    pub fn sign_action(&mut self, action_id: u64, signer: &str) -> Result<()> {
        if !self.signers.contains(signer) {
            return Err(GovernanceError::Unauthorized(
                format!("Signer {} is not authorized", signer)
            ));
        }

        let action = self.pending_actions.iter_mut()
            .find(|a| a.id == action_id)
            .ok_or(GovernanceError::ProposalNotFound(action_id))?;

        if action.signatures.contains(&signer.to_string()) {
            return Ok(()); // Already signed
        }

        action.signatures.push(signer.to_string());
        Ok(())
    }

    pub fn can_execute(&self, action_id: u64) -> Result<bool> {
        let action = self.pending_actions.iter()
            .find(|a| a.id == action_id)
            .ok_or(GovernanceError::ProposalNotFound(action_id))?;

        if action.executed {
            return Ok(false);
        }

        let (required, _) = self.config.emergency_threshold;
        Ok(action.signatures.len() >= required as usize)
    }

    pub fn execute_action(&mut self, action_id: u64) -> Result<EmergencyActionType> {
        if !self.can_execute(action_id)? {
            let action = self.pending_actions.iter()
                .find(|a| a.id == action_id)
                .ok_or(GovernanceError::ProposalNotFound(action_id))?;

            let (required, _) = self.config.emergency_threshold;
            return Err(GovernanceError::InsufficientSignatures {
                required,
                provided: action.signatures.len() as u8,
            });
        }

        let action = self.pending_actions.iter_mut()
            .find(|a| a.id == action_id)
            .ok_or(GovernanceError::ProposalNotFound(action_id))?;

        action.executed = true;
        Ok(action.action_type.clone())
    }

    pub fn get_signers(&self) -> Vec<String> {
        self.signers.iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_multisig() -> EmergencyMultisig {
        let config = GovernanceConfig::default();
        let signers = vec![
            "signer1".to_string(),
            "signer2".to_string(),
            "signer3".to_string(),
            "signer4".to_string(),
            "signer5".to_string(),
            "signer6".to_string(),
            "signer7".to_string(),
            "signer8".to_string(),
            "signer9".to_string(),
        ];
        EmergencyMultisig::new(config, signers)
    }

    #[test]
    fn test_multisig_creation() {
        let multisig = create_test_multisig();
        assert_eq!(multisig.get_signers().len(), 9);
    }

    #[test]
    fn test_propose_emergency_action() {
        let mut multisig = create_test_multisig();
        let result = multisig.propose_emergency_action(EmergencyActionType::Pause, "signer1");
        assert!(result.is_ok());
    }

    #[test]
    fn test_unauthorized_proposer() {
        let mut multisig = create_test_multisig();
        let result = multisig.propose_emergency_action(EmergencyActionType::Pause, "unauthorized");
        assert!(result.is_err());
    }

    #[test]
    fn test_sign_action() {
        let mut multisig = create_test_multisig();
        let action_id = multisig.propose_emergency_action(EmergencyActionType::Pause, "signer1").unwrap();

        assert!(multisig.sign_action(action_id, "signer2").is_ok());
        assert!(multisig.sign_action(action_id, "signer3").is_ok());
    }

    #[test]
    fn test_can_execute() {
        let mut multisig = create_test_multisig();
        let action_id = multisig.propose_emergency_action(EmergencyActionType::Pause, "signer1").unwrap();

        // Initially only 1 signature (proposer)
        assert!(!multisig.can_execute(action_id).unwrap());

        // Add signatures to reach threshold (5-of-9)
        multisig.sign_action(action_id, "signer2").unwrap();
        multisig.sign_action(action_id, "signer3").unwrap();
        multisig.sign_action(action_id, "signer4").unwrap();
        multisig.sign_action(action_id, "signer5").unwrap();

        // Now should have 5 signatures
        assert!(multisig.can_execute(action_id).unwrap());
    }

    #[test]
    fn test_execute_action() {
        let mut multisig = create_test_multisig();
        let action_id = multisig.propose_emergency_action(EmergencyActionType::Resume, "signer1").unwrap();

        // Add enough signatures
        for i in 2..=5 {
            multisig.sign_action(action_id, &format!("signer{}", i)).unwrap();
        }

        let result = multisig.execute_action(action_id);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), EmergencyActionType::Resume);
    }

    #[test]
    fn test_execute_insufficient_signatures() {
        let mut multisig = create_test_multisig();
        let action_id = multisig.propose_emergency_action(EmergencyActionType::Pause, "signer1").unwrap();

        // Only 3 signatures (need 5)
        multisig.sign_action(action_id, "signer2").unwrap();
        multisig.sign_action(action_id, "signer3").unwrap();

        let result = multisig.execute_action(action_id);
        assert!(result.is_err());
    }
}
