import Link from 'next/link';

export default function SdkPage() {
  return (
    <div className="max-w-4xl mx-auto px-6 lg:px-8 py-12">
      <div className="mb-8">
        <h1 className="text-4xl font-light mb-4 tracking-tight">SDK Documentation</h1>
          <p className="text-base text-muted-foreground leading-relaxed">
            Official SDKs and libraries for building on LEAN Chain.
          </p>
        </div>

        <div className="space-y-12">
          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">JavaScript/TypeScript SDK</h2>
            
            <div className="space-y-6">
              <div>
                <h3 className="text-lg font-medium mb-3">Installation</h3>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>npm install @leanchain/sdk{'\n'}# or{'\n'}yarn add @leanchain/sdk</code>
                  </pre>
                </div>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-3">Basic Usage</h3>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>{`import { LeanClient } from '@leanchain/sdk';

const client = new LeanClient('http://localhost:26657');

// Get balance
const balance = await client.getBalance('0x...');

// Send transaction
const tx = await client.sendTransaction({
  from: '0x...',
  to: '0x...',
  amount: '1000000000000000000'
});`}</code>
                  </pre>
                </div>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Python SDK</h2>
            
            <div className="space-y-6">
              <div>
                <h3 className="text-lg font-medium mb-3">Installation</h3>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>pip install leanchain-sdk</code>
                  </pre>
                </div>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-3">Basic Usage</h3>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>{`from leanchain import LeanClient

client = LeanClient('http://localhost:26657')

# Get balance
balance = client.get_balance('0x...')

# Send transaction
tx = client.send_transaction(
    from_address='0x...',
    to_address='0x...',
    amount='1000000000000000000'
)`}</code>
                  </pre>
                </div>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Rust SDK</h2>
            
            <div className="space-y-6">
              <div>
                <h3 className="text-lg font-medium mb-3">Installation</h3>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>{`[dependencies]
lean-sdk = "0.1.0"`}</code>
                  </pre>
                </div>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-3">Basic Usage</h3>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>{`use lean_sdk::Client;

#[tokio::main]
async fn main() {
    let client = Client::new("http://localhost:26657").unwrap();
    
    let balance = client.get_balance("0x...").await.unwrap();
    
    let tx = client.send_transaction(Transaction {
        from: "0x...",
        to: "0x...",
        amount: 1_000_000_000_000_000_000u128,
    }).await.unwrap();
}`}</code>
                  </pre>
                </div>
              </div>
            </div>
          </section>

          <section className="pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Available SDKs</h2>
            <div className="space-y-4">
              <div className="bg-card border border-border/50 p-6">
                <h3 className="text-lg font-medium mb-2">JavaScript/TypeScript</h3>
                <p className="text-sm text-muted-foreground mb-2">Status: Available</p>
                <Link href="https://github.com/dataswarmproject/lean-sdk-js" className="text-xs text-foreground hover:text-muted-foreground">
                  GitHub Repository →
                </Link>
              </div>
              <div className="bg-card border border-border/50 p-6">
                <h3 className="text-lg font-medium mb-2">Python</h3>
                <p className="text-sm text-muted-foreground mb-2">Status: Available</p>
                <Link href="https://github.com/dataswarmproject/lean-sdk-python" className="text-xs text-foreground hover:text-muted-foreground">
                  GitHub Repository →
                </Link>
              </div>
              <div className="bg-card border border-border/50 p-6">
                <h3 className="text-lg font-medium mb-2">Rust</h3>
                <p className="text-sm text-muted-foreground mb-2">Status: Available</p>
                <Link href="https://github.com/dataswarmproject/lean-sdk-rust" className="text-xs text-foreground hover:text-muted-foreground">
                  GitHub Repository →
                </Link>
              </div>
              <div className="bg-card border border-border/50 p-6">
                <h3 className="text-lg font-medium mb-2">Go</h3>
                <p className="text-sm text-muted-foreground mb-2">Status: Coming Soon</p>
              </div>
            </div>
          </section>

          <div className="border-t border-border/50 pt-8 mt-8">
            <div className="flex items-center justify-between">
              <Link href="/docs/evm" className="text-sm text-muted-foreground hover:text-foreground transition-colors inline-flex items-center space-x-2">
                <span>← EVM Compatibility</span>
              </Link>
              <Link href="/docs/defi/dex" className="text-sm text-foreground hover:text-muted-foreground transition-colors inline-flex items-center space-x-2">
                <span>DeFi: DEX Guide →</span>
              </Link>
            </div>
          </div>
        </div>
      </div>
  );
}

