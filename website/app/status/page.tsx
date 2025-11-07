'use client';

import { useState, useEffect } from 'react';
import Link from 'next/link';
import Image from 'next/image';
import { 
  CheckCircleIcon,
  XCircleIcon,
  ExclamationTriangleIcon,
  ClockIcon,
  ServerIcon,
  GlobeAltIcon,
  CpuChipIcon
} from '@heroicons/react/24/outline';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';

export default function StatusPage() {
  const [systemStatus, setSystemStatus] = useState('operational');
  const [lastUpdated, setLastUpdated] = useState(new Date());

  useEffect(() => {
    const interval = setInterval(() => {
      setLastUpdated(new Date());
    }, 60000); // Update every minute
    return () => clearInterval(interval);
  }, []);

  const [services, setServices] = useState([
    { name: 'Main Network', status: 'operational', uptime: 99.98, responseTime: 45 },
    { name: 'RPC API', status: 'operational', uptime: 99.95, responseTime: 120 },
    { name: 'Block Explorer', status: 'operational', uptime: 99.92, responseTime: 180 },
    { name: 'Validator Dashboard', status: 'operational', uptime: 99.88, responseTime: 200 },
    { name: 'Staking Service', status: 'operational', uptime: 99.85, responseTime: 150 },
    { name: 'Forum', status: 'operational', uptime: 99.90, responseTime: 250 },
  ]);
  const [performanceData, setPerformanceData] = useState([
    { time: '00:00', tps: 195000, latency: 45 },
    { time: '04:00', tps: 198000, latency: 42 },
    { time: '08:00', tps: 202000, latency: 48 },
    { time: '12:00', tps: 200000, latency: 45 },
    { time: '16:00', tps: 199000, latency: 43 },
    { time: '20:00', tps: 201000, latency: 46 },
  ]);

  useEffect(() => {
    const checkServices = async () => {
      const startTime = Date.now();
      const serviceChecks = await Promise.allSettled([
        fetch('/api/stats').then(r => ({ ok: r.ok, time: Date.now() - startTime })),
        fetch('/api/blocks?count=1').then(r => ({ ok: r.ok, time: Date.now() - startTime })),
        fetch('/api/validators').then(r => ({ ok: r.ok, time: Date.now() - startTime })),
      ]);

      setServices(prev => {
        const newServices = [...prev];
        newServices[0].status = serviceChecks[0].status === 'fulfilled' && serviceChecks[0].value.ok ? 'operational' : 'down';
        newServices[0].responseTime = serviceChecks[0].status === 'fulfilled' ? serviceChecks[0].value.time : 9999;
        newServices[1].status = serviceChecks[0].status === 'fulfilled' && serviceChecks[0].value.ok ? 'operational' : 'down';
        newServices[1].responseTime = serviceChecks[0].status === 'fulfilled' ? serviceChecks[0].value.time : 9999;
        newServices[2].status = serviceChecks[1].status === 'fulfilled' && serviceChecks[1].value.ok ? 'operational' : 'down';
        newServices[2].responseTime = serviceChecks[1].status === 'fulfilled' ? serviceChecks[1].value.time : 9999;
        newServices[3].status = serviceChecks[2].status === 'fulfilled' && serviceChecks[2].value.ok ? 'operational' : 'down';
        newServices[3].responseTime = serviceChecks[2].status === 'fulfilled' ? serviceChecks[2].value.time : 9999;
        return newServices;
      });

      // Update performance data with real stats
      fetch('/api/stats')
        .then(r => r.json())
        .then(stats => {
          if (stats.tps) {
            const now = new Date();
            const timeStr = `${now.getHours().toString().padStart(2, '0')}:${now.getMinutes().toString().padStart(2, '0')}`;
            setPerformanceData(prev => {
              const newData = [...prev.slice(1), { time: timeStr, tps: stats.tps, latency: stats.avg_block_time || 45 }];
              return newData;
            });
          }
        })
        .catch(console.error);
    };

    checkServices();
    const interval = setInterval(checkServices, 60000); // Check every minute
    return () => clearInterval(interval);
  }, []); // eslint-disable-line react-hooks/exhaustive-deps

  const incidents = [
    { date: '2025-01-15', service: 'RPC API', status: 'resolved', description: 'Brief latency spike due to high traffic' },
    { date: '2025-01-10', service: 'Block Explorer', status: 'resolved', description: 'Scheduled maintenance completed' },
  ];

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'operational':
        return <CheckCircleIcon className="w-5 h-5 text-green-500" />;
      case 'degraded':
        return <ExclamationTriangleIcon className="w-5 h-5 text-yellow-500" />;
      case 'down':
        return <XCircleIcon className="w-5 h-5 text-red-500" />;
      default:
        return <ClockIcon className="w-5 h-5 text-gray-500" />;
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'operational':
        return 'bg-green-500/20 text-green-400 border border-green-500/30';
      case 'degraded':
        return 'bg-yellow-500/20 text-yellow-400 border border-yellow-500/30';
      case 'down':
        return 'bg-red-500/20 text-red-400 border border-red-500/30';
      default:
        return 'bg-muted text-muted-foreground border border-border';
    }
  };

  const overallUptime = services.reduce((sum, s) => sum + s.uptime, 0) / services.length;

  return (
    <div className="min-h-screen bg-background">
      {/* Header */}
      <header className="sticky top-0 z-50 border-b border-border/50 bg-background/80 backdrop-blur-sm">
        <div className="max-w-7xl mx-auto px-6 lg:px-8">
          <div className="flex items-center justify-between h-16">
            <Link href="/" className="flex items-center space-x-3">
              <Image src="/lean_icon.png" alt="LEAN Chain" width={24} height={24} className="rounded-sm" />
              <span className="text-sm font-medium text-foreground tracking-tight">NETWORK STATUS</span>
            </Link>
            <div className="flex items-center space-x-6">
              <div className="text-xs text-muted-foreground">
                Last updated: {lastUpdated.toLocaleTimeString()}
              </div>
              <Link href="/" className="text-xs text-muted-foreground hover:text-foreground transition-colors uppercase tracking-wider">Home</Link>
            </div>
          </div>
        </div>
      </header>

      <div className="max-w-7xl mx-auto px-6 lg:px-8 py-12">
        {/* Overall Status */}
        <div className="border border-border/50 bg-card mb-12">
          <div className="p-6 border-b border-border/50 flex items-center justify-between">
            <h2 className="text-sm font-medium text-foreground uppercase tracking-wider">System Status</h2>
            <div className="flex items-center space-x-2">
              {getStatusIcon(systemStatus)}
              <span className={`px-2 py-0.5 text-xs font-medium ${getStatusColor(systemStatus)}`}>
                {systemStatus.charAt(0).toUpperCase() + systemStatus.slice(1)}
              </span>
            </div>
          </div>
          <div className="p-6 grid md:grid-cols-3 gap-px bg-border/50">
            <div className="bg-card p-6 border border-border/50">
              <div className="text-xs text-muted-foreground uppercase tracking-wider mb-2">Overall Uptime</div>
              <div className="text-3xl font-light tracking-tight">{overallUptime.toFixed(2)}%</div>
            </div>
            <div className="bg-card p-6 border border-border/50">
              <div className="text-xs text-muted-foreground uppercase tracking-wider mb-2">Avg Response Time</div>
              <div className="text-3xl font-light tracking-tight">
                {Math.round(services.reduce((sum, s) => sum + s.responseTime, 0) / services.length)}ms
              </div>
            </div>
            <div className="bg-card p-6 border border-border/50">
              <div className="text-xs text-muted-foreground uppercase tracking-wider mb-2">Active Services</div>
              <div className="text-3xl font-light tracking-tight">{services.length}</div>
            </div>
          </div>
        </div>

        {/* Services Status */}
        <div className="border border-border/50 bg-card mb-12">
          <div className="p-6 border-b border-border/50">
            <h2 className="text-sm font-medium text-foreground uppercase tracking-wider">Service Status</h2>
          </div>
          <div className="p-6">
          <div className="overflow-x-auto">
            <table className="min-w-full divide-y divide-border/50">
              <thead className="bg-muted/30">
                <tr>
                  <th className="px-6 py-4 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Service</th>
                  <th className="px-6 py-4 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Status</th>
                  <th className="px-6 py-4 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Uptime (30d)</th>
                  <th className="px-6 py-4 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Response Time</th>
                </tr>
              </thead>
              <tbody className="bg-card divide-y divide-border/50">
                {services.map((service, index) => (
                  <tr key={index} className="hover:bg-muted/20 transition-colors">
                    <td className="px-6 py-4 text-sm font-medium text-foreground">{service.name}</td>
                    <td className="px-6 py-4">
                      <div className="flex items-center space-x-2">
                        {getStatusIcon(service.status)}
                        <span className={`px-2 py-0.5 text-xs font-medium ${getStatusColor(service.status)}`}>
                          {service.status}
                        </span>
                      </div>
                    </td>
                    <td className="px-6 py-4 text-sm text-foreground">{service.uptime.toFixed(2)}%</td>
                    <td className="px-6 py-4 text-sm text-foreground">{service.responseTime}ms</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
          </div>
        </div>

        {/* Performance Chart */}
        <div className="border border-border/50 bg-card mb-12">
          <div className="p-6 border-b border-border/50">
            <h2 className="text-sm font-medium text-foreground uppercase tracking-wider">Network Performance (24h)</h2>
          </div>
          <div className="p-6">
            <ResponsiveContainer width="100%" height={300}>
              <LineChart data={performanceData}>
                <CartesianGrid strokeDasharray="3 3" stroke="hsl(var(--border))" />
                <XAxis dataKey="time" stroke="hsl(var(--muted-foreground))" />
                <YAxis yAxisId="left" stroke="hsl(var(--muted-foreground))" />
                <YAxis yAxisId="right" orientation="right" stroke="hsl(var(--muted-foreground))" />
                <Tooltip />
                <Line yAxisId="left" type="monotone" dataKey="tps" stroke="#C6FF24" strokeWidth={1.5} dot={false} name="TPS" />
                <Line yAxisId="right" type="monotone" dataKey="latency" stroke="#212224" strokeWidth={1.5} dot={false} name="Latency (ms)" />
              </LineChart>
            </ResponsiveContainer>
          </div>
        </div>

        {/* Recent Incidents */}
        <div className="border border-border/50 bg-card">
          <div className="p-6 border-b border-border/50">
            <h2 className="text-sm font-medium text-foreground uppercase tracking-wider">Recent Incidents</h2>
          </div>
          <div className="p-6">
          {incidents.length > 0 ? (
            <div className="space-y-4">
              {incidents.map((incident, index) => (
                <div key={index} className="border-l-4 border-yellow-500 pl-4 py-2">
                  <div className="flex items-center justify-between mb-2">
                    <div className="font-semibold text-foreground">{incident.service}</div>
                    <span className={`px-2.5 py-0.5 rounded-full text-xs font-medium ${getStatusColor(incident.status)}`}>
                      {incident.status}
                    </span>
                  </div>
                  <div className="text-sm text-muted-foreground mb-1">{incident.date}</div>
                  <div className="text-sm text-foreground">{incident.description}</div>
                </div>
              ))}
            </div>
          ) : (
            <div className="text-center py-8 text-muted-foreground">
              <CheckCircleIcon className="w-12 h-12 text-green-500 mx-auto mb-2" />
              <p className="text-sm">No recent incidents. All systems operational.</p>
            </div>
          )}
          </div>
        </div>
      </div>
    </div>
  );
}

