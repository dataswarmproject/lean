//! Loan management

use crate::{LendingError, Result};
use lean_crypto::Address;
use serde::{Deserialize, Serialize};

/// Loan unique identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LoanId(pub [u8; 32]);

/// Loan status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoanStatus {
    /// Loan is active and accruing interest
    Active,
    /// Loan has been fully repaid
    Repaid,
    /// Loan has been liquidated
    Liquidated,
    /// Loan is in default but not yet liquidated
    Default,
}

/// Loan configuration parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanConfig {
    /// Collateral ratio in basis points (e.g., 15000 = 150%)
    pub collateral_ratio_bps: u16,
    /// Liquidation threshold in basis points (e.g., 12500 = 125%)
    pub liquidation_threshold_bps: u16,
    /// Liquidation bonus in basis points (e.g., 500 = 5%)
    pub liquidation_bonus_bps: u16,
}

impl Default for LoanConfig {
    fn default() -> Self {
        Self {
            collateral_ratio_bps: 15000, // 150%
            liquidation_threshold_bps: 12500, // 125%
            liquidation_bonus_bps: 500, // 5%
        }
    }
}

/// Represents a loan in the protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Loan {
    /// Unique loan identifier
    pub id: LoanId,
    /// Borrower's address
    pub borrower: Address,
    /// Borrowed asset address
    pub borrow_asset: Address,
    /// Borrowed amount
    pub borrow_amount: u128,
    /// Collateral asset address
    pub collateral_asset: Address,
    /// Collateral amount
    pub collateral_amount: u128,
    /// Interest rate per second (in basis points)
    pub interest_rate_per_second: u64,
    /// Timestamp when loan was created
    pub created_at: i64,
    /// Timestamp of last interest accrual
    pub last_accrued_at: i64,
    /// Accumulated interest
    pub accrued_interest: u128,
    /// Loan status
    pub status: LoanStatus,
    /// Loan configuration
    pub config: LoanConfig,
}

impl Loan {
    /// Create a new loan
    pub fn new(
        id: LoanId,
        borrower: Address,
        borrow_asset: Address,
        borrow_amount: u128,
        collateral_asset: Address,
        collateral_amount: u128,
        interest_rate_per_second: u64,
        timestamp: i64,
        config: LoanConfig,
    ) -> Self {
        Self {
            id,
            borrower,
            borrow_asset,
            borrow_amount,
            collateral_asset,
            collateral_amount,
            interest_rate_per_second,
            created_at: timestamp,
            last_accrued_at: timestamp,
            accrued_interest: 0,
            status: LoanStatus::Active,
            config,
        }
    }

    /// Calculate accrued interest up to a given timestamp
    pub fn calculate_interest(&self, current_timestamp: i64) -> Result<u128> {
        if self.status != LoanStatus::Active {
            return Ok(0);
        }

        let elapsed = current_timestamp
            .checked_sub(self.last_accrued_at)
            .ok_or(LendingError::ArithmeticOverflow)?;

        if elapsed <= 0 {
            return Ok(0);
        }

        // Interest = principal * rate * time
        // rate is per second in basis points (1 bp = 0.0001)
        let interest = self
            .borrow_amount
            .checked_mul(self.interest_rate_per_second as u128)
            .and_then(|v| v.checked_mul(elapsed as u128))
            .and_then(|v| v.checked_div(10000)) // Convert basis points
            .ok_or(LendingError::InterestOverflow)?;

        Ok(interest)
    }

    /// Accrue interest up to the current timestamp
    pub fn accrue_interest(&mut self, current_timestamp: i64) -> Result<u128> {
        let new_interest = self.calculate_interest(current_timestamp)?;
        self.accrued_interest = self
            .accrued_interest
            .checked_add(new_interest)
            .ok_or(LendingError::InterestOverflow)?;
        self.last_accrued_at = current_timestamp;
        Ok(new_interest)
    }

    /// Get total debt (principal + interest)
    pub fn total_debt(&self) -> Result<u128> {
        self.borrow_amount
            .checked_add(self.accrued_interest)
            .ok_or(LendingError::ArithmeticOverflow)
    }

    /// Calculate health factor (collateral value / debt value)
    /// Returns value in basis points (e.g., 15000 = 1.5 = 150%)
    pub fn health_factor(&self, collateral_price: u128, borrow_price: u128) -> Result<u128> {
        let total_debt = self.total_debt()?;
        if total_debt == 0 {
            return Ok(u128::MAX); // No debt = infinite health
        }

        // Collateral value = collateral_amount * collateral_price
        let collateral_value = self
            .collateral_amount
            .checked_mul(collateral_price)
            .ok_or(LendingError::ArithmeticOverflow)?;

        // Debt value = total_debt * borrow_price
        let debt_value = total_debt
            .checked_mul(borrow_price)
            .ok_or(LendingError::ArithmeticOverflow)?;

        // Health factor = (collateral_value / debt_value) * 10000 (for basis points)
        let health_factor = collateral_value
            .checked_mul(10000)
            .and_then(|v| v.checked_div(debt_value))
            .ok_or(LendingError::ArithmeticOverflow)?;

        Ok(health_factor)
    }

    /// Check if loan is healthy (above liquidation threshold)
    pub fn is_healthy(&self, collateral_price: u128, borrow_price: u128) -> Result<bool> {
        let health = self.health_factor(collateral_price, borrow_price)?;
        Ok(health >= self.config.liquidation_threshold_bps as u128)
    }

    /// Check if loan can be liquidated
    pub fn can_liquidate(&self, collateral_price: u128, borrow_price: u128) -> Result<bool> {
        if self.status != LoanStatus::Active {
            return Ok(false);
        }
        Ok(!self.is_healthy(collateral_price, borrow_price)?)
    }

    /// Repay part of the loan
    pub fn repay(&mut self, amount: u128) -> Result<u128> {
        if self.status != LoanStatus::Active {
            return Err(LendingError::LoanNotFound("Loan is not active".to_string()));
        }

        let total_debt = self.total_debt()?;
        let actual_repay = amount.min(total_debt);

        // First pay off interest
        if actual_repay <= self.accrued_interest {
            self.accrued_interest -= actual_repay;
        } else {
            let remaining = actual_repay - self.accrued_interest;
            self.accrued_interest = 0;
            self.borrow_amount -= remaining;
        }

        // Check if loan is fully repaid
        if self.borrow_amount == 0 && self.accrued_interest == 0 {
            self.status = LoanStatus::Repaid;
        }

        Ok(actual_repay)
    }

    /// Mark loan as liquidated
    pub fn liquidate(&mut self) -> Result<()> {
        if self.status != LoanStatus::Active {
            return Err(LendingError::LoanNotFound("Loan is not active".to_string()));
        }
        self.status = LoanStatus::Liquidated;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_address() -> Address {
        Address::from([1u8; 20])
    }

    fn test_loan() -> Loan {
        Loan::new(
            LoanId([0u8; 32]),
            test_address(),
            test_address(),
            1000,
            test_address(),
            1500,
            10, // 0.1% per second
            0,
            LoanConfig::default(),
        )
    }

    #[test]
    fn test_interest_calculation() {
        let loan = test_loan();
        // After 100 seconds, interest should be: 1000 * 10 * 100 / 10000 = 100
        let interest = loan.calculate_interest(100).unwrap();
        assert_eq!(interest, 100);
    }

    #[test]
    fn test_health_factor() {
        let loan = test_loan();
        // Collateral: 1500, Debt: 1000, both at price 1
        // Health = (1500 * 1) / (1000 * 1) * 10000 = 15000 (150%)
        let health = loan.health_factor(1, 1).unwrap();
        assert_eq!(health, 15000);
    }

    #[test]
    fn test_repayment() {
        let mut loan = test_loan();
        loan.accrued_interest = 100;
        
        // Repay 50 (partial interest)
        loan.repay(50).unwrap();
        assert_eq!(loan.accrued_interest, 50);
        assert_eq!(loan.borrow_amount, 1000);
        
        // Repay 150 (rest of interest + partial principal)
        loan.repay(150).unwrap();
        assert_eq!(loan.accrued_interest, 0);
        assert_eq!(loan.borrow_amount, 900);
    }
}
