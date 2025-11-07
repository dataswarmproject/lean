import Link from 'next/link';

export default function OraclePage() {
  return (
    <div className="max-w-4xl mx-auto px-6 lg:px-8 py-12">
      <div className="mb-8">
        <h1 className="text-4xl font-light mb-4 tracking-tight">Oracle Network</h1>
          <p className="text-base text-muted-foreground leading-relaxed">
            Multi-source price feeds with aggregation and outlier detection.
          </p>
        </div>

        <div className="space-y-12">
          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Overview</h2>
            <p className="text-sm text-muted-foreground leading-relaxed mb-4">
              The oracle network aggregates price data from multiple sources to provide reliable, tamper-resistant price feeds for DeFi protocols.
            </p>
            <ul className="space-y-2 text-sm text-muted-foreground ml-4">
              <li>• Multi-source aggregation (3+ sources)</li>
              <li>• Outlier detection and filtering</li>
              <li>• Weighted averaging</li>
              <li>• Automatic updates</li>
            </ul>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Querying Prices</h2>
            <div className="bg-card border border-border/50 p-4">
              <pre className="text-xs font-mono text-foreground overflow-x-auto">
                <code>lean-cli oracle price \{'\n'}  --pair ETH/USD</code>
              </pre>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Price Aggregation</h2>
            <div className="space-y-4">
              <div>
                <h3 className="text-lg font-medium mb-2">Multi-Source Consensus</h3>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  Prices are aggregated from multiple independent sources to reduce single points of failure and manipulation risk.
                </p>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-2">Outlier Detection</h3>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  Prices that deviate significantly from the median are filtered out to prevent manipulation.
                </p>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-2">Weighted Averaging</h3>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  Sources are weighted based on reliability and historical accuracy.
                </p>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Supported Pairs</h2>
            <p className="text-sm text-muted-foreground leading-relaxed mb-4">
              Common trading pairs are supported:
            </p>
            <ul className="space-y-1 text-sm text-muted-foreground ml-4">
              <li>• ETH/USD</li>
              <li>• BTC/USD</li>
              <li>• LEAN/USD</li>
              <li>• And more...</li>
            </ul>
          </section>

          <section className="pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Integration</h2>
            <p className="text-sm text-muted-foreground leading-relaxed mb-4">
              DeFi protocols can query oracle prices directly:
            </p>
            <div className="bg-card border border-border/50 p-4">
              <pre className="text-xs font-mono text-foreground overflow-x-auto">
                <code>{`const oracle = client.oracle();
const price = await oracle.getPrice('ETH/USD');
// Use price in your protocol logic`}</code>
              </pre>
            </div>
          </section>

          <div className="border-t border-border/50 pt-8 mt-8">
            <div className="flex items-center justify-between">
              <Link href="/docs/defi/stablecoin" className="text-sm text-muted-foreground hover:text-foreground transition-colors inline-flex items-center space-x-2">
                <span>← Stablecoin</span>
              </Link>
              <Link href="/docs" className="text-sm text-foreground hover:text-muted-foreground transition-colors inline-flex items-center space-x-2">
                <span>Back to Docs →</span>
              </Link>
            </div>
          </div>
        </div>
      </div>
  );
}

