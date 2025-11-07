import Link from 'next/link';

export default function EvmPage() {
  return (
    <div className="max-w-4xl mx-auto px-6 lg:px-8 py-12">
      <div className="mb-8">
        <h1 className="text-4xl font-light mb-4 tracking-tight">EVM Compatibility</h1>
          <p className="text-base text-muted-foreground leading-relaxed">
            Complete compatibility with the Ethereum Virtual Machine and ecosystem.
          </p>
        </div>

        <div className="space-y-12">
          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Overview</h2>
            <p className="text-sm text-muted-foreground leading-relaxed mb-4">
              LEAN Chain implements a complete Ethereum Virtual Machine (EVM) with support for 140+ opcodes, enabling full compatibility with Ethereum smart contracts and tooling.
            </p>
            <p className="text-sm text-muted-foreground leading-relaxed">
              This means you can use all existing Ethereum development tools, libraries, and deploy contracts written in Solidity without any modifications.
            </p>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Supported Features</h2>
            
            <div className="space-y-4">
              <div>
                <h3 className="text-lg font-medium mb-2">Opcodes</h3>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  All standard EVM opcodes including arithmetic, logical, storage, memory, and control flow operations.
                </p>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-2">Precompiled Contracts</h3>
                <ul className="space-y-1 text-sm text-muted-foreground ml-4">
                  <li>• ECRECOVER - Elliptic curve signature recovery</li>
                  <li>• SHA256 - SHA-256 hashing</li>
                  <li>• RIPEMD160 - RIPEMD-160 hashing</li>
                  <li>• IDENTITY - Identity function</li>
                  <li>• MODEXP - Modular exponentiation</li>
                </ul>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-2">Gas Metering</h3>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  Accurate gas metering matching Ethereum's gas costs for all operations.
                </p>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-2">Stack and Memory</h3>
                <ul className="space-y-1 text-sm text-muted-foreground ml-4">
                  <li>• Stack depth: 1024 items</li>
                  <li>• Dynamic memory allocation</li>
                  <li>• Storage slots: 2^256</li>
                </ul>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">ERC Standards</h2>
            <p className="text-sm text-muted-foreground leading-relaxed mb-4">
              All standard Ethereum token interfaces are supported:
            </p>
            <ul className="space-y-2 text-sm text-muted-foreground ml-4">
              <li>• ERC-20 - Fungible tokens</li>
              <li>• ERC-721 - Non-fungible tokens (NFTs)</li>
              <li>• ERC-1155 - Multi-token standard</li>
              <li>• ERC-165 - Interface detection</li>
              <li>• ERC-1820 - Pseudo-introspection registry</li>
            </ul>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Development Tools</h2>
            
            <div className="space-y-4">
              <div>
                <h3 className="text-lg font-medium mb-2">Solidity Compiler</h3>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  Use any Solidity compiler version. Recommended: Solidity 0.8.x for latest features and security improvements.
                </p>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-2">Testing Frameworks</h3>
                <ul className="space-y-1 text-sm text-muted-foreground ml-4">
                  <li>• Truffle</li>
                  <li>• Hardhat</li>
                  <li>• Foundry</li>
                  <li>• Brownie (Python)</li>
                </ul>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-2">Libraries</h3>
                <ul className="space-y-1 text-sm text-muted-foreground ml-4">
                  <li>• Web3.js</li>
                  <li>• Ethers.js</li>
                  <li>• Web3.py</li>
                  <li>• Web3j (Java)</li>
                </ul>
              </div>
            </div>
          </section>

          <section className="pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Migration from Ethereum</h2>
            <p className="text-sm text-muted-foreground leading-relaxed mb-4">
              Migrating contracts from Ethereum to LEAN Chain is straightforward:
            </p>
            <ol className="space-y-3 text-sm text-muted-foreground ml-4 list-decimal">
              <li>No code changes required - contracts work as-is</li>
              <li>Update RPC endpoint to LEAN Chain node</li>
              <li>Redeploy contracts (addresses will differ)</li>
              <li>Update frontend to use new contract addresses</li>
            </ol>
          </section>

          <div className="border-t border-border/50 pt-8 mt-8">
            <div className="flex items-center justify-between">
              <Link href="/docs/smart-contracts" className="text-sm text-muted-foreground hover:text-foreground transition-colors inline-flex items-center space-x-2">
                <span>← Smart Contracts</span>
              </Link>
              <Link href="/docs/sdk" className="text-sm text-foreground hover:text-muted-foreground transition-colors inline-flex items-center space-x-2">
                <span>SDK Documentation →</span>
              </Link>
            </div>
          </div>
        </div>
      </div>
  );
}

