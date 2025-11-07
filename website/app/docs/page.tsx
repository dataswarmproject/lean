'use client';

import { useState } from 'react';
import Link from 'next/link';
import { MagnifyingGlassIcon } from '@heroicons/react/24/outline';
import { getSubdomainUrl } from '@/lib/utils';

export default function DocsPage() {
  const [searchQuery, setSearchQuery] = useState('');

  return (
    <div className="max-w-4xl mx-auto px-6 lg:px-8 py-12">
      <div className="mb-8">
        <h1 className="text-4xl font-light mb-4 tracking-tight">Documentation</h1>
        <p className="text-base text-muted-foreground leading-relaxed mb-8">
          Complete guides and references for building on LEAN Chain
        </p>
        
        {/* Search */}
        <div className="relative mb-12">
          <MagnifyingGlassIcon className="absolute left-4 top-1/2 transform -translate-y-1/2 w-4 h-4 text-muted-foreground" />
          <input
            type="text"
            placeholder="Search documentation..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full pl-11 pr-4 py-3 bg-card border border-border/50 focus:border-border focus:outline-none text-sm text-foreground placeholder:text-muted-foreground"
          />
        </div>
      </div>

      <div className="space-y-12">
        <section className="border-b border-border/50 pb-8">
          <h2 className="text-2xl font-light mb-6 tracking-tight">Welcome</h2>
          <p className="text-sm text-muted-foreground leading-relaxed mb-4">
            Welcome to the LEAN Chain documentation. This comprehensive guide will help you understand, 
            build, and deploy applications on the LEAN Chain network.
          </p>
          <p className="text-sm text-muted-foreground leading-relaxed">
            Use the sidebar navigation to explore different sections, or start with the{' '}
            <Link href="/docs/introduction" className="text-foreground hover:text-muted-foreground underline">
              Introduction
            </Link>{' '}
            to get an overview of the platform.
          </p>
        </section>

        <section className="border-b border-border/50 pb-8">
          <h2 className="text-2xl font-light mb-6 tracking-tight">Quick Start</h2>
          <p className="text-sm text-muted-foreground leading-relaxed mb-4">
            New to LEAN Chain? Start here:
          </p>
          <ol className="space-y-3 text-sm text-muted-foreground ml-4 list-decimal">
            <li>
              <Link href="/docs/introduction" className="text-foreground hover:text-muted-foreground underline">
                Read the Introduction
              </Link>{' '}
              to understand what LEAN Chain is
            </li>
            <li>
              <Link href="/docs/quick-start" className="text-foreground hover:text-muted-foreground underline">
                Follow the Quick Start Guide
              </Link>{' '}
              to set up your development environment
            </li>
            <li>
              <Link href="/docs/installation" className="text-foreground hover:text-muted-foreground underline">
                Install LEAN Chain
              </Link>{' '}
              on your system
            </li>
            <li>
              <Link href="/docs/architecture" className="text-foreground hover:text-muted-foreground underline">
                Learn about the Architecture
              </Link>{' '}
              to understand how it works
            </li>
          </ol>
        </section>

        <section className="pb-8">
          <h2 className="text-2xl font-light mb-6 tracking-tight">Additional Resources</h2>
          <div className="grid md:grid-cols-3 gap-12">
            <div>
              <h3 className="text-xs font-medium mb-4 uppercase tracking-wider text-muted-foreground">Community</h3>
              <ul className="space-y-3 text-sm text-foreground">
                <li><Link href={getSubdomainUrl('forum')} className="hover:text-muted-foreground transition-colors">Forum</Link></li>
                <li><Link href="https://discord.gg/leanchain" className="hover:text-muted-foreground transition-colors">Discord</Link></li>
                <li><Link href="https://twitter.com/leanchain" className="hover:text-muted-foreground transition-colors">Twitter</Link></li>
              </ul>
            </div>
            <div>
              <h3 className="text-xs font-medium mb-4 uppercase tracking-wider text-muted-foreground">Tools</h3>
              <ul className="space-y-3 text-sm text-foreground">
                <li><Link href={getSubdomainUrl('explorer')} className="hover:text-muted-foreground transition-colors">Block Explorer</Link></li>
                <li><Link href={getSubdomainUrl('validators')} className="hover:text-muted-foreground transition-colors">Validator Dashboard</Link></li>
                <li><Link href={getSubdomainUrl('staking')} className="hover:text-muted-foreground transition-colors">Staking Calculator</Link></li>
              </ul>
            </div>
            <div>
              <h3 className="text-xs font-medium mb-4 uppercase tracking-wider text-muted-foreground">Status</h3>
              <ul className="space-y-3 text-sm text-foreground">
                <li><Link href={getSubdomainUrl('status')} className="hover:text-muted-foreground transition-colors">Network Status</Link></li>
                <li><Link href="https://github.com/dataswarmproject/lean" className="hover:text-muted-foreground transition-colors">GitHub Repository</Link></li>
              </ul>
            </div>
          </div>
        </section>
      </div>
    </div>
  );
}

