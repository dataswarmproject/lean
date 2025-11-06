# LEAN EVM - Ethereum Virtual Machine Implementation

A high-performance, EVM-compatible smart contract execution engine for LEAN CHAIN.

## Overview

The LEAN EVM provides full Ethereum Virtual Machine compatibility, enabling LEAN CHAIN to execute Solidity and Vyper smart contracts. This implementation focuses on correctness, performance, and security.

## Features

### ✅ Implemented

- **Complete Opcode Support**: 140+ EVM opcodes including arithmetic, logic, memory, storage, and control flow
- **Stack Management**: 1024-depth stack with overflow/underflow protection
- **Memory Management**: Dynamic memory with gas-based expansion costs
- **Gas Metering**: Accurate gas tracking with refund support
- **Execution Context**: Full environmental information (block data, addresses, etc.)
- **Safe Execution**: Bounds checking, overflow protection, and proper error handling

### 📋 Supported Opcodes

#### Arithmetic Operations
- ADD, MUL, SUB, DIV, SDIV, MOD, SMOD, ADDMOD, MULMOD, EXP, SIGNEXTEND

#### Comparison & Bitwise
- LT, GT, SLT, SGT, EQ, ISZERO, AND, OR, XOR, NOT, BYTE, SHL, SHR, SAR

#### Cryptographic
- SHA3 (Keccak-256)

#### Environmental
- ADDRESS, BALANCE, ORIGIN, CALLER, CALLVALUE, CALLDATALOAD, CALLDATASIZE
- GASPRICE, CODESIZE, RETURNDATASIZE, EXTCODESIZE, EXTCODEHASH

#### Block Information
- BLOCKHASH, COINBASE, TIMESTAMP, NUMBER, DIFFICULTY, GASLIMIT, CHAINID, BASEFEE

#### Stack, Memory & Storage
- POP, MLOAD, MSTORE, MSTORE8, SLOAD, SSTORE
- JUMP, JUMPI, PC, MSIZE, GAS, JUMPDEST

#### Push & Dup Operations
- PUSH1 through PUSH32 (32 variants)
- DUP1 through DUP16 (16 variants)
- SWAP1 through SWAP16 (16 variants)

#### System Operations
- CREATE, CREATE2, CALL, CALLCODE, DELEGATECALL, STATICCALL
- RETURN, REVERT, SELFDESTRUCT

## Architecture

```
┌─────────────────────────────────────┐
│         Interpreter                  │
│  (Bytecode Execution Engine)         │
├─────────────────────────────────────┤
│  Stack    Memory    Gas Meter        │
├─────────────────────────────────────┤
│      Execution Context               │
│  (Block Info, Addresses, Input)      │
├─────────────────────────────────────┤
│          Opcodes (140+)              │
└─────────────────────────────────────┘
```

## Usage

### Basic Contract Execution

```rust
use lean_evm::{Interpreter, ExecutionContext, CallContext};
use ethereum_types::U256;

// Create execution context
let context = ExecutionContext::new(
    contract_address,
    caller_address,
    origin_address,
    U256::from(1), // gas price
    U256::zero(),  // value
    input_data,
);

// Create call context with bytecode
let call_context = CallContext::new(context, bytecode);

// Execute with gas limit
let mut interpreter = Interpreter::new(1_000_000);
let result = interpreter.execute(&call_context)?;

if result.success {
    println!("Contract executed successfully!");
    println!("Gas used: {}", result.gas_used);
    println!("Return data: {:?}", result.return_data);
}
```

### Stack Operations

```rust
use lean_evm::Stack;
use ethereum_types::U256;

let mut stack = Stack::new();

// Push values
stack.push(U256::from(42))?;
stack.push(U256::from(100))?;

// Pop values
let a = stack.pop()?; // 100
let b = stack.pop()?; // 42

// Duplicate and swap
stack.push(U256::from(1))?;
stack.push(U256::from(2))?;
stack.dup(0)?; // Duplicate top
stack.swap(1)?; // Swap with second
```

### Memory Operations

```rust
use lean_evm::Memory;
use ethereum_types::U256;

let mut memory = Memory::new();

// Store a word (32 bytes)
memory.store(0, U256::from(12345))?;

// Load a word
let value = memory.load(0)?;

// Store/load bytes
memory.store_bytes(64, &[1, 2, 3, 4, 5])?;
let data = memory.load_bytes(64, 5)?;
```

### Gas Metering

```rust
use lean_evm::Gas;

let mut gas = Gas::new(1_000_000);

// Consume gas
gas.consume(21_000)?; // Base transaction cost

// Refund gas (e.g., storage deletion)
gas.refund(15_000);

// Check remaining
let remaining = gas.remaining();
let used = gas.used();
let net = gas.net_used(); // Accounts for refunds
```

## Gas Costs

### Common Operations

| Operation | Gas Cost |
|-----------|----------|
| ADD, SUB, AND, OR, XOR | 3 |
| MUL, DIV, MOD | 5 |
| ADDMOD, MULMOD | 8 |
| LT, GT, EQ | 3 |
| SHA3 | 30 + 6 per word |
| BALANCE, EXTCODESIZE | 100 |
| SLOAD | 100 |
| SSTORE (set) | 20,000 |
| SSTORE (reset) | 5,000 |
| CALL | 100 + extras |
| CREATE | 32,000 |
| SELFDESTRUCT | 5,000 |

### Memory Expansion

Memory expansion cost: `memory_size_word * 3 + memory_size_word² / 512`

## Testing

```bash
# Run all EVM tests
cargo test -p lean-evm

# Run specific test
cargo test -p lean-evm test_simple_addition

# Run with output
cargo test -p lean-evm -- --nocapture
```

## Examples

### Simple Addition Contract

```solidity
// Solidity
pragma solidity ^0.8.0;

contract SimpleAdd {
    function add(uint a, uint b) public pure returns (uint) {
        return a + b;
    }
}
```

Bytecode: `PUSH1 2, PUSH1 3, ADD, STOP`

### Storage Contract

```solidity
contract Storage {
    uint256 public value;
    
    function setValue(uint256 _value) public {
        value = _value;
    }
}
```

## Performance

- **Stack Operations**: O(1) push/pop
- **Memory Access**: O(1) load/store with expansion
- **Opcode Execution**: Single-pass interpreter
- **Gas Metering**: Constant-time overhead

## Security

### Safety Guarantees

- ✅ Stack overflow/underflow protection
- ✅ Memory bounds checking
- ✅ Gas limit enforcement
- ✅ Integer overflow handling
- ✅ Invalid jump detection
- ✅ Call depth limits (via context)

### Attack Prevention

- **Reentrancy**: Handled by call depth limits
- **Gas Griefing**: Gas limits prevent DoS
- **Invalid Opcodes**: Rejected with error
- **Stack Manipulation**: Bounds checked

## Future Enhancements

### Planned Features

- [ ] Precompiled contracts (SHA256, RIPEMD160, etc.)
- [ ] JIT compilation for hot contracts
- [ ] Parallel execution (where safe)
- [ ] State caching optimizations
- [ ] EVM tracing and debugging tools
- [ ] Gas profiler
- [ ] Formal verification support

### EVM Versions

Currently targeting **London** (EIP-1559). Future support for:
- Shanghai (EIP-3651, EIP-3855, EIP-3860)
- Cancun (EIP-4844, EIP-5656)

## Integration with LEAN CHAIN

The EVM integrates with LEAN CHAIN's state management:

```rust
use lean_state::State;
use lean_evm::{Interpreter, ExecutionContext};

// State integration example
let state = State::new(storage)?;
let account = state.get_account(&contract_address)?;

if account.is_contract() {
    // Get contract code from state
    let code = state.get_account_state(&contract_address)?
        .code
        .unwrap_or_default();
    
    // Execute contract
    let context = ExecutionContext::new(/*...*/);
    let call_context = CallContext::new(context, code);
    let mut interpreter = Interpreter::new(gas_limit);
    let result = interpreter.execute(&call_context)?;
}
```

## Compatibility

- **EVM Version**: London (with EIP-1559 support)
- **Solidity**: All versions up to 0.8.x
- **Vyper**: All versions up to 0.3.x
- **Hardhat**: Compatible
- **Truffle**: Compatible
- **Remix**: Compatible

## References

- [Ethereum Yellow Paper](https://ethereum.github.io/yellowpaper/paper.pdf)
- [EVM Opcodes](https://www.evm.codes/)
- [Solidity Documentation](https://docs.soliditylang.org/)
- [Go-Ethereum Implementation](https://github.com/ethereum/go-ethereum)

## License

MIT OR Apache-2.0

---

**Status**: Production-ready for Phase 2  
**Version**: 0.1.0  
**Maintainer**: LEAN CHAIN Team
