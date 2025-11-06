//! Stability mechanisms and fee management

use crate::{StablecoinError, Result};
use lean_crypto::Address;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Stability fee configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StabilityFee {
    /// Fee rate per second (basis points)
    pub rate_per_second: u64,
    /// Last update timestamp
    pub last_update: i64,
}

impl StabilityFee {
    pub fn new(annual_rate_bps: u64, timestamp: i64) -> Self {
        const SECONDS_PER_YEAR: u64 = 31_557_600;
        Self {
            rate_per_second: annual_rate_bps / SECONDS_PER_YEAR,
            last_update: timestamp,
        }
    }
}

/// Stability pool for liquidations
pub struct StabilityPool {
    /// Total stablecoins deposited
    pub total_deposited: u128,
    /// Deposits by user
    deposits: HashMap<Address, u128>,
    /// Accumulated collateral from liquidations
    pub collateral_gains: HashMap<Address, HashMap<Address, u128>>,
}

impl StabilityPool {
    pub fn new() -> Self {
        Self {
            total_deposited: 0,
            deposits: HashMap::new(),
            collateral_gains: HashMap::new(),
        }
    }

    /// Deposit stablecoins into pool
    pub fn deposit(&mut self, user: Address, amount: u128) -> Result<()> {
        let current = self.deposits.entry(user).or_insert(0);
        *current = current
            .checked_add(amount)
            .ok_or(StablecoinError::ArithmeticOverflow)?;
        
        self.total_deposited = self
            .total_deposited
            .checked_add(amount)
            .ok_or(StablecoinError::ArithmeticOverflow)?;

        Ok(())
    }

    /// Withdraw stablecoins from pool
    pub fn withdraw(&mut self, user: &Address, amount: u128) -> Result<()> {
        let current = self.deposits.get_mut(user)
            .ok_or(StablecoinError::StabilityPoolInsufficientFunds)?;
        
        if *current < amount {
            return Err(StablecoinError::StabilityPoolInsufficientFunds);
        }

        *current -= amount;
        self.total_deposited = self
            .total_deposited
            .checked_sub(amount)
            .ok_or(StablecoinError::ArithmeticOverflow)?;

        Ok(())
    }

    /// Offset debt with pool funds during liquidation
    pub fn offset_debt(
        &mut self,
        debt_amount: u128,
        collateral_type: Address,
        collateral_amount: u128,
    ) -> Result<u128> {
        if self.total_deposited < debt_amount {
            return Err(StablecoinError::StabilityPoolInsufficientFunds);
        }

        // Distribute collateral proportionally to depositors
        for (user, deposit) in self.deposits.iter_mut() {
            if *deposit == 0 {
                continue;
            }

            let share = (*deposit as u128)
                .checked_mul(10000)
                .and_then(|v| v.checked_div(self.total_deposited))
                .ok_or(StablecoinError::ArithmeticOverflow)?;

            let collateral_share = collateral_amount
                .checked_mul(share)
                .and_then(|v| v.checked_div(10000))
                .ok_or(StablecoinError::ArithmeticOverflow)?;

            let user_gains = self.collateral_gains.entry(*user).or_insert_with(HashMap::new);
            let gain = user_gains.entry(collateral_type).or_insert(0);
            *gain = gain
                .checked_add(collateral_share)
                .ok_or(StablecoinError::ArithmeticOverflow)?;

            // Reduce deposit by debt share
            let debt_share = debt_amount
                .checked_mul(share)
                .and_then(|v| v.checked_div(10000))
                .ok_or(StablecoinError::ArithmeticOverflow)?;
            *deposit = deposit
                .checked_sub(debt_share)
                .ok_or(StablecoinError::ArithmeticOverflow)?;
        }

        self.total_deposited = self
            .total_deposited
            .checked_sub(debt_amount)
            .ok_or(StablecoinError::ArithmeticOverflow)?;

        Ok(debt_amount)
    }

    /// Claim collateral gains
    pub fn claim_collateral_gains(
        &mut self,
        user: &Address,
        collateral_type: &Address,
    ) -> Result<u128> {
        let gains = self.collateral_gains
            .get_mut(user)
            .and_then(|m| m.get_mut(collateral_type))
            .ok_or(StablecoinError::StabilityPoolInsufficientFunds)?;

        let amount = *gains;
        *gains = 0;
        Ok(amount)
    }
}

impl Default for StabilityPool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_address(n: u8) -> Address {
        Address::from([n; 20])
    }

    #[test]
    fn test_deposit_withdraw() {
        let mut pool = StabilityPool::new();
        pool.deposit(test_address(1), 1000).unwrap();
        assert_eq!(pool.total_deposited, 1000);

        pool.withdraw(&test_address(1), 500).unwrap();
        assert_eq!(pool.total_deposited, 500);
    }

    #[test]
    fn test_offset_debt() {
        let mut pool = StabilityPool::new();
        pool.deposit(test_address(1), 1000).unwrap();
        pool.deposit(test_address(2), 1000).unwrap();

        let offset = pool.offset_debt(500, test_address(3), 600).unwrap();
        assert_eq!(offset, 500);
        assert_eq!(pool.total_deposited, 1500);
    }
}
