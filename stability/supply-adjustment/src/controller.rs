use crate::types::{AdjustmentConfig, SupplyChange, AdjustmentType, AdjustmentError, Result};
use crate::expansion::ExpansionMechanism;
use crate::contraction::ContractionMechanism;
use crate::limits::AdjustmentLimits;

/// Main controller for supply adjustments
pub struct SupplyController {
    config: AdjustmentConfig,
    expansion: ExpansionMechanism,
    contraction: ContractionMechanism,
    limits: AdjustmentLimits,
}

impl SupplyController {
    /// Create a new supply controller
    pub fn new(config: AdjustmentConfig, treasury_balance: u128, total_supply: u128) -> Self {
        Self {
            expansion: ExpansionMechanism::new(),
            contraction: ContractionMechanism::new(treasury_balance),
            limits: AdjustmentLimits::new(
                config.max_daily_adjustment_bps,
                config.max_weekly_adjustment_bps,
                total_supply,
            ),
            config,
        }
    }

    /// Calculate required supply adjustment based on price deviation
    pub fn calculate_adjustment(
        &self,
        current_price: u128,
        target_price: u128,
        current_supply: u128,
    ) -> Result<Vec<SupplyChange>> {
        // Calculate price deviation
        let deviation_bps = self.calculate_deviation_bps(current_price, target_price)?;

        // Check if deviation is significant enough
        if deviation_bps.abs() < self.config.min_deviation_bps as i32 {
            return Err(AdjustmentError::DeviationTooSmall {
                deviation_bps,
                min_bps: self.config.min_deviation_bps,
            });
        }

        // Calculate base adjustment amount
        let base_adjustment = self.calculate_base_adjustment(current_supply, deviation_bps)?;

        // Apply sensitivity factor
        let adjusted_amount = (base_adjustment * self.config.sensitivity_bps as u128) / 10000;

        // Check limits
        self.limits.check_limits(adjusted_amount)?;

        // Determine adjustment type and distribute across mechanisms
        if deviation_bps > 0 {
            // Price above target -> expand supply
            self.calculate_expansion(adjusted_amount, current_supply)
        } else {
            // Price below target -> contract supply
            self.calculate_contraction(adjusted_amount, current_price, current_supply)
        }
    }

    /// Record an executed adjustment
    pub fn record_adjustment(&mut self, change: SupplyChange) {
        // Update supply based on adjustment
        let new_supply = if change.is_expansion() {
            self.limits.total_supply().saturating_add(change.amount)
        } else {
            self.limits.total_supply().saturating_sub(change.amount)
        };

        self.limits.update_supply(new_supply);
        self.limits.record_adjustment(change);
    }

    /// Get current adjustment limits status
    pub fn get_limits_status(&self) -> LimitsStatus {
        LimitsStatus {
            daily_usage_bps: self.limits.daily_usage_bps(),
            daily_limit_bps: self.config.max_daily_adjustment_bps,
            weekly_usage_bps: self.limits.weekly_usage_bps(),
            weekly_limit_bps: self.config.max_weekly_adjustment_bps,
            remaining_daily: self.limits.remaining_daily_capacity(),
            remaining_weekly: self.limits.remaining_weekly_capacity(),
        }
    }

    /// Update treasury balance
    pub fn update_treasury(&mut self, new_balance: u128) {
        self.contraction.update_treasury_balance(new_balance);
    }

    /// Get current supply
    pub fn total_supply(&self) -> u128 {
        self.limits.total_supply()
    }

    // Private helper methods

    fn calculate_deviation_bps(&self, current: u128, target: u128) -> Result<i32> {
        if target == 0 {
            return Err(AdjustmentError::InvalidAmount("Target price is zero".to_string()));
        }

        let deviation = if current > target {
            let diff = current - target;
            ((diff as i128 * 10000) / target as i128) as i32
        } else {
            let diff = target - current;
            -(((diff as i128 * 10000) / target as i128) as i32)
        };

        Ok(deviation)
    }

    fn calculate_base_adjustment(&self, supply: u128, deviation_bps: i32) -> Result<u128> {
        // Base adjustment is proportional to deviation
        // If price is 10% off, adjust supply by ~10%
        let adjustment_ratio = deviation_bps.abs() as u128;
        let adjustment = (supply * adjustment_ratio) / 10000;

        Ok(adjustment)
    }

    fn calculate_expansion(&self, amount: u128, total_supply: u128) -> Result<Vec<SupplyChange>> {
        // Assume 60% of supply is staked, 20% is in liquidity
        let total_staked = (total_supply * 60) / 100;
        let total_liquidity = (total_supply * 20) / 100;

        self.expansion.distribute_expansion(amount, total_staked, total_liquidity)
    }

    fn calculate_contraction(
        &self,
        amount: u128,
        current_price: u128,
        total_supply: u128,
    ) -> Result<Vec<SupplyChange>> {
        // Assume current emission is 10% annually
        let annual_emission = total_supply / 10;

        // Assume daily fees are 0.1% of supply
        let daily_fees = total_supply / 1000;

        self.contraction.distribute_contraction(
            amount,
            current_price,
            annual_emission,
            daily_fees,
        )
    }
}

/// Status of adjustment limits
#[derive(Debug, Clone)]
pub struct LimitsStatus {
    pub daily_usage_bps: u16,
    pub daily_limit_bps: u16,
    pub weekly_usage_bps: u16,
    pub weekly_limit_bps: u16,
    pub remaining_daily: u128,
    pub remaining_weekly: u128,
}

impl LimitsStatus {
    /// Check if daily limit is close to being reached (>80%)
    pub fn is_daily_limit_close(&self) -> bool {
        self.daily_usage_bps > (self.daily_limit_bps * 80) / 100
    }

    /// Check if weekly limit is close to being reached (>80%)
    pub fn is_weekly_limit_close(&self) -> bool {
        self.weekly_usage_bps > (self.weekly_limit_bps * 80) / 100
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_controller_creation() {
        let config = AdjustmentConfig::default();
        let controller = SupplyController::new(config, 1_000_000_000_000, 1_000_000_000_000);
        assert_eq!(controller.total_supply(), 1_000_000_000_000);
    }

    #[test]
    fn test_calculate_deviation() {
        let config = AdjustmentConfig::default();
        let controller = SupplyController::new(config, 1_000_000_000_000, 1_000_000_000_000);

        // Price 10% above target
        let deviation = controller.calculate_deviation_bps(110_000_000, 100_000_000).unwrap();
        assert_eq!(deviation, 1000); // 10%

        // Price 10% below target
        let deviation = controller.calculate_deviation_bps(90_000_000, 100_000_000).unwrap();
        assert_eq!(deviation, -1000); // -10%
    }

    #[test]
    fn test_calculate_expansion_adjustment() {
        let config = AdjustmentConfig::default();
        let controller = SupplyController::new(config, 1_000_000_000_000, 1_000_000_000_000);

        let current_price = 110_000_000; // 10% above target
        let target_price = 100_000_000;
        let current_supply = 1_000_000_000_000;

        let changes = controller.calculate_adjustment(current_price, target_price, current_supply).unwrap();

        // Should have expansion changes
        assert!(!changes.is_empty());
        assert!(changes.iter().all(|c| c.adjustment_type == AdjustmentType::Expansion));
    }

    #[test]
    fn test_calculate_contraction_adjustment() {
        let config = AdjustmentConfig::default();
        let controller = SupplyController::new(config, 1_000_000_000_000, 1_000_000_000_000);

        let current_price = 90_000_000; // 10% below target
        let target_price = 100_000_000;
        let current_supply = 1_000_000_000_000;

        let changes = controller.calculate_adjustment(current_price, target_price, current_supply).unwrap();

        // Should have contraction changes
        assert!(!changes.is_empty());
        assert!(changes.iter().all(|c| c.adjustment_type == AdjustmentType::Contraction));
    }

    #[test]
    fn test_deviation_too_small() {
        let config = AdjustmentConfig::default();
        let controller = SupplyController::new(config, 1_000_000_000_000, 1_000_000_000_000);

        let current_price = 102_000_000; // Only 2% above (below 5% threshold)
        let target_price = 100_000_000;
        let current_supply = 1_000_000_000_000;

        let result = controller.calculate_adjustment(current_price, target_price, current_supply);
        assert!(result.is_err());

        match result {
            Err(AdjustmentError::DeviationTooSmall { .. }) => {},
            _ => panic!("Expected DeviationTooSmall error"),
        }
    }

    #[test]
    fn test_record_adjustment_expansion() {
        let config = AdjustmentConfig::default();
        let mut controller = SupplyController::new(config, 1_000_000_000_000, 1_000_000_000_000);

        let change = SupplyChange::new(
            AdjustmentType::Expansion,
            crate::types::AdjustmentMechanism::StakingRewards,
            50_000_000_000,
            1000,
        );

        let initial_supply = controller.total_supply();
        controller.record_adjustment(change.clone());

        // Supply should have increased
        assert_eq!(controller.total_supply(), initial_supply + change.amount);
    }

    #[test]
    fn test_record_adjustment_contraction() {
        let config = AdjustmentConfig::default();
        let mut controller = SupplyController::new(config, 1_000_000_000_000, 1_000_000_000_000);

        let change = SupplyChange::new(
            AdjustmentType::Contraction,
            crate::types::AdjustmentMechanism::BuyBack,
            30_000_000_000,
            -800,
        );

        let initial_supply = controller.total_supply();
        controller.record_adjustment(change.clone());

        // Supply should have decreased
        assert_eq!(controller.total_supply(), initial_supply - change.amount);
    }

    #[test]
    fn test_limits_status() {
        let config = AdjustmentConfig::default();
        let controller = SupplyController::new(config, 1_000_000_000_000, 1_000_000_000_000);

        let status = controller.get_limits_status();
        assert_eq!(status.daily_usage_bps, 0); // No adjustments yet
        assert_eq!(status.daily_limit_bps, 500); // 5%
        assert!(!status.is_daily_limit_close());
    }

    #[test]
    fn test_update_treasury() {
        let config = AdjustmentConfig::default();
        let mut controller = SupplyController::new(config, 1_000_000_000_000, 1_000_000_000_000);

        controller.update_treasury(2_000_000_000_000);
        assert_eq!(controller.contraction.treasury_balance(), 2_000_000_000_000);
    }

    #[test]
    fn test_sensitivity_factor() {
        let mut config = AdjustmentConfig::default();
        config.sensitivity_bps = 2500; // 25% sensitivity (less aggressive)

        let controller = SupplyController::new(config, 1_000_000_000_000, 1_000_000_000_000);

        let current_price = 120_000_000; // 20% above target
        let target_price = 100_000_000;
        let current_supply = 1_000_000_000_000;

        let changes = controller.calculate_adjustment(current_price, target_price, current_supply).unwrap();

        // Total adjustment should be less due to lower sensitivity
        let total_adjustment: u128 = changes.iter().map(|c| c.amount).sum();

        // Base would be 20% of supply = 200B, but with 25% sensitivity = 50B
        assert!(total_adjustment < 100_000_000_000);
    }
}
