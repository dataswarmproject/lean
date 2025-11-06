//! EVM stack implementation

use ethereum_types::U256;

/// Maximum stack depth
const MAX_STACK_SIZE: usize = 1024;

/// Stack errors
#[derive(Debug, thiserror::Error)]
pub enum StackError {
    #[error("Stack overflow")]
    Overflow,
    
    #[error("Stack underflow")]
    Underflow,
    
    #[error("Invalid stack access at position {0}")]
    InvalidAccess(usize),
}

/// EVM stack
#[derive(Debug, Clone)]
pub struct Stack {
    data: Vec<U256>,
}

impl Stack {
    /// Create a new stack
    pub fn new() -> Self {
        Self {
            data: Vec::with_capacity(MAX_STACK_SIZE),
        }
    }
    
    /// Push a value onto the stack
    pub fn push(&mut self, value: U256) -> Result<(), StackError> {
        if self.data.len() >= MAX_STACK_SIZE {
            return Err(StackError::Overflow);
        }
        self.data.push(value);
        Ok(())
    }
    
    /// Pop a value from the stack
    pub fn pop(&mut self) -> Result<U256, StackError> {
        self.data.pop().ok_or(StackError::Underflow)
    }
    
    /// Peek at the top value without removing it
    pub fn peek(&self) -> Result<U256, StackError> {
        self.data.last().copied().ok_or(StackError::Underflow)
    }
    
    /// Peek at a value at a specific depth (0 = top)
    pub fn peek_at(&self, depth: usize) -> Result<U256, StackError> {
        if depth >= self.data.len() {
            return Err(StackError::Underflow);
        }
        let index = self.data.len() - 1 - depth;
        Ok(self.data[index])
    }
    
    /// Set a value at a specific depth (0 = top)
    pub fn set_at(&mut self, depth: usize, value: U256) -> Result<(), StackError> {
        if depth >= self.data.len() {
            return Err(StackError::Underflow);
        }
        let index = self.data.len() - 1 - depth;
        self.data[index] = value;
        Ok(())
    }
    
    /// Duplicate a value at depth n
    pub fn dup(&mut self, n: usize) -> Result<(), StackError> {
        let value = self.peek_at(n)?;
        self.push(value)
    }
    
    /// Swap the top value with a value at depth n
    pub fn swap(&mut self, n: usize) -> Result<(), StackError> {
        if n >= self.data.len() {
            return Err(StackError::Underflow);
        }
        let len = self.data.len();
        let index = len - 1 - n;
        self.data.swap(len - 1, index);
        Ok(())
    }
    
    /// Get stack size
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    /// Check if stack is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    /// Clear the stack
    pub fn clear(&mut self) {
        self.data.clear();
    }
    
    /// Get all stack data (for debugging)
    pub fn data(&self) -> &[U256] {
        &self.data
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_push_pop() {
        let mut stack = Stack::new();
        
        stack.push(U256::from(42)).unwrap();
        stack.push(U256::from(100)).unwrap();
        
        assert_eq!(stack.pop().unwrap(), U256::from(100));
        assert_eq!(stack.pop().unwrap(), U256::from(42));
    }
    
    #[test]
    fn test_peek() {
        let mut stack = Stack::new();
        stack.push(U256::from(42)).unwrap();
        
        assert_eq!(stack.peek().unwrap(), U256::from(42));
        assert_eq!(stack.len(), 1);
    }
    
    #[test]
    fn test_dup() {
        let mut stack = Stack::new();
        stack.push(U256::from(1)).unwrap();
        stack.push(U256::from(2)).unwrap();
        
        stack.dup(0).unwrap(); // Duplicate top
        assert_eq!(stack.pop().unwrap(), U256::from(2));
        assert_eq!(stack.pop().unwrap(), U256::from(2));
    }
    
    #[test]
    fn test_swap() {
        let mut stack = Stack::new();
        stack.push(U256::from(1)).unwrap();
        stack.push(U256::from(2)).unwrap();
        
        stack.swap(1).unwrap();
        assert_eq!(stack.pop().unwrap(), U256::from(1));
        assert_eq!(stack.pop().unwrap(), U256::from(2));
    }
    
    #[test]
    fn test_underflow() {
        let mut stack = Stack::new();
        assert!(stack.pop().is_err());
    }
}
