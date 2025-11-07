import { NextRequest, NextResponse } from 'next/server';
import { rpcClient } from '@/lib/rpc-client';

export async function GET(request: NextRequest) {
  try {
    const searchParams = request.nextUrl.searchParams;
    const hash = searchParams.get('hash');
    const count = parseInt(searchParams.get('count') || '10', 10);

    if (hash) {
      // Get specific transaction
      const tx = await rpcClient.getTransaction(hash);
      const receipt = await rpcClient.getTransactionReceipt(hash).catch(() => null);
      return NextResponse.json({ ...tx, receipt });
    } else {
      // Get recent transactions
      const transactions = await rpcClient.getRecentTransactions(count);
      return NextResponse.json(transactions);
    }
  } catch (error: any) {
    console.error('Error fetching transactions:', error);
    // Return empty array when blockchain node is not available
    return NextResponse.json([]);
  }
}

