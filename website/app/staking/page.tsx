'use client';

import { useState, useEffect } from 'react';
import Link from 'next/link';
import Image from 'next/image';
import { 
  CalculatorIcon,
  CurrencyDollarIcon,
  ChartBarIcon,
  InformationCircleIcon
} from '@heroicons/react/24/outline';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';

export default function StakingPage() {
  const [stakeAmount, setStakeAmount] = useState(10000);
  const [validatorCommission, setValidatorCommission] = useState(5);
  const [stakingPeriod, setStakingPeriod] = useState(365);

  const annualRewardRate = 8; // 8% annual inflation
  const validatorUptime = 99.5;
  const networkUptime = 99.8;

  // Calculate rewards
  const calculateRewards = () => {
    const baseReward = (stakeAmount * annualRewardRate) / 100;
    const commission = (baseReward * validatorCommission) / 100;
    const netReward = baseReward - commission;
    const adjustedReward = netReward * (validatorUptime / 100);
    const dailyReward = adjustedReward / 365;
    const totalReward = dailyReward * stakingPeriod;
    
    return {
      baseReward,
      commission,
      netReward,
      adjustedReward,
      dailyReward,
      totalReward,
      finalAmount: stakeAmount + totalReward,
      apy: (adjustedReward / stakeAmount) * 100,
    };
  };

  const rewards = calculateRewards();

  const projectionData = Array.from({ length: Math.min(stakingPeriod, 365) }, (_, i) => ({
    day: i + 1,
    amount: stakeAmount + (rewards.dailyReward * (i + 1)),
  }));

  const [validators, setValidators] = useState<Array<{
    name: string;
    commission: number;
    uptime: number;
    totalStaked: number;
    address: string;
  }>>([]);
  const [loadingValidators, setLoadingValidators] = useState(true);

  useEffect(() => {
    const fetchValidators = async () => {
      try {
        const res = await fetch('/api/validators');
        if (res.ok) {
          const data = await res.json();
          const validatorsList = (data.validators || []).slice(0, 3).map((v: any) => ({
            name: v.moniker || 'Validator',
            commission: v.commission || 5,
            uptime: v.uptime || 99.5,
            totalStaked: Number(BigInt(v.total_staked || v.voting_power || '0') / BigInt('1000000000000000000')),
            address: v.address,
          }));
          setValidators(validatorsList);
        }
      } catch (err) {
        console.error('Error fetching validators:', err);
      } finally {
        setLoadingValidators(false);
      }
    };
    fetchValidators();
  }, []);

  return (
    <div className="min-h-screen bg-background">
      {/* Header */}
      <header className="bg-card border-b border-border shadow-sm">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
          <div className="flex items-center justify-between">
            <Link href="/" className="flex items-center space-x-2">
              <Image src="/lean_icon.png" alt="LEAN Chain" width={32} height={32} className="rounded-lg" />
              <span className="text-xl font-bold gradient-text">Staking Calculator</span>
            </Link>
            <div className="flex items-center space-x-4">
              <Link href="/validators" className="text-muted-foreground hover:text-primary-400 transition-colors">Validator Dashboard</Link>
              <Link href="/" className="text-muted-foreground hover:text-primary-400 transition-colors">Home</Link>
            </div>
          </div>
        </div>
      </header>

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="grid lg:grid-cols-3 gap-8">
          {/* Calculator */}
          <div className="lg:col-span-2">
            <div className="card mb-8">
              <h2 className="text-2xl font-bold mb-6 flex items-center space-x-2 text-foreground">
                <CalculatorIcon className="w-6 h-6 text-primary-400" />
                <span>Staking Calculator</span>
              </h2>

              {/* Inputs */}
              <div className="space-y-6">
                <div>
                  <label className="block text-sm font-medium text-foreground mb-2">
                    Stake Amount (LEAN)
                  </label>
                  <input
                    type="number"
                    value={stakeAmount}
                    onChange={(e) => setStakeAmount(Number(e.target.value))}
                    className="w-full px-4 py-2 bg-card border border-border rounded-lg focus:ring-2 focus:ring-primary-400 focus:border-transparent text-foreground"
                    min="1"
                  />
                </div>

                <div>
                  <label className="block text-sm font-medium text-foreground mb-2">
                    Validator Commission (%)
                  </label>
                  <input
                    type="number"
                    value={validatorCommission}
                    onChange={(e) => setValidatorCommission(Number(e.target.value))}
                    className="w-full px-4 py-2 bg-card border border-border rounded-lg focus:ring-2 focus:ring-primary-400 focus:border-transparent text-foreground"
                    min="0"
                    max="100"
                    step="0.1"
                  />
                </div>

                <div>
                  <label className="block text-sm font-medium text-foreground mb-2">
                    Staking Period (days)
                  </label>
                  <input
                    type="number"
                    value={stakingPeriod}
                    onChange={(e) => setStakingPeriod(Number(e.target.value))}
                    className="w-full px-4 py-2 bg-card border border-border rounded-lg focus:ring-2 focus:ring-primary-400 focus:border-transparent text-foreground"
                    min="1"
                    max="3650"
                  />
                </div>
              </div>

              {/* Results */}
              <div className="mt-8 pt-8 border-t border-border">
                <h3 className="text-lg font-semibold mb-4 text-foreground">Projected Returns</h3>
                <div className="grid grid-cols-2 gap-4">
                  <div className="bg-muted/30 rounded-lg p-4">
                    <div className="text-sm text-muted-foreground mb-1">APY</div>
                    <div className="text-2xl font-bold gradient-text">{rewards.apy.toFixed(2)}%</div>
                  </div>
                  <div className="bg-muted/30 rounded-lg p-4">
                    <div className="text-sm text-muted-foreground mb-1">Daily Reward</div>
                    <div className="text-2xl font-bold gradient-text">{rewards.dailyReward.toFixed(2)} LEAN</div>
                  </div>
                  <div className="bg-muted/30 rounded-lg p-4">
                    <div className="text-sm text-muted-foreground mb-1">Total Reward</div>
                    <div className="text-2xl font-bold gradient-text">{rewards.totalReward.toFixed(2)} LEAN</div>
                  </div>
                  <div className="bg-muted/30 rounded-lg p-4">
                    <div className="text-sm text-muted-foreground mb-1">Final Amount</div>
                    <div className="text-2xl font-bold gradient-text">{rewards.finalAmount.toFixed(2)} LEAN</div>
                  </div>
                </div>
              </div>

              {/* Projection Chart */}
              <div className="mt-8">
                <h3 className="text-lg font-semibold mb-4 text-foreground">Reward Projection</h3>
                <ResponsiveContainer width="100%" height={300}>
                  <LineChart data={projectionData}>
                    <CartesianGrid strokeDasharray="3 3" />
                    <XAxis dataKey="day" />
                    <YAxis />
                    <Tooltip />
                    <Line type="monotone" dataKey="amount" stroke="#C6FF24" strokeWidth={2} />
                  </LineChart>
                </ResponsiveContainer>
              </div>
            </div>

            {/* Info Box */}
            <div className="card bg-blue-500/10 border-blue-500/30">
              <div className="flex items-start space-x-3">
                <InformationCircleIcon className="w-6 h-6 text-blue-400 flex-shrink-0 mt-1" />
                <div className="text-sm text-foreground">
                  <p className="font-semibold mb-2">Important Information</p>
                  <ul className="list-disc list-inside space-y-1 text-muted-foreground">
                    <li>Rewards are calculated based on 8% annual inflation rate</li>
                    <li>Validator commission and uptime affect your actual returns</li>
                    <li>Network uptime: {networkUptime}%</li>
                    <li>Unbonding period: 21 days</li>
                    <li>Slashing risk: 5% for double-signing, 0.01% for downtime</li>
                  </ul>
                </div>
              </div>
            </div>
          </div>

          {/* Validator Selection */}
          <div>
            <div className="card sticky top-8">
              <h3 className="text-xl font-bold mb-4 text-foreground">Recommended Validators</h3>
              {loadingValidators ? (
                <div className="text-center py-8 text-muted-foreground">Loading validators...</div>
              ) : validators.length === 0 ? (
                <div className="text-center py-8 text-muted-foreground">No validators available</div>
              ) : (
                <div className="space-y-4">
                  {validators.map((validator, index) => (
                  <div
                    key={index}
                    className="border-2 border-border rounded-lg p-4 hover:border-primary-400 cursor-pointer transition-colors"
                    onClick={() => setValidatorCommission(validator.commission)}
                  >
                    <div className="font-semibold mb-2 text-foreground">{validator.name}</div>
                    <div className="text-sm space-y-1">
                      <div className="flex justify-between">
                        <span className="text-muted-foreground">Commission:</span>
                        <span className="font-medium text-foreground">{validator.commission}%</span>
                      </div>
                      <div className="flex justify-between">
                        <span className="text-muted-foreground">Uptime:</span>
                        <span className="font-medium text-foreground">{validator.uptime}%</span>
                      </div>
                      <div className="flex justify-between">
                        <span className="text-muted-foreground">Total Staked:</span>
                        <span className="font-medium text-foreground">{(validator.totalStaked / 1000000).toFixed(1)}M</span>
                      </div>
                    </div>
                  </div>
                  ))}
                </div>
              )}
              <Link href="/validators" className="btn-primary w-full mt-4 text-center block">
                View All Validators
              </Link>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

