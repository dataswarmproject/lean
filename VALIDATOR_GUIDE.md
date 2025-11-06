# LEAN CHAIN Validator Guide

## Overview

This guide covers everything you need to know about running a validator on LEAN CHAIN.

## Table of Contents

- [Requirements](#requirements)
- [Quick Start](#quick-start)
- [Detailed Setup](#detailed-setup)
- [Validator Operations](#validator-operations)
- [Security Best Practices](#security-best-practices)
- [Monitoring](#monitoring)
- [Troubleshooting](#troubleshooting)
- [Economics](#economics)

## Requirements

### Hardware Requirements

**Minimum:**
- CPU: 4 cores
- RAM: 8 GB
- Storage: 500 GB SSD
- Network: 100 Mbps

**Recommended:**
- CPU: 8 cores
- RAM: 16 GB
- Storage: 1 TB NVMe SSD
- Network: 1 Gbps

### Software Requirements

- Linux (Ubuntu 20.04+ recommended)
- Rust 1.70+
- Git
- Build essentials

### Stake Requirements

- **Minimum Stake:** 10,000 LEAN tokens
- **Recommended Stake:** 50,000+ LEAN tokens
- Higher stake increases validator selection probability

## Quick Start

### 1. Install Dependencies

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install build essentials
sudo apt install -y build-essential pkg-config libssl-dev

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Clone and Build

```bash
# Clone repository
git clone https://github.com/dataswarmproject/lean.git
cd lean

# Build release binaries
cargo build --release

# Install binaries
sudo cp target/release/lean-validator /usr/local/bin/
sudo cp target/release/lean-cli /usr/local/bin/
```

### 3. Run Setup Script

```bash
# Run automated setup
./deploy/scripts/setup-validator.sh

# Or with custom moniker
MONIKER="my-validator" ./deploy/scripts/setup-validator.sh
```

### 4. Fund Validator

```bash
# Get validator address
lean-cli wallet address

# Fund with minimum stake (use faucet or transfer)
# Minimum: 10,000 LEAN + fees
```

### 5. Create Validator

```bash
# Create validator transaction
lean-cli validator create \
  --moniker "My Validator" \
  --commission-rate 0.10 \
  --commission-max-rate 0.20 \
  --commission-max-change 0.01 \
  --min-self-delegation 10000000000000000000000

# Check validator status
lean-cli validator status
```

### 6. Start Validator

```bash
# Start service
sudo systemctl start lean-validator

# Enable auto-start
sudo systemctl enable lean-validator

# Check logs
journalctl -u lean-validator -f
```

## Detailed Setup

### Directory Structure

```
~/.lean/
├── config/
│   ├── genesis.json        # Genesis configuration
│   ├── config.toml         # Node configuration
│   └── validator_key.json  # Validator private key (BACKUP!)
├── data/
│   ├── blockchain/         # Blockchain data
│   └── state/              # State database
└── logs/
    └── lean.log            # Application logs
```

### Configuration Files

#### config.toml

```toml
[node]
moniker = "my-validator"
chain_id = "lean-mainnet-1"

[p2p]
listen_addr = "0.0.0.0:26656"
external_address = "your.public.ip:26656"
persistent_peers = "peer1@ip1:26656,peer2@ip2:26656"
max_num_inbound_peers = 40
max_num_outbound_peers = 10

[rpc]
listen_addr = "127.0.0.1:26657"
cors_allowed_origins = ["*"]

[consensus]
timeout_propose = 2000
timeout_prevote = 1000
timeout_precommit = 1000
timeout_commit = 2000

[mempool]
size = 5000
cache_size = 10000

[logging]
level = "info"
format = "json"
output = "file"
```

### Firewall Configuration

```bash
# Allow P2P
sudo ufw allow 26656/tcp

# Allow RPC (only from trusted IPs)
sudo ufw allow from 192.168.1.0/24 to any port 26657

# Enable firewall
sudo ufw enable
```

## Validator Operations

### Creating a Validator

```bash
lean-cli validator create \
  --moniker "My Validator" \
  --identity "keybase-id" \
  --website "https://myvalidator.com" \
  --details "Professional validator services" \
  --commission-rate 0.10 \
  --commission-max-rate 0.20 \
  --commission-max-change 0.01 \
  --min-self-delegation 10000000000000000000000
```

### Editing Validator Info

```bash
lean-cli validator edit \
  --moniker "New Name" \
  --website "https://newsite.com" \
  --details "Updated description"
```

### Delegating Stake

```bash
# Self-delegation
lean-cli validator delegate \
  --validator <validator-address> \
  --amount 1000000000000000000000

# Check delegation
lean-cli validator delegation <validator-address>
```

### Withdrawing Rewards

```bash
# Withdraw commission
lean-cli validator withdraw-commission

# Withdraw rewards
lean-cli validator withdraw-rewards
```

### Unbonding

```bash
# Unbond stake
lean-cli validator unbond \
  --validator <validator-address> \
  --amount 1000000000000000000000

# Check unbonding status
lean-cli validator unbonding-delegations
```

### Unjailing

```bash
# If validator is jailed for downtime
lean-cli validator unjail
```

## Security Best Practices

### Key Management

1. **Backup validator_key.json**
   - Store in multiple secure locations
   - Use encrypted storage
   - Never share or commit to version control

2. **Use Hardware Security Module (HSM)**
   - YubiHSM2 recommended
   - Protects against key theft
   - Enables signing without exposing private key

3. **Key Security Checklist**
   - ✅ Backed up to encrypted location
   - ✅ Stored offline (cold storage)
   - ✅ Access restricted (file permissions 600)
   - ✅ Regular backup verification
   - ✅ HSM integration (for production)

### Node Security

1. **Firewall Configuration**
   ```bash
   # Only allow necessary ports
   sudo ufw default deny incoming
   sudo ufw default allow outgoing
   sudo ufw allow ssh
   sudo ufw allow 26656/tcp  # P2P only
   sudo ufw enable
   ```

2. **SSH Hardening**
   ```bash
   # Disable password auth
   sudo sed -i 's/#PasswordAuthentication yes/PasswordAuthentication no/' /etc/ssh/sshd_config
   sudo systemctl restart sshd
   
   # Use SSH keys only
   # Change default SSH port
   ```

3. **DDoS Protection**
   - Use Sentry nodes
   - Configure rate limiting
   - Monitor connection attempts

### Sentry Node Architecture

```
Internet
    │
    ├── Sentry Node 1 (Public)
    ├── Sentry Node 2 (Public)
    └── Sentry Node 3 (Public)
            │
            └── Validator Node (Private)
```

**Benefits:**
- Validator IP hidden
- DDoS protection
- Zero downtime during attacks

**Setup:**
1. Deploy 2-3 sentry nodes with public IPs
2. Configure validator to only connect to sentries
3. Configure sentries to forward to validator

## Monitoring

### Health Checks

```bash
# Check node status
lean-cli node status

# Check validator status
lean-cli validator status

# Check sync status
lean-cli node sync-status

# Check peer count
lean-cli node peers
```

### Prometheus Metrics

Add to config.toml:
```toml
[instrumentation]
prometheus = true
prometheus_listen_addr = "127.0.0.1:26660"
```

**Key Metrics:**
- `lean_validator_missed_blocks`
- `lean_validator_power`
- `lean_node_peers`
- `lean_mempool_size`
- `lean_consensus_height`

### Alerting

Set up alerts for:
- ⚠️ Missed blocks > 5%
- ⚠️ Node offline > 1 minute
- ⚠️ Disk usage > 80%
- ⚠️ Memory usage > 90%
- ⚠️ Peer count < 3

### Log Monitoring

```bash
# Real-time logs
journalctl -u lean-validator -f

# Error logs only
journalctl -u lean-validator -p err -f

# Last 100 lines
journalctl -u lean-validator -n 100
```

## Troubleshooting

### Common Issues

#### Node Won't Start

```bash
# Check logs
journalctl -u lean-validator -n 50

# Common causes:
# - Port already in use
# - Incorrect genesis file
# - Corrupted database
# - Insufficient permissions
```

#### Node Not Syncing

```bash
# Check peers
lean-cli node peers

# Add persistent peers in config.toml
# Restart node
sudo systemctl restart lean-validator
```

#### Validator Not Producing Blocks

```bash
# Check validator status
lean-cli validator status

# Possible causes:
# - Insufficient stake
# - Jailed for downtime
# - Wrong validator key
# - Not in active set
```

#### High Memory Usage

```bash
# Check memory
free -h

# Reduce cache size in config.toml:
[mempool]
cache_size = 5000  # Reduce from 10000
```

#### Disk Space Issues

```bash
# Check disk usage
df -h

# Prune old data
lean-cli node prune --keep-recent 100000
```

### Getting Help

- **Discord:** https://discord.gg/leanchain
- **Forum:** https://forum.leanchain.io
- **GitHub Issues:** https://github.com/dataswarmproject/lean/issues
- **Documentation:** https://docs.leanchain.io

## Economics

### Validator Rewards

**Block Rewards:**
- Base reward per block: 10 LEAN
- Distributed to validator + delegators
- Commission taken by validator

**Transaction Fees:**
- All fees in block go to validator
- Shared with delegators (after commission)

**Reward Formula:**
```
Validator Reward = (Block Reward + Tx Fees) × Commission Rate
Delegator Reward = (Block Reward + Tx Fees) × (1 - Commission) × (Delegation / Total Stake)
```

### Commission Rates

**Setting Commission:**
- Initial: Can be any value 0-100%
- Maximum: Cannot exceed max rate
- Change rate: Limited by max change rate

**Best Practices:**
- Start with 5-10% for new validators
- Competitive rates attract delegators
- Don't change frequently
- Communicate changes in advance

### Slashing

**Slashing Conditions:**
1. **Double Signing** (Byzantine behavior)
   - Slash: 5% of stake
   - Jail: Permanent
   - Requires governance to unjail

2. **Downtime** (Liveness fault)
   - Slash: 0.01% of stake
   - Jail: Temporary
   - Can unjail after 10 minutes

**Avoiding Slashing:**
- ✅ Never run validator key on multiple nodes
- ✅ Maintain high uptime (>99%)
- ✅ Monitor node health 24/7
- ✅ Have backup infrastructure
- ✅ Test upgrades on testnet first

### Profitability

**Example Calculation:**

Assumptions:
- Total network stake: 100M LEAN
- Your stake: 100k LEAN (0.1% of total)
- Commission: 10%
- Block reward: 10 LEAN/block
- Blocks per day: 43,200 (2s block time)
- Your blocks produced: 43 blocks/day (0.1%)

Daily earnings:
```
Block rewards: 43 blocks × 10 LEAN = 430 LEAN
Commission (10%): 43 LEAN
Delegator share (90%): 387 LEAN

Validator daily income: 43 LEAN
Annual income: 15,695 LEAN (~15.7% APR on 100k stake)
```

## Validator Checklist

### Pre-Launch
- [ ] Hardware meets requirements
- [ ] Software installed and tested
- [ ] Keys generated and backed up
- [ ] Firewall configured
- [ ] Monitoring setup
- [ ] Sufficient LEAN for stake + fees
- [ ] Identity verified (keybase)

### Launch
- [ ] Genesis file downloaded
- [ ] Config.toml updated with peers
- [ ] Node started and syncing
- [ ] Validator created
- [ ] Commission set appropriately
- [ ] Website/social media updated

### Post-Launch
- [ ] Monitor uptime daily
- [ ] Check rewards weekly
- [ ] Update software promptly
- [ ] Engage with community
- [ ] Maintain documentation
- [ ] Plan for upgrades

## Additional Resources

- **Validator Dashboard:** https://validators.leanchain.io
- **Block Explorer:** https://explorer.leanchain.io
- **Staking Calculator:** https://staking.leanchain.io
- **Network Status:** https://status.leanchain.io

---

**Thank you for securing LEAN CHAIN! 🚀**
