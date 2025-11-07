use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use thiserror::Error;

/// Circuit breaker levels based on price deviation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreakerLevel {
    /// Normal operations (deviation < 10%)
    Green,
    /// Enhanced monitoring (10-15% deviation)
    Yellow,
    /// Limited operations (15-20% deviation)
    Orange,
    /// Emergency mode (20-30% deviation)
    Red,
    /// Complete halt (>30% deviation)
    Black,
}

impl BreakerLevel {
    pub fn from_deviation_bps(deviation_bps: u16) -> Self {
        match deviation_bps {
            0..=1000 => BreakerLevel::Green,
            1001..=1500 => BreakerLevel::Yellow,
            1501..=2000 => BreakerLevel::Orange,
            2001..=3000 => BreakerLevel::Red,
            _ => BreakerLevel::Black,
        }
    }

    pub fn pause_duration(&self) -> Option<Duration> {
        match self {
            BreakerLevel::Green | BreakerLevel::Yellow => None,
            BreakerLevel::Orange => Some(Duration::hours(1)),
            BreakerLevel::Red => Some(Duration::hours(6)),
            BreakerLevel::Black => Some(Duration::hours(24)),
        }
    }

    pub fn allows_trading(&self) -> bool {
        matches!(self, BreakerLevel::Green | BreakerLevel::Yellow | BreakerLevel::Orange)
    }
}

/// Status of the circuit breaker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakerStatus {
    pub level: BreakerLevel,
    pub triggered_at: Option<DateTime<Utc>>,
    pub pause_until: Option<DateTime<Utc>>,
    pub current_deviation_bps: u16,
    pub volume_24h: u128,
    pub price_change_24h_bps: i32,
}

impl BreakerStatus {
    pub fn is_paused(&self) -> bool {
        if let Some(pause_until) = self.pause_until {
            Utc::now() < pause_until
        } else {
            false
        }
    }

    pub fn is_operational(&self) -> bool {
        self.level.allows_trading() && !self.is_paused()
    }

    pub fn time_until_resume(&self) -> Option<Duration> {
        self.pause_until.map(|until| until.signed_duration_since(Utc::now()))
    }
}

/// Circuit breaker configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakerConfig {
    /// Enable circuit breakers
    pub enabled: bool,
    /// Minimum time between breaker triggers (hours)
    pub cooldown_hours: i64,
    /// Volume threshold for suspicious activity (deviation from 24h average)
    pub volume_threshold_bps: u16,
    /// Price change rate limit (basis points per minute)
    pub max_price_change_per_minute_bps: u16,
}

impl Default for BreakerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cooldown_hours: 1,
            volume_threshold_bps: 20000, // 200% of average (2x spike)
            max_price_change_per_minute_bps: 100, // 1% per minute max
        }
    }
}

/// Errors related to circuit breaker operations
#[derive(Error, Debug)]
pub enum BreakerError {
    #[error("Trading paused until {pause_until}")]
    TradingPaused {
        pause_until: DateTime<Utc>,
    },

    #[error("Circuit breaker triggered at level {level:?}")]
    BreakerTriggered {
        level: BreakerLevel,
    },

    #[error("Operation not allowed in {level:?} mode")]
    OperationNotAllowed {
        level: BreakerLevel,
    },

    #[error("Cooldown period active: {remaining_seconds}s remaining")]
    CooldownActive {
        remaining_seconds: i64,
    },

    #[error("Price change too rapid: {change_bps}bps in {duration_seconds}s")]
    RapidPriceChange {
        change_bps: u16,
        duration_seconds: i64,
    },

    #[error("Abnormal volume detected: {current} vs average {average}")]
    AbnormalVolume {
        current: u128,
        average: u128,
    },
}

pub type Result<T> = std::result::Result<T, BreakerError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breaker_level_from_deviation() {
        assert_eq!(BreakerLevel::from_deviation_bps(500), BreakerLevel::Green);
        assert_eq!(BreakerLevel::from_deviation_bps(1200), BreakerLevel::Yellow);
        assert_eq!(BreakerLevel::from_deviation_bps(1800), BreakerLevel::Orange);
        assert_eq!(BreakerLevel::from_deviation_bps(2500), BreakerLevel::Red);
        assert_eq!(BreakerLevel::from_deviation_bps(4000), BreakerLevel::Black);
    }

    #[test]
    fn test_pause_duration() {
        assert_eq!(BreakerLevel::Green.pause_duration(), None);
        assert_eq!(BreakerLevel::Orange.pause_duration(), Some(Duration::hours(1)));
        assert_eq!(BreakerLevel::Red.pause_duration(), Some(Duration::hours(6)));
        assert_eq!(BreakerLevel::Black.pause_duration(), Some(Duration::hours(24)));
    }

    #[test]
    fn test_allows_trading() {
        assert!(BreakerLevel::Green.allows_trading());
        assert!(BreakerLevel::Yellow.allows_trading());
        assert!(BreakerLevel::Orange.allows_trading());
        assert!(!BreakerLevel::Red.allows_trading());
        assert!(!BreakerLevel::Black.allows_trading());
    }
}
