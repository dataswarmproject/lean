//! ECRECOVER precompiled contract (address 0x01)
//! Recovers the public key from an ECDSA signature

use crate::{Precompile, PrecompileError, Result};

/// ECRECOVER precompiled contract
/// 
/// Input: 128 bytes
/// - bytes 0-31: hash
/// - bytes 32-63: v (recovery id)
/// - bytes 64-95: r
/// - bytes 96-127: s
/// 
/// Output: 32 bytes (address, left-padded with zeros)
pub struct EcRecover;

impl Precompile for EcRecover {
    fn execute(&self, input: &[u8], gas_limit: u64) -> Result<(Vec<u8>, u64)> {
        let gas_cost = self.gas_cost(input);
        
        if gas_cost > gas_limit {
            return Err(PrecompileError::OutOfGas);
        }
        
        // Pad input to 128 bytes if needed
        let mut padded_input = input.to_vec();
        padded_input.resize(128, 0);
        
        // Extract components
        let hash = &padded_input[0..32];
        let v = &padded_input[32..64];
        let r = &padded_input[64..96];
        let s = &padded_input[96..128];
        
        // Extract v as u8 (should be 27 or 28, or 0 or 1)
        let v_byte = v[31];
        let recovery_id = if v_byte >= 27 { v_byte - 27 } else { v_byte };
        
        // Validate recovery id
        if recovery_id > 3 {
            return Ok((vec![0u8; 32], gas_cost));
        }
        
        // TODO: Implement actual signature recovery
        // For now, return zeros (simplified implementation)
        // In production, this would use secp256k1 library
        
        Ok((vec![0u8; 32], gas_cost))
    }
    
    fn gas_cost(&self, _input: &[u8]) -> u64 {
        3000 // Fixed cost for ECRECOVER
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ecrecover_gas() {
        let ecrecover = EcRecover;
        assert_eq!(ecrecover.gas_cost(&[]), 3000);
    }
    
    #[test]
    fn test_ecrecover_short_input() {
        let ecrecover = EcRecover;
        let input = vec![0u8; 64]; // Shorter than 128
        let (output, gas) = ecrecover.execute(&input, 5000).unwrap();
        
        assert_eq!(output.len(), 32);
        assert_eq!(gas, 3000);
    }
}
