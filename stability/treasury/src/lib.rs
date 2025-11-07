/// Treasury Operations System
///
/// Manages treasury operations including buy-backs, market making,
/// and asset diversification for price stability.

pub mod types;
pub mod operations;
pub mod allocation;
pub mod manager;

pub use types::{TreasuryAsset, TreasuryOperation, TreasuryError};
pub use operations::{BuyBackProgram, MarketMaker};
pub use allocation::AssetAllocator;
pub use manager::TreasuryManager;
