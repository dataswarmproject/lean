use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use thiserror::Error;

/// Represents a liquidity pool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidityPool {
    pub id: String,
    pub pair: String,
    pub dex: String,
    pub chain: String,
    pub token0_amount: u128,
    pub token1_amount: u128,
    pub total_value_usd: u128,
    pub apr_bps: u16,
    pub last_updated: DateTime<Utc>,
}

/// Pool performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolMetrics {
    pub pool_id: String,
    pub volume_24h: u128,
    pub fees_24h: u128,
    pub apr_7d: u16,
    pub impermanent_loss_bps: i16,
    pub liquidity_depth: u128,
}

/// Liquidity deployment strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeploymentStrategy {
    /// Wide range (traditional AMM)
    Wide,
    /// Concentrated liquidity (Uniswap V3 style)
    Concentrated { price_range_bps: u16 },
    /// Single-sided liquidity
    SingleSided,
}

/// Liquidity configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidityConfig {
    /// Target total liquidity (USD)
    pub target_total_liquidity_usd: u128,
    /// Minimum APR to maintain position (basis points)
    pub min_apr_bps: u16,
    /// Maximum impermanent loss tolerance (basis points)
    pub max_il_bps: u16,
    /// Rebalancing threshold (basis points)
    pub rebalancing_threshold_bps: u16,
}

impl Default for LiquidityConfig {
    fn default() -> Self {
        Self {
            target_total_liquidity_usd: 10_000_000_000_000, // $10M
            min_apr_bps: 500,                                // 5% minimum APR
            max_il_bps: 1000,                                // 10% max IL
            rebalancing_threshold_bps: 2000,                 // 20% deviation
        }
    }
}

/// Errors related to liquidity operations
#[derive(Error, Debug)]
pub enum LiquidityError {
    #[error("Insufficient liquidity: available {available}, requested {requested}")]
    InsufficientLiquidity {
        available: u128,
        requested: u128,
    },

    #[error("Pool not found: {0}")]
    PoolNotFound(String),

    #[error("APR below minimum: {current_bps}bps < {min_bps}bps")]
    AprTooLow {
        current_bps: u16,
        min_bps: u16,
    },

    #[error("Impermanent loss exceeds limit: {il_bps}bps > {max_bps}bps")]
    ExcessiveImpermanentLoss {
        il_bps: u16,
        max_bps: u16,
    },

    #[error("Invalid pool configuration: {0}")]
    InvalidConfig(String),

    #[error("Calculation overflow")]
    CalculationOverflow,

    #[error("Operation not supported: {0}")]
    NotSupported(String),
}

pub type Result<T> = std::result::Result<T, LiquidityError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = LiquidityConfig::default();
        assert_eq!(config.min_apr_bps, 500);
        assert_eq!(config.max_il_bps, 1000);
    }

    #[test]
    fn test_deployment_strategies() {
        let wide = DeploymentStrategy::Wide;
        let concentrated = DeploymentStrategy::Concentrated { price_range_bps: 1000 };
        let single = DeploymentStrategy::SingleSided;

        assert_ne!(wide, concentrated);
        assert_ne!(wide, single);
    }
}
