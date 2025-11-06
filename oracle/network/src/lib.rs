//! LEAN CHAIN Oracle Network
//!
//! Decentralized price feed oracle system with aggregation and validation.

mod feed;
mod aggregator;
mod validator;

pub use feed::{PriceFeed, PriceData};
pub use aggregator::{PriceAggregator, AggregationMethod};
pub use validator::{OracleValidator, ValidatorSet};

use thiserror::Error;

/// Oracle errors
#[derive(Debug, Error)]
pub enum OracleError {
    #[error("Price feed not found: {0}")]
    FeedNotFound(String),
    
    #[error("Stale price data: age {age}s exceeds threshold {threshold}s")]
    StalePrice { age: i64, threshold: i64 },
    
    #[error("Insufficient oracle responses: got {got}, need {required}")]
    InsufficientResponses { got: usize, required: usize },
    
    #[error("Price deviation too high: {0}%")]
    PriceDeviationTooHigh(u128),
    
    #[error("Invalid oracle signature")]
    InvalidSignature,
    
    #[error("Oracle not authorized: {0}")]
    UnauthorizedOracle(String),
    
    #[error("Arithmetic overflow")]
    ArithmeticOverflow,
}

pub type Result<T> = std::result::Result<T, OracleError>;
