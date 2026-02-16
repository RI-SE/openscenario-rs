//! Routing action builders for assign route and follow route actions
//!
//! This module provides builders for actions that control entity routing and navigation,
//! including assigning routes and following routes using direct or catalog-based references.
//!
//! # Available Builders
//!
//! - [`AssignRouteActionBuilder`] - Assign a route to an entity (direct or catalog)
//! - [`FollowRouteActionBuilder`] - Follow an assigned route
//!
//! # Usage Examples
//!
//! ```rust
//! use openscenario_rs::builder::actions::routing::{
//!     AssignRouteActionBuilder, FollowRouteActionBuilder
//! };
//! use openscenario_rs::types::routing::{Route, RouteRef};
//!
//! // Create assign route action with direct route
//! let route = Route::new("my_route", false);
//! let assign_action = AssignRouteActionBuilder::new()
//!     .for_entity("ego_vehicle")
//!     .with_direct_route(route);
//!
//! // Create assign route action with catalog reference
//! let assign_catalog_action = AssignRouteActionBuilder::new()
//!     .for_entity("ego_vehicle")
//!     .with_catalog_route("RouteCatalog", "StandardRoute");
//!
//! // Create follow route action
//! let route_ref = RouteRef::direct(Route::new("follow_route", false));
//! let follow_action = FollowRouteActionBuilder::new()
//!     .for_entity("ego_vehicle")
//!     .with_route_ref(route_ref);
//! ```

use crate::builder::actions::base::{ActionBuilder, ManeuverAction};
use crate::builder::{BuilderError, BuilderResult};
use crate::types::{
    actions::movement::{AssignRouteAction, FollowRouteAction, RoutingAction},
    actions::wrappers::PrivateAction,
    routing::{Route, RouteRef},
};

/// Builder for assign route actions
#[derive(Debug, Default)]
pub struct AssignRouteActionBuilder {
    entity_ref: Option<String>,
    route_ref: Option<RouteRef>,
}

impl AssignRouteActionBuilder {
    /// Create new assign route action builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set target entity for this action
    pub fn for_entity(mut self, entity_ref: &str) -> Self {
        self.entity_ref = Some(entity_ref.to_string());
        self
    }

    /// Set route using direct route definition
    pub fn with_direct_route(mut self, route: Route) -> Self {
        self.route_ref = Some(RouteRef::direct(route));
        self
    }

    /// Set route using catalog reference
    pub fn with_catalog_route(
        mut self,
        catalog_name: impl Into<String>,
        entry_name: impl Into<String>,
    ) -> Self {
        self.route_ref = Some(RouteRef::catalog(catalog_name, entry_name));
        self
    }

    /// Set route reference (direct or catalog)
    pub fn with_route_ref(mut self, route_ref: RouteRef) -> Self {
        self.route_ref = Some(route_ref);
        self
    }
}

impl ActionBuilder for AssignRouteActionBuilder {
    fn build_action(self) -> BuilderResult<PrivateAction> {
        self.validate()?;

        let action = AssignRouteAction {
            route: self.route_ref.unwrap(),
        };

        Ok(PrivateAction::RoutingAction(RoutingAction {
            assign_route_action: Some(action),
            follow_trajectory_action: None,
            follow_route_action: None,
        }))
    }

    fn validate(&self) -> BuilderResult<()> {
        if self.route_ref.is_none() {
            return Err(BuilderError::validation_error(
                "Route reference is required for assign route action",
            ));
        }
        Ok(())
    }
}

impl ManeuverAction for AssignRouteActionBuilder {
    fn entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

/// Builder for follow route actions
#[derive(Debug, Default)]
pub struct FollowRouteActionBuilder {
    entity_ref: Option<String>,
    route_ref: Option<RouteRef>,
}

impl FollowRouteActionBuilder {
    /// Create new follow route action builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set target entity for this action
    pub fn for_entity(mut self, entity_ref: &str) -> Self {
        self.entity_ref = Some(entity_ref.to_string());
        self
    }

    /// Set route using direct route definition
    pub fn with_direct_route(mut self, route: Route) -> Self {
        self.route_ref = Some(RouteRef::direct(route));
        self
    }

    /// Set route using catalog reference
    pub fn with_catalog_route(
        mut self,
        catalog_name: impl Into<String>,
        entry_name: impl Into<String>,
    ) -> Self {
        self.route_ref = Some(RouteRef::catalog(catalog_name, entry_name));
        self
    }

    /// Set route reference (direct or catalog)
    pub fn with_route_ref(mut self, route_ref: RouteRef) -> Self {
        self.route_ref = Some(route_ref);
        self
    }
}

impl ActionBuilder for FollowRouteActionBuilder {
    fn build_action(self) -> BuilderResult<PrivateAction> {
        self.validate()?;

        let action = FollowRouteAction {
            route_ref: self.route_ref.unwrap(),
        };

        Ok(PrivateAction::RoutingAction(RoutingAction {
            assign_route_action: None,
            follow_trajectory_action: None,
            follow_route_action: Some(action),
        }))
    }

    fn validate(&self) -> BuilderResult<()> {
        if self.route_ref.is_none() {
            return Err(BuilderError::validation_error(
                "Route reference is required for follow route action",
            ));
        }
        Ok(())
    }
}

impl ManeuverAction for FollowRouteActionBuilder {
    fn entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::enums::RouteStrategy;
    use crate::types::routing::Waypoint;

    #[test]
    fn test_assign_route_direct() {
        let route = Route::new("test_route", false).add_waypoint(Waypoint::world_position(
            0.0,
            0.0,
            0.0,
            RouteStrategy::Shortest,
        ));

        let builder = AssignRouteActionBuilder::new()
            .for_entity("ego")
            .with_direct_route(route)
            .build_action()
            .unwrap();

        // Verify structure
        match builder {
            PrivateAction::RoutingAction(ref action) => {
                assert!(action.assign_route_action.is_some());
                assert!(action.follow_route_action.is_none());
                assert!(action.follow_trajectory_action.is_none());

                let assign_action = action.assign_route_action.as_ref().unwrap();
                assert!(matches!(assign_action.route, RouteRef::Direct(_)));
            }
            _ => panic!("Expected RoutingAction"),
        }
    }

    #[test]
    fn test_assign_route_catalog() {
        let builder = AssignRouteActionBuilder::new()
            .for_entity("ego")
            .with_catalog_route("RouteCatalog", "MyRoute")
            .build_action()
            .unwrap();

        match builder {
            PrivateAction::RoutingAction(ref action) => {
                let assign_action = action.assign_route_action.as_ref().unwrap();
                assert!(matches!(assign_action.route, RouteRef::Catalog(_)));
            }
            _ => panic!("Expected RoutingAction"),
        }
    }

    #[test]
    fn test_assign_route_validation_requires_route() {
        let result = AssignRouteActionBuilder::new()
            .for_entity("ego")
            .build_action();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Route reference is required"));
    }

    #[test]
    fn test_follow_route_direct() {
        let route = Route::new("test_route", false);

        let builder = FollowRouteActionBuilder::new()
            .for_entity("ego")
            .with_direct_route(route)
            .build_action()
            .unwrap();

        match builder {
            PrivateAction::RoutingAction(ref action) => {
                assert!(action.follow_route_action.is_some());
                assert!(action.assign_route_action.is_none());
                assert!(action.follow_trajectory_action.is_none());

                let follow_action = action.follow_route_action.as_ref().unwrap();
                assert!(matches!(follow_action.route_ref, RouteRef::Direct(_)));
            }
            _ => panic!("Expected RoutingAction"),
        }
    }

    #[test]
    fn test_follow_route_catalog() {
        let builder = FollowRouteActionBuilder::new()
            .for_entity("ego")
            .with_catalog_route("RouteCatalog", "MyRoute")
            .build_action()
            .unwrap();

        match builder {
            PrivateAction::RoutingAction(ref action) => {
                let follow_action = action.follow_route_action.as_ref().unwrap();
                assert!(matches!(follow_action.route_ref, RouteRef::Catalog(_)));
            }
            _ => panic!("Expected RoutingAction"),
        }
    }

    #[test]
    fn test_follow_route_validation_requires_route() {
        let result = FollowRouteActionBuilder::new()
            .for_entity("ego")
            .build_action();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Route reference is required"));
    }

    #[test]
    fn test_maneuver_action_trait() {
        let assign_builder = AssignRouteActionBuilder::new().for_entity("test_entity");
        assert_eq!(assign_builder.entity_ref(), Some("test_entity"));

        let follow_builder = FollowRouteActionBuilder::new().for_entity("another_entity");
        assert_eq!(follow_builder.entity_ref(), Some("another_entity"));
    }
}
