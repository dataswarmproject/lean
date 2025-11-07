use crate::types::{AdjustmentMechanism, SupplyChange, AdjustmentType, AdjustmentError, Result};

/// Manages supply contraction mechanisms
pub struct ContractionMechanism {
    /// Current fee burn rate (in basis points)
    fee_burn_rate_bps: u16,
    /// Treasury available for buy-backs
    treasury_balance: u128,
    /// Maximum treasury spend per operation (in basis points of balance)
    max_treasury_spend_bps: u16,
}

impl ContractionMechanism {
    /// Create a new contraction mechanism
    pub fn new(treasury_balance: u128) -> Self {
        Self {
            fee_burn_rate_bps: 5000,      // 50% of fees burned
            treasury_balance,
            max_treasury_spend_bps: 100,  // 1% per operation
        }
    }

    /// Calculate contraction via treasury buy-back and burn
    pub fn calculate_buyback_contraction(
        &self,
        target_contraction: u128,
        current_price: u128,
    ) -> Result<SupplyChange> {
        if current_price == 0 {
            return Err(AdjustmentError::InvalidAmount("Price cannot be zero".to_string()));
        }

        // Calculate maximum spend from treasury
        let max_spend = (self.treasury_balance * self.max_treasury_spend_bps as u128) / 10000;

        // Calculate how many tokens can be bought back
        let tokens_buyable = max_spend / current_price;

        // Use the minimum of target and what's buyable
        let actual_contraction = target_contraction.min(tokens_buyable);

        if actual_contraction == 0 {
            return Err(AdjustmentError::InsufficientSupply {
                current: tokens_buyable,
                requested: target_contraction,
            });
        }

        Ok(SupplyChange::new(
            AdjustmentType::Contraction,
            AdjustmentMechanism::BuyBack,
            actual_contraction,
            0,
        ))
    }

    /// Calculate contraction via reduced emissions
    pub fn calculate_emission_reduction(
        &self,
        current_emission_rate: u128,
        target_contraction: u128,
    ) -> Result<SupplyChange> {
        if current_emission_rate == 0 {
            return Err(AdjustmentError::InvalidAmount("No emissions to reduce".to_string()));
        }

        // Calculate reduction as percentage of current emissions
        let reduction = target_contraction.min(current_emission_rate);

        Ok(SupplyChange::new(
            AdjustmentType::Contraction,
            AdjustmentMechanism::EmissionReduction,
            reduction,
            0,
        ))
    }

    /// Calculate contraction via fee burning
    pub fn calculate_fee_burn_contraction(
        &self,
        estimated_daily_fees: u128,
        days: u32,
    ) -> Result<SupplyChange> {
        if estimated_daily_fees == 0 {
            return Err(AdjustmentError::InvalidAmount("No fees to burn".to_string()));
        }

        // Calculate total fees that would be burned
        let total_fees = estimated_daily_fees
            .checked_mul(days as u128)
            .ok_or(AdjustmentError::CalculationOverflow)?;

        let burn_amount = (total_fees * self.fee_burn_rate_bps as u128) / 10000;

        Ok(SupplyChange::new(
            AdjustmentType::Contraction,
            AdjustmentMechanism::FeeBurn,
            burn_amount,
            0,
        ))
    }

    /// Distribute contraction across multiple mechanisms
    pub fn distribute_contraction(
        &self,
        total_contraction: u128,
        current_price: u128,
        current_emission_rate: u128,
        daily_fees: u128,
    ) -> Result<Vec<SupplyChange>> {
        let mut changes = Vec::new();
        let mut remaining = total_contraction;

        // Priority 1: Emission reduction (60%)
        let emission_portion = (total_contraction * 60) / 100;
        if current_emission_rate > 0 && emission_portion > 0 {
            if let Ok(change) = self.calculate_emission_reduction(current_emission_rate, emission_portion) {
                remaining = remaining.saturating_sub(change.amount);
                changes.push(change);
            }
        }

        // Priority 2: Treasury buy-back (30%)
        let buyback_portion = ((total_contraction * 30) / 100).min(remaining);
        if current_price > 0 && buyback_portion > 0 {
            if let Ok(change) = self.calculate_buyback_contraction(buyback_portion, current_price) {
                remaining = remaining.saturating_sub(change.amount);
                changes.push(change);
            }
        }

        // Priority 3: Fee burning (10%)
        let fee_portion = ((total_contraction * 10) / 100).min(remaining);
        if daily_fees > 0 && fee_portion > 0 {
            // Calculate days needed to achieve fee portion
            let daily_burn = (daily_fees * self.fee_burn_rate_bps as u128) / 10000;
            if daily_burn > 0 {
                let days_needed = (fee_portion / daily_burn).max(1) as u32;
                if let Ok(change) = self.calculate_fee_burn_contraction(daily_fees, days_needed) {
                    changes.push(change);
                }
            }
        }

        if changes.is_empty() {
            return Err(AdjustmentError::NoAdjustmentNeeded);
        }

        Ok(changes)
    }

    /// Update treasury balance after operations
    pub fn update_treasury_balance(&mut self, new_balance: u128) {
        self.treasury_balance = new_balance;
    }

    /// Get current treasury balance
    pub fn treasury_balance(&self) -> u128 {
        self.treasury_balance
    }

    /// Get current fee burn rate
    pub fn fee_burn_rate(&self) -> u16 {
        self.fee_burn_rate_bps
    }

    /// Set fee burn rate
    pub fn set_fee_burn_rate(&mut self, rate_bps: u16) -> Result<()> {
        if rate_bps > 10000 {
            return Err(AdjustmentError::InvalidAmount("Fee burn rate cannot exceed 100%".to_string()));
        }
        self.fee_burn_rate_bps = rate_bps;
        Ok(())
    }

    /// Calculate treasury cost for a buy-back
    pub fn calculate_buyback_cost(&self, amount: u128, price: u128) -> Result<u128> {
        amount
            .checked_mul(price)
            .ok_or(AdjustmentError::CalculationOverflow)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contraction_mechanism_creation() {
        let mechanism = ContractionMechanism::new(1_000_000_000_000);
        assert_eq!(mechanism.treasury_balance(), 1_000_000_000_000);
        assert_eq!(mechanism.fee_burn_rate(), 5000);
    }

    #[test]
    fn test_buyback_contraction() {
        let mechanism = ContractionMechanism::new(1_000_000_000_000); // 1T treasury
        let current_price = 1_000_000; // $1
        let target = 50_000_000_000;   // 50B tokens

        let change = mechanism.calculate_buyback_contraction(target, current_price).unwrap();
        assert_eq!(change.adjustment_type, AdjustmentType::Contraction);
        assert_eq!(change.mechanism, AdjustmentMechanism::BuyBack);
    }

    #[test]
    fn test_emission_reduction() {
        let mechanism = ContractionMechanism::new(1_000_000_000_000);
        let current_emission = 100_000_000_000; // 100B per year
        let target = 30_000_000_000;            // Reduce by 30B

        let change = mechanism.calculate_emission_reduction(current_emission, target).unwrap();
        assert_eq!(change.amount, target);
        assert_eq!(change.mechanism, AdjustmentMechanism::EmissionReduction);
    }

    #[test]
    fn test_fee_burn_contraction() {
        let mechanism = ContractionMechanism::new(1_000_000_000_000);
        let daily_fees = 1_000_000_000; // 1B per day
        let days = 30;

        let change = mechanism.calculate_fee_burn_contraction(daily_fees, days).unwrap();

        // 50% of 30B fees = 15B burned
        assert_eq!(change.amount, 15_000_000_000);
        assert_eq!(change.mechanism, AdjustmentMechanism::FeeBurn);
    }

    #[test]
    fn test_distribute_contraction() {
        let mechanism = ContractionMechanism::new(1_000_000_000_000);
        let total_contraction = 100_000_000_000; // 100B tokens
        let current_price = 1_000_000;           // $1
        let emission_rate = 200_000_000_000;     // 200B per year
        let daily_fees = 1_000_000_000;          // 1B per day

        let changes = mechanism.distribute_contraction(
            total_contraction,
            current_price,
            emission_rate,
            daily_fees,
        ).unwrap();

        // Should have multiple mechanisms
        assert!(changes.len() >= 2);

        // Check that emission reduction is present (highest priority)
        assert!(changes.iter().any(|c| c.mechanism == AdjustmentMechanism::EmissionReduction));
    }

    #[test]
    fn test_update_treasury_balance() {
        let mut mechanism = ContractionMechanism::new(1_000_000_000_000);
        mechanism.update_treasury_balance(2_000_000_000_000);
        assert_eq!(mechanism.treasury_balance(), 2_000_000_000_000);
    }

    #[test]
    fn test_set_fee_burn_rate() {
        let mut mechanism = ContractionMechanism::new(1_000_000_000_000);

        assert!(mechanism.set_fee_burn_rate(7500).is_ok()); // 75%
        assert_eq!(mechanism.fee_burn_rate(), 7500);

        // Cannot exceed 100%
        assert!(mechanism.set_fee_burn_rate(15000).is_err());
    }

    #[test]
    fn test_calculate_buyback_cost() {
        let mechanism = ContractionMechanism::new(1_000_000_000_000);
        let amount = 1_000_000_000; // 1B tokens
        let price = 1_500_000;      // $1.50

        let cost = mechanism.calculate_buyback_cost(amount, price).unwrap();
        assert_eq!(cost, 1_500_000_000_000); // $1.5B
    }

    #[test]
    fn test_insufficient_treasury() {
        let mechanism = ContractionMechanism::new(100_000); // Very small treasury
        let current_price = 1_000_000;
        let target = 1_000_000_000_000; // Huge target

        let result = mechanism.calculate_buyback_contraction(target, current_price);
        // Should either succeed with reduced amount or fail
        if let Ok(change) = result {
            assert!(change.amount < target);
        }
    }

    #[test]
    fn test_zero_emission_error() {
        let mechanism = ContractionMechanism::new(1_000_000_000_000);
        let result = mechanism.calculate_emission_reduction(0, 1_000_000);
        assert!(result.is_err());
    }

    #[test]
    fn test_zero_fees_error() {
        let mechanism = ContractionMechanism::new(1_000_000_000_000);
        let result = mechanism.calculate_fee_burn_contraction(0, 30);
        assert!(result.is_err());
    }
}
