//! Approval management for ERC-721

use crate::TokenId;
use lean_crypto::Address;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Operator approval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorApproval {
    pub owner: Address,
    pub operator: Address,
    pub approved: bool,
}

/// Manages token approvals and operator approvals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalManager {
    /// Token approvals: token_id -> approved_address
    token_approvals: HashMap<TokenId, Address>,
    /// Operator approvals: (owner, operator) -> approved
    operator_approvals: HashMap<(Address, Address), bool>,
}

impl ApprovalManager {
    pub fn new() -> Self {
        Self {
            token_approvals: HashMap::new(),
            operator_approvals: HashMap::new(),
        }
    }

    /// Set approval for specific token
    pub fn set_approval(&mut self, token_id: TokenId, approved: Address) {
        if approved == Address::zero() {
            self.token_approvals.remove(&token_id);
        } else {
            self.token_approvals.insert(token_id, approved);
        }
    }

    /// Clear approval for token
    pub fn clear_approval(&mut self, token_id: TokenId) {
        self.token_approvals.remove(&token_id);
    }

    /// Get approved address for token
    pub fn get_approved(&self, token_id: TokenId) -> Option<Address> {
        self.token_approvals.get(&token_id).copied()
    }

    /// Check if address is approved for token
    pub fn is_approved(&self, token_id: TokenId, address: &Address) -> bool {
        self.token_approvals
            .get(&token_id)
            .map(|a| a == address)
            .unwrap_or(false)
    }

    /// Set approval for all tokens of owner
    pub fn set_approval_for_all(&mut self, owner: Address, operator: Address, approved: bool) {
        if approved {
            self.operator_approvals.insert((owner, operator), true);
        } else {
            self.operator_approvals.remove(&(owner, operator));
        }
    }

    /// Check if operator is approved for all tokens of owner
    pub fn is_approved_for_all(&self, owner: &Address, operator: &Address) -> bool {
        self.operator_approvals
            .get(&(*owner, *operator))
            .copied()
            .unwrap_or(false)
    }
}

impl Default for ApprovalManager {
    fn default() -> Self {
        Self::new()
    }
}
