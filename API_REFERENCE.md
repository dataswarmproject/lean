# LEAN CHAIN API Reference

## Overview

LEAN CHAIN provides multiple interfaces for interacting with the blockchain:
- **JSON-RPC API** - HTTP/WebSocket interface
- **CLI** - Command-line interface
- **SDKs** - Language-specific libraries (coming soon)

## Table of Contents

- [JSON-RPC API](#json-rpc-api)
- [CLI Reference](#cli-reference)
- [Smart Contract Integration](#smart-contract-integration)
- [DeFi Protocol APIs](#defi-protocol-apis)
- [Code Examples](#code-examples)

## JSON-RPC API

### Connection

**HTTP:**
```
POST http://localhost:26657
Content-Type: application/json
```

**WebSocket:**
```
ws://localhost:26657/websocket
```

### Chain Methods

#### `chain_getBlockByNumber`

Get block by height.

**Parameters:**
- `height` (number): Block height
- `include_txs` (boolean): Include full transactions

**Example:**
```json
{
  "jsonrpc": "2.0",
  "method": "chain_getBlockByNumber",
  "params": [100, true],
  "id": 1
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "height": 100,
    "hash": "0x...",
    "parent_hash": "0x...",
    "timestamp": 1704067200,
    "proposer": "0x...",
    "transactions": [...]
  },
  "id": 1
}
```

#### `chain_getLatestBlock`

Get latest block.

**Example:**
```json
{
  "jsonrpc": "2.0",
  "method": "chain_getLatestBlock",
  "params": [],
  "id": 1
}
```

#### `chain_getBlockByHash`

Get block by hash.

**Parameters:**
- `hash` (string): Block hash

### Transaction Methods

#### `tx_sendTransaction`

Submit a transaction.

**Parameters:**
- `from` (address): Sender address
- `to` (address): Recipient address
- `amount` (string): Amount in wei
- `data` (hex): Optional transaction data
- `gas_price` (string): Gas price
- `gas_limit` (string): Gas limit

**Example:**
```json
{
  "jsonrpc": "2.0",
  "method": "tx_sendTransaction",
  "params": [{
    "from": "0x...",
    "to": "0x...",
    "amount": "1000000000000000000",
    "gas_price": "1000000000",
    "gas_limit": "21000"
  }],
  "id": 1
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "tx_hash": "0x..."
  },
  "id": 1
}
```

#### `tx_getTransaction`

Get transaction by hash.

**Parameters:**
- `hash` (string): Transaction hash

**Example:**
```json
{
  "jsonrpc": "2.0",
  "method": "tx_getTransaction",
  "params": ["0x..."],
  "id": 1
}
```

#### `tx_getTransactionReceipt`

Get transaction receipt.

**Parameters:**
- `hash` (string): Transaction hash

**Response includes:**
- `status` (number): 1 = success, 0 = failure
- `gas_used` (string): Gas consumed
- `logs` (array): Event logs

### Account Methods

#### `account_getBalance`

Get account balance.

**Parameters:**
- `address` (address): Account address
- `block` (string): Block height or "latest"

**Example:**
```json
{
  "jsonrpc": "2.0",
  "method": "account_getBalance",
  "params": ["0x...", "latest"],
  "id": 1
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": "1000000000000000000000",
  "id": 1
}
```

#### `account_getNonce`

Get account nonce.

**Parameters:**
- `address` (address): Account address

#### `account_getCode`

Get contract code.

**Parameters:**
- `address` (address): Contract address

### State Methods

#### `state_call`

Call contract method (read-only).

**Parameters:**
- `to` (address): Contract address
- `data` (hex): Encoded function call
- `block` (string): Block number or "latest"

**Example:**
```json
{
  "jsonrpc": "2.0",
  "method": "state_call",
  "params": [{
    "to": "0x...",
    "data": "0x70a08231000000000000000000000000..."
  }, "latest"],
  "id": 1
}
```

#### `state_getStorageAt`

Get contract storage.

**Parameters:**
- `address` (address): Contract address
- `position` (string): Storage slot
- `block` (string): Block number

### Validator Methods

#### `validator_getValidators`

Get active validator set.

**Example:**
```json
{
  "jsonrpc": "2.0",
  "method": "validator_getValidators",
  "params": [],
  "id": 1
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "validators": [
      {
        "address": "0x...",
        "voting_power": "50000000000000000000000",
        "proposer_priority": 1000
      }
    ]
  },
  "id": 1
}
```

#### `validator_getValidator`

Get validator info.

**Parameters:**
- `address` (address): Validator address

### Subscription Methods (WebSocket)

#### `subscribe`

Subscribe to events.

**Parameters:**
- `query` (string): Event query

**Example:**
```json
{
  "jsonrpc": "2.0",
  "method": "subscribe",
  "params": ["tm.event='NewBlock'"],
  "id": 1
}
```

**Events:**
- `tm.event='NewBlock'` - New blocks
- `tm.event='Tx'` - New transactions
- `tm.event='ValidatorSetUpdates'` - Validator changes

## CLI Reference

### Wallet Commands

#### `lean-cli wallet new`

Create new wallet.

```bash
lean-cli wallet new --output wallet.json
```

#### `lean-cli wallet address`

Show wallet address.

```bash
lean-cli wallet address
```

#### `lean-cli wallet balance`

Check balance.

```bash
lean-cli wallet balance <address>
```

### Transaction Commands

#### `lean-cli tx send`

Send tokens.

```bash
lean-cli tx send \
  --to 0x... \
  --amount 1000000000000000000 \
  --gas-price 1000000000 \
  --gas-limit 21000
```

#### `lean-cli tx status`

Check transaction status.

```bash
lean-cli tx status <tx-hash>
```

### Validator Commands

#### `lean-cli validator create`

Create validator.

```bash
lean-cli validator create \
  --moniker "My Validator" \
  --commission-rate 0.10 \
  --commission-max-rate 0.20 \
  --commission-max-change 0.01 \
  --min-self-delegation 10000000000000000000000
```

#### `lean-cli validator delegate`

Delegate stake.

```bash
lean-cli validator delegate \
  --validator 0x... \
  --amount 1000000000000000000000
```

#### `lean-cli validator status`

Check validator status.

```bash
lean-cli validator status <validator-address>
```

### Query Commands

#### `lean-cli query block`

Query block.

```bash
lean-cli query block <height>
```

#### `lean-cli query tx`

Query transaction.

```bash
lean-cli query tx <hash>
```

#### `lean-cli query account`

Query account.

```bash
lean-cli query account <address>
```

## Smart Contract Integration

### Deploying Contracts

```bash
# Compile Solidity contract
solc --bin --abi Contract.sol -o build/

# Deploy contract
lean-cli contract deploy \
  --bytecode $(cat build/Contract.bin) \
  --gas-limit 5000000
```

### Calling Contracts

```bash
# Call contract method
lean-cli contract call \
  --to 0x... \
  --method "transfer(address,uint256)" \
  --args "0x...,1000000000000000000" \
  --gas-limit 100000
```

### Events

```bash
# Query contract events
lean-cli contract events \
  --address 0x... \
  --from-block 1000 \
  --to-block 2000
```

## DeFi Protocol APIs

### Lending Protocol

#### Supply Liquidity

```bash
lean-cli lending supply \
  --asset 0x... \
  --amount 1000000000000000000
```

#### Borrow

```bash
lean-cli lending borrow \
  --asset 0x... \
  --amount 500000000000000000 \
  --collateral 0x... \
  --collateral-amount 1500000000000000000
```

#### Repay

```bash
lean-cli lending repay \
  --loan-id 0x... \
  --amount 500000000000000000
```

### Stablecoin (Vault)

#### Create Vault

```bash
lean-cli stablecoin create-vault \
  --collateral-type 0x... \
  --collateral-amount 2000000000000000000 \
  --debt-amount 1000000000000000000
```

#### Mint

```bash
lean-cli stablecoin mint \
  --vault-id 0x... \
  --amount 100000000000000000
```

### DEX

#### Add Liquidity

```bash
lean-cli dex add-liquidity \
  --token-a 0x... \
  --token-b 0x... \
  --amount-a 1000000000000000000 \
  --amount-b 2000000000000000000
```

#### Swap

```bash
lean-cli dex swap \
  --token-in 0x... \
  --token-out 0x... \
  --amount-in 1000000000000000000 \
  --min-amount-out 1950000000000000000
```

### NFT

#### Mint NFT

```bash
lean-cli nft mint \
  --to 0x... \
  --token-id 1 \
  --metadata '{"name":"NFT #1","image":"..."}'
```

#### Transfer NFT

```bash
lean-cli nft transfer \
  --to 0x... \
  --token-id 1
```

## Code Examples

### JavaScript (Web3.js)

```javascript
const Web3 = require('web3');
const web3 = new Web3('http://localhost:26657');

// Get balance
const balance = await web3.eth.getBalance('0x...');
console.log(web3.utils.fromWei(balance, 'ether'));

// Send transaction
const tx = await web3.eth.sendTransaction({
  from: '0x...',
  to: '0x...',
  value: web3.utils.toWei('1', 'ether'),
  gas: 21000
});

// Call contract
const contract = new web3.eth.Contract(abi, address);
const result = await contract.methods.balanceOf('0x...').call();
```

### Python

```python
from web3 import Web3

# Connect
w3 = Web3(Web3.HTTPProvider('http://localhost:26657'))

# Check connection
print(w3.isConnected())

# Get balance
balance = w3.eth.get_balance('0x...')
print(w3.fromWei(balance, 'ether'))

# Send transaction
tx = {
    'from': '0x...',
    'to': '0x...',
    'value': w3.toWei(1, 'ether'),
    'gas': 21000,
    'gasPrice': w3.toWei('10', 'gwei')
}
signed = w3.eth.account.sign_transaction(tx, private_key)
tx_hash = w3.eth.send_raw_transaction(signed.rawTransaction)
```

### Rust

```rust
use lean_sdk::Client;

#[tokio::main]
async fn main() {
    // Connect to node
    let client = Client::new("http://localhost:26657").unwrap();
    
    // Get balance
    let balance = client.get_balance("0x...").await.unwrap();
    println!("Balance: {}", balance);
    
    // Send transaction
    let tx = client.build_transaction()
        .to("0x...")
        .amount(1_000_000_000_000_000_000u128)
        .build();
    
    let hash = client.send_transaction(tx).await.unwrap();
    println!("TX Hash: {}", hash);
}
```

## Error Codes

| Code | Message | Description |
|------|---------|-------------|
| -32700 | Parse error | Invalid JSON |
| -32600 | Invalid request | Invalid request object |
| -32601 | Method not found | Method doesn't exist |
| -32602 | Invalid params | Invalid parameters |
| -32603 | Internal error | Server error |
| 1001 | Insufficient balance | Account balance too low |
| 1002 | Invalid nonce | Nonce mismatch |
| 1003 | Gas too low | Insufficient gas |
| 1004 | Transaction failed | Execution failure |

## Rate Limits

**Public RPC:**
- 100 requests/minute per IP
- 10 WebSocket connections per IP

**Authenticated:**
- Contact for higher limits

## Additional Resources

- **Interactive API Explorer:** https://api.leanchain.io
- **Postman Collection:** https://github.com/dataswarmproject/lean-api
- **SDKs:** https://github.com/dataswarmproject/lean-sdks

---

**Need help? Join our Discord: https://discord.gg/leanchain**
