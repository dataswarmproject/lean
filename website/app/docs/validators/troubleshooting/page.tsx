import Link from 'next/link';
import { getSubdomainUrl } from '@/lib/utils';

export default function TroubleshootingPage() {
  return (
    <div className="max-w-4xl mx-auto px-6 lg:px-8 py-12">
      <div className="mb-8">
        <h1 className="text-4xl font-light mb-4 tracking-tight">Troubleshooting</h1>
          <p className="text-base text-muted-foreground leading-relaxed">
            Common issues and solutions for validator operations.
          </p>
        </div>

        <div className="space-y-12">
          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Node Not Syncing</h2>
            
            <div className="space-y-4">
              <div>
                <h3 className="text-lg font-medium mb-2">Symptoms</h3>
                <ul className="space-y-1 text-sm text-muted-foreground ml-4">
                  <li>• Block height not increasing</li>
                  <li>• "No peers" errors in logs</li>
                  <li>• Validator missing blocks</li>
                </ul>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-2">Solutions</h3>
                <ul className="space-y-2 text-sm text-muted-foreground ml-4">
                  <li>• Check firewall allows port 26656</li>
                  <li>• Verify persistent peers in config.toml</li>
                  <li>• Check network connectivity</li>
                  <li>• Restart node: <code className="text-xs bg-card px-1 py-0.5 rounded">sudo systemctl restart lean-validator</code></li>
                </ul>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Validator Not Producing Blocks</h2>
            
            <div className="space-y-4">
              <div>
                <h3 className="text-lg font-medium mb-2">Check Validator Status</h3>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>lean-cli validator status</code>
                  </pre>
                </div>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-2">Common Causes</h3>
                <ul className="space-y-2 text-sm text-muted-foreground ml-4">
                  <li>• Insufficient stake (below minimum)</li>
                  <li>• Validator jailed (check status)</li>
                  <li>• Network connectivity issues</li>
                  <li>• Node not fully synced</li>
                </ul>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">High Memory Usage</h2>
            
            <div className="space-y-4">
              <div>
                <h3 className="text-lg font-medium mb-2">Monitor Resources</h3>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code># Check memory usage{'\n'}free -h{'\n'}{'\n'}# Check process memory{'\n'}ps aux | grep lean-validator</code>
                  </pre>
                </div>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-2">Solutions</h3>
                <ul className="space-y-2 text-sm text-muted-foreground ml-4">
                  <li>• Increase system RAM</li>
                  <li>• Reduce cache size in config</li>
                  <li>• Restart node periodically</li>
                  <li>• Check for memory leaks in logs</li>
                </ul>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Connection Issues</h2>
            
            <div className="space-y-4">
              <div>
                <h3 className="text-lg font-medium mb-2">Test Connectivity</h3>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>{`# Test RPC endpoint
curl -X POST http://localhost:26657 \\
  -H "Content-Type: application/json" \\
  -d '{"jsonrpc":"2.0","method":"chain_getLatestBlock","params":[],"id":1}'`}</code>
                  </pre>
                </div>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-2">Check Logs</h3>
                <div className="bg-card border border-border/50 p-4">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>journalctl -u lean-validator -n 100 --no-pager</code>
                  </pre>
                </div>
              </div>
            </div>
          </section>

          <section className="pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Getting Help</h2>
            <div className="space-y-4">
              <div>
                <h3 className="text-lg font-medium mb-2">Resources</h3>
                <ul className="space-y-2 text-sm text-muted-foreground">
                  <li>• <Link href={getSubdomainUrl('forum')} className="text-foreground hover:text-muted-foreground">Community Forum</Link></li>
                  <li>• <Link href="https://discord.gg/leanchain" className="text-foreground hover:text-muted-foreground">Discord Support</Link></li>
                  <li>• <Link href="https://github.com/dataswarmproject/lean/issues" className="text-foreground hover:text-muted-foreground">GitHub Issues</Link></li>
                </ul>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-2">When Reporting Issues</h3>
                <ul className="space-y-2 text-sm text-muted-foreground ml-4">
                  <li>• Include node version and OS</li>
                  <li>• Provide relevant log excerpts</li>
                  <li>• Describe steps to reproduce</li>
                  <li>• Include configuration (sanitized)</li>
                </ul>
              </div>
            </div>
          </section>

          <div className="border-t border-border/50 pt-8 mt-8">
            <div className="flex items-center justify-between">
              <Link href="/docs/validators/security" className="text-sm text-muted-foreground hover:text-foreground transition-colors inline-flex items-center space-x-2">
                <span>← Security Best Practices</span>
              </Link>
              <Link href="/docs/smart-contracts" className="text-sm text-foreground hover:text-muted-foreground transition-colors inline-flex items-center space-x-2">
                <span>Smart Contracts →</span>
              </Link>
            </div>
          </div>
        </div>
      </div>
  );
}

