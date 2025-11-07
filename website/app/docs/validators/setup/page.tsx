import Link from 'next/link';

export default function ValidatorSetupPage() {
  return (
    <div className="max-w-4xl mx-auto px-6 lg:px-8 py-12">
      <div className="mb-8">
        <h1 className="text-4xl font-light mb-4 tracking-tight">Validator Setup</h1>
          <p className="text-base text-muted-foreground leading-relaxed">
            Complete guide to setting up and running a validator node on LEAN Chain.
          </p>
        </div>

        <div className="space-y-12">
          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Requirements</h2>
            
            <div className="mb-6">
              <h3 className="text-lg font-medium mb-3">Hardware</h3>
              <ul className="space-y-2 text-sm text-muted-foreground">
                <li className="flex items-start space-x-2">
                  <span className="text-foreground mt-0.5">•</span>
                  <span><strong className="text-foreground">CPU:</strong> 8+ cores (3.0 GHz+)</span>
                </li>
                <li className="flex items-start space-x-2">
                  <span className="text-foreground mt-0.5">•</span>
                  <span><strong className="text-foreground">RAM:</strong> 16 GB+</span>
                </li>
                <li className="flex items-start space-x-2">
                  <span className="text-foreground mt-0.5">•</span>
                  <span><strong className="text-foreground">Storage:</strong> 1 TB+ NVMe SSD</span>
                </li>
                <li className="flex items-start space-x-2">
                  <span className="text-foreground mt-0.5">•</span>
                  <span><strong className="text-foreground">Network:</strong> 1 Gbps</span>
                </li>
              </ul>
            </div>

            <div>
              <h3 className="text-lg font-medium mb-3">Stake Requirements</h3>
              <ul className="space-y-2 text-sm text-muted-foreground">
                <li className="flex items-start space-x-2">
                  <span className="text-foreground mt-0.5">•</span>
                  <span><strong className="text-foreground">Minimum:</strong> 10,000 LEAN tokens</span>
                </li>
                <li className="flex items-start space-x-2">
                  <span className="text-foreground mt-0.5">•</span>
                  <span><strong className="text-foreground">Recommended:</strong> 50,000+ LEAN tokens</span>
                </li>
              </ul>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Automated Setup</h2>
            
            <div className="mb-6">
              <p className="text-sm text-muted-foreground mb-3">Run the automated setup script:</p>
              <div className="bg-card border border-border/50 p-4">
                <pre className="text-xs font-mono text-foreground overflow-x-auto">
                  <code>./deploy/scripts/setup-validator.sh{'\n'}{'\n'}# Or with custom moniker{'\n'}MONIKER="my-validator" ./deploy/scripts/setup-validator.sh</code>
                </pre>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Manual Setup</h2>
            
            <div className="space-y-6">
              <div>
                <h3 className="text-lg font-medium mb-3">1. Generate Validator Key</h3>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>lean-cli wallet new --output validator_key.json</code>
                  </pre>
                </div>
                <p className="text-xs text-muted-foreground mt-2">Backup the mnemonic phrase securely.</p>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-3">2. Fund Validator Account</h3>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code># Get validator address{'\n'}lean-cli wallet address{'\n'}{'\n'}# Fund with minimum stake + fees{'\n'}# Minimum: 10,000 LEAN</code>
                  </pre>
                </div>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-3">3. Create Validator</h3>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>lean-cli validator create \{'\n'}  --moniker "My Validator" \{'\n'}  --commission-rate 0.10 \{'\n'}  --commission-max-rate 0.20 \{'\n'}  --commission-max-change 0.01 \{'\n'}  --min-self-delegation 10000000000000000000000</code>
                  </pre>
                </div>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-3">4. Start Validator</h3>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code># Start service{'\n'}sudo systemctl start lean-validator{'\n'}{'\n'}# Enable auto-start{'\n'}sudo systemctl enable lean-validator{'\n'}{'\n'}# Check logs{'\n'}journalctl -u lean-validator -f</code>
                  </pre>
                </div>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Configuration</h2>
            
            <div className="mb-6">
              <h3 className="text-lg font-medium mb-3">config.toml</h3>
              <div className="bg-card border border-border/50 p-4">
                <pre className="text-xs font-mono text-foreground overflow-x-auto">
                  <code>{`[node]
moniker = "my-validator"
chain_id = "lean-mainnet-1"

[p2p]
listen_addr = "0.0.0.0:26656"
external_address = "your.public.ip:26656"
persistent_peers = "peer1@ip1:26656,peer2@ip2:26656"

[rpc]
listen_addr = "127.0.0.1:26657"

[consensus]
timeout_propose = 2000
timeout_prevote = 1000
timeout_precommit = 1000`}</code>
                </pre>
              </div>
            </div>
          </section>

          <section className="pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Firewall Setup</h2>
            <div className="bg-card border border-border/50 p-4">
              <pre className="text-xs font-mono text-foreground overflow-x-auto">
                <code>{`# Allow P2P
sudo ufw allow 26656/tcp

# Allow RPC (only from trusted IPs)
sudo ufw allow from 192.168.1.0/24 to any port 26657

# Enable firewall
sudo ufw enable`}</code>
              </pre>
            </div>
          </section>

          <div className="border-t border-border/50 pt-8 mt-8">
            <div className="flex items-center justify-between">
              <Link href="/docs/architecture" className="text-sm text-muted-foreground hover:text-foreground transition-colors inline-flex items-center space-x-2">
                <span>← Architecture</span>
              </Link>
              <Link href="/docs/validators/staking" className="text-sm text-foreground hover:text-muted-foreground transition-colors inline-flex items-center space-x-2">
                <span>Staking Guide →</span>
              </Link>
            </div>
          </div>
        </div>
    </div>
  );
}

