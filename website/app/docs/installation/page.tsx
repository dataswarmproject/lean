import Link from 'next/link';

export default function InstallationPage() {
  return (
    <div className="max-w-4xl mx-auto px-6 lg:px-8 py-12">
      <div className="mb-8">
        <h1 className="text-4xl font-light mb-4 tracking-tight">Installation</h1>
          <p className="text-base text-muted-foreground leading-relaxed">
            Detailed installation instructions for different platforms and use cases.
          </p>
        </div>

        <div className="space-y-12">
          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">System Requirements</h2>
            
            <div className="mb-6">
              <h3 className="text-lg font-medium mb-3">Minimum Requirements</h3>
              <ul className="space-y-2 text-sm text-muted-foreground">
                <li className="flex items-start space-x-2">
                  <span className="text-foreground mt-0.5">•</span>
                  <span><strong className="text-foreground">CPU:</strong> 4 cores (2.0 GHz+)</span>
                </li>
                <li className="flex items-start space-x-2">
                  <span className="text-foreground mt-0.5">•</span>
                  <span><strong className="text-foreground">RAM:</strong> 8 GB</span>
                </li>
                <li className="flex items-start space-x-2">
                  <span className="text-foreground mt-0.5">•</span>
                  <span><strong className="text-foreground">Storage:</strong> 500 GB SSD</span>
                </li>
                <li className="flex items-start space-x-2">
                  <span className="text-foreground mt-0.5">•</span>
                  <span><strong className="text-foreground">Network:</strong> 100 Mbps</span>
                </li>
              </ul>
            </div>

            <div>
              <h3 className="text-lg font-medium mb-3">Recommended for Validators</h3>
              <ul className="space-y-2 text-sm text-muted-foreground">
                <li className="flex items-start space-x-2">
                  <span className="text-foreground mt-0.5">•</span>
                  <span><strong className="text-foreground">CPU:</strong> 8+ cores (3.0 GHz+)</span>
                </li>
                <li className="flex items-start space-x-2">
                  <span className="text-foreground mt-0.5">•</span>
                  <span><strong className="text-foreground">RAM:</strong> 16 GB+</span>
                </li>
                <li className="flex items-start space-x-2">
                  <span className="text-foreground mt-0.5">•</span>
                  <span><strong className="text-foreground">Storage:</strong> 1 TB+ NVMe SSD</span>
                </li>
                <li className="flex items-start space-x-2">
                  <span className="text-foreground mt-0.5">•</span>
                  <span><strong className="text-foreground">Network:</strong> 1 Gbps</span>
                </li>
              </ul>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Linux Installation</h2>
            
            <div className="mb-6">
              <h3 className="text-lg font-medium mb-3">Ubuntu/Debian</h3>
              <div className="space-y-4">
                <div>
                  <p className="text-sm text-muted-foreground mb-2">1. Update system packages:</p>
                  <div className="bg-card border border-border/50 p-4">
                    <pre className="text-xs font-mono text-foreground overflow-x-auto">
                      <code>sudo apt update && sudo apt upgrade -y</code>
                    </pre>
                  </div>
                </div>
                <div>
                  <p className="text-sm text-muted-foreground mb-2">2. Install build dependencies:</p>
                  <div className="bg-card border border-border/50 p-4">
                    <pre className="text-xs font-mono text-foreground overflow-x-auto">
                      <code>sudo apt install -y build-essential pkg-config libssl-dev curl git</code>
                    </pre>
                  </div>
                </div>
                <div>
                  <p className="text-sm text-muted-foreground mb-2">3. Install Rust:</p>
                  <div className="bg-card border border-border/50 p-4">
                    <pre className="text-xs font-mono text-foreground overflow-x-auto">
                      <code>curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh{'\n'}source $HOME/.cargo/env{'\n'}rustup default stable</code>
                    </pre>
                  </div>
                </div>
                <div>
                  <p className="text-sm text-muted-foreground mb-2">4. Clone and build:</p>
                  <div className="bg-card border border-border/50 p-4">
                    <pre className="text-xs font-mono text-foreground overflow-x-auto">
                      <code>git clone https://github.com/dataswarmproject/lean.git{'\n'}cd lean{'\n'}cargo build --release</code>
                    </pre>
                  </div>
                </div>
              </div>
            </div>

            <div>
              <h3 className="text-lg font-medium mb-3">CentOS/RHEL</h3>
              <div className="space-y-4">
                <div>
                  <p className="text-sm text-muted-foreground mb-2">1. Install dependencies:</p>
                  <div className="bg-card border border-border/50 p-4">
                    <pre className="text-xs font-mono text-foreground overflow-x-auto">
                      <code>sudo yum groupinstall -y "Development Tools"{'\n'}sudo yum install -y openssl-devel pkg-config curl git</code>
                    </pre>
                  </div>
                </div>
                <div>
                  <p className="text-sm text-muted-foreground mb-2">2. Install Rust and build (same as Ubuntu):</p>
                  <div className="bg-card border border-border/50 p-4">
                    <pre className="text-xs font-mono text-foreground overflow-x-auto">
                      <code>curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh{'\n'}source $HOME/.cargo/env{'\n'}git clone https://github.com/dataswarmproject/lean.git{'\n'}cd lean{'\n'}cargo build --release</code>
                    </pre>
                  </div>
                </div>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">macOS Installation</h2>
            
            <div className="space-y-4">
              <div>
                <p className="text-sm text-muted-foreground mb-2">1. Install Homebrew (if not installed):</p>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"</code>
                  </pre>
                </div>
              </div>
              <div>
                <p className="text-sm text-muted-foreground mb-2">2. Install dependencies:</p>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>brew install openssl pkg-config</code>
                  </pre>
                </div>
              </div>
              <div>
                <p className="text-sm text-muted-foreground mb-2">3. Install Rust and build:</p>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh{'\n'}source $HOME/.cargo/env{'\n'}git clone https://github.com/dataswarmproject/lean.git{'\n'}cd lean{'\n'}cargo build --release</code>
                  </pre>
                </div>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Windows Installation</h2>
            
            <div className="space-y-4">
              <div>
                <p className="text-sm text-muted-foreground mb-2">Windows Subsystem for Linux (WSL) is recommended:</p>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code># Install WSL 2{'\n'}wsl --install{'\n'}{'\n'}# Then follow Linux installation steps</code>
                  </pre>
                </div>
              </div>
              <div>
                <p className="text-sm text-muted-foreground mb-2">For native Windows, install Rust and build tools:</p>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code># Download and install Rust from rustup.rs{'\n'}# Install Visual Studio Build Tools{'\n'}# Clone and build{'\n'}git clone https://github.com/dataswarmproject/lean.git{'\n'}cd lean{'\n'}cargo build --release</code>
                  </pre>
                </div>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Docker Installation</h2>
            
            <div className="space-y-4">
              <div>
                <p className="text-sm text-muted-foreground mb-2">Pull and run the official image:</p>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>docker pull leanchain/lean-node:latest{'\n'}{'\n'}docker run -d \{'\n'}  --name lean-node \{'\n'}  -p 26656:26656 \{'\n'}  -p 26657:26657 \{'\n'}  -v ~/.lean:/root/.lean \{'\n'}  leanchain/lean-node:latest</code>
                  </pre>
                </div>
              </div>
            </div>
          </section>

          <section className="pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Verification</h2>
            
            <div className="space-y-4">
              <div>
                <p className="text-sm text-muted-foreground mb-2">Verify installation:</p>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>lean-cli --version{'\n'}lean-validator --version{'\n'}lean-full-node --version</code>
                  </pre>
                </div>
              </div>
              <div>
                <p className="text-sm text-muted-foreground mb-2">Check Rust version:</p>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>rustc --version  # Should be 1.70 or higher</code>
                  </pre>
                </div>
              </div>
            </div>
          </section>

          <div className="border-t border-border/50 pt-8 mt-8">
            <div className="flex items-center justify-between">
              <Link href="/docs/quick-start" className="text-sm text-muted-foreground hover:text-foreground transition-colors inline-flex items-center space-x-2">
                <span>← Quick Start</span>
              </Link>
              <Link href="/docs/architecture" className="text-sm text-foreground hover:text-muted-foreground transition-colors inline-flex items-center space-x-2">
                <span>Architecture Overview →</span>
              </Link>
            </div>
          </div>
        </div>
      </div>
  );
}

