//! `CatalogRoute` and `RouteWaypoint`: a route in its catalog-file form, with parameter
//! declarations covering the waypoints it holds.

use crate::types::basic::{Boolean, MinVec, OSString, ParameterDeclarations, Value};
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
    /// Unique name for this route in the catalog. XSD `Route`
    /// (`Schema/OpenSCENARIO.xsd:1955`, the same complex type `CatalogRoute`
    /// mirrors) types `@name` as the schema's `String`, a union including the
    /// parameter member.
    #[serde(rename = "@name")]
    pub name: OSString,

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
    ///
    /// XSD `Route` (`:1958-1965`, the same complex type `CatalogRoute` mirrors):
    /// `Waypoint` carries `minOccurs="2"`, unbounded.
    #[serde(rename = "Waypoint")]
    pub waypoints: MinVec<RouteWaypoint, 2>,
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
    pub route_strategy: Value<RouteStrategy>,
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
    /// Creates a new catalog route with the given name and waypoints.
    ///
    /// XSD `Route` (`Schema/OpenSCENARIO.xsd:1958`, the complex type `CatalogRoute`
    /// mirrors) declares `<xsd:element name="Waypoint" minOccurs="2" maxOccurs="unbounded"/>`,
    /// so this constructor takes the waypoints up front and returns a `Result` rather
    /// than offering an `add_waypoint` chain that could stop after one and leave nothing
    /// downstream aware of it. Build the vector first and hand it over once.
    pub fn new(name: String, waypoints: Vec<RouteWaypoint>) -> crate::error::Result<Self> {
        Ok(Self {
            name: OSString::literal(name),
            closed: Value::Literal(false),
            parameter_declarations: None,
            waypoints: MinVec::new(waypoints)?,
        })
    }

    /// Creates a catalog route with parameter declarations
    pub fn with_parameters(
        name: String,
        waypoints: Vec<RouteWaypoint>,
        parameters: ParameterDeclarations,
    ) -> crate::error::Result<Self> {
        Ok(Self {
            name: OSString::literal(name),
            closed: Value::Literal(false),
            parameter_declarations: Some(parameters),
            waypoints: MinVec::new(waypoints)?,
        })
    }

    /// Creates a closed route (forms a loop)
    pub fn with_closed(
        name: String,
        waypoints: Vec<RouteWaypoint>,
        closed: bool,
    ) -> crate::error::Result<Self> {
        Ok(Self {
            name: OSString::literal(name),
            closed: Value::Literal(closed),
            parameter_declarations: None,
            waypoints: MinVec::new(waypoints)?,
        })
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
            route_strategy: Value::Literal(route_strategy),
        }
    }

    /// Creates a waypoint with routing strategy
    pub fn with_strategy(position: Position, strategy: RouteStrategy) -> Self {
        Self {
            position,
            route_strategy: Value::Literal(strategy),
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

/// Catalog entity integration so `CatalogRoute` can be used as the entry type
/// in `CatalogContent` and behind a `CatalogReference`.
impl crate::types::catalogs::entities::CatalogEntity for CatalogRoute {
    fn entity_name(&self) -> &str {
        self.name
            .as_literal()
            .map(String::as_str)
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::basic::ParameterDeclaration;
    use crate::types::enums::ParameterType;

    #[test]
    fn test_catalog_route_creation() {
        let waypoints = vec![
            RouteWaypoint::new(Position::world_origin(), RouteStrategy::Shortest),
            RouteWaypoint::new(Position::world_origin(), RouteStrategy::Fastest),
        ];
        let route = CatalogRoute::new("TestRoute".to_string(), waypoints).unwrap();

        assert_eq!(route.name.as_literal().unwrap(), "TestRoute");
        assert_eq!(route.closed.as_literal().unwrap(), &false);
        assert!(route.parameter_declarations.is_none());
        assert_eq!(route.waypoint_count(), 2);
    }

    #[test]
    fn test_catalog_route_rejects_fewer_than_two_waypoints() {
        let one = vec![RouteWaypoint::new(
            Position::world_origin(),
            RouteStrategy::Shortest,
        )];
        assert!(CatalogRoute::new("ShortRoute".to_string(), one).is_err());
        assert!(CatalogRoute::new("EmptyRoute".to_string(), Vec::new()).is_err());
    }

    #[test]
    fn test_route_waypoints() {
        let pos1 = Position::world_origin();
        let pos2 = Position::world_origin();

        let waypoint1 = RouteWaypoint::with_strategy(pos1, RouteStrategy::Shortest);
        let waypoint2 = RouteWaypoint::with_strategy(pos2, RouteStrategy::Fastest);

        let route =
            CatalogRoute::new("WaypointRoute".to_string(), vec![waypoint1, waypoint2]).unwrap();

        assert_eq!(route.waypoint_count(), 2);
        assert_eq!(
            route.waypoints[0].route_strategy,
            Value::Literal(RouteStrategy::Shortest)
        );
        assert_eq!(
            route.waypoints[1].route_strategy,
            Value::Literal(RouteStrategy::Fastest)
        );
    }

    #[test]
    fn test_waypoint_creation() {
        let pos = Position::world_origin();

        let waypoint1 = RouteWaypoint::new(pos.clone(), RouteStrategy::Fastest);
        let waypoint2 = RouteWaypoint::with_strategy(pos, RouteStrategy::Shortest);

        assert_eq!(
            waypoint1.route_strategy,
            Value::Literal(RouteStrategy::Fastest)
        );
        assert_eq!(
            waypoint2.route_strategy,
            Value::Literal(RouteStrategy::Shortest)
        );
    }

    #[test]
    fn test_route_with_parameters() {
        let param_decl = ParameterDeclarations {
            parameter_declarations: vec![ParameterDeclaration {
                name: OSString::literal("targetSpeed".to_string()),
                parameter_type: Value::Literal(ParameterType::Double),
                value: OSString::literal("50.0".to_string()),
                constraint_groups: Vec::new(),
            }],
        };

        let waypoints = vec![
            RouteWaypoint::new(Position::world_origin(), RouteStrategy::Shortest),
            RouteWaypoint::new(Position::world_origin(), RouteStrategy::Fastest),
        ];
        let route =
            CatalogRoute::with_parameters("ParameterizedRoute".to_string(), waypoints, param_decl)
                .unwrap();

        assert_eq!(route.name.as_literal().unwrap(), "ParameterizedRoute");
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
        let waypoints = vec![
            RouteWaypoint::new(Position::world_origin(), RouteStrategy::Shortest),
            RouteWaypoint::new(Position::world_origin(), RouteStrategy::Fastest),
        ];
        let route = CatalogRoute::with_closed("ClosedRoute".to_string(), waypoints, true).unwrap();

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
    fn test_constructors_do_not_fabricate_name_or_strategy() {
        let waypoints = vec![
            RouteWaypoint::new(Position::world_origin(), RouteStrategy::Shortest),
            RouteWaypoint::new(Position::world_origin(), RouteStrategy::Fastest),
        ];
        let route = CatalogRoute::new("ExplicitRoute".to_string(), waypoints).unwrap();
        let waypoint = RouteWaypoint::new(Position::world_origin(), RouteStrategy::Fastest);

        assert_eq!(route.name.as_literal().unwrap(), "ExplicitRoute");
        assert_eq!(
            waypoint.route_strategy,
            Value::Literal(RouteStrategy::Fastest)
        );
    }
}
