//! EVM memory implementation

use ethereum_types::U256;

/// Memory errors
#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    #[error("Memory access out of bounds")]
    OutOfBounds,
    
    #[error("Memory allocation too large")]
    AllocationTooLarge,
}

/// EVM memory (byte-addressable, word-sized operations)
#[derive(Debug, Clone)]
pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    /// Create a new memory instance
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }
    
    /// Expand memory to accommodate an offset + size
    pub fn expand(&mut self, offset: usize, size: usize) -> Result<u64, MemoryError> {
        if size == 0 {
            return Ok(0);
        }
        
        let end = offset.checked_add(size)
            .ok_or(MemoryError::AllocationTooLarge)?;
        
        if end > self.data.len() {
            // Calculate gas cost for expansion
            let old_size = self.data.len();
            let new_size = ((end + 31) / 32) * 32; // Round up to next word
            
            if new_size > 1024 * 1024 * 100 { // 100 MB limit
                return Err(MemoryError::AllocationTooLarge);
            }
            
            self.data.resize(new_size, 0);
            
            // Calculate memory expansion gas cost
            let gas_cost = memory_gas_cost(new_size) - memory_gas_cost(old_size);
            Ok(gas_cost)
        } else {
            Ok(0)
        }
    }
    
    /// Load a word (32 bytes) from memory
    pub fn load(&mut self, offset: usize) -> Result<U256, MemoryError> {
        self.expand(offset, 32)?;
        
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&self.data[offset..offset + 32]);
        Ok(U256::from_big_endian(&bytes))
    }
    
    /// Store a word (32 bytes) to memory
    pub fn store(&mut self, offset: usize, value: U256) -> Result<(), MemoryError> {
        self.expand(offset, 32)?;
        
        let mut bytes = [0u8; 32];
        value.to_big_endian(&mut bytes);
        self.data[offset..offset + 32].copy_from_slice(&bytes);
        Ok(())
    }
    
    /// Store a single byte to memory
    pub fn store_byte(&mut self, offset: usize, value: u8) -> Result<(), MemoryError> {
        self.expand(offset, 1)?;
        self.data[offset] = value;
        Ok(())
    }
    
    /// Load bytes from memory
    pub fn load_bytes(&mut self, offset: usize, size: usize) -> Result<Vec<u8>, MemoryError> {
        if size == 0 {
            return Ok(Vec::new());
        }
        
        self.expand(offset, size)?;
        Ok(self.data[offset..offset + size].to_vec())
    }
    
    /// Store bytes to memory
    pub fn store_bytes(&mut self, offset: usize, data: &[u8]) -> Result<(), MemoryError> {
        if data.is_empty() {
            return Ok(());
        }
        
        self.expand(offset, data.len())?;
        self.data[offset..offset + data.len()].copy_from_slice(data);
        Ok(())
    }
    
    /// Get memory size in bytes
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    /// Check if memory is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    /// Clear memory
    pub fn clear(&mut self) {
        self.data.clear();
    }
    
    /// Get all memory data (for debugging)
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculate memory gas cost
/// Formula: memory_size_word * 3 + memory_size_word^2 / 512
fn memory_gas_cost(size: usize) -> u64 {
    let size_word = (size + 31) / 32;
    let linear = size_word as u64 * 3;
    let quadratic = (size_word as u64 * size_word as u64) / 512;
    linear + quadratic
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_store_load() {
        let mut memory = Memory::new();
        
        let value = U256::from(12345);
        memory.store(0, value).unwrap();
        
        let loaded = memory.load(0).unwrap();
        assert_eq!(loaded, value);
    }
    
    #[test]
    fn test_store_byte() {
        let mut memory = Memory::new();
        
        memory.store_byte(0, 0xFF).unwrap();
        let bytes = memory.load_bytes(0, 1).unwrap();
        assert_eq!(bytes[0], 0xFF);
    }
    
    #[test]
    fn test_memory_expansion() {
        let mut memory = Memory::new();
        
        assert_eq!(memory.len(), 0);
        
        memory.expand(0, 32).unwrap();
        assert!(memory.len() >= 32);
    }
    
    #[test]
    fn test_store_load_bytes() {
        let mut memory = Memory::new();
        
        let data = vec![1, 2, 3, 4, 5];
        memory.store_bytes(10, &data).unwrap();
        
        let loaded = memory.load_bytes(10, 5).unwrap();
        assert_eq!(loaded, data);
    }
}
