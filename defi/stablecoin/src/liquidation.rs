//! Liquidation engine and auctions

use crate::{Vault, VaultId, StabilityPool, StablecoinError, Result};
use lean_crypto::Address;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Liquidation auction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidationAuction {
    pub vault_id: VaultId,
    pub collateral_type: Address,
    pub collateral_amount: u128,
    pub debt_amount: u128,
    pub start_time: i64,
    pub end_time: i64,
    pub highest_bid: u128,
    pub highest_bidder: Option<Address>,
}

/// Liquidation engine
pub struct LiquidationEngine {
    /// Active auctions
    auctions: HashMap<VaultId, LiquidationAuction>,
    /// Liquidation penalty (basis points)
    pub liquidation_penalty: u16,
}

impl LiquidationEngine {
    pub fn new(liquidation_penalty: u16) -> Self {
        Self {
            auctions: HashMap::new(),
            liquidation_penalty,
        }
    }

    /// Liquidate a vault
    pub fn liquidate(
        &mut self,
        vault: &mut Vault,
        stability_pool: &mut StabilityPool,
        collateral_price: u128,
        timestamp: i64,
    ) -> Result<()> {
        if vault.is_safe(collateral_price)? {
            return Err(StablecoinError::VaultSafe);
        }

        vault.start_liquidation()?;
        let total_debt = vault.total_debt()?;

        // Try stability pool first
        if stability_pool.total_deposited >= total_debt {
            stability_pool.offset_debt(
                total_debt,
                vault.collateral_type,
                vault.collateral_amount,
            )?;
            vault.complete_liquidation()?;
            return Ok(());
        }

        // Otherwise start auction
        let auction = LiquidationAuction {
            vault_id: vault.id,
            collateral_type: vault.collateral_type,
            collateral_amount: vault.collateral_amount,
            debt_amount: total_debt,
            start_time: timestamp,
            end_time: timestamp + 86400, // 24 hours
            highest_bid: 0,
            highest_bidder: None,
        };

        self.auctions.insert(vault.id, auction);
        Ok(())
    }

    /// Place bid on auction
    pub fn bid(&mut self, vault_id: &VaultId, bidder: Address, amount: u128) -> Result<()> {
        let auction = self.auctions.get_mut(vault_id)
            .ok_or_else(|| StablecoinError::VaultNotFound(hex::encode(&vault_id.0)))?;

        if amount <= auction.highest_bid {
            return Err(StablecoinError::VaultNotFound("Bid too low".to_string()));
        }

        auction.highest_bid = amount;
        auction.highest_bidder = Some(bidder);
        Ok(())
    }

    /// Finalize auction
    pub fn finalize_auction(&mut self, vault_id: &VaultId, vault: &mut Vault) -> Result<()> {
        let auction = self.auctions.remove(vault_id)
            .ok_or_else(|| StablecoinError::VaultNotFound(hex::encode(&vault_id.0)))?;

        vault.complete_liquidation()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_liquidation_engine() {
        let engine = LiquidationEngine::new(500);
        assert_eq!(engine.liquidation_penalty, 500);
    }
}
