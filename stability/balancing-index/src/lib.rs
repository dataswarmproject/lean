/// Balancing Index System
///
/// Multi-asset basket for price stability and value pegging.
/// Supports dynamic weighting based on market conditions.

pub mod types;
pub mod basket;
pub mod rebalancing;
pub mod calculator;

pub use types::{Asset, AssetWeight, IndexComposition, BalancingError};
pub use basket::AssetBasket;
pub use rebalancing::{RebalancingStrategy, RebalancingEngine};
pub use calculator::TargetPriceCalculator;
