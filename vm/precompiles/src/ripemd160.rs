//! RIPEMD-160 precompiled contract (address 0x03)

use crate::{Precompile, PrecompileError, Result};
use ripemd::{Ripemd160 as Ripemd160Hasher, Digest};

/// RIPEMD-160 precompiled contract
pub struct Ripemd160;

impl Precompile for Ripemd160 {
    fn execute(&self, input: &[u8], gas_limit: u64) -> Result<(Vec<u8>, u64)> {
        let gas_cost = self.gas_cost(input);
        
        if gas_cost > gas_limit {
            return Err(PrecompileError::OutOfGas);
        }
        
        let mut hasher = Ripemd160Hasher::new();
        hasher.update(input);
        let result = hasher.finalize();
        
        // Pad to 32 bytes (Ethereum standard - left-padded with zeros)
        let mut output = vec![0u8; 12];
        output.extend_from_slice(&result);
        
        Ok((output, gas_cost))
    }
    
    fn gas_cost(&self, input: &[u8]) -> u64 {
        // 600 gas base + 120 gas per word (32 bytes)
        let words = (input.len() + 31) / 32;
        600 + (words as u64 * 120)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ripemd160_empty() {
        let ripemd = Ripemd160;
        let (output, gas) = ripemd.execute(&[], 1000).unwrap();
        
        assert_eq!(output.len(), 32); // Padded to 32 bytes
        assert_eq!(gas, 600); // Base cost
    }
    
    #[test]
    fn test_ripemd160_gas_cost() {
        let ripemd = Ripemd160;
        
        // Empty: 600
        assert_eq!(ripemd.gas_cost(&[]), 600);
        
        // 32 bytes: 600 + 120 = 720
        assert_eq!(ripemd.gas_cost(&[0u8; 32]), 720);
        
        // 64 bytes: 600 + 240 = 840
        assert_eq!(ripemd.gas_cost(&[0u8; 64]), 840);
    }
}
