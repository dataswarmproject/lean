import { NextResponse } from 'next/server';
import { rpcClient } from '@/lib/rpc-client';

export async function GET() {
  try {
    const [networkStats, latestBlock] = await Promise.all([
      rpcClient.getNetworkStats(),
      rpcClient.getLatestBlock(),
    ]);

    return NextResponse.json({
      ...networkStats,
      current_height: latestBlock.height,
      latest_block_hash: latestBlock.hash,
      latest_block_time: latestBlock.timestamp,
    });
  } catch (error: any) {
    console.error('Error fetching network stats:', error);
    // Return default stats when blockchain node is not available
    return NextResponse.json({
      current_height: 0,
      total_transactions: 0,
      total_accounts: 0,
      total_supply: '0',
      avg_block_time: 0,
      tps: 0,
      latest_block_hash: '',
      latest_block_time: 0,
    });
  }
}

