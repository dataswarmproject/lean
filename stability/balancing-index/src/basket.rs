use crate::types::{Asset, AssetWeight, IndexComposition, BalancingError, Result};
use std::collections::HashMap;

/// Manages the asset basket and its composition
pub struct AssetBasket {
    /// Current composition of the basket
    composition: IndexComposition,
    /// Current prices for each asset (in smallest unit)
    prices: HashMap<Asset, u128>,
    /// Historical compositions
    history: Vec<IndexComposition>,
}

impl AssetBasket {
    /// Create a new basket with the default composition
    pub fn new() -> Self {
        Self {
            composition: IndexComposition::default_composition(),
            prices: HashMap::new(),
            history: vec![],
        }
    }

    /// Create a basket with a custom composition
    pub fn with_composition(composition: IndexComposition) -> Result<()> {
        composition.validate()?;
        Ok(())
    }

    /// Get the current composition
    pub fn composition(&self) -> &IndexComposition {
        &self.composition
    }

    /// Update the price for an asset
    pub fn update_price(&mut self, asset: Asset, price: u128) -> Result<()> {
        if price == 0 {
            return Err(BalancingError::CalculationError("Price cannot be zero".to_string()));
        }
        self.prices.insert(asset, price);
        Ok(())
    }

    /// Get the price for an asset
    pub fn get_price(&self, asset: &Asset) -> Result<u128> {
        self.prices
            .get(asset)
            .copied()
            .ok_or_else(|| BalancingError::PriceNotAvailable(asset.ticker().to_string()))
    }

    /// Calculate the total index value based on current prices
    pub fn calculate_index_value(&self) -> Result<u128> {
        let mut total_value: u128 = 0;

        for weight in &self.composition.weights {
            let price = self.get_price(&weight.asset)?;

            // Calculate weighted contribution
            let weighted_price = price
                .checked_mul(weight.weight_bps as u128)
                .ok_or_else(|| BalancingError::CalculationError("Overflow in price calculation".to_string()))?
                / 10000;

            total_value = total_value
                .checked_add(weighted_price)
                .ok_or_else(|| BalancingError::CalculationError("Overflow in total value".to_string()))?;
        }

        Ok(total_value)
    }

    /// Calculate the value contribution of a specific asset
    pub fn calculate_asset_value(&self, asset: &Asset) -> Result<u128> {
        let price = self.get_price(asset)?;
        let weight = self.composition
            .get_weight(asset)
            .ok_or_else(|| BalancingError::AssetNotFound(asset.ticker().to_string()))?;

        let value = price
            .checked_mul(weight as u128)
            .ok_or_else(|| BalancingError::CalculationError("Overflow in asset value".to_string()))?
            / 10000;

        Ok(value)
    }

    /// Update the basket composition
    pub fn update_composition(&mut self, new_composition: IndexComposition) -> Result<()> {
        new_composition.validate()?;

        // Store the old composition in history
        self.history.push(self.composition.clone());

        // Keep only last 100 compositions
        if self.history.len() > 100 {
            self.history.remove(0);
        }

        self.composition = new_composition;
        Ok(())
    }

    /// Get historical compositions
    pub fn get_history(&self) -> &[IndexComposition] {
        &self.history
    }

    /// Check if all required prices are available
    pub fn has_all_prices(&self) -> bool {
        self.composition.weights.iter().all(|w| self.prices.contains_key(&w.asset))
    }

    /// Get the current weights as a map
    pub fn get_weights_map(&self) -> HashMap<Asset, u16> {
        self.composition
            .weights
            .iter()
            .map(|w| (w.asset.clone(), w.weight_bps))
            .collect()
    }

    /// Calculate the deviation from target weights
    pub fn calculate_weight_deviation(&self, current_values: &HashMap<Asset, u128>) -> Result<HashMap<Asset, i32>> {
        let mut deviations = HashMap::new();

        // Calculate total value
        let total_value: u128 = current_values.values().sum();

        if total_value == 0 {
            return Err(BalancingError::CalculationError("Total value is zero".to_string()));
        }

        for weight in &self.composition.weights {
            let current_value = current_values
                .get(&weight.asset)
                .copied()
                .unwrap_or(0);

            // Calculate current weight in basis points
            let current_weight_bps = ((current_value * 10000) / total_value) as i32;

            // Calculate deviation from target
            let deviation = current_weight_bps - weight.weight_bps as i32;

            deviations.insert(weight.asset.clone(), deviation);
        }

        Ok(deviations)
    }

    /// Check if rebalancing is needed based on weight deviations
    pub fn needs_rebalancing(&self, threshold_bps: u16) -> Result<bool> {
        if !self.has_all_prices() {
            return Ok(false);
        }

        // Calculate current values for each asset
        let mut current_values = HashMap::new();
        for weight in &self.composition.weights {
            let value = self.calculate_asset_value(&weight.asset)?;
            current_values.insert(weight.asset.clone(), value);
        }

        // Check deviations
        let deviations = self.calculate_weight_deviation(&current_values)?;

        for deviation in deviations.values() {
            if deviation.abs() as u16 > threshold_bps {
                return Ok(true);
            }
        }

        Ok(false)
    }
}

impl Default for AssetBasket {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basket_creation() {
        let basket = AssetBasket::new();
        assert_eq!(basket.composition().weights.len(), 5);
    }

    #[test]
    fn test_price_updates() {
        let mut basket = AssetBasket::new();

        assert!(basket.update_price(Asset::USD, 100_000_000).is_ok());
        assert_eq!(basket.get_price(&Asset::USD).unwrap(), 100_000_000);

        // Zero price should error
        assert!(basket.update_price(Asset::BTC, 0).is_err());
    }

    #[test]
    fn test_index_value_calculation() {
        let mut basket = AssetBasket::new();

        // Set simple prices
        basket.update_price(Asset::USD, 1_000_000).unwrap();    // $1
        basket.update_price(Asset::BTC, 50_000_000_000).unwrap(); // $50,000
        basket.update_price(Asset::ETH, 3_000_000_000).unwrap();  // $3,000
        basket.update_price(Asset::Gold, 2_000_000_000).unwrap(); // $2,000/oz
        basket.update_price(Asset::SP500, 4_500_000_000).unwrap(); // $4,500

        let index_value = basket.calculate_index_value().unwrap();

        // Expected: 0.40*$1 + 0.25*$50k + 0.20*$3k + 0.10*$2k + 0.05*$4.5k
        //         = $0.40 + $12,500 + $600 + $200 + $225 = $13,525.40
        assert!(index_value > 13_000_000_000); // > $13,000
        assert!(index_value < 14_000_000_000); // < $14,000
    }

    #[test]
    fn test_asset_value_calculation() {
        let mut basket = AssetBasket::new();
        basket.update_price(Asset::USD, 1_000_000).unwrap();

        let value = basket.calculate_asset_value(&Asset::USD).unwrap();

        // USD is 40% of basket, so contribution is 0.40 * $1 = $0.40
        assert_eq!(value, 400_000);
    }

    #[test]
    fn test_composition_update() {
        let mut basket = AssetBasket::new();
        let initial_version = basket.composition().version;

        let new_composition = IndexComposition::new(vec![
            AssetWeight::new(Asset::USD, 5000),
            AssetWeight::new(Asset::BTC, 5000),
        ]).unwrap();

        basket.update_composition(new_composition).unwrap();

        assert_eq!(basket.composition().version, initial_version + 1);
        assert_eq!(basket.history.len(), 1);
    }

    #[test]
    fn test_has_all_prices() {
        let mut basket = AssetBasket::new();
        assert!(!basket.has_all_prices());

        basket.update_price(Asset::USD, 1_000_000).unwrap();
        basket.update_price(Asset::BTC, 50_000_000_000).unwrap();
        basket.update_price(Asset::ETH, 3_000_000_000).unwrap();
        basket.update_price(Asset::Gold, 2_000_000_000).unwrap();
        basket.update_price(Asset::SP500, 4_500_000_000).unwrap();

        assert!(basket.has_all_prices());
    }

    #[test]
    fn test_weight_deviation_calculation() {
        let basket = AssetBasket::new();

        let mut current_values = HashMap::new();
        current_values.insert(Asset::USD, 5000);  // 50% (target: 40%)
        current_values.insert(Asset::BTC, 2000);  // 20% (target: 25%)
        current_values.insert(Asset::ETH, 1500);  // 15% (target: 20%)
        current_values.insert(Asset::Gold, 1000); // 10% (target: 10%)
        current_values.insert(Asset::SP500, 500); // 5% (target: 5%)

        let deviations = basket.calculate_weight_deviation(&current_values).unwrap();

        // USD: 50% - 40% = +10% = +1000bps
        assert_eq!(deviations.get(&Asset::USD), Some(&1000));

        // BTC: 20% - 25% = -5% = -500bps
        assert_eq!(deviations.get(&Asset::BTC), Some(&-500));
    }

    #[test]
    fn test_needs_rebalancing() {
        let mut basket = AssetBasket::new();

        // Set prices such that weights are significantly off
        basket.update_price(Asset::USD, 2_000_000).unwrap();    // Double the normal
        basket.update_price(Asset::BTC, 50_000_000_000).unwrap();
        basket.update_price(Asset::ETH, 3_000_000_000).unwrap();
        basket.update_price(Asset::Gold, 2_000_000_000).unwrap();
        basket.update_price(Asset::SP500, 4_500_000_000).unwrap();

        // With 5% threshold (500bps), this should trigger rebalancing
        let needs_rebalancing = basket.needs_rebalancing(500).unwrap();
        assert!(needs_rebalancing);
    }
}
