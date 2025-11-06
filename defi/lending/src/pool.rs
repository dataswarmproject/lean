//! Lending pool management

use crate::{
    Loan, LoanId, LoanConfig, LoanStatus,
    CollateralManager, CollateralAsset,
    InterestRateModel, LinearInterestModel,
    Liquidator, LiquidationResult,
    LendingError, Result,
};
use lean_crypto::Address;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Lending pool statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolStats {
    /// Total supplied to the pool
    pub total_supplied: u128,
    /// Total borrowed from the pool
    pub total_borrowed: u128,
    /// Total reserves
    pub total_reserves: u128,
    /// Current utilization rate (basis points)
    pub utilization_rate: u128,
    /// Current borrow APR (basis points)
    pub borrow_apr: u64,
    /// Current supply APR (basis points)
    pub supply_apr: u64,
}

/// Lending pool for a specific asset
pub struct LendingPool {
    /// Pool asset address
    pub asset: Address,
    /// Total supplied to pool
    pub total_supplied: u128,
    /// Total borrowed from pool
    pub total_borrowed: u128,
    /// Total reserves (protocol fees)
    pub total_reserves: u128,
    /// Reserve factor (percentage of interest going to reserves, in basis points)
    pub reserve_factor: u16,
    /// Active loans
    loans: HashMap<LoanId, Loan>,
    /// Collateral manager
    collateral_manager: CollateralManager,
    /// Interest rate model
    interest_model: Box<dyn InterestRateModel>,
    /// Liquidator
    liquidator: Liquidator,
    /// Last update timestamp
    last_update: i64,
}

impl LendingPool {
    /// Create a new lending pool
    pub fn new(
        asset: Address,
        reserve_factor: u16,
        collateral_manager: CollateralManager,
    ) -> Self {
        let liquidator = Liquidator::new(collateral_manager.clone());
        
        Self {
            asset,
            total_supplied: 0,
            total_borrowed: 0,
            total_reserves: 0,
            reserve_factor,
            loans: HashMap::new(),
            collateral_manager,
            interest_model: Box::new(LinearInterestModel::default_model()),
            liquidator,
            last_update: 0,
        }
    }

    /// Supply assets to the pool
    pub fn supply(&mut self, amount: u128, timestamp: i64) -> Result<()> {
        self.accrue_interest(timestamp)?;
        
        self.total_supplied = self
            .total_supplied
            .checked_add(amount)
            .ok_or(LendingError::ArithmeticOverflow)?;

        Ok(())
    }

    /// Withdraw assets from the pool
    pub fn withdraw(&mut self, amount: u128, timestamp: i64) -> Result<()> {
        self.accrue_interest(timestamp)?;

        let available = self.available_liquidity();
        if amount > available {
            return Err(LendingError::InsufficientLiquidity);
        }

        self.total_supplied = self
            .total_supplied
            .checked_sub(amount)
            .ok_or(LendingError::ArithmeticOverflow)?;

        Ok(())
    }

    /// Create a new loan
    pub fn borrow(
        &mut self,
        loan_id: LoanId,
        borrower: Address,
        borrow_amount: u128,
        collateral_asset: Address,
        collateral_amount: u128,
        collateral_price: u128,
        borrow_price: u128,
        timestamp: i64,
        config: LoanConfig,
    ) -> Result<()> {
        self.accrue_interest(timestamp)?;

        // Check liquidity
        if borrow_amount > self.available_liquidity() {
            return Err(LendingError::InsufficientLiquidity);
        }

        // Validate collateral
        self.collateral_manager.validate_collateral(
            &collateral_asset,
            collateral_amount,
            &self.asset,
            borrow_amount,
            collateral_price,
            borrow_price,
        )?;

        // Calculate interest rate
        let utilization = LinearInterestModel::utilization_rate(
            self.total_borrowed + borrow_amount,
            self.total_supplied,
        );
        let interest_rate = self.interest_model.borrow_rate_per_second(utilization)?;

        // Create loan
        let loan = Loan::new(
            loan_id,
            borrower,
            self.asset,
            borrow_amount,
            collateral_asset,
            collateral_amount,
            interest_rate,
            timestamp,
            config,
        );

        self.loans.insert(loan_id, loan);
        self.total_borrowed = self
            .total_borrowed
            .checked_add(borrow_amount)
            .ok_or(LendingError::ArithmeticOverflow)?;

        Ok(())
    }

    /// Repay a loan
    pub fn repay(
        &mut self,
        loan_id: &LoanId,
        amount: u128,
        timestamp: i64,
    ) -> Result<u128> {
        self.accrue_interest(timestamp)?;

        let loan = self
            .loans
            .get_mut(loan_id)
            .ok_or_else(|| LendingError::LoanNotFound(hex::encode(&loan_id.0)))?;

        loan.accrue_interest(timestamp)?;
        let repaid = loan.repay(amount)?;

        self.total_borrowed = self
            .total_borrowed
            .checked_sub(repaid)
            .ok_or(LendingError::ArithmeticOverflow)?;

        Ok(repaid)
    }

    /// Liquidate an unhealthy loan
    pub fn liquidate(
        &mut self,
        loan_id: &LoanId,
        liquidator: Address,
        collateral_price: u128,
        borrow_price: u128,
        timestamp: i64,
    ) -> Result<LiquidationResult> {
        self.accrue_interest(timestamp)?;

        let loan = self
            .loans
            .get_mut(loan_id)
            .ok_or_else(|| LendingError::LoanNotFound(hex::encode(&loan_id.0)))?;

        loan.accrue_interest(timestamp)?;

        let result = self.liquidator.liquidate(
            loan,
            liquidator,
            collateral_price,
            borrow_price,
            timestamp,
        )?;

        self.total_borrowed = self
            .total_borrowed
            .checked_sub(result.debt_repaid)
            .ok_or(LendingError::ArithmeticOverflow)?;

        Ok(result)
    }

    /// Accrue interest for all loans
    fn accrue_interest(&mut self, current_timestamp: i64) -> Result<()> {
        if current_timestamp <= self.last_update {
            return Ok(());
        }

        let elapsed = current_timestamp - self.last_update;
        if elapsed == 0 {
            return Ok(());
        }

        // Calculate current rates
        let utilization = LinearInterestModel::utilization_rate(
            self.total_borrowed,
            self.total_supplied,
        );
        let borrow_rate = self.interest_model.borrow_rate_per_second(utilization)?;

        // Accrue interest on all active loans
        let mut total_interest = 0u128;
        for loan in self.loans.values_mut() {
            if loan.status == LoanStatus::Active {
                let interest = loan.accrue_interest(current_timestamp)?;
                total_interest = total_interest
                    .checked_add(interest)
                    .ok_or(LendingError::InterestOverflow)?;
            }
        }

        // Add interest to borrowed amount and reserves
        self.total_borrowed = self
            .total_borrowed
            .checked_add(total_interest)
            .ok_or(LendingError::ArithmeticOverflow)?;

        let reserve_amount = total_interest
            .checked_mul(self.reserve_factor as u128)
            .and_then(|v| v.checked_div(10000))
            .ok_or(LendingError::ArithmeticOverflow)?;

        self.total_reserves = self
            .total_reserves
            .checked_add(reserve_amount)
            .ok_or(LendingError::ArithmeticOverflow)?;

        self.last_update = current_timestamp;
        Ok(())
    }

    /// Get available liquidity
    pub fn available_liquidity(&self) -> u128 {
        self.total_supplied
            .saturating_sub(self.total_borrowed)
            .saturating_sub(self.total_reserves)
    }

    /// Get pool statistics
    pub fn stats(&self) -> Result<PoolStats> {
        let utilization = LinearInterestModel::utilization_rate(
            self.total_borrowed,
            self.total_supplied,
        );

        let borrow_apr = self.interest_model.borrow_rate_per_second(utilization)?
            .checked_mul(31_557_600) // Convert to annual
            .unwrap_or(0);

        let supply_apr = self.interest_model.supply_rate_per_second(utilization, self.reserve_factor)?
            .checked_mul(31_557_600)
            .unwrap_or(0);

        Ok(PoolStats {
            total_supplied: self.total_supplied,
            total_borrowed: self.total_borrowed,
            total_reserves: self.total_reserves,
            utilization_rate: utilization,
            borrow_apr,
            supply_apr,
        })
    }

    /// Get a loan
    pub fn get_loan(&self, loan_id: &LoanId) -> Option<&Loan> {
        self.loans.get(loan_id)
    }

    /// Add collateral asset support
    pub fn add_collateral_asset(&mut self, asset: CollateralAsset) -> Result<()> {
        self.collateral_manager.add_asset(asset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_address(n: u8) -> Address {
        Address::from([n; 20])
    }

    fn setup_pool() -> LendingPool {
        let mut manager = CollateralManager::new();
        let collateral = CollateralAsset::new(
            test_address(2),
            7500,
            8000,
            500,
        ).unwrap();
        manager.add_asset(collateral).unwrap();

        LendingPool::new(test_address(1), 1000, manager)
    }

    #[test]
    fn test_supply_withdraw() {
        let mut pool = setup_pool();
        
        pool.supply(1000, 0).unwrap();
        assert_eq!(pool.total_supplied, 1000);
        
        pool.withdraw(500, 1).unwrap();
        assert_eq!(pool.total_supplied, 500);
    }

    #[test]
    fn test_borrow_repay() {
        let mut pool = setup_pool();
        pool.supply(10000, 0).unwrap();

        let loan_id = LoanId([1u8; 32]);
        pool.borrow(
            loan_id,
            test_address(3),
            1000,
            test_address(2),
            1500,
            1,
            1,
            0,
            LoanConfig::default(),
        ).unwrap();

        assert_eq!(pool.total_borrowed, 1000);

        pool.repay(&loan_id, 500, 100).unwrap();
        assert!(pool.total_borrowed < 1000); // May have accrued interest
    }

    #[test]
    fn test_insufficient_liquidity() {
        let mut pool = setup_pool();
        pool.supply(100, 0).unwrap();

        let loan_id = LoanId([1u8; 32]);
        let result = pool.borrow(
            loan_id,
            test_address(3),
            200, // More than available
            test_address(2),
            300,
            1,
            1,
            0,
            LoanConfig::default(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_pool_stats() {
        let mut pool = setup_pool();
        pool.supply(10000, 0).unwrap();

        let loan_id = LoanId([1u8; 32]);
        pool.borrow(
            loan_id,
            test_address(3),
            5000,
            test_address(2),
            7500,
            1,
            1,
            0,
            LoanConfig::default(),
        ).unwrap();

        let stats = pool.stats().unwrap();
        assert_eq!(stats.total_supplied, 10000);
        assert_eq!(stats.total_borrowed, 5000);
        assert_eq!(stats.utilization_rate, 5000); // 50%
    }
}
