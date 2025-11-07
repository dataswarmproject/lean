import Link from 'next/link';

export default function StakingPage() {
  return (
    <div className="max-w-4xl mx-auto px-6 lg:px-8 py-12">
      <div className="mb-8">
        <h1 className="text-4xl font-light mb-4 tracking-tight">Staking Guide</h1>
          <p className="text-base text-muted-foreground leading-relaxed">
            Learn how to stake LEAN tokens and earn rewards by delegating to validators.
          </p>
        </div>

        <div className="space-y-12">
          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Overview</h2>
            <p className="text-sm text-muted-foreground leading-relaxed mb-4">
              Staking on LEAN Chain allows token holders to delegate their tokens to validators and earn rewards while securing the network. Validators are selected based on their stake and performance.
            </p>
            <p className="text-sm text-muted-foreground leading-relaxed">
              Delegators share in validator rewards proportional to their stake, minus the validator's commission rate.
            </p>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Delegating to a Validator</h2>
            
            <div className="mb-6">
              <h3 className="text-lg font-medium mb-3">1. Choose a Validator</h3>
              <p className="text-sm text-muted-foreground mb-3">Research validators using the validator dashboard:</p>
              <ul className="space-y-2 text-sm text-muted-foreground ml-4 mb-4">
                <li>• Commission rate</li>
                <li>• Uptime percentage</li>
                <li>• Total stake</li>
                <li>• Performance history</li>
              </ul>
            </div>

            <div>
              <h3 className="text-lg font-medium mb-3">2. Delegate Tokens</h3>
              <div className="bg-card border border-border/50 p-4">
                <pre className="text-xs font-mono text-foreground overflow-x-auto">
                  <code>lean-cli validator delegate \{'\n'}  --validator 0x... \{'\n'}  --amount 1000000000000000000000</code>
                </pre>
              </div>
              <p className="text-xs text-muted-foreground mt-2">Amount is specified in wei (1 LEAN = 10^18 wei).</p>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Rewards</h2>
            
            <div className="space-y-4">
              <div>
                <h3 className="text-lg font-medium mb-2">Reward Calculation</h3>
                <p className="text-sm text-muted-foreground leading-relaxed mb-3">
                  Rewards are distributed based on:
                </p>
                <ul className="space-y-2 text-sm text-muted-foreground ml-4">
                  <li>• Validator's total stake</li>
                  <li>• Validator's commission rate</li>
                  <li>• Your delegation amount</li>
                  <li>• Network inflation rate</li>
                </ul>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-2">Claiming Rewards</h3>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>lean-cli validator claim-rewards \{'\n'}  --validator 0x...</code>
                  </pre>
                </div>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Undelegating</h2>
            
            <div className="space-y-4">
              <div>
                <h3 className="text-lg font-medium mb-2">Unbonding Period</h3>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  When you undelegate, tokens enter an unbonding period (typically 21 days) before they can be withdrawn. During this period, tokens do not earn rewards.
                </p>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-3">Undelegate Command</h3>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>lean-cli validator undelegate \{'\n'}  --validator 0x... \{'\n'}  --amount 1000000000000000000000</code>
                  </pre>
                </div>
              </div>
            </div>
          </section>

          <section className="pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Best Practices</h2>
            <ul className="space-y-3 text-sm text-muted-foreground">
              <li className="flex items-start space-x-2">
                <span className="text-foreground mt-0.5">•</span>
                <span>Diversify across multiple validators to reduce risk</span>
              </li>
              <li className="flex items-start space-x-2">
                <span className="text-foreground mt-0.5">•</span>
                <span>Monitor validator performance regularly</span>
              </li>
              <li className="flex items-start space-x-2">
                <span className="text-foreground mt-0.5">•</span>
                <span>Consider commission rates vs. validator reliability</span>
              </li>
              <li className="flex items-start space-x-2">
                <span className="text-foreground mt-0.5">•</span>
                <span>Keep some tokens unstaked for flexibility</span>
              </li>
            </ul>
          </section>

          <div className="border-t border-border/50 pt-8 mt-8">
            <div className="flex items-center justify-between">
              <Link href="/docs/validators/setup" className="text-sm text-muted-foreground hover:text-foreground transition-colors inline-flex items-center space-x-2">
                <span>← Validator Setup</span>
              </Link>
              <Link href="/docs/validators/security" className="text-sm text-foreground hover:text-muted-foreground transition-colors inline-flex items-center space-x-2">
                <span>Security Best Practices →</span>
              </Link>
            </div>
          </div>
        </div>
      </div>
  );
}

