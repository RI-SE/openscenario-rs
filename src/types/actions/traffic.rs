//! Phase 4C: Traffic Management Actions Implementation
//!
//! This file contains:
//! - Traffic source and sink actions for dynamic traffic generation
//! - Traffic swarm actions for crowd simulation around entities
//! - Traffic area actions for regional traffic density control
//! - Traffic signal control actions for intersection management
//! - Background traffic definition and distribution specifications
//!
use crate::types::basic::{Boolean, Double, OSString, UnsignedInt};
use crate::types::positions::Position;
use serde::{Deserialize, Serialize};

// PHASE 4C: Core Traffic Management Actions

/// Traffic source action for traffic generation with rate and position
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficSourceAction {
    #[serde(rename = "@rate")]
    pub rate: Double,
    #[serde(rename = "@velocity")]
    pub velocity: Option<Double>,
    #[serde(rename = "Position")]
    pub position: Position,
    #[serde(rename = "TrafficDefinition")]
    pub traffic_definition: TrafficDefinition,
}

/// Traffic sink action for traffic removal with radius control
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficSinkAction {
    #[serde(rename = "@rate")]
    pub rate: Double,
    #[serde(rename = "@radius")]
    pub radius: Double,
    #[serde(rename = "Position")]
    pub position: Position,
    #[serde(rename = "TrafficDefinition")]
    pub traffic_definition: Option<TrafficDefinition>,
}

/// Traffic swarm action for swarm behavior around central object
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficSwarmAction {
    #[serde(rename = "@innerRadius")]
    pub inner_radius: Double,
    #[serde(rename = "@numberOfVehicles")]
    pub number_of_vehicles: UnsignedInt,
    #[serde(rename = "@offset")]
    pub offset: Double,
    #[serde(rename = "@semiMajorAxis")]
    pub semi_major_axis: Double,
    #[serde(rename = "@semiMinorAxis")]
    pub semi_minor_axis: Double,
    #[serde(rename = "@velocity", skip_serializing_if = "Option::is_none")]
    pub velocity: Option<Double>,
    #[serde(rename = "CentralObject")]
    pub central_object: CentralSwarmObject,
    #[serde(rename = "TrafficDefinition", skip_serializing_if = "Option::is_none")]
    pub traffic_definition: Option<TrafficDefinition>,
}

/// Traffic area action for area-based traffic management
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficAreaAction {
    #[serde(rename = "TrafficArea")]
    pub traffic_area: TrafficArea,
    #[serde(rename = "TrafficDefinition")]
    pub traffic_definition: TrafficDefinition,
}

/// Traffic signal action wrapper for all traffic signal operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficSignalAction {
    #[serde(flatten)]
    pub signal_action_choice: TrafficSignalActionChoice,
}

/// Traffic signal action choice - controller or state action
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum TrafficSignalActionChoice {
    TrafficSignalControllerAction(TrafficSignalControllerAction),
    TrafficSignalStateAction(TrafficSignalStateAction),
}

/// Traffic signal state action for individual signal state control
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficSignalStateAction {
    #[serde(rename = "@name")]
    pub name: OSString,
    #[serde(rename = "@state")]
    pub state: OSString,
}

/// Traffic signal controller action for signal controller management
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficSignalControllerAction {
    #[serde(rename = "@trafficSignalControllerRef")]
    pub traffic_signal_controller_ref: OSString,
    #[serde(rename = "@phase")]
    pub phase_ref: OSString,
}

/// Traffic signal controller definition with phases and timing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficSignalController {
    #[serde(rename = "@name")]
    pub name: OSString,
    #[serde(rename = "@delay")]
    pub delay: Option<Double>,
    #[serde(rename = "@reference")]
    pub reference: Option<OSString>,
    #[serde(rename = "Phase", default)]
    pub phases: Vec<Phase>,
}

/// Phase definition for traffic signal controller
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Phase {
    #[serde(rename = "@name")]
    pub name: OSString,
    #[serde(rename = "@duration")]
    pub duration: Double,
    #[serde(rename = "TrafficSignalState", default)]
    pub traffic_signal_states: Vec<TrafficSignalState>,
    #[serde(rename = "TrafficSignalGroupState")]
    pub traffic_signal_group_state: Option<TrafficSignalGroupState>,
}

/// Traffic signal state for individual signal control
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficSignalState {
    #[serde(rename = "@trafficSignalId")]
    pub traffic_signal_id: OSString,
    #[serde(rename = "@state")]
    pub state: OSString,
}

/// Traffic signal group state for signal group control
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficSignalGroupState {
    #[serde(rename = "@state")]
    pub state: OSString,
}

/// Traffic stop action for traffic stop enable/disable
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficStopAction {
    #[serde(rename = "@enable")]
    pub enable: Boolean,
}

// PHASE 4C: Supporting Types for Traffic Management

/// Traffic definition for vehicle category and controller distribution
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficDefinition {
    #[serde(rename = "VehicleCategoryDistribution")]
    pub vehicle_category_distribution: Option<VehicleCategoryDistribution>,
    #[serde(rename = "ControllerDistribution")]
    pub controller_distribution: Option<ControllerDistribution>,
}

/// Vehicle category distribution for traffic composition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VehicleCategoryDistribution {
    #[serde(rename = "VehicleCategoryDistributionEntry", default)]
    pub entries: Vec<VehicleCategoryDistributionEntry>,
}

/// Vehicle category distribution entry
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VehicleCategoryDistributionEntry {
    #[serde(rename = "@category")]
    pub category: VehicleCategory,
    #[serde(rename = "@weight")]
    pub weight: Double,
}

/// Vehicle category enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VehicleCategory {
    #[serde(rename = "car")]
    Car,
    #[serde(rename = "van")]
    Van,
    #[serde(rename = "truck")]
    Truck,
    #[serde(rename = "bus")]
    Bus,
    #[serde(rename = "motorbike")]
    Motorbike,
    #[serde(rename = "bicycle")]
    Bicycle,
    #[serde(rename = "trailer")]
    Trailer,
    #[serde(rename = "semitrailer")]
    Semitrailer,
}

/// Controller distribution for traffic behavior
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ControllerDistribution {
    #[serde(rename = "ControllerDistributionEntry", default)]
    pub entries: Vec<ControllerDistributionEntry>,
}

/// Controller distribution entry
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ControllerDistributionEntry {
    #[serde(rename = "@weight")]
    pub weight: Double,
    #[serde(rename = "Controller")]
    pub controller: OSString,
}

/// Central swarm object specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CentralSwarmObject {
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
}

/// Traffic area definition with vertices
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficArea {
    #[serde(rename = "Vertex", default)]
    pub vertices: Vec<TrafficAreaVertex>,
}

/// Simple vertex for traffic area definition (x, y, z coordinates)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficAreaVertex {
    #[serde(rename = "@x")]
    pub x: Double,
    #[serde(rename = "@y")]
    pub y: Double,
    #[serde(rename = "@z")]
    pub z: Double,
}

// PHASE 4C: Default implementations

impl Default for TrafficSourceAction {
    fn default() -> Self {
        Self {
            rate: Double::literal(10.0),           // 10 vehicles per minute
            velocity: Some(Double::literal(50.0)), // 50 km/h default velocity
            position: Position::default(),
            traffic_definition: TrafficDefinition::default(),
        }
    }
}

impl Default for TrafficSinkAction {
    fn default() -> Self {
        Self {
            rate: Double::literal(10.0),   // 10 vehicles per minute
            radius: Double::literal(50.0), // 50 meter radius
            position: Position::default(),
            traffic_definition: None,
        }
    }
}

impl Default for TrafficSwarmAction {
    fn default() -> Self {
        Self {
            inner_radius: Double::literal(10.0),
            number_of_vehicles: UnsignedInt::literal(20),
            offset: Double::literal(0.0),
            semi_major_axis: Double::literal(100.0),
            semi_minor_axis: Double::literal(50.0),
            velocity: None,
            central_object: CentralSwarmObject::default(),
            traffic_definition: Some(TrafficDefinition::default()),
        }
    }
}

impl Default for TrafficAreaAction {
    fn default() -> Self {
        Self {
            traffic_area: TrafficArea::default(),
            traffic_definition: TrafficDefinition::default(),
        }
    }
}

impl Default for TrafficSignalAction {
    fn default() -> Self {
        Self {
            signal_action_choice: TrafficSignalActionChoice::TrafficSignalStateAction(
                TrafficSignalStateAction::default(),
            ),
        }
    }
}

impl Default for TrafficSignalStateAction {
    fn default() -> Self {
        Self {
            name: OSString::literal("DefaultSignal".to_string()),
            state: OSString::literal("green".to_string()),
        }
    }
}

impl Default for TrafficSignalControllerAction {
    fn default() -> Self {
        Self {
            traffic_signal_controller_ref: OSString::literal("DefaultController".to_string()),
            phase_ref: OSString::literal("Phase1".to_string()),
        }
    }
}

impl Default for TrafficSignalController {
    fn default() -> Self {
        Self {
            name: OSString::literal("DefaultController".to_string()),
            delay: None,
            reference: None,
            phases: Vec::new(),
        }
    }
}

impl Default for Phase {
    fn default() -> Self {
        Self {
            name: OSString::literal("DefaultPhase".to_string()),
            duration: Double::literal(30.0),
            traffic_signal_states: Vec::new(),
            traffic_signal_group_state: None,
        }
    }
}

impl Default for TrafficSignalState {
    fn default() -> Self {
        Self {
            traffic_signal_id: OSString::literal("signal_1".to_string()),
            state: OSString::literal("green".to_string()),
        }
    }
}

impl Default for TrafficSignalGroupState {
    fn default() -> Self {
        Self {
            state: OSString::literal("green".to_string()),
        }
    }
}

impl Default for TrafficStopAction {
    fn default() -> Self {
        Self {
            enable: Boolean::literal(true),
        }
    }
}

impl Default for TrafficDefinition {
    fn default() -> Self {
        Self {
            vehicle_category_distribution: Some(VehicleCategoryDistribution::default()),
            controller_distribution: None,
        }
    }
}

impl Default for VehicleCategoryDistribution {
    fn default() -> Self {
        Self {
            entries: vec![
                VehicleCategoryDistributionEntry {
                    category: VehicleCategory::Car,
                    weight: Double::literal(0.7), // 70% cars
                },
                VehicleCategoryDistributionEntry {
                    category: VehicleCategory::Truck,
                    weight: Double::literal(0.2), // 20% trucks
                },
                VehicleCategoryDistributionEntry {
                    category: VehicleCategory::Van,
                    weight: Double::literal(0.1), // 10% vans
                },
            ],
        }
    }
}

impl Default for ControllerDistribution {
    fn default() -> Self {
        Self {
            entries: vec![ControllerDistributionEntry {
                weight: Double::literal(1.0),
                controller: OSString::literal("DefaultTrafficController".to_string()),
            }],
        }
    }
}

impl Default for CentralSwarmObject {
    fn default() -> Self {
        Self {
            entity_ref: OSString::literal("SwarmCenter".to_string()),
        }
    }
}

impl Default for TrafficArea {
    fn default() -> Self {
        Self {
            vertices: vec![
                TrafficAreaVertex::new(0.0, 0.0, 0.0),
                TrafficAreaVertex::new(100.0, 0.0, 0.0),
                TrafficAreaVertex::new(100.0, 100.0, 0.0),
                TrafficAreaVertex::new(0.0, 100.0, 0.0),
            ],
        }
    }
}

impl Default for TrafficAreaVertex {
    fn default() -> Self {
        Self {
            x: Double::literal(0.0),
            y: Double::literal(0.0),
            z: Double::literal(0.0),
        }
    }
}

impl Default for VehicleCategory {
    fn default() -> Self {
        VehicleCategory::Car
    }
}

// PHASE 4C: Helper implementations

impl TrafficSourceAction {
    /// Create traffic source with rate and position
    pub fn new(rate: f64, position: Position, traffic_definition: TrafficDefinition) -> Self {
        Self {
            rate: Double::literal(rate),
            velocity: None,
            position,
            traffic_definition,
        }
    }

    /// Create traffic source with velocity
    pub fn with_velocity(
        rate: f64,
        velocity: f64,
        position: Position,
        traffic_definition: TrafficDefinition,
    ) -> Self {
        Self {
            rate: Double::literal(rate),
            velocity: Some(Double::literal(velocity)),
            position,
            traffic_definition,
        }
    }
}

impl TrafficSinkAction {
    /// Create traffic sink with rate, radius and position
    pub fn new(rate: f64, radius: f64, position: Position) -> Self {
        Self {
            rate: Double::literal(rate),
            radius: Double::literal(radius),
            position,
            traffic_definition: None,
        }
    }

    /// Create traffic sink with traffic definition
    pub fn with_traffic_definition(
        rate: f64,
        radius: f64,
        position: Position,
        traffic_definition: TrafficDefinition,
    ) -> Self {
        Self {
            rate: Double::literal(rate),
            radius: Double::literal(radius),
            position,
            traffic_definition: Some(traffic_definition),
        }
    }
}

impl TrafficSwarmAction {
    /// Create traffic swarm around central object
    pub fn new(
        central_object: impl Into<String>,
        semi_major_axis: f64,
        semi_minor_axis: f64,
        number_of_vehicles: u32,
    ) -> Self {
        Self {
            inner_radius: Double::literal(0.0),
            number_of_vehicles: UnsignedInt::literal(number_of_vehicles),
            offset: Double::literal(0.0),
            semi_major_axis: Double::literal(semi_major_axis),
            semi_minor_axis: Double::literal(semi_minor_axis),
            velocity: None,
            central_object: CentralSwarmObject {
                entity_ref: OSString::literal(central_object.into()),
            },
            traffic_definition: None,
        }
    }

    /// Set inner radius for the swarm
    pub fn with_inner_radius(mut self, radius: f64) -> Self {
        self.inner_radius = Double::literal(radius);
        self
    }

    /// Set offset for the swarm
    pub fn with_offset(mut self, offset: f64) -> Self {
        self.offset = Double::literal(offset);
        self
    }

    /// Set velocity for the swarm (deprecated)
    pub fn with_velocity(mut self, velocity: f64) -> Self {
        self.velocity = Some(Double::literal(velocity));
        self
    }

    /// Set traffic definition for the swarm
    pub fn with_traffic_definition(mut self, traffic_definition: TrafficDefinition) -> Self {
        self.traffic_definition = Some(traffic_definition);
        self
    }

    /// Set central swarm object entity reference
    pub fn with_central_swarm_object(mut self, entity_ref: impl Into<String>) -> Self {
        self.central_object = CentralSwarmObject {
            entity_ref: OSString::literal(entity_ref.into()),
        };
        self
    }
}

impl TrafficAreaAction {
    /// Create traffic area with vertices
    pub fn new(vertices: Vec<TrafficAreaVertex>, traffic_definition: TrafficDefinition) -> Self {
        Self {
            traffic_area: TrafficArea { vertices },
            traffic_definition,
        }
    }
}

impl TrafficSignalAction {
    /// Create signal state action
    pub fn state_action(name: String, state: String) -> Self {
        Self {
            signal_action_choice: TrafficSignalActionChoice::TrafficSignalStateAction(
                TrafficSignalStateAction {
                    name: OSString::literal(name),
                    state: OSString::literal(state),
                },
            ),
        }
    }

    /// Create signal controller action
    pub fn controller_action(controller_ref: String, phase_ref: String) -> Self {
        Self {
            signal_action_choice: TrafficSignalActionChoice::TrafficSignalControllerAction(
                TrafficSignalControllerAction {
                    traffic_signal_controller_ref: OSString::literal(controller_ref),
                    phase_ref: OSString::literal(phase_ref),
                },
            ),
        }
    }
}

impl TrafficSignalController {
    /// Create new traffic signal controller
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: OSString::literal(name.into()),
            delay: None,
            reference: None,
            phases: Vec::new(),
        }
    }

    /// Set delay for the controller
    pub fn with_delay(mut self, delay: f64) -> Self {
        self.delay = Some(Double::literal(delay));
        self
    }

    /// Set reference for the controller
    pub fn with_reference(mut self, reference: impl Into<String>) -> Self {
        self.reference = Some(OSString::literal(reference.into()));
        self
    }

    /// Add a phase to the controller
    pub fn add_phase(mut self, phase: Phase) -> Self {
        self.phases.push(phase);
        self
    }

    /// Add multiple phases to the controller
    pub fn with_phases(mut self, phases: Vec<Phase>) -> Self {
        self.phases = phases;
        self
    }
}

impl Phase {
    /// Create new phase
    pub fn new(name: impl Into<String>, duration: f64) -> Self {
        Self {
            name: OSString::literal(name.into()),
            duration: Double::literal(duration),
            traffic_signal_states: Vec::new(),
            traffic_signal_group_state: None,
        }
    }

    /// Add signal state to the phase
    pub fn add_signal_state(
        mut self,
        signal_id: impl Into<String>,
        state: impl Into<String>,
    ) -> Self {
        self.traffic_signal_states.push(TrafficSignalState {
            traffic_signal_id: OSString::literal(signal_id.into()),
            state: OSString::literal(state.into()),
        });
        self
    }

    /// Set traffic signal group state
    pub fn with_group_state(mut self, state: impl Into<String>) -> Self {
        self.traffic_signal_group_state = Some(TrafficSignalGroupState {
            state: OSString::literal(state.into()),
        });
        self
    }

    /// Add multiple signal states
    pub fn with_signal_states(mut self, states: Vec<(String, String)>) -> Self {
        for (signal_id, state) in states {
            self.traffic_signal_states.push(TrafficSignalState {
                traffic_signal_id: OSString::literal(signal_id),
                state: OSString::literal(state),
            });
        }
        self
    }
}

impl TrafficSignalState {
    /// Create new traffic signal state
    pub fn new(signal_id: impl Into<String>, state: impl Into<String>) -> Self {
        Self {
            traffic_signal_id: OSString::literal(signal_id.into()),
            state: OSString::literal(state.into()),
        }
    }
}

impl TrafficSignalGroupState {
    /// Create new traffic signal group state
    pub fn new(state: impl Into<String>) -> Self {
        Self {
            state: OSString::literal(state.into()),
        }
    }
}

impl TrafficSignalStateAction {
    /// Create new traffic signal state action
    pub fn new(name: impl Into<String>, state: impl Into<String>) -> Self {
        Self {
            name: OSString::literal(name.into()),
            state: OSString::literal(state.into()),
        }
    }
}

impl TrafficSignalControllerAction {
    /// Create new traffic signal controller action
    pub fn new(controller_ref: impl Into<String>, phase_ref: impl Into<String>) -> Self {
        Self {
            traffic_signal_controller_ref: OSString::literal(controller_ref.into()),
            phase_ref: OSString::literal(phase_ref.into()),
        }
    }
}

impl TrafficStopAction {
    /// Enable traffic stop
    pub fn enable() -> Self {
        Self {
            enable: Boolean::literal(true),
        }
    }

    /// Disable traffic stop
    pub fn disable() -> Self {
        Self {
            enable: Boolean::literal(false),
        }
    }
}

impl TrafficDefinition {
    /// Create traffic definition with vehicle categories only
    pub fn with_vehicles(distribution: VehicleCategoryDistribution) -> Self {
        Self {
            vehicle_category_distribution: Some(distribution),
            controller_distribution: None,
        }
    }

    /// Create traffic definition with controllers only
    pub fn with_controllers(distribution: ControllerDistribution) -> Self {
        Self {
            vehicle_category_distribution: None,
            controller_distribution: Some(distribution),
        }
    }

    /// Create traffic definition with both vehicles and controllers
    pub fn with_both(
        vehicles: VehicleCategoryDistribution,
        controllers: ControllerDistribution,
    ) -> Self {
        Self {
            vehicle_category_distribution: Some(vehicles),
            controller_distribution: Some(controllers),
        }
    }
}

impl VehicleCategoryDistribution {
    /// Create distribution with single category
    pub fn single_category(category: VehicleCategory, weight: f64) -> Self {
        Self {
            entries: vec![VehicleCategoryDistributionEntry {
                category,
                weight: Double::literal(weight),
            }],
        }
    }

    /// Create distribution for mixed traffic (cars, trucks, vans)
    pub fn mixed_traffic() -> Self {
        Self::default()
    }

    /// Create distribution for urban traffic (mostly cars)
    pub fn urban_traffic() -> Self {
        Self {
            entries: vec![
                VehicleCategoryDistributionEntry {
                    category: VehicleCategory::Car,
                    weight: Double::literal(0.85),
                },
                VehicleCategoryDistributionEntry {
                    category: VehicleCategory::Bus,
                    weight: Double::literal(0.1),
                },
                VehicleCategoryDistributionEntry {
                    category: VehicleCategory::Van,
                    weight: Double::literal(0.05),
                },
            ],
        }
    }
}

impl ControllerDistribution {
    /// Create distribution with single controller
    pub fn single_controller(controller: String, weight: f64) -> Self {
        Self {
            entries: vec![ControllerDistributionEntry {
                weight: Double::literal(weight),
                controller: OSString::literal(controller),
            }],
        }
    }
}

impl CentralSwarmObject {
    /// Create new central swarm object
    pub fn new(entity_ref: impl Into<String>) -> Self {
        Self {
            entity_ref: OSString::literal(entity_ref.into()),
        }
    }
}

impl TrafficAreaVertex {
    /// Create new traffic area vertex
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x: Double::literal(x),
            y: Double::literal(y),
            z: Double::literal(z),
        }
    }

    /// Get x coordinate value
    pub fn x(&self) -> Option<&f64> {
        self.x.as_literal()
    }

    /// Get y coordinate value
    pub fn y(&self) -> Option<&f64> {
        self.y.as_literal()
    }

    /// Get z coordinate value
    pub fn z(&self) -> Option<&f64> {
        self.z.as_literal()
    }
}

impl TrafficArea {
    /// Create rectangular traffic area
    pub fn rectangle(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            vertices: vec![
                TrafficAreaVertex::new(x, y, 0.0),
                TrafficAreaVertex::new(x + width, y, 0.0),
                TrafficAreaVertex::new(x + width, y + height, 0.0),
                TrafficAreaVertex::new(x, y + height, 0.0),
            ],
        }
    }

    /// Create circular traffic area (approximated with polygon)
    pub fn circle(center_x: f64, center_y: f64, radius: f64, segments: u32) -> Self {
        let mut vertices = Vec::new();
        let segment_angle = 2.0 * std::f64::consts::PI / segments as f64;

        for i in 0..segments {
            let angle = i as f64 * segment_angle;
            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();
            vertices.push(TrafficAreaVertex::new(x, y, 0.0));
        }

        Self { vertices }
    }
}

// PHASE 4C: Unit tests for all traffic actions
#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::positions::Position;

    #[test]
    fn test_traffic_source_action_creation() {
        let source =
            TrafficSourceAction::new(15.0, Position::default(), TrafficDefinition::default());

        assert_eq!(source.rate.as_literal(), Some(&15.0));
        assert!(source.velocity.is_none());
    }

    #[test]
    fn test_traffic_source_with_velocity() {
        let source = TrafficSourceAction::with_velocity(
            20.0,
            60.0,
            Position::default(),
            TrafficDefinition::default(),
        );

        assert_eq!(source.rate.as_literal(), Some(&20.0));
        assert_eq!(
            source.velocity.as_ref().and_then(|v| v.as_literal()),
            Some(&60.0)
        );
    }

    #[test]
    fn test_traffic_sink_action_creation() {
        let sink = TrafficSinkAction::new(10.0, 30.0, Position::default());

        assert_eq!(sink.rate.as_literal(), Some(&10.0));
        assert_eq!(sink.radius.as_literal(), Some(&30.0));
        assert!(sink.traffic_definition.is_none());
    }

    #[test]
    fn test_traffic_sink_with_definition() {
        let sink = TrafficSinkAction::with_traffic_definition(
            12.0,
            40.0,
            Position::default(),
            TrafficDefinition::default(),
        );

        assert_eq!(sink.rate.as_literal(), Some(&12.0));
        assert_eq!(sink.radius.as_literal(), Some(&40.0));
        assert!(sink.traffic_definition.is_some());
    }

    #[test]
    fn test_traffic_swarm_action_creation() {
        let swarm = TrafficSwarmAction::new("LeadVehicle", 100.0, 50.0, 15)
            .with_inner_radius(5.0)
            .with_traffic_definition(TrafficDefinition::default());

        assert_eq!(
            swarm.central_object.entity_ref.as_literal(),
            Some(&"LeadVehicle".to_string())
        );
        assert_eq!(swarm.inner_radius.as_literal(), Some(&5.0));
        assert_eq!(swarm.semi_major_axis.as_literal(), Some(&100.0));
        assert_eq!(swarm.semi_minor_axis.as_literal(), Some(&50.0));
        assert_eq!(swarm.number_of_vehicles.as_literal(), Some(&15));
    }

    #[test]
    fn test_traffic_area_action_creation() {
        let vertices = vec![
            TrafficAreaVertex::new(0.0, 0.0, 0.0),
            TrafficAreaVertex::new(50.0, 0.0, 0.0),
            TrafficAreaVertex::new(50.0, 50.0, 0.0),
            TrafficAreaVertex::new(0.0, 50.0, 0.0),
        ];

        let area = TrafficAreaAction::new(vertices.clone(), TrafficDefinition::default());

        assert_eq!(area.traffic_area.vertices.len(), 4);
        assert_eq!(area.traffic_area.vertices[0].x(), Some(&0.0));
        assert_eq!(area.traffic_area.vertices[1].x(), Some(&50.0));
    }

    #[test]
    fn test_traffic_signal_state_action() {
        let signal =
            TrafficSignalAction::state_action("Intersection1".to_string(), "red".to_string());

        if let TrafficSignalActionChoice::TrafficSignalStateAction(state_action) =
            signal.signal_action_choice
        {
            assert_eq!(
                state_action.name.as_literal(),
                Some(&"Intersection1".to_string())
            );
            assert_eq!(state_action.state.as_literal(), Some(&"red".to_string()));
        } else {
            panic!("Expected TrafficSignalStateAction");
        }
    }

    #[test]
    fn test_traffic_signal_controller_action() {
        let signal =
            TrafficSignalAction::controller_action("Controller1".to_string(), "Phase2".to_string());

        if let TrafficSignalActionChoice::TrafficSignalControllerAction(controller_action) =
            signal.signal_action_choice
        {
            assert_eq!(
                controller_action.traffic_signal_controller_ref.as_literal(),
                Some(&"Controller1".to_string())
            );
            assert_eq!(
                controller_action.phase_ref.as_literal(),
                Some(&"Phase2".to_string())
            );
        } else {
            panic!("Expected TrafficSignalControllerAction");
        }
    }

    #[test]
    fn test_traffic_stop_action() {
        let enable = TrafficStopAction::enable();
        assert_eq!(enable.enable.as_literal(), Some(&true));

        let disable = TrafficStopAction::disable();
        assert_eq!(disable.enable.as_literal(), Some(&false));
    }

    #[test]
    fn test_vehicle_category_distribution() {
        let mixed = VehicleCategoryDistribution::mixed_traffic();
        assert_eq!(mixed.entries.len(), 3);
        assert!(mixed
            .entries
            .iter()
            .any(|e| matches!(e.category, VehicleCategory::Car)));

        let urban = VehicleCategoryDistribution::urban_traffic();
        assert_eq!(urban.entries.len(), 3);
        assert_eq!(urban.entries[0].weight.as_literal().unwrap(), &0.85); // Mostly cars
    }

    #[test]
    fn test_traffic_definition_creation() {
        let vehicles = VehicleCategoryDistribution::urban_traffic();
        let controllers = ControllerDistribution::single_controller("AI1".to_string(), 1.0);

        let definition = TrafficDefinition::with_both(vehicles, controllers);

        assert!(definition.vehicle_category_distribution.is_some());
        assert!(definition.controller_distribution.is_some());
    }

    #[test]
    fn test_traffic_area_shapes() {
        let rect = TrafficArea::rectangle(10.0, 20.0, 30.0, 40.0);
        assert_eq!(rect.vertices.len(), 4);
        assert_eq!(rect.vertices[0].x(), Some(&10.0));
        assert_eq!(rect.vertices[0].y(), Some(&20.0));
        assert_eq!(rect.vertices[2].x(), Some(&40.0)); // 10 + 30
        assert_eq!(rect.vertices[2].y(), Some(&60.0)); // 20 + 40

        let circle = TrafficArea::circle(0.0, 0.0, 10.0, 8);
        assert_eq!(circle.vertices.len(), 8);

        // First vertex should be at (10, 0) for angle 0
        assert!((circle.vertices[0].x().unwrap() - 10.0).abs() < 1e-10);
        assert!((circle.vertices[0].y().unwrap() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_vehicle_categories() {
        let car = VehicleCategory::Car;
        let truck = VehicleCategory::Truck;
        let bike = VehicleCategory::Bicycle;

        // Test that enum variants exist and can be used
        let entry1 = VehicleCategoryDistributionEntry {
            category: car,
            weight: Double::literal(0.6),
        };
        let entry2 = VehicleCategoryDistributionEntry {
            category: truck,
            weight: Double::literal(0.3),
        };
        let entry3 = VehicleCategoryDistributionEntry {
            category: bike,
            weight: Double::literal(0.1),
        };

        assert_eq!(entry1.weight.as_literal().unwrap(), &0.6);
        assert_eq!(entry2.weight.as_literal().unwrap(), &0.3);
        assert_eq!(entry3.weight.as_literal().unwrap(), &0.1);
    }

    #[test]
    fn test_traffic_action_defaults() {
        let source = TrafficSourceAction::default();
        assert_eq!(source.rate.as_literal(), Some(&10.0));
        assert_eq!(
            source.velocity.as_ref().and_then(|v| v.as_literal()),
            Some(&50.0)
        );

        let sink = TrafficSinkAction::default();
        assert_eq!(sink.rate.as_literal(), Some(&10.0));
        assert_eq!(sink.radius.as_literal(), Some(&50.0));

        let swarm = TrafficSwarmAction::default();
        assert_eq!(swarm.number_of_vehicles.as_literal(), Some(&20));
        assert_eq!(swarm.inner_radius.as_literal(), Some(&10.0));
        assert_eq!(swarm.semi_major_axis.as_literal(), Some(&100.0));
        assert_eq!(swarm.semi_minor_axis.as_literal(), Some(&50.0));

        let stop = TrafficStopAction::default();
        assert_eq!(stop.enable.as_literal(), Some(&true));
    }

    // TRAFFIC SIGNAL SYSTEM TESTS

    #[test]
    fn test_traffic_signal_controller_creation() {
        let controller = TrafficSignalController::new("intersection_1")
            .with_delay(1.0)
            .with_reference("ref_1")
            .add_phase(
                Phase::new("green_ns", 30.0)
                    .add_signal_state("signal_1", "green")
                    .add_signal_state("signal_2", "red"),
            );

        assert_eq!(
            controller.name.as_literal(),
            Some(&"intersection_1".to_string())
        );
        assert_eq!(controller.delay.as_ref().unwrap().as_literal(), Some(&1.0));
        assert_eq!(
            controller.reference.as_ref().unwrap().as_literal(),
            Some(&"ref_1".to_string())
        );
        assert_eq!(controller.phases.len(), 1);
        assert_eq!(controller.phases[0].traffic_signal_states.len(), 2);
    }

    #[test]
    fn test_phase_creation_and_manipulation() {
        let phase = Phase::new("green_phase", 45.0)
            .add_signal_state("north_signal", "green")
            .add_signal_state("south_signal", "green")
            .add_signal_state("east_signal", "red")
            .add_signal_state("west_signal", "red")
            .with_group_state("active");

        assert_eq!(phase.name.as_literal(), Some(&"green_phase".to_string()));
        assert_eq!(phase.duration.as_literal(), Some(&45.0));
        assert_eq!(phase.traffic_signal_states.len(), 4);
        assert!(phase.traffic_signal_group_state.is_some());
        assert_eq!(
            phase.traffic_signal_group_state.unwrap().state.as_literal(),
            Some(&"active".to_string())
        );
    }

    #[test]
    fn test_traffic_signal_state_creation() {
        let state = TrafficSignalState::new("signal_123", "yellow");

        assert_eq!(
            state.traffic_signal_id.as_literal(),
            Some(&"signal_123".to_string())
        );
        assert_eq!(state.state.as_literal(), Some(&"yellow".to_string()));
    }

    #[test]
    fn test_traffic_signal_group_state_creation() {
        let group_state = TrafficSignalGroupState::new("flashing");

        assert_eq!(
            group_state.state.as_literal(),
            Some(&"flashing".to_string())
        );
    }

    #[test]
    fn test_traffic_signal_state_action_creation() {
        let action = TrafficSignalStateAction::new("main_signal", "red");

        assert_eq!(action.name.as_literal(), Some(&"main_signal".to_string()));
        assert_eq!(action.state.as_literal(), Some(&"red".to_string()));
    }

    #[test]
    fn test_traffic_signal_controller_action_creation() {
        let action = TrafficSignalControllerAction::new("controller_1", "phase_2");

        assert_eq!(
            action.traffic_signal_controller_ref.as_literal(),
            Some(&"controller_1".to_string())
        );
        assert_eq!(action.phase_ref.as_literal(), Some(&"phase_2".to_string()));
    }

    #[test]
    fn test_complex_traffic_signal_controller() {
        let controller = TrafficSignalController::new("complex_intersection")
            .with_delay(2.5)
            .add_phase(
                Phase::new("north_south_green", 60.0)
                    .add_signal_state("ns_signal", "green")
                    .add_signal_state("ew_signal", "red")
                    .with_group_state("ns_active"),
            )
            .add_phase(
                Phase::new("north_south_yellow", 5.0)
                    .add_signal_state("ns_signal", "yellow")
                    .add_signal_state("ew_signal", "red"),
            )
            .add_phase(
                Phase::new("east_west_green", 45.0)
                    .add_signal_state("ns_signal", "red")
                    .add_signal_state("ew_signal", "green")
                    .with_group_state("ew_active"),
            )
            .add_phase(
                Phase::new("east_west_yellow", 5.0)
                    .add_signal_state("ns_signal", "red")
                    .add_signal_state("ew_signal", "yellow"),
            );

        assert_eq!(controller.phases.len(), 4);
        assert_eq!(controller.phases[0].duration.as_literal(), Some(&60.0));
        assert_eq!(controller.phases[1].duration.as_literal(), Some(&5.0));
        assert_eq!(controller.phases[2].duration.as_literal(), Some(&45.0));
        assert_eq!(controller.phases[3].duration.as_literal(), Some(&5.0));

        // Check first phase details
        let first_phase = &controller.phases[0];
        assert_eq!(first_phase.traffic_signal_states.len(), 2);
        assert!(first_phase.traffic_signal_group_state.is_some());
    }

    #[test]
    fn test_traffic_signal_xml_serialization() {
        let controller = TrafficSignalController::new("test_controller")
            .add_phase(Phase::new("test_phase", 30.0).add_signal_state("signal_1", "green"));

        // Test that serialization works (basic check)
        let serialized = serde_json::to_string(&controller);
        assert!(serialized.is_ok());
    }

    #[test]
    fn test_traffic_signal_defaults() {
        let controller = TrafficSignalController::default();
        assert_eq!(
            controller.name.as_literal(),
            Some(&"DefaultController".to_string())
        );
        assert!(controller.delay.is_none());
        assert!(controller.reference.is_none());
        assert_eq!(controller.phases.len(), 0);

        let phase = Phase::default();
        assert_eq!(phase.name.as_literal(), Some(&"DefaultPhase".to_string()));
        assert_eq!(phase.duration.as_literal(), Some(&30.0));
        assert_eq!(phase.traffic_signal_states.len(), 0);
        assert!(phase.traffic_signal_group_state.is_none());

        let state = TrafficSignalState::default();
        assert_eq!(
            state.traffic_signal_id.as_literal(),
            Some(&"signal_1".to_string())
        );
        assert_eq!(state.state.as_literal(), Some(&"green".to_string()));

        let group_state = TrafficSignalGroupState::default();
        assert_eq!(group_state.state.as_literal(), Some(&"green".to_string()));

        let state_action = TrafficSignalStateAction::default();
        assert_eq!(
            state_action.name.as_literal(),
            Some(&"DefaultSignal".to_string())
        );
        assert_eq!(state_action.state.as_literal(), Some(&"green".to_string()));

        let controller_action = TrafficSignalControllerAction::default();
        assert_eq!(
            controller_action.traffic_signal_controller_ref.as_literal(),
            Some(&"DefaultController".to_string())
        );
        assert_eq!(
            controller_action.phase_ref.as_literal(),
            Some(&"Phase1".to_string())
        );
    }

    #[test]
    fn test_traffic_signal_timing_validation() {
        // Test realistic traffic signal timing
        let controller = TrafficSignalController::new("realistic_intersection")
            .with_delay(1.0)
            .add_phase(Phase::new("green_ns", 45.0))
            .add_phase(Phase::new("yellow_ns", 3.0))
            .add_phase(Phase::new("red_ns_green_ew", 40.0))
            .add_phase(Phase::new("yellow_ew", 3.0));

        // Total cycle time should be reasonable
        let total_time: f64 = controller
            .phases
            .iter()
            .map(|p| p.duration.as_literal().unwrap_or(&0.0))
            .sum();

        assert_eq!(total_time, 91.0); // 45 + 3 + 40 + 3
        assert!(total_time > 60.0 && total_time < 180.0); // Reasonable cycle time
    }

    #[test]
    fn test_traffic_signal_state_transitions() {
        // Test that we can model state transitions properly
        let states = vec![
            TrafficSignalState::new("main", "green"),
            TrafficSignalState::new("main", "yellow"),
            TrafficSignalState::new("main", "red"),
        ];

        assert_eq!(states[0].state.as_literal(), Some(&"green".to_string()));
        assert_eq!(states[1].state.as_literal(), Some(&"yellow".to_string()));
        assert_eq!(states[2].state.as_literal(), Some(&"red".to_string()));

        // All should reference the same signal
        for state in &states {
            assert_eq!(
                state.traffic_signal_id.as_literal(),
                Some(&"main".to_string())
            );
        }
    }

    // COMPREHENSIVE TRAFFIC SWARM ACTION TESTS (Task 2.2 Requirements)

    #[test]
    fn test_traffic_swarm_action_creation_comprehensive() {
        let swarm = TrafficSwarmAction::new("Ego", 100.0, 50.0, 10)
            .with_inner_radius(20.0)
            .with_central_swarm_object("CentralVehicle");

        assert_eq!(swarm.number_of_vehicles.as_literal().unwrap(), &10);
        assert_eq!(swarm.inner_radius.as_literal().unwrap(), &20.0);
        assert_eq!(swarm.semi_major_axis.as_literal().unwrap(), &100.0);
        assert_eq!(swarm.semi_minor_axis.as_literal().unwrap(), &50.0);
        assert_eq!(
            swarm.central_object.entity_ref.as_literal().unwrap(),
            "CentralVehicle"
        );
    }

    #[test]
    fn test_central_swarm_object_creation() {
        let central = CentralSwarmObject::new("LeadVehicle");
        assert_eq!(central.entity_ref.as_literal().unwrap(), "LeadVehicle");
    }

    #[test]
    fn test_xml_serialization_traffic_swarm() {
        let swarm = TrafficSwarmAction::new("Ego", 75.0, 30.0, 5);

        let xml = quick_xml::se::to_string(&swarm).unwrap();
        assert!(xml.contains("TrafficSwarmAction"));
        assert!(xml.contains("semiMajorAxis=\"75\""));
        assert!(xml.contains("semiMinorAxis=\"30\""));
        assert!(xml.contains("numberOfVehicles=\"5\""));
    }

    #[test]
    fn test_xml_round_trip_traffic_swarm() {
        let original = TrafficSwarmAction::new("TestEntity", 120.0, 60.0, 8)
            .with_inner_radius(15.0)
            .with_offset(5.0);

        let xml = quick_xml::se::to_string(&original).unwrap();

        // Try to parse, but handle the error gracefully
        let deserialized = match quick_xml::de::from_str::<TrafficSwarmAction>(&xml) {
            Ok(result) => result,
            Err(e) => {
                println!("Deserialization error: {}", e);
                panic!("Failed to deserialize: {}", e);
            }
        };

        assert_eq!(
            original.central_object.entity_ref.as_literal(),
            deserialized.central_object.entity_ref.as_literal()
        );
        assert_eq!(
            original.semi_major_axis.as_literal(),
            deserialized.semi_major_axis.as_literal()
        );
        assert_eq!(
            original.semi_minor_axis.as_literal(),
            deserialized.semi_minor_axis.as_literal()
        );
        assert_eq!(
            original.number_of_vehicles.as_literal(),
            deserialized.number_of_vehicles.as_literal()
        );
        assert_eq!(
            original.inner_radius.as_literal(),
            deserialized.inner_radius.as_literal()
        );
        assert_eq!(
            original.offset.as_literal(),
            deserialized.offset.as_literal()
        );
    }

    #[test]
    fn test_traffic_swarm_with_central_object() {
        let swarm = TrafficSwarmAction::new("MainVehicle", 80.0, 40.0, 6)
            .with_central_swarm_object("CentralEntity");

        assert_eq!(
            swarm.central_object.entity_ref.as_literal().unwrap(),
            "CentralEntity"
        );
    }

    #[test]
    fn test_traffic_swarm_defaults() {
        let swarm = TrafficSwarmAction::default();
        // Test that defaults are reasonable
        assert!(swarm.central_object.entity_ref.as_literal().is_some());
        assert!(swarm.semi_major_axis.as_literal().is_some());
        assert!(swarm.semi_minor_axis.as_literal().is_some());
        assert!(swarm.number_of_vehicles.as_literal().is_some());
        assert!(swarm.inner_radius.as_literal().is_some());
        assert!(swarm.offset.as_literal().is_some());
    }

    #[test]
    fn test_realistic_swarm_parameters() {
        // Test realistic highway swarm scenario
        let highway_swarm = TrafficSwarmAction::new("EgoVehicle", 200.0, 100.0, 15)
            .with_inner_radius(50.0)
            .with_offset(10.0);

        assert_eq!(highway_swarm.semi_major_axis.as_literal().unwrap(), &200.0);
        assert_eq!(highway_swarm.inner_radius.as_literal().unwrap(), &50.0);
        assert_eq!(highway_swarm.offset.as_literal().unwrap(), &10.0);

        // Test city intersection swarm scenario
        let city_swarm = TrafficSwarmAction::new("CityEgo", 80.0, 60.0, 8)
            .with_central_swarm_object("IntersectionController");

        assert_eq!(
            city_swarm.central_object.entity_ref.as_literal().unwrap(),
            "IntersectionController"
        );
        assert_eq!(city_swarm.number_of_vehicles.as_literal().unwrap(), &8);
    }

    #[test]
    fn test_traffic_swarm_with_velocity() {
        let swarm = TrafficSwarmAction::new("TestVehicle", 90.0, 45.0, 12).with_velocity(60.0);

        assert!(swarm.velocity.is_some());
        assert_eq!(swarm.velocity.unwrap().as_literal().unwrap(), &60.0);
    }

    #[test]
    fn test_traffic_swarm_elliptical_parameters() {
        // Test elliptical swarm with different major/minor axes
        let elliptical_swarm = TrafficSwarmAction::new("EllipseCenter", 150.0, 75.0, 20);

        // Major axis should be larger than minor axis for proper ellipse
        assert!(
            elliptical_swarm.semi_major_axis.as_literal().unwrap()
                > elliptical_swarm.semi_minor_axis.as_literal().unwrap()
        );

        // Test circular swarm (equal axes)
        let circular_swarm = TrafficSwarmAction::new("CircleCenter", 100.0, 100.0, 15);
        assert_eq!(
            circular_swarm.semi_major_axis.as_literal().unwrap(),
            circular_swarm.semi_minor_axis.as_literal().unwrap()
        );
    }

    #[test]
    fn test_traffic_swarm_builder_pattern() {
        let swarm = TrafficSwarmAction::new("BuilderTest", 120.0, 80.0, 10)
            .with_inner_radius(25.0)
            .with_offset(15.0)
            .with_velocity(55.0)
            .with_traffic_definition(TrafficDefinition::default())
            .with_central_swarm_object("NewCentralEntity");

        assert_eq!(swarm.inner_radius.as_literal().unwrap(), &25.0);
        assert_eq!(swarm.offset.as_literal().unwrap(), &15.0);
        assert_eq!(swarm.velocity.unwrap().as_literal().unwrap(), &55.0);
        assert!(swarm.traffic_definition.is_some());
        assert_eq!(
            swarm.central_object.entity_ref.as_literal().unwrap(),
            "NewCentralEntity"
        );
    }
}
