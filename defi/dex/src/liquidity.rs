//! Liquidity provider interface

use crate::{Pool, SwapRouter, TokenPair, DexError, Result};
use lean_crypto::Address;

/// Liquidity provider operations
pub struct LiquidityProvider {
    /// Reference to swap router
    router: SwapRouter,
}

impl LiquidityProvider {
    /// Create a new liquidity provider
    pub fn new() -> Self {
        Self {
            router: SwapRouter::new(),
        }
    }
    
    /// Create a new pool
    pub fn create_pool(&mut self, token_a: Address, token_b: Address, fee_bps: u16) -> Pool {
        let pair = TokenPair::new(token_a, token_b);
        Pool::new(pair, fee_bps)
    }
    
    /// Add liquidity to a pool
    pub fn add_liquidity(
        &mut self,
        token_a: Address,
        token_b: Address,
        amount_a: u128,
        amount_b: u128,
        amount_a_min: u128,
        amount_b_min: u128,
        provider: Address,
    ) -> Result<(u128, u128, u128)> {
        let pair = TokenPair::new(token_a, token_b);
        
        // Find or create pool
        let pool = self.router.find_pool(token_a, token_b)
            .map(|p| p.clone())
            .unwrap_or_else(|| Pool::new(pair, 30)); // Default 0.3% fee
        
        // Calculate optimal amounts
        let (final_amount_a, final_amount_b) = if pool.total_liquidity == 0 {
            // First liquidity provision
            (amount_a, amount_b)
        } else {
            // Calculate amounts based on existing ratio
            let amount_b_optimal = quote(amount_a, pool.reserve0, pool.reserve1)?;
            
            if amount_b_optimal <= amount_b {
                if amount_b_optimal < amount_b_min {
                    return Err(DexError::InsufficientOutputAmount);
                }
                (amount_a, amount_b_optimal)
            } else {
                let amount_a_optimal = quote(amount_b, pool.reserve1, pool.reserve0)?;
                
                if amount_a_optimal > amount_a || amount_a_optimal < amount_a_min {
                    return Err(DexError::InsufficientOutputAmount);
                }
                (amount_a_optimal, amount_b)
            }
        };
        
        // Add liquidity to pool
        let mut pool_clone = pool.clone();
        let liquidity = pool_clone.add_liquidity(provider, final_amount_a, final_amount_b)?;
        
        // Update router
        self.router.add_pool(pool_clone);
        
        Ok((final_amount_a, final_amount_b, liquidity))
    }
    
    /// Remove liquidity from a pool
    pub fn remove_liquidity(
        &mut self,
        token_a: Address,
        token_b: Address,
        liquidity: u128,
        amount_a_min: u128,
        amount_b_min: u128,
        provider: Address,
    ) -> Result<(u128, u128)> {
        let pair = TokenPair::new(token_a, token_b);
        
        // Find pool
        let pool_id = *self.router.pair_to_pool.get(&pair)
            .ok_or(DexError::PoolNotFound)?;
        
        let pool = self.router.get_pool_mut(&pool_id)
            .ok_or(DexError::PoolNotFound)?;
        
        // Remove liquidity
        let (amount_a, amount_b) = pool.remove_liquidity(provider, liquidity)?;
        
        // Check minimums
        if amount_a < amount_a_min || amount_b < amount_b_min {
            return Err(DexError::SlippageExceeded {
                expected: amount_a_min.max(amount_b_min),
                actual: amount_a.min(amount_b),
            });
        }
        
        Ok((amount_a, amount_b))
    }
    
    /// Get liquidity provider's share in a pool
    pub fn get_liquidity_balance(&self, token_a: Address, token_b: Address, provider: Address) -> Option<u128> {
        let pool = self.router.find_pool(token_a, token_b)?;
        pool.liquidity_providers.get(&provider).copied()
    }
    
    /// Get router reference
    pub fn router(&self) -> &SwapRouter {
        &self.router
    }
    
    /// Get mutable router reference
    pub fn router_mut(&mut self) -> &mut SwapRouter {
        &mut self.router
    }
}

impl Default for LiquidityProvider {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculate quote for adding liquidity
fn quote(amount_a: u128, reserve_a: u128, reserve_b: u128) -> Result<u128> {
    if amount_a == 0 {
        return Err(DexError::InsufficientInputAmount);
    }
    
    if reserve_a == 0 || reserve_b == 0 {
        return Err(DexError::InsufficientLiquidity);
    }
    
    amount_a.checked_mul(reserve_b)
        .ok_or(DexError::Overflow)?
        .checked_div(reserve_a)
        .ok_or(DexError::DivisionByZero)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lean_crypto::KeyPair;
    
    #[test]
    fn test_create_pool() {
        let mut lp = LiquidityProvider::new();
        
        let token_a = Address::from_public_key(&KeyPair::generate().public_key());
        let token_b = Address::from_public_key(&KeyPair::generate().public_key());
        
        let pool = lp.create_pool(token_a, token_b, 30);
        
        assert_eq!(pool.fee_bps, 30);
        assert_eq!(pool.total_liquidity, 0);
    }
    
    #[test]
    fn test_add_initial_liquidity() {
        let mut lp = LiquidityProvider::new();
        
        let token_a = Address::from_public_key(&KeyPair::generate().public_key());
        let token_b = Address::from_public_key(&KeyPair::generate().public_key());
        let provider = Address::from_public_key(&KeyPair::generate().public_key());
        
        let (amount_a, amount_b, liquidity) = lp.add_liquidity(
            token_a,
            token_b,
            1000,
            2000,
            0,
            0,
            provider,
        ).unwrap();
        
        assert_eq!(amount_a, 1000);
        assert_eq!(amount_b, 2000);
        assert!(liquidity > 0);
    }
    
    #[test]
    fn test_liquidity_balance() {
        let mut lp = LiquidityProvider::new();
        
        let token_a = Address::from_public_key(&KeyPair::generate().public_key());
        let token_b = Address::from_public_key(&KeyPair::generate().public_key());
        let provider = Address::from_public_key(&KeyPair::generate().public_key());
        
        lp.add_liquidity(token_a, token_b, 1000, 2000, 0, 0, provider).unwrap();
        
        let balance = lp.get_liquidity_balance(token_a, token_b, provider);
        assert!(balance.is_some());
        assert!(balance.unwrap() > 0);
    }
}
