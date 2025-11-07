use crate::types::{Asset, AssetWeight, IndexComposition, BalancingError, Result};
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;

/// Strategy for rebalancing the basket
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RebalancingStrategy {
    /// Rebalance to exact target weights
    Exact,
    /// Rebalance gradually to minimize market impact
    Gradual { adjustment_rate_bps: u16 },
    /// Threshold-based: only rebalance if deviation exceeds threshold
    Threshold { threshold_bps: u16 },
    /// Time-based: rebalance at fixed intervals
    Periodic { interval_hours: u64 },
}

impl Default for RebalancingStrategy {
    fn default() -> Self {
        RebalancingStrategy::Threshold { threshold_bps: 500 } // 5% threshold
    }
}

/// Configuration for the rebalancing engine
#[derive(Debug, Clone)]
pub struct RebalancingConfig {
    /// Minimum time between rebalancing operations (hours)
    pub min_rebalancing_interval_hours: i64,
    /// Maximum single rebalancing adjustment (in basis points)
    pub max_adjustment_bps: u16,
    /// Whether to allow rebalancing outside market hours
    pub allow_outside_market_hours: bool,
}

impl Default for RebalancingConfig {
    fn default() -> Self {
        Self {
            min_rebalancing_interval_hours: 168, // 1 week
            max_adjustment_bps: 1000,             // 10% max adjustment
            allow_outside_market_hours: true,
        }
    }
}

/// Manages rebalancing operations for the asset basket
pub struct RebalancingEngine {
    config: RebalancingConfig,
    strategy: RebalancingStrategy,
    last_rebalancing: Option<DateTime<Utc>>,
}

impl RebalancingEngine {
    /// Create a new rebalancing engine
    pub fn new(config: RebalancingConfig, strategy: RebalancingStrategy) -> Self {
        Self {
            config,
            strategy,
            last_rebalancing: None,
        }
    }

    /// Create with default configuration
    pub fn with_strategy(strategy: RebalancingStrategy) -> Self {
        Self::new(RebalancingConfig::default(), strategy)
    }

    /// Check if rebalancing is allowed at the current time
    pub fn can_rebalance(&self) -> Result<()> {
        if let Some(last) = self.last_rebalancing {
            let elapsed = Utc::now().signed_duration_since(last);
            let elapsed_hours = elapsed.num_hours();

            if elapsed_hours < self.config.min_rebalancing_interval_hours {
                return Err(BalancingError::RebalancingFrequency {
                    last_rebalance_hours: elapsed_hours,
                    min_hours: self.config.min_rebalancing_interval_hours,
                });
            }
        }

        Ok(())
    }

    /// Calculate rebalancing actions needed
    pub fn calculate_rebalancing(
        &self,
        current_composition: &IndexComposition,
        current_values: &HashMap<Asset, u128>,
    ) -> Result<Vec<RebalancingAction>> {
        // Calculate total value
        let total_value: u128 = current_values.values().sum();
        if total_value == 0 {
            return Err(BalancingError::CalculationError("Total value is zero".to_string()));
        }

        let mut actions = Vec::new();

        for target_weight in &current_composition.weights {
            let current_value = current_values
                .get(&target_weight.asset)
                .copied()
                .unwrap_or(0);

            // Calculate current weight (in basis points)
            let current_weight_bps = ((current_value * 10000) / total_value) as u16;

            // Calculate target value based on target weight
            let target_value = (total_value * target_weight.weight_bps as u128) / 10000;

            // Determine if we need to buy or sell
            let (action_type, amount) = if current_value < target_value {
                (ActionType::Buy, target_value - current_value)
            } else if current_value > target_value {
                (ActionType::Sell, current_value - target_value)
            } else {
                continue; // No action needed
            };

            // Apply strategy-specific adjustments
            let adjusted_amount = self.apply_strategy_adjustment(
                amount,
                total_value,
                current_weight_bps,
                target_weight.weight_bps,
            )?;

            if adjusted_amount > 0 {
                actions.push(RebalancingAction {
                    asset: target_weight.asset.clone(),
                    action_type,
                    amount: adjusted_amount,
                    current_weight_bps,
                    target_weight_bps: target_weight.weight_bps,
                });
            }
        }

        Ok(actions)
    }

    /// Execute rebalancing (marks the rebalancing as done)
    pub fn mark_rebalancing_complete(&mut self) {
        self.last_rebalancing = Some(Utc::now());
    }

    /// Get time since last rebalancing
    pub fn time_since_last_rebalancing(&self) -> Option<Duration> {
        self.last_rebalancing.map(|last| Utc::now().signed_duration_since(last))
    }

    // Private helper methods

    fn apply_strategy_adjustment(
        &self,
        amount: u128,
        total_value: u128,
        current_weight_bps: u16,
        target_weight_bps: u16,
    ) -> Result<u128> {
        match self.strategy {
            RebalancingStrategy::Exact => Ok(amount),

            RebalancingStrategy::Gradual { adjustment_rate_bps } => {
                // Only adjust by a percentage of the needed amount
                let adjusted = (amount * adjustment_rate_bps as u128) / 10000;
                Ok(adjusted)
            }

            RebalancingStrategy::Threshold { threshold_bps } => {
                // Only rebalance if deviation exceeds threshold
                let deviation = if current_weight_bps > target_weight_bps {
                    current_weight_bps - target_weight_bps
                } else {
                    target_weight_bps - current_weight_bps
                };

                if deviation >= threshold_bps {
                    Ok(amount)
                } else {
                    Ok(0) // Below threshold, no action
                }
            }

            RebalancingStrategy::Periodic { .. } => {
                // For periodic strategy, rebalance fully when triggered
                Ok(amount)
            }
        }
    }

    /// Check if periodic rebalancing is due
    pub fn is_periodic_rebalancing_due(&self) -> bool {
        if let RebalancingStrategy::Periodic { interval_hours } = self.strategy {
            if let Some(last) = self.last_rebalancing {
                let elapsed = Utc::now().signed_duration_since(last);
                return elapsed.num_hours() >= interval_hours as i64;
            }
            return true; // Never rebalanced, so it's due
        }
        false
    }
}

impl Default for RebalancingEngine {
    fn default() -> Self {
        Self::with_strategy(RebalancingStrategy::default())
    }
}

/// Type of rebalancing action
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActionType {
    /// Buy the asset
    Buy,
    /// Sell the asset
    Sell,
}

/// Represents a single rebalancing action
#[derive(Debug, Clone)]
pub struct RebalancingAction {
    /// The asset to rebalance
    pub asset: Asset,
    /// Type of action (buy or sell)
    pub action_type: ActionType,
    /// Amount to buy or sell (in smallest unit)
    pub amount: u128,
    /// Current weight (in basis points)
    pub current_weight_bps: u16,
    /// Target weight (in basis points)
    pub target_weight_bps: u16,
}

impl RebalancingAction {
    /// Get the deviation in basis points
    pub fn deviation_bps(&self) -> u16 {
        if self.current_weight_bps > self.target_weight_bps {
            self.current_weight_bps - self.target_weight_bps
        } else {
            self.target_weight_bps - self.current_weight_bps
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rebalancing_engine_creation() {
        let engine = RebalancingEngine::default();
        assert!(engine.last_rebalancing.is_none());
    }

    #[test]
    fn test_can_rebalance_initially() {
        let engine = RebalancingEngine::default();
        assert!(engine.can_rebalance().is_ok());
    }

    #[test]
    fn test_rebalancing_frequency_limit() {
        let mut engine = RebalancingEngine::default();
        engine.mark_rebalancing_complete();

        // Immediately after rebalancing, should not be allowed
        assert!(engine.can_rebalance().is_err());
    }

    #[test]
    fn test_calculate_exact_rebalancing() {
        let engine = RebalancingEngine::with_strategy(RebalancingStrategy::Exact);

        let composition = IndexComposition::new(vec![
            AssetWeight::new(Asset::USD, 5000), // 50%
            AssetWeight::new(Asset::BTC, 5000), // 50%
        ]).unwrap();

        let mut current_values = HashMap::new();
        current_values.insert(Asset::USD, 6000); // 60% (overweight)
        current_values.insert(Asset::BTC, 4000); // 40% (underweight)

        let actions = engine.calculate_rebalancing(&composition, &current_values).unwrap();

        assert_eq!(actions.len(), 2);

        // USD should be sold (overweight)
        let usd_action = actions.iter().find(|a| a.asset == Asset::USD).unwrap();
        assert_eq!(usd_action.action_type, ActionType::Sell);

        // BTC should be bought (underweight)
        let btc_action = actions.iter().find(|a| a.asset == Asset::BTC).unwrap();
        assert_eq!(btc_action.action_type, ActionType::Buy);
    }

    #[test]
    fn test_calculate_threshold_rebalancing() {
        let engine = RebalancingEngine::with_strategy(
            RebalancingStrategy::Threshold { threshold_bps: 1000 } // 10% threshold
        );

        let composition = IndexComposition::new(vec![
            AssetWeight::new(Asset::USD, 5000), // 50%
            AssetWeight::new(Asset::BTC, 5000), // 50%
        ]).unwrap();

        // Small deviation (below threshold)
        let mut small_deviation = HashMap::new();
        small_deviation.insert(Asset::USD, 5200); // 52% (2% deviation)
        small_deviation.insert(Asset::BTC, 4800); // 48% (2% deviation)

        let actions = engine.calculate_rebalancing(&composition, &small_deviation).unwrap();
        // Should have no actions since deviation is below 10% threshold
        assert!(actions.iter().all(|a| a.amount == 0));

        // Large deviation (above threshold)
        let mut large_deviation = HashMap::new();
        large_deviation.insert(Asset::USD, 7000); // 70% (20% deviation)
        large_deviation.insert(Asset::BTC, 3000); // 30% (20% deviation)

        let actions = engine.calculate_rebalancing(&composition, &large_deviation).unwrap();
        // Should have actions since deviation exceeds 10% threshold
        assert!(actions.iter().any(|a| a.amount > 0));
    }

    #[test]
    fn test_calculate_gradual_rebalancing() {
        let engine = RebalancingEngine::with_strategy(
            RebalancingStrategy::Gradual { adjustment_rate_bps: 2500 } // 25% adjustment rate
        );

        let composition = IndexComposition::new(vec![
            AssetWeight::new(Asset::USD, 5000),
            AssetWeight::new(Asset::BTC, 5000),
        ]).unwrap();

        let mut current_values = HashMap::new();
        current_values.insert(Asset::USD, 6000); // 60%
        current_values.insert(Asset::BTC, 4000); // 40%

        let actions = engine.calculate_rebalancing(&composition, &current_values).unwrap();

        // With gradual strategy, adjustments should be smaller than full rebalancing
        for action in actions {
            if action.amount > 0 {
                // The adjustment should be approximately 25% of the full needed amount
                // Full rebalancing would need 1000 (from 6000 to 5000 or 4000 to 5000)
                // Gradual should adjust by ~250
                assert!(action.amount < 500); // Less than half of full adjustment
            }
        }
    }

    #[test]
    fn test_periodic_rebalancing_due() {
        let mut engine = RebalancingEngine::with_strategy(
            RebalancingStrategy::Periodic { interval_hours: 168 } // Weekly
        );

        // Initially, should be due
        assert!(engine.is_periodic_rebalancing_due());

        // After marking complete, should not be due
        engine.mark_rebalancing_complete();
        assert!(!engine.is_periodic_rebalancing_due());
    }

    #[test]
    fn test_rebalancing_action_deviation() {
        let action = RebalancingAction {
            asset: Asset::USD,
            action_type: ActionType::Buy,
            amount: 1000,
            current_weight_bps: 4000, // 40%
            target_weight_bps: 5000,  // 50%
        };

        assert_eq!(action.deviation_bps(), 1000); // 10% deviation
    }

    #[test]
    fn test_mark_rebalancing_complete() {
        let mut engine = RebalancingEngine::default();
        assert!(engine.time_since_last_rebalancing().is_none());

        engine.mark_rebalancing_complete();
        assert!(engine.time_since_last_rebalancing().is_some());

        let duration = engine.time_since_last_rebalancing().unwrap();
        assert!(duration.num_seconds() < 10); // Should be recent
    }
}
