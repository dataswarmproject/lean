import Link from 'next/link';

export default function StablecoinPage() {
  return (
    <div className="max-w-4xl mx-auto px-6 lg:px-8 py-12">
      <div className="mb-8">
        <h1 className="text-4xl font-light mb-4 tracking-tight">Stablecoin</h1>
          <p className="text-base text-muted-foreground leading-relaxed">
            Collateralized Debt Position (CDP) vaults for minting stablecoins.
          </p>
        </div>

        <div className="space-y-12">
          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Overview</h2>
            <p className="text-sm text-muted-foreground leading-relaxed mb-4">
              The stablecoin system uses Collateralized Debt Position (CDP) vaults. Users lock collateral to mint stablecoins, which maintain price stability through various mechanisms.
            </p>
            <ul className="space-y-2 text-sm text-muted-foreground ml-4">
              <li>• Lock collateral to mint stablecoins</li>
              <li>• Repay debt to unlock collateral</li>
              <li>• Stability mechanisms maintain peg</li>
              <li>• Automated liquidations protect system</li>
            </ul>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Creating a Vault</h2>
            <div className="bg-card border border-border/50 p-4">
              <pre className="text-xs font-mono text-foreground overflow-x-auto">
                <code>lean-cli stablecoin create-vault \{'\n'}  --collateral-type 0x... \{'\n'}  --collateral-amount 2000000000000000000 \{'\n'}  --debt-amount 1000000000000000000</code>
              </pre>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Minting Stablecoins</h2>
            <div className="bg-card border border-border/50 p-4">
              <pre className="text-xs font-mono text-foreground overflow-x-auto">
                <code>lean-cli stablecoin mint \{'\n'}  --vault-id 0x... \{'\n'}  --amount 100000000000000000</code>
              </pre>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Repaying Debt</h2>
            <div className="bg-card border border-border/50 p-4">
              <pre className="text-xs font-mono text-foreground overflow-x-auto">
                <code>lean-cli stablecoin repay \{'\n'}  --vault-id 0x... \{'\n'}  --amount 100000000000000000</code>
              </pre>
            </div>
            <p className="text-sm text-muted-foreground mt-3">
              Repaying debt reduces your loan amount and allows you to unlock collateral.
            </p>
          </section>

          <section className="pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Stability Mechanisms</h2>
            <div className="space-y-4">
              <div>
                <h3 className="text-lg font-medium mb-2">Price Stability</h3>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  The system maintains price stability through:
                </p>
                <ul className="space-y-1 text-sm text-muted-foreground ml-4 mt-2">
                  <li>• Collateralization requirements</li>
                  <li>• Liquidation mechanisms</li>
                  <li>• Stability fees</li>
                  <li>• Market arbitrage incentives</li>
                </ul>
              </div>
            </div>
          </section>

          <div className="border-t border-border/50 pt-8 mt-8">
            <div className="flex items-center justify-between">
              <Link href="/docs/defi/lending" className="text-sm text-muted-foreground hover:text-foreground transition-colors inline-flex items-center space-x-2">
                <span>← Lending Protocol</span>
              </Link>
              <Link href="/docs/defi/oracle" className="text-sm text-foreground hover:text-muted-foreground transition-colors inline-flex items-center space-x-2">
                <span>Oracle Network →</span>
              </Link>
            </div>
          </div>
        </div>
      </div>
  );
}

