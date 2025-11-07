import { NextRequest, NextResponse } from 'next/server';
import { rpcClient } from '@/lib/rpc-client';

export async function GET(request: NextRequest) {
  try {
    const searchParams = request.nextUrl.searchParams;
    const address = searchParams.get('address');

    if (!address) {
      return NextResponse.json(
        { error: 'Address parameter is required' },
        { status: 400 }
      );
    }

    const account = await rpcClient.getAccount(address);
    return NextResponse.json(account);
  } catch (error: any) {
    console.error('Error fetching account:', error);
    // Return empty account data when blockchain node is not available
    return NextResponse.json({
      address: address || '',
      balance: '0',
      nonce: 0,
    });
  }
}

