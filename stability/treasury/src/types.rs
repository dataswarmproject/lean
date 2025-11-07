use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use thiserror::Error;

/// Asset types held in treasury
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TreasuryAsset {
    /// Stablecoins (USDC, USDT, DAI)
    Stablecoin,
    /// Bitcoin
    Bitcoin,
    /// Ethereum
    Ethereum,
    /// Native LEAN tokens
    NativeToken,
    /// Other assets
    Other,
}

/// Treasury operation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TreasuryOperation {
    /// Buy-back program execution
    BuyBack {
        amount_tokens: u128,
        amount_spent: u128,
        price: u128,
    },
    /// Market making operation
    MarketMaking {
        liquidity_provided: u128,
        pair: String,
    },
    /// Asset reallocation
    Reallocation {
        from_asset: TreasuryAsset,
        to_asset: TreasuryAsset,
        amount: u128,
    },
    /// Revenue collection
    RevenueCollection {
        asset: TreasuryAsset,
        amount: u128,
        source: String,
    },
}

/// Treasury holding information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryHolding {
    pub asset: TreasuryAsset,
    pub amount: u128,
    pub value_usd: u128,
    pub last_updated: DateTime<Utc>,
}

/// Treasury configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryConfig {
    /// Target allocation percentages (in basis points)
    pub target_allocation: [(TreasuryAsset, u16); 5],
    /// Maximum daily spend (in basis points of treasury)
    pub max_daily_spend_bps: u16,
    /// Minimum reserve level (in basis points)
    pub min_reserve_bps: u16,
    /// Rebalancing threshold (in basis points)
    pub rebalancing_threshold_bps: u16,
}

impl Default for TreasuryConfig {
    fn default() -> Self {
        Self {
            target_allocation: [
                (TreasuryAsset::Stablecoin, 4000),   // 40%
                (TreasuryAsset::Bitcoin, 1500),      // 15%
                (TreasuryAsset::Ethereum, 1500),     // 15%
                (TreasuryAsset::NativeToken, 2000),  // 20%
                (TreasuryAsset::Other, 1000),        // 10%
            ],
            max_daily_spend_bps: 100,      // 1% per day
            min_reserve_bps: 2000,         // 20% minimum reserve
            rebalancing_threshold_bps: 500, // 5% deviation triggers rebalancing
        }
    }
}

/// Errors related to treasury operations
#[derive(Error, Debug)]
pub enum TreasuryError {
    #[error("Insufficient balance: available {available}, requested {requested}")]
    InsufficientBalance {
        available: u128,
        requested: u128,
    },

    #[error("Daily spend limit exceeded: spent {spent}, limit {limit}")]
    DailyLimitExceeded {
        spent: u128,
        limit: u128,
    },

    #[error("Reserve level too low: current {current_bps}bps, minimum {min_bps}bps")]
    ReserveTooLow {
        current_bps: u16,
        min_bps: u16,
    },

    #[error("Invalid allocation: total {total_bps}bps, expected 10000bps")]
    InvalidAllocation {
        total_bps: u32,
    },

    #[error("Asset not found: {0:?}")]
    AssetNotFound(TreasuryAsset),

    #[error("Operation not allowed: {0}")]
    OperationNotAllowed(String),

    #[error("Calculation overflow")]
    CalculationOverflow,
}

pub type Result<T> = std::result::Result<T, TreasuryError>;
