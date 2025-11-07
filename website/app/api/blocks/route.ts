import { NextRequest, NextResponse } from 'next/server';
import { rpcClient } from '@/lib/rpc-client';

export async function GET(request: NextRequest) {
  try {
    const searchParams = request.nextUrl.searchParams;
    const count = parseInt(searchParams.get('count') || '10', 10);
    const height = searchParams.get('height');

    if (height) {
      // Get specific block
      const block = await rpcClient.getBlockByNumber(parseInt(height, 10), true);
      return NextResponse.json(block);
    } else {
      // Get recent blocks
      const blocks = await rpcClient.getRecentBlocks(count);
      return NextResponse.json(blocks);
    }
  } catch (error: any) {
    console.error('Error fetching blocks:', error);
    // Return empty array when blockchain node is not available
    return NextResponse.json([]);
  }
}

