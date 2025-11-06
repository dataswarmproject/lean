#!/bin/bash
set -e

# LEAN CHAIN Validator Setup Script
# This script sets up a new validator node

echo "=== LEAN CHAIN Validator Setup ==="
echo ""

# Configuration
LEAN_HOME=${LEAN_HOME:-~/.lean}
CHAIN_ID=${CHAIN_ID:-lean-mainnet-1}
MONIKER=${MONIKER:-my-validator}
GENESIS_URL=${GENESIS_URL:-https://genesis.leanchain.io/genesis.json}

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${GREEN}[1/8] Checking system requirements...${NC}"
# Check Rust installation
if ! command -v cargo &> /dev/null; then
    echo "Rust not found. Installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# Check system resources
TOTAL_RAM=$(free -g | awk '/^Mem:/{print $2}')
if [ "$TOTAL_RAM" -lt 8 ]; then
    echo -e "${YELLOW}Warning: Less than 8GB RAM detected. Recommended: 16GB+${NC}"
fi

echo -e "${GREEN}[2/8] Creating directory structure...${NC}"
mkdir -p $LEAN_HOME/config
mkdir -p $LEAN_HOME/data
mkdir -p $LEAN_HOME/logs

echo -e "${GREEN}[3/8] Generating validator keys...${NC}"
if [ ! -f "$LEAN_HOME/config/validator_key.json" ]; then
    cargo run --release --bin lean-cli wallet new \
        --output $LEAN_HOME/config/validator_key.json
    echo "Validator key generated and saved to $LEAN_HOME/config/validator_key.json"
    echo -e "${YELLOW}IMPORTANT: Backup this file securely!${NC}"
else
    echo "Validator key already exists"
fi

echo -e "${GREEN}[4/8] Downloading genesis file...${NC}"
curl -s $GENESIS_URL > $LEAN_HOME/config/genesis.json
echo "Genesis file downloaded"

echo -e "${GREEN}[5/8] Creating node configuration...${NC}"
cat > $LEAN_HOME/config/config.toml << TOML_EOF
# LEAN CHAIN Node Configuration

[node]
moniker = "$MONIKER"
chain_id = "$CHAIN_ID"

[p2p]
listen_addr = "0.0.0.0:26656"
external_address = ""
persistent_peers = ""
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
max_tx_bytes = 1048576

[statesync]
enable = false
rpc_servers = ""
trust_height = 0
trust_hash = ""

[logging]
level = "info"
format = "plain"
output = "file"
TOML_EOF

echo -e "${GREEN}[6/8] Creating systemd service...${NC}"
sudo tee /etc/systemd/system/lean-validator.service > /dev/null << SERVICE_EOF
[Unit]
Description=LEAN CHAIN Validator Node
After=network-online.target

[Service]
User=$USER
ExecStart=$(which cargo) run --release --bin lean-validator -- --home $LEAN_HOME
Restart=on-failure
RestartSec=3
LimitNOFILE=65535

[Install]
WantedBy=multi-user.target
SERVICE_EOF

echo -e "${GREEN}[7/8] Enabling service...${NC}"
sudo systemctl daemon-reload
sudo systemctl enable lean-validator

echo -e "${GREEN}[8/8] Setup complete!${NC}"
echo ""
echo "=== Next Steps ==="
echo "1. Fund your validator address with minimum stake"
echo "2. Update persistent_peers in $LEAN_HOME/config/config.toml"
echo "3. Start validator: sudo systemctl start lean-validator"
echo "4. Check logs: journalctl -u lean-validator -f"
echo "5. Create validator tx: lean-cli validator create"
echo ""
echo "Validator Address: $(cat $LEAN_HOME/config/validator_key.json | jq -r .address)"
echo ""
echo -e "${YELLOW}IMPORTANT: Backup $LEAN_HOME/config/validator_key.json securely!${NC}"
