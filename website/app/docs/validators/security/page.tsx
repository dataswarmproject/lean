import Link from 'next/link';

export default function SecurityPage() {
  return (
    <div className="max-w-4xl mx-auto px-6 lg:px-8 py-12">
      <div className="mb-8">
        <h1 className="text-4xl font-light mb-4 tracking-tight">Security Best Practices</h1>
          <p className="text-base text-muted-foreground leading-relaxed">
            Essential security practices for running a validator node securely.
          </p>
        </div>

        <div className="space-y-12">
          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Key Management</h2>
            
            <div className="space-y-4">
              <div>
                <h3 className="text-lg font-medium mb-2">Hardware Security Module (HSM)</h3>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  Use an HSM for production validators to store private keys securely. This prevents key extraction even if the server is compromised.
                </p>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-2">Key Backup</h3>
                <ul className="space-y-2 text-sm text-muted-foreground ml-4">
                  <li>• Store mnemonic phrases in secure, offline locations</li>
                  <li>• Use multiple backup locations (safe deposit box, encrypted storage)</li>
                  <li>• Never store keys in cloud services without encryption</li>
                  <li>• Test key recovery procedures regularly</li>
                </ul>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-2">Key Rotation</h3>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  Regularly rotate validator keys and update validator configuration. Keep old keys for historical verification.
                </p>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Network Security</h2>
            
            <div className="space-y-4">
              <div>
                <h3 className="text-lg font-medium mb-2">Firewall Configuration</h3>
                <div className="bg-card border border-border/50 p-4 mb-3">
                  <pre className="text-xs font-mono text-foreground overflow-x-auto">
                    <code>{`# Only allow P2P from trusted sources
sudo ufw allow from <trusted-ip> to any port 26656

# Restrict RPC to localhost or VPN
sudo ufw allow from 127.0.0.1 to any port 26657

# Enable firewall
sudo ufw enable`}</code>
                  </pre>
                </div>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-2">Sentry Node Architecture</h3>
                <p className="text-sm text-muted-foreground leading-relaxed mb-3">
                  Use sentry nodes to protect your validator from direct exposure:
                </p>
                <ul className="space-y-2 text-sm text-muted-foreground ml-4">
                  <li>• Validator node: Private network, no public IP</li>
                  <li>• Sentry nodes: Public IPs, connect to validator</li>
                  <li>• Validator only connects to sentry nodes</li>
                  <li>• Reduces DDoS attack surface</li>
                </ul>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">System Hardening</h2>
            
            <div className="space-y-4">
              <div>
                <h3 className="text-lg font-medium mb-2">Operating System</h3>
                <ul className="space-y-2 text-sm text-muted-foreground ml-4">
                  <li>• Use minimal OS installation</li>
                  <li>• Disable unnecessary services</li>
                  <li>• Enable automatic security updates</li>
                  <li>• Use SSH key authentication only</li>
                  <li>• Disable root login</li>
                </ul>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-2">Monitoring</h3>
                <ul className="space-y-2 text-sm text-muted-foreground ml-4">
                  <li>• Set up intrusion detection (fail2ban)</li>
                  <li>• Monitor system logs for suspicious activity</li>
                  <li>• Use monitoring tools (Prometheus, Grafana)</li>
                  <li>• Set up alerts for validator downtime</li>
                </ul>
              </div>
            </div>
          </section>

          <section className="border-b border-border/50 pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Backup and Recovery</h2>
            
            <div className="space-y-4">
              <div>
                <h3 className="text-lg font-medium mb-2">Regular Backups</h3>
                <ul className="space-y-2 text-sm text-muted-foreground ml-4">
                  <li>• Backup validator keys daily</li>
                  <li>• Backup configuration files</li>
                  <li>• Test restore procedures monthly</li>
                  <li>• Store backups in multiple locations</li>
                </ul>
              </div>

              <div>
                <h3 className="text-lg font-medium mb-2">Disaster Recovery Plan</h3>
                <p className="text-sm text-muted-foreground leading-relaxed">
                  Have a documented recovery plan including key restoration, node rebuild procedures, and communication protocols.
                </p>
              </div>
            </div>
          </section>

          <section className="pb-8">
            <h2 className="text-2xl font-light mb-6 tracking-tight">Security Checklist</h2>
            <div className="space-y-3 text-sm">
              <div className="flex items-start space-x-3 border-b border-border/30 pb-3">
                <input type="checkbox" className="mt-1" />
                <span className="text-muted-foreground">Validator keys stored in HSM or secure hardware</span>
              </div>
              <div className="flex items-start space-x-3 border-b border-border/30 pb-3">
                <input type="checkbox" className="mt-1" />
                <span className="text-muted-foreground">Mnemonic phrases backed up in secure locations</span>
              </div>
              <div className="flex items-start space-x-3 border-b border-border/30 pb-3">
                <input type="checkbox" className="mt-1" />
                <span className="text-muted-foreground">Firewall configured and enabled</span>
              </div>
              <div className="flex items-start space-x-3 border-b border-border/30 pb-3">
                <input type="checkbox" className="mt-1" />
                <span className="text-muted-foreground">Sentry node architecture implemented</span>
              </div>
              <div className="flex items-start space-x-3 border-b border-border/30 pb-3">
                <input type="checkbox" className="mt-1" />
                <span className="text-muted-foreground">SSH key authentication only</span>
              </div>
              <div className="flex items-start space-x-3 border-b border-border/30 pb-3">
                <input type="checkbox" className="mt-1" />
                <span className="text-muted-foreground">Automatic security updates enabled</span>
              </div>
              <div className="flex items-start space-x-3 border-b border-border/30 pb-3">
                <input type="checkbox" className="mt-1" />
                <span className="text-muted-foreground">Monitoring and alerting configured</span>
              </div>
              <div className="flex items-start space-x-3 pb-3">
                <input type="checkbox" className="mt-1" />
                <span className="text-muted-foreground">Backup and recovery procedures tested</span>
              </div>
            </div>
          </section>

          <div className="border-t border-border/50 pt-8 mt-8">
            <div className="flex items-center justify-between">
              <Link href="/docs/validators/staking" className="text-sm text-muted-foreground hover:text-foreground transition-colors inline-flex items-center space-x-2">
                <span>← Staking Guide</span>
              </Link>
              <Link href="/docs/validators/troubleshooting" className="text-sm text-foreground hover:text-muted-foreground transition-colors inline-flex items-center space-x-2">
                <span>Troubleshooting →</span>
              </Link>
            </div>
          </div>
        </div>
      </div>
  );
}

