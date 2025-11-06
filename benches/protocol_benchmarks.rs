//! Performance Benchmarks for LEAN CHAIN Protocols
//!
//! Benchmarks critical operations to ensure performance targets.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

// Mock structures for benchmarking
#[derive(Clone)]
struct MockTransaction {
    from: [u8; 20],
    to: [u8; 20],
    amount: u128,
    nonce: u64,
}

impl MockTransaction {
    fn new(nonce: u64) -> Self {
        Self {
            from: [1u8; 20],
            to: [2u8; 20],
            amount: 1000,
            nonce,
        }
    }

    fn hash(&self) -> [u8; 32] {
        // Simplified hash
        let mut result = [0u8; 32];
        for i in 0..20 {
            result[i] = self.from[i] ^ self.to[i];
        }
        result
    }
}

struct MockBlock {
    transactions: Vec<MockTransaction>,
    merkle_root: [u8; 32],
}

impl MockBlock {
    fn new(tx_count: usize) -> Self {
        let transactions: Vec<_> = (0..tx_count)
            .map(|i| MockTransaction::new(i as u64))
            .collect();

        Self {
            merkle_root: Self::calculate_merkle_root(&transactions),
            transactions,
        }
    }

    fn calculate_merkle_root(txs: &[MockTransaction]) -> [u8; 32] {
        if txs.is_empty() {
            return [0u8; 32];
        }

        let mut hashes: Vec<[u8; 32]> = txs.iter().map(|tx| tx.hash()).collect();

        while hashes.len() > 1 {
            let mut next_level = Vec::new();
            for chunk in hashes.chunks(2) {
                let combined = if chunk.len() == 2 {
                    combine_hashes(&chunk[0], &chunk[1])
                } else {
                    combine_hashes(&chunk[0], &chunk[0])
                };
                next_level.push(combined);
            }
            hashes = next_level;
        }

        hashes[0]
    }
}

fn combine_hashes(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut result = [0u8; 32];
    for i in 0..32 {
        result[i] = left[i] ^ right[i];
    }
    result
}

// Benchmark transaction validation
fn bench_transaction_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("transaction_validation");
    
    for count in [100, 1000, 10000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(count),
            count,
            |b, &count| {
                let txs: Vec<_> = (0..count)
                    .map(|i| MockTransaction::new(i as u64))
                    .collect();
                
                b.iter(|| {
                    for tx in &txs {
                        black_box(tx.hash());
                    }
                });
            },
        );
    }
    
    group.finish();
}

// Benchmark block creation
fn bench_block_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("block_creation");
    
    for tx_count in [100, 500, 1000, 2000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(tx_count),
            tx_count,
            |b, &tx_count| {
                b.iter(|| {
                    black_box(MockBlock::new(tx_count));
                });
            },
        );
    }
    
    group.finish();
}

// Benchmark merkle tree construction
fn bench_merkle_tree(c: &mut Criterion) {
    let mut group = c.benchmark_group("merkle_tree");
    
    for count in [10, 100, 1000, 10000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(count),
            count,
            |b, &count| {
                let txs: Vec<_> = (0..count)
                    .map(|i| MockTransaction::new(i as u64))
                    .collect();
                
                b.iter(|| {
                    black_box(MockBlock::calculate_merkle_root(&txs));
                });
            },
        );
    }
    
    group.finish();
}

// Benchmark lending pool operations
fn bench_lending_operations(c: &mut Criterion) {
    struct SimpleLendingPool {
        supplied: u128,
        borrowed: u128,
    }

    impl SimpleLendingPool {
        fn new() -> Self {
            Self { supplied: 0, borrowed: 0 }
        }

        fn supply(&mut self, amount: u128) {
            self.supplied += amount;
        }

        fn borrow(&mut self, amount: u128) -> Result<(), ()> {
            if amount > self.supplied - self.borrowed {
                return Err(());
            }
            self.borrowed += amount;
            Ok(())
        }

        fn utilization(&self) -> u128 {
            if self.supplied == 0 {
                return 0;
            }
            (self.borrowed * 10000) / self.supplied
        }
    }

    c.bench_function("lending_supply", |b| {
        b.iter(|| {
            let mut pool = SimpleLendingPool::new();
            for i in 0..1000 {
                pool.supply(black_box(1000 + i));
            }
        });
    });

    c.bench_function("lending_borrow", |b| {
        let mut pool = SimpleLendingPool::new();
        pool.supply(1_000_000);
        
        b.iter(|| {
            let _ = pool.borrow(black_box(1000));
        });
    });

    c.bench_function("lending_utilization", |b| {
        let mut pool = SimpleLendingPool::new();
        pool.supply(1_000_000);
        pool.borrow(500_000).unwrap();
        
        b.iter(|| {
            black_box(pool.utilization());
        });
    });
}

// Benchmark oracle price aggregation
fn bench_oracle_aggregation(c: &mut Criterion) {
    fn median(prices: &[u128]) -> u128 {
        let mut sorted = prices.to_vec();
        sorted.sort();
        sorted[sorted.len() / 2]
    }

    fn mean(prices: &[u128]) -> u128 {
        let sum: u128 = prices.iter().sum();
        sum / prices.len() as u128
    }

    let mut group = c.benchmark_group("oracle_aggregation");
    
    for count in [5, 10, 25, 50].iter() {
        let prices: Vec<u128> = (0..*count).map(|i| 2000 + i as u128).collect();
        
        group.bench_with_input(
            BenchmarkId::new("median", count),
            &prices,
            |b, prices| {
                b.iter(|| black_box(median(prices)));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("mean", count),
            &prices,
            |b, prices| {
                b.iter(|| black_box(mean(prices)));
            },
        );
    }
    
    group.finish();
}

// Benchmark NFT operations
fn bench_nft_operations(c: &mut Criterion) {
    use std::collections::HashMap;

    struct SimpleNFT {
        owners: HashMap<u128, [u8; 20]>,
        balances: HashMap<[u8; 20], u128>,
    }

    impl SimpleNFT {
        fn new() -> Self {
            Self {
                owners: HashMap::new(),
                balances: HashMap::new(),
            }
        }

        fn mint(&mut self, token_id: u128, owner: [u8; 20]) {
            self.owners.insert(token_id, owner);
            *self.balances.entry(owner).or_insert(0) += 1;
        }

        fn owner_of(&self, token_id: u128) -> Option<[u8; 20]> {
            self.owners.get(&token_id).copied()
        }

        fn transfer(&mut self, token_id: u128, from: [u8; 20], to: [u8; 20]) {
            if let Some(owner) = self.owners.get_mut(&token_id) {
                *owner = to;
                if let Some(balance) = self.balances.get_mut(&from) {
                    *balance -= 1;
                }
                *self.balances.entry(to).or_insert(0) += 1;
            }
        }
    }

    c.bench_function("nft_mint_1000", |b| {
        b.iter(|| {
            let mut nft = SimpleNFT::new();
            for i in 0..1000 {
                nft.mint(black_box(i), [1u8; 20]);
            }
        });
    });

    c.bench_function("nft_transfer", |b| {
        let mut nft = SimpleNFT::new();
        for i in 0..1000 {
            nft.mint(i, [1u8; 20]);
        }
        
        b.iter(|| {
            nft.transfer(black_box(500), [1u8; 20], [2u8; 20]);
        });
    });
}

// Benchmark vault operations
fn bench_vault_operations(c: &mut Criterion) {
    struct SimpleVault {
        collateral: u128,
        debt: u128,
    }

    impl SimpleVault {
        fn new(collateral: u128, debt: u128) -> Self {
            Self { collateral, debt }
        }

        fn collateral_ratio(&self, price: u128) -> u128 {
            if self.debt == 0 {
                return u128::MAX;
            }
            (self.collateral * price * 10000) / self.debt
        }

        fn is_safe(&self, price: u128, min_ratio: u128) -> bool {
            self.collateral_ratio(price) >= min_ratio
        }
    }

    c.bench_function("vault_health_check", |b| {
        let vault = SimpleVault::new(1_00000000, 1500_00000000);
        
        b.iter(|| {
            black_box(vault.is_safe(2000_00000000, 15000));
        });
    });

    c.bench_function("vault_health_check_1000", |b| {
        let vaults: Vec<_> = (0..1000)
            .map(|i| SimpleVault::new(1_00000000, 1500_00000000 + i))
            .collect();
        
        b.iter(|| {
            for vault in &vaults {
                black_box(vault.is_safe(2000_00000000, 15000));
            }
        });
    });
}

criterion_group!(
    benches,
    bench_transaction_validation,
    bench_block_creation,
    bench_merkle_tree,
    bench_lending_operations,
    bench_oracle_aggregation,
    bench_nft_operations,
    bench_vault_operations,
);

criterion_main!(benches);
