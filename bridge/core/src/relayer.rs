//! Bridge relayer management

use crate::{BridgeError, Result};
use lean_crypto::Address;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Relayer information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relayer {
    pub address: Address,
    pub stake: u128,
    pub is_active: bool,
    pub transfers_relayed: u64,
}

/// Relayer set
pub struct RelayerSet {
    relayers: HashMap<Address, Relayer>,
    min_stake: u128,
}

impl RelayerSet {
    pub fn new(min_stake: u128) -> Self {
        Self {
            relayers: HashMap::new(),
            min_stake,
        }
    }

    /// Add relayer
    pub fn add_relayer(&mut self, relayer: Relayer) -> Result<()> {
        if relayer.stake < self.min_stake {
            return Err(BridgeError::UnauthorizedRelayer(
                "Insufficient stake".to_string(),
            ));
        }
        self.relayers.insert(relayer.address, relayer);
        Ok(())
    }

    /// Check if address is authorized relayer
    pub fn is_authorized(&self, address: &Address) -> bool {
        self.relayers
            .get(address)
            .map(|r| r.is_active && r.stake >= self.min_stake)
            .unwrap_or(false)
    }

    /// Increment relayer transfer count
    pub fn record_transfer(&mut self, address: &Address) {
        if let Some(relayer) = self.relayers.get_mut(address) {
            relayer.transfers_relayed += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relayer_set() {
        let mut set = RelayerSet::new(1000);
        
        let relayer = Relayer {
            address: Address::from([1u8; 20]),
            stake: 5000,
            is_active: true,
            transfers_relayed: 0,
        };

        set.add_relayer(relayer).unwrap();
        assert!(set.is_authorized(&Address::from([1u8; 20])));
    }
}
