import Link from "next/link"

export default function IntroductionPage() {
  return (
    <div className="max-w-4xl mx-auto px-6 lg:px-8 py-12">
      <div className="mb-8">
        <h1 className="text-4xl font-light mb-4 tracking-tight">Introduction</h1>
          <p className="text-base text-muted-foreground leading-relaxed">
            Learn about LEAN Chain, a next-generation blockchain platform designed for speed, scalability, and enterprise adoption.
          </p>
        </div>

        <div className="prose prose-invert max-w-none space-y-8">
          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-4 tracking-tight">What is LEAN Chain?</h2>
            <p className="text-sm text-muted-foreground leading-relaxed mb-4">
              LEAN Chain is a production-ready blockchain platform that combines the best of traditional finance integration with modern DeFi capabilities. Built with 100% Safe Rust, it delivers exceptional performance while maintaining security and decentralization.
            </p>
            <p className="text-sm text-muted-foreground leading-relaxed">
              The platform surpasses XRP in speed, scalability, and decentralization while maintaining regulatory compliance capabilities, making it ideal for both institutional and decentralized applications.
            </p>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-4 tracking-tight">Key Features</h2>
            <div className="space-y-4">
              <div>
                <h3 className="text-lg font-medium mb-2">Ultra-Fast Performance</h3>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  Achieves &lt;100ms block finality and 200,000+ transactions per second, making it one of the fastest blockchain platforms available.
                </p>
              </div>
              <div>
                <h3 className="text-lg font-medium mb-2">Cost-Effective</h3>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  Transaction costs are ≤ $0.0001, enabling micro-transactions and high-frequency applications.
                </p>
              </div>
              <div>
                <h3 className="text-lg font-medium mb-2">Complete DeFi Suite</h3>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  Built-in protocols including DEX, lending, stablecoin, oracle, NFT, and bridge functionality.
                </p>
              </div>
              <div>
                <h3 className="text-lg font-medium mb-2">Enterprise-Ready</h3>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  Built-in KYC/AML compliance modules and private channels for institutional use cases.
                </p>
              </div>
              <div>
                <h3 className="text-lg font-medium mb-2">EVM Compatible</h3>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  Full Ethereum Virtual Machine support with 140+ opcodes, enabling seamless migration of Ethereum smart contracts.
                </p>
              </div>
              <div>
                <h3 className="text-lg font-medium mb-2">Decentralized</h3>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  BFT-PoS consensus mechanism supporting up to 100 validators at launch, ensuring network security and decentralization.
                </p>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-4 tracking-tight">Technology Stack</h2>
            <ul className="space-y-2 text-sm text-muted-foreground">
              <li><strong className="text-foreground">Language:</strong> 100% Safe Rust (18,680 lines, zero unsafe blocks)</li>
              <li><strong className="text-foreground">Consensus:</strong> Byzantine Fault Tolerant Proof of Stake (BFT-PoS)</li>
              <li><strong className="text-foreground">Smart Contracts:</strong> Full EVM compatibility (140+ opcodes)</li>
              <li><strong className="text-foreground">Storage:</strong> RocksDB for persistence, optimized state management</li>
              <li><strong className="text-foreground">Networking:</strong> libp2p-based P2P with DDoS protection</li>
              <li><strong className="text-foreground">Testing:</strong> Comprehensive test suite (200+ tests, 100% pass rate)</li>
            </ul>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-4 tracking-tight">Performance Metrics</h2>
            <div className="space-y-3 text-sm">
              <div className="flex justify-between items-center py-2 border-b border-border/30">
                <span className="text-muted-foreground">Block Finality</span>
                <span className="font-mono text-foreground">&lt;100ms</span>
              </div>
              <div className="flex justify-between items-center py-2 border-b border-border/30">
                <span className="text-muted-foreground">Throughput</span>
                <span className="font-mono text-foreground">200,000+ TPS</span>
              </div>
              <div className="flex justify-between items-center py-2 border-b border-border/30">
                <span className="text-muted-foreground">Transaction Cost</span>
                <span className="font-mono text-foreground">≤ $0.0001</span>
              </div>
              <div className="flex justify-between items-center py-2 border-b border-border/30">
                <span className="text-muted-foreground">Energy Efficiency</span>
                <span className="font-mono text-foreground">≤ 0.0001 kWh/tx</span>
              </div>
            </div>
          </section>

          <section className="pb-8">
            <h2 className="text-2xl font-light mb-4 tracking-tight">Use Cases</h2>
            <div className="space-y-4">
              <div>
                <h3 className="text-lg font-medium mb-2">DeFi Applications</h3>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  Build decentralized exchanges, lending protocols, and yield farming applications with built-in DeFi primitives.
                </p>
              </div>
              <div>
                <h3 className="text-lg font-medium mb-2">Enterprise Solutions</h3>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  Deploy institutional-grade applications with compliance features, private channels, and high throughput.
                </p>
              </div>
              <div>
                <h3 className="text-lg font-medium mb-2">Cross-Chain Bridges</h3>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  Connect to Ethereum, Bitcoin, and Cosmos networks through the universal bridge framework.
                </p>
              </div>
              <div>
                <h3 className="text-lg font-medium mb-2">NFT Marketplaces</h3>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  Create NFT platforms with full ERC-721 support and metadata management.
                </p>
              </div>
            </div>
          </section>

          <div className="border-t border-border/50 pt-8 mt-8">
            <div className="flex items-center justify-between">
              <div></div>
              <Link href="/docs/quick-start" className="text-sm text-foreground hover:text-muted-foreground transition-colors inline-flex items-center space-x-2">
                <span>Next: Quick Start Guide</span>
                <span>→</span>
              </Link>
            </div>
          </div>
        </div>
    </div>
  );
}

