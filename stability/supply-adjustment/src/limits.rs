use crate::types::{SupplyChange, AdjustmentError, Result};
use chrono::{DateTime, Utc, Duration};
use std::collections::VecDeque;

/// Tracks and enforces adjustment limits
pub struct AdjustmentLimits {
    /// Maximum daily adjustment (in basis points of supply)
    max_daily_bps: u16,
    /// Maximum weekly adjustment (in basis points of supply)
    max_weekly_bps: u16,
    /// Current total supply
    total_supply: u128,
    /// History of recent adjustments
    adjustment_history: VecDeque<SupplyChange>,
}

impl AdjustmentLimits {
    /// Create new adjustment limits
    pub fn new(max_daily_bps: u16, max_weekly_bps: u16, total_supply: u128) -> Self {
        Self {
            max_daily_bps,
            max_weekly_bps,
            total_supply,
            adjustment_history: VecDeque::new(),
        }
    }

    /// Check if an adjustment is within daily limits
    pub fn check_daily_limit(&self, adjustment_amount: u128) -> Result<()> {
        let daily_adjustments = self.get_adjustments_in_period(Duration::days(1));
        let total_daily = self.sum_adjustments(&daily_adjustments);

        let proposed_total = total_daily.saturating_add(adjustment_amount as i128).abs() as u128;
        let max_daily = (self.total_supply * self.max_daily_bps as u128) / 10000;

        if proposed_total > max_daily {
            return Err(AdjustmentError::DailyLimitExceeded {
                requested: proposed_total,
                limit: max_daily,
            });
        }

        Ok(())
    }

    /// Check if an adjustment is within weekly limits
    pub fn check_weekly_limit(&self, adjustment_amount: u128) -> Result<()> {
        let weekly_adjustments = self.get_adjustments_in_period(Duration::weeks(1));
        let total_weekly = self.sum_adjustments(&weekly_adjustments);

        let proposed_total = total_weekly.saturating_add(adjustment_amount as i128).abs() as u128;
        let max_weekly = (self.total_supply * self.max_weekly_bps as u128) / 10000;

        if proposed_total > max_weekly {
            return Err(AdjustmentError::WeeklyLimitExceeded {
                requested: proposed_total,
                limit: max_weekly,
            });
        }

        Ok(())
    }

    /// Check all limits for an adjustment
    pub fn check_limits(&self, adjustment_amount: u128) -> Result<()> {
        self.check_daily_limit(adjustment_amount)?;
        self.check_weekly_limit(adjustment_amount)?;
        Ok(())
    }

    /// Record an adjustment in history
    pub fn record_adjustment(&mut self, change: SupplyChange) {
        self.adjustment_history.push_back(change);

        // Keep only last 30 days of history
        let cutoff = Utc::now() - Duration::days(30);
        while let Some(oldest) = self.adjustment_history.front() {
            if oldest.timestamp < cutoff {
                self.adjustment_history.pop_front();
            } else {
                break;
            }
        }
    }

    /// Update total supply
    pub fn update_supply(&mut self, new_supply: u128) {
        self.total_supply = new_supply;
    }

    /// Get total supply
    pub fn total_supply(&self) -> u128 {
        self.total_supply
    }

    /// Get adjustments within a time period
    fn get_adjustments_in_period(&self, period: Duration) -> Vec<&SupplyChange> {
        let cutoff = Utc::now() - period;
        self.adjustment_history
            .iter()
            .filter(|change| change.timestamp >= cutoff)
            .collect()
    }

    /// Sum adjustments (accounting for expansion vs contraction)
    fn sum_adjustments(&self, adjustments: &[&SupplyChange]) -> i128 {
        adjustments.iter().map(|change| change.signed_amount()).sum()
    }

    /// Get daily adjustment usage (in basis points)
    pub fn daily_usage_bps(&self) -> u16 {
        let daily_adjustments = self.get_adjustments_in_period(Duration::days(1));
        let total = self.sum_adjustments(&daily_adjustments).abs() as u128;

        if self.total_supply == 0 {
            return 0;
        }

        ((total * 10000) / self.total_supply) as u16
    }

    /// Get weekly adjustment usage (in basis points)
    pub fn weekly_usage_bps(&self) -> u16 {
        let weekly_adjustments = self.get_adjustments_in_period(Duration::weeks(1));
        let total = self.sum_adjustments(&weekly_adjustments).abs() as u128;

        if self.total_supply == 0 {
            return 0;
        }

        ((total * 10000) / self.total_supply) as u16
    }

    /// Get remaining daily capacity
    pub fn remaining_daily_capacity(&self) -> u128 {
        let used = self.daily_usage_bps();
        let remaining_bps = self.max_daily_bps.saturating_sub(used);
        (self.total_supply * remaining_bps as u128) / 10000
    }

    /// Get remaining weekly capacity
    pub fn remaining_weekly_capacity(&self) -> u128 {
        let used = self.weekly_usage_bps();
        let remaining_bps = self.max_weekly_bps.saturating_sub(used);
        (self.total_supply * remaining_bps as u128) / 10000
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{AdjustmentType, AdjustmentMechanism};

    #[test]
    fn test_limits_creation() {
        let limits = AdjustmentLimits::new(500, 1500, 1_000_000_000_000);
        assert_eq!(limits.total_supply(), 1_000_000_000_000);
    }

    #[test]
    fn test_daily_limit_check() {
        let limits = AdjustmentLimits::new(500, 1500, 1_000_000_000_000);

        // 5% of 1T = 50B max daily
        // Try to adjust by 30B - should succeed
        assert!(limits.check_daily_limit(30_000_000_000).is_ok());

        // Try to adjust by 60B - should fail
        assert!(limits.check_daily_limit(60_000_000_000).is_err());
    }

    #[test]
    fn test_weekly_limit_check() {
        let limits = AdjustmentLimits::new(500, 1500, 1_000_000_000_000);

        // 15% of 1T = 150B max weekly
        // Try to adjust by 100B - should succeed
        assert!(limits.check_weekly_limit(100_000_000_000).is_ok());

        // Try to adjust by 200B - should fail
        assert!(limits.check_weekly_limit(200_000_000_000).is_err());
    }

    #[test]
    fn test_record_adjustment() {
        let mut limits = AdjustmentLimits::new(500, 1500, 1_000_000_000_000);

        let change = SupplyChange::new(
            AdjustmentType::Expansion,
            AdjustmentMechanism::StakingRewards,
            10_000_000_000,
            500,
        );

        limits.record_adjustment(change);
        assert_eq!(limits.adjustment_history.len(), 1);
    }

    #[test]
    fn test_usage_calculation() {
        let mut limits = AdjustmentLimits::new(500, 1500, 1_000_000_000_000);

        // Record a 2% expansion
        let change = SupplyChange::new(
            AdjustmentType::Expansion,
            AdjustmentMechanism::StakingRewards,
            20_000_000_000, // 2% of 1T
            500,
        );

        limits.record_adjustment(change);

        // Daily usage should be ~200bps (2%)
        let usage = limits.daily_usage_bps();
        assert!(usage >= 190 && usage <= 210);
    }

    #[test]
    fn test_remaining_capacity() {
        let mut limits = AdjustmentLimits::new(500, 1500, 1_000_000_000_000);

        // Initially, should have full capacity
        assert_eq!(limits.remaining_daily_capacity(), 50_000_000_000); // 5% of 1T

        // After using 2%, should have 3% remaining
        let change = SupplyChange::new(
            AdjustmentType::Expansion,
            AdjustmentMechanism::StakingRewards,
            20_000_000_000,
            500,
        );
        limits.record_adjustment(change);

        let remaining = limits.remaining_daily_capacity();
        assert!(remaining >= 25_000_000_000 && remaining <= 35_000_000_000); // ~3%
    }

    #[test]
    fn test_update_supply() {
        let mut limits = AdjustmentLimits::new(500, 1500, 1_000_000_000_000);
        limits.update_supply(2_000_000_000_000);
        assert_eq!(limits.total_supply(), 2_000_000_000_000);
    }

    #[test]
    fn test_expansion_and_contraction() {
        let mut limits = AdjustmentLimits::new(500, 1500, 1_000_000_000_000);

        // Expand by 2%
        limits.record_adjustment(SupplyChange::new(
            AdjustmentType::Expansion,
            AdjustmentMechanism::StakingRewards,
            20_000_000_000,
            500,
        ));

        // Contract by 1%
        limits.record_adjustment(SupplyChange::new(
            AdjustmentType::Contraction,
            AdjustmentMechanism::BuyBack,
            10_000_000_000,
            -250,
        ));

        // Net should be +1%
        let usage = limits.daily_usage_bps();
        assert!(usage >= 90 && usage <= 110); // ~1% net
    }
}
