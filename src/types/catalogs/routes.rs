//! Route catalog types for OpenSCENARIO reusable route definitions
//!
//! This module contains catalog-specific route types that enable reuse of
//! route definitions across multiple scenarios with parameter substitution.

use crate::types::basic::{Boolean, OSString, ParameterDeclarations, Value};
use crate::types::enums::RouteStrategy;
use crate::types::positions::Position;
use serde::{Deserialize, Serialize};

/// Route definition within a catalog
///
/// Represents a route with waypoints and routing strategies that can be
/// parameterized and reused across multiple scenarios.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Route")]
pub struct CatalogRoute {
    /// Unique name for this route in the catalog
    #[serde(rename = "@name")]
    pub name: String,

    /// Whether the route is closed (forms a loop) — required per XSD
    #[serde(rename = "@closed")]
    pub closed: Boolean,

    /// Parameter declarations for this route
    #[serde(
        rename = "ParameterDeclarations",
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<ParameterDeclarations>,

    /// Waypoints defining the route
    #[serde(rename = "Waypoint")]
    pub waypoints: Vec<RouteWaypoint>,
}

impl Default for CatalogRoute {
    fn default() -> Self {
        Self {
            name: "DefaultCatalogRoute".to_string(),
            closed: Value::Literal(false),
            parameter_declarations: None,
            waypoints: Vec::new(),
        }
    }
}

/// Waypoint in a route with position and routing configuration
///
/// Represents a point along a route with optional routing strategy
/// and timing information that can be parameterized.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Waypoint")]
pub struct RouteWaypoint {
    /// Position of this waypoint
    #[serde(rename = "Position")]
    pub position: Position,

    /// Routing strategy to reach this waypoint (required per XSD)
    #[serde(rename = "@routeStrategy")]
    pub route_strategy: RouteStrategy,
}

impl Default for RouteWaypoint {
    fn default() -> Self {
        use crate::types::positions::WorldPosition;

        Self {
            position: Position {
                world_position: Some(WorldPosition {
                    x: Value::Literal(0.0),
                    y: Value::Literal(0.0),
                    z: Some(Value::Literal(0.0)),
                    h: None,
                    p: None,
                    r: None,
                }),
                relative_world_position: None,
                road_position: None,
                relative_road_position: None,
                lane_position: None,
                relative_lane_position: None,
                route_position: None,
                trajectory_position: None,
                geographic_position: None,
                relative_object_position: None,
            },
            route_strategy: RouteStrategy::Fastest,
        }
    }
}

/// Parameter assignments for route references
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "ParameterAssignments")]
pub struct RouteParameterAssignments {
    /// List of parameter assignments
    #[serde(rename = "ParameterAssignment")]
    pub assignments: Vec<RouteParameterAssignment>,
}

/// Individual parameter assignment for routes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "ParameterAssignment")]
pub struct RouteParameterAssignment {
    /// Parameter name to assign
    #[serde(rename = "@parameterRef")]
    pub parameter_ref: OSString,

    /// Value to assign to the parameter
    #[serde(rename = "@value")]
    pub value: OSString,
}

// Implementation methods for catalog routes

impl CatalogRoute {
    /// Creates a new catalog route with the specified name
    pub fn new(name: String) -> Self {
        Self {
            name,
            closed: Value::Literal(false),
            parameter_declarations: None,
            waypoints: Vec::new(),
        }
    }

    /// Creates a catalog route with parameter declarations
    pub fn with_parameters(name: String, parameters: ParameterDeclarations) -> Self {
        Self {
            name,
            closed: Value::Literal(false),
            parameter_declarations: Some(parameters),
            waypoints: Vec::new(),
        }
    }

    /// Creates a closed route (forms a loop)
    pub fn with_closed(name: String, closed: bool) -> Self {
        Self {
            name,
            closed: Value::Literal(closed),
            parameter_declarations: None,
            waypoints: Vec::new(),
        }
    }

    /// Adds a waypoint to this route
    pub fn add_waypoint(&mut self, waypoint: RouteWaypoint) {
        self.waypoints.push(waypoint);
    }

    /// Adds a simple waypoint with a position and routing strategy
    pub fn add_position_waypoint(&mut self, position: Position, route_strategy: RouteStrategy) {
        self.waypoints.push(RouteWaypoint {
            position,
            route_strategy,
        });
    }

    /// Gets the number of waypoints in this route
    pub fn waypoint_count(&self) -> usize {
        self.waypoints.len()
    }
}

impl RouteWaypoint {
    /// Creates a new waypoint with the specified position and routing strategy
    pub fn new(position: Position, route_strategy: RouteStrategy) -> Self {
        Self {
            position,
            route_strategy,
        }
    }

    /// Creates a waypoint with routing strategy
    pub fn with_strategy(position: Position, strategy: RouteStrategy) -> Self {
        Self {
            position,
            route_strategy: strategy,
        }
    }
}

impl RouteParameterAssignments {
    /// Creates parameter assignments from a list of pairs
    pub fn from_pairs<I>(pairs: I) -> Self
    where
        I: IntoIterator<Item = (OSString, OSString)>,
    {
        let assignments = pairs
            .into_iter()
            .map(|(parameter_ref, value)| RouteParameterAssignment {
                parameter_ref,
                value,
            })
            .collect();

        Self { assignments }
    }

    /// Adds a parameter assignment
    pub fn add_assignment(&mut self, parameter_ref: OSString, value: OSString) {
        self.assignments.push(RouteParameterAssignment {
            parameter_ref,
            value,
        });
    }
}

impl RouteParameterAssignment {
    /// Creates a new parameter assignment
    pub fn new(parameter_ref: OSString, value: OSString) -> Self {
        Self {
            parameter_ref,
            value,
        }
    }
}

/// Converts canonical `basic::ParameterDeclarations` into the
/// `routing::ParameterDeclarations` used by the scenario `Route` type.
///
/// `routing::ValueConstraint`/`ValueConstraintGroup` are re-exports of the
/// `basic` types, so constraint groups carry over unchanged.
fn route_parameter_declarations(
    declarations: ParameterDeclarations,
) -> crate::error::Result<crate::types::routing::ParameterDeclarations> {
    let parameter_declarations = declarations
        .parameter_declarations
        .into_iter()
        .map(|decl| {
            Ok(crate::types::routing::ParameterDeclaration {
                name: decl.name,
                parameter_type: decl.parameter_type,
                value: decl.value,
                constraint_groups: decl.constraint_groups,
            })
        })
        .collect::<crate::error::Result<Vec<_>>>()?;

    Ok(crate::types::routing::ParameterDeclarations {
        parameter_declarations,
    })
}

/// Catalog entity integration so `CatalogRoute` can be used as the entry type
/// in `CatalogContent` and behind a `CatalogReference`.
impl crate::types::catalogs::entities::CatalogEntity for CatalogRoute {
    type ResolvedType = crate::types::routing::Route;

    fn into_scenario_entity(
        self,
        parameters: std::collections::HashMap<String, String>,
    ) -> crate::error::Result<Self::ResolvedType> {
        let waypoints = self
            .waypoints
            .into_iter()
            .map(|w| crate::types::routing::Waypoint {
                position: w.position,
                route_strategy: w.route_strategy,
            })
            .collect::<Vec<_>>();

        Ok(crate::types::routing::Route {
            name: OSString::literal(crate::types::catalogs::entities::resolve_parameter(
                &self.name,
                &parameters,
            )?),
            closed: Boolean::literal(self.closed.resolve(&parameters)?),
            parameter_declarations: self
                .parameter_declarations
                .map(route_parameter_declarations)
                .transpose()?,
            waypoints,
        })
    }

    fn parameter_schema() -> Vec<crate::types::catalogs::entities::ParameterDefinition> {
        vec![
            crate::types::catalogs::entities::ParameterDefinition {
                name: "StartRoadId".to_string(),
                parameter_type: "String".to_string(),
                default_value: Some("road_1".to_string()),
                description: Some("ID of the starting road".to_string()),
            },
            crate::types::catalogs::entities::ParameterDefinition {
                name: "EndRoadId".to_string(),
                parameter_type: "String".to_string(),
                default_value: Some("road_2".to_string()),
                description: Some("ID of the ending road".to_string()),
            },
            crate::types::catalogs::entities::ParameterDefinition {
                name: "Closed".to_string(),
                parameter_type: "Boolean".to_string(),
                default_value: Some("false".to_string()),
                description: Some("Whether the route is closed (loops back to start)".to_string()),
            },
        ]
    }

    fn entity_name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::basic::ParameterDeclaration;
    use crate::types::enums::ParameterType;

    #[test]
    fn test_catalog_route_creation() {
        let route = CatalogRoute::new("TestRoute".to_string());

        assert_eq!(route.name, "TestRoute");
        assert_eq!(route.closed.as_literal().unwrap(), &false);
        assert!(route.parameter_declarations.is_none());
        assert!(route.waypoints.is_empty());
    }

    #[test]
    fn test_route_waypoints() {
        let mut route = CatalogRoute::new("WaypointRoute".to_string());

        let pos1 = Position::default();
        let pos2 = Position::default();

        route.add_position_waypoint(pos1, RouteStrategy::Shortest);

        let waypoint2 = RouteWaypoint::with_strategy(pos2, RouteStrategy::Fastest);
        route.add_waypoint(waypoint2);

        assert_eq!(route.waypoint_count(), 2);
        assert_eq!(route.waypoints[0].route_strategy, RouteStrategy::Shortest);
        assert_eq!(route.waypoints[1].route_strategy, RouteStrategy::Fastest);
    }

    #[test]
    fn test_waypoint_creation() {
        let pos = Position::default();

        let waypoint1 = RouteWaypoint::new(pos.clone(), RouteStrategy::Fastest);
        let waypoint2 = RouteWaypoint::with_strategy(pos, RouteStrategy::Shortest);

        assert_eq!(waypoint1.route_strategy, RouteStrategy::Fastest);
        assert_eq!(waypoint2.route_strategy, RouteStrategy::Shortest);
    }

    #[test]
    fn test_route_with_parameters() {
        let param_decl = ParameterDeclarations {
            parameter_declarations: vec![ParameterDeclaration {
                name: OSString::literal("targetSpeed".to_string()),
                parameter_type: ParameterType::Double,
                value: OSString::literal("50.0".to_string()),
                constraint_groups: Vec::new(),
            }],
        };

        let route = CatalogRoute::with_parameters("ParameterizedRoute".to_string(), param_decl);

        assert_eq!(route.name, "ParameterizedRoute");
        assert!(route.parameter_declarations.is_some());
        assert_eq!(
            route
                .parameter_declarations
                .as_ref()
                .unwrap()
                .parameter_declarations
                .len(),
            1
        );
    }

    #[test]
    fn test_closed_route() {
        let route = CatalogRoute::with_closed("ClosedRoute".to_string(), true);

        assert_eq!(route.closed.as_literal().unwrap(), &true);
    }

    #[test]
    fn test_parameter_assignments() {
        let pairs = vec![
            (
                Value::Literal("param1".to_string()),
                Value::Literal("value1".to_string()),
            ),
            (
                Value::Parameter("param2".to_string()),
                Value::Literal("value2".to_string()),
            ),
        ];

        let assignments = RouteParameterAssignments::from_pairs(pairs);

        assert_eq!(assignments.assignments.len(), 2);
        assert_eq!(
            assignments.assignments[0]
                .parameter_ref
                .as_literal()
                .unwrap(),
            "param1"
        );
        assert!(matches!(
            assignments.assignments[1].parameter_ref,
            Value::Parameter(_)
        ));
    }

    #[test]
    fn test_catalog_route_into_scenario_entity() {
        use crate::types::catalogs::entities::CatalogEntity;

        let mut route = CatalogRoute::new("ResolvedRoute".to_string());
        route.closed = Value::Parameter("isClosed".to_string());
        route.add_waypoint(RouteWaypoint::with_strategy(
            Position::default(),
            RouteStrategy::Shortest,
        ));
        route.add_waypoint(RouteWaypoint::with_strategy(
            Position::default(),
            RouteStrategy::Fastest,
        ));

        let mut parameters = std::collections::HashMap::new();
        parameters.insert("isClosed".to_string(), "true".to_string());

        let resolved = route.into_scenario_entity(parameters).unwrap();

        assert_eq!(resolved.name.as_literal().unwrap(), "ResolvedRoute");
        assert_eq!(resolved.closed.as_literal(), Some(&true));
        assert_eq!(resolved.waypoints.len(), 2);
        assert_eq!(resolved.waypoints[0].route_strategy, RouteStrategy::Shortest);
        assert_eq!(resolved.waypoints[1].route_strategy, RouteStrategy::Fastest);
    }

    #[test]
    fn test_defaults() {
        let route = CatalogRoute::default();
        let waypoint = RouteWaypoint::default();

        assert_eq!(route.name, "DefaultCatalogRoute");
        assert_eq!(waypoint.route_strategy, RouteStrategy::Fastest);
    }
}
