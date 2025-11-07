use crate::types::{TreasuryOperation, TreasuryError, Result};
use chrono::{DateTime, Utc, Duration};
use std::collections::VecDeque;

/// Buy-back program for price support
pub struct BuyBackProgram {
    /// Total budget allocated
    budget: u128,
    /// Amount spent so far
    spent: u128,
    /// Target price for buy-backs
    target_price: u128,
    /// Maximum price to buy at (in basis points above target)
    max_premium_bps: u16,
    /// History of buy-backs
    history: VecDeque<BuyBackRecord>,
}

#[derive(Debug, Clone)]
struct BuyBackRecord {
    timestamp: DateTime<Utc>,
    amount_tokens: u128,
    amount_spent: u128,
    price: u128,
}

impl BuyBackProgram {
    pub fn new(budget: u128, target_price: u128) -> Self {
        Self {
            budget,
            spent: 0,
            target_price,
            max_premium_bps: 500, // Can buy up to 5% above target
            history: VecDeque::new(),
        }
    }

    pub fn should_execute(&self, current_price: u128) -> bool {
        // Only buy back when price is below target (or slightly above within premium)
        let max_price = self.target_price + (self.target_price * self.max_premium_bps as u128) / 10000;
        current_price <= max_price && self.remaining_budget() > 0
    }

    pub fn calculate_buyback_amount(&self, current_price: u128) -> Result<u128> {
        let remaining = self.remaining_budget();
        if remaining == 0 {
            return Err(TreasuryError::InsufficientBalance {
                available: 0,
                requested: 1,
            });
        }

        // Calculate tokens that can be bought with remaining budget
        let tokens = remaining / current_price;
        Ok(tokens)
    }

    pub fn execute_buyback(&mut self, amount_tokens: u128, price: u128) -> Result<TreasuryOperation> {
        let cost = amount_tokens.checked_mul(price)
            .ok_or(TreasuryError::CalculationOverflow)?;

        if cost > self.remaining_budget() {
            return Err(TreasuryError::InsufficientBalance {
                available: self.remaining_budget(),
                requested: cost,
            });
        }

        self.spent += cost;

        self.history.push_back(BuyBackRecord {
            timestamp: Utc::now(),
            amount_tokens,
            amount_spent: cost,
            price,
        });

        // Keep last 1000 records
        if self.history.len() > 1000 {
            self.history.pop_front();
        }

        Ok(TreasuryOperation::BuyBack {
            amount_tokens,
            amount_spent: cost,
            price,
        })
    }

    pub fn remaining_budget(&self) -> u128 {
        self.budget.saturating_sub(self.spent)
    }

    pub fn total_bought(&self) -> u128 {
        self.history.iter().map(|r| r.amount_tokens).sum()
    }

    pub fn average_price(&self) -> Option<u128> {
        if self.history.is_empty() {
            return None;
        }

        let total_spent: u128 = self.history.iter().map(|r| r.amount_spent).sum();
        let total_tokens: u128 = self.history.iter().map(|r| r.amount_tokens).sum();

        if total_tokens == 0 {
            None
        } else {
            Some(total_spent / total_tokens)
        }
    }
}

/// Market maker for providing liquidity
pub struct MarketMaker {
    /// Total liquidity allocated
    total_liquidity: u128,
    /// Current deployed liquidity
    deployed: u128,
    /// Target liquidity pairs
    pairs: Vec<String>,
}

impl MarketMaker {
    pub fn new(total_liquidity: u128, pairs: Vec<String>) -> Self {
        Self {
            total_liquidity,
            deployed: 0,
            pairs,
        }
    }

    pub fn can_provide_liquidity(&self, amount: u128) -> bool {
        self.deployed + amount <= self.total_liquidity
    }

    pub fn provide_liquidity(&mut self, amount: u128, pair: String) -> Result<TreasuryOperation> {
        if !self.can_provide_liquidity(amount) {
            return Err(TreasuryError::InsufficientBalance {
                available: self.total_liquidity - self.deployed,
                requested: amount,
            });
        }

        self.deployed += amount;

        Ok(TreasuryOperation::MarketMaking {
            liquidity_provided: amount,
            pair,
        })
    }

    pub fn remove_liquidity(&mut self, amount: u128) -> Result<()> {
        if amount > self.deployed {
            return Err(TreasuryError::InsufficientBalance {
                available: self.deployed,
                requested: amount,
            });
        }

        self.deployed -= amount;
        Ok(())
    }

    pub fn available_liquidity(&self) -> u128 {
        self.total_liquidity.saturating_sub(self.deployed)
    }

    pub fn utilization_rate(&self) -> u16 {
        if self.total_liquidity == 0 {
            return 0;
        }
        ((self.deployed * 10000) / self.total_liquidity) as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buyback_creation() {
        let program = BuyBackProgram::new(1_000_000_000_000, 100_000_000);
        assert_eq!(program.remaining_budget(), 1_000_000_000_000);
    }

    #[test]
    fn test_buyback_execution() {
        let mut program = BuyBackProgram::new(1_000_000_000_000, 100_000_000);
        let price = 95_000_000; // Below target

        let result = program.execute_buyback(1_000_000, price);
        assert!(result.is_ok());
        assert_eq!(program.remaining_budget(), 1_000_000_000_000 - 95_000_000_000_000);
    }

    #[test]
    fn test_market_maker() {
        let mut mm = MarketMaker::new(10_000_000_000, vec!["LEAN/USDC".to_string()]);
        assert!(mm.can_provide_liquidity(5_000_000_000));

        let result = mm.provide_liquidity(5_000_000_000, "LEAN/USDC".to_string());
        assert!(result.is_ok());
        assert_eq!(mm.utilization_rate(), 5000); // 50%
    }
}
