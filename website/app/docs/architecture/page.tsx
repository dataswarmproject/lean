import Link from 'next/link';

export default function ArchitecturePage() {
  return (
    <div className="max-w-4xl mx-auto px-6 lg:px-8 py-12">
      <div className="mb-8">
        <h1 className="text-4xl font-light mb-4 tracking-tight">Architecture Overview</h1>
          <p className="text-base text-muted-foreground leading-relaxed">
            Understanding the modular architecture and design principles of LEAN Chain.
          </p>
        </div>

        <div className="space-y-12">
          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Modular Design</h2>
            <p className="text-sm text-muted-foreground leading-relaxed mb-6">
              LEAN Chain is built with a modular architecture consisting of 35+ independent crates, organized into four distinct layers. This design enables maintainability, testability, and extensibility.
            </p>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Layer 1: Foundation</h2>
            <div className="space-y-4">
              <div className="bg-card border border-border/50 p-6">
                <h3 className="text-lg font-medium mb-2">Blockchain Core</h3>
                <p className="text-xs text-muted-foreground mb-2 font-mono">lean-blockchain (~800 lines)</p>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  Block structure, Merkle trees, chain management, and block validation logic.
                </p>
              </div>
              <div className="bg-card border border-border/50 p-6">
                <h3 className="text-lg font-medium mb-2">Consensus</h3>
                <p className="text-xs text-muted-foreground mb-2 font-mono">lean-consensus (~700 lines)</p>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  BFT-PoS consensus mechanism with validator selection, voting, and slashing logic.
                </p>
              </div>
              <div className="bg-card border border-border/50 p-6">
                <h3 className="text-lg font-medium mb-2">State Management</h3>
                <p className="text-xs text-muted-foreground mb-2 font-mono">lean-state (~600 lines)</p>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  Account state, storage management, caching, and atomic commit operations.
                </p>
              </div>
              <div className="bg-card border border-border/50 p-6">
                <h3 className="text-lg font-medium mb-2">Transactions</h3>
                <p className="text-xs text-muted-foreground mb-2 font-mono">lean-transaction (~800 lines)</p>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  Support for 10+ transaction types with comprehensive validation and execution.
                </p>
              </div>
              <div className="bg-card border border-border/50 p-6">
                <h3 className="text-lg font-medium mb-2">Cryptography</h3>
                <p className="text-xs text-muted-foreground mb-2 font-mono">lean-crypto (~500 lines)</p>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  Ed25519 signatures, SHA-3 hashing, BLAKE3, and address generation utilities.
                </p>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Layer 2: Smart Contracts</h2>
            <div className="space-y-4">
              <div className="bg-card border border-border/50 p-6">
                <h3 className="text-lg font-medium mb-2">EVM</h3>
                <p className="text-xs text-muted-foreground mb-2 font-mono">lean-evm (~3,600 lines)</p>
                <p className="text-sm text-muted-foreground leading-relaxed mb-3">
                  Full Ethereum Virtual Machine implementation with 140+ opcodes.
                </p>
                <ul className="space-y-1 text-xs text-muted-foreground ml-4">
                  <li>• Stack management (1024 depth)</li>
                  <li>• Dynamic memory allocation</li>
                  <li>• Accurate gas metering</li>
                  <li>• Complete opcode execution engine</li>
                </ul>
              </div>
              <div className="bg-card border border-border/50 p-6">
                <h3 className="text-lg font-medium mb-2">Precompiles</h3>
                <p className="text-xs text-muted-foreground mb-2 font-mono">lean-precompiles (~580 lines)</p>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  Optimized implementations: ECRECOVER, SHA256, RIPEMD160, IDENTITY, MODEXP.
                </p>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Layer 3: DeFi Protocols</h2>
            <div className="space-y-4">
              <div className="bg-card border border-border/50 p-6">
                <h3 className="text-lg font-medium mb-2">DEX</h3>
                <p className="text-xs text-muted-foreground mb-2 font-mono">lean-dex (~830 lines)</p>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  Automated Market Maker (Uniswap V2-style) with liquidity pools and swap functionality.
                </p>
              </div>
              <div className="bg-card border border-border/50 p-6">
                <h3 className="text-lg font-medium mb-2">Lending</h3>
                <p className="text-xs text-muted-foreground mb-2 font-mono">lean-lending (~1,200 lines)</p>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  Over-collateralized lending protocol with automated liquidations and interest accrual.
                </p>
              </div>
              <div className="bg-card border border-border/50 p-6">
                <h3 className="text-lg font-medium mb-2">Stablecoin</h3>
                <p className="text-xs text-muted-foreground mb-2 font-mono">lean-stablecoin (~900 lines)</p>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  CDP vaults with stability mechanisms, debt management, and collateral ratios.
                </p>
              </div>
              <div className="bg-card border border-border/50 p-6">
                <h3 className="text-lg font-medium mb-2">Oracle</h3>
                <p className="text-xs text-muted-foreground mb-2 font-mono">lean-oracle (~800 lines)</p>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  Multi-source price feeds with aggregation, outlier detection, and weighted averaging.
                </p>
              </div>
              <div className="bg-card border border-border/50 p-6">
                <h3 className="text-lg font-medium mb-2">NFT</h3>
                <p className="text-xs text-muted-foreground mb-2 font-mono">lean-nft (~1,000 lines)</p>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  Full ERC-721 implementation with metadata management and transfer functionality.
                </p>
              </div>
              <div className="bg-card border border-border/50 p-6">
                <h3 className="text-lg font-medium mb-2">Bridge</h3>
                <p className="text-xs text-muted-foreground mb-2 font-mono">lean-bridge (~900 lines)</p>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  Cross-chain transfer framework supporting Ethereum, Bitcoin, and Cosmos networks.
                </p>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Layer 4: Enterprise & Tools</h2>
            <div className="space-y-4">
              <div className="bg-card border border-border/50 p-6">
                <h3 className="text-lg font-medium mb-2">Compliance</h3>
                <p className="text-xs text-muted-foreground mb-2 font-mono">lean-compliance</p>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  KYC/AML integration modules for institutional compliance requirements.
                </p>
              </div>
              <div className="bg-card border border-border/50 p-6">
                <h3 className="text-lg font-medium mb-2">CLI</h3>
                <p className="text-xs text-muted-foreground mb-2 font-mono">lean-cli</p>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  Command-line interface for wallet management, transactions, and validator operations.
                </p>
              </div>
              <div className="bg-card border border-border/50 p-6">
                <h3 className="text-lg font-medium mb-2">Node Software</h3>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  Validator, full node, and light client implementations for different use cases.
                </p>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Consensus Mechanism</h2>
            <div className="space-y-4">
              <p className="text-sm text-muted-foreground leading-relaxed">
                LEAN Chain uses Byzantine Fault Tolerant Proof of Stake (BFT-PoS) consensus, which provides:
              </p>
              <ul className="space-y-2 text-sm text-muted-foreground ml-4">
                <li>• <strong className="text-foreground">Fast Finality:</strong> Blocks are finalized in &lt;100ms</li>
                <li>• <strong className="text-foreground">Security:</strong> Requires 2/3+ validator agreement</li>
                <li>• <strong className="text-foreground">Decentralization:</strong> Supports up to 100 validators</li>
                <li>• <strong className="text-foreground">Slashing:</strong> Penalties for malicious behavior</li>
                <li>• <strong className="text-foreground">Validator Selection:</strong> Based on stake and performance</li>
              </ul>
            </div>
          </section>

          <section className="pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Network Architecture</h2>
            <div className="space-y-4">
              <div>
                <h3 className="text-lg font-medium mb-2">P2P Networking</h3>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  Built on libp2p with DDoS protection, peer discovery, and efficient block propagation.
                </p>
              </div>
              <div>
                <h3 className="text-lg font-medium mb-2">RPC Interface</h3>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  JSON-RPC API over HTTP and WebSocket for querying chain state and submitting transactions.
                </p>
              </div>
              <div>
                <h3 className="text-lg font-medium mb-2">State Storage</h3>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  RocksDB for persistent storage with optimized state management and caching layers.
                </p>
              </div>
            </div>
          </section>

          <div className="border-t border-border/50 pt-8 mt-8">
            <div className="flex items-center justify-between">
              <Link href="/docs/installation" className="text-sm text-muted-foreground hover:text-foreground transition-colors inline-flex items-center space-x-2">
                <span>← Installation</span>
              </Link>
              <Link href="/docs/validators/setup" className="text-sm text-foreground hover:text-muted-foreground transition-colors inline-flex items-center space-x-2">
                <span>Validator Setup →</span>
              </Link>
            </div>
          </div>
        </div>
      </div>
  );
}

