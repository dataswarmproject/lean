//! Governance parameters for stablecoin system

use serde::{Deserialize, Serialize};

/// Governance parameters for the stablecoin system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceParameters {
    /// Global debt ceiling
    pub debt_ceiling: u128,
    /// Minimum debt per vault
    pub min_debt: u128,
    /// Liquidation penalty (basis points)
    pub liquidation_penalty: u16,
}

impl Default for GovernanceParameters {
    fn default() -> Self {
        Self {
            debt_ceiling: 1_000_000_000, // 1 billion
            min_debt: 1000,
            liquidation_penalty: 500, // 5%
        }
    }
}

/// Risk parameters per collateral type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskParameters {
    /// Minimum collateralization ratio (basis points)
    pub min_collateral_ratio: u16,
    /// Liquidation ratio (basis points)
    pub liquidation_ratio: u16,
    /// Stability fee per year (basis points)
    pub stability_fee: u64,
    /// Debt ceiling for this collateral type
    pub debt_ceiling: u128,
}

impl Default for RiskParameters {
    fn default() -> Self {
        Self {
            min_collateral_ratio: 15000, // 150%
            liquidation_ratio: 13000, // 130%
            stability_fee: 200, // 2% per year
            debt_ceiling: 100_000_000, // 100 million
        }
    }
}
