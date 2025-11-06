//! LEAN CHAIN Stablecoin Framework
//!
//! Collateral-backed stablecoin system with stability mechanisms.

mod vault;
mod stability;
mod liquidation;
mod governance;

pub use vault::{Vault, VaultId, VaultStatus};
pub use stability::{StabilityPool, StabilityFee};
pub use liquidation::{LiquidationEngine, LiquidationAuction};
pub use governance::{GovernanceParameters, RiskParameters};

use thiserror::Error;

/// Stablecoin framework errors
#[derive(Debug, Error)]
pub enum StablecoinError {
    #[error("Insufficient collateral: required {required}, provided {provided}")]
    InsufficientCollateral { required: u128, provided: u128 },
    
    #[error("Vault not found: {0}")]
    VaultNotFound(String),
    
    #[error("Vault undercollateralized")]
    Undercollateralized,
    
    #[error("Debt ceiling reached")]
    DebtCeilingReached,
    
    #[error("Minimum debt not met")]
    MinimumDebtNotMet,
    
    #[error("Vault is safe, cannot liquidate")]
    VaultSafe,
    
    #[error("Stability pool insufficient funds")]
    StabilityPoolInsufficientFunds,
    
    #[error("Invalid collateral type: {0}")]
    InvalidCollateralType(String),
    
    #[error("Arithmetic overflow")]
    ArithmeticOverflow,
    
    #[error("Price feed unavailable")]
    PriceFeedUnavailable,
}

pub type Result<T> = std::result::Result<T, StablecoinError>;
