use crate::types::{Asset, IndexComposition, BalancingError, Result};
use crate::basket::AssetBasket;

/// Calculates target prices based on the index composition
pub struct TargetPriceCalculator {
    /// The asset basket to calculate from
    basket: AssetBasket,
}

impl TargetPriceCalculator {
    /// Create a new target price calculator
    pub fn new(basket: AssetBasket) -> Self {
        Self { basket }
    }

    /// Calculate the target price for LEAN token based on current index
    pub fn calculate_target_price(&self) -> Result<u128> {
        self.basket.calculate_index_value()
    }

    /// Calculate the deviation from target price
    pub fn calculate_price_deviation(&self, current_price: u128) -> Result<i128> {
        let target_price = self.calculate_target_price()?;

        // Calculate deviation in basis points
        let deviation = if current_price > target_price {
            let diff = current_price - target_price;
            ((diff as i128 * 10000) / target_price as i128)
        } else {
            let diff = target_price - current_price;
            -((diff as i128 * 10000) / target_price as i128)
        };

        Ok(deviation)
    }

    /// Check if current price is within acceptable bounds
    pub fn is_within_bounds(&self, current_price: u128, tolerance_bps: u16) -> Result<bool> {
        let deviation = self.calculate_price_deviation(current_price)?;
        Ok(deviation.abs() <= tolerance_bps as i128)
    }

    /// Calculate the required supply adjustment to reach target price
    /// Returns positive for expansion (mint), negative for contraction (burn)
    pub fn calculate_supply_adjustment(
        &self,
        current_price: u128,
        current_supply: u128,
    ) -> Result<i128> {
        let target_price = self.calculate_target_price()?;

        if target_price == 0 {
            return Err(BalancingError::CalculationError("Target price is zero".to_string()));
        }

        // Using simple economics: if price is too high, need to increase supply
        // If price is too low, need to decrease supply
        let price_ratio = (current_price as f64) / (target_price as f64);

        // Supply adjustment is inverse to price deviation
        // If price is 10% above target, increase supply by ~10%
        // If price is 10% below target, decrease supply by ~10%
        let supply_adjustment_ratio = price_ratio - 1.0;

        let adjustment = (current_supply as f64 * supply_adjustment_ratio) as i128;

        Ok(adjustment)
    }

    /// Get the current basket composition
    pub fn composition(&self) -> &IndexComposition {
        self.basket.composition()
    }

    /// Update the basket for calculations
    pub fn update_basket(&mut self, basket: AssetBasket) {
        self.basket = basket;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::AssetWeight;

    fn create_test_basket() -> AssetBasket {
        let mut basket = AssetBasket::new();

        // Set simple prices for testing
        basket.update_price(Asset::USD, 1_000_000).unwrap();         // $1
        basket.update_price(Asset::BTC, 50_000_000_000).unwrap();    // $50,000
        basket.update_price(Asset::ETH, 3_000_000_000).unwrap();     // $3,000
        basket.update_price(Asset::Gold, 2_000_000_000).unwrap();    // $2,000
        basket.update_price(Asset::SP500, 4_500_000_000).unwrap();   // $4,500

        basket
    }

    #[test]
    fn test_calculate_target_price() {
        let basket = create_test_basket();
        let calculator = TargetPriceCalculator::new(basket);

        let target = calculator.calculate_target_price().unwrap();

        // Expected: 0.40*$1 + 0.25*$50k + 0.20*$3k + 0.10*$2k + 0.05*$4.5k
        //         = $0.40 + $12,500 + $600 + $200 + $225 = $13,525.40
        assert!(target > 13_000_000_000); // > $13,000
        assert!(target < 14_000_000_000); // < $14,000
    }

    #[test]
    fn test_calculate_price_deviation() {
        let basket = create_test_basket();
        let calculator = TargetPriceCalculator::new(basket);

        let target = calculator.calculate_target_price().unwrap();

        // Test positive deviation (price above target)
        let high_price = target + (target / 10); // 10% above
        let deviation = calculator.calculate_price_deviation(high_price).unwrap();
        assert!(deviation > 900 && deviation < 1100); // Should be ~1000bps (10%)

        // Test negative deviation (price below target)
        let low_price = target - (target / 10); // 10% below
        let deviation = calculator.calculate_price_deviation(low_price).unwrap();
        assert!(deviation < -900 && deviation > -1100); // Should be ~-1000bps (-10%)

        // Test exact match
        let deviation = calculator.calculate_price_deviation(target).unwrap();
        assert_eq!(deviation, 0);
    }

    #[test]
    fn test_is_within_bounds() {
        let basket = create_test_basket();
        let calculator = TargetPriceCalculator::new(basket);

        let target = calculator.calculate_target_price().unwrap();

        // Within 5% bounds
        let price_5_percent_above = target + (target / 20);
        assert!(calculator.is_within_bounds(price_5_percent_above, 500).unwrap());

        // Outside 5% bounds
        let price_10_percent_above = target + (target / 10);
        assert!(!calculator.is_within_bounds(price_10_percent_above, 500).unwrap());
    }

    #[test]
    fn test_calculate_supply_adjustment() {
        let basket = create_test_basket();
        let calculator = TargetPriceCalculator::new(basket);

        let target = calculator.calculate_target_price().unwrap();
        let current_supply = 1_000_000_000_000; // 1 trillion tokens

        // Price is 10% above target -> need to increase supply
        let high_price = target + (target / 10);
        let adjustment = calculator.calculate_supply_adjustment(high_price, current_supply).unwrap();
        assert!(adjustment > 0); // Should be positive (expansion)
        assert!(adjustment > 90_000_000_000); // Should be roughly 10% of supply

        // Price is 10% below target -> need to decrease supply
        let low_price = target - (target / 10);
        let adjustment = calculator.calculate_supply_adjustment(low_price, current_supply).unwrap();
        assert!(adjustment < 0); // Should be negative (contraction)
        assert!(adjustment < -90_000_000_000); // Should be roughly -10% of supply

        // Price at target -> no adjustment needed
        let adjustment = calculator.calculate_supply_adjustment(target, current_supply).unwrap();
        assert_eq!(adjustment, 0);
    }

    #[test]
    fn test_composition_access() {
        let basket = create_test_basket();
        let calculator = TargetPriceCalculator::new(basket);

        let composition = calculator.composition();
        assert_eq!(composition.weights.len(), 5);
    }

    #[test]
    fn test_update_basket() {
        let basket1 = create_test_basket();
        let mut calculator = TargetPriceCalculator::new(basket1);

        let target1 = calculator.calculate_target_price().unwrap();

        // Create a new basket with different prices
        let mut basket2 = AssetBasket::new();
        basket2.update_price(Asset::USD, 1_000_000).unwrap();
        basket2.update_price(Asset::BTC, 60_000_000_000).unwrap(); // Higher BTC price
        basket2.update_price(Asset::ETH, 3_500_000_000).unwrap();  // Higher ETH price
        basket2.update_price(Asset::Gold, 2_100_000_000).unwrap();
        basket2.update_price(Asset::SP500, 4_600_000_000).unwrap();

        calculator.update_basket(basket2);

        let target2 = calculator.calculate_target_price().unwrap();

        // New target should be higher due to increased crypto prices
        assert!(target2 > target1);
    }
}
