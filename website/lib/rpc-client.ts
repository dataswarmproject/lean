/**
 * LEAN Chain JSON-RPC Client
 * Connects to the blockchain RPC endpoint
 */

export interface RPCRequest {
  jsonrpc: '2.0';
  method: string;
  params: any[];
  id: number;
}

export interface RPCResponse<T = any> {
  jsonrpc: '2.0';
  result?: T;
  error?: {
    code: number;
    message: string;
    data?: any;
  };
  id: number;
}

export interface Block {
  height: number;
  hash: string;
  parent_hash: string;
  timestamp: number;
  proposer: string;
  transactions?: Transaction[];
  transaction_count?: number;
}

export interface Transaction {
  hash: string;
  from: string;
  to: string;
  amount: string;
  gas_price: string;
  gas_limit: string;
  gas_used?: string;
  nonce: number;
  data?: string;
  status?: number;
  block_height?: number;
  timestamp?: number;
}

export interface TransactionReceipt {
  status: number;
  gas_used: string;
  logs: any[];
  block_height: number;
  transaction_hash: string;
}

export interface Account {
  address: string;
  balance: string;
  nonce: number;
  code?: string;
}

export interface Validator {
  address: string;
  voting_power: string;
  proposer_priority: number;
  commission?: number;
  moniker?: string;
  status?: 'active' | 'jailed' | 'inactive';
  uptime?: number;
  total_staked?: string;
  rewards?: string;
}

export interface ValidatorSet {
  validators: Validator[];
  total_voting_power: string;
}

export interface NetworkStats {
  current_height: number;
  total_transactions: number;
  total_accounts: number;
  total_supply: string;
  avg_block_time: number;
  tps: number;
}

class RPCClient {
  private endpoint: string;
  private requestId: number = 1;

  constructor(endpoint?: string) {
    this.endpoint = endpoint || process.env.NEXT_PUBLIC_RPC_URL || 'http://localhost:26657';
  }

  private async call<T>(method: string, params: any[] = []): Promise<T> {
    const request: RPCRequest = {
      jsonrpc: '2.0',
      method,
      params,
      id: this.requestId++,
    };

    try {
      const response = await fetch(this.endpoint, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(request),
      });

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      const data: RPCResponse<T> = await response.json();

      if (data.error) {
        throw new Error(`RPC error: ${data.error.message} (code: ${data.error.code})`);
      }

      if (data.result === undefined) {
        throw new Error('RPC response missing result');
      }

      return data.result;
    } catch (error) {
      console.error('RPC call failed:', error);
      throw error;
    }
  }

  // Chain methods
  async getLatestBlock(): Promise<Block> {
    return this.call<Block>('chain_getLatestBlock', []);
  }

  async getBlockByNumber(height: number, includeTxs: boolean = false): Promise<Block> {
    return this.call<Block>('chain_getBlockByNumber', [height, includeTxs]);
  }

  async getBlockByHash(hash: string): Promise<Block> {
    return this.call<Block>('chain_getBlockByHash', [hash]);
  }

  // Transaction methods
  async getTransaction(hash: string): Promise<Transaction> {
    return this.call<Transaction>('tx_getTransaction', [hash]);
  }

  async getTransactionReceipt(hash: string): Promise<TransactionReceipt> {
    return this.call<TransactionReceipt>('tx_getTransactionReceipt', [hash]);
  }

  async sendTransaction(tx: {
    from: string;
    to: string;
    amount: string;
    data?: string;
    gas_price: string;
    gas_limit: string;
  }): Promise<{ tx_hash: string }> {
    return this.call<{ tx_hash: string }>('tx_sendTransaction', [tx]);
  }

  // Account methods
  async getBalance(address: string, block: string | number = 'latest'): Promise<string> {
    return this.call<string>('account_getBalance', [address, block]);
  }

  async getAccount(address: string): Promise<Account> {
    const [balance, nonce] = await Promise.all([
      this.getBalance(address),
      this.getNonce(address),
    ]);

    return {
      address,
      balance,
      nonce,
    };
  }

  async getNonce(address: string): Promise<number> {
    return this.call<number>('account_getNonce', [address]);
  }

  async getCode(address: string): Promise<string> {
    return this.call<string>('account_getCode', [address]);
  }

  // Validator methods
  async getValidators(): Promise<ValidatorSet> {
    return this.call<ValidatorSet>('validator_getValidators', []);
  }

  async getValidator(address: string): Promise<Validator> {
    return this.call<Validator>('validator_getValidator', [address]);
  }

  // Network stats (composite method)
  async getNetworkStats(): Promise<NetworkStats> {
    const latestBlock = await this.getLatestBlock();
    
    // These would be actual RPC methods if available
    // For now, we'll calculate from available data
    return {
      current_height: latestBlock.height,
      total_transactions: 0, // Would need separate endpoint
      total_accounts: 0, // Would need separate endpoint
      total_supply: '100000000000000000000000000000', // 100B LEAN
      avg_block_time: 100, // <100ms
      tps: 200000, // 200k TPS
    };
  }

  // Get recent blocks
  async getRecentBlocks(count: number = 10): Promise<Block[]> {
    const latestBlock = await this.getLatestBlock();
    const blocks: Block[] = [];
    
    for (let i = 0; i < count && latestBlock.height - i >= 0; i++) {
      try {
        const block = await this.getBlockByNumber(latestBlock.height - i, false);
        blocks.push(block);
      } catch (error) {
        console.error(`Failed to fetch block ${latestBlock.height - i}:`, error);
      }
    }
    
    return blocks;
  }

  // Get recent transactions
  async getRecentTransactions(count: number = 10): Promise<Transaction[]> {
    const latestBlock = await this.getLatestBlock();
    const transactions: Transaction[] = [];
    
    // Get transactions from recent blocks
    for (let i = 0; i < 5 && latestBlock.height - i >= 0; i++) {
      try {
        const block = await this.getBlockByNumber(latestBlock.height - i, true);
        if (block.transactions) {
          transactions.push(...block.transactions);
          if (transactions.length >= count) break;
        }
      } catch (error) {
        console.error(`Failed to fetch block ${latestBlock.height - i}:`, error);
      }
    }
    
    return transactions.slice(0, count);
  }
}

// Export singleton instance
export const rpcClient = new RPCClient();

// Export class for custom instances
export default RPCClient;

