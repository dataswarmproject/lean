/// Price Index Oracle System
///
/// Multi-source price aggregation with outlier detection and volume weighting.
/// Aggregates prices from CEX (30%), DEX (40%), Oracle (20%), and Index (10%) sources.

pub mod types;
pub mod aggregator;
pub mod sources;
pub mod validation;

pub use types::{PriceData, PriceSource, AggregatedPrice, PriceIndexError};
pub use aggregator::PriceIndexAggregator;
pub use sources::{CexSource, DexSource, OracleSource, IndexSource};
pub use validation::PriceValidator;
