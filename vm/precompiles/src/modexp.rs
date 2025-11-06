//! MODEXP precompiled contract (address 0x05)
//! Modular exponentiation: (base^exp) % modulus

use crate::{Precompile, PrecompileError, Result};
use num_bigint::BigUint;
use num_traits::{One, Zero};

/// MODEXP precompiled contract
///
/// Input format:
/// - bytes 0-31: length of base (B)
/// - bytes 32-63: length of exponent (E)
/// - bytes 64-95: length of modulus (M)
/// - bytes 96..: base, exponent, modulus (concatenated)
pub struct ModExp;

impl Precompile for ModExp {
    fn execute(&self, input: &[u8], gas_limit: u64) -> Result<(Vec<u8>, u64)> {
        let gas_cost = self.gas_cost(input);
        
        if gas_cost > gas_limit {
            return Err(PrecompileError::OutOfGas);
        }
        
        if input.len() < 96 {
            return Err(PrecompileError::InvalidInputLength {
                expected: 96,
                actual: input.len(),
            });
        }
        
        // Read lengths
        let base_len = read_u256(&input[0..32]) as usize;
        let exp_len = read_u256(&input[32..64]) as usize;
        let mod_len = read_u256(&input[64..96]) as usize;
        
        // Check input has enough data
        let required_len = 96 + base_len + exp_len + mod_len;
        if input.len() < required_len {
            return Err(PrecompileError::InvalidInputLength {
                expected: required_len,
                actual: input.len(),
            });
        }
        
        // Extract base, exponent, modulus
        let base_start = 96;
        let exp_start = base_start + base_len;
        let mod_start = exp_start + exp_len;
        
        let base = BigUint::from_bytes_be(&input[base_start..exp_start]);
        let exp = BigUint::from_bytes_be(&input[exp_start..mod_start]);
        let modulus = BigUint::from_bytes_be(&input[mod_start..mod_start + mod_len]);
        
        // Compute result
        let result = if modulus.is_zero() || modulus.is_one() {
            BigUint::zero()
        } else {
            base.modpow(&exp, &modulus)
        };
        
        // Convert result to bytes (left-padded to modulus length)
        let mut result_bytes = result.to_bytes_be();
        if result_bytes.len() < mod_len {
            let mut padded = vec![0u8; mod_len - result_bytes.len()];
            padded.extend_from_slice(&result_bytes);
            result_bytes = padded;
        } else if result_bytes.len() > mod_len {
            result_bytes = result_bytes[result_bytes.len() - mod_len..].to_vec();
        }
        
        Ok((result_bytes, gas_cost))
    }
    
    fn gas_cost(&self, input: &[u8]) -> u64 {
        if input.len() < 96 {
            return 200; // Minimum cost
        }
        
        let base_len = read_u256(&input[0..32]) as u64;
        let exp_len = read_u256(&input[32..64]) as u64;
        let mod_len = read_u256(&input[64..96]) as u64;
        
        // Simplified gas calculation (EIP-2565)
        let max_len = base_len.max(mod_len);
        let words = (max_len + 7) / 8;
        
        // Complexity = words^2
        let complexity = words * words;
        
        // Iteration count based on exponent length
        let iter_count = if exp_len <= 32 && input.len() >= 96 + base_len as usize {
            1
        } else {
            exp_len.max(1)
        };
        
        // Gas = max(200, complexity * iter_count / 3)
        ((complexity * iter_count) / 3).max(200)
    }
}

/// Read a U256 from big-endian bytes (simplified - reads as u64)
fn read_u256(bytes: &[u8]) -> u64 {
    if bytes.len() < 32 {
        return 0;
    }
    
    // Read last 8 bytes as u64 (simplified)
    let mut buf = [0u8; 8];
    buf.copy_from_slice(&bytes[24..32]);
    u64::from_be_bytes(buf)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_modexp_simple() {
        let modexp = ModExp;
        
        // Compute 3^5 mod 7 = 243 mod 7 = 5
        let mut input = vec![0u8; 96];
        
        // base_len = 1
        input[31] = 1;
        // exp_len = 1
        input[63] = 1;
        // mod_len = 1
        input[95] = 1;
        
        // base = 3
        input.push(3);
        // exp = 5
        input.push(5);
        // mod = 7
        input.push(7);
        
        let (output, _) = modexp.execute(&input, 100000).unwrap();
        
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], 5);
    }
    
    #[test]
    fn test_modexp_zero_modulus() {
        let modexp = ModExp;
        
        let mut input = vec![0u8; 96];
        input[31] = 1;
        input[63] = 1;
        input[95] = 1;
        
        input.push(3); // base
        input.push(5); // exp
        input.push(0); // mod (zero)
        
        let (output, _) = modexp.execute(&input, 100000).unwrap();
        
        assert_eq!(output[0], 0);
    }
}
