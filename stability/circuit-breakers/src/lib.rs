/// Circuit Breaker System
///
/// Implements multi-level circuit breakers to protect against extreme
/// price movements and market volatility.

pub mod types;
pub mod breaker;
pub mod monitor;

pub use types::{BreakerLevel, BreakerStatus, BreakerError};
pub use breaker::CircuitBreaker;
pub use monitor::VolatilityMonitor;
