//! Route catalog types for OpenSCENARIO reusable route definitions
//!
//! This module contains catalog-specific route types that enable reuse of
//! route definitions across multiple scenarios with parameter substitution.

use crate::types::basic::{Boolean, Double, Int, OSString, ParameterDeclarations, Value};
use crate::types::enums::{RouteStrategy, RoutingAlgorithm};
use crate::types::positions::Position;
use serde::{Deserialize, Serialize};

/// Route catalog containing reusable route definitions
///
/// Represents a collection of route definitions that can be referenced
/// from scenarios, enabling modular route design and path reuse.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "RouteCatalog")]
pub struct RouteCatalog {
    /// Version information for catalog compatibility
    #[serde(rename = "@revMajor")]
    pub rev_major: Int,

    #[serde(rename = "@revMinor")]
    pub rev_minor: Int,

    /// Collection of route entries in this catalog
    #[serde(rename = "Route")]
    pub routes: Vec<CatalogRoute>,
}

impl Default for RouteCatalog {
    fn default() -> Self {
        Self {
            rev_major: Value::Literal(1),
            rev_minor: Value::Literal(0),
            routes: Vec::new(),
        }
    }
}

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

    /// Whether the route is closed (forms a loop)
    #[serde(rename = "@closed", skip_serializing_if = "Option::is_none")]
    pub closed: Option<Boolean>,

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
            closed: None,
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

    /// Routing strategy to reach this waypoint
    #[serde(rename = "@routeStrategy", skip_serializing_if = "Option::is_none")]
    pub route_strategy: Option<RouteStrategy>,

    /// Routing algorithm to use
    #[serde(rename = "@routingAlgorithm", skip_serializing_if = "Option::is_none")]
    pub routing_algorithm: Option<RoutingAlgorithm>,

    /// Time constraint for reaching this waypoint
    #[serde(rename = "@time", skip_serializing_if = "Option::is_none")]
    pub time: Option<Double>,

    /// Speed constraint at this waypoint
    #[serde(rename = "@speed", skip_serializing_if = "Option::is_none")]
    pub speed: Option<Double>,

    /// Lane constraints at this waypoint
    #[serde(rename = "LaneConstraints", skip_serializing_if = "Option::is_none")]
    pub lane_constraints: Option<LaneConstraints>,
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
                trajectory_position: None,
                geographic_position: None,
                relative_object_position: None,
            },
            route_strategy: None,
            routing_algorithm: None,
            time: None,
            speed: None,
            lane_constraints: None,
        }
    }
}

/// Lane constraints for route waypoints
///
/// Specifies which lanes are allowed or preferred at a waypoint,
/// with optional parameterization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "LaneConstraints")]
pub struct LaneConstraints {
    /// Preferred lane ID (can be parameterized)
    #[serde(rename = "@preferredLane", skip_serializing_if = "Option::is_none")]
    pub preferred_lane: Option<Int>,

    /// Allowed lane IDs
    #[serde(rename = "AllowedLane", skip_serializing_if = "Vec::is_empty", default)]
    pub allowed_lanes: Vec<AllowedLane>,

    /// Forbidden lane IDs
    #[serde(
        rename = "ForbiddenLane",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub forbidden_lanes: Vec<ForbiddenLane>,
}

impl Default for LaneConstraints {
    fn default() -> Self {
        Self {
            preferred_lane: None,
            allowed_lanes: Vec::new(),
            forbidden_lanes: Vec::new(),
        }
    }
}

/// Allowed lane specification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "AllowedLane")]
pub struct AllowedLane {
    /// Lane ID that is allowed (can be parameterized)
    #[serde(rename = "@laneId")]
    pub lane_id: Int,
}

/// Forbidden lane specification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "ForbiddenLane")]
pub struct ForbiddenLane {
    /// Lane ID that is forbidden (can be parameterized)
    #[serde(rename = "@laneId")]
    pub lane_id: Int,
}

/// Route reference for use in scenarios
///
/// References a route from a catalog with optional parameter overrides.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "RouteRef")]
pub struct RouteRef {
    /// Name of the route in the catalog
    #[serde(rename = "@route")]
    pub route: OSString,

    /// Optional parameter assignments
    #[serde(
        rename = "ParameterAssignments",
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_assignments: Option<RouteParameterAssignments>,
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

impl RouteCatalog {
    /// Creates a new route catalog with version information
    pub fn new(rev_major: i32, rev_minor: i32) -> Self {
        Self {
            rev_major: Value::Literal(rev_major),
            rev_minor: Value::Literal(rev_minor),
            routes: Vec::new(),
        }
    }

    /// Adds a route to this catalog
    pub fn add_route(&mut self, route: CatalogRoute) {
        self.routes.push(route);
    }

    /// Finds a route by name in this catalog
    pub fn find_route(&self, name: &str) -> Option<&CatalogRoute> {
        self.routes.iter().find(|r| r.name == name)
    }

    /// Gets all route names in this catalog
    pub fn route_names(&self) -> Vec<&str> {
        self.routes.iter().map(|r| r.name.as_str()).collect()
    }
}

impl CatalogRoute {
    /// Creates a new catalog route with the specified name
    pub fn new(name: String) -> Self {
        Self {
            name,
            closed: None,
            parameter_declarations: None,
            waypoints: Vec::new(),
        }
    }

    /// Creates a catalog route with parameter declarations
    pub fn with_parameters(name: String, parameters: ParameterDeclarations) -> Self {
        Self {
            name,
            closed: None,
            parameter_declarations: Some(parameters),
            waypoints: Vec::new(),
        }
    }

    /// Creates a closed route (forms a loop)
    pub fn with_closed(name: String, closed: bool) -> Self {
        Self {
            name,
            closed: Some(Value::Literal(closed)),
            parameter_declarations: None,
            waypoints: Vec::new(),
        }
    }

    /// Adds a waypoint to this route
    pub fn add_waypoint(&mut self, waypoint: RouteWaypoint) {
        self.waypoints.push(waypoint);
    }

    /// Adds a simple waypoint with just a position
    pub fn add_position_waypoint(&mut self, position: Position) {
        self.waypoints.push(RouteWaypoint {
            position,
            route_strategy: None,
            routing_algorithm: None,
            time: None,
            speed: None,
            lane_constraints: None,
        });
    }

    /// Gets the number of waypoints in this route
    pub fn waypoint_count(&self) -> usize {
        self.waypoints.len()
    }
}

impl RouteWaypoint {
    /// Creates a new waypoint with the specified position
    pub fn new(position: Position) -> Self {
        Self {
            position,
            route_strategy: None,
            routing_algorithm: None,
            time: None,
            speed: None,
            lane_constraints: None,
        }
    }

    /// Creates a waypoint with routing strategy
    pub fn with_strategy(position: Position, strategy: RouteStrategy) -> Self {
        Self {
            position,
            route_strategy: Some(strategy),
            routing_algorithm: None,
            time: None,
            speed: None,
            lane_constraints: None,
        }
    }

    /// Creates a waypoint with timing constraint
    pub fn with_time(position: Position, time: Double) -> Self {
        Self {
            position,
            route_strategy: None,
            routing_algorithm: None,
            time: Some(time),
            speed: None,
            lane_constraints: None,
        }
    }

    /// Creates a waypoint with speed constraint
    pub fn with_speed(position: Position, speed: Double) -> Self {
        Self {
            position,
            route_strategy: None,
            routing_algorithm: None,
            time: None,
            speed: Some(speed),
            lane_constraints: None,
        }
    }

    /// Sets lane constraints for this waypoint
    pub fn set_lane_constraints(&mut self, constraints: LaneConstraints) {
        self.lane_constraints = Some(constraints);
    }
}

impl LaneConstraints {
    /// Creates lane constraints with a preferred lane
    pub fn with_preferred_lane(lane_id: Int) -> Self {
        Self {
            preferred_lane: Some(lane_id),
            allowed_lanes: Vec::new(),
            forbidden_lanes: Vec::new(),
        }
    }

    /// Adds an allowed lane
    pub fn add_allowed_lane(&mut self, lane_id: Int) {
        self.allowed_lanes.push(AllowedLane { lane_id });
    }

    /// Adds a forbidden lane
    pub fn add_forbidden_lane(&mut self, lane_id: Int) {
        self.forbidden_lanes.push(ForbiddenLane { lane_id });
    }
}

impl AllowedLane {
    /// Creates an allowed lane specification
    pub fn new(lane_id: Int) -> Self {
        Self { lane_id }
    }
}

impl ForbiddenLane {
    /// Creates a forbidden lane specification
    pub fn new(lane_id: Int) -> Self {
        Self { lane_id }
    }
}

impl RouteRef {
    /// Creates a new route reference
    pub fn new(route: OSString) -> Self {
        Self {
            route,
            parameter_assignments: None,
        }
    }

    /// Creates a route reference with parameter assignments
    pub fn with_parameters(route: OSString, assignments: RouteParameterAssignments) -> Self {
        Self {
            route,
            parameter_assignments: Some(assignments),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::basic::ParameterDeclaration;
    use crate::types::enums::ParameterType;
    

    #[test]
    fn test_route_catalog_creation() {
        let catalog = RouteCatalog::new(1, 2);

        assert_eq!(catalog.rev_major.as_literal().unwrap(), &1);
        assert_eq!(catalog.rev_minor.as_literal().unwrap(), &2);
        assert!(catalog.routes.is_empty());
    }

    #[test]
    fn test_catalog_route_creation() {
        let route = CatalogRoute::new("TestRoute".to_string());

        assert_eq!(route.name, "TestRoute");
        assert!(route.closed.is_none());
        assert!(route.parameter_declarations.is_none());
        assert!(route.waypoints.is_empty());
    }

    #[test]
    fn test_route_catalog_operations() {
        let mut catalog = RouteCatalog::new(1, 0);
        let route1 = CatalogRoute::new("Route1".to_string());
        let route2 = CatalogRoute::new("Route2".to_string());

        catalog.add_route(route1);
        catalog.add_route(route2);

        assert_eq!(catalog.routes.len(), 2);
        assert!(catalog.find_route("Route1").is_some());
        assert!(catalog.find_route("Route2").is_some());
        assert!(catalog.find_route("NonExistent").is_none());

        let names = catalog.route_names();
        assert!(names.contains(&"Route1"));
        assert!(names.contains(&"Route2"));
    }

    #[test]
    fn test_route_waypoints() {
        let mut route = CatalogRoute::new("WaypointRoute".to_string());

        let pos1 = Position::default();
        let pos2 = Position::default();

        route.add_position_waypoint(pos1);

        let waypoint2 = RouteWaypoint::with_strategy(pos2, RouteStrategy::Fastest);
        route.add_waypoint(waypoint2);

        assert_eq!(route.waypoint_count(), 2);
        assert!(route.waypoints[0].route_strategy.is_none());
        assert_eq!(
            route.waypoints[1].route_strategy,
            Some(RouteStrategy::Fastest)
        );
    }

    #[test]
    fn test_waypoint_creation() {
        let pos = Position::default();

        let waypoint1 = RouteWaypoint::new(pos.clone());
        let waypoint2 = RouteWaypoint::with_strategy(pos.clone(), RouteStrategy::Shortest);
        let waypoint3 = RouteWaypoint::with_time(pos.clone(), Value::Literal(30.0));
        let waypoint4 = RouteWaypoint::with_speed(pos, Value::Parameter("maxSpeed".to_string()));

        assert!(waypoint1.route_strategy.is_none());
        assert_eq!(waypoint2.route_strategy, Some(RouteStrategy::Shortest));
        assert_eq!(
            waypoint3.time.as_ref().unwrap().as_literal().unwrap(),
            &30.0
        );
        assert!(matches!(
            waypoint4.speed.as_ref().unwrap(),
            Value::Parameter(_)
        ));
    }

    #[test]
    fn test_lane_constraints() {
        let mut constraints = LaneConstraints::with_preferred_lane(Value::Literal(2));
        constraints.add_allowed_lane(Value::Literal(1));
        constraints.add_allowed_lane(Value::Parameter("laneId".to_string()));
        constraints.add_forbidden_lane(Value::Literal(0));

        assert_eq!(
            constraints
                .preferred_lane
                .as_ref()
                .unwrap()
                .as_literal()
                .unwrap(),
            &2
        );
        assert_eq!(constraints.allowed_lanes.len(), 2);
        assert_eq!(constraints.forbidden_lanes.len(), 1);
        assert!(matches!(
            constraints.allowed_lanes[1].lane_id,
            Value::Parameter(_)
        ));
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

        assert_eq!(route.closed.as_ref().unwrap().as_literal().unwrap(), &true);
    }

    #[test]
    fn test_route_ref() {
        let mut assignments = RouteParameterAssignments {
            assignments: Vec::new(),
        };
        assignments.add_assignment(
            Value::Literal("speed".to_string()),
            Value::Literal("60.0".to_string()),
        );

        let route_ref =
            RouteRef::with_parameters(Value::Literal("MyRoute".to_string()), assignments);

        assert_eq!(route_ref.route.as_literal().unwrap(), "MyRoute");
        assert!(route_ref.parameter_assignments.is_some());
        assert_eq!(
            route_ref
                .parameter_assignments
                .as_ref()
                .unwrap()
                .assignments
                .len(),
            1
        );
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
    fn test_route_serialization() {
        let catalog = RouteCatalog::new(1, 0);

        // Test XML serialization
        let xml_result = quick_xml::se::to_string(&catalog);
        assert!(xml_result.is_ok());

        let xml = xml_result.unwrap();
        assert!(xml.contains("RouteCatalog"));
        assert!(xml.contains("revMajor=\"1\""));
        assert!(xml.contains("revMinor=\"0\""));
    }

    #[test]
    fn test_defaults() {
        let catalog = RouteCatalog::default();
        let route = CatalogRoute::default();
        let waypoint = RouteWaypoint::default();
        let constraints = LaneConstraints::default();

        assert_eq!(catalog.rev_major.as_literal().unwrap(), &1);
        assert_eq!(route.name, "DefaultCatalogRoute");
        assert!(waypoint.route_strategy.is_none());
        assert!(constraints.preferred_lane.is_none());
    }
}
