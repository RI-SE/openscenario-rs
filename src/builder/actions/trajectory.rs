//! Trajectory action builders for path-following scenarios
//!
//! This module provides builders for creating trajectory-based actions, including
//! trajectory definitions with polyline shapes and follow trajectory actions.
//!
//! # Available Builders
//!
//! - [`TrajectoryBuilder`] - Build trajectory definitions with polyline shapes
//! - [`PolylineBuilder`] - Build polyline shapes with time-positioned vertices
//! - [`VertexBuilder`] - Build individual trajectory vertices
//! - [`FollowTrajectoryActionBuilder`] - Build follow trajectory actions
//!
//! # Usage Examples
//!
//! ```rust
//! use openscenario_rs::builder::actions::trajectory::{
//!     TrajectoryBuilder, FollowTrajectoryActionBuilder
//! };
//!
//! // Create a trajectory with vertices
//! let trajectory = TrajectoryBuilder::new()
//!     .name("ego_path")
//!     .closed(false)
//!     .polyline()
//!         .add_vertex()
//!             .time(0.0)
//!             .world_position(0.0, 0.0, 0.0, 0.0)
//!             .finish()?
//!         .add_vertex()
//!             .time(1.0)
//!             .world_position(10.0, 0.0, 0.0, 0.0)
//!             .finish()?
//!         .finish()
//!     .build()?;
//!
//! // Create a follow trajectory action
//! let action = FollowTrajectoryActionBuilder::new()
//!     .for_entity("ego_vehicle")
//!     .with_trajectory(trajectory)
//!     .following_mode_follow()
//!     .build_action()?;
//! # Ok::<(), openscenario_rs::builder::BuilderError>(())
//! ```

use crate::builder::actions::base::{ActionBuilder, ManeuverAction};
use crate::builder::{BuilderError, BuilderResult};
use crate::types::{
    actions::movement::{
        FollowTrajectoryAction, RoutingAction, TimeReference, Timing, Trajectory,
        TrajectoryFollowingMode,
    },
    actions::wrappers::{CorePrivateAction, PrivateAction},
    basic::{Boolean, Double, OSString},
    enums::FollowingMode,
    geometry::shapes::{Polyline, Shape, Vertex},
    positions::{world::WorldPosition, Position},
};

/// Builder for trajectory definitions
///
/// Creates trajectories with polyline shapes and multiple vertices.
/// Trajectories define paths that entities can follow in a scenario.
///
/// # Example
///
/// ```rust
/// use openscenario_rs::builder::actions::trajectory::TrajectoryBuilder;
///
/// let trajectory = TrajectoryBuilder::new()
///     .name("curved_path")
///     .closed(false)
///     .polyline()
///         .add_vertex()
///             .time(0.0)
///             .world_position(0.0, 0.0, 0.0, 0.0)
///             .finish()?
///         .add_vertex()
///             .time(2.0)
///             .world_position(20.0, 5.0, 0.0, 0.5)
///             .finish()?
///         .finish()
///     .build()?;
/// # Ok::<(), openscenario_rs::builder::BuilderError>(())
/// ```
#[derive(Debug, Default)]
pub struct TrajectoryBuilder {
    name: Option<String>,
    closed: bool,
    shape: Option<Shape>,
}

impl TrajectoryBuilder {
    /// Create a new trajectory builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the trajectory name
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Set whether the trajectory is closed (forms a loop)
    pub fn closed(mut self, closed: bool) -> Self {
        self.closed = closed;
        self
    }

    /// Start building a polyline shape
    pub fn polyline(self) -> PolylineBuilder {
        PolylineBuilder::new(self)
    }

    /// Build the trajectory
    pub fn build(self) -> BuilderResult<Trajectory> {
        self.validate()?;

        Ok(Trajectory {
            name: OSString::literal(self.name.unwrap()),
            closed: Boolean::literal(self.closed),
            shape: self.shape.unwrap(),
        })
    }

    fn validate(&self) -> BuilderResult<()> {
        if self.name.is_none() {
            return Err(BuilderError::validation_error("Trajectory name is required"));
        }
        if self.shape.is_none() {
            return Err(BuilderError::validation_error(
                "Trajectory shape is required (use .polyline())",
            ));
        }
        Ok(())
    }
}

/// Builder for polyline shapes
///
/// Creates polyline shapes with multiple vertices. Each vertex has a time
/// and position along the trajectory.
///
/// # Example
///
/// ```rust
/// use openscenario_rs::builder::actions::trajectory::TrajectoryBuilder;
///
/// let trajectory = TrajectoryBuilder::new()
///     .name("straight_line")
///     .polyline()
///         .add_vertex()
///             .time(0.0)
///             .world_position(0.0, 0.0, 0.0, 0.0)
///             .finish()?
///         .add_vertex()
///             .time(5.0)
///             .world_position(50.0, 0.0, 0.0, 0.0)
///             .finish()?
///         .finish()
///     .build()?;
/// # Ok::<(), openscenario_rs::builder::BuilderError>(())
/// ```
#[derive(Debug)]
pub struct PolylineBuilder {
    parent: TrajectoryBuilder,
    vertices: Vec<Vertex>,
}

impl PolylineBuilder {
    fn new(parent: TrajectoryBuilder) -> Self {
        Self {
            parent,
            vertices: Vec::new(),
        }
    }

    /// Add a vertex to the polyline
    pub fn add_vertex(self) -> VertexBuilder {
        VertexBuilder::new(self)
    }

    /// Finish building the polyline and return to trajectory builder
    pub fn finish(mut self) -> TrajectoryBuilder {
        let polyline = Polyline {
            vertices: self.vertices,
        };
        self.parent.shape = Some(Shape {
            polyline: Some(polyline),
        });
        self.parent
    }

    fn add_vertex_internal(&mut self, vertex: Vertex) {
        self.vertices.push(vertex);
    }
}

/// Builder for trajectory vertices
///
/// Creates individual vertices with time and position information.
/// Vertices must be added in chronological order (increasing time values).
///
/// # Example
///
/// ```rust
/// use openscenario_rs::builder::actions::trajectory::TrajectoryBuilder;
///
/// let trajectory = TrajectoryBuilder::new()
///     .name("waypoint_path")
///     .polyline()
///         .add_vertex()
///             .time(0.0)
///             .world_position(0.0, 0.0, 0.0, 0.0)
///             .finish()?
///         .add_vertex()
///             .time(1.5)
///             .world_position(15.0, 0.0, 0.0, 0.0)
///             .finish()?
///         .add_vertex()
///             .time(3.0)
///             .world_position(30.0, 5.0, 0.0, 0.2)
///             .finish()?
///         .finish()
///     .build()?;
/// # Ok::<(), openscenario_rs::builder::BuilderError>(())
/// ```
#[derive(Debug)]
pub struct VertexBuilder {
    parent: PolylineBuilder,
    time: Option<f64>,
    position: Option<Position>,
}

impl VertexBuilder {
    fn new(parent: PolylineBuilder) -> Self {
        Self {
            parent,
            time: None,
            position: None,
        }
    }

    /// Set the time at this vertex (in seconds)
    pub fn time(mut self, time: f64) -> Self {
        self.time = Some(time);
        self
    }

    /// Set position using an existing Position object
    pub fn position(mut self, position: Position) -> Self {
        self.position = Some(position);
        self
    }

    /// Set position using world coordinates
    ///
    /// # Arguments
    /// * `x` - X coordinate in world frame
    /// * `y` - Y coordinate in world frame
    /// * `z` - Z coordinate (elevation)
    /// * `h` - Heading angle in radians
    pub fn world_position(mut self, x: f64, y: f64, z: f64, h: f64) -> Self {
        let world_pos = WorldPosition {
            x: Double::literal(x),
            y: Double::literal(y),
            z: Some(Double::literal(z)),
            h: Some(Double::literal(h)),
            p: None,
            r: None,
        };
        self.position = Some(Position {
            world_position: Some(world_pos),
            relative_world_position: None,
            road_position: None,
            relative_road_position: None,
            lane_position: None,
            relative_lane_position: None,
            trajectory_position: None,
            geographic_position: None,
            relative_object_position: None,
        });
        self
    }

    /// Finish building the vertex and return to polyline builder
    pub fn finish(mut self) -> BuilderResult<PolylineBuilder> {
        self.validate()?;

        let vertex = Vertex {
            time: Double::literal(self.time.unwrap()),
            position: self.position.unwrap(),
        };

        self.parent.add_vertex_internal(vertex);
        Ok(self.parent)
    }

    fn validate(&self) -> BuilderResult<()> {
        if self.time.is_none() {
            return Err(BuilderError::validation_error(
                "Vertex time is required",
            ));
        }
        if self.position.is_none() {
            return Err(BuilderError::validation_error(
                "Vertex position is required",
            ));
        }
        Ok(())
    }
}

/// Builder for follow trajectory actions
///
/// Creates actions that make entities follow predefined trajectories.
/// Requires a trajectory definition and following mode.
///
/// # Example
///
/// ```rust
/// use openscenario_rs::builder::actions::trajectory::{
///     TrajectoryBuilder, FollowTrajectoryActionBuilder
/// };
///
/// // First create a trajectory
/// let trajectory = TrajectoryBuilder::new()
///     .name("vehicle_path")
///     .closed(false)
///     .polyline()
///         .add_vertex()
///             .time(0.0)
///             .world_position(0.0, 0.0, 0.0, 0.0)
///             .finish()?
///         .add_vertex()
///             .time(10.0)
///             .world_position(100.0, 0.0, 0.0, 0.0)
///             .finish()?
///         .finish()
///     .build()?;
///
/// // Then create the action
/// let action = FollowTrajectoryActionBuilder::new()
///     .for_entity("ego_vehicle")
///     .with_trajectory(trajectory)
///     .following_mode_follow()
///     .build_action()?;
/// # Ok::<(), openscenario_rs::builder::BuilderError>(())
/// ```
#[derive(Debug, Default)]
pub struct FollowTrajectoryActionBuilder {
    entity_ref: Option<String>,
    trajectory: Option<Trajectory>,
    following_mode: Option<FollowingMode>,
    initial_distance_offset: Option<f64>,
}

impl FollowTrajectoryActionBuilder {
    /// Create a new follow trajectory action builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the entity that will follow the trajectory
    pub fn for_entity(mut self, entity_ref: &str) -> Self {
        self.entity_ref = Some(entity_ref.to_string());
        self
    }

    /// Set the trajectory to follow
    pub fn with_trajectory(mut self, trajectory: Trajectory) -> Self {
        self.trajectory = Some(trajectory);
        self
    }

    /// Set following mode to "follow" (entity follows trajectory timing)
    pub fn following_mode_follow(mut self) -> Self {
        self.following_mode = Some(FollowingMode::Follow);
        self
    }

    /// Set following mode to "position" (entity reaches positions at specified times)
    pub fn following_mode_position(mut self) -> Self {
        self.following_mode = Some(FollowingMode::Position);
        self
    }

    /// Set initial distance offset along trajectory (in meters)
    pub fn initial_distance_offset(mut self, offset: f64) -> Self {
        self.initial_distance_offset = Some(offset);
        self
    }
}

impl ActionBuilder for FollowTrajectoryActionBuilder {
    fn build_action(self) -> BuilderResult<PrivateAction> {
        self.validate()?;

        let follow_trajectory_action = FollowTrajectoryAction {
            trajectory: self.trajectory,
            catalog_reference: None,
            time_reference: TimeReference {
                timing: Timing {
                    domain_absolute_relative: OSString::literal("absolute".to_string()),
                    scale: Double::literal(1.0),
                    offset: Double::literal(0.0),
                },
            },
            trajectory_ref: None,
            trajectory_following_mode: TrajectoryFollowingMode {
                following_mode: self.following_mode.unwrap(),
            },
            initial_distance_offset: self
                .initial_distance_offset
                .map(|v| Double::literal(v)),
        };

        Ok(PrivateAction {
            action: CorePrivateAction::RoutingAction(RoutingAction {
                assign_route_action: None,
                follow_trajectory_action: Some(follow_trajectory_action),
                follow_route_action: None,
            }),
        })
    }

    fn validate(&self) -> BuilderResult<()> {
        if self.trajectory.is_none() {
            return Err(BuilderError::validation_error(
                "Trajectory is required for follow trajectory action",
            ));
        }
        if self.following_mode.is_none() {
            return Err(BuilderError::validation_error(
                "Following mode is required (use .following_mode_follow() or .following_mode_position())",
            ));
        }
        Ok(())
    }
}

impl ManeuverAction for FollowTrajectoryActionBuilder {
    fn entity_ref(&self) -> Option<&str> {
        self.entity_ref.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trajectory_builder_basic() {
        let trajectory = TrajectoryBuilder::new()
            .name("test_trajectory")
            .closed(false)
            .polyline()
            .add_vertex()
            .time(0.0)
            .world_position(0.0, 0.0, 0.0, 0.0)
            .finish()
            .unwrap()
            .add_vertex()
            .time(1.0)
            .world_position(10.0, 0.0, 0.0, 0.0)
            .finish()
            .unwrap()
            .finish()
            .build()
            .unwrap();

        assert_eq!(
            trajectory.name.as_literal(),
            Some(&"test_trajectory".to_string())
        );
        assert_eq!(trajectory.closed.as_literal(), Some(&false));
        assert!(trajectory.shape.polyline.is_some());
        assert_eq!(trajectory.shape.polyline.as_ref().unwrap().vertices.len(), 2);
    }

    #[test]
    fn test_trajectory_builder_closed() {
        let trajectory = TrajectoryBuilder::new()
            .name("loop_path")
            .closed(true)
            .polyline()
            .add_vertex()
            .time(0.0)
            .world_position(0.0, 0.0, 0.0, 0.0)
            .finish()
            .unwrap()
            .finish()
            .build()
            .unwrap();

        assert_eq!(trajectory.closed.as_literal(), Some(&true));
    }

    #[test]
    fn test_trajectory_validation_fails_without_name() {
        let result = TrajectoryBuilder::new()
            .polyline()
            .add_vertex()
            .time(0.0)
            .world_position(0.0, 0.0, 0.0, 0.0)
            .finish()
            .unwrap()
            .finish()
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Trajectory name is required"));
    }

    #[test]
    fn test_trajectory_validation_fails_without_shape() {
        let result = TrajectoryBuilder {
            name: Some("test".to_string()),
            closed: false,
            shape: None,
        }
        .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Trajectory shape is required"));
    }

    #[test]
    fn test_vertex_builder() {
        let trajectory = TrajectoryBuilder::new()
            .name("multi_vertex")
            .polyline()
            .add_vertex()
            .time(0.0)
            .world_position(0.0, 0.0, 0.0, 0.0)
            .finish()
            .unwrap()
            .add_vertex()
            .time(1.0)
            .world_position(10.0, 5.0, 0.0, 0.5)
            .finish()
            .unwrap()
            .add_vertex()
            .time(2.0)
            .world_position(20.0, 10.0, 0.0, 1.0)
            .finish()
            .unwrap()
            .finish()
            .build()
            .unwrap();

        let vertices = &trajectory.shape.polyline.as_ref().unwrap().vertices;
        assert_eq!(vertices.len(), 3);

        // Check first vertex
        assert_eq!(vertices[0].time.as_literal(), Some(&0.0));
        if let Some(ref pos) = vertices[0].position.world_position {
            assert_eq!(pos.x.as_literal(), Some(&0.0));
            assert_eq!(pos.y.as_literal(), Some(&0.0));
        } else {
            panic!("Expected WorldPosition");
        }

        // Check last vertex
        assert_eq!(vertices[2].time.as_literal(), Some(&2.0));
        if let Some(ref pos) = vertices[2].position.world_position {
            assert_eq!(pos.x.as_literal(), Some(&20.0));
            assert_eq!(pos.y.as_literal(), Some(&10.0));
        } else {
            panic!("Expected WorldPosition");
        }
    }

    #[test]
    fn test_vertex_validation_fails_without_time() {
        let result = TrajectoryBuilder::new()
            .name("test")
            .polyline()
            .add_vertex()
            .world_position(0.0, 0.0, 0.0, 0.0)
            .finish();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Vertex time is required"));
    }

    #[test]
    fn test_vertex_validation_fails_without_position() {
        let result = TrajectoryBuilder::new()
            .name("test")
            .polyline()
            .add_vertex()
            .time(0.0)
            .finish();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Vertex position is required"));
    }

    #[test]
    fn test_follow_trajectory_action_builder() {
        let trajectory = TrajectoryBuilder::new()
            .name("action_test_path")
            .closed(false)
            .polyline()
            .add_vertex()
            .time(0.0)
            .world_position(0.0, 0.0, 0.0, 0.0)
            .finish()
            .unwrap()
            .add_vertex()
            .time(5.0)
            .world_position(50.0, 0.0, 0.0, 0.0)
            .finish()
            .unwrap()
            .finish()
            .build()
            .unwrap();

        let action = FollowTrajectoryActionBuilder::new()
            .for_entity("ego")
            .with_trajectory(trajectory)
            .following_mode_follow()
            .build_action()
            .unwrap();

        // Verify action structure
        match action.action {
            CorePrivateAction::RoutingAction(ref routing) => {
                assert!(routing.follow_trajectory_action.is_some());
                let follow_action = routing.follow_trajectory_action.as_ref().unwrap();
                assert!(follow_action.trajectory.is_some());
                assert_eq!(
                    follow_action.trajectory_following_mode.following_mode,
                    FollowingMode::Follow
                );
            }
            _ => panic!("Expected RoutingAction"),
        }
    }

    #[test]
    fn test_follow_trajectory_action_with_position_mode() {
        let trajectory = TrajectoryBuilder::new()
            .name("position_mode_path")
            .polyline()
            .add_vertex()
            .time(0.0)
            .world_position(0.0, 0.0, 0.0, 0.0)
            .finish()
            .unwrap()
            .finish()
            .build()
            .unwrap();

        let action = FollowTrajectoryActionBuilder::new()
            .with_trajectory(trajectory)
            .following_mode_position()
            .build_action()
            .unwrap();

        match action.action {
            CorePrivateAction::RoutingAction(ref routing) => {
                let follow_action = routing.follow_trajectory_action.as_ref().unwrap();
                assert_eq!(
                    follow_action.trajectory_following_mode.following_mode,
                    FollowingMode::Position
                );
            }
            _ => panic!("Expected RoutingAction"),
        }
    }

    #[test]
    fn test_follow_trajectory_action_with_offset() {
        let trajectory = TrajectoryBuilder::new()
            .name("offset_path")
            .polyline()
            .add_vertex()
            .time(0.0)
            .world_position(0.0, 0.0, 0.0, 0.0)
            .finish()
            .unwrap()
            .finish()
            .build()
            .unwrap();

        let action = FollowTrajectoryActionBuilder::new()
            .with_trajectory(trajectory)
            .following_mode_follow()
            .initial_distance_offset(10.0)
            .build_action()
            .unwrap();

        match action.action {
            CorePrivateAction::RoutingAction(ref routing) => {
                let follow_action = routing.follow_trajectory_action.as_ref().unwrap();
                assert_eq!(
                    follow_action.initial_distance_offset.as_ref().unwrap().as_literal(),
                    Some(&10.0)
                );
            }
            _ => panic!("Expected RoutingAction"),
        }
    }

    #[test]
    fn test_follow_trajectory_validation_fails_without_trajectory() {
        let result = FollowTrajectoryActionBuilder::new()
            .following_mode_follow()
            .build_action();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Trajectory is required"));
    }

    #[test]
    fn test_follow_trajectory_validation_fails_without_mode() {
        let trajectory = TrajectoryBuilder::new()
            .name("test")
            .polyline()
            .add_vertex()
            .time(0.0)
            .world_position(0.0, 0.0, 0.0, 0.0)
            .finish()
            .unwrap()
            .finish()
            .build()
            .unwrap();

        let result = FollowTrajectoryActionBuilder::new()
            .with_trajectory(trajectory)
            .build_action();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Following mode is required"));
    }

    #[test]
    fn test_maneuver_action_trait() {
        let builder = FollowTrajectoryActionBuilder::new().for_entity("test_entity");

        assert_eq!(builder.entity_ref(), Some("test_entity"));
    }
}
