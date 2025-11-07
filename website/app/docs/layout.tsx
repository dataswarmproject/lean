'use client';

import { useState, useEffect } from 'react';
import { usePathname } from 'next/navigation';
import Link from 'next/link';
import Image from 'next/image';
import {
  RocketLaunchIcon,
  ShieldCheckIcon,
  CodeBracketIcon,
  CurrencyDollarIcon,
  Bars3Icon,
  XMarkIcon,
  ChevronRightIcon,
} from '@heroicons/react/24/outline';

const navigation = [
  {
    title: 'Getting Started',
    icon: RocketLaunchIcon,
    items: [
      { name: 'Introduction', path: '/docs/introduction' },
      { name: 'Quick Start Guide', path: '/docs/quick-start' },
      { name: 'Installation', path: '/docs/installation' },
      { name: 'Architecture Overview', path: '/docs/architecture' },
    ],
  },
  {
    title: 'Validators',
    icon: ShieldCheckIcon,
    items: [
      { name: 'Validator Setup', path: '/docs/validators/setup' },
      { name: 'Staking Guide', path: '/docs/validators/staking' },
      { name: 'Security Best Practices', path: '/docs/validators/security' },
      { name: 'Troubleshooting', path: '/docs/validators/troubleshooting' },
    ],
  },
  {
    title: 'Development',
    icon: CodeBracketIcon,
    items: [
      { name: 'API Reference', path: '/docs/api' },
      { name: 'Smart Contracts', path: '/docs/smart-contracts' },
      { name: 'EVM Compatibility', path: '/docs/evm' },
      { name: 'SDK Documentation', path: '/docs/sdk' },
    ],
  },
  {
    title: 'DeFi',
    icon: CurrencyDollarIcon,
    items: [
      { name: 'DEX Guide', path: '/docs/defi/dex' },
      { name: 'Lending Protocol', path: '/docs/defi/lending' },
      { name: 'Stablecoin', path: '/docs/defi/stablecoin' },
      { name: 'Oracle Network', path: '/docs/defi/oracle' },
    ],
  },
];

export default function DocsLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const [sidebarOpen, setSidebarOpen] = useState(false);
  const pathname = usePathname();

  useEffect(() => {
    setSidebarOpen(false);
  }, [pathname]);

  return (
    <div className="min-h-screen bg-background">
      {/* Header */}
      <header className="sticky top-0 z-50 border-b border-border/50 bg-background/80 backdrop-blur-sm">
        <div className="flex items-center justify-between h-16 px-6 lg:px-8">
          <div className="flex items-center space-x-4">
            <button
              onClick={() => setSidebarOpen(!sidebarOpen)}
              className="lg:hidden text-foreground"
              aria-label="Toggle sidebar"
            >
              {sidebarOpen ? (
                <XMarkIcon className="w-6 h-6" />
              ) : (
                <Bars3Icon className="w-6 h-6" />
              )}
            </button>
            <Link href="/docs" className="flex items-center space-x-3">
              <Image src="/lean_icon.png" alt="LEAN Chain" width={24} height={24} className="rounded-sm" />
              <span className="text-sm font-medium text-foreground tracking-tight">DOCUMENTATION</span>
            </Link>
          </div>
          <div className="flex items-center space-x-6">
            <Link href="https://github.com/dataswarmproject/lean" className="text-xs text-muted-foreground hover:text-foreground transition-colors uppercase tracking-wider hidden sm:inline">
              GitHub
            </Link>
            <Link href="/" className="text-xs text-muted-foreground hover:text-foreground transition-colors uppercase tracking-wider">
              Home
            </Link>
          </div>
        </div>
      </header>

      <div className="flex">
        {/* Sidebar */}
        <aside
          className={`
            fixed lg:sticky top-16 left-0 h-[calc(100vh-4rem)] 
            w-64 border-r border-border/50 bg-background
            overflow-y-auto z-40
            transform transition-transform duration-200 ease-in-out
            ${sidebarOpen ? 'translate-x-0' : '-translate-x-full lg:translate-x-0'}
          `}
        >
          <nav className="p-6 space-y-8">
            {navigation.map((section) => (
              <div key={section.title}>
                <div className="flex items-center space-x-2 mb-4">
                  <section.icon className="w-4 h-4 text-foreground" />
                  <h2 className="text-xs font-medium text-muted-foreground uppercase tracking-wider">
                    {section.title}
                  </h2>
                </div>
                <ul className="space-y-1">
                  {section.items.map((item) => {
                    const isActive = pathname === item.path;
                    return (
                      <li key={item.path}>
                        <Link
                          href={item.path}
                          className={`
                            block px-3 py-2 text-sm rounded-sm transition-colors
                            ${
                              isActive
                                ? 'bg-muted/50 text-foreground font-medium'
                                : 'text-muted-foreground hover:text-foreground hover:bg-muted/30'
                            }
                          `}
                        >
                          {item.name}
                        </Link>
                      </li>
                    );
                  })}
                </ul>
              </div>
            ))}
          </nav>
        </aside>

        {/* Overlay for mobile */}
        {sidebarOpen && (
          <div
            className="fixed inset-0 bg-background/80 backdrop-blur-sm z-30 lg:hidden"
            onClick={() => setSidebarOpen(false)}
          />
        )}

        {/* Main content */}
        <main className="flex-1 min-w-0">
          {children}
        </main>
      </div>
    </div>
  );
}

