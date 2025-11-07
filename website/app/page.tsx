'use client';

import { useState, useEffect } from 'react';
import Link from 'next/link';
import Image from 'next/image';
import { 
  BoltIcon, 
  ShieldCheckIcon, 
  ChartBarIcon,
  GlobeAltIcon,
  CurrencyDollarIcon,
  CpuChipIcon,
  ArrowRightIcon,
  CheckCircleIcon,
  BookOpenIcon,
  ChatBubbleLeftRightIcon,
  MagnifyingGlassIcon
} from '@heroicons/react/24/outline';
import { getSubdomainUrl } from '@/lib/utils';

export default function Home() {
  const [stats, setStats] = useState({
    tps: 200000,
    finality: '<100ms',
    cost: '$0.0001',
    validators: 100,
  });

  useEffect(() => {
    const fetchStats = async () => {
      try {
        const [statsRes, validatorsRes] = await Promise.all([
          fetch('/api/stats'),
          fetch('/api/validators'),
        ]);

        if (statsRes.ok) {
          const networkStats = await statsRes.json();
          setStats(prev => ({
            ...prev,
            tps: networkStats.tps || prev.tps,
            finality: networkStats.avg_block_time ? `<${networkStats.avg_block_time}ms` : prev.finality,
          }));
        }

        if (validatorsRes.ok) {
          const validatorsData = await validatorsRes.json();
          setStats(prev => ({
            ...prev,
            validators: validatorsData.validators?.length || prev.validators,
          }));
        }
      } catch (err) {
        console.error('Error fetching stats:', err);
      }
    };

    fetchStats();
    const interval = setInterval(fetchStats, 30000); // Refresh every 30 seconds
    return () => clearInterval(interval);
  }, []);

  const features = [
    {
      icon: BoltIcon,
      title: 'Ultra-Fast',
      description: '<100ms block finality, 200,000+ TPS',
      color: 'text-yellow-500',
    },
    {
      icon: CurrencyDollarIcon,
      title: 'Cost-Effective',
      description: '≤ $0.0001 per transaction',
      color: 'text-green-500',
    },
    {
      icon: ShieldCheckIcon,
      title: 'Secure',
      description: 'BFT-PoS consensus, 100% Safe Rust',
      color: 'text-blue-500',
    },
    {
      icon: CpuChipIcon,
      title: 'EVM Compatible',
      description: 'Full Ethereum Virtual Machine support',
      color: 'text-purple-500',
    },
    {
      icon: GlobeAltIcon,
      title: 'Decentralized',
      description: 'Up to 100 validators at launch',
      color: 'text-indigo-500',
    },
    {
      icon: ChartBarIcon,
      title: 'DeFi Ready',
      description: 'Complete DeFi suite built-in',
      color: 'text-pink-500',
    },
  ];

  const services = [
    { name: 'Documentation', url: getSubdomainUrl('docs'), icon: BookOpenIcon },
    { name: 'Forum', url: getSubdomainUrl('forum'), icon: ChatBubbleLeftRightIcon },
    { name: 'Validator Dashboard', url: getSubdomainUrl('validators'), icon: ShieldCheckIcon },
    { name: 'Block Explorer', url: getSubdomainUrl('explorer'), icon: MagnifyingGlassIcon },
    { name: 'Staking Calculator', url: getSubdomainUrl('staking'), icon: CurrencyDollarIcon },
    { name: 'Network Status', url: getSubdomainUrl('status'), icon: ChartBarIcon },
  ];

  return (
    <div className="min-h-screen bg-background">
      {/* Navigation */}
      <nav className="sticky top-0 z-50 border-b border-border/50 bg-background/80 backdrop-blur-sm">
        <div className="max-w-7xl mx-auto px-6 lg:px-8">
          <div className="flex justify-between items-center h-16">
            <div className="flex items-center space-x-3">
              <Image src="/lean_icon.png" alt="LEAN Chain" width={24} height={24} className="rounded-sm" />
              <span className="text-sm font-medium text-foreground tracking-tight">LEAN CHAIN</span>
            </div>
            <div className="hidden md:flex items-center space-x-8">
              <Link href="#features" className="text-xs text-muted-foreground hover:text-foreground transition-colors tracking-wide uppercase">Features</Link>
              <Link href="#stats" className="text-xs text-muted-foreground hover:text-foreground transition-colors tracking-wide uppercase">Stats</Link>
              <Link href="#services" className="text-xs text-muted-foreground hover:text-foreground transition-colors tracking-wide uppercase">Services</Link>
              <Link href={getSubdomainUrl('docs')} className="text-xs text-muted-foreground hover:text-foreground transition-colors tracking-wide uppercase">Docs</Link>
            </div>
            <div className="flex items-center space-x-3">
              <Link href="https://github.com/dataswarmproject/lean" className="btn-secondary">GitHub</Link>
              <Link href={getSubdomainUrl('validators')} className="btn-primary">Get Started</Link>
            </div>
          </div>
        </div>
      </nav>

      {/* Hero Section */}
      <section className="border-b border-border/50">
        <div className="max-w-7xl mx-auto px-6 lg:px-8 py-32">
          <div className="max-w-4xl">
            <div className="mb-12">
              <svg 
                xmlns="http://www.w3.org/2000/svg" 
                viewBox="0 0 34160.45 5970.49" 
                className="w-32 h-auto"
                aria-label="LEAN Chain Logo"
              >
                <path 
                  className="fill-foreground" 
                  d="M-0 0l1434.71 0 0 5970.49 690.31 0c410,0 894.29,-3.68 1084.42,-366.94l1368.07 -2613.73 -1563.23 0 1579.58 -2980.67 690.31 0c410,0 744.4,334.4 744.4,744.4l0 5216.94 -329.5 0 -1105.21 0 0 -5961.33 -690.31 0c-410,0 -894.29,3.68 -1084.42,366.94l-1368.07 2613.73 1563.23 0 -1579.58 2980.67 -690.31 0c-409.99,0 -744.4,-334.4 -744.4,-744.4l0 -5226.1zm7168.7 4776.97l0 -3505.42 661.02 0 0 2904.49 1602.48 0 0 600.93 -2263.5 0zm2714.2 0l0 -3505.42 2253.48 0 0 600.93 -1592.46 0 0 836.3 1452.25 0 0 600.93 -1452.25 0 0 866.34 1622.51 0 0 600.93 -2283.53 0zm2533.92 0l921.42 -3505.42 1151.78 0 921.43 3505.42 -681.06 0 -190.3 -771.19 -1251.93 0 -190.3 771.19 -681.05 0zm1026.59 -1382.14l941.46 0 -425.66 -1707.64 -90.14 0 -425.66 1707.64zm2388.69 1382.14l0 -3505.42 1256.94 0 696.08 3054.72 90.14 0 0 -3054.72 651.01 0 0 3505.42 -1256.94 0 -696.07 -3054.72 -90.14 0 0 3054.72 -651.01 0zm5959.22 70.11c-434.01,0 -777.87,-121.02 -1031.6,-363.06 -253.73,-242.04 -380.59,-588.41 -380.59,-1039.11l0 -841.3c0,-450.69 126.86,-797.07 380.59,-1039.11 253.72,-242.04 597.59,-363.06 1031.6,-363.06 430.67,0 763.68,117.68 999.04,353.04 235.36,235.36 353.05,558.37 353.05,969l0 30.05 -651.01 0 0 -50.08c0,-206.98 -57.59,-377.25 -172.76,-510.79 -115.18,-133.54 -291.29,-200.31 -528.32,-200.31 -233.7,0 -417.31,71.78 -550.85,215.33 -133.54,143.56 -200.31,338.86 -200.31,585.91l0 861.33c0,243.71 66.77,438.18 200.31,583.4 133.54,145.23 317.15,217.84 550.85,217.84 237.03,0 413.13,-67.61 528.32,-202.81 115.18,-135.21 172.76,-304.64 172.76,-508.29l0 -90.14 651.01 0 0 70.11c0,410.64 -117.68,733.63 -353.05,969 -235.36,235.37 -568.38,353.05 -999.04,353.05zm1892.93 -70.11l0 -3505.42 661.02 0 0 1447.24 1302.01 0 0 -1447.24 661.02 0 0 3505.42 -661.02 0 0 -1457.26 -1302.01 0 0 1457.26 -661.02 0zm3044.71 0l921.42 -3505.42 1151.78 0 921.43 3505.42 -681.06 0 -190.3 -771.19 -1251.93 0 -190.3 771.19 -681.05 0zm1026.59 -1382.14l941.46 0 -425.66 -1707.64 -90.14 0 -425.66 1707.64zm2388.69 1382.14l0 -3505.42 661.02 0 0 3505.42 -661.02 0zm1322.05 0l0 -3505.42 1256.94 0 696.08 3054.72 90.14 0 0 -3054.72 651.01 0 0 3505.42 -1256.94 0 -696.07 -3054.72 -90.14 0 0 3054.72 -651.01 0z"
                />
              </svg>
            </div>
            <h1 className="text-5xl md:text-6xl font-light mb-6 leading-tight tracking-tight">
              Lightning-fast.
              <br />
              Enterprise-ready.
              <br />
              <span className="font-normal">Adaptive Network.</span>
            </h1>
            <p className="text-base text-muted-foreground mb-12 max-w-2xl leading-relaxed">
              A next-generation blockchain platform that surpasses XRP in speed, scalability, 
              and decentralization while maintaining regulatory compliance capabilities.
            </p>
            <div className="flex flex-col sm:flex-row gap-3">
              <Link href={getSubdomainUrl('validators')} className="btn-primary inline-flex items-center justify-center">
                Start Validating
                <ArrowRightIcon className="ml-2 w-4 h-4" />
              </Link>
              <Link href={getSubdomainUrl('docs')} className="btn-secondary inline-flex items-center justify-center">
                Documentation
              </Link>
            </div>
          </div>
        </div>
      </section>

      {/* Stats Section */}
      <section id="stats" className="border-b border-border/50 py-20">
        <div className="max-w-7xl mx-auto px-6 lg:px-8">
          <div className="grid grid-cols-2 md:grid-cols-4 gap-12">
            <div>
              <div className="text-3xl font-light mb-1 tracking-tight">{stats.tps.toLocaleString()}+</div>
              <div className="text-xs text-muted-foreground uppercase tracking-wider">Transactions Per Second</div>
            </div>
            <div>
              <div className="text-3xl font-light mb-1 tracking-tight">{stats.finality}</div>
              <div className="text-xs text-muted-foreground uppercase tracking-wider">Block Finality</div>
            </div>
            <div>
              <div className="text-3xl font-light mb-1 tracking-tight">{stats.cost}</div>
              <div className="text-xs text-muted-foreground uppercase tracking-wider">Per Transaction</div>
            </div>
            <div>
              <div className="text-3xl font-light mb-1 tracking-tight">{stats.validators}</div>
              <div className="text-xs text-muted-foreground uppercase tracking-wider">Validators</div>
            </div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section id="features" className="border-b border-border/50 py-24">
        <div className="max-w-7xl mx-auto px-6 lg:px-8">
          <h2 className="text-sm font-medium text-muted-foreground uppercase tracking-wider mb-16">Key Features</h2>
          <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-px bg-border/50">
            {features.map((feature, index) => (
              <div key={index} className="bg-card p-8 border border-border/50">
                <feature.icon className="w-5 h-5 text-foreground mb-6" />
                <h3 className="text-base font-medium mb-2 tracking-tight">{feature.title}</h3>
                <p className="text-sm text-muted-foreground leading-relaxed">{feature.description}</p>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* Services Section */}
      <section id="services" className="border-b border-border/50 py-24">
        <div className="max-w-7xl mx-auto px-6 lg:px-8">
          <h2 className="text-sm font-medium text-muted-foreground uppercase tracking-wider mb-16">Services</h2>
          <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-px bg-border/50">
            {services.map((service, index) => (
              <a
                key={index}
                href={service.url}
                target="_blank"
                rel="noopener noreferrer"
                className="bg-card p-8 border border-border/50 hover:border-border transition-all group"
              >
                <div className="flex items-start justify-between mb-4">
                  <service.icon className="w-5 h-5 text-foreground" />
                  <ArrowRightIcon className="w-4 h-4 text-muted-foreground group-hover:text-foreground transition-colors" />
                </div>
                <h3 className="text-base font-medium mb-1 tracking-tight">{service.name}</h3>
                <p className="text-xs text-muted-foreground font-mono">{service.url.replace('https://', '')}</p>
              </a>
            ))}
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer className="border-t border-border/50 py-16">
        <div className="max-w-7xl mx-auto px-6 lg:px-8">
          <div className="grid md:grid-cols-4 gap-12 mb-12">
            <div>
              <div className="flex items-center space-x-2 mb-6">
                <Image src="/lean_icon.png" alt="LEAN Chain" width={20} height={20} className="rounded-sm" />
                <span className="text-xs font-medium tracking-tight">LEAN CHAIN</span>
              </div>
              <p className="text-xs text-muted-foreground leading-relaxed">
                Lightning-fast, enterprise-ready blockchain platform.
              </p>
            </div>
            <div>
              <h4 className="text-xs font-medium mb-4 uppercase tracking-wider text-muted-foreground">Resources</h4>
              <ul className="space-y-3 text-xs text-foreground">
                <li><Link href={getSubdomainUrl('docs')} className="hover:text-muted-foreground transition-colors">Documentation</Link></li>
                <li><Link href={getSubdomainUrl('forum')} className="hover:text-muted-foreground transition-colors">Forum</Link></li>
                <li><Link href="https://github.com/dataswarmproject/lean" className="hover:text-muted-foreground transition-colors">GitHub</Link></li>
              </ul>
            </div>
            <div>
              <h4 className="text-xs font-medium mb-4 uppercase tracking-wider text-muted-foreground">Tools</h4>
              <ul className="space-y-3 text-xs text-foreground">
                <li><Link href={getSubdomainUrl('explorer')} className="hover:text-muted-foreground transition-colors">Block Explorer</Link></li>
                <li><Link href={getSubdomainUrl('validators')} className="hover:text-muted-foreground transition-colors">Validator Dashboard</Link></li>
                <li><Link href={getSubdomainUrl('staking')} className="hover:text-muted-foreground transition-colors">Staking Calculator</Link></li>
              </ul>
            </div>
            <div>
              <h4 className="text-xs font-medium mb-4 uppercase tracking-wider text-muted-foreground">Status</h4>
              <ul className="space-y-3 text-xs text-foreground">
                <li><Link href={getSubdomainUrl('status')} className="hover:text-muted-foreground transition-colors">Network Status</Link></li>
                <li><Link href={getSubdomainUrl('docs')} className="hover:text-muted-foreground transition-colors">API Reference</Link></li>
              </ul>
            </div>
          </div>
          <div className="border-t border-border/50 pt-8 text-xs text-muted-foreground">
            <p>&copy; 2025 LEAN Chain. All rights reserved. MIT / Apache-2.0 License.</p>
          </div>
        </div>
      </footer>
    </div>
  );
}

