'use client';

import { useState, useEffect } from 'react';
import Link from 'next/link';
import Image from 'next/image';
import { 
  MagnifyingGlassIcon,
  CubeIcon,
  ArrowRightIcon,
  ClockIcon,
  CurrencyDollarIcon,
  UserIcon
} from '@heroicons/react/24/outline';
import { formatAddress, formatLEAN, formatRelativeTime } from '@/lib/utils';
import type { Block, Transaction } from '@/lib/rpc-client';

export default function ExplorerPage() {
  const [searchQuery, setSearchQuery] = useState('');
  const [recentBlocks, setRecentBlocks] = useState<Block[]>([]);
  const [recentTransactions, setRecentTransactions] = useState<Transaction[]>([]);
  const [stats, setStats] = useState({
    currentHeight: 0,
    totalTransactions: 0,
    avgBlockTime: '<100ms',
    totalSupply: '100,000,000,000',
    activeAccounts: 0,
  });
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchData = async () => {
      try {
        setLoading(true);
        const [blocksRes, txsRes, statsRes] = await Promise.all([
          fetch('/api/blocks?count=10'),
          fetch('/api/transactions?count=10'),
          fetch('/api/stats'),
        ]);

        const blocks = await blocksRes.json();
        const txs = await txsRes.json();
        const networkStats = await statsRes.json();

        setRecentBlocks(blocks);
        setRecentTransactions(txs);
        setStats({
          currentHeight: networkStats.current_height || 0,
          totalTransactions: networkStats.total_transactions || 0,
          avgBlockTime: networkStats.avg_block_time || '<100ms',
          totalSupply: networkStats.total_supply || '100000000000000000000000000000',
          activeAccounts: networkStats.total_accounts || 0,
        });
        setError(null);
      } catch (err: any) {
        console.error('Error fetching explorer data:', err);
        setError(err.message || 'Failed to load data');
      } finally {
        setLoading(false);
      }
    };

    fetchData();
    const interval = setInterval(fetchData, 10000); // Refresh every 10 seconds
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="min-h-screen bg-background">
      {/* Header */}
      <header className="sticky top-0 z-50 border-b border-border/50 bg-background/80 backdrop-blur-sm">
        <div className="max-w-7xl mx-auto px-6 lg:px-8">
          <div className="flex items-center justify-between h-16">
            <Link href="/" className="flex items-center space-x-3">
              <Image src="/lean_icon.png" alt="LEAN Chain" width={24} height={24} className="rounded-sm" />
              <span className="text-sm font-medium text-foreground tracking-tight">BLOCK EXPLORER</span>
            </Link>
            <Link href="/" className="text-xs text-muted-foreground hover:text-foreground transition-colors uppercase tracking-wider">Home</Link>
          </div>
        </div>
      </header>

      <div className="max-w-7xl mx-auto px-6 lg:px-8 py-12">
        {/* Search Bar */}
        <div className="mb-12">
          <div className="relative">
            <MagnifyingGlassIcon className="absolute left-4 top-1/2 transform -translate-y-1/2 w-4 h-4 text-muted-foreground" />
            <input
              type="text"
              placeholder="Search by block height, transaction hash, or address..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full pl-11 pr-4 py-3 bg-card border border-border/50 focus:border-border focus:outline-none text-sm text-foreground placeholder:text-muted-foreground"
            />
          </div>
        </div>

        {/* Stats */}
        <div className="grid grid-cols-2 md:grid-cols-5 gap-px bg-border/50 mb-12">
          <div className="bg-card p-6 border border-border/50">
            <div className="text-xs text-muted-foreground uppercase tracking-wider mb-2">Current Height</div>
            <div className="text-2xl font-light tracking-tight">{stats.currentHeight.toLocaleString()}</div>
          </div>
          <div className="bg-card p-6 border border-border/50">
            <div className="text-xs text-muted-foreground uppercase tracking-wider mb-2">Total Transactions</div>
            <div className="text-2xl font-light tracking-tight">{(stats.totalTransactions / 1000000).toFixed(1)}M</div>
          </div>
          <div className="bg-card p-6 border border-border/50">
            <div className="text-xs text-muted-foreground uppercase tracking-wider mb-2">Avg Block Time</div>
            <div className="text-2xl font-light tracking-tight">{stats.avgBlockTime}</div>
          </div>
          <div className="bg-card p-6 border border-border/50">
            <div className="text-xs text-muted-foreground uppercase tracking-wider mb-2">Total Supply</div>
            <div className="text-2xl font-light tracking-tight">{formatLEAN(stats.totalSupply)}</div>
          </div>
          <div className="bg-card p-6 border border-border/50">
            <div className="text-xs text-muted-foreground uppercase tracking-wider mb-2">Active Accounts</div>
            <div className="text-2xl font-light tracking-tight">{(stats.activeAccounts / 1000).toFixed(1)}K</div>
          </div>
        </div>

        {/* Recent Blocks */}
        <div className="border border-border/50 bg-card mb-12">
          <div className="p-6 border-b border-border/50 flex items-center justify-between">
            <h2 className="text-sm font-medium text-foreground uppercase tracking-wider flex items-center space-x-2">
              <CubeIcon className="w-4 h-4" />
              <span>Recent Blocks</span>
            </h2>
            <Link href="/explorer/blocks" className="text-xs text-muted-foreground hover:text-foreground transition-colors uppercase tracking-wider flex items-center space-x-1">
              <span>View All</span>
              <ArrowRightIcon className="w-3 h-3" />
            </Link>
          </div>
          <div className="overflow-x-auto">
            <table className="min-w-full divide-y divide-border/50">
              <thead className="bg-muted/30">
                <tr>
                  <th className="px-6 py-4 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Height</th>
                  <th className="px-6 py-4 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Hash</th>
                  <th className="px-6 py-4 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Transactions</th>
                  <th className="px-6 py-4 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Validator</th>
                  <th className="px-6 py-4 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Time</th>
                </tr>
              </thead>
              <tbody className="bg-card divide-y divide-border/50">
                {loading ? (
                  <tr>
                    <td colSpan={5} className="px-6 py-8 text-center text-sm text-muted-foreground">
                      Loading blocks...
                    </td>
                  </tr>
                ) : error ? (
                  <tr>
                    <td colSpan={5} className="px-6 py-8 text-center text-sm text-destructive">
                      {error}
                    </td>
                  </tr>
                ) : recentBlocks.length === 0 ? (
                  <tr>
                    <td colSpan={5} className="px-6 py-8 text-center text-sm text-muted-foreground">
                      No blocks found
                    </td>
                  </tr>
                ) : (
                  recentBlocks.map((block) => (
                    <tr key={block.height} className="hover:bg-muted/20 cursor-pointer transition-colors">
                      <td className="px-6 py-4 text-sm font-medium text-foreground">{block.height.toLocaleString()}</td>
                      <td className="px-6 py-4 text-xs font-mono text-foreground">{formatAddress(block.hash)}</td>
                      <td className="px-6 py-4 text-sm text-foreground">{(block.transactions?.length || block.transaction_count || 0).toLocaleString()}</td>
                      <td className="px-6 py-4 text-xs font-mono text-foreground">{formatAddress(block.proposer)}</td>
                      <td className="px-6 py-4 text-xs text-muted-foreground">
                        <div className="flex items-center space-x-1.5">
                          <ClockIcon className="w-3 h-3" />
                          <span>{formatRelativeTime(block.timestamp)}</span>
                        </div>
                      </td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>
        </div>

        {/* Recent Transactions */}
        <div className="border border-border/50 bg-card">
          <div className="p-6 border-b border-border/50 flex items-center justify-between">
            <h2 className="text-sm font-medium text-foreground uppercase tracking-wider flex items-center space-x-2">
              <ArrowRightIcon className="w-4 h-4" />
              <span>Recent Transactions</span>
            </h2>
            <Link href="/explorer/transactions" className="text-xs text-muted-foreground hover:text-foreground transition-colors uppercase tracking-wider flex items-center space-x-1">
              <span>View All</span>
              <ArrowRightIcon className="w-3 h-3" />
            </Link>
          </div>
          <div className="overflow-x-auto">
            <table className="min-w-full divide-y divide-border/50">
              <thead className="bg-muted/30">
                <tr>
                  <th className="px-6 py-4 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Hash</th>
                  <th className="px-6 py-4 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">From</th>
                  <th className="px-6 py-4 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">To</th>
                  <th className="px-6 py-4 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Amount</th>
                  <th className="px-6 py-4 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Time</th>
                  <th className="px-6 py-4 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Status</th>
                </tr>
              </thead>
              <tbody className="bg-card divide-y divide-border/50">
                {loading ? (
                  <tr>
                    <td colSpan={6} className="px-6 py-8 text-center text-sm text-muted-foreground">
                      Loading transactions...
                    </td>
                  </tr>
                ) : error ? (
                  <tr>
                    <td colSpan={6} className="px-6 py-8 text-center text-sm text-destructive">
                      {error}
                    </td>
                  </tr>
                ) : recentTransactions.length === 0 ? (
                  <tr>
                    <td colSpan={6} className="px-6 py-8 text-center text-sm text-muted-foreground">
                      No transactions found
                    </td>
                  </tr>
                ) : (
                  recentTransactions.map((tx) => (
                    <tr key={tx.hash} className="hover:bg-muted/20 cursor-pointer transition-colors">
                      <td className="px-6 py-4 text-xs font-mono text-foreground">{formatAddress(tx.hash)}</td>
                      <td className="px-6 py-4 text-xs font-mono text-foreground">{formatAddress(tx.from)}</td>
                      <td className="px-6 py-4 text-xs font-mono text-foreground">{formatAddress(tx.to)}</td>
                      <td className="px-6 py-4 text-sm font-mono text-foreground">{formatLEAN(tx.amount)} LEAN</td>
                      <td className="px-6 py-4 text-xs text-muted-foreground">
                        <div className="flex items-center space-x-1.5">
                          <ClockIcon className="w-3 h-3" />
                          <span>{tx.timestamp ? formatRelativeTime(tx.timestamp) : 'N/A'}</span>
                        </div>
                      </td>
                      <td className="px-6 py-4">
                        <span className={`inline-flex items-center px-2 py-0.5 text-xs font-medium ${
                          (tx.status === 1 || tx.status === undefined) 
                            ? 'bg-green-500/10 text-green-400 border border-green-500/20' 
                            : 'bg-red-500/10 text-red-400 border border-red-500/20'
                        }`}>
                          {tx.status === 1 || tx.status === undefined ? 'success' : 'failed'}
                        </span>
                      </td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  );
}

