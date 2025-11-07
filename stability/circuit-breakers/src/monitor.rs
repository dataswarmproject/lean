use crate::types::{BreakerConfig, BreakerError, Result};
use chrono::{DateTime, Utc};
use std::collections::VecDeque;

/// Monitors volatility and detects anomalous market activity
pub struct VolatilityMonitor {
    config: BreakerConfig,
    price_history: VecDeque<PricePoint>,
    volume_history: VecDeque<VolumePoint>,
}

#[derive(Debug, Clone)]
struct PricePoint {
    timestamp: DateTime<Utc>,
    price: u128,
}

#[derive(Debug, Clone)]
struct VolumePoint {
    timestamp: DateTime<Utc>,
    volume: u128,
}

impl VolatilityMonitor {
    pub fn new(config: BreakerConfig) -> Self {
        Self {
            config,
            price_history: VecDeque::new(),
            volume_history: VecDeque::new(),
        }
    }

    pub fn record_price(&mut self, price: u128) {
        self.price_history.push_back(PricePoint {
            timestamp: Utc::now(),
            price,
        });

        // Keep last 24 hours
        let cutoff = Utc::now() - chrono::Duration::hours(24);
        while let Some(oldest) = self.price_history.front() {
            if oldest.timestamp < cutoff {
                self.price_history.pop_front();
            } else {
                break;
            }
        }
    }

    pub fn record_volume(&mut self, volume: u128) {
        self.volume_history.push_back(VolumePoint {
            timestamp: Utc::now(),
            volume,
        });

        // Keep last 24 hours
        let cutoff = Utc::now() - chrono::Duration::hours(24);
        while let Some(oldest) = self.volume_history.front() {
            if oldest.timestamp < cutoff {
                self.volume_history.pop_front();
            } else {
                break;
            }
        }
    }

    pub fn check_price_velocity(&self) -> Result<()> {
        if self.price_history.len() < 2 {
            return Ok(());
        }

        // Check last minute's price change
        let one_minute_ago = Utc::now() - chrono::Duration::minutes(1);
        let recent_prices: Vec<&PricePoint> = self.price_history
            .iter()
            .filter(|p| p.timestamp >= one_minute_ago)
            .collect();

        if recent_prices.len() < 2 {
            return Ok(());
        }

        let oldest = recent_prices.first().unwrap();
        let newest = recent_prices.last().unwrap();

        let change_bps = self.calculate_change_bps(oldest.price, newest.price);
        let duration_seconds = newest.timestamp.signed_duration_since(oldest.timestamp).num_seconds();

        if duration_seconds == 0 {
            return Ok(());
        }

        // Normalize to per-minute rate
        let change_per_minute = (change_bps as i64 * 60) / duration_seconds;

        if change_per_minute.abs() > self.config.max_price_change_per_minute_bps as i64 {
            return Err(BreakerError::RapidPriceChange {
                change_bps: change_per_minute.abs() as u16,
                duration_seconds,
            });
        }

        Ok(())
    }

    pub fn check_volume_anomaly(&self) -> Result<()> {
        if self.volume_history.len() < 10 {
            return Ok(());
        }

        let average_volume = self.average_volume();
        let current_volume = self.volume_history.back().map(|v| v.volume).unwrap_or(0);

        let threshold = (average_volume * self.config.volume_threshold_bps as u128) / 10000;

        if current_volume > threshold {
            return Err(BreakerError::AbnormalVolume {
                current: current_volume,
                average: average_volume,
            });
        }

        Ok(())
    }

    pub fn calculate_volatility(&self) -> u16 {
        if self.price_history.len() < 2 {
            return 0;
        }

        let prices: Vec<u128> = self.price_history.iter().map(|p| p.price).collect();
        let mean = prices.iter().sum::<u128>() / prices.len() as u128;

        let variance_sum: u128 = prices.iter().map(|&p| {
            let diff = if p > mean { p - mean } else { mean - p };
            diff * diff
        }).sum();

        let variance = variance_sum / prices.len() as u128;
        let std_dev = self.isqrt(variance);

        // Return as percentage (basis points)
        if mean > 0 {
            ((std_dev * 10000) / mean) as u16
        } else {
            0
        }
    }

    pub fn price_change_24h(&self) -> Option<i32> {
        if self.price_history.len() < 2 {
            return None;
        }

        let oldest = self.price_history.front()?;
        let newest = self.price_history.back()?;

        Some(self.calculate_change_bps(oldest.price, newest.price))
    }

    pub fn average_volume(&self) -> u128 {
        if self.volume_history.is_empty() {
            return 0;
        }

        let total: u128 = self.volume_history.iter().map(|v| v.volume).sum();
        total / self.volume_history.len() as u128
    }

    fn calculate_change_bps(&self, old_price: u128, new_price: u128) -> i32 {
        if old_price == 0 {
            return 0;
        }

        if new_price > old_price {
            let diff = new_price - old_price;
            ((diff as i128 * 10000) / old_price as i128) as i32
        } else {
            let diff = old_price - new_price;
            -(((diff as i128 * 10000) / old_price as i128) as i32)
        }
    }

    fn isqrt(&self, n: u128) -> u128 {
        if n == 0 {
            return 0;
        }

        let mut x = n;
        let mut y = (x + 1) / 2;

        while y < x {
            x = y;
            y = (x + n / x) / 2;
        }

        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_creation() {
        let config = BreakerConfig::default();
        let monitor = VolatilityMonitor::new(config);
        assert_eq!(monitor.price_history.len(), 0);
    }

    #[test]
    fn test_record_price() {
        let config = BreakerConfig::default();
        let mut monitor = VolatilityMonitor::new(config);

        monitor.record_price(100_000_000);
        assert_eq!(monitor.price_history.len(), 1);
    }

    #[test]
    fn test_record_volume() {
        let config = BreakerConfig::default();
        let mut monitor = VolatilityMonitor::new(config);

        monitor.record_volume(1_000_000_000);
        assert_eq!(monitor.volume_history.len(), 1);
    }

    #[test]
    fn test_calculate_volatility() {
        let config = BreakerConfig::default();
        let mut monitor = VolatilityMonitor::new(config);

        // Add prices with some variance
        monitor.record_price(100_000_000);
        std::thread::sleep(std::time::Duration::from_millis(10));
        monitor.record_price(105_000_000);
        std::thread::sleep(std::time::Duration::from_millis(10));
        monitor.record_price(98_000_000);

        let volatility = monitor.calculate_volatility();
        assert!(volatility > 0);
    }

    #[test]
    fn test_average_volume() {
        let config = BreakerConfig::default();
        let mut monitor = VolatilityMonitor::new(config);

        monitor.record_volume(1_000_000);
        monitor.record_volume(2_000_000);
        monitor.record_volume(3_000_000);

        assert_eq!(monitor.average_volume(), 2_000_000);
    }
}
