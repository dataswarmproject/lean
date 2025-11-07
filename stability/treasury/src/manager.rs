use crate::types::{TreasuryConfig, TreasuryOperation, TreasuryError, Result};
use crate::operations::{BuyBackProgram, MarketMaker};
use crate::allocation::AssetAllocator;
use chrono::{DateTime, Utc, Duration};
use std::collections::VecDeque;

/// Main treasury manager coordinating all operations
pub struct TreasuryManager {
    config: TreasuryConfig,
    allocator: AssetAllocator,
    buyback: Option<BuyBackProgram>,
    market_maker: Option<MarketMaker>,
    operations_history: VecDeque<OperationRecord>,
    daily_spend: u128,
    last_reset: DateTime<Utc>,
}

#[derive(Debug, Clone)]
struct OperationRecord {
    timestamp: DateTime<Utc>,
    operation: TreasuryOperation,
    cost: u128,
}

impl TreasuryManager {
    pub fn new(config: TreasuryConfig) -> Self {
        Self {
            allocator: AssetAllocator::new(config.clone()),
            config,
            buyback: None,
            market_maker: None,
            operations_history: VecDeque::new(),
            daily_spend: 0,
            last_reset: Utc::now(),
        }
    }

    pub fn initialize_buyback(&mut self, budget: u128, target_price: u128) {
        self.buyback = Some(BuyBackProgram::new(budget, target_price));
    }

    pub fn initialize_market_maker(&mut self, liquidity: u128, pairs: Vec<String>) {
        self.market_maker = Some(MarketMaker::new(liquidity, pairs));
    }

    pub fn execute_buyback(&mut self, current_price: u128) -> Result<TreasuryOperation> {
        self.reset_daily_limit_if_needed();

        let buyback = self.buyback.as_mut()
            .ok_or(TreasuryError::OperationNotAllowed("Buyback not initialized".to_string()))?;

        if !buyback.should_execute(current_price) {
            return Err(TreasuryError::OperationNotAllowed("Conditions not met for buyback".to_string()));
        }

        let amount = buyback.calculate_buyback_amount(current_price)?;
        let cost = amount.checked_mul(current_price)
            .ok_or(TreasuryError::CalculationOverflow)?;

        self.check_daily_limit(cost)?;
        self.check_reserve_level(cost)?;

        let operation = buyback.execute_buyback(amount, current_price)?;
        self.record_operation(operation.clone(), cost);

        Ok(operation)
    }

    pub fn provide_liquidity(&mut self, amount: u128, pair: String) -> Result<TreasuryOperation> {
        self.reset_daily_limit_if_needed();

        let market_maker = self.market_maker.as_mut()
            .ok_or(TreasuryError::OperationNotAllowed("Market maker not initialized".to_string()))?;

        self.check_daily_limit(amount)?;
        self.check_reserve_level(amount)?;

        let operation = market_maker.provide_liquidity(amount, pair)?;
        self.record_operation(operation.clone(), amount);

        Ok(operation)
    }

    pub fn total_value(&self) -> u128 {
        self.allocator.total_value_usd()
    }

    pub fn needs_rebalancing(&self) -> bool {
        self.allocator.needs_rebalancing()
    }

    pub fn get_allocator(&self) -> &AssetAllocator {
        &self.allocator
    }

    pub fn get_allocator_mut(&mut self) -> &mut AssetAllocator {
        &mut self.allocator
    }

    pub fn daily_spend_remaining(&self) -> u128 {
        let limit = (self.total_value() * self.config.max_daily_spend_bps as u128) / 10000;
        limit.saturating_sub(self.daily_spend)
    }

    fn check_daily_limit(&self, amount: u128) -> Result<()> {
        let limit = (self.total_value() * self.config.max_daily_spend_bps as u128) / 10000;

        if self.daily_spend + amount > limit {
            return Err(TreasuryError::DailyLimitExceeded {
                spent: self.daily_spend + amount,
                limit,
            });
        }

        Ok(())
    }

    fn check_reserve_level(&self, spend_amount: u128) -> Result<()> {
        let total = self.total_value();
        let after_spend = total.saturating_sub(spend_amount);
        let reserve_bps = ((after_spend * 10000) / total) as u16;

        if reserve_bps < self.config.min_reserve_bps {
            return Err(TreasuryError::ReserveTooLow {
                current_bps: reserve_bps,
                min_bps: self.config.min_reserve_bps,
            });
        }

        Ok(())
    }

    fn reset_daily_limit_if_needed(&mut self) {
        let elapsed = Utc::now().signed_duration_since(self.last_reset);

        if elapsed >= Duration::days(1) {
            self.daily_spend = 0;
            self.last_reset = Utc::now();
        }
    }

    fn record_operation(&mut self, operation: TreasuryOperation, cost: u128) {
        self.daily_spend += cost;

        self.operations_history.push_back(OperationRecord {
            timestamp: Utc::now(),
            operation,
            cost,
        });

        // Keep last 10000 operations
        if self.operations_history.len() > 10000 {
            self.operations_history.pop_front();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_creation() {
        let config = TreasuryConfig::default();
        let manager = TreasuryManager::new(config);
        assert_eq!(manager.total_value(), 0);
    }

    #[test]
    fn test_buyback_initialization() {
        let config = TreasuryConfig::default();
        let mut manager = TreasuryManager::new(config);

        manager.initialize_buyback(1_000_000_000_000, 100_000_000);
        assert!(manager.buyback.is_some());
    }

    #[test]
    fn test_market_maker_initialization() {
        let config = TreasuryConfig::default();
        let mut manager = TreasuryManager::new(config);

        manager.initialize_market_maker(10_000_000_000, vec!["LEAN/USDC".to_string()]);
        assert!(manager.market_maker.is_some());
    }
}
