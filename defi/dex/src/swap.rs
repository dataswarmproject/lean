//! Swap router for multi-hop swaps

use crate::{Pool, PoolId, TokenPair, DexError, Result};
use lean_crypto::Address;
use std::collections::HashMap;

/// Swap router for executing trades
pub struct SwapRouter {
    /// All pools indexed by pool ID
    pools: HashMap<PoolId, Pool>,
    
    /// Pool lookup by token pair
    pair_to_pool: HashMap<TokenPair, PoolId>,
}

impl SwapRouter {
    /// Create a new swap router
    pub fn new() -> Self {
        Self {
            pools: HashMap::new(),
            pair_to_pool: HashMap::new(),
        }
    }
    
    /// Add a pool to the router
    pub fn add_pool(&mut self, pool: Pool) {
        self.pair_to_pool.insert(pool.pair, pool.id);
        self.pools.insert(pool.id, pool);
    }
    
    /// Get a pool by ID
    pub fn get_pool(&self, pool_id: &PoolId) -> Option<&Pool> {
        self.pools.get(pool_id)
    }
    
    /// Get a mutable pool by ID
    pub fn get_pool_mut(&mut self, pool_id: &PoolId) -> Option<&mut Pool> {
        self.pools.get_mut(pool_id)
    }
    
    /// Find pool for a token pair
    pub fn find_pool(&self, token_a: Address, token_b: Address) -> Option<&Pool> {
        let pair = TokenPair::new(token_a, token_b);
        self.pair_to_pool.get(&pair)
            .and_then(|pool_id| self.pools.get(pool_id))
    }
    
    /// Execute a direct swap
    pub fn swap_exact_tokens_for_tokens(
        &mut self,
        amount_in: u128,
        amount_out_min: u128,
        token_in: Address,
        token_out: Address,
        deadline: i64,
    ) -> Result<u128> {
        // Check deadline (simplified - would check against block timestamp)
        // if current_time > deadline { return Err(DexError::DeadlineExceeded); }
        
        // Find pool
        let pair = TokenPair::new(token_in, token_out);
        let pool_id = *self.pair_to_pool.get(&pair)
            .ok_or(DexError::PoolNotFound)?;
        
        let pool = self.pools.get_mut(&pool_id)
            .ok_or(DexError::PoolNotFound)?;
        
        // Execute swap
        let amount_out = pool.swap(amount_in, &token_in)?;
        
        // Check slippage
        if amount_out < amount_out_min {
            return Err(DexError::SlippageExceeded {
                expected: amount_out_min,
                actual: amount_out,
            });
        }
        
        Ok(amount_out)
    }
    
    /// Execute a multi-hop swap through intermediate tokens
    pub fn swap_exact_tokens_for_tokens_multi_hop(
        &mut self,
        amount_in: u128,
        amount_out_min: u128,
        path: Vec<Address>,
        deadline: i64,
    ) -> Result<u128> {
        if path.len() < 2 {
            return Err(DexError::InvalidTokenPair);
        }
        
        let mut current_amount = amount_in;
        
        // Execute swaps through the path
        for i in 0..path.len() - 1 {
            let token_in = path[i];
            let token_out = path[i + 1];
            
            let pair = TokenPair::new(token_in, token_out);
            let pool_id = *self.pair_to_pool.get(&pair)
                .ok_or(DexError::PoolNotFound)?;
            
            let pool = self.pools.get_mut(&pool_id)
                .ok_or(DexError::PoolNotFound)?;
            
            current_amount = pool.swap(current_amount, &token_in)?;
        }
        
        // Check final output meets minimum
        if current_amount < amount_out_min {
            return Err(DexError::SlippageExceeded {
                expected: amount_out_min,
                actual: current_amount,
            });
        }
        
        Ok(current_amount)
    }
    
    /// Quote output amount for a swap (without executing)
    pub fn quote(&self, amount_in: u128, token_in: Address, token_out: Address) -> Result<u128> {
        let pool = self.find_pool(token_in, token_out)
            .ok_or(DexError::PoolNotFound)?;
        
        let (reserve_in, reserve_out) = if token_in == pool.pair.token0 {
            (pool.reserve0, pool.reserve1)
        } else {
            (pool.reserve1, pool.reserve0)
        };
        
        pool.get_amount_out(amount_in, reserve_in, reserve_out)
    }
    
    /// Get all pools
    pub fn pools(&self) -> Vec<&Pool> {
        self.pools.values().collect()
    }
}

impl Default for SwapRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lean_crypto::KeyPair;
    
    #[test]
    fn test_swap_router() {
        let mut router = SwapRouter::new();
        
        let token_a = Address::from_public_key(&KeyPair::generate().public_key());
        let token_b = Address::from_public_key(&KeyPair::generate().public_key());
        let provider = Address::from_public_key(&KeyPair::generate().public_key());
        
        let pair = TokenPair::new(token_a, token_b);
        let mut pool = Pool::new(pair, 30);
        
        // Add liquidity
        pool.add_liquidity(provider, 10000, 20000).unwrap();
        
        router.add_pool(pool);
        
        // Execute swap
        let amount_out = router.swap_exact_tokens_for_tokens(
            1000,
            0, // No minimum
            token_a,
            token_b,
            9999999999,
        ).unwrap();
        
        assert!(amount_out > 0);
    }
    
    #[test]
    fn test_quote() {
        let mut router = SwapRouter::new();
        
        let token_a = Address::from_public_key(&KeyPair::generate().public_key());
        let token_b = Address::from_public_key(&KeyPair::generate().public_key());
        let provider = Address::from_public_key(&KeyPair::generate().public_key());
        
        let pair = TokenPair::new(token_a, token_b);
        let mut pool = Pool::new(pair, 30);
        
        pool.add_liquidity(provider, 10000, 20000).unwrap();
        router.add_pool(pool);
        
        let quote = router.quote(1000, token_a, token_b).unwrap();
        assert!(quote > 0);
    }
}
