use crate::types::{TreasuryAsset, TreasuryHolding, TreasuryConfig, TreasuryError, Result};
use std::collections::HashMap;

/// Manages asset allocation in treasury
pub struct AssetAllocator {
    config: TreasuryConfig,
    holdings: HashMap<TreasuryAsset, TreasuryHolding>,
}

impl AssetAllocator {
    pub fn new(config: TreasuryConfig) -> Self {
        Self {
            config,
            holdings: HashMap::new(),
        }
    }

    pub fn update_holding(&mut self, holding: TreasuryHolding) {
        self.holdings.insert(holding.asset, holding);
    }

    pub fn get_holding(&self, asset: &TreasuryAsset) -> Option<&TreasuryHolding> {
        self.holdings.get(asset)
    }

    pub fn total_value_usd(&self) -> u128 {
        self.holdings.values().map(|h| h.value_usd).sum()
    }

    pub fn current_allocation(&self) -> HashMap<TreasuryAsset, u16> {
        let total = self.total_value_usd();
        if total == 0 {
            return HashMap::new();
        }

        self.holdings.iter().map(|(asset, holding)| {
            let allocation_bps = ((holding.value_usd * 10000) / total) as u16;
            (*asset, allocation_bps)
        }).collect()
    }

    pub fn needs_rebalancing(&self) -> bool {
        let current = self.current_allocation();
        let total = self.total_value_usd();

        if total == 0 {
            return false;
        }

        for (target_asset, target_bps) in &self.config.target_allocation {
            let current_bps = current.get(target_asset).copied().unwrap_or(0);
            let deviation = if current_bps > *target_bps {
                current_bps - target_bps
            } else {
                target_bps - current_bps
            };

            if deviation > self.config.rebalancing_threshold_bps {
                return true;
            }
        }

        false
    }

    pub fn calculate_rebalancing_actions(&self) -> Vec<RebalancingAction> {
        let mut actions = Vec::new();
        let total = self.total_value_usd();

        if total == 0 {
            return actions;
        }

        let current = self.current_allocation();

        for (target_asset, target_bps) in &self.config.target_allocation {
            let current_bps = current.get(target_asset).copied().unwrap_or(0);
            let current_value = self.holdings.get(target_asset)
                .map(|h| h.value_usd)
                .unwrap_or(0);

            let target_value = (total * *target_bps as u128) / 10000;

            if current_value < target_value {
                actions.push(RebalancingAction {
                    asset: *target_asset,
                    action: ActionType::Buy,
                    amount_usd: target_value - current_value,
                });
            } else if current_value > target_value {
                actions.push(RebalancingAction {
                    asset: *target_asset,
                    action: ActionType::Sell,
                    amount_usd: current_value - target_value,
                });
            }
        }

        actions
    }

    pub fn get_target_allocation(&self, asset: &TreasuryAsset) -> Option<u16> {
        self.config.target_allocation.iter()
            .find(|(a, _)| a == asset)
            .map(|(_, bps)| *bps)
    }
}

#[derive(Debug, Clone)]
pub struct RebalancingAction {
    pub asset: TreasuryAsset,
    pub action: ActionType,
    pub amount_usd: u128,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActionType {
    Buy,
    Sell,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_allocator_creation() {
        let config = TreasuryConfig::default();
        let allocator = AssetAllocator::new(config);
        assert_eq!(allocator.total_value_usd(), 0);
    }

    #[test]
    fn test_update_and_total_value() {
        let config = TreasuryConfig::default();
        let mut allocator = AssetAllocator::new(config);

        let holding = TreasuryHolding {
            asset: TreasuryAsset::Stablecoin,
            amount: 1_000_000_000_000,
            value_usd: 1_000_000_000_000,
            last_updated: Utc::now(),
        };

        allocator.update_holding(holding);
        assert_eq!(allocator.total_value_usd(), 1_000_000_000_000);
    }

    #[test]
    fn test_current_allocation() {
        let config = TreasuryConfig::default();
        let mut allocator = AssetAllocator::new(config);

        allocator.update_holding(TreasuryHolding {
            asset: TreasuryAsset::Stablecoin,
            amount: 1000,
            value_usd: 5000,
            last_updated: Utc::now(),
        });

        allocator.update_holding(TreasuryHolding {
            asset: TreasuryAsset::Bitcoin,
            amount: 1,
            value_usd: 5000,
            last_updated: Utc::now(),
        });

        let allocation = allocator.current_allocation();
        assert_eq!(allocation.get(&TreasuryAsset::Stablecoin), Some(&5000)); // 50%
        assert_eq!(allocation.get(&TreasuryAsset::Bitcoin), Some(&5000)); // 50%
    }
}
