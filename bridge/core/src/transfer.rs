//! Bridge transfer management

use crate::{BridgeError, Result, ChainType};
use lean_crypto::Address;
use serde::{Deserialize, Serialize};

/// Transfer ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TransferId(pub [u8; 32]);

/// Transfer status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransferStatus {
    /// Transfer initiated
    Pending,
    /// Sufficient signatures collected
    Confirmed,
    /// Transfer executed on destination
    Completed,
    /// Transfer failed or rejected
    Failed,
}

/// Bridge transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeTransfer {
    /// Unique transfer ID
    pub id: TransferId,
    /// Source chain
    pub source_chain: ChainType,
    /// Destination chain
    pub destination_chain: ChainType,
    /// Source address
    pub source_address: Address,
    /// Destination address (encoded as bytes)
    pub destination_address: Vec<u8>,
    /// Asset being transferred
    pub asset: Address,
    /// Amount
    pub amount: u128,
    /// Fee paid for bridging
    pub fee: u128,
    /// Timestamp of initiation
    pub timestamp: i64,
    /// Status
    pub status: TransferStatus,
    /// Validator signatures
    pub signatures: Vec<Vec<u8>>,
    /// Transaction hash on source chain
    pub source_tx_hash: Option<Vec<u8>>,
    /// Transaction hash on destination chain
    pub destination_tx_hash: Option<Vec<u8>>,
}

impl BridgeTransfer {
    /// Create new bridge transfer
    pub fn new(
        id: TransferId,
        source_chain: ChainType,
        destination_chain: ChainType,
        source_address: Address,
        destination_address: Vec<u8>,
        asset: Address,
        amount: u128,
        fee: u128,
        timestamp: i64,
    ) -> Result<Self> {
        if amount == 0 {
            return Err(BridgeError::AmountBelowMinimum {
                amount: 0,
                minimum: 1,
            });
        }

        Ok(Self {
            id,
            source_chain,
            destination_chain,
            source_address,
            destination_address,
            asset,
            amount,
            fee,
            timestamp,
            status: TransferStatus::Pending,
            signatures: Vec::new(),
            source_tx_hash: None,
            destination_tx_hash: None,
        })
    }

    /// Add validator signature
    pub fn add_signature(&mut self, signature: Vec<u8>) {
        self.signatures.push(signature);
    }

    /// Check if transfer has enough signatures
    pub fn has_sufficient_signatures(&self, threshold: usize) -> bool {
        self.signatures.len() >= threshold
    }

    /// Confirm transfer (sufficient signatures)
    pub fn confirm(&mut self) -> Result<()> {
        if self.status != TransferStatus::Pending {
            return Err(BridgeError::TransferAlreadyProcessed(
                hex::encode(&self.id.0),
            ));
        }
        self.status = TransferStatus::Confirmed;
        Ok(())
    }

    /// Complete transfer
    pub fn complete(&mut self, destination_tx_hash: Vec<u8>) -> Result<()> {
        if self.status != TransferStatus::Confirmed {
            return Err(BridgeError::TransferAlreadyProcessed(
                hex::encode(&self.id.0),
            ));
        }
        self.destination_tx_hash = Some(destination_tx_hash);
        self.status = TransferStatus::Completed;
        Ok(())
    }

    /// Fail transfer
    pub fn fail(&mut self) -> Result<()> {
        self.status = TransferStatus::Failed;
        Ok(())
    }

    /// Calculate net amount (after fees)
    pub fn net_amount(&self) -> Result<u128> {
        self.amount
            .checked_sub(self.fee)
            .ok_or(BridgeError::ArithmeticOverflow)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_transfer() -> BridgeTransfer {
        BridgeTransfer::new(
            TransferId([0u8; 32]),
            ChainType::Ethereum,
            ChainType::Cosmos,
            Address::from([1u8; 20]),
            vec![2u8; 32],
            Address::from([3u8; 20]),
            1000,
            10,
            0,
        ).unwrap()
    }

    #[test]
    fn test_create_transfer() {
        let transfer = test_transfer();
        assert_eq!(transfer.status, TransferStatus::Pending);
        assert_eq!(transfer.amount, 1000);
        assert_eq!(transfer.fee, 10);
    }

    #[test]
    fn test_add_signatures() {
        let mut transfer = test_transfer();
        transfer.add_signature(vec![1, 2, 3]);
        transfer.add_signature(vec![4, 5, 6]);
        assert_eq!(transfer.signatures.len(), 2);
        assert!(transfer.has_sufficient_signatures(2));
    }

    #[test]
    fn test_confirm_transfer() {
        let mut transfer = test_transfer();
        transfer.confirm().unwrap();
        assert_eq!(transfer.status, TransferStatus::Confirmed);
    }

    #[test]
    fn test_complete_transfer() {
        let mut transfer = test_transfer();
        transfer.confirm().unwrap();
        transfer.complete(vec![1, 2, 3]).unwrap();
        assert_eq!(transfer.status, TransferStatus::Completed);
    }

    #[test]
    fn test_net_amount() {
        let transfer = test_transfer();
        assert_eq!(transfer.net_amount().unwrap(), 990);
    }
}
