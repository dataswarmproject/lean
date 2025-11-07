import Link from 'next/link';

export default function ApiPage() {
  return (
    <div className="max-w-4xl mx-auto px-6 lg:px-8 py-12">
      <div className="mb-8">
        <h1 className="text-4xl font-light mb-4 tracking-tight">API Reference</h1>
          <p className="text-base text-muted-foreground leading-relaxed">
            Complete reference for the JSON-RPC API and CLI commands.
          </p>
        </div>

        <div className="space-y-12">
          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Connection</h2>
            
            <div className="mb-6">
              <h3 className="text-lg font-medium mb-3">HTTP</h3>
              <div className="bg-card border border-border/50 p-4">
                <pre className="text-xs font-mono text-foreground overflow-x-auto">
                  <code>POST http://localhost:26657{'\n'}Content-Type: application/json</code>
                </pre>
              </div>
            </div>

            <div>
              <h3 className="text-lg font-medium mb-3">WebSocket</h3>
              <div className="bg-card border border-border/50 p-4">
                <pre className="text-xs font-mono text-foreground overflow-x-auto">
                  <code>ws://localhost:26657/websocket</code>
                </pre>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Chain Methods</h2>
            
            <div className="space-y-6">
              <div>
                <h3 className="text-lg font-medium mb-2">chain_getBlockByNumber</h3>
                <p className="text-sm text-muted-foreground mb-3">Get block by height.</p>
                <div className="bg-card border border-border/50 p-4 mb-2">
                  <p className="text-xs text-muted-foreground mb-2">Request:</p>
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>{`{
  "jsonrpc": "2.0",
  "method": "chain_getBlockByNumber",
  "params": [100, true],
  "id": 1
}`}</code>
                  </pre>
                </div>
                <div className="bg-card border border-border/50 p-4">
                  <p className="text-xs text-muted-foreground mb-2">Response:</p>
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>{`{
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
}`}</code>
                  </pre>
                </div>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-2">chain_getLatestBlock</h3>
                <p className="text-sm text-muted-foreground mb-3">Get the latest block.</p>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>{`{
  "jsonrpc": "2.0",
  "method": "chain_getLatestBlock",
  "params": [],
  "id": 1
}`}</code>
                  </pre>
                </div>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-2">chain_getBlockByHash</h3>
                <p className="text-sm text-muted-foreground mb-3">Get block by hash.</p>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>{`{
  "jsonrpc": "2.0",
  "method": "chain_getBlockByHash",
  "params": ["0x..."],
  "id": 1
}`}</code>
                  </pre>
                </div>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Transaction Methods</h2>
            
            <div className="space-y-6">
              <div>
                <h3 className="text-lg font-medium mb-2">tx_sendTransaction</h3>
                <p className="text-sm text-muted-foreground mb-3">Submit a transaction.</p>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>{`{
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
}`}</code>
                  </pre>
                </div>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-2">tx_getTransaction</h3>
                <p className="text-sm text-muted-foreground mb-3">Get transaction by hash.</p>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>{`{
  "jsonrpc": "2.0",
  "method": "tx_getTransaction",
  "params": ["0x..."],
  "id": 1
}`}</code>
                  </pre>
                </div>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Account Methods</h2>
            
            <div className="space-y-6">
              <div>
                <h3 className="text-lg font-medium mb-2">account_getBalance</h3>
                <p className="text-sm text-muted-foreground mb-3">Get account balance.</p>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>{`{
  "jsonrpc": "2.0",
  "method": "account_getBalance",
  "params": ["0x...", "latest"],
  "id": 1
}`}</code>
                  </pre>
                </div>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-2">account_getNonce</h3>
                <p className="text-sm text-muted-foreground mb-3">Get account nonce.</p>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>{`{
  "jsonrpc": "2.0",
  "method": "account_getNonce",
  "params": ["0x..."],
  "id": 1
}`}</code>
                  </pre>
                </div>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">CLI Commands</h2>
            
            <div className="space-y-6">
              <div>
                <h3 className="text-lg font-medium mb-3">Wallet Commands</h3>
                <div className="bg-card border border-border/50 p-4 mb-3">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>{`# Create new wallet
lean-cli wallet new --output wallet.json

# Show wallet address
lean-cli wallet address

# Check balance
lean-cli wallet balance <address>`}</code>
                  </pre>
                </div>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-3">Transaction Commands</h3>
                <div className="bg-card border border-border/50 p-4 mb-3">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>{`# Send tokens
lean-cli tx send \\
  --to 0x... \\
  --amount 1000000000000000000 \\
  --gas-price 1000000000 \\
  --gas-limit 21000

# Check transaction status
lean-cli tx status <tx-hash>`}</code>
                  </pre>
                </div>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-3">Query Commands</h3>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>{`# Query block
lean-cli query block <height>

# Query transaction
lean-cli query tx <hash>

# Query account
lean-cli query account <address>`}</code>
                  </pre>
                </div>
              </div>
            </div>
          </section>

          <section className="pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Error Codes</h2>
            <div className="bg-card border border-border/50 overflow-x-auto">
              <table className="min-w-full divide-y divide-border/50">
                <thead className="bg-muted/30">
                  <tr>
                    <th className="px-4 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Code</th>
                    <th className="px-4 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Message</th>
                    <th className="px-4 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Description</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-border/50">
                  <tr>
                    <td className="px-4 py-3 text-xs font-mono text-foreground">-32700</td>
                    <td className="px-4 py-3 text-xs text-foreground">Parse error</td>
                    <td className="px-4 py-3 text-xs text-muted-foreground">Invalid JSON</td>
                  </tr>
                  <tr>
                    <td className="px-4 py-3 text-xs font-mono text-foreground">-32600</td>
                    <td className="px-4 py-3 text-xs text-foreground">Invalid request</td>
                    <td className="px-4 py-3 text-xs text-muted-foreground">Invalid request object</td>
                  </tr>
                  <tr>
                    <td className="px-4 py-3 text-xs font-mono text-foreground">-32601</td>
                    <td className="px-4 py-3 text-xs text-foreground">Method not found</td>
                    <td className="px-4 py-3 text-xs text-muted-foreground">Method doesn't exist</td>
                  </tr>
                  <tr>
                    <td className="px-4 py-3 text-xs font-mono text-foreground">1001</td>
                    <td className="px-4 py-3 text-xs text-foreground">Insufficient balance</td>
                    <td className="px-4 py-3 text-xs text-muted-foreground">Account balance too low</td>
                  </tr>
                  <tr>
                    <td className="px-4 py-3 text-xs font-mono text-foreground">1002</td>
                    <td className="px-4 py-3 text-xs text-foreground">Invalid nonce</td>
                    <td className="px-4 py-3 text-xs text-muted-foreground">Nonce mismatch</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </section>

          <div className="border-t border-border/50 pt-8 mt-8">
            <div className="flex items-center justify-between">
              <div></div>
              <Link href="/docs/smart-contracts" className="text-sm text-foreground hover:text-muted-foreground transition-colors inline-flex items-center space-x-2">
                <span>Smart Contracts →</span>
              </Link>
            </div>
          </div>
        </div>
    </div>
  );
}

