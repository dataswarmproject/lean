//! Bridge state management

use crate::{BridgeTransfer, TransferId, ChainType, BridgeError, Result};
use lean_crypto::Address;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Chain state tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainState {
    pub chain_type: ChainType,
    pub latest_block: u64,
    pub total_locked: u128,
    pub total_minted: u128,
}

impl ChainState {
    pub fn new(chain_type: ChainType) -> Self {
        Self {
            chain_type,
            latest_block: 0,
            total_locked: 0,
            total_minted: 0,
        }
    }

    /// Lock assets on source chain
    pub fn lock(&mut self, amount: u128) -> Result<()> {
        self.total_locked = self
            .total_locked
            .checked_add(amount)
            .ok_or(BridgeError::ArithmeticOverflow)?;
        Ok(())
    }

    /// Unlock assets on source chain
    pub fn unlock(&mut self, amount: u128) -> Result<()> {
        self.total_locked = self
            .total_locked
            .checked_sub(amount)
            .ok_or(BridgeError::ArithmeticOverflow)?;
        Ok(())
    }

    /// Mint wrapped assets on destination chain
    pub fn mint(&mut self, amount: u128) -> Result<()> {
        self.total_minted = self
            .total_minted
            .checked_add(amount)
            .ok_or(BridgeError::ArithmeticOverflow)?;
        Ok(())
    }

    /// Burn wrapped assets on destination chain
    pub fn burn(&mut self, amount: u128) -> Result<()> {
        self.total_minted = self
            .total_minted
            .checked_sub(amount)
            .ok_or(BridgeError::ArithmeticOverflow)?;
        Ok(())
    }
}

/// Bridge state manager
pub struct BridgeState {
    /// Active transfers
    transfers: HashMap<TransferId, BridgeTransfer>,
    /// Chain states
    chains: HashMap<ChainType, ChainState>,
    /// Liquidity pools per asset
    liquidity: HashMap<Address, u128>,
}

impl BridgeState {
    pub fn new() -> Self {
        Self {
            transfers: HashMap::new(),
            chains: HashMap::new(),
            liquidity: HashMap::new(),
        }
    }

    /// Add supported chain
    pub fn add_chain(&mut self, chain_type: ChainType) {
        self.chains.insert(chain_type, ChainState::new(chain_type));
    }

    /// Get chain state
    pub fn get_chain(&self, chain_type: ChainType) -> Result<&ChainState> {
        self.chains
            .get(&chain_type)
            .ok_or_else(|| BridgeError::ChainNotSupported(format!("{:?}", chain_type)))
    }

    /// Get mutable chain state
    pub fn get_chain_mut(&mut self, chain_type: ChainType) -> Result<&mut ChainState> {
        self.chains
            .get_mut(&chain_type)
            .ok_or_else(|| BridgeError::ChainNotSupported(format!("{:?}", chain_type)))
    }

    /// Add transfer
    pub fn add_transfer(&mut self, transfer: BridgeTransfer) -> Result<()> {
        let id = transfer.id;
        if self.transfers.contains_key(&id) {
            return Err(BridgeError::TransferAlreadyProcessed(hex::encode(&id.0)));
        }
        self.transfers.insert(id, transfer);
        Ok(())
    }

    /// Get transfer
    pub fn get_transfer(&self, id: &TransferId) -> Result<&BridgeTransfer> {
        self.transfers
            .get(id)
            .ok_or_else(|| BridgeError::TransferNotFound(hex::encode(&id.0)))
    }

    /// Get mutable transfer
    pub fn get_transfer_mut(&mut self, id: &TransferId) -> Result<&mut BridgeTransfer> {
        self.transfers
            .get_mut(id)
            .ok_or_else(|| BridgeError::TransferNotFound(hex::encode(&id.0)))
    }

    /// Add liquidity
    pub fn add_liquidity(&mut self, asset: Address, amount: u128) -> Result<()> {
        let liquidity = self.liquidity.entry(asset).or_insert(0);
        *liquidity = liquidity
            .checked_add(amount)
            .ok_or(BridgeError::ArithmeticOverflow)?;
        Ok(())
    }

    /// Remove liquidity
    pub fn remove_liquidity(&mut self, asset: &Address, amount: u128) -> Result<()> {
        let liquidity = self
            .liquidity
            .get_mut(asset)
            .ok_or(BridgeError::InsufficientLiquidity)?;
        
        if *liquidity < amount {
            return Err(BridgeError::InsufficientLiquidity);
        }

        *liquidity -= amount;
        Ok(())
    }

    /// Get liquidity for asset
    pub fn get_liquidity(&self, asset: &Address) -> u128 {
        self.liquidity.get(asset).copied().unwrap_or(0)
    }
}

impl Default for BridgeState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_state() {
        let mut state = ChainState::new(ChainType::Ethereum);
        state.lock(1000).unwrap();
        assert_eq!(state.total_locked, 1000);
        
        state.mint(1000).unwrap();
        assert_eq!(state.total_minted, 1000);
    }

    #[test]
    fn test_bridge_state() {
        let mut state = BridgeState::new();
        state.add_chain(ChainType::Ethereum);
        
        let chain = state.get_chain(ChainType::Ethereum).unwrap();
        assert_eq!(chain.total_locked, 0);
    }

    #[test]
    fn test_liquidity() {
        let mut state = BridgeState::new();
        let asset = Address::from([1u8; 20]);
        
        state.add_liquidity(asset, 5000).unwrap();
        assert_eq!(state.get_liquidity(&asset), 5000);
        
        state.remove_liquidity(&asset, 2000).unwrap();
        assert_eq!(state.get_liquidity(&asset), 3000);
    }
}
