//! Native DEX for LEAN CHAIN
//!
//! Automated Market Maker (AMM) based on constant product formula
//! Similar to Uniswap V2

pub mod pool;
pub mod swap;
pub mod liquidity;

pub use pool::{Pool, PoolId};
pub use swap::SwapRouter;
pub use liquidity::LiquidityProvider;

use lean_crypto::Address;

/// Result type for DEX operations
pub type Result<T> = std::result::Result<T, DexError>;

/// DEX errors
#[derive(Debug, thiserror::Error)]
pub enum DexError {
    #[error("Insufficient liquidity")]
    InsufficientLiquidity,
    
    #[error("Insufficient input amount")]
    InsufficientInputAmount,
    
    #[error("Insufficient output amount")]
    InsufficientOutputAmount,
    
    #[error("Invalid token pair")]
    InvalidTokenPair,
    
    #[error("Pool not found")]
    PoolNotFound,
    
    #[error("Slippage tolerance exceeded: expected {expected}, got {actual}")]
    SlippageExceeded { expected: u128, actual: u128 },
    
    #[error("Deadline exceeded")]
    DeadlineExceeded,
    
    #[error("Insufficient balance: have {have}, need {need}")]
    InsufficientBalance { have: u128, need: u128 },
    
    #[error("Math overflow")]
    Overflow,
    
    #[error("Division by zero")]
    DivisionByZero,
}

/// Token pair representing two assets
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TokenPair {
    pub token0: Address,
    pub token1: Address,
}

impl TokenPair {
    /// Create a new token pair (tokens are sorted)
    pub fn new(token_a: Address, token_b: Address) -> Self {
        if token_a < token_b {
            Self {
                token0: token_a,
                token1: token_b,
            }
        } else {
            Self {
                token0: token_b,
                token1: token_a,
            }
        }
    }
    
    /// Check if this pair contains a token
    pub fn contains(&self, token: &Address) -> bool {
        &self.token0 == token || &self.token1 == token
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lean_crypto::KeyPair;
    
    #[test]
    fn test_token_pair_sorting() {
        let token_a = Address::from_public_key(&KeyPair::generate().public_key());
        let token_b = Address::from_public_key(&KeyPair::generate().public_key());
        
        let pair1 = TokenPair::new(token_a, token_b);
        let pair2 = TokenPair::new(token_b, token_a);
        
        // Pairs should be equal regardless of input order
        assert_eq!(pair1, pair2);
    }
}
