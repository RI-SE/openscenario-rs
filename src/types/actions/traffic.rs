//! Phase 4C: Traffic Management Actions Implementation
//!
//! This file contains:
//! - Traffic source and sink actions for dynamic traffic generation
//! - Traffic swarm actions for crowd simulation around entities
//! - Traffic area actions for regional traffic density control
//! - Traffic signal control actions for intersection management
//! - Background traffic definition and distribution specifications
//!
//! Contributes to project by:
//! - Enabling realistic traffic environments for scenario testing
//! - Providing dynamic traffic generation and removal capabilities
//! - Supporting complex traffic patterns and density variations
//! - Facilitating intersection and traffic signal simulation
//! - Following complete OpenSCENARIO specification for traffic management

use serde::{Deserialize, Serialize};
use crate::types::basic::{Boolean, OSString, Double};
use crate::types::positions::Position;

// PHASE 4C: Core Traffic Management Actions

/// Traffic source action for traffic generation with rate and position
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficSourceAction {
    #[serde(rename = "@rate")]
    pub rate: f64,
    #[serde(rename = "@velocity")]
    pub velocity: Option<f64>,
    #[serde(rename = "Position")]
    pub position: Position,
    #[serde(rename = "TrafficDefinition")]
    pub traffic_definition: TrafficDefinition,
}

/// Traffic sink action for traffic removal with radius control
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficSinkAction {
    #[serde(rename = "@rate")]
    pub rate: f64,
    #[serde(rename = "@radius")]
    pub radius: f64,
    #[serde(rename = "Position")]
    pub position: Position,
    #[serde(rename = "TrafficDefinition")]
    pub traffic_definition: Option<TrafficDefinition>,
}

/// Traffic swarm action for swarm behavior around central object
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficSwarmAction {
    #[serde(rename = "@centralObject")]
    pub central_object: OSString,
    #[serde(rename = "@innerRadius")]
    pub inner_radius: f64,
    #[serde(rename = "@outerRadius")]
    pub outer_radius: f64,
    #[serde(rename = "@numberOfVehicles")]
    pub number_of_vehicles: u32,
    #[serde(rename = "CentralSwarmObject")]
    pub central_swarm_object: CentralSwarmObject,
    #[serde(rename = "TrafficDefinition")]
    pub traffic_definition: TrafficDefinition,
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
    #[serde(rename = "@phaseRef")]
    pub phase_ref: OSString,
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
    pub weight: f64,
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
    pub weight: f64,
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
            rate: 10.0, // 10 vehicles per minute
            velocity: Some(50.0), // 50 km/h default velocity
            position: Position::default(),
            traffic_definition: TrafficDefinition::default(),
        }
    }
}

impl Default for TrafficSinkAction {
    fn default() -> Self {
        Self {
            rate: 10.0, // 10 vehicles per minute
            radius: 50.0, // 50 meter radius
            position: Position::default(),
            traffic_definition: None,
        }
    }
}

impl Default for TrafficSwarmAction {
    fn default() -> Self {
        Self {
            central_object: OSString::literal("SwarmCenter".to_string()),
            inner_radius: 10.0,
            outer_radius: 50.0,
            number_of_vehicles: 20,
            central_swarm_object: CentralSwarmObject::default(),
            traffic_definition: TrafficDefinition::default(),
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
                TrafficSignalStateAction::default()
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
                    weight: 0.7, // 70% cars
                },
                VehicleCategoryDistributionEntry {
                    category: VehicleCategory::Truck,
                    weight: 0.2, // 20% trucks
                },
                VehicleCategoryDistributionEntry {
                    category: VehicleCategory::Van,
                    weight: 0.1, // 10% vans
                },
            ],
        }
    }
}

impl Default for ControllerDistribution {
    fn default() -> Self {
        Self {
            entries: vec![
                ControllerDistributionEntry {
                    weight: 1.0,
                    controller: OSString::literal("DefaultTrafficController".to_string()),
                }
            ],
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
            rate,
            velocity: None,
            position,
            traffic_definition,
        }
    }

    /// Create traffic source with velocity
    pub fn with_velocity(rate: f64, velocity: f64, position: Position, traffic_definition: TrafficDefinition) -> Self {
        Self {
            rate,
            velocity: Some(velocity),
            position,
            traffic_definition,
        }
    }
}

impl TrafficSinkAction {
    /// Create traffic sink with rate, radius and position
    pub fn new(rate: f64, radius: f64, position: Position) -> Self {
        Self {
            rate,
            radius,
            position,
            traffic_definition: None,
        }
    }

    /// Create traffic sink with traffic definition
    pub fn with_traffic_definition(
        rate: f64, 
        radius: f64, 
        position: Position, 
        traffic_definition: TrafficDefinition
    ) -> Self {
        Self {
            rate,
            radius,
            position,
            traffic_definition: Some(traffic_definition),
        }
    }
}

impl TrafficSwarmAction {
    /// Create traffic swarm around central object
    pub fn new(
        central_object: String,
        inner_radius: f64,
        outer_radius: f64,
        number_of_vehicles: u32,
        traffic_definition: TrafficDefinition,
    ) -> Self {
        Self {
            central_object: OSString::literal(central_object.clone()),
            inner_radius,
            outer_radius,
            number_of_vehicles,
            central_swarm_object: CentralSwarmObject {
                entity_ref: OSString::literal(central_object),
            },
            traffic_definition,
        }
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
                }
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
                }
            ),
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
        controllers: ControllerDistribution
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
            entries: vec![VehicleCategoryDistributionEntry { category, weight }],
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
                    weight: 0.85,
                },
                VehicleCategoryDistributionEntry {
                    category: VehicleCategory::Bus,
                    weight: 0.1,
                },
                VehicleCategoryDistributionEntry {
                    category: VehicleCategory::Van,
                    weight: 0.05,
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
                weight,
                controller: OSString::literal(controller),
            }],
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
        let source = TrafficSourceAction::new(
            15.0, 
            Position::default(), 
            TrafficDefinition::default()
        );
        
        assert_eq!(source.rate, 15.0);
        assert!(source.velocity.is_none());
    }

    #[test]
    fn test_traffic_source_with_velocity() {
        let source = TrafficSourceAction::with_velocity(
            20.0, 
            60.0, 
            Position::default(), 
            TrafficDefinition::default()
        );
        
        assert_eq!(source.rate, 20.0);
        assert_eq!(source.velocity, Some(60.0));
    }

    #[test]
    fn test_traffic_sink_action_creation() {
        let sink = TrafficSinkAction::new(10.0, 30.0, Position::default());
        
        assert_eq!(sink.rate, 10.0);
        assert_eq!(sink.radius, 30.0);
        assert!(sink.traffic_definition.is_none());
    }

    #[test]
    fn test_traffic_sink_with_definition() {
        let sink = TrafficSinkAction::with_traffic_definition(
            12.0, 
            40.0, 
            Position::default(), 
            TrafficDefinition::default()
        );
        
        assert_eq!(sink.rate, 12.0);
        assert_eq!(sink.radius, 40.0);
        assert!(sink.traffic_definition.is_some());
    }

    #[test]
    fn test_traffic_swarm_action_creation() {
        let swarm = TrafficSwarmAction::new(
            "LeadVehicle".to_string(),
            5.0,
            25.0,
            15,
            TrafficDefinition::default(),
        );
        
        assert_eq!(swarm.central_object.as_literal(), Some(&"LeadVehicle".to_string()));
        assert_eq!(swarm.inner_radius, 5.0);
        assert_eq!(swarm.outer_radius, 25.0);
        assert_eq!(swarm.number_of_vehicles, 15);
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
        let signal = TrafficSignalAction::state_action(
            "Intersection1".to_string(),
            "red".to_string(),
        );
        
        if let TrafficSignalActionChoice::TrafficSignalStateAction(state_action) = signal.signal_action_choice {
            assert_eq!(state_action.name.as_literal(), Some(&"Intersection1".to_string()));
            assert_eq!(state_action.state.as_literal(), Some(&"red".to_string()));
        } else {
            panic!("Expected TrafficSignalStateAction");
        }
    }

    #[test]
    fn test_traffic_signal_controller_action() {
        let signal = TrafficSignalAction::controller_action(
            "Controller1".to_string(),
            "Phase2".to_string(),
        );
        
        if let TrafficSignalActionChoice::TrafficSignalControllerAction(controller_action) = signal.signal_action_choice {
            assert_eq!(controller_action.traffic_signal_controller_ref.as_literal(), Some(&"Controller1".to_string()));
            assert_eq!(controller_action.phase_ref.as_literal(), Some(&"Phase2".to_string()));
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
        assert!(mixed.entries.iter().any(|e| matches!(e.category, VehicleCategory::Car)));
        
        let urban = VehicleCategoryDistribution::urban_traffic();
        assert_eq!(urban.entries.len(), 3);
        assert_eq!(urban.entries[0].weight, 0.85); // Mostly cars
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
        let entry1 = VehicleCategoryDistributionEntry { category: car, weight: 0.6 };
        let entry2 = VehicleCategoryDistributionEntry { category: truck, weight: 0.3 };
        let entry3 = VehicleCategoryDistributionEntry { category: bike, weight: 0.1 };
        
        assert_eq!(entry1.weight, 0.6);
        assert_eq!(entry2.weight, 0.3);
        assert_eq!(entry3.weight, 0.1);
    }

    #[test]
    fn test_traffic_action_defaults() {
        let source = TrafficSourceAction::default();
        assert_eq!(source.rate, 10.0);
        assert_eq!(source.velocity, Some(50.0));
        
        let sink = TrafficSinkAction::default();
        assert_eq!(sink.rate, 10.0);
        assert_eq!(sink.radius, 50.0);
        
        let swarm = TrafficSwarmAction::default();
        assert_eq!(swarm.number_of_vehicles, 20);
        assert_eq!(swarm.inner_radius, 10.0);
        assert_eq!(swarm.outer_radius, 50.0);
        
        let stop = TrafficStopAction::default();
        assert_eq!(stop.enable.as_literal(), Some(&true));
    }
}