import Link from 'next/link';

export default function SmartContractsPage() {
  return (
    <div className="max-w-4xl mx-auto px-6 lg:px-8 py-12">
      <div className="mb-8">
        <h1 className="text-4xl font-light mb-4 tracking-tight">Smart Contracts</h1>
          <p className="text-base text-muted-foreground leading-relaxed">
            Deploy and interact with smart contracts on LEAN Chain using Solidity and the EVM.
          </p>
        </div>

        <div className="space-y-12">
          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">EVM Compatibility</h2>
            <p className="text-sm text-muted-foreground leading-relaxed mb-4">
              LEAN Chain provides full Ethereum Virtual Machine (EVM) compatibility, supporting 140+ opcodes and all standard Ethereum tooling. This means you can deploy existing Ethereum smart contracts without modification.
            </p>
            <ul className="space-y-2 text-sm text-muted-foreground ml-4">
              <li>• Full Solidity compiler support</li>
              <li>• Standard Ethereum JSON-RPC API</li>
              <li>• Compatible with Web3.js, Ethers.js, and other Ethereum libraries</li>
              <li>• Support for all ERC standards (ERC-20, ERC-721, ERC-1155, etc.)</li>
            </ul>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Deploying Contracts</h2>
            
            <div className="space-y-6">
              <div>
                <h3 className="text-lg font-medium mb-3">1. Compile Solidity Contract</h3>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>solc --bin --abi Contract.sol -o build/</code>
                  </pre>
                </div>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-3">2. Deploy Contract</h3>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>lean-cli contract deploy \{'\n'}  --bytecode $(cat build/Contract.bin) \{'\n'}  --gas-limit 5000000</code>
                  </pre>
                </div>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-3">3. Verify Deployment</h3>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>lean-cli contract info 0x...</code>
                  </pre>
                </div>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Interacting with Contracts</h2>
            
            <div className="space-y-6">
              <div>
                <h3 className="text-lg font-medium mb-3">Read Operations</h3>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>lean-cli contract call \{'\n'}  --to 0x... \{'\n'}  --method "balanceOf(address)" \{'\n'}  --args "0x..."</code>
                  </pre>
                </div>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-3">Write Operations</h3>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>lean-cli contract send \{'\n'}  --to 0x... \{'\n'}  --method "transfer(address,uint256)" \{'\n'}  --args "0x...,1000000000000000000" \{'\n'}  --gas-limit 100000</code>
                  </pre>
                </div>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Using Web3 Libraries</h2>
            
            <div className="space-y-6">
              <div>
                <h3 className="text-lg font-medium mb-3">Web3.js Example</h3>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>{`const Web3 = require('web3');
const web3 = new Web3('http://localhost:26657');

const contract = new web3.eth.Contract(abi, address);
const balance = await contract.methods.balanceOf('0x...').call();
await contract.methods.transfer('0x...', amount).send({from: '0x...'});`}</code>
                  </pre>
                </div>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-3">Ethers.js Example</h3>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>{`const { ethers } = require('ethers');
const provider = new ethers.providers.JsonRpcProvider('http://localhost:26657');
const contract = new ethers.Contract(address, abi, provider);
const balance = await contract.balanceOf('0x...');`}</code>
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
                <span>Use established patterns (Checks-Effects-Interactions) to prevent reentrancy</span>
              </li>
              <li className="flex items-start space-x-2">
                <span className="text-foreground mt-0.5">•</span>
                <span>Implement access control for sensitive functions</span>
              </li>
              <li className="flex items-start space-x-2">
                <span className="text-foreground mt-0.5">•</span>
                <span>Use SafeMath or Solidity 0.8+ for arithmetic operations</span>
              </li>
              <li className="flex items-start space-x-2">
                <span className="text-foreground mt-0.5">•</span>
                <span>Test contracts thoroughly before deployment</span>
              </li>
              <li className="flex items-start space-x-2">
                <span className="text-foreground mt-0.5">•</span>
                <span>Consider gas optimization in contract design</span>
              </li>
            </ul>
          </section>

          <div className="border-t border-border/50 pt-8 mt-8">
            <div className="flex items-center justify-between">
              <Link href="/docs/api" className="text-sm text-muted-foreground hover:text-foreground transition-colors inline-flex items-center space-x-2">
                <span>← API Reference</span>
              </Link>
              <Link href="/docs/evm" className="text-sm text-foreground hover:text-muted-foreground transition-colors inline-flex items-center space-x-2">
                <span>EVM Compatibility →</span>
              </Link>
            </div>
          </div>
        </div>
      </div>
  );
}

