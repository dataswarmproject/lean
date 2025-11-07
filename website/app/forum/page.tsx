'use client';

import { useState } from 'react';
import Link from 'next/link';
import Image from 'next/image';
import { 
  MagnifyingGlassIcon, 
  ChatBubbleLeftRightIcon,
  FireIcon,
  ClockIcon,
  UserIcon,
  ShieldCheckIcon,
  CodeBracketIcon,
  CurrencyDollarIcon,
  ClipboardDocumentCheckIcon,
  MegaphoneIcon
} from '@heroicons/react/24/outline';

export default function ForumPage() {
  const [searchQuery, setSearchQuery] = useState('');

  const categories = [
    { name: 'General Discussion', count: 124, icon: ChatBubbleLeftRightIcon, color: 'bg-blue-100 text-blue-600' },
    { name: 'Validator Support', count: 89, icon: ShieldCheckIcon, color: 'bg-green-100 text-green-600' },
    { name: 'Development', count: 156, icon: CodeBracketIcon, color: 'bg-purple-100 text-purple-600' },
    { name: 'DeFi & Trading', count: 203, icon: CurrencyDollarIcon, color: 'bg-yellow-100 text-yellow-600' },
    { name: 'Governance', count: 67, icon: ClipboardDocumentCheckIcon, color: 'bg-red-100 text-red-600' },
    { name: 'Announcements', count: 45, icon: MegaphoneIcon, color: 'bg-indigo-100 text-indigo-600' },
  ];

  const recentPosts = [
    { title: 'Mainnet Launch Date Announced', author: 'Admin', time: '2 hours ago', replies: 23, views: 456, category: 'Announcements' },
    { title: 'Validator Setup Guide - Step by Step', author: 'ValidatorPro', time: '5 hours ago', replies: 12, views: 234, category: 'Validator Support' },
    { title: 'New DEX Features Coming Soon', author: 'DevTeam', time: '1 day ago', replies: 45, views: 789, category: 'Development' },
    { title: 'Staking Rewards Calculation', author: 'CryptoUser', time: '2 days ago', replies: 8, views: 123, category: 'DeFi & Trading' },
  ];

  return (
    <div className="min-h-screen bg-background">
      {/* Header */}
      <header className="bg-card border-b border-border shadow-sm">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
          <div className="flex items-center justify-between">
            <Link href="/" className="flex items-center space-x-2">
              <Image src="/lean_icon.png" alt="LEAN Chain" width={32} height={32} className="rounded-lg" />
              <span className="text-xl font-bold gradient-text">LEAN Chain Forum</span>
            </Link>
            <div className="flex items-center space-x-4">
              <button className="btn-primary text-sm">Sign In</button>
            </div>
          </div>
        </div>
      </header>

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Search Bar */}
        <div className="mb-8">
          <div className="relative">
            <MagnifyingGlassIcon className="absolute left-4 top-1/2 transform -translate-y-1/2 w-5 h-5 text-muted-foreground" />
            <input
              type="text"
              placeholder="Search discussions..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full pl-12 pr-4 py-3 bg-card border border-border rounded-lg focus:ring-2 focus:ring-primary-400 focus:border-transparent text-foreground placeholder:text-muted-foreground"
            />
          </div>
        </div>

        {/* Categories */}
        <div className="mb-8">
          <h2 className="text-2xl font-bold mb-4 text-foreground">Categories</h2>
          <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-4">
            {categories.map((category, index) => (
              <div key={index} className="card cursor-pointer hover:scale-105 transition-transform">
                <div className="flex items-center justify-between">
                  <div className="flex items-center space-x-3">
                    <div className={`w-12 h-12 rounded-lg ${category.color} flex items-center justify-center`}>
                      <category.icon className="w-6 h-6" />
                    </div>
                    <div>
                      <h3 className="font-semibold text-foreground">{category.name}</h3>
                      <p className="text-sm text-muted-foreground">{category.count} topics</p>
                    </div>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Recent Posts */}
        <div>
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-2xl font-bold text-foreground">Recent Discussions</h2>
            <button className="btn-primary text-sm">New Topic</button>
          </div>
          <div className="bg-card rounded-xl shadow-lg overflow-hidden border border-border">
            <table className="min-w-full divide-y divide-border">
              <thead className="bg-muted/50">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Topic</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Author</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Replies</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Views</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Last Activity</th>
                </tr>
              </thead>
              <tbody className="bg-card divide-y divide-border">
                {recentPosts.map((post, index) => (
                  <tr key={index} className="hover:bg-muted/30 cursor-pointer transition-colors">
                    <td className="px-6 py-4">
                      <div className="flex items-center space-x-2">
                        <FireIcon className="w-4 h-4 text-orange-500" />
                        <div>
                          <div className="text-sm font-medium text-foreground">{post.title}</div>
                          <div className="text-xs text-muted-foreground">{post.category}</div>
                        </div>
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="flex items-center space-x-2">
                        <UserIcon className="w-4 h-4 text-muted-foreground" />
                        <span className="text-sm text-foreground">{post.author}</span>
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-muted-foreground">{post.replies}</td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-muted-foreground">{post.views}</td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-muted-foreground">
                      <div className="flex items-center space-x-1">
                        <ClockIcon className="w-4 h-4" />
                        <span>{post.time}</span>
                      </div>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  );
}

