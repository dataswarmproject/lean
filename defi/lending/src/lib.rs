//! LEAN CHAIN Lending Protocol
//!
//! Over-collateralized lending protocol with interest accrual and liquidations.

mod loan;
mod collateral;
mod interest;
mod liquidation;
mod pool;

pub use loan::{Loan, LoanStatus, LoanConfig};
pub use collateral::{CollateralManager, CollateralAsset};
pub use interest::{InterestRateModel, LinearInterestModel};
pub use liquidation::{Liquidator, LiquidationResult};
pub use pool::{LendingPool, PoolStats};

use thiserror::Error;

/// Lending protocol errors
#[derive(Debug, Error)]
pub enum LendingError {
    #[error("Insufficient collateral: required {required}, provided {provided}")]
    InsufficientCollateral { required: u128, provided: u128 },
    
    #[error("Loan not found: {0}")]
    LoanNotFound(String),
    
    #[error("Health factor too low: {0}")]
    UnhealthyLoan(String),
    
    #[error("Liquidation threshold not reached")]
    LiquidationThresholdNotReached,
    
    #[error("Insufficient liquidity in pool")]
    InsufficientLiquidity,
    
    #[error("Interest calculation overflow")]
    InterestOverflow,
    
    #[error("Invalid collateral ratio: {0}")]
    InvalidCollateralRatio(u16),
    
    #[error("Asset not supported: {0}")]
    UnsupportedAsset(String),
    
    #[error("Arithmetic overflow")]
    ArithmeticOverflow,
}

pub type Result<T> = std::result::Result<T, LendingError>;
