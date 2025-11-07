import Link from 'next/link';

export default function QuickStartPage() {
  return (
    <div className="max-w-4xl mx-auto px-6 lg:px-8 py-12">
      <div className="mb-8">
        <h1 className="text-4xl font-light mb-4 tracking-tight">Quick Start Guide</h1>
          <p className="text-base text-muted-foreground leading-relaxed">
            Get up and running with LEAN Chain in minutes.
          </p>
        </div>

        <div className="space-y-12">
          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Prerequisites</h2>
            <ul className="space-y-3 text-sm text-muted-foreground">
              <li className="flex items-start space-x-2">
                <span className="text-foreground mt-0.5">•</span>
                <span>Rust 1.70+ installed</span>
              </li>
              <li className="flex items-start space-x-2">
                <span className="text-foreground mt-0.5">•</span>
                <span>Git installed</span>
              </li>
              <li className="flex items-start space-x-2">
                <span className="text-foreground mt-0.5">•</span>
                <span>Linux, macOS, or Windows (WSL recommended)</span>
              </li>
              <li className="flex items-start space-x-2">
                <span className="text-foreground mt-0.5">•</span>
                <span>Build essentials (gcc, make, pkg-config, libssl-dev)</span>
              </li>
            </ul>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Installation</h2>
            
            <div className="mb-6">
              <h3 className="text-lg font-medium mb-3">1. Install Rust</h3>
              <div className="bg-card border border-border/50 p-4">
                <pre className="text-xs font-mono text-foreground overflow-x-auto">
                  <code>curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh{'\n'}source $HOME/.cargo/env</code>
                </pre>
              </div>
            </div>

            <div className="mb-6">
              <h3 className="text-lg font-medium mb-3">2. Clone Repository</h3>
              <div className="bg-card border border-border/50 p-4">
                <pre className="text-xs font-mono text-foreground overflow-x-auto">
                  <code>git clone https://github.com/dataswarmproject/lean.git{'\n'}cd lean</code>
                </pre>
              </div>
            </div>

            <div className="mb-6">
              <h3 className="text-lg font-medium mb-3">3. Build from Source</h3>
              <div className="bg-card border border-border/50 p-4">
                <pre className="text-xs font-mono text-foreground overflow-x-auto">
                  <code>cargo build --release</code>
                </pre>
              </div>
              <p className="text-xs text-muted-foreground mt-2">This may take 10-20 minutes depending on your system.</p>
            </div>

            <div>
              <h3 className="text-lg font-medium mb-3">4. Install Binaries</h3>
              <div className="bg-card border border-border/50 p-4">
                <pre className="text-xs font-mono text-foreground overflow-x-auto">
                  <code>sudo cp target/release/lean-validator /usr/local/bin/{'\n'}sudo cp target/release/lean-cli /usr/local/bin/{'\n'}sudo cp target/release/lean-full-node /usr/local/bin/</code>
                </pre>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Running a Node</h2>
            
            <div className="mb-6">
              <h3 className="text-lg font-medium mb-3">Start a Full Node</h3>
              <div className="bg-card border border-border/50 p-4">
                <pre className="text-xs font-mono text-foreground overflow-x-auto">
                  <code>lean-full-node --home ~/.lean</code>
                </pre>
              </div>
              <p className="text-xs text-muted-foreground mt-2">The node will sync with the network and start processing blocks.</p>
            </div>

            <div>
              <h3 className="text-lg font-medium mb-3">Verify Installation</h3>
              <div className="bg-card border border-border/50 p-4">
                <pre className="text-xs font-mono text-foreground overflow-x-auto">
                  <code>lean-cli --version{'\n'}lean-cli query status</code>
                </pre>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Your First Transaction</h2>
            
            <div className="mb-6">
              <h3 className="text-lg font-medium mb-3">1. Create a Wallet</h3>
              <div className="bg-card border border-border/50 p-4">
                <pre className="text-xs font-mono text-foreground overflow-x-auto">
                  <code>lean-cli wallet new --output wallet.json</code>
                </pre>
              </div>
              <p className="text-xs text-muted-foreground mt-2">Save the mnemonic phrase securely. This cannot be recovered.</p>
            </div>

            <div className="mb-6">
              <h3 className="text-lg font-medium mb-3">2. Get Your Address</h3>
              <div className="bg-card border border-border/50 p-4">
                <pre className="text-xs font-mono text-foreground overflow-x-auto">
                  <code>lean-cli wallet address</code>
                </pre>
              </div>
            </div>

            <div className="mb-6">
              <h3 className="text-lg font-medium mb-3">3. Fund Your Account</h3>
              <p className="text-sm text-muted-foreground mb-3">Use the testnet faucet or receive tokens from another account.</p>
              <div className="bg-card border border-border/50 p-4">
                <pre className="text-xs font-mono text-foreground overflow-x-auto">
                  <code># Check balance{'\n'}lean-cli wallet balance</code>
                </pre>
              </div>
            </div>

            <div>
              <h3 className="text-lg font-medium mb-3">4. Send a Transaction</h3>
              <div className="bg-card border border-border/50 p-4">
                <pre className="text-xs font-mono text-foreground overflow-x-auto">
                  <code>lean-cli tx send \{'\n'}  --to 0x... \{'\n'}  --amount 1000000000000000000 \{'\n'}  --gas-price 1000000000 \{'\n'}  --gas-limit 21000</code>
                </pre>
              </div>
            </div>
          </section>

          <section className="pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Next Steps</h2>
            <div className="space-y-4">
              <div>
                <h3 className="text-lg font-medium mb-2">For Validators</h3>
                <p className="text-sm text-muted-foreground mb-3">Learn how to set up and run a validator node.</p>
                <Link href="/docs/validators/setup" className="text-sm text-foreground hover:text-muted-foreground transition-colors">
                  Validator Setup Guide →
                </Link>
              </div>
              <div>
                <h3 className="text-lg font-medium mb-2">For Developers</h3>
                <p className="text-sm text-muted-foreground mb-3">Explore the API and start building applications.</p>
                <Link href="/docs/api" className="text-sm text-foreground hover:text-muted-foreground transition-colors">
                  API Reference →
                </Link>
              </div>
              <div>
                <h3 className="text-lg font-medium mb-2">For DeFi Builders</h3>
                <p className="text-sm text-muted-foreground mb-3">Integrate with built-in DeFi protocols.</p>
                <Link href="/docs/defi/dex" className="text-sm text-foreground hover:text-muted-foreground transition-colors">
                  DeFi Documentation →
                </Link>
              </div>
            </div>
          </section>

          <div className="border-t border-border/50 pt-8 mt-8">
            <div className="flex items-center justify-between">
              <Link href="/docs/introduction" className="text-sm text-muted-foreground hover:text-foreground transition-colors inline-flex items-center space-x-2">
                <span>← Introduction</span>
              </Link>
              <Link href="/docs/installation" className="text-sm text-foreground hover:text-muted-foreground transition-colors inline-flex items-center space-x-2">
                <span>Installation →</span>
              </Link>
            </div>
          </div>
        </div>
      </div>
  );
}

