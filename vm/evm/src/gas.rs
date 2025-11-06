//! Gas metering for EVM

/// Gas meter for tracking execution costs
#[derive(Debug, Clone)]
pub struct Gas {
    /// Gas limit
    limit: u64,
    
    /// Gas used
    used: u64,
    
    /// Gas refunded
    refunded: u64,
}

impl Gas {
    /// Create a new gas meter
    pub fn new(limit: u64) -> Self {
        Self {
            limit,
            used: 0,
            refunded: 0,
        }
    }
    
    /// Consume gas
    pub fn consume(&mut self, amount: u64) -> Result<(), super::EvmError> {
        let new_used = self.used.checked_add(amount)
            .ok_or(super::EvmError::OutOfGas)?;
        
        if new_used > self.limit {
            return Err(super::EvmError::OutOfGas);
        }
        
        self.used = new_used;
        Ok(())
    }
    
    /// Refund gas
    pub fn refund(&mut self, amount: u64) {
        self.refunded = self.refunded.saturating_add(amount);
    }
    
    /// Get gas remaining
    pub fn remaining(&self) -> u64 {
        self.limit.saturating_sub(self.used)
    }
    
    /// Get gas used
    pub fn used(&self) -> u64 {
        self.used
    }
    
    /// Get gas refunded
    pub fn refunded(&self) -> u64 {
        self.refunded
    }
    
    /// Get net gas used (used - refunded, capped at used / 2)
    pub fn net_used(&self) -> u64 {
        let max_refund = self.used / 2;
        let actual_refund = self.refunded.min(max_refund);
        self.used - actual_refund
    }
}

/// Gas costs for common operations
pub struct GasCost;

impl GasCost {
    pub const ZERO: u64 = 0;
    pub const BASE: u64 = 2;
    pub const VERY_LOW: u64 = 3;
    pub const LOW: u64 = 5;
    pub const MID: u64 = 8;
    pub const HIGH: u64 = 10;
    
    pub const JUMPDEST: u64 = 1;
    pub const SLOAD: u64 = 100;
    pub const SSTORE_SET: u64 = 20000;
    pub const SSTORE_RESET: u64 = 5000;
    pub const SSTORE_REFUND: u64 = 15000;
    
    pub const CREATE: u64 = 32000;
    pub const CALL: u64 = 100;
    pub const CALL_VALUE: u64 = 9000;
    pub const CALL_STIPEND: u64 = 2300;
    
    pub const SELFDESTRUCT: u64 = 5000;
    pub const SELFDESTRUCT_REFUND: u64 = 24000;
    
    pub const MEMORY_WORD: u64 = 3;
    pub const COPY_WORD: u64 = 3;
    pub const SHA3_WORD: u64 = 6;
    pub const LOG_BASE: u64 = 375;
    pub const LOG_TOPIC: u64 = 375;
    pub const LOG_DATA: u64 = 8;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gas_consumption() {
        let mut gas = Gas::new(1000);
        
        gas.consume(100).unwrap();
        assert_eq!(gas.used(), 100);
        assert_eq!(gas.remaining(), 900);
    }
    
    #[test]
    fn test_out_of_gas() {
        let mut gas = Gas::new(100);
        
        assert!(gas.consume(101).is_err());
    }
    
    #[test]
    fn test_gas_refund() {
        let mut gas = Gas::new(1000);
        
        gas.consume(500).unwrap();
        gas.refund(100);
        
        assert_eq!(gas.used(), 500);
        assert_eq!(gas.refunded(), 100);
        assert_eq!(gas.net_used(), 400);
    }
    
    #[test]
    fn test_refund_cap() {
        let mut gas = Gas::new(1000);
        
        gas.consume(500).unwrap();
        gas.refund(300); // More than half of used
        
        // Refund should be capped at used / 2 = 250
        assert_eq!(gas.net_used(), 250);
    }
}
