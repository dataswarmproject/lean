//! Interest rate models

use crate::Result;
use serde::{Deserialize, Serialize};

/// Interest rate model trait
pub trait InterestRateModel {
    /// Calculate borrow rate per second in basis points
    fn borrow_rate_per_second(&self, utilization_rate: u128) -> Result<u64>;
    
    /// Calculate supply rate per second in basis points
    fn supply_rate_per_second(&self, utilization_rate: u128, reserve_factor: u16) -> Result<u64>;
}

/// Linear interest rate model
/// Rate increases linearly with utilization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearInterestModel {
    /// Base rate per year (in basis points)
    pub base_rate_per_year: u64,
    /// Rate multiplier (slope) per year (in basis points)
    pub multiplier_per_year: u64,
    /// Kink point (utilization at which rate jumps) in basis points
    pub kink: u16,
    /// Jump multiplier after kink (in basis points)
    pub jump_multiplier_per_year: u64,
}

impl LinearInterestModel {
    /// Create a new linear interest rate model
    pub fn new(
        base_rate_per_year: u64,
        multiplier_per_year: u64,
        kink: u16,
        jump_multiplier_per_year: u64,
    ) -> Self {
        Self {
            base_rate_per_year,
            multiplier_per_year,
            kink,
            jump_multiplier_per_year,
        }
    }

    /// Create a default model (typical DeFi parameters)
    pub fn default_model() -> Self {
        Self::new(
            200,    // 2% base APR
            1000,   // 10% multiplier
            8000,   // 80% kink
            10000,  // 100% jump multiplier
        )
    }

    /// Calculate utilization rate (borrowed / supplied) in basis points
    pub fn utilization_rate(borrowed: u128, supplied: u128) -> u128 {
        if supplied == 0 {
            return 0;
        }
        borrowed
            .checked_mul(10000)
            .and_then(|v| v.checked_div(supplied))
            .unwrap_or(0)
    }

    /// Convert annual rate to per-second rate
    fn annual_to_per_second(annual_rate: u64) -> u64 {
        // Seconds per year: 365.25 * 24 * 60 * 60 = 31_557_600
        const SECONDS_PER_YEAR: u64 = 31_557_600;
        annual_rate / SECONDS_PER_YEAR
    }
}

impl InterestRateModel for LinearInterestModel {
    fn borrow_rate_per_second(&self, utilization_rate: u128) -> Result<u64> {
        let kink_utilization = self.kink as u128;
        
        let annual_rate = if utilization_rate <= kink_utilization {
            // Below kink: base_rate + (utilization * multiplier)
            let slope_rate = utilization_rate
                .checked_mul(self.multiplier_per_year as u128)
                .and_then(|v| v.checked_div(10000))
                .unwrap_or(0) as u64;
            
            self.base_rate_per_year + slope_rate
        } else {
            // Above kink: base_rate + (kink * multiplier) + ((utilization - kink) * jump_multiplier)
            let normal_rate = kink_utilization
                .checked_mul(self.multiplier_per_year as u128)
                .and_then(|v| v.checked_div(10000))
                .unwrap_or(0) as u64;
            
            let excess_utilization = utilization_rate - kink_utilization;
            let jump_rate = excess_utilization
                .checked_mul(self.jump_multiplier_per_year as u128)
                .and_then(|v| v.checked_div(10000))
                .unwrap_or(0) as u64;
            
            self.base_rate_per_year + normal_rate + jump_rate
        };

        Ok(Self::annual_to_per_second(annual_rate))
    }

    fn supply_rate_per_second(&self, utilization_rate: u128, reserve_factor: u16) -> Result<u64> {
        let borrow_rate = self.borrow_rate_per_second(utilization_rate)?;
        
        // Supply rate = borrow_rate * utilization * (1 - reserve_factor)
        let one_minus_reserve = 10000u128
            .checked_sub(reserve_factor as u128)
            .unwrap_or(10000);
        
        let supply_rate = (borrow_rate as u128)
            .checked_mul(utilization_rate)
            .and_then(|v| v.checked_div(10000))
            .and_then(|v| v.checked_mul(one_minus_reserve))
            .and_then(|v| v.checked_div(10000))
            .unwrap_or(0) as u64;

        Ok(supply_rate)
    }
}

/// Jump rate model (more aggressive after kink)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpRateModel {
    pub base_model: LinearInterestModel,
}

impl JumpRateModel {
    /// Create a new jump rate model with aggressive parameters
    pub fn new() -> Self {
        Self {
            base_model: LinearInterestModel::new(
                0,      // 0% base
                500,    // 5% multiplier
                8000,   // 80% kink
                20000,  // 200% jump multiplier (very steep)
            ),
        }
    }
}

impl Default for JumpRateModel {
    fn default() -> Self {
        Self::new()
    }
}

impl InterestRateModel for JumpRateModel {
    fn borrow_rate_per_second(&self, utilization_rate: u128) -> Result<u64> {
        self.base_model.borrow_rate_per_second(utilization_rate)
    }

    fn supply_rate_per_second(&self, utilization_rate: u128, reserve_factor: u16) -> Result<u64> {
        self.base_model.supply_rate_per_second(utilization_rate, reserve_factor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utilization_rate() {
        let rate = LinearInterestModel::utilization_rate(50, 100);
        assert_eq!(rate, 5000); // 50%
    }

    #[test]
    fn test_borrow_rate_below_kink() {
        let model = LinearInterestModel::default_model();
        // At 50% utilization (below 80% kink)
        let rate = model.borrow_rate_per_second(5000).unwrap();
        // base (200) + (5000 * 1000 / 10000) = 200 + 500 = 700 per year
        // per second = 700 / 31557600
        assert!(rate > 0);
    }

    #[test]
    fn test_borrow_rate_above_kink() {
        let model = LinearInterestModel::default_model();
        // At 90% utilization (above 80% kink)
        let rate = model.borrow_rate_per_second(9000).unwrap();
        // Should be higher than below kink
        let below_kink = model.borrow_rate_per_second(5000).unwrap();
        assert!(rate > below_kink);
    }

    #[test]
    fn test_supply_rate() {
        let model = LinearInterestModel::default_model();
        let utilization = 5000; // 50%
        let reserve_factor = 1000; // 10%
        
        let supply_rate = model.supply_rate_per_second(utilization, reserve_factor).unwrap();
        let borrow_rate = model.borrow_rate_per_second(utilization).unwrap();
        
        // Supply rate should be less than borrow rate
        assert!(supply_rate < borrow_rate);
    }
}
