//! Price feed management

use crate::{OracleError, Result};
use lean_crypto::Address;
use serde::{Deserialize, Serialize};

/// Price data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceData {
    /// Asset identifier (address)
    pub asset: Address,
    /// Price in base units (e.g., USD with 8 decimals)
    pub price: u128,
    /// Timestamp of price update
    pub timestamp: i64,
    /// Oracle that provided this price
    pub oracle: Address,
    /// Signature over (asset, price, timestamp)
    pub signature: [u8; 64],
    /// Confidence interval (basis points)
    pub confidence: u16,
}

impl PriceData {
    /// Create new price data
    pub fn new(
        asset: Address,
        price: u128,
        timestamp: i64,
        oracle: Address,
        signature: [u8; 64],
        confidence: u16,
    ) -> Self {
        Self {
            asset,
            price,
            timestamp,
            oracle,
            signature,
            confidence,
        }
    }

    /// Check if price is stale
    pub fn is_stale(&self, current_time: i64, threshold: i64) -> bool {
        current_time - self.timestamp > threshold
    }

    /// Get age in seconds
    pub fn age(&self, current_time: i64) -> i64 {
        current_time - self.timestamp
    }
}

/// Price feed for an asset
pub struct PriceFeed {
    /// Asset identifier
    pub asset: Address,
    /// Latest price data points from different oracles
    prices: Vec<PriceData>,
    /// Maximum age for valid price (seconds)
    pub max_age: i64,
    /// Minimum number of oracle responses required
    pub min_responses: usize,
}

impl PriceFeed {
    /// Create a new price feed
    pub fn new(asset: Address, max_age: i64, min_responses: usize) -> Self {
        Self {
            asset,
            prices: Vec::new(),
            max_age,
            min_responses,
        }
    }

    /// Add price data
    pub fn add_price(&mut self, price_data: PriceData) -> Result<()> {
        if price_data.asset != self.asset {
            return Err(OracleError::FeedNotFound(
                hex::encode(price_data.asset.as_bytes()),
            ));
        }

        // Remove old price from same oracle
        self.prices.retain(|p| p.oracle != price_data.oracle);
        
        // Add new price
        self.prices.push(price_data);
        
        Ok(())
    }

    /// Get valid prices (not stale)
    pub fn valid_prices(&self, current_time: i64) -> Vec<&PriceData> {
        self.prices
            .iter()
            .filter(|p| !p.is_stale(current_time, self.max_age))
            .collect()
    }

    /// Get latest aggregated price
    pub fn latest_price(&self, current_time: i64) -> Result<u128> {
        let valid = self.valid_prices(current_time);
        
        if valid.len() < self.min_responses {
            return Err(OracleError::InsufficientResponses {
                got: valid.len(),
                required: self.min_responses,
            });
        }

        // Calculate median price
        let mut prices: Vec<u128> = valid.iter().map(|p| p.price).collect();
        prices.sort();
        
        let median = if prices.len() % 2 == 0 {
            let mid = prices.len() / 2;
            (prices[mid - 1] + prices[mid]) / 2
        } else {
            prices[prices.len() / 2]
        };

        Ok(median)
    }

    /// Get price confidence (based on standard deviation)
    pub fn confidence(&self, current_time: i64) -> Result<u16> {
        let valid = self.valid_prices(current_time);
        
        if valid.is_empty() {
            return Ok(0);
        }

        let avg_confidence: u128 = valid.iter().map(|p| p.confidence as u128).sum::<u128>() 
            / valid.len() as u128;

        Ok(avg_confidence as u16)
    }

    /// Remove stale prices
    pub fn cleanup(&mut self, current_time: i64) {
        self.prices.retain(|p| !p.is_stale(current_time, self.max_age));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_address(n: u8) -> Address {
        Address::from([n; 20])
    }

    fn test_price_data(oracle: u8, price: u128, timestamp: i64) -> PriceData {
        PriceData::new(
            test_address(1),
            price,
            timestamp,
            test_address(oracle),
            [0u8; 64],
            9500, // 95% confidence
        )
    }

    #[test]
    fn test_price_feed() {
        let mut feed = PriceFeed::new(test_address(1), 300, 3);
        
        feed.add_price(test_price_data(1, 100, 0)).unwrap();
        feed.add_price(test_price_data(2, 102, 0)).unwrap();
        feed.add_price(test_price_data(3, 98, 0)).unwrap();

        let price = feed.latest_price(100).unwrap();
        assert_eq!(price, 100); // Median of [98, 100, 102]
    }

    #[test]
    fn test_stale_price() {
        let mut feed = PriceFeed::new(test_address(1), 300, 1);
        feed.add_price(test_price_data(1, 100, 0)).unwrap();

        // At time 400, price is stale (age 400 > max 300)
        let result = feed.latest_price(400);
        assert!(result.is_err());
    }

    #[test]
    fn test_insufficient_responses() {
        let feed = PriceFeed::new(test_address(1), 300, 3);
        // No prices added
        let result = feed.latest_price(0);
        assert!(result.is_err());
    }
}
