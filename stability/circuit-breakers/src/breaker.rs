use crate::types::{BreakerLevel, BreakerStatus, BreakerConfig, BreakerError, Result};
use chrono::{DateTime, Utc, Duration};

/// Main circuit breaker implementation
pub struct CircuitBreaker {
    config: BreakerConfig,
    status: BreakerStatus,
    last_trigger: Option<DateTime<Utc>>,
    target_price: u128,
}

impl CircuitBreaker {
    pub fn new(config: BreakerConfig, target_price: u128) -> Self {
        Self {
            config,
            status: BreakerStatus {
                level: BreakerLevel::Green,
                triggered_at: None,
                pause_until: None,
                current_deviation_bps: 0,
                volume_24h: 0,
                price_change_24h_bps: 0,
            },
            last_trigger: None,
            target_price,
        }
    }

    pub fn check_price(&mut self, current_price: u128) -> Result<()> {
        if !self.config.enabled {
            return Ok(());
        }

        // Check if currently paused
        if self.status.is_paused() {
            return Err(BreakerError::TradingPaused {
                pause_until: self.status.pause_until.unwrap(),
            });
        }

        // Calculate deviation
        let deviation_bps = self.calculate_deviation_bps(current_price)?;
        let level = BreakerLevel::from_deviation_bps(deviation_bps);

        self.status.current_deviation_bps = deviation_bps;

        // Check if level changed and requires action
        if level != self.status.level {
            self.handle_level_change(level)?;
        }

        Ok(())
    }

    pub fn update_volume(&mut self, volume_24h: u128) {
        self.status.volume_24h = volume_24h;
    }

    pub fn update_price_change(&mut self, change_24h_bps: i32) {
        self.status.price_change_24h_bps = change_24h_bps;
    }

    pub fn get_status(&self) -> &BreakerStatus {
        &self.status
    }

    pub fn can_execute_operation(&self, operation: OperationType) -> Result<()> {
        if self.status.is_paused() {
            return Err(BreakerError::TradingPaused {
                pause_until: self.status.pause_until.unwrap(),
            });
        }

        match (self.status.level, operation) {
            (BreakerLevel::Green, _) => Ok(()),
            (BreakerLevel::Yellow, _) => Ok(()),
            (BreakerLevel::Orange, OperationType::EmergencyOnly) => Ok(()),
            (BreakerLevel::Orange, _) => Err(BreakerError::OperationNotAllowed {
                level: BreakerLevel::Orange,
            }),
            (BreakerLevel::Red | BreakerLevel::Black, OperationType::EmergencyOnly) => Ok(()),
            (level, _) => Err(BreakerError::OperationNotAllowed { level }),
        }
    }

    pub fn manually_trigger(&mut self, level: BreakerLevel) -> Result<()> {
        self.handle_level_change(level)
    }

    pub fn manually_resume(&mut self) {
        self.status.level = BreakerLevel::Green;
        self.status.pause_until = None;
        self.status.triggered_at = None;
    }

    fn calculate_deviation_bps(&self, current_price: u128) -> Result<u16> {
        let diff = if current_price > self.target_price {
            current_price - self.target_price
        } else {
            self.target_price - current_price
        };

        Ok(((diff * 10000) / self.target_price) as u16)
    }

    fn handle_level_change(&mut self, new_level: BreakerLevel) -> Result<()> {
        // Check cooldown
        if let Some(last) = self.last_trigger {
            let elapsed = Utc::now().signed_duration_since(last);
            if elapsed.num_hours() < self.config.cooldown_hours {
                return Err(BreakerError::CooldownActive {
                    remaining_seconds: (self.config.cooldown_hours * 3600) - elapsed.num_seconds(),
                });
            }
        }

        let old_level = self.status.level;
        self.status.level = new_level;
        self.status.triggered_at = Some(Utc::now());
        self.last_trigger = Some(Utc::now());

        // Set pause if required
        if let Some(duration) = new_level.pause_duration() {
            self.status.pause_until = Some(Utc::now() + duration);
        } else {
            self.status.pause_until = None;
        }

        // Only error if moving to more restrictive level
        if new_level as u8 > old_level as u8 {
            Err(BreakerError::BreakerTriggered { level: new_level })
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperationType {
    Normal,
    EmergencyOnly,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_breaker_creation() {
        let config = BreakerConfig::default();
        let breaker = CircuitBreaker::new(config, 100_000_000);
        assert_eq!(breaker.status.level, BreakerLevel::Green);
    }

    #[test]
    fn test_price_check_normal() {
        let config = BreakerConfig::default();
        let mut breaker = CircuitBreaker::new(config, 100_000_000);

        // 5% deviation - should stay green
        let result = breaker.check_price(105_000_000);
        assert!(result.is_ok());
        assert_eq!(breaker.status.level, BreakerLevel::Green);
    }

    #[test]
    fn test_price_check_triggers_yellow() {
        let config = BreakerConfig::default();
        let mut breaker = CircuitBreaker::new(config, 100_000_000);

        // 12% deviation - should trigger yellow
        let result = breaker.check_price(112_000_000);
        assert!(result.is_err());
        assert_eq!(breaker.status.level, BreakerLevel::Yellow);
    }

    #[test]
    fn test_price_check_triggers_red() {
        let config = BreakerConfig::default();
        let mut breaker = CircuitBreaker::new(config, 100_000_000);

        // 25% deviation - should trigger red
        let result = breaker.check_price(125_000_000);
        assert!(result.is_err());
        assert_eq!(breaker.status.level, BreakerLevel::Red);
        assert!(breaker.status.is_paused());
    }

    #[test]
    fn test_can_execute_operation() {
        let config = BreakerConfig::default();
        let mut breaker = CircuitBreaker::new(config, 100_000_000);

        // Green level - all operations allowed
        assert!(breaker.can_execute_operation(OperationType::Normal).is_ok());

        // Manually trigger red
        breaker.manually_trigger(BreakerLevel::Red).ok();

        // Red level - only emergency operations
        assert!(breaker.can_execute_operation(OperationType::Normal).is_err());
        assert!(breaker.can_execute_operation(OperationType::EmergencyOnly).is_ok());
    }

    #[test]
    fn test_manual_resume() {
        let config = BreakerConfig::default();
        let mut breaker = CircuitBreaker::new(config, 100_000_000);

        breaker.manually_trigger(BreakerLevel::Red).ok();
        assert!(breaker.status.is_paused());

        breaker.manually_resume();
        assert!(!breaker.status.is_paused());
        assert_eq!(breaker.status.level, BreakerLevel::Green);
    }
}
