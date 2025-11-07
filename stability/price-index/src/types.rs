use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use thiserror::Error;

/// Represents a single price data point from a source
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PriceData {
    /// The price value in the smallest unit (e.g., cents for USD, wei for ETH)
    pub price: u128,
    /// Timestamp when this price was observed
    pub timestamp: DateTime<Utc>,
    /// The source of this price
    pub source: PriceSource,
    /// Trading volume associated with this price (if available)
    pub volume: Option<u128>,
    /// Confidence score (0-10000, where 10000 = 100%)
    pub confidence: u16,
}

/// Source types for price data
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PriceSource {
    /// Centralized exchange (Binance, Coinbase, Kraken, etc.)
    CentralizedExchange,
    /// Decentralized exchange (Uniswap, PancakeSwap, etc.)
    DecentralizedExchange,
    /// Oracle network (Chainlink, Band Protocol, etc.)
    OracleNetwork,
    /// Price index provider (CoinGecko, CoinMarketCap, etc.)
    IndexProvider,
}

impl PriceSource {
    /// Get the weight for this source type (in basis points)
    pub fn weight(&self) -> u16 {
        match self {
            PriceSource::CentralizedExchange => 3000,  // 30%
            PriceSource::DecentralizedExchange => 4000, // 40%
            PriceSource::OracleNetwork => 2000,         // 20%
            PriceSource::IndexProvider => 1000,         // 10%
        }
    }
}

/// Aggregated price result with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedPrice {
    /// The final aggregated price
    pub price: u128,
    /// Timestamp of aggregation
    pub timestamp: DateTime<Utc>,
    /// Number of sources used in aggregation
    pub source_count: usize,
    /// Confidence level of the aggregated price (0-10000)
    pub confidence: u16,
    /// Standard deviation of source prices
    pub std_deviation: u128,
    /// Individual source prices that contributed
    pub sources: Vec<PriceData>,
}

impl AggregatedPrice {
    /// Check if the aggregated price is reliable
    pub fn is_reliable(&self) -> bool {
        self.source_count >= 3 && self.confidence >= 8000 // 80% confidence
    }

    /// Calculate the coefficient of variation (CV) as a measure of dispersion
    pub fn coefficient_of_variation(&self) -> u16 {
        if self.price == 0 {
            return 0;
        }
        // CV = (std_deviation / mean) * 10000 (in basis points)
        ((self.std_deviation as u128 * 10000) / self.price) as u16
    }
}

/// Errors that can occur in price index operations
#[derive(Error, Debug)]
pub enum PriceIndexError {
    #[error("Insufficient price sources: got {got}, required {required}")]
    InsufficientSources {
        got: usize,
        required: usize,
    },

    #[error("Price data is stale: age {age_seconds}s exceeds max {max_age_seconds}s")]
    StaleData {
        age_seconds: i64,
        max_age_seconds: i64,
    },

    #[error("Price deviation too high: {deviation_bps} basis points")]
    HighDeviation {
        deviation_bps: u16,
    },

    #[error("Invalid price value: {0}")]
    InvalidPrice(String),

    #[error("Outlier detection failed: {0}")]
    OutlierDetection(String),

    #[error("Calculation overflow")]
    CalculationOverflow,

    #[error("No valid prices available")]
    NoValidPrices,

    #[error("Source error: {0}")]
    SourceError(String),
}

pub type Result<T> = std::result::Result<T, PriceIndexError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_weights() {
        assert_eq!(PriceSource::CentralizedExchange.weight(), 3000);
        assert_eq!(PriceSource::DecentralizedExchange.weight(), 4000);
        assert_eq!(PriceSource::OracleNetwork.weight(), 2000);
        assert_eq!(PriceSource::IndexProvider.weight(), 1000);

        // Total should be 100%
        let total: u16 = vec![
            PriceSource::CentralizedExchange,
            PriceSource::DecentralizedExchange,
            PriceSource::OracleNetwork,
            PriceSource::IndexProvider,
        ].iter().map(|s| s.weight()).sum();
        assert_eq!(total, 10000);
    }

    #[test]
    fn test_aggregated_price_reliability() {
        let price = AggregatedPrice {
            price: 100_000_000,
            timestamp: Utc::now(),
            source_count: 4,
            confidence: 9000,
            std_deviation: 1_000_000,
            sources: vec![],
        };
        assert!(price.is_reliable());

        let unreliable_price = AggregatedPrice {
            price: 100_000_000,
            timestamp: Utc::now(),
            source_count: 2,
            confidence: 7000,
            std_deviation: 1_000_000,
            sources: vec![],
        };
        assert!(!unreliable_price.is_reliable());
    }

    #[test]
    fn test_coefficient_of_variation() {
        let price = AggregatedPrice {
            price: 100_000_000,
            timestamp: Utc::now(),
            source_count: 4,
            confidence: 9000,
            std_deviation: 5_000_000, // 5% std dev
            sources: vec![],
        };
        assert_eq!(price.coefficient_of_variation(), 500); // 5%
    }
}
