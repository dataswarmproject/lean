import Link from 'next/link';

export default function LendingPage() {
  return (
    <div className="max-w-4xl mx-auto px-6 lg:px-8 py-12">
      <div className="mb-8">
        <h1 className="text-4xl font-light mb-4 tracking-tight">Lending Protocol</h1>
          <p className="text-base text-muted-foreground leading-relaxed">
            Over-collateralized lending with automated liquidations.
          </p>
        </div>

        <div className="space-y-12">
          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Overview</h2>
            <p className="text-sm text-muted-foreground leading-relaxed mb-4">
              The lending protocol allows users to supply assets as collateral and borrow other assets against them. All loans are over-collateralized to ensure system solvency.
            </p>
            <ul className="space-y-2 text-sm text-muted-foreground ml-4">
              <li>• Minimum collateral ratio: 150%</li>
              <li>• Liquidation threshold: 125%</li>
              <li>• Interest accrues continuously</li>
              <li>• Automated liquidations protect the protocol</li>
            </ul>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Supplying Assets</h2>
            <div className="bg-card border border-border/50 p-4">
              <pre className="text-xs font-mono text-foreground overflow-x-auto">
                <code>lean-cli lending supply \{'\n'}  --asset 0x... \{'\n'}  --amount 1000000000000000000</code>
              </pre>
            </div>
            <p className="text-sm text-muted-foreground mt-3">
              Supplied assets earn interest and can be used as collateral for borrowing.
            </p>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Borrowing</h2>
            <div className="space-y-6">
              <div>
                <h3 className="text-lg font-medium mb-3">Borrow Assets</h3>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>lean-cli lending borrow \{'\n'}  --asset 0x... \{'\n'}  --amount 500000000000000000 \{'\n'}  --collateral 0x... \{'\n'}  --collateral-amount 1500000000000000000</code>
                  </pre>
                </div>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-2">Collateral Requirements</h3>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  You must maintain a collateral ratio above 150%. If it drops below 125%, your position will be liquidated.
                </p>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Repaying Loans</h2>
            <div className="bg-card border border-border/50 p-4">
              <pre className="text-xs font-mono text-foreground overflow-x-auto">
                <code>lean-cli lending repay \{'\n'}  --loan-id 0x... \{'\n'}  --amount 500000000000000000</code>
              </pre>
            </div>
          </section>

          <section className="pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Interest Rates</h2>
            <p className="text-sm text-muted-foreground leading-relaxed mb-4">
              Interest rates are determined algorithmically based on supply and demand:
            </p>
            <ul className="space-y-2 text-sm text-muted-foreground ml-4">
              <li>• Utilization rate affects borrowing rates</li>
              <li>• Higher utilization = higher borrowing rates</li>
              <li>• Supply rates are derived from borrowing rates</li>
              <li>• Rates update continuously</li>
            </ul>
          </section>

          <div className="border-t border-border/50 pt-8 mt-8">
            <div className="flex items-center justify-between">
              <Link href="/docs/defi/dex" className="text-sm text-muted-foreground hover:text-foreground transition-colors inline-flex items-center space-x-2">
                <span>← DEX Guide</span>
              </Link>
              <Link href="/docs/defi/stablecoin" className="text-sm text-foreground hover:text-muted-foreground transition-colors inline-flex items-center space-x-2">
                <span>Stablecoin →</span>
              </Link>
            </div>
          </div>
        </div>
      </div>
  );
}

