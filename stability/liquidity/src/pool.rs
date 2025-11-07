use crate::types::{LiquidityPool, PoolMetrics, DeploymentStrategy, LiquidityError, Result};
use std::collections::HashMap;

/// Manages individual liquidity pools
pub struct PoolManager {
    pools: HashMap<String, LiquidityPool>,
    metrics: HashMap<String, PoolMetrics>,
}

impl PoolManager {
    pub fn new() -> Self {
        Self {
            pools: HashMap::new(),
            metrics: HashMap::new(),
        }
    }

    pub fn add_pool(&mut self, pool: LiquidityPool) {
        self.pools.insert(pool.id.clone(), pool);
    }

    pub fn get_pool(&self, pool_id: &str) -> Option<&LiquidityPool> {
        self.pools.get(pool_id)
    }

    pub fn update_metrics(&mut self, metrics: PoolMetrics) {
        self.metrics.insert(metrics.pool_id.clone(), metrics);
    }

    pub fn calculate_total_liquidity(&self) -> u128 {
        self.pools.values().map(|p| p.total_value_usd).sum()
    }

    pub fn get_best_pools(&self, count: usize) -> Vec<&LiquidityPool> {
        let mut pools: Vec<&LiquidityPool> = self.pools.values().collect();
        pools.sort_by_key(|p| std::cmp::Reverse(p.apr_bps));
        pools.into_iter().take(count).collect()
    }

    pub fn needs_rebalancing(&self, pool_id: &str, threshold_bps: u16) -> Result<bool> {
        let pool = self.pools.get(pool_id)
            .ok_or_else(|| LiquidityError::PoolNotFound(pool_id.to_string()))?;

        // Check if token ratio has deviated significantly
        let total = pool.token0_amount + pool.token1_amount;
        if total == 0 {
            return Ok(false);
        }

        let ratio0 = ((pool.token0_amount * 10000) / total) as u16;
        let expected_ratio = 5000; // 50/50

        let deviation = if ratio0 > expected_ratio {
            ratio0 - expected_ratio
        } else {
            expected_ratio - ratio0
        };

        Ok(deviation > threshold_bps)
    }
}

impl Default for PoolManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_pool_manager_creation() {
        let manager = PoolManager::new();
        assert_eq!(manager.calculate_total_liquidity(), 0);
    }

    #[test]
    fn test_add_and_get_pool() {
        let mut manager = PoolManager::new();

        let pool = LiquidityPool {
            id: "pool1".to_string(),
            pair: "LEAN/USDC".to_string(),
            dex: "Uniswap".to_string(),
            chain: "Ethereum".to_string(),
            token0_amount: 1_000_000_000,
            token1_amount: 1_000_000_000,
            total_value_usd: 2_000_000_000,
            apr_bps: 1200,
            last_updated: Utc::now(),
        };

        manager.add_pool(pool.clone());
        assert!(manager.get_pool("pool1").is_some());
        assert_eq!(manager.calculate_total_liquidity(), 2_000_000_000);
    }
}
