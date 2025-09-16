//! Routing and navigation types for OpenSCENARIO
//!
//! This module contains:
//! - Core Route and Waypoint types per XSD specification
//! - RouteRef for direct and catalog-based route references
//! - Route analytics and validation utilities
//! - Integration with existing position and action systems
//! - Support for parameterizable routes and waypoints
//!
//! Contributes to project by:
//! - Implementing complete routing system for autonomous driving scenarios
//! - Enabling mission planning and navigation scenario testing
//! - Supporting both simple point-to-point and complex multi-waypoint routes
//! - Providing route analytics for distance calculation and validation
//! - Facilitating route reuse through catalog integration

use crate::types::basic::{Boolean, Double, OSString};
use crate::types::enums::RouteStrategy;
use crate::types::positions::Position;
use serde::{Deserialize, Serialize};

/// Simple catalog reference for routes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename = "CatalogReference")]
pub struct CatalogReference {
    /// Name of the catalog
    #[serde(rename = "@catalogName")]
    pub catalog_name: OSString,
    
    /// Name of the entry within the catalog
    #[serde(rename = "@entryName")]
    pub entry_name: OSString,
    
    /// Optional parameter assignments
    #[serde(rename = "ParameterAssignments", skip_serializing_if = "Option::is_none")]
    pub parameter_assignments: Option<ParameterAssignments>,
}

/// Parameter assignments for catalog references
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename = "ParameterAssignments")]
pub struct ParameterAssignments {
    /// List of parameter assignments
    #[serde(rename = "ParameterAssignment")]
    pub assignments: Vec<ParameterAssignment>,
}

/// Individual parameter assignment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename = "ParameterAssignment")]
pub struct ParameterAssignment {
    /// Parameter name to assign
    #[serde(rename = "@parameterRef")]
    pub parameter_ref: OSString,
    
    /// Value to assign to the parameter
    #[serde(rename = "@value")]
    pub value: OSString,
}

/// Simple parameter declarations for routes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename = "ParameterDeclarations")]
pub struct ParameterDeclarations {
    /// List of parameter declarations
    #[serde(rename = "ParameterDeclaration")]
    pub parameter_declarations: Vec<ParameterDeclaration>,
}

/// Individual parameter declaration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename = "ParameterDeclaration")]
pub struct ParameterDeclaration {
    /// Parameter name
    #[serde(rename = "@name")]
    pub name: OSString,
    
    /// Parameter type
    #[serde(rename = "@parameterType")]
    pub parameter_type: ParameterType,
    
    /// Default value
    #[serde(rename = "@value")]
    pub value: OSString,
    
    /// Constraint groups (simplified for now)
    #[serde(rename = "ConstraintGroup", skip_serializing_if = "Vec::is_empty", default)]
    pub constraint_groups: Vec<ValueConstraintGroup>,
}

/// Parameter type enumeration (simplified)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ParameterType {
    #[serde(rename = "double")]
    Double,
    #[serde(rename = "int")]
    Int,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "boolean")]
    Boolean,
}

/// Value constraint group (simplified)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValueConstraintGroup {
    /// List of constraints
    pub constraints: Vec<ValueConstraint>,
}

/// Value constraint (simplified)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValueConstraint {
    /// Constraint rule
    pub rule: String,
    /// Constraint value
    pub value: String,
}

/// Complete route definition with waypoints and metadata
///
/// Represents a route as defined in the OpenSCENARIO XSD specification,
/// containing a sequence of waypoints with routing strategies and optional
/// parameter declarations for reusable, parameterizable routes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Route")]
pub struct Route {
    /// Optional parameter declarations for this route
    #[serde(rename = "ParameterDeclarations", skip_serializing_if = "Option::is_none")]
    pub parameter_declarations: Option<ParameterDeclarations>,
    
    /// Waypoints defining the route (minimum 2 required per XSD)
    #[serde(rename = "Waypoint")]
    pub waypoints: Vec<Waypoint>,
    
    /// Whether the route is closed (forms a loop)
    #[serde(rename = "@closed")]
    pub closed: Boolean,
    
    /// Unique name for this route
    #[serde(rename = "@name")]
    pub name: OSString,
}

/// Waypoint in a route with position and routing strategy
///
/// Represents a single waypoint along a route with a specific position
/// and routing strategy for how to reach this waypoint from the previous one.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Waypoint")]
pub struct Waypoint {
    /// Position of this waypoint
    #[serde(rename = "Position")]
    pub position: Position,
    
    /// Routing strategy to reach this waypoint
    #[serde(rename = "@routeStrategy")]
    pub route_strategy: RouteStrategy,
}

/// Route reference - can contain direct route or catalog reference
///
/// Represents a reference to a route that can be either defined inline
/// or referenced from a catalog, enabling route reuse and parameterization.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RouteRef {
    /// Direct route definition
    #[serde(rename = "Route")]
    Direct(Route),
    /// Reference to a route in a catalog
    #[serde(rename = "CatalogReference")]
    Catalog(CatalogReference),
}

// Default implementations
impl Default for Route {
    fn default() -> Self {
        Self {
            parameter_declarations: None,
            waypoints: Vec::new(),
            closed: Boolean::literal(false),
            name: OSString::literal("DefaultRoute".to_string()),
        }
    }
}

impl Default for Waypoint {
    fn default() -> Self {
        Self {
            position: Position::default(),
            route_strategy: RouteStrategy::Shortest,
        }
    }
}

impl Default for RouteRef {
    fn default() -> Self {
        RouteRef::Direct(Route::default())
    }
}

// Implementation methods for Route
impl Route {
    /// Create a new route with the given name and closure setting
    pub fn new(name: impl Into<String>, closed: bool) -> Self {
        Self {
            parameter_declarations: None,
            waypoints: Vec::new(),
            closed: Boolean::literal(closed),
            name: OSString::literal(name.into()),
        }
    }

    /// Add a waypoint to this route
    pub fn add_waypoint(mut self, waypoint: Waypoint) -> Self {
        self.waypoints.push(waypoint);
        self
    }

    /// Add a position with routing strategy as a waypoint
    pub fn add_position(mut self, position: Position, strategy: RouteStrategy) -> Self {
        self.waypoints.push(Waypoint::new(position, strategy));
        self
    }

    /// Set parameter declarations for this route
    pub fn with_parameter_declarations(mut self, declarations: ParameterDeclarations) -> Self {
        self.parameter_declarations = Some(declarations);
        self
    }

    /// Get the number of waypoints in this route
    pub fn waypoint_count(&self) -> usize {
        self.waypoints.len()
    }

    /// Check if the route is closed (forms a loop)
    pub fn is_closed(&self) -> crate::Result<bool> {
        self.closed.resolve(&std::collections::HashMap::new())
    }

    /// Calculate total distance of the route
    ///
    /// This is a simplified implementation that calculates Euclidean distance
    /// between consecutive waypoints. In a real implementation, this would
    /// consider the actual road network and routing strategies.
    pub fn total_distance(&self) -> crate::Result<f64> {
        if self.waypoints.len() < 2 {
            return Ok(0.0);
        }

        let mut total = 0.0;
        for i in 1..self.waypoints.len() {
            total += self.calculate_waypoint_distance(&self.waypoints[i-1], &self.waypoints[i])?;
        }

        // If closed, add distance from last to first waypoint
        if self.is_closed()? && self.waypoints.len() > 2 {
            total += self.calculate_waypoint_distance(
                self.waypoints.last().unwrap(),
                &self.waypoints[0]
            )?;
        }

        Ok(total)
    }

    /// Calculate distances between consecutive waypoints
    pub fn segment_distances(&self) -> crate::Result<Vec<f64>> {
        if self.waypoints.len() < 2 {
            return Ok(Vec::new());
        }

        let mut distances = Vec::new();
        for i in 1..self.waypoints.len() {
            distances.push(self.calculate_waypoint_distance(&self.waypoints[i-1], &self.waypoints[i])?);
        }

        // If closed, add distance from last to first waypoint
        if self.is_closed()? && self.waypoints.len() > 2 {
            distances.push(self.calculate_waypoint_distance(
                self.waypoints.last().unwrap(),
                &self.waypoints[0]
            )?);
        }

        Ok(distances)
    }

    /// Validate route continuity and constraints
    pub fn validate_continuity(&self) -> crate::Result<()> {
        if self.waypoints.len() < 2 {
            return Err(crate::Error::ValidationError {
                field: "waypoints".to_string(),
                message: "Route must have at least 2 waypoints".to_string(),
            });
        }

        // Additional validation logic can be added here
        // - Check for reasonable distances between waypoints
        // - Validate position types are compatible
        // - Check routing strategies are appropriate

        Ok(())
    }

    /// Check if all waypoints are reachable from their predecessors
    ///
    /// This is a simplified implementation that always returns true.
    /// In a real implementation, this would check road network connectivity.
    pub fn check_waypoint_reachability(&self) -> crate::Result<Vec<bool>> {
        Ok(vec![true; self.waypoints.len()])
    }

    /// Calculate distance between two waypoints
    ///
    /// This is a simplified implementation using Euclidean distance for WorldPosition.
    /// In a real implementation, this would consider the routing strategy and road network.
    fn calculate_waypoint_distance(&self, wp1: &Waypoint, wp2: &Waypoint) -> crate::Result<f64> {
        // Simplified distance calculation - only handles WorldPosition for now
        if let (Some(pos1), Some(pos2)) = (&wp1.position.world_position, &wp2.position.world_position) {
            let dx = pos2.x.resolve(&std::collections::HashMap::new())? - pos1.x.resolve(&std::collections::HashMap::new())?;
            let dy = pos2.y.resolve(&std::collections::HashMap::new())? - pos1.y.resolve(&std::collections::HashMap::new())?;
            let dz = match (&pos1.z, &pos2.z) {
                (Some(z1), Some(z2)) => z2.resolve(&std::collections::HashMap::new())? - z1.resolve(&std::collections::HashMap::new())?,
                _ => 0.0, // If z coordinates are missing, assume 2D distance
            };
            Ok((dx * dx + dy * dy + dz * dz).sqrt())
        } else {
            // For other position types, return a default distance
            // In a real implementation, this would handle all position types
            Ok(100.0) // Default distance in meters
        }
    }
}

// Implementation methods for Waypoint
impl Waypoint {
    /// Create a new waypoint with the given position and routing strategy
    pub fn new(position: Position, route_strategy: RouteStrategy) -> Self {
        Self {
            position,
            route_strategy,
        }
    }

    /// Create a waypoint with a world position
    pub fn world_position(x: f64, y: f64, z: f64, strategy: RouteStrategy) -> Self {
        use crate::types::positions::WorldPosition;
        
        let mut position = Position::default();
        position.world_position = Some(WorldPosition {
            x: Double::literal(x),
            y: Double::literal(y),
            z: Some(Double::literal(z)),
            h: Some(Double::literal(0.0)),
            p: Some(Double::literal(0.0)),
            r: Some(Double::literal(0.0)),
        });
        position.relative_world_position = None;
        position.road_position = None;
        position.lane_position = None;

        Self::new(position, strategy)
    }

    /// Create a waypoint with a lane position
    pub fn lane_position(road_id: impl Into<String>, lane_id: impl Into<String>, s: f64, strategy: RouteStrategy) -> Self {
        use crate::types::positions::{LanePosition, Orientation};
        
        let mut position = Position::default();
        position.world_position = None;
        position.relative_world_position = None;
        position.road_position = None;
        position.lane_position = Some(LanePosition {
            road_id: OSString::literal(road_id.into()),
            lane_id: OSString::literal(lane_id.into()),
            s: Double::literal(s),
            offset: Double::literal(0.0),
            orientation: Some(Orientation {
                h: Some(Double::literal(0.0)),
                p: Some(Double::literal(0.0)),
                r: Some(Double::literal(0.0)),
            }),
        });

        Self::new(position, strategy)
    }

    /// Create a waypoint with a relative world position
    pub fn relative_world_position(entity_ref: impl Into<String>, dx: f64, dy: f64, dz: f64, strategy: RouteStrategy) -> Self {
        use crate::types::positions::RelativeWorldPosition;
        
        let mut position = Position::default();
        position.world_position = None;
        position.relative_world_position = Some(RelativeWorldPosition {
            entity_ref: OSString::literal(entity_ref.into()),
            dx: Double::literal(dx),
            dy: Double::literal(dy),
            dz: Double::literal(dz),
        });
        position.road_position = None;
        position.lane_position = None;

        Self::new(position, strategy)
    }
}

// Implementation methods for RouteRef
impl RouteRef {
    /// Create a direct route reference
    pub fn direct(route: Route) -> Self {
        RouteRef::Direct(route)
    }

    /// Create a catalog route reference
    pub fn catalog(catalog_name: impl Into<String>, entry_name: impl Into<String>) -> Self {
        RouteRef::Catalog(CatalogReference {
            catalog_name: OSString::literal(catalog_name.into()),
            entry_name: OSString::literal(entry_name.into()),
            parameter_assignments: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::enums::RouteStrategy;

    #[test]
    fn test_route_creation_and_building() {
        let route = Route::new("TestRoute", false)
            .add_position(Position::default(), RouteStrategy::Shortest)
            .add_position(Position::default(), RouteStrategy::Fastest);

        assert_eq!(route.name.resolve(&std::collections::HashMap::new()).unwrap(), "TestRoute");
        assert_eq!(route.waypoint_count(), 2);
        assert!(!route.is_closed().unwrap());
    }

    #[test]
    fn test_waypoint_convenience_constructors() {
        let wp1 = Waypoint::world_position(100.0, 200.0, 0.0, RouteStrategy::Shortest);
        assert!(wp1.position.world_position.is_some());
        assert_eq!(wp1.route_strategy, RouteStrategy::Shortest);

        let wp2 = Waypoint::lane_position("road1", "lane1", 50.0, RouteStrategy::Fastest);
        assert!(wp2.position.lane_position.is_some());
        assert_eq!(wp2.route_strategy, RouteStrategy::Fastest);

        let wp3 = Waypoint::relative_world_position("entity1", 10.0, 20.0, 0.0, RouteStrategy::LeastIntersections);
        assert!(wp3.position.relative_world_position.is_some());
        assert_eq!(wp3.route_strategy, RouteStrategy::LeastIntersections);
    }

    #[test]
    fn test_route_distance_calculations() {
        let route = Route::new("DistanceTest", false)
            .add_waypoint(Waypoint::world_position(0.0, 0.0, 0.0, RouteStrategy::Shortest))
            .add_waypoint(Waypoint::world_position(100.0, 0.0, 0.0, RouteStrategy::Shortest));

        let total_distance = route.total_distance().unwrap();
        assert!((total_distance - 100.0).abs() < 0.001);

        let segment_distances = route.segment_distances().unwrap();
        assert_eq!(segment_distances.len(), 1);
        assert!((segment_distances[0] - 100.0).abs() < 0.001);
    }

    #[test]
    fn test_closed_route_behavior() {
        let route = Route::new("ClosedRoute", true)
            .add_waypoint(Waypoint::world_position(0.0, 0.0, 0.0, RouteStrategy::Shortest))
            .add_waypoint(Waypoint::world_position(100.0, 0.0, 0.0, RouteStrategy::Shortest))
            .add_waypoint(Waypoint::world_position(100.0, 100.0, 0.0, RouteStrategy::Shortest));

        assert!(route.is_closed().unwrap());
        
        let segment_distances = route.segment_distances().unwrap();
        assert_eq!(segment_distances.len(), 3); // Including return to start
    }

    #[test]
    fn test_route_validation() {
        let empty_route = Route::new("Empty", false);
        assert!(empty_route.validate_continuity().is_err());

        let single_waypoint_route = Route::new("Single", false)
            .add_waypoint(Waypoint::world_position(0.0, 0.0, 0.0, RouteStrategy::Shortest));
        assert!(single_waypoint_route.validate_continuity().is_err());

        let valid_route = Route::new("Valid", false)
            .add_waypoint(Waypoint::world_position(0.0, 0.0, 0.0, RouteStrategy::Shortest))
            .add_waypoint(Waypoint::world_position(100.0, 0.0, 0.0, RouteStrategy::Shortest));
        assert!(valid_route.validate_continuity().is_ok());
    }

    #[test]
    fn test_route_ref_variants() {
        let direct_route = Route::new("DirectRoute", false);
        let route_ref1 = RouteRef::direct(direct_route);
        assert!(matches!(route_ref1, RouteRef::Direct(_)));

        let route_ref2 = RouteRef::catalog("RouteCatalog", "MyRoute");
        assert!(matches!(route_ref2, RouteRef::Catalog(_)));
    }

    #[test]
    fn test_xml_serialization_roundtrip() {
        let route = Route::new("SerializationTest", false)
            .add_waypoint(Waypoint::world_position(0.0, 0.0, 0.0, RouteStrategy::Shortest))
            .add_waypoint(Waypoint::world_position(100.0, 100.0, 0.0, RouteStrategy::Fastest));

        // Test serialization
        let xml = quick_xml::se::to_string(&route).expect("Failed to serialize");
        assert!(xml.contains("SerializationTest"));
        assert!(xml.contains("shortest"));
        assert!(xml.contains("fastest"));

        // Test deserialization
        let deserialized: Route = quick_xml::de::from_str(&xml).expect("Failed to deserialize");
        assert_eq!(deserialized.name.resolve(&std::collections::HashMap::new()).unwrap(), "SerializationTest");
        assert_eq!(deserialized.waypoints.len(), 2);
    }

    #[test]
    fn test_complex_multi_waypoint_route() {
        let route = Route::new("ComplexRoute", true)
            .add_waypoint(Waypoint::world_position(0.0, 0.0, 0.0, RouteStrategy::Shortest))
            .add_waypoint(Waypoint::lane_position("road1", "lane1", 100.0, RouteStrategy::Fastest))
            .add_waypoint(Waypoint::relative_world_position("vehicle1", 50.0, 0.0, 0.0, RouteStrategy::LeastIntersections))
            .add_waypoint(Waypoint::world_position(200.0, 200.0, 0.0, RouteStrategy::Random));

        assert_eq!(route.waypoint_count(), 4);
        assert!(route.is_closed().unwrap());
        assert!(route.validate_continuity().is_ok());
        
        let reachability = route.check_waypoint_reachability().unwrap();
        assert_eq!(reachability.len(), 4);
        assert!(reachability.iter().all(|&x| x)); // All should be reachable in this simple implementation
    }

    #[test]
    fn test_parameter_declarations_support() {
        let mut declarations = ParameterDeclarations::default();
        declarations.parameter_declarations.push(ParameterDeclaration {
            name: OSString::literal("routeSpeed".to_string()),
            parameter_type: ParameterType::Double,
            value: OSString::literal("50.0".to_string()),
            constraint_groups: Vec::new(),
        });

        let route = Route::new("ParameterizedRoute", false)
            .with_parameter_declarations(declarations)
            .add_waypoint(Waypoint::world_position(0.0, 0.0, 0.0, RouteStrategy::Shortest))
            .add_waypoint(Waypoint::world_position(100.0, 0.0, 0.0, RouteStrategy::Fastest));

        assert!(route.parameter_declarations.is_some());
        assert_eq!(route.parameter_declarations.as_ref().unwrap().parameter_declarations.len(), 1);
    }
}