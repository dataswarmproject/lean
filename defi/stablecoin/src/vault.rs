//! Collateralized debt position (CDP) vaults

use crate::{StablecoinError, Result};
use lean_crypto::Address;
use serde::{Deserialize, Serialize};

/// Vault unique identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VaultId(pub [u8; 32]);

/// Vault status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VaultStatus {
    /// Vault is active and can be modified
    Active,
    /// Vault is being liquidated
    Liquidating,
    /// Vault has been liquidated
    Liquidated,
    /// Vault is closed (all debt repaid, collateral withdrawn)
    Closed,
}

/// Collateralized debt position vault
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vault {
    /// Unique vault identifier
    pub id: VaultId,
    /// Vault owner
    pub owner: Address,
    /// Collateral type (asset address)
    pub collateral_type: Address,
    /// Collateral amount deposited
    pub collateral_amount: u128,
    /// Debt amount (stablecoins minted)
    pub debt_amount: u128,
    /// Accumulated stability fee
    pub accumulated_fee: u128,
    /// Collateralization ratio requirement (basis points)
    pub min_collateral_ratio: u16,
    /// Last fee accrual timestamp
    pub last_fee_accrual: i64,
    /// Vault creation timestamp
    pub created_at: i64,
    /// Vault status
    pub status: VaultStatus,
}

impl Vault {
    /// Create a new vault
    pub fn new(
        id: VaultId,
        owner: Address,
        collateral_type: Address,
        collateral_amount: u128,
        debt_amount: u128,
        min_collateral_ratio: u16,
        timestamp: i64,
    ) -> Result<Self> {
        if min_collateral_ratio < 10000 {
            return Err(StablecoinError::InsufficientCollateral {
                required: 10000,
                provided: min_collateral_ratio as u128,
            });
        }

        Ok(Self {
            id,
            owner,
            collateral_type,
            collateral_amount,
            debt_amount,
            accumulated_fee: 0,
            min_collateral_ratio,
            last_fee_accrual: timestamp,
            created_at: timestamp,
            status: VaultStatus::Active,
        })
    }

    /// Calculate collateralization ratio
    /// Returns ratio in basis points (e.g., 15000 = 150%)
    pub fn collateralization_ratio(&self, collateral_price: u128) -> Result<u128> {
        let total_debt = self.total_debt()?;
        if total_debt == 0 {
            return Ok(u128::MAX);
        }

        // Ratio = (collateral_amount * price) / debt * 10000
        let collateral_value = self
            .collateral_amount
            .checked_mul(collateral_price)
            .ok_or(StablecoinError::ArithmeticOverflow)?;

        let ratio = collateral_value
            .checked_mul(10000)
            .and_then(|v| v.checked_div(total_debt))
            .ok_or(StablecoinError::ArithmeticOverflow)?;

        Ok(ratio)
    }

    /// Check if vault is safe (above minimum collateral ratio)
    pub fn is_safe(&self, collateral_price: u128) -> Result<bool> {
        let ratio = self.collateralization_ratio(collateral_price)?;
        Ok(ratio >= self.min_collateral_ratio as u128)
    }

    /// Total debt including fees
    pub fn total_debt(&self) -> Result<u128> {
        self.debt_amount
            .checked_add(self.accumulated_fee)
            .ok_or(StablecoinError::ArithmeticOverflow)
    }

    /// Add collateral to vault
    pub fn add_collateral(&mut self, amount: u128) -> Result<()> {
        if self.status != VaultStatus::Active {
            return Err(StablecoinError::VaultNotFound("Vault not active".to_string()));
        }

        self.collateral_amount = self
            .collateral_amount
            .checked_add(amount)
            .ok_or(StablecoinError::ArithmeticOverflow)?;

        Ok(())
    }

    /// Withdraw collateral from vault
    pub fn withdraw_collateral(
        &mut self,
        amount: u128,
        collateral_price: u128,
    ) -> Result<()> {
        if self.status != VaultStatus::Active {
            return Err(StablecoinError::VaultNotFound("Vault not active".to_string()));
        }

        if amount > self.collateral_amount {
            return Err(StablecoinError::InsufficientCollateral {
                required: amount,
                provided: self.collateral_amount,
            });
        }

        // Check if vault will remain safe after withdrawal
        let new_collateral = self.collateral_amount - amount;
        let new_value = new_collateral
            .checked_mul(collateral_price)
            .ok_or(StablecoinError::ArithmeticOverflow)?;

        let total_debt = self.total_debt()?;
        let new_ratio = new_value
            .checked_mul(10000)
            .and_then(|v| v.checked_div(total_debt))
            .ok_or(StablecoinError::ArithmeticOverflow)?;

        if new_ratio < self.min_collateral_ratio as u128 {
            return Err(StablecoinError::Undercollateralized);
        }

        self.collateral_amount = new_collateral;
        Ok(())
    }

    /// Mint more stablecoins (increase debt)
    pub fn mint(&mut self, amount: u128, collateral_price: u128) -> Result<()> {
        if self.status != VaultStatus::Active {
            return Err(StablecoinError::VaultNotFound("Vault not active".to_string()));
        }

        let new_debt = self
            .debt_amount
            .checked_add(amount)
            .ok_or(StablecoinError::ArithmeticOverflow)?;

        // Check if vault will remain safe
        let collateral_value = self
            .collateral_amount
            .checked_mul(collateral_price)
            .ok_or(StablecoinError::ArithmeticOverflow)?;

        let new_total_debt = new_debt
            .checked_add(self.accumulated_fee)
            .ok_or(StablecoinError::ArithmeticOverflow)?;

        let new_ratio = collateral_value
            .checked_mul(10000)
            .and_then(|v| v.checked_div(new_total_debt))
            .ok_or(StablecoinError::ArithmeticOverflow)?;

        if new_ratio < self.min_collateral_ratio as u128 {
            return Err(StablecoinError::Undercollateralized);
        }

        self.debt_amount = new_debt;
        Ok(())
    }

    /// Repay debt (burn stablecoins)
    pub fn repay(&mut self, amount: u128) -> Result<u128> {
        if self.status != VaultStatus::Active {
            return Err(StablecoinError::VaultNotFound("Vault not active".to_string()));
        }

        let total_debt = self.total_debt()?;
        let actual_repay = amount.min(total_debt);

        // First pay off fees
        if actual_repay <= self.accumulated_fee {
            self.accumulated_fee -= actual_repay;
        } else {
            let remaining = actual_repay - self.accumulated_fee;
            self.accumulated_fee = 0;
            self.debt_amount -= remaining;
        }

        // Close vault if fully repaid
        if self.debt_amount == 0 && self.accumulated_fee == 0 {
            self.status = VaultStatus::Closed;
        }

        Ok(actual_repay)
    }

    /// Accrue stability fee
    pub fn accrue_fee(&mut self, fee_rate_per_second: u64, current_timestamp: i64) -> Result<u128> {
        if self.status != VaultStatus::Active || self.debt_amount == 0 {
            return Ok(0);
        }

        let elapsed = current_timestamp
            .checked_sub(self.last_fee_accrual)
            .ok_or(StablecoinError::ArithmeticOverflow)?;

        if elapsed <= 0 {
            return Ok(0);
        }

        // Fee = debt * rate * time
        let fee = self
            .debt_amount
            .checked_mul(fee_rate_per_second as u128)
            .and_then(|v| v.checked_mul(elapsed as u128))
            .and_then(|v| v.checked_div(10000))
            .ok_or(StablecoinError::ArithmeticOverflow)?;

        self.accumulated_fee = self
            .accumulated_fee
            .checked_add(fee)
            .ok_or(StablecoinError::ArithmeticOverflow)?;

        self.last_fee_accrual = current_timestamp;
        Ok(fee)
    }

    /// Mark vault as liquidating
    pub fn start_liquidation(&mut self) -> Result<()> {
        if self.status != VaultStatus::Active {
            return Err(StablecoinError::VaultNotFound("Vault not active".to_string()));
        }
        self.status = VaultStatus::Liquidating;
        Ok(())
    }

    /// Complete liquidation
    pub fn complete_liquidation(&mut self) -> Result<()> {
        if self.status != VaultStatus::Liquidating {
            return Err(StablecoinError::VaultNotFound("Vault not liquidating".to_string()));
        }
        self.status = VaultStatus::Liquidated;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_address() -> Address {
        Address::from([1u8; 20])
    }

    fn test_vault() -> Vault {
        Vault::new(
            VaultId([0u8; 32]),
            test_address(),
            test_address(),
            1500,  // collateral
            1000,  // debt
            15000, // 150% min ratio
            0,
        ).unwrap()
    }

    #[test]
    fn test_collateralization_ratio() {
        let vault = test_vault();
        // 1500 collateral * 1 price / 1000 debt * 10000 = 15000 (150%)
        let ratio = vault.collateralization_ratio(1).unwrap();
        assert_eq!(ratio, 15000);
    }

    #[test]
    fn test_is_safe() {
        let vault = test_vault();
        assert!(vault.is_safe(1).unwrap());
        
        // Below minimum ratio
        assert!(!vault.is_safe(1).unwrap() || vault.collateralization_ratio(1).unwrap() < 15000);
    }

    #[test]
    fn test_add_collateral() {
        let mut vault = test_vault();
        vault.add_collateral(500).unwrap();
        assert_eq!(vault.collateral_amount, 2000);
    }

    #[test]
    fn test_mint() {
        let mut vault = test_vault();
        // Can mint more if stays above 150%
        // Current: 1500/1000 = 150%
        // After minting 100: 1500/1100 = 136% (below 150%)
        let result = vault.mint(100, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_repay() {
        let mut vault = test_vault();
        vault.accumulated_fee = 50;
        
        let repaid = vault.repay(30).unwrap();
        assert_eq!(repaid, 30);
        assert_eq!(vault.accumulated_fee, 20);
    }

    #[test]
    fn test_accrue_fee() {
        let mut vault = test_vault();
        // 1% per second for 100 seconds
        let fee = vault.accrue_fee(100, 100).unwrap();
        // 1000 * 100 * 100 / 10000 = 100000
        assert_eq!(fee, 100000);
    }
}
