//! Identity precompiled contract (address 0x04)
//! Simply copies input to output (like memcpy)

use crate::{Precompile, PrecompileError, Result};

/// Identity precompiled contract (data copy)
pub struct Identity;

impl Precompile for Identity {
    fn execute(&self, input: &[u8], gas_limit: u64) -> Result<(Vec<u8>, u64)> {
        let gas_cost = self.gas_cost(input);
        
        if gas_cost > gas_limit {
            return Err(PrecompileError::OutOfGas);
        }
        
        Ok((input.to_vec(), gas_cost))
    }
    
    fn gas_cost(&self, input: &[u8]) -> u64 {
        // 15 gas base + 3 gas per word (32 bytes)
        let words = (input.len() + 31) / 32;
        15 + (words as u64 * 3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_identity_empty() {
        let identity = Identity;
        let (output, gas) = identity.execute(&[], 100).unwrap();
        
        assert_eq!(output.len(), 0);
        assert_eq!(gas, 15); // Base cost
    }
    
    #[test]
    fn test_identity_copy() {
        let identity = Identity;
        let input = b"hello world";
        let (output, _) = identity.execute(input, 100).unwrap();
        
        assert_eq!(output, input);
    }
    
    #[test]
    fn test_identity_gas_cost() {
        let identity = Identity;
        
        // Empty: 15
        assert_eq!(identity.gas_cost(&[]), 15);
        
        // 32 bytes: 15 + 3 = 18
        assert_eq!(identity.gas_cost(&[0u8; 32]), 18);
        
        // 64 bytes: 15 + 6 = 21
        assert_eq!(identity.gas_cost(&[0u8; 64]), 21);
    }
}
