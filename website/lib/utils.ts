import { type ClassValue, clsx } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

/**
 * Format LEAN token amount
 */
export function formatLEAN(amount: string | number | bigint): string {
  const num = typeof amount === 'bigint' ? Number(amount) : typeof amount === 'string' ? parseFloat(amount) : amount;
  if (isNaN(num)) return '0';
  
  if (num >= 1e9) {
    return (num / 1e9).toFixed(2) + 'B';
  } else if (num >= 1e6) {
    return (num / 1e6).toFixed(2) + 'M';
  } else if (num >= 1e3) {
    return (num / 1e3).toFixed(2) + 'K';
  }
  return num.toLocaleString('en-US', { maximumFractionDigits: 2 });
}

/**
 * Format blockchain address (truncate and add ellipsis)
 */
export function formatAddress(address: string, start: number = 6, end: number = 4): string {
  if (!address || address.length < start + end) return address;
  return `${address.slice(0, start)}...${address.slice(-end)}`;
}

/**
 * Format timestamp to relative time
 */
export function formatRelativeTime(timestamp: number | string): string {
  const date = typeof timestamp === 'string' ? new Date(timestamp) : new Date(timestamp * 1000);
  const now = new Date();
  const diff = now.getTime() - date.getTime();
  const seconds = Math.floor(diff / 1000);
  const minutes = Math.floor(seconds / 60);
  const hours = Math.floor(minutes / 60);
  const days = Math.floor(hours / 24);

  if (seconds < 60) return 'Just now';
  if (minutes < 60) return `${minutes}m ago`;
  if (hours < 24) return `${hours}h ago`;
  if (days < 7) return `${days}d ago`;
  return date.toLocaleDateString();
}

/**
 * Get base URL dynamically based on current hostname
 * Works for localhost, Vercel preview, and production
 */
export function getBaseUrl(): string {
  if (typeof window !== 'undefined') {
    // Client-side: use current origin
    return window.location.origin;
  }
  
  // Server-side: use environment variable or default
  if (process.env.NEXT_PUBLIC_BASE_URL) {
    return process.env.NEXT_PUBLIC_BASE_URL;
  }
  
  if (process.env.VERCEL_URL) {
    return `https://${process.env.VERCEL_URL}`;
  }
  
  // Fallback for local development
  return 'http://localhost:3000';
}

/**
 * Get subdomain URL (e.g., docs, forum, validators, etc.)
 * Works dynamically for localhost (path-based) and production (subdomain-based)
 */
export function getSubdomainUrl(subdomain: string): string {
  // Client-side: use current origin
  if (typeof window !== 'undefined') {
    const origin = window.location.origin;
    
    // For localhost, use path-based routing
    if (origin.includes('localhost') || origin.includes('127.0.0.1')) {
      return `${origin}/${subdomain}`;
    }
    
    // For production/preview, use subdomain
    try {
      const url = new URL(origin);
      const hostname = url.hostname;
      
      // Extract root domain (e.g., 'leanchain.io' from 'www.leanchain.io' or 'app-xyz.vercel.app')
      let rootDomain = hostname;
      
      // Handle Vercel preview URLs (e.g., app-xyz.vercel.app)
      if (hostname.includes('.vercel.app')) {
        // For Vercel, we'll use path-based routing since subdomains need DNS setup
        return `${origin}/${subdomain}`;
      }
      
      // For custom domains, try to use subdomain
      if (hostname.includes('.')) {
        const parts = hostname.split('.');
        // If it's a subdomain (more than 2 parts), replace the first part
        if (parts.length > 2) {
          parts[0] = subdomain;
          url.hostname = parts.join('.');
        } else {
          // Just domain.tld, add subdomain
          url.hostname = `${subdomain}.${hostname}`;
        }
      } else {
        // No dots, add subdomain
        url.hostname = `${subdomain}.${hostname}`;
      }
      
      return url.toString().replace(/\/$/, '');
    } catch {
      // Fallback to path-based
      return `${origin}/${subdomain}`;
    }
  }
  
  // Server-side: similar logic
  const baseUrl = getBaseUrl();
  
  if (baseUrl.includes('localhost') || baseUrl.includes('127.0.0.1')) {
    return `${baseUrl}/${subdomain}`;
  }
  
  try {
    const url = new URL(baseUrl);
    const hostname = url.hostname;
    
    if (hostname.includes('.vercel.app')) {
      return `${baseUrl}/${subdomain}`;
    }
    
    if (hostname.includes('.')) {
      const parts = hostname.split('.');
      if (parts.length > 2) {
        parts[0] = subdomain;
        url.hostname = parts.join('.');
      } else {
        url.hostname = `${subdomain}.${hostname}`;
      }
    } else {
      url.hostname = `${subdomain}.${hostname}`;
    }
    
    return url.toString().replace(/\/$/, '');
  } catch {
    return `${baseUrl}/${subdomain}`;
  }
}
