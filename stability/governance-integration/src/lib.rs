/// Governance Integration for Stability System
///
/// Integrates governance controls for stability parameters,
/// emergency actions, and transparent on-chain logging.

pub mod types;
pub mod parameters;
pub mod emergency;
pub mod coordinator;

pub use types::{GovernanceAction, ActionStatus, GovernanceError};
pub use parameters::ParameterController;
pub use emergency::EmergencyMultisig;
pub use coordinator::StabilityGovernance;
