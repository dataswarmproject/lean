//! Liquidity pools for the DEX

use crate::{TokenPair, DexError, Result};
use lean_crypto::{Address, Hash256};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Pool ID (hash of token pair)
pub type PoolId = Hash256;

/// Liquidity pool implementing constant product formula (x * y = k)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pool {
    /// Pool ID
    pub id: PoolId,
    
    /// Token pair
    pub pair: TokenPair,
    
    /// Reserve of token0
    pub reserve0: u128,
    
    /// Reserve of token1
    pub reserve1: u128,
    
    /// Total liquidity tokens
    pub total_liquidity: u128,
    
    /// Fee (in basis points, e.g., 30 = 0.3%)
    pub fee_bps: u16,
    
    /// Liquidity providers and their shares
    pub liquidity_providers: HashMap<Address, u128>,
}

impl Pool {
    /// Create a new pool
    pub fn new(pair: TokenPair, fee_bps: u16) -> Self {
        let id = Self::compute_pool_id(&pair);
        
        Self {
            id,
            pair,
            reserve0: 0,
            reserve1: 0,
            total_liquidity: 0,
            fee_bps,
            liquidity_providers: HashMap::new(),
        }
    }
    
    /// Compute pool ID from token pair
    pub fn compute_pool_id(pair: &TokenPair) -> PoolId {
        let mut data = Vec::new();
        data.extend_from_slice(pair.token0.as_bytes());
        data.extend_from_slice(pair.token1.as_bytes());
        Hash256::hash(&data)
    }
    
    /// Add liquidity to the pool
    pub fn add_liquidity(
        &mut self,
        provider: Address,
        amount0: u128,
        amount1: u128,
    ) -> Result<u128> {
        if amount0 == 0 || amount1 == 0 {
            return Err(DexError::InsufficientInputAmount);
        }
        
        let liquidity = if self.total_liquidity == 0 {
            // First liquidity provision
            // Use geometric mean of amounts
            let product = amount0.checked_mul(amount1)
                .ok_or(DexError::Overflow)?;
            integer_sqrt(product)
        } else {
            // Subsequent liquidity provision
            // liquidity = min(amount0 * totalLiquidity / reserve0, amount1 * totalLiquidity / reserve1)
            let liquidity0 = amount0.checked_mul(self.total_liquidity)
                .ok_or(DexError::Overflow)?
                .checked_div(self.reserve0)
                .ok_or(DexError::DivisionByZero)?;
            
            let liquidity1 = amount1.checked_mul(self.total_liquidity)
                .ok_or(DexError::Overflow)?
                .checked_div(self.reserve1)
                .ok_or(DexError::DivisionByZero)?;
            
            liquidity0.min(liquidity1)
        };
        
        if liquidity == 0 {
            return Err(DexError::InsufficientLiquidity);
        }
        
        // Update reserves
        self.reserve0 = self.reserve0.checked_add(amount0)
            .ok_or(DexError::Overflow)?;
        self.reserve1 = self.reserve1.checked_add(amount1)
            .ok_or(DexError::Overflow)?;
        
        // Update liquidity
        self.total_liquidity = self.total_liquidity.checked_add(liquidity)
            .ok_or(DexError::Overflow)?;
        
        // Update provider's share
        *self.liquidity_providers.entry(provider).or_insert(0) = 
            self.liquidity_providers.get(&provider).unwrap_or(&0)
                .checked_add(liquidity)
                .ok_or(DexError::Overflow)?;
        
        Ok(liquidity)
    }
    
    /// Remove liquidity from the pool
    pub fn remove_liquidity(
        &mut self,
        provider: Address,
        liquidity: u128,
    ) -> Result<(u128, u128)> {
        if liquidity == 0 {
            return Err(DexError::InsufficientInputAmount);
        }
        
        // Check provider has enough liquidity
        let provider_liquidity = self.liquidity_providers.get(&provider)
            .copied()
            .unwrap_or(0);
        
        if provider_liquidity < liquidity {
            return Err(DexError::InsufficientBalance {
                have: provider_liquidity,
                need: liquidity,
            });
        }
        
        // Calculate amounts to return
        let amount0 = liquidity.checked_mul(self.reserve0)
            .ok_or(DexError::Overflow)?
            .checked_div(self.total_liquidity)
            .ok_or(DexError::DivisionByZero)?;
        
        let amount1 = liquidity.checked_mul(self.reserve1)
            .ok_or(DexError::Overflow)?
            .checked_div(self.total_liquidity)
            .ok_or(DexError::DivisionByZero)?;
        
        if amount0 == 0 || amount1 == 0 {
            return Err(DexError::InsufficientLiquidity);
        }
        
        // Update reserves
        self.reserve0 = self.reserve0.checked_sub(amount0)
            .ok_or(DexError::Overflow)?;
        self.reserve1 = self.reserve1.checked_sub(amount1)
            .ok_or(DexError::Overflow)?;
        
        // Update liquidity
        self.total_liquidity = self.total_liquidity.checked_sub(liquidity)
            .ok_or(DexError::Overflow)?;
        
        // Update provider's share
        let new_liquidity = provider_liquidity.checked_sub(liquidity)
            .ok_or(DexError::Overflow)?;
        
        if new_liquidity == 0 {
            self.liquidity_providers.remove(&provider);
        } else {
            self.liquidity_providers.insert(provider, new_liquidity);
        }
        
        Ok((amount0, amount1))
    }
    
    /// Get output amount for a given input (implements constant product formula)
    pub fn get_amount_out(&self, amount_in: u128, reserve_in: u128, reserve_out: u128) -> Result<u128> {
        if amount_in == 0 {
            return Err(DexError::InsufficientInputAmount);
        }
        
        if reserve_in == 0 || reserve_out == 0 {
            return Err(DexError::InsufficientLiquidity);
        }
        
        // Apply fee (e.g., 0.3% = 30 bps)
        let fee_multiplier = 10000u128.checked_sub(self.fee_bps as u128)
            .ok_or(DexError::Overflow)?;
        
        let amount_in_with_fee = amount_in.checked_mul(fee_multiplier)
            .ok_or(DexError::Overflow)?;
        
        let numerator = amount_in_with_fee.checked_mul(reserve_out)
            .ok_or(DexError::Overflow)?;
        
        let denominator = reserve_in.checked_mul(10000)
            .ok_or(DexError::Overflow)?
            .checked_add(amount_in_with_fee)
            .ok_or(DexError::Overflow)?;
        
        let amount_out = numerator.checked_div(denominator)
            .ok_or(DexError::DivisionByZero)?;
        
        if amount_out == 0 {
            return Err(DexError::InsufficientOutputAmount);
        }
        
        Ok(amount_out)
    }
    
    /// Execute a swap
    pub fn swap(&mut self, amount_in: u128, token_in: &Address) -> Result<u128> {
        let (reserve_in, reserve_out) = if token_in == &self.pair.token0 {
            (self.reserve0, self.reserve1)
        } else if token_in == &self.pair.token1 {
            (self.reserve1, self.reserve0)
        } else {
            return Err(DexError::InvalidTokenPair);
        };
        
        let amount_out = self.get_amount_out(amount_in, reserve_in, reserve_out)?;
        
        // Update reserves
        if token_in == &self.pair.token0 {
            self.reserve0 = self.reserve0.checked_add(amount_in)
                .ok_or(DexError::Overflow)?;
            self.reserve1 = self.reserve1.checked_sub(amount_out)
                .ok_or(DexError::Overflow)?;
        } else {
            self.reserve1 = self.reserve1.checked_add(amount_in)
                .ok_or(DexError::Overflow)?;
            self.reserve0 = self.reserve0.checked_sub(amount_out)
                .ok_or(DexError::Overflow)?;
        }
        
        Ok(amount_out)
    }
    
    /// Get current price (token1 per token0)
    pub fn price(&self) -> Result<f64> {
        if self.reserve0 == 0 {
            return Err(DexError::DivisionByZero);
        }
        Ok(self.reserve1 as f64 / self.reserve0 as f64)
    }
}

/// Integer square root using Newton's method
fn integer_sqrt(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    
    let mut x = n;
    let mut y = (x + 1) / 2;
    
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    
    x
}

#[cfg(test)]
mod tests {
    use super::*;
    use lean_crypto::KeyPair;
    
    #[test]
    fn test_pool_creation() {
        let token0 = Address::from_public_key(&KeyPair::generate().public_key());
        let token1 = Address::from_public_key(&KeyPair::generate().public_key());
        
        let pair = TokenPair::new(token0, token1);
        let pool = Pool::new(pair, 30); // 0.3% fee
        
        assert_eq!(pool.reserve0, 0);
        assert_eq!(pool.reserve1, 0);
        assert_eq!(pool.total_liquidity, 0);
        assert_eq!(pool.fee_bps, 30);
    }
    
    #[test]
    fn test_add_initial_liquidity() {
        let token0 = Address::from_public_key(&KeyPair::generate().public_key());
        let token1 = Address::from_public_key(&KeyPair::generate().public_key());
        let provider = Address::from_public_key(&KeyPair::generate().public_key());
        
        let pair = TokenPair::new(token0, token1);
        let mut pool = Pool::new(pair, 30);
        
        let liquidity = pool.add_liquidity(provider, 1000, 2000).unwrap();
        
        assert!(liquidity > 0);
        assert_eq!(pool.reserve0, 1000);
        assert_eq!(pool.reserve1, 2000);
        assert_eq!(pool.total_liquidity, liquidity);
    }
    
    #[test]
    fn test_swap() {
        let token0 = Address::from_public_key(&KeyPair::generate().public_key());
        let token1 = Address::from_public_key(&KeyPair::generate().public_key());
        let provider = Address::from_public_key(&KeyPair::generate().public_key());
        
        let pair = TokenPair::new(token0, token1);
        let mut pool = Pool::new(pair, 30);
        
        // Add liquidity
        pool.add_liquidity(provider, 10000, 20000).unwrap();
        
        // Swap 1000 token0 for token1
        let amount_out = pool.swap(1000, &token0).unwrap();
        
        assert!(amount_out > 0);
        assert!(amount_out < 2000); // Should get less than proportional due to slippage
    }
    
    #[test]
    fn test_remove_liquidity() {
        let token0 = Address::from_public_key(&KeyPair::generate().public_key());
        let token1 = Address::from_public_key(&KeyPair::generate().public_key());
        let provider = Address::from_public_key(&KeyPair::generate().public_key());
        
        let pair = TokenPair::new(token0, token1);
        let mut pool = Pool::new(pair, 30);
        
        // Add liquidity
        let liquidity = pool.add_liquidity(provider, 1000, 2000).unwrap();
        
        // Remove half
        let (amount0, amount1) = pool.remove_liquidity(provider, liquidity / 2).unwrap();
        
        assert!(amount0 > 0);
        assert!(amount1 > 0);
    }
}
