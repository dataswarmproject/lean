/// Supply Adjustment System
///
/// Manages token supply expansion and contraction to maintain price stability.
/// Implements daily and weekly adjustment limits with various mechanisms.

pub mod types;
pub mod expansion;
pub mod contraction;
pub mod controller;
pub mod limits;

pub use types::{AdjustmentType, SupplyChange, AdjustmentError};
pub use expansion::ExpansionMechanism;
pub use contraction::ContractionMechanism;
pub use controller::SupplyController;
pub use limits::AdjustmentLimits;
