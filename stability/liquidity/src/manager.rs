use crate::types::{LiquidityConfig, LiquidityPool, LiquidityError, Result};
use crate::pool::PoolManager;
use crate::incentives::IncentiveProgram;

/// Main liquidity manager coordinating all operations
pub struct LiquidityManager {
    config: LiquidityConfig,
    pool_manager: PoolManager,
    incentive_program: Option<IncentiveProgram>,
}

impl LiquidityManager {
    pub fn new(config: LiquidityConfig) -> Self {
        Self {
            config,
            pool_manager: PoolManager::new(),
            incentive_program: None,
        }
    }

    pub fn initialize_incentives(&mut self, total_rewards: u128, reward_rate: u128) {
        self.incentive_program = Some(IncentiveProgram::new(total_rewards, reward_rate));
    }

    pub fn add_pool(&mut self, pool: LiquidityPool) -> Result<()> {
        // Validate APR meets minimum
        if pool.apr_bps < self.config.min_apr_bps {
            return Err(LiquidityError::AprTooLow {
                current_bps: pool.apr_bps,
                min_bps: self.config.min_apr_bps,
            });
        }

        self.pool_manager.add_pool(pool);
        Ok(())
    }

    pub fn get_pool(&self, pool_id: &str) -> Option<&LiquidityPool> {
        self.pool_manager.get_pool(pool_id)
    }

    pub fn total_liquidity(&self) -> u128 {
        self.pool_manager.calculate_total_liquidity()
    }

    pub fn needs_more_liquidity(&self) -> bool {
        self.total_liquidity() < self.config.target_total_liquidity_usd
    }

    pub fn liquidity_shortfall(&self) -> u128 {
        self.config.target_total_liquidity_usd.saturating_sub(self.total_liquidity())
    }

    pub fn get_pool_manager(&self) -> &PoolManager {
        &self.pool_manager
    }

    pub fn get_pool_manager_mut(&mut self) -> &mut PoolManager {
        &mut self.pool_manager
    }

    pub fn calculate_optimal_deployment(&self, amount: u128) -> Vec<Deployment> {
        let best_pools = self.pool_manager.get_best_pools(5);
        let mut deployments = Vec::new();

        if best_pools.is_empty() {
            return deployments;
        }

        // Simple equal distribution across top pools
        let per_pool = amount / best_pools.len() as u128;

        for pool in best_pools {
            deployments.push(Deployment {
                pool_id: pool.id.clone(),
                amount: per_pool,
                expected_apr_bps: pool.apr_bps,
            });
        }

        deployments
    }
}

#[derive(Debug, Clone)]
pub struct Deployment {
    pub pool_id: String,
    pub amount: u128,
    pub expected_apr_bps: u16,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_manager_creation() {
        let config = LiquidityConfig::default();
        let manager = LiquidityManager::new(config);
        assert_eq!(manager.total_liquidity(), 0);
    }

    #[test]
    fn test_add_pool() {
        let config = LiquidityConfig::default();
        let mut manager = LiquidityManager::new(config);

        let pool = LiquidityPool {
            id: "pool1".to_string(),
            pair: "LEAN/USDC".to_string(),
            dex: "Uniswap".to_string(),
            chain: "Ethereum".to_string(),
            token0_amount: 1_000_000_000,
            token1_amount: 1_000_000_000,
            total_value_usd: 2_000_000_000,
            apr_bps: 1200, // 12% APR
            last_updated: Utc::now(),
        };

        assert!(manager.add_pool(pool).is_ok());
        assert_eq!(manager.total_liquidity(), 2_000_000_000);
    }

    #[test]
    fn test_needs_more_liquidity() {
        let config = LiquidityConfig::default();
        let manager = LiquidityManager::new(config);
        assert!(manager.needs_more_liquidity());
    }
}
