#!/bin/bash
set -e

# LEAN CHAIN Full Node Setup Script

echo "=== LEAN CHAIN Full Node Setup ==="
echo ""

# Configuration
LEAN_HOME=${LEAN_HOME:-~/.lean}
CHAIN_ID=${CHAIN_ID:-lean-mainnet-1}
MONIKER=${MONIKER:-my-node}
GENESIS_URL=${GENESIS_URL:-https://genesis.leanchain.io/genesis.json}

# Colors
GREEN='\033[0;32m'
NC='\033[0m'

echo -e "${GREEN}[1/6] Checking system requirements...${NC}"
if ! command -v cargo &> /dev/null; then
    echo "Rust not found. Installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

echo -e "${GREEN}[2/6] Creating directory structure...${NC}"
mkdir -p $LEAN_HOME/config
mkdir -p $LEAN_HOME/data
mkdir -p $LEAN_HOME/logs

echo -e "${GREEN}[3/6] Downloading genesis file...${NC}"
curl -s $GENESIS_URL > $LEAN_HOME/config/genesis.json

echo -e "${GREEN}[4/6] Creating node configuration...${NC}"
cat > $LEAN_HOME/config/config.toml << TOML_EOF
[node]
moniker = "$MONIKER"
chain_id = "$CHAIN_ID"
mode = "full"

[p2p]
listen_addr = "0.0.0.0:26656"
persistent_peers = ""

[rpc]
listen_addr = "127.0.0.1:26657"

[logging]
level = "info"
TOML_EOF

echo -e "${GREEN}[5/6] Creating systemd service...${NC}"
sudo tee /etc/systemd/system/lean-node.service > /dev/null << SERVICE_EOF
[Unit]
Description=LEAN CHAIN Full Node
After=network-online.target

[Service]
User=$USER
ExecStart=$(which cargo) run --release --bin lean-full-node -- --home $LEAN_HOME
Restart=on-failure
RestartSec=3
LimitNOFILE=65535

[Install]
WantedBy=multi-user.target
SERVICE_EOF

sudo systemctl daemon-reload
sudo systemctl enable lean-node

echo -e "${GREEN}[6/6] Setup complete!${NC}"
echo ""
echo "Start node: sudo systemctl start lean-node"
echo "Check logs: journalctl -u lean-node -f"
