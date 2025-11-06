//! Price aggregation methods

use crate::{PriceData, OracleError, Result};
use serde::{Deserialize, Serialize};

/// Aggregation method for combining oracle prices
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AggregationMethod {
    /// Simple median of all prices
    Median,
    /// Weighted median by oracle confidence
    WeightedMedian,
    /// Mean average
    Mean,
    /// Weighted mean by confidence
    WeightedMean,
}

/// Price aggregator
pub struct PriceAggregator {
    pub method: AggregationMethod,
}

impl PriceAggregator {
    pub fn new(method: AggregationMethod) -> Self {
        Self { method }
    }

    /// Aggregate prices using configured method
    pub fn aggregate(&self, prices: &[&PriceData]) -> Result<u128> {
        if prices.is_empty() {
            return Err(OracleError::InsufficientResponses {
                got: 0,
                required: 1,
            });
        }

        match self.method {
            AggregationMethod::Median => self.median(prices),
            AggregationMethod::WeightedMedian => self.weighted_median(prices),
            AggregationMethod::Mean => self.mean(prices),
            AggregationMethod::WeightedMean => self.weighted_mean(prices),
        }
    }

    fn median(&self, prices: &[&PriceData]) -> Result<u128> {
        let mut sorted: Vec<u128> = prices.iter().map(|p| p.price).collect();
        sorted.sort();

        let median = if sorted.len() % 2 == 0 {
            let mid = sorted.len() / 2;
            (sorted[mid - 1] + sorted[mid]) / 2
        } else {
            sorted[sorted.len() / 2]
        };

        Ok(median)
    }

    fn weighted_median(&self, prices: &[&PriceData]) -> Result<u128> {
        // Sort by price
        let mut weighted: Vec<(u128, u128)> = prices
            .iter()
            .map(|p| (p.price, p.confidence as u128))
            .collect();
        weighted.sort_by_key(|&(price, _)| price);

        let total_weight: u128 = weighted.iter().map(|(_, w)| w).sum();
        let half_weight = total_weight / 2;

        let mut cumulative = 0u128;
        for (price, weight) in weighted {
            cumulative += weight;
            if cumulative >= half_weight {
                return Ok(price);
            }
        }

        Ok(weighted.last().unwrap().0)
    }

    fn mean(&self, prices: &[&PriceData]) -> Result<u128> {
        let sum: u128 = prices.iter().map(|p| p.price).sum();
        Ok(sum / prices.len() as u128)
    }

    fn weighted_mean(&self, prices: &[&PriceData]) -> Result<u128> {
        let total_weight: u128 = prices.iter().map(|p| p.confidence as u128).sum();
        let weighted_sum: u128 = prices
            .iter()
            .map(|p| p.price * p.confidence as u128)
            .sum();

        Ok(weighted_sum / total_weight)
    }

    /// Calculate standard deviation of prices
    pub fn standard_deviation(&self, prices: &[&PriceData]) -> Result<u128> {
        if prices.is_empty() {
            return Ok(0);
        }

        let mean = self.mean(prices)?;
        let variance: u128 = prices
            .iter()
            .map(|p| {
                let diff = if p.price > mean {
                    p.price - mean
                } else {
                    mean - p.price
                };
                diff * diff
            })
            .sum::<u128>()
            / prices.len() as u128;

        // Simple integer square root
        Ok(integer_sqrt(variance))
    }

    /// Check if price deviation is acceptable
    pub fn check_deviation(
        &self,
        prices: &[&PriceData],
        max_deviation_bps: u16,
    ) -> Result<bool> {
        let mean = self.mean(prices)?;
        let std_dev = self.standard_deviation(prices)?;

        let deviation_bps = std_dev
            .checked_mul(10000)
            .and_then(|v| v.checked_div(mean))
            .ok_or(OracleError::ArithmeticOverflow)?;

        Ok(deviation_bps <= max_deviation_bps as u128)
    }
}

/// Integer square root using Newton's method
fn integer_sqrt(n: u128) -> u128 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use lean_crypto::Address;

    fn test_price(price: u128, confidence: u16) -> PriceData {
        PriceData::new(
            Address::from([1u8; 20]),
            price,
            0,
            Address::from([2u8; 20]),
            [0u8; 64],
            confidence,
        )
    }

    #[test]
    fn test_median() {
        let aggregator = PriceAggregator::new(AggregationMethod::Median);
        let prices = vec![
            test_price(98, 9500),
            test_price(100, 9500),
            test_price(102, 9500),
        ];
        let refs: Vec<&PriceData> = prices.iter().collect();
        let result = aggregator.aggregate(&refs).unwrap();
        assert_eq!(result, 100);
    }

    #[test]
    fn test_mean() {
        let aggregator = PriceAggregator::new(AggregationMethod::Mean);
        let prices = vec![
            test_price(100, 9500),
            test_price(200, 9500),
        ];
        let refs: Vec<&PriceData> = prices.iter().collect();
        let result = aggregator.aggregate(&refs).unwrap();
        assert_eq!(result, 150);
    }

    #[test]
    fn test_weighted_mean() {
        let aggregator = PriceAggregator::new(AggregationMethod::WeightedMean);
        let prices = vec![
            test_price(100, 5000), // 50% confidence
            test_price(200, 5000), // 50% confidence
        ];
        let refs: Vec<&PriceData> = prices.iter().collect();
        let result = aggregator.aggregate(&refs).unwrap();
        assert_eq!(result, 150);
    }
}
