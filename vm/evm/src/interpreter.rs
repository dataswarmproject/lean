//! EVM interpreter - executes bytecode

use crate::{Stack, Memory, Gas, OpCode, CallContext, EvmError, Result};
use ethereum_types::U256;
use lean_crypto::Hash256;

/// Interpreter result
#[derive(Debug, Clone)]
pub struct InterpreterResult {
    /// Success status
    pub success: bool,
    
    /// Gas used
    pub gas_used: u64,
    
    /// Return data
    pub return_data: Vec<u8>,
    
    /// Logs emitted
    pub logs: Vec<Log>,
    
    /// Reason for failure (if any)
    pub revert_reason: Option<String>,
}

/// Event log
#[derive(Debug, Clone)]
pub struct Log {
    /// Contract address
    pub address: lean_crypto::Address,
    
    /// Topics (indexed parameters)
    pub topics: Vec<Hash256>,
    
    /// Data (non-indexed parameters)
    pub data: Vec<u8>,
}

/// EVM interpreter
pub struct Interpreter {
    /// Program counter
    pc: usize,
    
    /// Stack
    stack: Stack,
    
    /// Memory
    memory: Memory,
    
    /// Gas meter
    gas: Gas,
    
    /// Return data from last call
    return_data: Vec<u8>,
    
    /// Logs
    logs: Vec<Log>,
    
    /// Stopped flag
    stopped: bool,
    
    /// Reverted flag
    reverted: bool,
}

impl Interpreter {
    /// Create a new interpreter
    pub fn new(gas_limit: u64) -> Self {
        Self {
            pc: 0,
            stack: Stack::new(),
            memory: Memory::new(),
            gas: Gas::new(gas_limit),
            return_data: Vec::new(),
            logs: Vec::new(),
            stopped: false,
            reverted: false,
        }
    }
    
    /// Execute contract code
    pub fn execute(&mut self, context: &CallContext) -> Result<InterpreterResult> {
        let code = &context.code;
        
        while self.pc < code.len() && !self.stopped && !self.reverted {
            let opcode = OpCode(code[self.pc]);
            
            // Consume gas for opcode
            self.gas.consume(opcode.gas_cost())?;
            
            // Execute opcode
            self.execute_opcode(opcode, code, context)?;
            
            if !self.stopped && !self.reverted {
                self.pc += 1;
            }
        }
        
        Ok(InterpreterResult {
            success: !self.reverted,
            gas_used: self.gas.used(),
            return_data: self.return_data.clone(),
            logs: self.logs.clone(),
            revert_reason: if self.reverted {
                Some("Execution reverted".to_string())
            } else {
                None
            },
        })
    }
    
    /// Execute a single opcode
    fn execute_opcode(&mut self, opcode: OpCode, code: &[u8], context: &CallContext) -> Result<()> {
        match opcode.0 {
            // STOP
            0x00 => {
                self.stopped = true;
            }
            
            // ADD
            0x01 => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(a.overflowing_add(b).0)?;
            }
            
            // MUL
            0x02 => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(a.overflowing_mul(b).0)?;
            }
            
            // SUB
            0x03 => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(a.overflowing_sub(b).0)?;
            }
            
            // DIV
            0x04 => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                if b.is_zero() {
                    self.stack.push(U256::zero())?;
                } else {
                    self.stack.push(a / b)?;
                }
            }
            
            // MOD
            0x06 => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                if b.is_zero() {
                    self.stack.push(U256::zero())?;
                } else {
                    self.stack.push(a % b)?;
                }
            }
            
            // LT
            0x10 => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(if a < b { U256::one() } else { U256::zero() })?;
            }
            
            // GT
            0x11 => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(if a > b { U256::one() } else { U256::zero() })?;
            }
            
            // EQ
            0x14 => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(if a == b { U256::one() } else { U256::zero() })?;
            }
            
            // ISZERO
            0x15 => {
                let a = self.stack.pop()?;
                self.stack.push(if a.is_zero() { U256::one() } else { U256::zero() })?;
            }
            
            // AND
            0x16 => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(a & b)?;
            }
            
            // OR
            0x17 => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(a | b)?;
            }
            
            // XOR
            0x18 => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                self.stack.push(a ^ b)?;
            }
            
            // NOT
            0x19 => {
                let a = self.stack.pop()?;
                self.stack.push(!a)?;
            }
            
            // ADDRESS
            0x30 => {
                let addr_bytes = context.context.address.as_bytes();
                let mut bytes = [0u8; 32];
                bytes[12..].copy_from_slice(addr_bytes);
                self.stack.push(U256::from_big_endian(&bytes))?;
            }
            
            // CALLER
            0x33 => {
                let addr_bytes = context.context.caller.as_bytes();
                let mut bytes = [0u8; 32];
                bytes[12..].copy_from_slice(addr_bytes);
                self.stack.push(U256::from_big_endian(&bytes))?;
            }
            
            // CALLVALUE
            0x34 => {
                self.stack.push(context.context.value)?;
            }
            
            // CALLDATALOAD
            0x35 => {
                let offset = self.stack.pop()?.as_usize();
                let mut data = [0u8; 32];
                let input = &context.context.input;
                
                for i in 0..32 {
                    if offset + i < input.len() {
                        data[i] = input[offset + i];
                    }
                }
                
                self.stack.push(U256::from_big_endian(&data))?;
            }
            
            // CALLDATASIZE
            0x36 => {
                self.stack.push(U256::from(context.context.input.len()))?;
            }
            
            // POP
            0x50 => {
                self.stack.pop()?;
            }
            
            // MLOAD
            0x51 => {
                let offset = self.stack.pop()?.as_usize();
                let value = self.memory.load(offset)?;
                self.stack.push(value)?;
            }
            
            // MSTORE
            0x52 => {
                let offset = self.stack.pop()?.as_usize();
                let value = self.stack.pop()?;
                self.memory.store(offset, value)?;
            }
            
            // MSTORE8
            0x53 => {
                let offset = self.stack.pop()?.as_usize();
                let value = self.stack.pop()?;
                self.memory.store_byte(offset, value.byte(0))?;
            }
            
            // JUMP
            0x56 => {
                let dest = self.stack.pop()?.as_usize();
                if dest >= code.len() || code[dest] != 0x5b {
                    return Err(EvmError::InvalidJump);
                }
                self.pc = dest;
                return Ok(()); // Don't increment PC
            }
            
            // JUMPI
            0x57 => {
                let dest = self.stack.pop()?.as_usize();
                let condition = self.stack.pop()?;
                
                if !condition.is_zero() {
                    if dest >= code.len() || code[dest] != 0x5b {
                        return Err(EvmError::InvalidJump);
                    }
                    self.pc = dest;
                    return Ok(()); // Don't increment PC
                }
            }
            
            // PC
            0x58 => {
                self.stack.push(U256::from(self.pc))?;
            }
            
            // MSIZE
            0x59 => {
                self.stack.push(U256::from(self.memory.len()))?;
            }
            
            // JUMPDEST
            0x5b => {
                // Just a marker, no operation
            }
            
            // PUSH1-PUSH32
            0x60..=0x7f => {
                let n = (opcode.0 - 0x5f) as usize;
                let mut bytes = [0u8; 32];
                
                for i in 0..n {
                    if self.pc + 1 + i < code.len() {
                        bytes[32 - n + i] = code[self.pc + 1 + i];
                    }
                }
                
                self.stack.push(U256::from_big_endian(&bytes))?;
                self.pc += n;
            }
            
            // DUP1-DUP16
            0x80..=0x8f => {
                let n = (opcode.0 - 0x7f) as usize;
                self.stack.dup(n - 1)?;
            }
            
            // SWAP1-SWAP16
            0x90..=0x9f => {
                let n = (opcode.0 - 0x8f) as usize;
                self.stack.swap(n)?;
            }
            
            // RETURN
            0xf3 => {
                let offset = self.stack.pop()?.as_usize();
                let size = self.stack.pop()?.as_usize();
                self.return_data = self.memory.load_bytes(offset, size)?;
                self.stopped = true;
            }
            
            // REVERT
            0xfd => {
                let offset = self.stack.pop()?.as_usize();
                let size = self.stack.pop()?.as_usize();
                self.return_data = self.memory.load_bytes(offset, size)?;
                self.reverted = true;
            }
            
            _ => {
                // Unsupported opcode - simplified implementation
                return Err(EvmError::InvalidOpcode(opcode.0));
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lean_crypto::KeyPair;
    use crate::ExecutionContext;
    
    #[test]
    fn test_simple_addition() {
        // PUSH1 2, PUSH1 3, ADD, STOP
        let code = vec![0x60, 0x02, 0x60, 0x03, 0x01, 0x00];
        
        let address = lean_crypto::Address::from_public_key(&KeyPair::generate().public_key());
        let ctx = ExecutionContext::new(
            address,
            address,
            address,
            U256::one(),
            U256::zero(),
            vec![],
        );
        
        let call_ctx = CallContext::new(ctx, code);
        let mut interpreter = Interpreter::new(100000);
        
        let result = interpreter.execute(&call_ctx).unwrap();
        assert!(result.success);
    }
    
    #[test]
    fn test_return() {
        // PUSH1 42, PUSH1 0, MSTORE, PUSH1 32, PUSH1 0, RETURN
        let code = vec![
            0x60, 0x2a,       // PUSH1 42
            0x60, 0x00,       // PUSH1 0
            0x52,             // MSTORE
            0x60, 0x20,       // PUSH1 32
            0x60, 0x00,       // PUSH1 0
            0xf3,             // RETURN
        ];
        
        let address = lean_crypto::Address::from_public_key(&KeyPair::generate().public_key());
        let ctx = ExecutionContext::new(
            address,
            address,
            address,
            U256::one(),
            U256::zero(),
            vec![],
        );
        
        let call_ctx = CallContext::new(ctx, code);
        let mut interpreter = Interpreter::new(100000);
        
        let result = interpreter.execute(&call_ctx).unwrap();
        assert!(result.success);
        assert_eq!(result.return_data.len(), 32);
    }
}
