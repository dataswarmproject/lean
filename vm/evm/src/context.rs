//! Execution context for EVM

use lean_crypto::Address;
use ethereum_types::U256;

/// Execution context containing environmental information
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// Current contract address
    pub address: Address,
    
    /// Caller address
    pub caller: Address,
    
    /// Transaction origin
    pub origin: Address,
    
    /// Gas price
    pub gas_price: U256,
    
    /// Value sent with call
    pub value: U256,
    
    /// Input data
    pub input: Vec<u8>,
    
    /// Block number
    pub block_number: u64,
    
    /// Block timestamp
    pub block_timestamp: i64,
    
    /// Block coinbase (validator)
    pub block_coinbase: Address,
    
    /// Block difficulty
    pub block_difficulty: U256,
    
    /// Block gas limit
    pub block_gas_limit: u64,
    
    /// Chain ID
    pub chain_id: u64,
    
    /// Block base fee (EIP-1559)
    pub block_base_fee: U256,
}

impl ExecutionContext {
    /// Create a new execution context
    pub fn new(
        address: Address,
        caller: Address,
        origin: Address,
        gas_price: U256,
        value: U256,
        input: Vec<u8>,
    ) -> Self {
        Self {
            address,
            caller,
            origin,
            gas_price,
            value,
            input,
            block_number: 0,
            block_timestamp: 0,
            block_coinbase: Address::zero(),
            block_difficulty: U256::zero(),
            block_gas_limit: 30_000_000,
            chain_id: 1,
            block_base_fee: U256::zero(),
        }
    }
    
    /// Set block information
    pub fn with_block_info(
        mut self,
        number: u64,
        timestamp: i64,
        coinbase: Address,
        difficulty: U256,
        gas_limit: u64,
    ) -> Self {
        self.block_number = number;
        self.block_timestamp = timestamp;
        self.block_coinbase = coinbase;
        self.block_difficulty = difficulty;
        self.block_gas_limit = gas_limit;
        self
    }
    
    /// Set chain ID
    pub fn with_chain_id(mut self, chain_id: u64) -> Self {
        self.chain_id = chain_id;
        self
    }
}

/// Call context for contract calls
#[derive(Debug, Clone)]
pub struct CallContext {
    /// Execution context
    pub context: ExecutionContext,
    
    /// Contract code
    pub code: Vec<u8>,
    
    /// Is this a static call (read-only)
    pub is_static: bool,
    
    /// Call depth
    pub depth: usize,
}

impl CallContext {
    /// Create a new call context
    pub fn new(context: ExecutionContext, code: Vec<u8>) -> Self {
        Self {
            context,
            code,
            is_static: false,
            depth: 0,
        }
    }
    
    /// Create a static call context
    pub fn static_call(context: ExecutionContext, code: Vec<u8>) -> Self {
        Self {
            context,
            code,
            is_static: true,
            depth: 0,
        }
    }
    
    /// Increment call depth
    pub fn deeper(&self) -> Self {
        let mut ctx = self.clone();
        ctx.depth += 1;
        ctx
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lean_crypto::KeyPair;
    
    #[test]
    fn test_execution_context() {
        let address = Address::from_public_key(&KeyPair::generate().public_key());
        let caller = Address::from_public_key(&KeyPair::generate().public_key());
        
        let ctx = ExecutionContext::new(
            address,
            caller,
            caller,
            U256::from(1),
            U256::from(0),
            vec![],
        );
        
        assert_eq!(ctx.address, address);
        assert_eq!(ctx.caller, caller);
    }
    
    #[test]
    fn test_call_context() {
        let address = Address::from_public_key(&KeyPair::generate().public_key());
        let caller = Address::from_public_key(&KeyPair::generate().public_key());
        
        let exec_ctx = ExecutionContext::new(
            address,
            caller,
            caller,
            U256::from(1),
            U256::from(0),
            vec![],
        );
        
        let call_ctx = CallContext::new(exec_ctx, vec![0x60, 0x00]);
        assert_eq!(call_ctx.depth, 0);
        
        let deeper = call_ctx.deeper();
        assert_eq!(deeper.depth, 1);
    }
}
