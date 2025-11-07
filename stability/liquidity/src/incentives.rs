use crate::types::{LiquidityError, Result};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Manages liquidity mining incentives
pub struct IncentiveProgram {
    total_rewards: u128,
    distributed: u128,
    reward_rate_per_usd_per_day: u128,
    pool_allocations: HashMap<String, u16>, // pool_id -> allocation in bps
}

impl IncentiveProgram {
    pub fn new(total_rewards: u128, reward_rate_per_usd_per_day: u128) -> Self {
        Self {
            total_rewards,
            distributed: 0,
            reward_rate_per_usd_per_day,
            pool_allocations: HashMap::new(),
        }
    }

    pub fn set_pool_allocation(&mut self, pool_id: String, allocation_bps: u16) -> Result<()> {
        if allocation_bps > 10000 {
            return Err(LiquidityError::InvalidConfig(
                format!("Allocation {}bps exceeds 100%", allocation_bps)
            ));
        }
        self.pool_allocations.insert(pool_id, allocation_bps);
        self.validate_total_allocation()?;
        Ok(())
    }

    pub fn calculate_rewards(&self, pool_id: &str, liquidity_usd: u128, days: u32) -> Result<u128> {
        let allocation_bps = self.pool_allocations.get(pool_id)
            .ok_or_else(|| LiquidityError::PoolNotFound(pool_id.to_string()))?;

        let base_rewards = liquidity_usd
            .checked_mul(self.reward_rate_per_usd_per_day)
            .and_then(|r| r.checked_mul(days as u128))
            .ok_or(LiquidityError::CalculationOverflow)?;

        let allocated_rewards = (base_rewards * *allocation_bps as u128) / 10000;

        if self.distributed + allocated_rewards > self.total_rewards {
            return Err(LiquidityError::InsufficientLiquidity {
                available: self.total_rewards - self.distributed,
                requested: allocated_rewards,
            });
        }

        Ok(allocated_rewards)
    }

    pub fn distribute_rewards(&mut self, amount: u128) -> Result<()> {
        if self.distributed + amount > self.total_rewards {
            return Err(LiquidityError::InsufficientLiquidity {
                available: self.total_rewards - self.distributed,
                requested: amount,
            });
        }

        self.distributed += amount;
        Ok(())
    }

    pub fn remaining_rewards(&self) -> u128 {
        self.total_rewards.saturating_sub(self.distributed)
    }

    fn validate_total_allocation(&self) -> Result<()> {
        let total: u32 = self.pool_allocations.values().map(|&a| a as u32).sum();

        if total > 10000 {
            return Err(LiquidityError::InvalidConfig(
                format!("Total allocation {}bps exceeds 100%", total)
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incentive_program_creation() {
        let program = IncentiveProgram::new(1_000_000_000_000, 1000);
        assert_eq!(program.remaining_rewards(), 1_000_000_000_000);
    }

    #[test]
    fn test_set_pool_allocation() {
        let mut program = IncentiveProgram::new(1_000_000_000_000, 1000);
        assert!(program.set_pool_allocation("pool1".to_string(), 5000).is_ok());
    }

    #[test]
    fn test_calculate_rewards() {
        let mut program = IncentiveProgram::new(1_000_000_000_000, 1000);
        program.set_pool_allocation("pool1".to_string(), 5000).unwrap(); // 50%

        let rewards = program.calculate_rewards("pool1", 1_000_000_000, 30).unwrap();
        // 1B USD * 1000 reward rate * 30 days * 50% = 15B
        assert_eq!(rewards, 15_000_000_000_000);
    }

    #[test]
    fn test_distribute_rewards() {
        let mut program = IncentiveProgram::new(1_000_000_000_000, 1000);
        assert!(program.distribute_rewards(500_000_000_000).is_ok());
        assert_eq!(program.distributed, 500_000_000_000);
        assert_eq!(program.remaining_rewards(), 500_000_000_000);
    }
}
