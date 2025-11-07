'use client';

import { useState, useEffect } from 'react';
import Link from 'next/link';
import Image from 'next/image';
import { 
  ShieldCheckIcon,
  ChartBarIcon,
  CurrencyDollarIcon,
  ClockIcon,
  CheckCircleIcon,
  XCircleIcon,
  ArrowTrendingUpIcon
} from '@heroicons/react/24/outline';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, BarChart, Bar } from 'recharts';
import { formatAddress, formatLEAN } from '@/lib/utils';
import type { Validator } from '@/lib/rpc-client';

export default function ValidatorsPage() {
  const [validators, setValidators] = useState<Validator[]>([]);
  const [stats, setStats] = useState({
    totalValidators: 0,
    activeValidators: 0,
    totalStaked: '0',
    avgUptime: 0,
    totalRewards: '0',
  });
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchData = async () => {
      try {
        setLoading(true);
        const res = await fetch('/api/validators');
        const data = await res.json();
        const validatorsList = data.validators || [];
        
        setValidators(validatorsList);
        
        const active = validatorsList.filter((v: Validator) => v.status === 'active');
        const totalStaked = validatorsList.length > 0
          ? validatorsList.reduce((sum: bigint, v: Validator) => {
              return sum + BigInt(v.total_staked || v.voting_power || '0');
            }, BigInt(0))
          : BigInt(0);
        const avgUptime = validatorsList.length > 0
          ? validatorsList.reduce((sum: number, v: Validator) => sum + (v.uptime || 0), 0) / validatorsList.length
          : 0;

        setStats({
          totalValidators: validatorsList.length,
          activeValidators: active.length,
          totalStaked: totalStaked.toString(),
          avgUptime,
          totalRewards: '0', // Would need separate calculation
        });
        setError(null);
      } catch (err: any) {
        console.error('Error fetching validators:', err);
        setError(err.message || 'Failed to load validators');
      } finally {
        setLoading(false);
      }
    };

    fetchData();
    const interval = setInterval(fetchData, 30000); // Refresh every 30 seconds
    return () => clearInterval(interval);
  }, []);

  const performanceData = [
    { time: '00:00', tps: 195000 },
    { time: '04:00', tps: 198000 },
    { time: '08:00', tps: 202000 },
    { time: '12:00', tps: 200000 },
    { time: '16:00', tps: 199000 },
    { time: '20:00', tps: 201000 },
  ];

  const stakingData = validators.slice(0, 10).map((v, i) => ({
    validator: `V${i + 1}`,
    stake: Number(BigInt(v.total_staked || v.voting_power || '0') / BigInt('1000000000000000000')),
  }));

  return (
    <div className="min-h-screen bg-background">
      {/* Header */}
      <header className="sticky top-0 z-50 border-b border-border/50 bg-background/80 backdrop-blur-sm">
        <div className="max-w-7xl mx-auto px-6 lg:px-8">
          <div className="flex items-center justify-between h-16">
            <Link href="/" className="flex items-center space-x-3">
              <Image src="/lean_icon.png" alt="LEAN Chain" width={24} height={24} className="rounded-sm" />
              <span className="text-sm font-medium text-foreground tracking-tight">VALIDATOR DASHBOARD</span>
            </Link>
            <button className="btn-primary">Connect Wallet</button>
          </div>
        </div>
      </header>

      <div className="max-w-7xl mx-auto px-6 lg:px-8 py-12">
        {/* Stats Overview */}
        <div className="grid grid-cols-2 md:grid-cols-5 gap-px bg-border/50 mb-12">
          <div className="bg-card p-6 border border-border/50">
            <div className="text-2xl font-light mb-1 tracking-tight">{stats.totalValidators}</div>
            <div className="text-xs text-muted-foreground uppercase tracking-wider">Total Validators</div>
          </div>
          <div className="bg-card p-6 border border-border/50">
            <div className="text-2xl font-light mb-1 tracking-tight">{stats.activeValidators}</div>
            <div className="text-xs text-muted-foreground uppercase tracking-wider">Active</div>
          </div>
          <div className="bg-card p-6 border border-border/50">
            <div className="text-2xl font-light mb-1 tracking-tight">{formatLEAN(stats.totalStaked)}</div>
            <div className="text-xs text-muted-foreground uppercase tracking-wider">Total Staked</div>
          </div>
          <div className="bg-card p-6 border border-border/50">
            <div className="text-2xl font-light mb-1 tracking-tight">{stats.avgUptime.toFixed(1)}%</div>
            <div className="text-xs text-muted-foreground uppercase tracking-wider">Avg Uptime</div>
          </div>
          <div className="bg-card p-6 border border-border/50">
            <div className="text-2xl font-light mb-1 tracking-tight">{formatLEAN(stats.totalRewards)}</div>
            <div className="text-xs text-muted-foreground uppercase tracking-wider">Total Rewards</div>
          </div>
        </div>

        {/* Performance Chart */}
        <div className="border border-border/50 bg-card mb-12">
          <div className="p-6 border-b border-border/50">
            <h2 className="text-sm font-medium text-foreground uppercase tracking-wider flex items-center space-x-2">
              <ChartBarIcon className="w-4 h-4" />
              <span>Network Performance</span>
            </h2>
          </div>
          <div className="p-6">
            <ResponsiveContainer width="100%" height={300}>
              <LineChart data={performanceData}>
                <CartesianGrid strokeDasharray="3 3" stroke="hsl(var(--border))" />
                <XAxis dataKey="time" stroke="hsl(var(--muted-foreground))" />
                <YAxis stroke="hsl(var(--muted-foreground))" />
                <Tooltip />
                <Line type="monotone" dataKey="tps" stroke="#C6FF24" strokeWidth={1.5} dot={false} />
              </LineChart>
            </ResponsiveContainer>
          </div>
        </div>

        {/* Validators List */}
        <div className="border border-border/50 bg-card mb-12">
          <div className="p-6 border-b border-border/50 flex items-center justify-between">
            <h2 className="text-sm font-medium text-foreground uppercase tracking-wider flex items-center space-x-2">
              <ShieldCheckIcon className="w-4 h-4" />
              <span>Validators</span>
            </h2>
            <button className="btn-primary">Become Validator</button>
          </div>
          <div className="overflow-x-auto">
            <table className="min-w-full divide-y divide-border/50">
              <thead className="bg-muted/30">
                <tr>
                  <th className="px-6 py-4 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Validator</th>
                  <th className="px-6 py-4 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Status</th>
                  <th className="px-6 py-4 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Stake</th>
                  <th className="px-6 py-4 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Commission</th>
                  <th className="px-6 py-4 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Uptime</th>
                  <th className="px-6 py-4 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Rewards</th>
                </tr>
              </thead>
              <tbody className="bg-card divide-y divide-border/50">
                {loading ? (
                  <tr>
                    <td colSpan={6} className="px-6 py-8 text-center text-sm text-muted-foreground">
                      Loading validators...
                    </td>
                  </tr>
                ) : error ? (
                  <tr>
                    <td colSpan={6} className="px-6 py-8 text-center text-sm text-destructive">
                      {error}
                    </td>
                  </tr>
                ) : validators.length === 0 ? (
                  <tr>
                    <td colSpan={6} className="px-6 py-8 text-center text-sm text-muted-foreground">
                      No validators found
                    </td>
                  </tr>
                ) : (
                  validators.map((validator) => (
                    <tr key={validator.address} className="hover:bg-muted/20 transition-colors">
                      <td className="px-6 py-4">
                        <div>
                          <div className="text-sm font-medium text-foreground mb-1">{validator.moniker || 'Validator'}</div>
                          <div className="text-xs text-muted-foreground font-mono">{formatAddress(validator.address)}</div>
                        </div>
                      </td>
                      <td className="px-6 py-4">
                        {validator.status === 'active' ? (
                          <span className="inline-flex items-center px-2 py-0.5 text-xs font-medium bg-green-500/10 text-green-400 border border-green-500/20">
                            <CheckCircleIcon className="w-3 h-3 mr-1.5" />
                            Active
                          </span>
                        ) : (
                          <span className="inline-flex items-center px-2 py-0.5 text-xs font-medium bg-red-500/10 text-red-400 border border-red-500/20">
                            <XCircleIcon className="w-3 h-3 mr-1.5" />
                            {validator.status || 'Inactive'}
                          </span>
                        )}
                      </td>
                      <td className="px-6 py-4 text-sm text-foreground font-mono">{formatLEAN(validator.total_staked || validator.voting_power || '0')} LEAN</td>
                      <td className="px-6 py-4 text-sm text-foreground">{validator.commission || 0}%</td>
                      <td className="px-6 py-4 text-sm text-foreground">{validator.uptime?.toFixed(1) || 'N/A'}%</td>
                      <td className="px-6 py-4 text-sm font-mono text-foreground">{formatLEAN(validator.rewards || '0')} LEAN</td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>
        </div>

        {/* Staking Distribution */}
        <div className="border border-border/50 bg-card">
          <div className="p-6 border-b border-border/50">
            <h2 className="text-sm font-medium text-foreground uppercase tracking-wider flex items-center space-x-2">
              <CurrencyDollarIcon className="w-4 h-4" />
              <span>Staking Distribution</span>
            </h2>
          </div>
          <div className="p-6">
            <ResponsiveContainer width="100%" height={300}>
              <BarChart data={stakingData}>
                <CartesianGrid strokeDasharray="3 3" stroke="hsl(var(--border))" />
                <XAxis dataKey="validator" stroke="hsl(var(--muted-foreground))" />
                <YAxis stroke="hsl(var(--muted-foreground))" />
                <Tooltip />
                <Bar dataKey="stake" fill="#C6FF24" />
              </BarChart>
            </ResponsiveContainer>
          </div>
        </div>
      </div>
    </div>
  );
}

