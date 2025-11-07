/// Liquidity Management System
///
/// Manages liquidity provision across DEXs, concentrated liquidity,
/// and cross-chain deployments.

pub mod types;
pub mod pool;
pub mod incentives;
pub mod manager;

pub use types::{LiquidityPool, PoolMetrics, LiquidityError};
pub use pool::PoolManager;
pub use incentives::IncentiveProgram;
pub use manager::LiquidityManager;
