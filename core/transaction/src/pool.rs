//! Transaction pool (mempool) for pending transactions

use crate::{Transaction, TransactionError, Result};
use lean_crypto::Hash256;
use std::collections::{HashMap, BTreeMap};

/// Maximum number of transactions in the pool
const MAX_POOL_SIZE: usize = 10_000;

/// Transaction pool for managing pending transactions
pub struct TransactionPool {
    /// Transactions by hash
    transactions: HashMap<Hash256, Transaction>,

    /// Transactions ordered by gas price (for selection)
    by_gas_price: BTreeMap<u64, Vec<Hash256>>,

    /// Transactions by sender nonce (for ordering)
    by_sender: HashMap<lean_crypto::Address, BTreeMap<u64, Hash256>>,

    /// Maximum pool size
    max_size: usize,
}

impl TransactionPool {
    /// Create a new transaction pool
    pub fn new() -> Self {
        Self {
            transactions: HashMap::new(),
            by_gas_price: BTreeMap::new(),
            by_sender: HashMap::new(),
            max_size: MAX_POOL_SIZE,
        }
    }

    /// Create a pool with custom max size
    pub fn with_capacity(max_size: usize) -> Self {
        Self {
            transactions: HashMap::new(),
            by_gas_price: BTreeMap::new(),
            by_sender: HashMap::new(),
            max_size,
        }
    }

    /// Add a transaction to the pool
    pub fn add(&mut self, mut tx: Transaction) -> Result<()> {
        // Check if pool is full
        if self.transactions.len() >= self.max_size {
            // Try to evict lowest gas price transaction
            self.evict_lowest_gas_price()?;
        }

        let tx_hash = tx.hash();

        // Check if transaction already exists
        if self.transactions.contains_key(&tx_hash) {
            return Err(TransactionError::ExecutionFailed(
                "Transaction already in pool".to_string(),
            ));
        }

        // Check for nonce conflicts
        if let Some(sender_txs) = self.by_sender.get(&tx.from) {
            if sender_txs.contains_key(&tx.nonce) {
                return Err(TransactionError::InvalidNonce {
                    expected: tx.nonce + 1,
                    actual: tx.nonce,
                });
            }
        }

        // Add to gas price index
        self.by_gas_price
            .entry(tx.gas_price)
            .or_insert_with(Vec::new)
            .push(tx_hash);

        // Add to sender index
        self.by_sender
            .entry(tx.from)
            .or_insert_with(BTreeMap::new)
            .insert(tx.nonce, tx_hash);

        // Add to main storage
        self.transactions.insert(tx_hash, tx);

        Ok(())
    }

    /// Remove a transaction from the pool
    pub fn remove(&mut self, tx_hash: &Hash256) -> Option<Transaction> {
        if let Some(tx) = self.transactions.remove(tx_hash) {
            // Remove from gas price index
            if let Some(txs) = self.by_gas_price.get_mut(&tx.gas_price) {
                txs.retain(|h| h != tx_hash);
                if txs.is_empty() {
                    self.by_gas_price.remove(&tx.gas_price);
                }
            }

            // Remove from sender index
            if let Some(sender_txs) = self.by_sender.get_mut(&tx.from) {
                sender_txs.remove(&tx.nonce);
                if sender_txs.is_empty() {
                    self.by_sender.remove(&tx.from);
                }
            }

            Some(tx)
        } else {
            None
        }
    }

    /// Get a transaction by hash
    pub fn get(&self, tx_hash: &Hash256) -> Option<&Transaction> {
        self.transactions.get(tx_hash)
    }

    /// Get all transactions for a sender
    pub fn get_sender_transactions(&self, sender: &lean_crypto::Address) -> Vec<&Transaction> {
        if let Some(sender_txs) = self.by_sender.get(sender) {
            sender_txs
                .values()
                .filter_map(|hash| self.transactions.get(hash))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get pending transactions ordered by gas price (highest first)
    pub fn get_pending(&self, limit: usize) -> Vec<&Transaction> {
        let mut result = Vec::new();

        for (_gas_price, tx_hashes) in self.by_gas_price.iter().rev() {
            for hash in tx_hashes {
                if let Some(tx) = self.transactions.get(hash) {
                    result.push(tx);
                    if result.len() >= limit {
                        return result;
                    }
                }
            }
        }

        result
    }

    /// Get pending transactions for a block (ordered by nonce for each sender)
    pub fn get_ready_transactions(&self, limit: usize) -> Vec<&Transaction> {
        let mut result = Vec::new();
        let mut added = std::collections::HashSet::new();

        // Iterate through gas prices from highest to lowest
        for (_gas_price, tx_hashes) in self.by_gas_price.iter().rev() {
            for hash in tx_hashes {
                if added.contains(hash) {
                    continue;
                }

                if let Some(tx) = self.transactions.get(hash) {
                    // Check if this transaction is ready (previous nonces are satisfied)
                    if self.is_transaction_ready(tx) {
                        result.push(tx);
                        added.insert(*hash);

                        if result.len() >= limit {
                            return result;
                        }
                    }
                }
            }
        }

        result
    }

    /// Check if a transaction is ready to be included (no missing nonces)
    fn is_transaction_ready(&self, tx: &Transaction) -> bool {
        if tx.nonce == 0 {
            return true;
        }

        // Check if all previous nonces exist in the pool
        if let Some(sender_txs) = self.by_sender.get(&tx.from) {
            for nonce in 0..tx.nonce {
                if !sender_txs.contains_key(&nonce) {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }

    /// Evict the lowest gas price transaction
    fn evict_lowest_gas_price(&mut self) -> Result<()> {
        if let Some((&lowest_gas_price, _)) = self.by_gas_price.iter().next() {
            if let Some(tx_hashes) = self.by_gas_price.get(&lowest_gas_price) {
                if let Some(&tx_hash) = tx_hashes.first() {
                    self.remove(&tx_hash);
                    return Ok(());
                }
            }
        }

        Err(TransactionError::ExecutionFailed(
            "Cannot evict transaction".to_string(),
        ))
    }

    /// Get pool size
    pub fn len(&self) -> usize {
        self.transactions.len()
    }

    /// Check if pool is empty
    pub fn is_empty(&self) -> bool {
        self.transactions.is_empty()
    }

    /// Clear all transactions
    pub fn clear(&mut self) {
        self.transactions.clear();
        self.by_gas_price.clear();
        self.by_sender.clear();
    }

    /// Get pool statistics
    pub fn stats(&self) -> PoolStats {
        PoolStats {
            total_transactions: self.transactions.len(),
            unique_senders: self.by_sender.len(),
            max_size: self.max_size,
        }
    }
}

impl Default for TransactionPool {
    fn default() -> Self {
        Self::new()
    }
}

/// Transaction pool statistics
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub total_transactions: usize,
    pub unique_senders: usize,
    pub max_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PaymentTransaction, TransactionType};
    use lean_crypto::{Address, KeyPair};

    #[test]
    fn test_pool_add_remove() {
        let mut pool = TransactionPool::new();
        let keypair = KeyPair::generate();
        let from = Address::from_public_key(keypair.public_key());
        let to = Address::from_public_key(&KeyPair::generate().public_key());

        let payment = PaymentTransaction {
            to,
            amount: 1_000_000,
        };

        let mut tx = Transaction::new(
            0,
            from,
            TransactionType::Payment(payment),
            21_000,
            10,
        );

        tx.sign(&keypair);
        let tx_hash = tx.hash();

        pool.add(tx).unwrap();
        assert_eq!(pool.len(), 1);

        let removed = pool.remove(&tx_hash);
        assert!(removed.is_some());
        assert_eq!(pool.len(), 0);
    }

    #[test]
    fn test_pool_gas_price_ordering() {
        let mut pool = TransactionPool::new();
        let keypair = KeyPair::generate();
        let from = Address::from_public_key(keypair.public_key());
        let to = Address::from_public_key(&KeyPair::generate().public_key());

        // Add transactions with different gas prices
        for i in 0..5 {
            let payment = PaymentTransaction {
                to,
                amount: 1_000_000,
            };

            let mut tx = Transaction::new(
                i,
                from,
                TransactionType::Payment(payment),
                21_000,
                (i as u64 + 1) * 10, // Different gas prices
            );

            tx.sign(&keypair);
            pool.add(tx).unwrap();
        }

        // Get pending transactions (should be ordered by gas price, highest first)
        let pending = pool.get_pending(5);
        assert_eq!(pending.len(), 5);

        // Verify ordering (highest gas price first)
        for i in 0..pending.len() - 1 {
            assert!(pending[i].gas_price >= pending[i + 1].gas_price);
        }
    }

    #[test]
    fn test_pool_nonce_ordering() {
        let mut pool = TransactionPool::new();
        let keypair = KeyPair::generate();
        let from = Address::from_public_key(keypair.public_key());
        let to = Address::from_public_key(&KeyPair::generate().public_key());

        // Add transactions with sequential nonces
        for i in 0..3 {
            let payment = PaymentTransaction {
                to,
                amount: 1_000_000,
            };

            let mut tx = Transaction::new(
                i,
                from,
                TransactionType::Payment(payment),
                21_000,
                10,
            );

            tx.sign(&keypair);
            pool.add(tx).unwrap();
        }

        let sender_txs = pool.get_sender_transactions(&from);
        assert_eq!(sender_txs.len(), 3);
    }
}
