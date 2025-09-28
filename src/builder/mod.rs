//! Builder module root for programmatic scenario construction

// Import from new error module
mod error;
pub use error::{BuilderError, BuilderResult};

// Existing modules
pub mod positions;
pub mod scenario; 
pub mod fluent;

// New modules (Sprint 1+)
pub mod entities;
pub mod actions;    // Sprint 2

// Export main builders
pub use scenario::ScenarioBuilder;
pub use entities::VehicleBuilder;
pub use actions::{SpeedActionBuilder, TeleportActionBuilder};
