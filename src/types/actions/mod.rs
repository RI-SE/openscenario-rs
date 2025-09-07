//! Action type module organizing all OpenSCENARIO action definitions
//!
//! This file contains:
//! - Re-exports from action submodules (movement, control, appearance, traffic)
//! - Base Action trait defining common action behaviors
//! - Action validation logic and constraint checking
//! - Action execution context and state management
//! - Cross-cutting action concerns (timing, priority, conditions)
//!
//! Contributes to project by:
//! - Organizing 48+ action types into logical, manageable categories
//! - Providing consistent interface for all action types
//! - Enabling polymorphic action handling and execution
//! - Supporting action composition and complex scenario building
//! - Facilitating action validation and constraint enforcement

pub mod movement;   // Movement actions (SpeedAction, TeleportAction, etc.)
// pub mod control;    // Controller actions (skip for MVP)
// pub mod appearance; // Appearance actions (skip for MVP)
// pub mod traffic;    // Traffic actions (skip for MVP)

pub use movement::{SpeedAction, TeleportAction};

use serde::{Deserialize, Serialize};

// Define base Action enum for polymorphic handling
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Action { 
    Speed(SpeedAction), 
    Teleport(TeleportAction) 
}

impl Default for Action {
    fn default() -> Self {
        Action::Speed(SpeedAction::default())
    }
}

// Define Action wrapper with common attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionWrapper { 
    #[serde(rename = "@name")] 
    pub name: String, 
    #[serde(flatten)] 
    pub action: Action 
}

use crate::types::ValidationContext;

// Add action validation trait (Week 6) - KEEP AS FUTURE WORK
pub trait ValidateAction { 
    fn validate(&self, ctx: &ValidationContext) -> crate::error::Result<()>; 
}