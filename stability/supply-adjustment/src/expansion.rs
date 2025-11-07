use crate::types::{AdjustmentMechanism, SupplyChange, AdjustmentType, AdjustmentError, Result};
use chrono::Utc;

/// Manages supply expansion mechanisms
pub struct ExpansionMechanism {
    /// Current staking reward rate (APR in basis points)
    staking_reward_rate_bps: u16,
    /// Current liquidity mining rate (APR in basis points)
    liquidity_mining_rate_bps: u16,
    /// Maximum allowed staking reward rate
    max_staking_rate_bps: u16,
    /// Maximum allowed liquidity mining rate
    max_liquidity_mining_rate_bps: u16,
}

impl ExpansionMechanism {
    /// Create a new expansion mechanism
    pub fn new() -> Self {
        Self {
            staking_reward_rate_bps: 1000,        // 10% APR default
            liquidity_mining_rate_bps: 1500,      // 15% APR default
            max_staking_rate_bps: 3000,           // 30% max
            max_liquidity_mining_rate_bps: 5000,  // 50% max
        }
    }

    /// Calculate expansion via increased staking rewards
    pub fn calculate_staking_expansion(
        &self,
        total_staked: u128,
        target_adjustment: u128,
    ) -> Result<SupplyChange> {
        if total_staked == 0 {
            return Err(AdjustmentError::InvalidAmount("No tokens staked".to_string()));
        }

        // Calculate how much to increase rewards
        let current_annual_rewards = (total_staked * self.staking_reward_rate_bps as u128) / 10000;

        // Determine new rate needed to achieve target adjustment over time
        // Assume we want to achieve target over 1 year of staking
        let needed_rate = ((target_adjustment + current_annual_rewards) * 10000) / total_staked;

        if needed_rate as u16 > self.max_staking_rate_bps {
            return Err(AdjustmentError::InvalidAmount(
                format!("Required rate {}bps exceeds maximum {}bps",
                    needed_rate, self.max_staking_rate_bps)
            ));
        }

        Ok(SupplyChange::new(
            AdjustmentType::Expansion,
            AdjustmentMechanism::StakingRewards,
            target_adjustment,
            0, // Deviation should be set by caller
        ))
    }

    /// Calculate expansion via liquidity mining
    pub fn calculate_liquidity_mining_expansion(
        &self,
        total_liquidity: u128,
        target_adjustment: u128,
    ) -> Result<SupplyChange> {
        if total_liquidity == 0 {
            return Err(AdjustmentError::InvalidAmount("No liquidity provided".to_string()));
        }

        // Calculate rewards as percentage of liquidity
        let annual_rewards = (total_liquidity * self.liquidity_mining_rate_bps as u128) / 10000;

        if target_adjustment > annual_rewards * 2 {
            return Err(AdjustmentError::InvalidAmount(
                "Target adjustment too large for liquidity mining".to_string()
            ));
        }

        Ok(SupplyChange::new(
            AdjustmentType::Expansion,
            AdjustmentMechanism::LiquidityMining,
            target_adjustment,
            0,
        ))
    }

    /// Calculate direct airdrop expansion
    pub fn calculate_airdrop_expansion(
        &self,
        holder_count: u64,
        amount_per_holder: u128,
    ) -> Result<SupplyChange> {
        let total_amount = (holder_count as u128)
            .checked_mul(amount_per_holder)
            .ok_or(AdjustmentError::CalculationOverflow)?;

        Ok(SupplyChange::new(
            AdjustmentType::Expansion,
            AdjustmentMechanism::Airdrop,
            total_amount,
            0,
        ))
    }

    /// Distribute expansion across multiple mechanisms
    pub fn distribute_expansion(
        &self,
        total_expansion: u128,
        total_staked: u128,
        total_liquidity: u128,
    ) -> Result<Vec<SupplyChange>> {
        let mut changes = Vec::new();

        // Distribute: 60% to staking, 40% to liquidity mining
        let staking_portion = (total_expansion * 60) / 100;
        let liquidity_portion = total_expansion - staking_portion;

        // Staking rewards
        if total_staked > 0 && staking_portion > 0 {
            if let Ok(change) = self.calculate_staking_expansion(total_staked, staking_portion) {
                changes.push(change);
            }
        }

        // Liquidity mining
        if total_liquidity > 0 && liquidity_portion > 0 {
            if let Ok(change) = self.calculate_liquidity_mining_expansion(total_liquidity, liquidity_portion) {
                changes.push(change);
            }
        }

        if changes.is_empty() {
            return Err(AdjustmentError::NoAdjustmentNeeded);
        }

        Ok(changes)
    }

    /// Get current staking reward rate
    pub fn staking_rate(&self) -> u16 {
        self.staking_reward_rate_bps
    }

    /// Get current liquidity mining rate
    pub fn liquidity_mining_rate(&self) -> u16 {
        self.liquidity_mining_rate_bps
    }

    /// Update staking reward rate
    pub fn set_staking_rate(&mut self, rate_bps: u16) -> Result<()> {
        if rate_bps > self.max_staking_rate_bps {
            return Err(AdjustmentError::InvalidAmount(
                format!("Rate {}bps exceeds maximum {}bps", rate_bps, self.max_staking_rate_bps)
            ));
        }
        self.staking_reward_rate_bps = rate_bps;
        Ok(())
    }

    /// Update liquidity mining rate
    pub fn set_liquidity_mining_rate(&mut self, rate_bps: u16) -> Result<()> {
        if rate_bps > self.max_liquidity_mining_rate_bps {
            return Err(AdjustmentError::InvalidAmount(
                format!("Rate {}bps exceeds maximum {}bps", rate_bps, self.max_liquidity_mining_rate_bps)
            ));
        }
        self.liquidity_mining_rate_bps = rate_bps;
        Ok(())
    }
}

impl Default for ExpansionMechanism {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expansion_mechanism_creation() {
        let mechanism = ExpansionMechanism::new();
        assert_eq!(mechanism.staking_rate(), 1000);
        assert_eq!(mechanism.liquidity_mining_rate(), 1500);
    }

    #[test]
    fn test_staking_expansion() {
        let mechanism = ExpansionMechanism::new();
        let total_staked = 1_000_000_000_000; // 1T tokens
        let target = 50_000_000_000;          // 50B tokens (5% expansion)

        let change = mechanism.calculate_staking_expansion(total_staked, target).unwrap();
        assert_eq!(change.adjustment_type, AdjustmentType::Expansion);
        assert_eq!(change.mechanism, AdjustmentMechanism::StakingRewards);
        assert_eq!(change.amount, target);
    }

    #[test]
    fn test_liquidity_mining_expansion() {
        let mechanism = ExpansionMechanism::new();
        let total_liquidity = 500_000_000_000; // 500B tokens
        let target = 25_000_000_000;           // 25B tokens

        let change = mechanism.calculate_liquidity_mining_expansion(total_liquidity, target).unwrap();
        assert_eq!(change.adjustment_type, AdjustmentType::Expansion);
        assert_eq!(change.mechanism, AdjustmentMechanism::LiquidityMining);
    }

    #[test]
    fn test_airdrop_expansion() {
        let mechanism = ExpansionMechanism::new();
        let holders = 100_000;
        let amount_per_holder = 1_000_000; // 1M tokens each

        let change = mechanism.calculate_airdrop_expansion(holders, amount_per_holder).unwrap();
        assert_eq!(change.amount, 100_000_000_000); // 100B total
        assert_eq!(change.mechanism, AdjustmentMechanism::Airdrop);
    }

    #[test]
    fn test_distribute_expansion() {
        let mechanism = ExpansionMechanism::new();
        let total_expansion = 100_000_000_000; // 100B tokens
        let total_staked = 1_000_000_000_000;  // 1T staked
        let total_liquidity = 500_000_000_000; // 500B liquidity

        let changes = mechanism.distribute_expansion(
            total_expansion,
            total_staked,
            total_liquidity
        ).unwrap();

        assert_eq!(changes.len(), 2);

        let staking_change = changes.iter().find(|c| c.mechanism == AdjustmentMechanism::StakingRewards).unwrap();
        let liquidity_change = changes.iter().find(|c| c.mechanism == AdjustmentMechanism::LiquidityMining).unwrap();

        // 60% to staking = 60B
        assert_eq!(staking_change.amount, 60_000_000_000);
        // 40% to liquidity = 40B
        assert_eq!(liquidity_change.amount, 40_000_000_000);
    }

    #[test]
    fn test_update_rates() {
        let mut mechanism = ExpansionMechanism::new();

        assert!(mechanism.set_staking_rate(2000).is_ok());
        assert_eq!(mechanism.staking_rate(), 2000);

        assert!(mechanism.set_liquidity_mining_rate(2500).is_ok());
        assert_eq!(mechanism.liquidity_mining_rate(), 2500);

        // Test exceeding maximum
        assert!(mechanism.set_staking_rate(5000).is_err());
    }

    #[test]
    fn test_zero_staked_error() {
        let mechanism = ExpansionMechanism::new();
        let result = mechanism.calculate_staking_expansion(0, 1_000_000);
        assert!(result.is_err());
    }
}
