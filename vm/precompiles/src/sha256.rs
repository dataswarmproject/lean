//! SHA-256 precompiled contract (address 0x02)

use crate::{Precompile, PrecompileError, Result};
use sha2::{Sha256 as Sha256Hasher, Digest};

/// SHA-256 precompiled contract
pub struct Sha256;

impl Precompile for Sha256 {
    fn execute(&self, input: &[u8], gas_limit: u64) -> Result<(Vec<u8>, u64)> {
        let gas_cost = self.gas_cost(input);
        
        if gas_cost > gas_limit {
            return Err(PrecompileError::OutOfGas);
        }
        
        let mut hasher = Sha256Hasher::new();
        hasher.update(input);
        let result = hasher.finalize();
        
        Ok((result.to_vec(), gas_cost))
    }
    
    fn gas_cost(&self, input: &[u8]) -> u64 {
        // 60 gas base + 12 gas per word (32 bytes)
        let words = (input.len() + 31) / 32;
        60 + (words as u64 * 12)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sha256_empty() {
        let sha256 = Sha256;
        let (output, gas) = sha256.execute(&[], 1000).unwrap();
        
        assert_eq!(output.len(), 32);
        assert_eq!(gas, 60); // Base cost for empty input
    }
    
    #[test]
    fn test_sha256_hello_world() {
        let sha256 = Sha256;
        let input = b"hello world";
        let (output, _) = sha256.execute(input, 1000).unwrap();
        
        assert_eq!(output.len(), 32);
        
        // Verify against known SHA-256 of "hello world"
        let expected = hex::decode("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9").unwrap();
        assert_eq!(output, expected);
    }
    
    #[test]
    fn test_sha256_gas_cost() {
        let sha256 = Sha256;
        
        // Empty input: 60 gas
        assert_eq!(sha256.gas_cost(&[]), 60);
        
        // 32 bytes (1 word): 60 + 12 = 72
        assert_eq!(sha256.gas_cost(&[0u8; 32]), 72);
        
        // 64 bytes (2 words): 60 + 24 = 84
        assert_eq!(sha256.gas_cost(&[0u8; 64]), 84);
    }
}
