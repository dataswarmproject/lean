use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use thiserror::Error;

/// Type of supply adjustment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdjustmentType {
    /// Increase supply (minting)
    Expansion,
    /// Decrease supply (burning)
    Contraction,
}

/// Mechanism for supply adjustment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdjustmentMechanism {
    /// Increase staking rewards
    StakingRewards,
    /// Liquidity mining incentives
    LiquidityMining,
    /// Direct distribution to holders
    Airdrop,
    /// Treasury buy-back and burn
    BuyBack,
    /// Reduce emissions
    EmissionReduction,
    /// Burn transaction fees
    FeeBurn,
}

/// Represents a supply change operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplyChange {
    /// Type of adjustment
    pub adjustment_type: AdjustmentType,
    /// Mechanism used
    pub mechanism: AdjustmentMechanism,
    /// Amount to adjust (in smallest unit)
    pub amount: u128,
    /// Timestamp of the adjustment
    pub timestamp: DateTime<Utc>,
    /// Target price deviation that triggered this (in basis points)
    pub price_deviation_bps: i32,
    /// Transaction hash or identifier
    pub tx_id: Option<String>,
}

impl SupplyChange {
    /// Create a new supply change
    pub fn new(
        adjustment_type: AdjustmentType,
        mechanism: AdjustmentMechanism,
        amount: u128,
        price_deviation_bps: i32,
    ) -> Self {
        Self {
            adjustment_type,
            mechanism,
            amount,
            timestamp: Utc::now(),
            price_deviation_bps,
            tx_id: None,
        }
    }

    /// Check if this is an expansion
    pub fn is_expansion(&self) -> bool {
        self.adjustment_type == AdjustmentType::Expansion
    }

    /// Check if this is a contraction
    pub fn is_contraction(&self) -> bool {
        self.adjustment_type == AdjustmentType::Contraction
    }

    /// Get the signed amount (positive for expansion, negative for contraction)
    pub fn signed_amount(&self) -> i128 {
        match self.adjustment_type {
            AdjustmentType::Expansion => self.amount as i128,
            AdjustmentType::Contraction => -(self.amount as i128),
        }
    }
}

/// Configuration for supply adjustment limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdjustmentConfig {
    /// Maximum daily adjustment rate (in basis points of total supply)
    pub max_daily_adjustment_bps: u16,
    /// Maximum weekly adjustment rate (in basis points of total supply)
    pub max_weekly_adjustment_bps: u16,
    /// Minimum price deviation to trigger adjustment (in basis points)
    pub min_deviation_bps: u16,
    /// Adjustment sensitivity (how aggressively to adjust, 0-10000)
    pub sensitivity_bps: u16,
}

impl Default for AdjustmentConfig {
    fn default() -> Self {
        Self {
            max_daily_adjustment_bps: 500,    // 5% per day
            max_weekly_adjustment_bps: 1500,  // 15% per week
            min_deviation_bps: 500,           // 5% minimum deviation
            sensitivity_bps: 5000,            // 50% sensitivity
        }
    }
}

/// Errors related to supply adjustment
#[derive(Error, Debug)]
pub enum AdjustmentError {
    #[error("Daily adjustment limit exceeded: requested {requested}, limit {limit}")]
    DailyLimitExceeded {
        requested: u128,
        limit: u128,
    },

    #[error("Weekly adjustment limit exceeded: requested {requested}, limit {limit}")]
    WeeklyLimitExceeded {
        requested: u128,
        limit: u128,
    },

    #[error("Price deviation too small: {deviation_bps}bps, minimum {min_bps}bps")]
    DeviationTooSmall {
        deviation_bps: i32,
        min_bps: u16,
    },

    #[error("Invalid adjustment amount: {0}")]
    InvalidAmount(String),

    #[error("Insufficient supply for contraction: current {current}, requested {requested}")]
    InsufficientSupply {
        current: u128,
        requested: u128,
    },

    #[error("Calculation overflow")]
    CalculationOverflow,

    #[error("No adjustment needed")]
    NoAdjustmentNeeded,

    #[error("Mechanism not available: {0}")]
    MechanismNotAvailable(String),
}

pub type Result<T> = std::result::Result<T, AdjustmentError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supply_change_creation() {
        let change = SupplyChange::new(
            AdjustmentType::Expansion,
            AdjustmentMechanism::StakingRewards,
            1_000_000,
            1000, // 10% deviation
        );

        assert!(change.is_expansion());
        assert!(!change.is_contraction());
        assert_eq!(change.signed_amount(), 1_000_000);
    }

    #[test]
    fn test_supply_change_contraction() {
        let change = SupplyChange::new(
            AdjustmentType::Contraction,
            AdjustmentMechanism::BuyBack,
            500_000,
            -800, // -8% deviation
        );

        assert!(!change.is_expansion());
        assert!(change.is_contraction());
        assert_eq!(change.signed_amount(), -500_000);
    }

    #[test]
    fn test_default_adjustment_config() {
        let config = AdjustmentConfig::default();
        assert_eq!(config.max_daily_adjustment_bps, 500);
        assert_eq!(config.max_weekly_adjustment_bps, 1500);
        assert_eq!(config.min_deviation_bps, 500);
    }

    #[test]
    fn test_adjustment_mechanisms() {
        let mechanisms = vec![
            AdjustmentMechanism::StakingRewards,
            AdjustmentMechanism::LiquidityMining,
            AdjustmentMechanism::Airdrop,
            AdjustmentMechanism::BuyBack,
            AdjustmentMechanism::EmissionReduction,
            AdjustmentMechanism::FeeBurn,
        ];

        // All mechanisms should be serializable
        for mechanism in mechanisms {
            let serialized = serde_json::to_string(&mechanism).unwrap();
            assert!(!serialized.is_empty());
        }
    }
}
