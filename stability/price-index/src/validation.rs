use crate::types::{PriceData, AggregatedPrice, PriceIndexError, Result};
use chrono::{DateTime, Utc};

/// Validates price data for quality and reliability
pub struct PriceValidator {
    /// Maximum allowed price change per second (in basis points)
    max_change_per_second_bps: u32,
    /// Minimum confidence threshold (0-10000)
    min_confidence: u16,
    /// Maximum staleness in seconds
    max_staleness_seconds: i64,
}

impl PriceValidator {
    /// Create a new validator with default settings
    pub fn new() -> Self {
        Self {
            max_change_per_second_bps: 50, // 0.5% per second max
            min_confidence: 7000,            // 70% minimum confidence
            max_staleness_seconds: 300,      // 5 minutes
        }
    }

    /// Create a validator with custom settings
    pub fn with_config(
        max_change_per_second_bps: u32,
        min_confidence: u16,
        max_staleness_seconds: i64,
    ) -> Self {
        Self {
            max_change_per_second_bps,
            min_confidence,
            max_staleness_seconds,
        }
    }

    /// Validate a single price data point
    pub fn validate_price(&self, price: &PriceData) -> Result<()> {
        // Check price is non-zero
        if price.price == 0 {
            return Err(PriceIndexError::InvalidPrice("Price cannot be zero".to_string()));
        }

        // Check confidence meets minimum threshold
        if price.confidence < self.min_confidence {
            return Err(PriceIndexError::InvalidPrice(
                format!("Confidence {} below minimum {}", price.confidence, self.min_confidence)
            ));
        }

        // Check staleness
        let age = Utc::now().signed_duration_since(price.timestamp);
        if age.num_seconds() > self.max_staleness_seconds {
            return Err(PriceIndexError::StaleData {
                age_seconds: age.num_seconds(),
                max_age_seconds: self.max_staleness_seconds,
            });
        }

        Ok(())
    }

    /// Validate price change rate
    pub fn validate_price_change(
        &self,
        old_price: &PriceData,
        new_price: &PriceData,
    ) -> Result<()> {
        let time_diff = new_price.timestamp.signed_duration_since(old_price.timestamp);
        let time_seconds = time_diff.num_seconds();

        if time_seconds <= 0 {
            return Err(PriceIndexError::InvalidPrice("New price timestamp must be after old price".to_string()));
        }

        // Calculate price change in basis points
        let price_diff = if new_price.price > old_price.price {
            new_price.price - old_price.price
        } else {
            old_price.price - new_price.price
        };

        let change_bps = (price_diff as u128 * 10000) / old_price.price as u128;

        // Calculate maximum allowed change for this time period
        let max_allowed_change = self.max_change_per_second_bps as u128 * time_seconds as u128;

        if change_bps > max_allowed_change {
            return Err(PriceIndexError::InvalidPrice(
                format!(
                    "Price change {}bps exceeds maximum {}bps for {}s period",
                    change_bps, max_allowed_change, time_seconds
                )
            ));
        }

        Ok(())
    }

    /// Validate an aggregated price result
    pub fn validate_aggregated(&self, aggregated: &AggregatedPrice) -> Result<()> {
        // Check price is non-zero
        if aggregated.price == 0 {
            return Err(PriceIndexError::InvalidPrice("Aggregated price cannot be zero".to_string()));
        }

        // Check confidence
        if aggregated.confidence < self.min_confidence {
            return Err(PriceIndexError::InvalidPrice(
                format!("Aggregated confidence {} below minimum {}",
                    aggregated.confidence, self.min_confidence)
            ));
        }

        // Check coefficient of variation (should be below 10% ideally)
        let cv = aggregated.coefficient_of_variation();
        if cv > 1000 { // 10%
            return Err(PriceIndexError::HighDeviation {
                deviation_bps: cv,
            });
        }

        Ok(())
    }

    /// Check if two prices are within acceptable deviation
    pub fn prices_within_deviation(&self, price1: u128, price2: u128, max_deviation_bps: u16) -> bool {
        let diff = if price1 > price2 {
            price1 - price2
        } else {
            price2 - price1
        };

        let larger = price1.max(price2);
        if larger == 0 {
            return false;
        }

        let deviation_bps = ((diff * 10000) / larger) as u16;
        deviation_bps <= max_deviation_bps
    }

    /// Validate that a price is within expected bounds
    pub fn validate_price_bounds(
        &self,
        price: u128,
        expected_price: u128,
        max_deviation_bps: u16,
    ) -> Result<()> {
        if !self.prices_within_deviation(price, expected_price, max_deviation_bps) {
            let diff = if price > expected_price {
                price - expected_price
            } else {
                expected_price - price
            };
            let actual_deviation = ((diff * 10000) / expected_price) as u16;

            return Err(PriceIndexError::HighDeviation {
                deviation_bps: actual_deviation,
            });
        }

        Ok(())
    }
}

impl Default for PriceValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::PriceSource;

    fn create_test_price(price: u128, confidence: u16) -> PriceData {
        PriceData {
            price,
            timestamp: Utc::now(),
            source: PriceSource::CentralizedExchange,
            volume: Some(1_000_000),
            confidence,
        }
    }

    #[test]
    fn test_validate_valid_price() {
        let validator = PriceValidator::new();
        let price = create_test_price(100_000_000, 9000);

        assert!(validator.validate_price(&price).is_ok());
    }

    #[test]
    fn test_validate_zero_price() {
        let validator = PriceValidator::new();
        let price = create_test_price(0, 9000);

        assert!(validator.validate_price(&price).is_err());
    }

    #[test]
    fn test_validate_low_confidence() {
        let validator = PriceValidator::new();
        let price = create_test_price(100_000_000, 5000); // 50% confidence

        let result = validator.validate_price(&price);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_price_change() {
        let validator = PriceValidator::new();
        let old_price = create_test_price(100_000_000, 9000);

        // Create new price 10 seconds later with 0.4% increase (within 0.5% per second limit)
        let mut new_price = create_test_price(100_400_000, 9000);
        new_price.timestamp = old_price.timestamp + chrono::Duration::seconds(10);

        assert!(validator.validate_price_change(&old_price, &new_price).is_ok());
    }

    #[test]
    fn test_validate_excessive_price_change() {
        let validator = PriceValidator::new();
        let old_price = create_test_price(100_000_000, 9000);

        // Create new price 1 second later with 10% increase (exceeds 0.5% per second limit)
        let mut new_price = create_test_price(110_000_000, 9000);
        new_price.timestamp = old_price.timestamp + chrono::Duration::seconds(1);

        assert!(validator.validate_price_change(&old_price, &new_price).is_err());
    }

    #[test]
    fn test_prices_within_deviation() {
        let validator = PriceValidator::new();

        // 1% deviation
        assert!(validator.prices_within_deviation(100_000_000, 101_000_000, 200)); // 2% max

        // 5% deviation
        assert!(!validator.prices_within_deviation(100_000_000, 105_000_000, 200)); // 2% max
    }

    #[test]
    fn test_validate_price_bounds() {
        let validator = PriceValidator::new();

        // Within 5% bounds
        assert!(validator.validate_price_bounds(100_000_000, 102_000_000, 500).is_ok());

        // Outside 5% bounds
        assert!(validator.validate_price_bounds(100_000_000, 110_000_000, 500).is_err());
    }

    #[test]
    fn test_validate_aggregated_price() {
        let validator = PriceValidator::new();

        let aggregated = AggregatedPrice {
            price: 100_000_000,
            timestamp: Utc::now(),
            source_count: 4,
            confidence: 9000,
            std_deviation: 500_000, // 0.5% deviation
            sources: vec![],
        };

        assert!(validator.validate_aggregated(&aggregated).is_ok());
    }

    #[test]
    fn test_validate_aggregated_high_deviation() {
        let validator = PriceValidator::new();

        let aggregated = AggregatedPrice {
            price: 100_000_000,
            timestamp: Utc::now(),
            source_count: 4,
            confidence: 9000,
            std_deviation: 12_000_000, // 12% deviation
            sources: vec![],
        };

        assert!(validator.validate_aggregated(&aggregated).is_err());
    }
}
