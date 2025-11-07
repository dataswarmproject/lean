import { NextRequest, NextResponse } from 'next/server';
import { rpcClient } from '@/lib/rpc-client';

export async function GET(request: NextRequest) {
  try {
    const searchParams = request.nextUrl.searchParams;
    const address = searchParams.get('address');

    if (address) {
      // Get specific validator
      const validator = await rpcClient.getValidator(address);
      return NextResponse.json(validator);
    } else {
      // Get all validators
      const validatorSet = await rpcClient.getValidators();
      
      // Enhance validator data with additional info
      const validators = await Promise.all(
        validatorSet.validators.map(async (validator) => {
          try {
            // Get balance as stake
            const balance = await rpcClient.getBalance(validator.address).catch(() => '0');
            
            return {
              ...validator,
              total_staked: balance,
              // These would come from validator info if available
              commission: 5, // Default, should come from validator_getValidator
              status: 'active' as const,
              uptime: 99.5, // Would need historical data
              rewards: '0', // Would need to calculate
            };
          } catch (error) {
            return {
              ...validator,
              total_staked: validator.voting_power,
              commission: 5,
              status: 'active' as const,
              uptime: 99.5,
              rewards: '0',
            };
          }
        })
      );

      return NextResponse.json({
        validators,
        total_voting_power: validatorSet.total_voting_power,
      });
    }
  } catch (error: any) {
    console.error('Error fetching validators:', error);
    // Return empty data when blockchain node is not available
    return NextResponse.json({
      validators: [],
      total_voting_power: '0',
    });
  }
}

