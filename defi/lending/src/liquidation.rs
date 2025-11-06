//! Liquidation mechanism

use crate::{Loan, LoanStatus, CollateralManager, LendingError, Result};
use lean_crypto::Address;
use serde::{Deserialize, Serialize};

/// Liquidation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidationResult {
    /// Loan that was liquidated
    pub loan_id: crate::loan::LoanId,
    /// Amount of debt repaid
    pub debt_repaid: u128,
    /// Amount of collateral seized
    pub collateral_seized: u128,
    /// Liquidator address
    pub liquidator: Address,
    /// Timestamp of liquidation
    pub timestamp: i64,
}

/// Handles loan liquidations
pub struct Liquidator {
    /// Collateral manager for asset configurations
    collateral_manager: CollateralManager,
}

impl Liquidator {
    /// Create a new liquidator
    pub fn new(collateral_manager: CollateralManager) -> Self {
        Self { collateral_manager }
    }

    /// Check if a loan can be liquidated
    pub fn can_liquidate(
        &self,
        loan: &Loan,
        collateral_price: u128,
        borrow_price: u128,
    ) -> Result<bool> {
        if loan.status != LoanStatus::Active {
            return Ok(false);
        }

        let asset = self.collateral_manager.get_asset(&loan.collateral_asset)?;
        let health_factor = loan.health_factor(collateral_price, borrow_price)?;
        
        Ok(health_factor < asset.liquidation_threshold_bps as u128)
    }

    /// Liquidate a loan
    pub fn liquidate(
        &self,
        loan: &mut Loan,
        liquidator: Address,
        collateral_price: u128,
        borrow_price: u128,
        timestamp: i64,
    ) -> Result<LiquidationResult> {
        // Verify loan can be liquidated
        if !self.can_liquidate(loan, collateral_price, borrow_price)? {
            return Err(LendingError::LiquidationThresholdNotReached);
        }

        // Calculate liquidation amounts
        let total_debt = loan.total_debt()?;
        let (debt_to_repay, collateral_to_seize) = self.collateral_manager.calculate_liquidation(
            &loan.collateral_asset,
            total_debt,
            collateral_price,
            borrow_price,
        )?;

        // Ensure we don't seize more collateral than available
        let actual_collateral_seized = collateral_to_seize.min(loan.collateral_amount);

        // Calculate actual debt repaid based on collateral seized
        let actual_debt_repaid = if actual_collateral_seized < collateral_to_seize {
            // Partial liquidation
            let asset = self.collateral_manager.get_asset(&loan.collateral_asset)?;
            let bonus_multiplier = 10000u128 + asset.liquidation_bonus_bps as u128;
            
            actual_collateral_seized
                .checked_mul(collateral_price)
                .and_then(|v| v.checked_mul(10000))
                .and_then(|v| v.checked_div(bonus_multiplier))
                .and_then(|v| v.checked_div(borrow_price))
                .ok_or(LendingError::ArithmeticOverflow)?
        } else {
            debt_to_repay
        };

        // Update loan state
        loan.collateral_amount -= actual_collateral_seized;
        loan.repay(actual_debt_repaid)?;

        // If all collateral is seized or debt is fully repaid, mark as liquidated
        if loan.collateral_amount == 0 || (loan.borrow_amount == 0 && loan.accrued_interest == 0) {
            loan.liquidate()?;
        }

        Ok(LiquidationResult {
            loan_id: loan.id,
            debt_repaid: actual_debt_repaid,
            collateral_seized: actual_collateral_seized,
            liquidator,
            timestamp,
        })
    }

    /// Calculate liquidation bonus for liquidator
    pub fn calculate_bonus(
        &self,
        collateral_asset: &Address,
        collateral_seized: u128,
    ) -> Result<u128> {
        let asset = self.collateral_manager.get_asset(collateral_asset)?;
        
        let bonus = collateral_seized
            .checked_mul(asset.liquidation_bonus_bps as u128)
            .and_then(|v| v.checked_div(10000))
            .ok_or(LendingError::ArithmeticOverflow)?;

        Ok(bonus)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CollateralAsset, LoanConfig};

    fn test_address(n: u8) -> Address {
        Address::from([n; 20])
    }

    fn setup_liquidator() -> Liquidator {
        let mut manager = CollateralManager::new();
        let asset = CollateralAsset::new(
            test_address(1),
            7500,  // 75% LTV
            12500, // 125% liquidation threshold
            500,   // 5% liquidation bonus
        ).unwrap();
        manager.add_asset(asset).unwrap();
        Liquidator::new(manager)
    }

    fn create_unhealthy_loan() -> Loan {
        let mut loan = Loan::new(
            crate::loan::LoanId([0u8; 32]),
            test_address(1),
            test_address(2),
            1000,  // borrowed
            test_address(1),
            1200,  // collateral (only 120% ratio, below 125% threshold)
            10,
            0,
            LoanConfig {
                collateral_ratio_bps: 15000,
                liquidation_threshold_bps: 12500,
                liquidation_bonus_bps: 500,
            },
        );
        loan.accrued_interest = 0;
        loan
    }

    #[test]
    fn test_can_liquidate_unhealthy_loan() {
        let liquidator = setup_liquidator();
        let loan = create_unhealthy_loan();
        
        // At equal prices, health factor = 1200/1000 * 10000 = 12000 (120%)
        // Below threshold of 12500 (125%), so can liquidate
        let can_liquidate = liquidator.can_liquidate(&loan, 1, 1).unwrap();
        assert!(can_liquidate);
    }

    #[test]
    fn test_cannot_liquidate_healthy_loan() {
        let liquidator = setup_liquidator();
        let mut loan = create_unhealthy_loan();
        loan.collateral_amount = 1500; // Increase to 150% ratio
        
        let can_liquidate = liquidator.can_liquidate(&loan, 1, 1).unwrap();
        assert!(!can_liquidate);
    }

    #[test]
    fn test_liquidation_execution() {
        let liquidator = setup_liquidator();
        let mut loan = create_unhealthy_loan();
        
        let result = liquidator.liquidate(
            &mut loan,
            test_address(3),
            1,
            1,
            100,
        ).unwrap();

        assert!(result.debt_repaid > 0);
        assert!(result.collateral_seized > 0);
        assert_eq!(result.liquidator, test_address(3));
    }

    #[test]
    fn test_liquidation_bonus() {
        let liquidator = setup_liquidator();
        let bonus = liquidator.calculate_bonus(&test_address(1), 1000).unwrap();
        // 5% of 1000 = 50
        assert_eq!(bonus, 50);
    }
}
