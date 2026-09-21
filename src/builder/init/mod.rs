//! Builders for the `Init` block: entity and environment state before the story runs.
//!
//! [`InitActionBuilder`] assembles the whole block; `PrivateActionBuilder` covers
//! per-entity setup and `GlobalActionBuilder` the environment and infrastructure.
//!
//! ```rust
//! use openscenario_rs::builder::init::InitActionBuilder;
//! use openscenario_rs::builder::positions::WorldPositionBuilder;
//!
//! let init = InitActionBuilder::new()
//!     .add_teleport_action("ego",
//!         WorldPositionBuilder::new()
//!             .at_coordinates(0.0, 0.0, 0.0)
//!             .build()
//!             .unwrap()
//!     )
//!     .unwrap()
//!     .add_speed_action("ego", 30.0)
//!     .unwrap()
//!     .build()
//!     .unwrap();
//! ```

pub mod actions;
pub mod private;

pub use actions::InitActionBuilder;
pub use private::{GlobalActionBuilder, PrivateActionBuilder};

/// Convenience functions for common initialization patterns
impl InitActionBuilder {
    /// Create a basic initialization with environment setup
    ///
    /// `environment_name` is required by XSD `Environment` (`Schema/OpenSCENARIO.xsd:1186-1194`,
    /// `@name` `use="required"`, no schema default) — it used to be silently invented as
    /// `"DefaultEnvironment"` here; callers now state it.
    pub fn with_default_environment(environment_name: &str) -> Self {
        Self::new().add_global_environment_action(environment_name)
    }

    // `for_single_vehicle` and `for_multiple_vehicles` are gone with
    // `add_private_action`. Each named an entity and opened a `<Private>` container for
    // it with no action inside, and nothing later filled the container. XSD `Private`
    // (`Schema/OpenSCENARIO.xsd:1773`) requires at least one `PrivateAction`, so every
    // document these produced was invalid at the point they produced it. There is no
    // useful replacement: an entity's private actions have to be known before its
    // container can be opened, which is what `create_private_action` already does.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_builder_basic() {
        let init = InitActionBuilder::new().build().unwrap();

        assert!(init.actions.global_actions.is_empty());
        assert!(init.actions.private_actions.is_empty());
    }

    #[test]
    fn test_init_builder_with_environment() {
        let init = InitActionBuilder::with_default_environment("TestEnvironment")
            .build()
            .unwrap();

        assert_eq!(init.actions.global_actions.len(), 1);
        assert_eq!(
            init.actions.global_actions[0].action_type(),
            "EnvironmentAction"
        );
    }

    // `test_init_builder_single_vehicle` and `test_init_builder_multiple_vehicles`
    // covered the two removed convenience constructors. Both asserted a `<Private>`
    // count while every container they counted was empty, which XSD `Private` forbids.
}
