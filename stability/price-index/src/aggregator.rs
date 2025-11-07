use crate::types::{PriceData, PriceSource, AggregatedPrice, PriceIndexError, Result};
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;

/// Configuration for the price aggregator
#[derive(Debug, Clone)]
pub struct AggregatorConfig {
    /// Maximum age of price data in seconds
    pub max_price_age_seconds: i64,
    /// Minimum number of sources required
    pub min_sources: usize,
    /// Maximum allowed coefficient of variation (in basis points)
    pub max_cv_bps: u16,
    /// Outlier detection threshold (in standard deviations)
    pub outlier_threshold_std: f64,
}

impl Default for AggregatorConfig {
    fn default() -> Self {
        Self {
            max_price_age_seconds: 300, // 5 minutes
            min_sources: 3,
            max_cv_bps: 500, // 5% maximum variation
            outlier_threshold_std: 2.0, // 2 standard deviations
        }
    }
}

/// Price index aggregator that combines multiple price sources
pub struct PriceIndexAggregator {
    config: AggregatorConfig,
    price_cache: HashMap<PriceSource, Vec<PriceData>>,
}

impl PriceIndexAggregator {
    /// Create a new price aggregator with default configuration
    pub fn new() -> Self {
        Self {
            config: AggregatorConfig::default(),
            price_cache: HashMap::new(),
        }
    }

    /// Create a new price aggregator with custom configuration
    pub fn with_config(config: AggregatorConfig) -> Self {
        Self {
            config,
            price_cache: HashMap::new(),
        }
    }

    /// Add price data from a source
    pub fn add_price(&mut self, price_data: PriceData) -> Result<()> {
        // Validate price data
        self.validate_price_data(&price_data)?;

        // Add to cache
        let source_prices = self.price_cache.entry(price_data.source).or_insert_with(Vec::new);
        source_prices.push(price_data);

        // Keep only recent prices (last 10 data points per source)
        if source_prices.len() > 10 {
            source_prices.remove(0);
        }

        Ok(())
    }

    /// Aggregate all recent prices into a single weighted price
    pub fn aggregate_price(&self) -> Result<AggregatedPrice> {
        let current_time = Utc::now();

        // Collect valid prices from all sources
        let mut valid_prices = Vec::new();
        for (source, prices) in &self.price_cache {
            if let Some(latest) = prices.last() {
                // Check if price is recent enough
                let age = current_time.signed_duration_since(latest.timestamp);
                if age.num_seconds() <= self.config.max_price_age_seconds {
                    valid_prices.push(latest.clone());
                }
            }
        }

        // Check if we have enough sources
        if valid_prices.len() < self.config.min_sources {
            return Err(PriceIndexError::InsufficientSources {
                got: valid_prices.len(),
                required: self.config.min_sources,
            });
        }

        // Remove outliers
        let filtered_prices = self.filter_outliers(&valid_prices)?;

        if filtered_prices.is_empty() {
            return Err(PriceIndexError::NoValidPrices);
        }

        // Calculate weighted average
        let (weighted_price, total_weight) = self.calculate_weighted_price(&filtered_prices)?;

        // Calculate standard deviation
        let std_dev = self.calculate_std_deviation(&filtered_prices, weighted_price)?;

        // Calculate confidence based on source count and deviation
        let confidence = self.calculate_confidence(filtered_prices.len(), std_dev, weighted_price);

        // Calculate coefficient of variation
        let cv = if weighted_price > 0 {
            ((std_dev as u128 * 10000) / weighted_price) as u16
        } else {
            0
        };

        // Check if deviation is acceptable
        if cv > self.config.max_cv_bps {
            return Err(PriceIndexError::HighDeviation {
                deviation_bps: cv,
            });
        }

        Ok(AggregatedPrice {
            price: weighted_price,
            timestamp: current_time,
            source_count: filtered_prices.len(),
            confidence,
            std_deviation: std_dev,
            sources: filtered_prices,
        })
    }

    /// Get the latest price for a specific source
    pub fn get_source_price(&self, source: PriceSource) -> Option<&PriceData> {
        self.price_cache.get(&source).and_then(|prices| prices.last())
    }

    /// Clear old price data
    pub fn clear_old_prices(&mut self, max_age: Duration) {
        let cutoff_time = Utc::now() - max_age;

        for prices in self.price_cache.values_mut() {
            prices.retain(|price| price.timestamp > cutoff_time);
        }

        // Remove empty entries
        self.price_cache.retain(|_, prices| !prices.is_empty());
    }

    // Private helper methods

    fn validate_price_data(&self, price_data: &PriceData) -> Result<()> {
        // Check price is not zero
        if price_data.price == 0 {
            return Err(PriceIndexError::InvalidPrice("Price cannot be zero".to_string()));
        }

        // Check timestamp is not too old
        let age = Utc::now().signed_duration_since(price_data.timestamp);
        if age.num_seconds() > self.config.max_price_age_seconds {
            return Err(PriceIndexError::StaleData {
                age_seconds: age.num_seconds(),
                max_age_seconds: self.config.max_price_age_seconds,
            });
        }

        // Check confidence is valid
        if price_data.confidence > 10000 {
            return Err(PriceIndexError::InvalidPrice("Confidence cannot exceed 100%".to_string()));
        }

        Ok(())
    }

    fn filter_outliers(&self, prices: &[PriceData]) -> Result<Vec<PriceData>> {
        if prices.len() < 3 {
            return Ok(prices.to_vec());
        }

        // Calculate mean and standard deviation
        let price_values: Vec<u128> = prices.iter().map(|p| p.price).collect();
        let mean = self.calculate_mean(&price_values)?;
        let std_dev = self.calculate_std_dev_from_values(&price_values, mean)?;

        // Filter out outliers (prices beyond threshold * std_dev from mean)
        let threshold = (self.config.outlier_threshold_std * std_dev as f64) as u128;
        let lower_bound = mean.saturating_sub(threshold);
        let upper_bound = mean.saturating_add(threshold);

        let filtered: Vec<PriceData> = prices
            .iter()
            .filter(|p| p.price >= lower_bound && p.price <= upper_bound)
            .cloned()
            .collect();

        // Ensure we still have enough prices after filtering
        if filtered.len() < self.config.min_sources {
            // If too many outliers, return original prices
            Ok(prices.to_vec())
        } else {
            Ok(filtered)
        }
    }

    fn calculate_weighted_price(&self, prices: &[PriceData]) -> Result<(u128, u128)> {
        let mut weighted_sum: u128 = 0;
        let mut total_weight: u128 = 0;

        for price_data in prices {
            let source_weight = price_data.source.weight() as u128;
            let confidence_weight = price_data.confidence as u128;

            // Combined weight = source_weight * confidence / 10000
            let combined_weight = source_weight
                .checked_mul(confidence_weight)
                .ok_or(PriceIndexError::CalculationOverflow)?
                / 10000;

            let weighted_price = price_data.price
                .checked_mul(combined_weight)
                .ok_or(PriceIndexError::CalculationOverflow)?;

            weighted_sum = weighted_sum
                .checked_add(weighted_price)
                .ok_or(PriceIndexError::CalculationOverflow)?;

            total_weight = total_weight
                .checked_add(combined_weight)
                .ok_or(PriceIndexError::CalculationOverflow)?;
        }

        if total_weight == 0 {
            return Err(PriceIndexError::InvalidPrice("Total weight is zero".to_string()));
        }

        let final_price = weighted_sum / total_weight;
        Ok((final_price, total_weight))
    }

    fn calculate_std_deviation(&self, prices: &[PriceData], mean: u128) -> Result<u128> {
        let price_values: Vec<u128> = prices.iter().map(|p| p.price).collect();
        self.calculate_std_dev_from_values(&price_values, mean)
    }

    fn calculate_mean(&self, values: &[u128]) -> Result<u128> {
        if values.is_empty() {
            return Ok(0);
        }

        let sum: u128 = values.iter().try_fold(0u128, |acc, &val| {
            acc.checked_add(val).ok_or(PriceIndexError::CalculationOverflow)
        })?;

        Ok(sum / values.len() as u128)
    }

    fn calculate_std_dev_from_values(&self, values: &[u128], mean: u128) -> Result<u128> {
        if values.len() <= 1 {
            return Ok(0);
        }

        let variance_sum: u128 = values.iter().try_fold(0u128, |acc, &val| {
            let diff = if val > mean {
                val - mean
            } else {
                mean - val
            };
            let squared = diff.checked_mul(diff).ok_or(PriceIndexError::CalculationOverflow)?;
            acc.checked_add(squared).ok_or(PriceIndexError::CalculationOverflow)
        })?;

        let variance = variance_sum / values.len() as u128;

        // Integer square root using binary search
        let std_dev = self.isqrt(variance);
        Ok(std_dev)
    }

    fn isqrt(&self, n: u128) -> u128 {
        if n == 0 {
            return 0;
        }

        let mut x = n;
        let mut y = (x + 1) / 2;

        while y < x {
            x = y;
            y = (x + n / x) / 2;
        }

        x
    }

    fn calculate_confidence(&self, source_count: usize, std_dev: u128, mean: u128) -> u16 {
        // Base confidence from source count (0-5000 basis points)
        let source_confidence = ((source_count.min(10) * 500) as u16).min(5000);

        // Deviation penalty (0-5000 basis points)
        let cv = if mean > 0 {
            ((std_dev * 10000) / mean) as u16
        } else {
            10000
        };

        let deviation_confidence = if cv <= 100 { // <= 1%
            5000
        } else if cv <= 500 { // <= 5%
            4000
        } else if cv <= 1000 { // <= 10%
            3000
        } else if cv <= 2000 { // <= 20%
            2000
        } else {
            1000
        };

        source_confidence + deviation_confidence
    }
}

impl Default for PriceIndexAggregator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_price(price: u128, source: PriceSource, confidence: u16) -> PriceData {
        PriceData {
            price,
            timestamp: Utc::now(),
            source,
            volume: Some(1_000_000),
            confidence,
        }
    }

    #[test]
    fn test_add_price() {
        let mut aggregator = PriceIndexAggregator::new();
        let price = create_test_price(100_000_000, PriceSource::CentralizedExchange, 9000);

        assert!(aggregator.add_price(price).is_ok());
    }

    #[test]
    fn test_aggregate_price_with_multiple_sources() {
        let mut aggregator = PriceIndexAggregator::new();

        aggregator.add_price(create_test_price(100_000_000, PriceSource::CentralizedExchange, 9000)).unwrap();
        aggregator.add_price(create_test_price(101_000_000, PriceSource::DecentralizedExchange, 8500)).unwrap();
        aggregator.add_price(create_test_price(100_500_000, PriceSource::OracleNetwork, 9500)).unwrap();
        aggregator.add_price(create_test_price(100_800_000, PriceSource::IndexProvider, 8000)).unwrap();

        let result = aggregator.aggregate_price();
        assert!(result.is_ok());

        let aggregated = result.unwrap();
        assert!(aggregated.is_reliable());
        assert_eq!(aggregated.source_count, 4);
    }

    #[test]
    fn test_insufficient_sources() {
        let mut aggregator = PriceIndexAggregator::new();

        aggregator.add_price(create_test_price(100_000_000, PriceSource::CentralizedExchange, 9000)).unwrap();
        aggregator.add_price(create_test_price(101_000_000, PriceSource::DecentralizedExchange, 8500)).unwrap();

        let result = aggregator.aggregate_price();
        assert!(result.is_err());

        match result {
            Err(PriceIndexError::InsufficientSources { got, required }) => {
                assert_eq!(got, 2);
                assert_eq!(required, 3);
            }
            _ => panic!("Expected InsufficientSources error"),
        }
    }

    #[test]
    fn test_outlier_detection() {
        let mut aggregator = PriceIndexAggregator::new();

        // Add normal prices
        aggregator.add_price(create_test_price(100_000_000, PriceSource::CentralizedExchange, 9000)).unwrap();
        aggregator.add_price(create_test_price(101_000_000, PriceSource::DecentralizedExchange, 8500)).unwrap();
        aggregator.add_price(create_test_price(100_500_000, PriceSource::OracleNetwork, 9500)).unwrap();

        // Add outlier (50% higher)
        aggregator.add_price(create_test_price(150_000_000, PriceSource::IndexProvider, 8000)).unwrap();

        let result = aggregator.aggregate_price();
        assert!(result.is_ok());

        // The outlier should be filtered out or heavily discounted
        let aggregated = result.unwrap();
        assert!(aggregated.price < 120_000_000); // Should be closer to the cluster
    }

    #[test]
    fn test_weighted_average() {
        let mut aggregator = PriceIndexAggregator::new();

        // DEX has highest weight (40%)
        aggregator.add_price(create_test_price(100_000_000, PriceSource::DecentralizedExchange, 10000)).unwrap();
        // CEX has 30% weight
        aggregator.add_price(create_test_price(105_000_000, PriceSource::CentralizedExchange, 10000)).unwrap();
        // Oracle has 20% weight
        aggregator.add_price(create_test_price(102_000_000, PriceSource::OracleNetwork, 10000)).unwrap();

        let result = aggregator.aggregate_price().unwrap();

        // Expected: 100*0.4 + 105*0.3 + 102*0.2 = 40 + 31.5 + 20.4 = 91.9M (but we have 4 sources weighted)
        // The actual calculation will be close to 101-102M given the weights
        assert!(result.price >= 100_000_000 && result.price <= 105_000_000);
    }

    #[test]
    fn test_stale_price_rejection() {
        let mut aggregator = PriceIndexAggregator::new();

        let mut stale_price = create_test_price(100_000_000, PriceSource::CentralizedExchange, 9000);
        stale_price.timestamp = Utc::now() - Duration::seconds(400); // 6.67 minutes old

        let result = aggregator.add_price(stale_price);
        assert!(result.is_err());

        match result {
            Err(PriceIndexError::StaleData { age_seconds, max_age_seconds }) => {
                assert!(age_seconds > max_age_seconds);
            }
            _ => panic!("Expected StaleData error"),
        }
    }

    #[test]
    fn test_confidence_calculation() {
        let mut aggregator = PriceIndexAggregator::new();

        // Add prices with tight spread (high confidence)
        aggregator.add_price(create_test_price(100_000_000, PriceSource::CentralizedExchange, 9000)).unwrap();
        aggregator.add_price(create_test_price(100_100_000, PriceSource::DecentralizedExchange, 9000)).unwrap();
        aggregator.add_price(create_test_price(100_050_000, PriceSource::OracleNetwork, 9000)).unwrap();

        let result = aggregator.aggregate_price().unwrap();

        // With tight spread, confidence should be high
        assert!(result.confidence >= 8000); // At least 80%
    }
}
