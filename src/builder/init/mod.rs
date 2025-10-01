//! Init action builders for scenario initialization
//!
//! This module provides builders for creating OpenSCENARIO initialization actions
//! that set up entities and environment before scenario execution begins.
//!
//! # Architecture
//!
//! - **InitActionBuilder**: Main builder for complete Init structure
//! - **PrivateActionBuilder**: Builder for entity-specific initialization
//! - **GlobalActionBuilder**: Builder for environment and infrastructure setup
//!
//! # Usage
//!
//! ```rust
//! use openscenario_rs::builder::init::InitActionBuilder;
//! use openscenario_rs::builder::positions::WorldPositionBuilder;
//!
//! let init = InitActionBuilder::new()
//!     .add_teleport_action("ego", 
//!         WorldPositionBuilder::new()
//!             .at_coordinates(0.0, 0.0, 0.0)
//!             .build()?
//!     )
//!     .add_speed_action("ego", 30.0)
//!     .build()?;
//! ```

pub mod actions;
pub mod private;

pub use actions::InitActionBuilder;
pub use private::{PrivateActionBuilder, GlobalActionBuilder};


/// Convenience functions for common initialization patterns
impl InitActionBuilder {
    /// Create a basic initialization with environment setup
    pub fn with_default_environment() -> Self {
        Self::new().add_global_environment_action()
    }
    
    /// Create initialization for a single vehicle scenario
    pub fn for_single_vehicle(entity_ref: &str) -> Self {
        Self::new()
            .add_global_environment_action()
            .add_private_action(entity_ref)
    }
    
    /// Create initialization for multi-vehicle scenario
    pub fn for_multiple_vehicles(entity_refs: &[&str]) -> Self {
        let mut builder = Self::new().add_global_environment_action();
        
        for entity_ref in entity_refs {
            builder = builder.add_private_action(entity_ref);
        }
        
        builder
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::positions::Position;
    use crate::builder::positions::WorldPositionBuilder;

    #[test]
    fn test_init_builder_basic() {
        let init = InitActionBuilder::new()
            .build()
            .unwrap();
            
        assert!(init.actions.global_actions.is_empty());
        assert!(init.actions.private_actions.is_empty());
    }

    #[test]
    fn test_init_builder_with_environment() {
        let init = InitActionBuilder::with_default_environment()
            .build()
            .unwrap();
            
        assert_eq!(init.actions.global_actions.len(), 1);
        assert!(init.actions.global_actions[0].environment_action.is_some());
    }

    #[test]
    fn test_init_builder_single_vehicle() {
        let init = InitActionBuilder::for_single_vehicle("ego")
            .build()
            .unwrap();
            
        assert_eq!(init.actions.global_actions.len(), 1);
        assert_eq!(init.actions.private_actions.len(), 1);
        assert_eq!(
            init.actions.private_actions[0].entity_ref.as_literal().unwrap(),
            "ego"
        );
    }

    #[test]
    fn test_init_builder_multiple_vehicles() {
        let vehicles = ["ego", "target", "obstacle"];
        let init = InitActionBuilder::for_multiple_vehicles(&vehicles)
            .build()
            .unwrap();
            
        assert_eq!(init.actions.global_actions.len(), 1);
        assert_eq!(init.actions.private_actions.len(), 3);
        
        for (i, vehicle) in vehicles.iter().enumerate() {
            assert_eq!(
                init.actions.private_actions[i].entity_ref.as_literal().unwrap(),
                *vehicle
            );
        }
    }
}