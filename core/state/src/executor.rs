//! Transaction execution engine

use crate::{State, StateError};
use lean_crypto::Address;
use lean_transaction::{Transaction, TransactionType, TransactionError};

/// Execution result
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    /// Gas used
    pub gas_used: u64,
    
    /// Success status
    pub success: bool,
    
    /// Output data
    pub output: Vec<u8>,
    
    /// Logs emitted
    pub logs: Vec<ExecutionLog>,
    
    /// Error message (if failed)
    pub error: Option<String>,
}

/// Execution log
#[derive(Debug, Clone)]
pub struct ExecutionLog {
    /// Contract address
    pub address: Address,
    
    /// Log data
    pub data: Vec<u8>,
}

/// State executor for transactions
pub struct StateExecutor {
    /// State instance
    state: State,
}

impl StateExecutor {
    /// Create a new state executor
    pub fn new(state: State) -> Self {
        Self { state }
    }
    
    /// Execute a transaction
    pub fn execute(&self, tx: &Transaction) -> Result<ExecutionResult, TransactionError> {
        // Verify nonce
        let current_nonce = self.state.get_nonce(&tx.from)
            .map_err(|_| TransactionError::ExecutionFailed("Failed to get nonce".to_string()))?;
        
        if tx.nonce != current_nonce {
            return Err(TransactionError::InvalidNonce {
                expected: current_nonce,
                actual: tx.nonce,
            });
        }
        
        // Check balance for transaction cost
        let total_cost = tx.cost() + tx.value();
        let balance = self.state.get_balance(&tx.from)
            .map_err(|_| TransactionError::ExecutionFailed("Failed to get balance".to_string()))?;
        
        if balance < total_cost {
            return Err(TransactionError::InsufficientBalance {
                have: balance,
                need: total_cost,
            });
        }
        
        // Execute based on transaction type
        let result = match &tx.tx_type {
            TransactionType::Payment(payment) => {
                self.execute_payment(tx, payment)?
            }
            TransactionType::ContractDeploy(deploy) => {
                self.execute_contract_deploy(tx, deploy)?
            }
            TransactionType::ContractCall(call) => {
                self.execute_contract_call(tx, call)?
            }
            TransactionType::Stake(stake) => {
                self.execute_stake(tx, stake)?
            }
            TransactionType::Unstake(unstake) => {
                self.execute_unstake(tx, unstake)?
            }
            TransactionType::Delegate(delegate) => {
                self.execute_delegate(tx, delegate)?
            }
            _ => {
                // For other types, return a basic success result
                ExecutionResult {
                    gas_used: tx.gas_limit,
                    success: true,
                    output: Vec::new(),
                    logs: Vec::new(),
                    error: None,
                }
            }
        };
        
        // Deduct gas cost
        let gas_cost = result.gas_used as u128 * tx.gas_price as u128;
        self.state.sub_balance(tx.from, gas_cost)
            .map_err(|e| TransactionError::ExecutionFailed(e.to_string()))?;
        
        // Increment nonce if successful
        if result.success {
            self.state.increment_nonce(tx.from)
                .map_err(|e| TransactionError::ExecutionFailed(e.to_string()))?;
        }
        
        Ok(result)
    }
    
    /// Execute a payment transaction
    fn execute_payment(
        &self,
        tx: &Transaction,
        payment: &lean_transaction::PaymentTransaction,
    ) -> Result<ExecutionResult, TransactionError> {
        // Transfer balance
        self.state.transfer(tx.from, payment.to, payment.amount)
            .map_err(|e| TransactionError::ExecutionFailed(e.to_string()))?;
        
        Ok(ExecutionResult {
            gas_used: 21_000, // Standard gas for transfer
            success: true,
            output: Vec::new(),
            logs: Vec::new(),
            error: None,
        })
    }
    
    /// Execute contract deployment
    fn execute_contract_deploy(
        &self,
        tx: &Transaction,
        deploy: &lean_transaction::ContractDeployTransaction,
    ) -> Result<ExecutionResult, TransactionError> {
        // Create contract address (simplified - use hash of deployer + nonce)
        let nonce = self.state.get_nonce(&tx.from)
            .map_err(|e| TransactionError::ExecutionFailed(e.to_string()))?;
        
        let mut data = Vec::new();
        data.extend_from_slice(tx.from.as_bytes());
        data.extend_from_slice(&nonce.to_le_bytes());
        let contract_address = Address::from_public_key(
            &lean_crypto::KeyPair::generate().public_key()
        ); // Simplified
        
        // Create contract account
        let mut account_state = crate::AccountState::empty();
        account_state.set_code(deploy.code.clone());
        account_state.account.balance = deploy.value;
        
        self.state.set_account(contract_address, account_state.account)
            .map_err(|e| TransactionError::DeploymentFailed(e.to_string()))?;
        
        // Deduct deployment value from sender
        if deploy.value > 0 {
            self.state.sub_balance(tx.from, deploy.value)
                .map_err(|e| TransactionError::ExecutionFailed(e.to_string()))?;
        }
        
        Ok(ExecutionResult {
            gas_used: 53_000 + (deploy.code.len() as u64 * 200), // Base + code size
            success: true,
            output: contract_address.as_bytes().to_vec(),
            logs: Vec::new(),
            error: None,
        })
    }
    
    /// Execute contract call
    fn execute_contract_call(
        &self,
        tx: &Transaction,
        call: &lean_transaction::ContractCallTransaction,
    ) -> Result<ExecutionResult, TransactionError> {
        // Check if contract exists
        let account = self.state.get_account(&call.to)
            .map_err(|e| TransactionError::ExecutionFailed(e.to_string()))?;
        
        if !account.is_contract() {
            return Err(TransactionError::ExecutionFailed(
                "Target is not a contract".to_string()
            ));
        }
        
        // Transfer value if any
        if call.value > 0 {
            self.state.transfer(tx.from, call.to, call.value)
                .map_err(|e| TransactionError::ExecutionFailed(e.to_string()))?;
        }
        
        // In a real implementation, we would execute the contract code here
        // For now, return a success result
        Ok(ExecutionResult {
            gas_used: 21_000 + (call.data.len() as u64 * 68),
            success: true,
            output: Vec::new(),
            logs: Vec::new(),
            error: None,
        })
    }
    
    /// Execute stake transaction
    fn execute_stake(
        &self,
        tx: &Transaction,
        stake: &lean_transaction::StakeTransaction,
    ) -> Result<ExecutionResult, TransactionError> {
        // Verify minimum stake
        if stake.amount < 100_000_000_000_000 {
            return Err(TransactionError::ExecutionFailed(
                "Stake amount below minimum".to_string()
            ));
        }
        
        // Deduct stake from balance
        self.state.sub_balance(tx.from, stake.amount)
            .map_err(|e| TransactionError::ExecutionFailed(e.to_string()))?;
        
        // In a real implementation, we would register the validator here
        
        Ok(ExecutionResult {
            gas_used: 50_000,
            success: true,
            output: Vec::new(),
            logs: Vec::new(),
            error: None,
        })
    }
    
    /// Execute unstake transaction
    fn execute_unstake(
        &self,
        tx: &Transaction,
        unstake: &lean_transaction::UnstakeTransaction,
    ) -> Result<ExecutionResult, TransactionError> {
        // In a real implementation, we would:
        // 1. Verify the validator exists
        // 2. Start unbonding period
        // 3. Queue for withdrawal
        
        Ok(ExecutionResult {
            gas_used: 40_000,
            success: true,
            output: Vec::new(),
            logs: Vec::new(),
            error: None,
        })
    }
    
    /// Execute delegate transaction
    fn execute_delegate(
        &self,
        tx: &Transaction,
        delegate: &lean_transaction::DelegateTransaction,
    ) -> Result<ExecutionResult, TransactionError> {
        // Deduct delegation amount
        self.state.sub_balance(tx.from, delegate.amount)
            .map_err(|e| TransactionError::ExecutionFailed(e.to_string()))?;
        
        // In a real implementation, we would record the delegation
        
        Ok(ExecutionResult {
            gas_used: 35_000,
            success: true,
            output: Vec::new(),
            logs: Vec::new(),
            error: None,
        })
    }
    
    /// Commit state changes
    pub fn commit(&self) -> Result<lean_crypto::Hash256, StateError> {
        self.state.commit()
    }
    
    /// Rollback state changes
    pub fn rollback(&self) {
        self.state.clear_cache();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MemoryStateStorage;
    use lean_crypto::KeyPair;
    use lean_transaction::{PaymentTransaction, TransactionType};
    use std::sync::Arc;
    
    #[test]
    fn test_payment_execution() {
        let storage = Arc::new(MemoryStateStorage::new());
        let state = State::new(storage).unwrap();
        let executor = StateExecutor::new(state);
        
        let keypair = KeyPair::generate();
        let from = Address::from_public_key(keypair.public_key());
        let to = Address::from_public_key(&KeyPair::generate().public_key());
        
        // Give sender some balance
        executor.state.add_balance(from, 1_000_000).unwrap();
        
        let payment = PaymentTransaction {
            to,
            amount: 500_000,
        };
        
        let mut tx = Transaction::new(
            0,
            from,
            TransactionType::Payment(payment),
            21_000,
            1,
        );
        tx.sign(&keypair);
        
        let result = executor.execute(&tx).unwrap();
        assert!(result.success);
        assert_eq!(result.gas_used, 21_000);
        
        // Verify balances
        assert_eq!(executor.state.get_balance(&to).unwrap(), 500_000);
    }
    
    #[test]
    fn test_insufficient_balance() {
        let storage = Arc::new(MemoryStateStorage::new());
        let state = State::new(storage).unwrap();
        let executor = StateExecutor::new(state);
        
        let keypair = KeyPair::generate();
        let from = Address::from_public_key(keypair.public_key());
        let to = Address::from_public_key(&KeyPair::generate().public_key());
        
        // Give sender insufficient balance
        executor.state.add_balance(from, 100_000).unwrap();
        
        let payment = PaymentTransaction {
            to,
            amount: 500_000,
        };
        
        let mut tx = Transaction::new(
            0,
            from,
            TransactionType::Payment(payment),
            21_000,
            1,
        );
        tx.sign(&keypair);
        
        assert!(executor.execute(&tx).is_err());
    }
}
