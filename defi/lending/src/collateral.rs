//! Collateral management

use crate::{LendingError, Result};
use lean_crypto::Address;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Collateral asset configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollateralAsset {
    /// Asset address (token contract)
    pub address: Address,
    /// Collateral factor in basis points (e.g., 7500 = 75%)
    /// This is the maximum LTV (loan-to-value) ratio
    pub collateral_factor_bps: u16,
    /// Liquidation threshold in basis points
    pub liquidation_threshold_bps: u16,
    /// Liquidation bonus for liquidators (in basis points)
    pub liquidation_bonus_bps: u16,
    /// Whether asset is enabled as collateral
    pub enabled: bool,
}

impl CollateralAsset {
    /// Create a new collateral asset configuration
    pub fn new(
        address: Address,
        collateral_factor_bps: u16,
        liquidation_threshold_bps: u16,
        liquidation_bonus_bps: u16,
    ) -> Result<Self> {
        if collateral_factor_bps > 10000 {
            return Err(LendingError::InvalidCollateralRatio(collateral_factor_bps));
        }
        if liquidation_threshold_bps > 10000 {
            return Err(LendingError::InvalidCollateralRatio(liquidation_threshold_bps));
        }
        if collateral_factor_bps >= liquidation_threshold_bps {
            return Err(LendingError::InvalidCollateralRatio(collateral_factor_bps));
        }

        Ok(Self {
            address,
            collateral_factor_bps,
            liquidation_threshold_bps,
            liquidation_bonus_bps,
            enabled: true,
        })
    }

    /// Calculate maximum borrowable amount for given collateral
    pub fn max_borrow_amount(
        &self,
        collateral_amount: u128,
        collateral_price: u128,
        borrow_price: u128,
    ) -> Result<u128> {
        // Max borrow value = collateral_value * collateral_factor
        let collateral_value = collateral_amount
            .checked_mul(collateral_price)
            .ok_or(LendingError::ArithmeticOverflow)?;

        let max_borrow_value = collateral_value
            .checked_mul(self.collateral_factor_bps as u128)
            .and_then(|v| v.checked_div(10000))
            .ok_or(LendingError::ArithmeticOverflow)?;

        // Convert to borrow asset amount
        let max_borrow_amount = max_borrow_value
            .checked_div(borrow_price)
            .ok_or(LendingError::ArithmeticOverflow)?;

        Ok(max_borrow_amount)
    }
}

/// Manages collateral assets and their configurations
pub struct CollateralManager {
    /// Map of asset address to collateral configuration
    assets: HashMap<Address, CollateralAsset>,
}

impl CollateralManager {
    /// Create a new collateral manager
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
        }
    }

    /// Add a collateral asset
    pub fn add_asset(&mut self, asset: CollateralAsset) -> Result<()> {
        self.assets.insert(asset.address, asset);
        Ok(())
    }

    /// Get collateral asset configuration
    pub fn get_asset(&self, address: &Address) -> Result<&CollateralAsset> {
        self.assets
            .get(address)
            .ok_or_else(|| LendingError::UnsupportedAsset(hex::encode(address.as_bytes())))
    }

    /// Check if asset is supported as collateral
    pub fn is_supported(&self, address: &Address) -> bool {
        self.assets
            .get(address)
            .map(|a| a.enabled)
            .unwrap_or(false)
    }

    /// Enable/disable asset as collateral
    pub fn set_enabled(&mut self, address: &Address, enabled: bool) -> Result<()> {
        let asset = self
            .assets
            .get_mut(address)
            .ok_or_else(|| LendingError::UnsupportedAsset(hex::encode(address.as_bytes())))?;
        asset.enabled = enabled;
        Ok(())
    }

    /// Validate collateral amount for a loan
    pub fn validate_collateral(
        &self,
        collateral_asset: &Address,
        collateral_amount: u128,
        borrow_asset: &Address,
        borrow_amount: u128,
        collateral_price: u128,
        borrow_price: u128,
    ) -> Result<()> {
        let asset = self.get_asset(collateral_asset)?;
        
        if !asset.enabled {
            return Err(LendingError::UnsupportedAsset(
                hex::encode(collateral_asset.as_bytes()),
            ));
        }

        let max_borrow = asset.max_borrow_amount(
            collateral_amount,
            collateral_price,
            borrow_price,
        )?;

        if borrow_amount > max_borrow {
            return Err(LendingError::InsufficientCollateral {
                required: collateral_amount
                    .checked_mul(borrow_amount)
                    .and_then(|v| v.checked_div(max_borrow))
                    .unwrap_or(u128::MAX),
                provided: collateral_amount,
            });
        }

        Ok(())
    }

    /// Calculate liquidation amount
    pub fn calculate_liquidation(
        &self,
        collateral_asset: &Address,
        total_debt: u128,
        collateral_price: u128,
        borrow_price: u128,
    ) -> Result<(u128, u128)> {
        let asset = self.get_asset(collateral_asset)?;

        // Debt value
        let debt_value = total_debt
            .checked_mul(borrow_price)
            .ok_or(LendingError::ArithmeticOverflow)?;

        // Collateral to seize = debt_value / collateral_price * (1 + liquidation_bonus)
        let bonus_multiplier = 10000u128
            .checked_add(asset.liquidation_bonus_bps as u128)
            .ok_or(LendingError::ArithmeticOverflow)?;

        let collateral_to_seize = debt_value
            .checked_mul(bonus_multiplier)
            .and_then(|v| v.checked_div(10000))
            .and_then(|v| v.checked_div(collateral_price))
            .ok_or(LendingError::ArithmeticOverflow)?;

        Ok((total_debt, collateral_to_seize))
    }
}

impl Default for CollateralManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_address(n: u8) -> Address {
        Address::from([n; 20])
    }

    #[test]
    fn test_collateral_asset_creation() {
        let asset = CollateralAsset::new(
            test_address(1),
            7500,  // 75% LTV
            8000,  // 80% liquidation threshold
            500,   // 5% liquidation bonus
        ).unwrap();

        assert_eq!(asset.collateral_factor_bps, 7500);
        assert!(asset.enabled);
    }

    #[test]
    fn test_invalid_collateral_factor() {
        let result = CollateralAsset::new(
            test_address(1),
            12000, // Invalid: > 100%
            8000,
            500,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_max_borrow_amount() {
        let asset = CollateralAsset::new(
            test_address(1),
            7500, // 75%
            8000,
            500,
        ).unwrap();

        // Collateral: 1000 units at price 2 = 2000 value
        // Max borrow value = 2000 * 0.75 = 1500
        // At borrow price 1, max borrow = 1500
        let max_borrow = asset.max_borrow_amount(1000, 2, 1).unwrap();
        assert_eq!(max_borrow, 1500);
    }

    #[test]
    fn test_collateral_manager() {
        let mut manager = CollateralManager::new();
        
        let asset = CollateralAsset::new(
            test_address(1),
            7500,
            8000,
            500,
        ).unwrap();
        
        manager.add_asset(asset).unwrap();
        assert!(manager.is_supported(&test_address(1)));
        assert!(!manager.is_supported(&test_address(2)));
    }

    #[test]
    fn test_validate_collateral() {
        let mut manager = CollateralManager::new();
        
        let asset = CollateralAsset::new(
            test_address(1),
            7500,
            8000,
            500,
        ).unwrap();
        
        manager.add_asset(asset).unwrap();

        // Valid collateral
        assert!(manager.validate_collateral(
            &test_address(1),
            1000,  // collateral amount
            &test_address(2),
            750,   // borrow amount (within 75% LTV)
            1,     // collateral price
            1,     // borrow price
        ).is_ok());

        // Insufficient collateral
        assert!(manager.validate_collateral(
            &test_address(1),
            1000,
            &test_address(2),
            800,  // exceeds 75% LTV
            1,
            1,
        ).is_err());
    }
}
