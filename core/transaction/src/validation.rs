//! Transaction validation logic

use crate::{Transaction, TransactionError, Result};

/// Maximum transaction size in bytes
pub const MAX_TRANSACTION_SIZE: usize = 1_048_576; // 1 MB

/// Minimum gas limit
pub const MIN_GAS_LIMIT: u64 = 21_000;

/// Minimum gas price
pub const MIN_GAS_PRICE: u64 = 1;

/// Minimum stake amount (100,000 LEAN tokens)
pub const MIN_STAKE_AMOUNT: u128 = 100_000_000_000_000; // Assuming 6 decimals

/// Transaction validator
pub struct TransactionValidator {
    /// Chain ID (for replay protection)
    chain_id: u64,
}

impl TransactionValidator {
    /// Create a new transaction validator
    pub fn new(chain_id: u64) -> Self {
        Self { chain_id }
    }

    /// Validate a transaction (basic checks)
    pub fn validate(&self, tx: &Transaction) -> Result<()> {
        // Check transaction size
        self.validate_size(tx)?;

        // Check gas limit
        self.validate_gas_limit(tx)?;

        // Check gas price
        self.validate_gas_price(tx)?;

        // Check signature
        self.validate_signature(tx)?;

        // Type-specific validation
        self.validate_type_specific(tx)?;

        Ok(())
    }

    /// Validate transaction size
    fn validate_size(&self, tx: &Transaction) -> Result<()> {
        let size = tx.size();
        if size > MAX_TRANSACTION_SIZE {
            return Err(TransactionError::TransactionTooLarge {
                size,
                max: MAX_TRANSACTION_SIZE,
            });
        }
        Ok(())
    }

    /// Validate gas limit
    fn validate_gas_limit(&self, tx: &Transaction) -> Result<()> {
        if tx.gas_limit < MIN_GAS_LIMIT {
            return Err(TransactionError::GasLimitTooLow {
                minimum: MIN_GAS_LIMIT,
                provided: tx.gas_limit,
            });
        }
        Ok(())
    }

    /// Validate gas price
    fn validate_gas_price(&self, tx: &Transaction) -> Result<()> {
        if tx.gas_price < MIN_GAS_PRICE {
            return Err(TransactionError::GasPriceTooLow {
                minimum: MIN_GAS_PRICE,
                provided: tx.gas_price,
            });
        }
        Ok(())
    }

    /// Validate signature (simplified for now)
    fn validate_signature(&self, _tx: &Transaction) -> Result<()> {
        // In a real implementation, we would:
        // 1. Recover the public key from the signature
        // 2. Derive the address from the public key
        // 3. Compare with tx.from
        // For now, we'll skip this as it requires proper ECDSA recovery
        Ok(())
    }

    /// Validate type-specific rules
    fn validate_type_specific(&self, tx: &Transaction) -> Result<()> {
        use crate::TransactionType;

        match &tx.tx_type {
            TransactionType::Payment(payment) => {
                if payment.amount == 0 {
                    return Err(TransactionError::ExecutionFailed(
                        "Payment amount must be greater than 0".to_string(),
                    ));
                }
            }
            TransactionType::Stake(stake) => {
                if stake.amount < MIN_STAKE_AMOUNT {
                    return Err(TransactionError::ExecutionFailed(
                        format!("Stake amount must be at least {}", MIN_STAKE_AMOUNT),
                    ));
                }
                if stake.commission_rate > 100 {
                    return Err(TransactionError::ExecutionFailed(
                        "Commission rate must be between 0 and 100".to_string(),
                    ));
                }
            }
            TransactionType::ContractDeploy(deploy) => {
                if deploy.code.is_empty() {
                    return Err(TransactionError::DeploymentFailed(
                        "Contract code cannot be empty".to_string(),
                    ));
                }
            }
            _ => {}
        }

        Ok(())
    }

    /// Validate nonce against account state
    pub fn validate_nonce(&self, tx: &Transaction, expected_nonce: u64) -> Result<()> {
        if tx.nonce != expected_nonce {
            return Err(TransactionError::InvalidNonce {
                expected: expected_nonce,
                actual: tx.nonce,
            });
        }
        Ok(())
    }

    /// Validate balance for transaction
    pub fn validate_balance(&self, tx: &Transaction, account_balance: u128) -> Result<()> {
        let total_cost = tx.cost() + tx.value();
        if account_balance < total_cost {
            return Err(TransactionError::InsufficientBalance {
                have: account_balance,
                need: total_cost,
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PaymentTransaction, TransactionType};
    use lean_crypto::{Address, KeyPair};

    #[test]
    fn test_valid_transaction() {
        let validator = TransactionValidator::new(1);
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
            1,
        );

        tx.sign(&keypair);

        // Basic validation should pass
        assert!(validator.validate_size(&tx).is_ok());
        assert!(validator.validate_gas_limit(&tx).is_ok());
        assert!(validator.validate_gas_price(&tx).is_ok());
    }

    #[test]
    fn test_low_gas_limit() {
        let validator = TransactionValidator::new(1);
        let keypair = KeyPair::generate();
        let from = Address::from_public_key(keypair.public_key());
        let to = Address::from_public_key(&KeyPair::generate().public_key());

        let payment = PaymentTransaction {
            to,
            amount: 1_000_000,
        };

        let tx = Transaction::new(
            0,
            from,
            TransactionType::Payment(payment),
            1_000, // Too low
            1,
        );

        assert!(validator.validate_gas_limit(&tx).is_err());
    }

    #[test]
    fn test_invalid_nonce() {
        let validator = TransactionValidator::new(1);
        let keypair = KeyPair::generate();
        let from = Address::from_public_key(keypair.public_key());
        let to = Address::from_public_key(&KeyPair::generate().public_key());

        let payment = PaymentTransaction {
            to,
            amount: 1_000_000,
        };

        let tx = Transaction::new(
            5, // Wrong nonce
            from,
            TransactionType::Payment(payment),
            21_000,
            1,
        );

        assert!(validator.validate_nonce(&tx, 0).is_err());
    }

    #[test]
    fn test_insufficient_balance() {
        let validator = TransactionValidator::new(1);
        let keypair = KeyPair::generate();
        let from = Address::from_public_key(keypair.public_key());
        let to = Address::from_public_key(&KeyPair::generate().public_key());

        let payment = PaymentTransaction {
            to,
            amount: 1_000_000,
        };

        let tx = Transaction::new(
            0,
            from,
            TransactionType::Payment(payment),
            21_000,
            1,
        );

        assert!(validator.validate_balance(&tx, 100_000).is_err());
    }
}
