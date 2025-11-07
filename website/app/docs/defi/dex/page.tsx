import Link from 'next/link';

export default function DexPage() {
  return (
    <div className="max-w-4xl mx-auto px-6 lg:px-8 py-12">
      <div className="mb-8">
        <h1 className="text-4xl font-light mb-4 tracking-tight">DEX Guide</h1>
          <p className="text-base text-muted-foreground leading-relaxed">
            Automated Market Maker (AMM) for token swaps and liquidity provision.
          </p>
        </div>

        <div className="space-y-12">
          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Overview</h2>
            <p className="text-sm text-muted-foreground leading-relaxed mb-4">
              LEAN Chain includes a built-in Decentralized Exchange (DEX) based on the Automated Market Maker (AMM) model, similar to Uniswap V2. It enables trustless token swaps and liquidity provision.
            </p>
            <p className="text-sm text-muted-foreground leading-relaxed">
              Liquidity providers earn fees from trades proportional to their share of the liquidity pool.
            </p>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Adding Liquidity</h2>
            
            <div className="space-y-6">
              <div>
                <h3 className="text-lg font-medium mb-3">Using CLI</h3>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>lean-cli dex add-liquidity \{'\n'}  --token-a 0x... \{'\n'}  --token-b 0x... \{'\n'}  --amount-a 1000000000000000000 \{'\n'}  --amount-b 2000000000000000000</code>
                  </pre>
                </div>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-2">Using SDK</h3>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>{`const dex = client.dex();
await dex.addLiquidity({
  tokenA: '0x...',
  tokenB: '0x...',
  amountA: '1000000000000000000',
  amountB: '2000000000000000000'
});`}</code>
                  </pre>
                </div>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Swapping Tokens</h2>
            
            <div className="space-y-6">
              <div>
                <h3 className="text-lg font-medium mb-3">Using CLI</h3>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>lean-cli dex swap \{'\n'}  --token-in 0x... \{'\n'}  --token-out 0x... \{'\n'}  --amount-in 1000000000000000000 \{'\n'}  --min-amount-out 1950000000000000000</code>
                  </pre>
                </div>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-2">Price Impact</h3>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  Large swaps may experience price slippage. Always set a minimum amount out to protect against unfavorable prices.
                </p>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Removing Liquidity</h2>
            <div className="bg-card border border-border/50 p-4">
              <pre className="text-xs font-mono text-foreground overflow-x-auto">
                <code>lean-cli dex remove-liquidity \{'\n'}  --token-a 0x... \{'\n'}  --token-b 0x... \{'\n'}  --liquidity-tokens 1000000000000000000</code>
              </pre>
            </div>
          </section>

          <section className="pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Fee Structure</h2>
            <ul className="space-y-2 text-sm text-muted-foreground">
              <li className="flex items-start space-x-2">
                <span className="text-foreground mt-0.5">•</span>
                <span><strong className="text-foreground">Trading Fee:</strong> 0.3% of swap amount</span>
              </li>
              <li className="flex items-start space-x-2">
                <span className="text-foreground mt-0.5">•</span>
                <span><strong className="text-foreground">Liquidity Provider Share:</strong> 100% of trading fees</span>
              </li>
              <li className="flex items-start space-x-2">
                <span className="text-foreground mt-0.5">•</span>
                <span>Fees are distributed proportionally to liquidity providers</span>
              </li>
            </ul>
          </section>

          <div className="border-t border-border/50 pt-8 mt-8">
            <div className="flex items-center justify-between">
              <Link href="/docs/sdk" className="text-sm text-muted-foreground hover:text-foreground transition-colors inline-flex items-center space-x-2">
                <span>← SDK Documentation</span>
              </Link>
              <Link href="/docs/defi/lending" className="text-sm text-foreground hover:text-muted-foreground transition-colors inline-flex items-center space-x-2">
                <span>Lending Protocol →</span>
              </Link>
            </div>
          </div>
        </div>
      </div>
  );
}

