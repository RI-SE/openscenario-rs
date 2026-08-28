//! Traffic management actions implementation
//!
//! This file contains:
//! - Traffic source and sink actions for dynamic traffic generation
//! - Traffic swarm actions for crowd simulation around entities
//! - Traffic area actions for regional traffic density control
//! - Traffic signal control actions for intersection management
//! - Background traffic definition and distribution specifications
//!
use crate::types::basic::{Boolean, Double, Int, OSString, Range, UnsignedInt};
use crate::types::catalogs::references::ControllerCatalogReference;
use crate::types::controllers::Controller;
use crate::types::entities::{EntityDistribution, Properties};
use crate::types::enums::VehicleCategory;
use crate::types::positions::Position;
use serde::{Deserialize, Serialize};


/// Traffic source action for traffic generation with rate and position
///
/// This action generates traffic vehicles at a specified position with a given rate.
/// Used to create dynamic traffic scenarios where vehicles enter the simulation over time.
///
/// # Fields
///
/// * `rate` - Rate of vehicle generation (vehicles per minute)
/// * `velocity` - Optional velocity for generated vehicles (meters/second)
/// * `position` - Position where vehicles are generated
/// * `traffic_definition` - Definition of traffic properties for generated vehicles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficSourceAction {
    #[serde(rename = "@radius")]
    pub radius: Double,
    #[serde(rename = "@rate")]
    pub rate: Double,
    /// Deprecated in favor of `speed`.
    #[serde(rename = "@velocity", default, skip_serializing_if = "Option::is_none")]
    pub velocity: Option<Double>,
    /// Speed for generated vehicles (current replacement for `velocity`)
    #[serde(rename = "@speed", default, skip_serializing_if = "Option::is_none")]
    pub speed: Option<Double>,
    #[serde(rename = "Position")]
    pub position: Position,
    /// Deprecated in favor of TrafficDistribution; kept optional per XSD (minOccurs=0)
    #[serde(rename = "TrafficDefinition", default, skip_serializing_if = "Option::is_none")]
    pub traffic_definition: Option<TrafficDefinition>,
    #[serde(rename = "TrafficDistribution", default, skip_serializing_if = "Option::is_none")]
    pub traffic_distribution: Option<TrafficDistribution>,
}

/// Traffic sink action for traffic removal with radius control
///
/// This action removes vehicles that enter a specified radius around a position.
/// Used to prevent traffic accumulation and manage vehicle lifecycle in simulations.
///
/// # Fields
///
/// * `rate` - Rate of vehicle removal (vehicles per minute)
/// * `radius` - Radius around position for vehicle removal (meters)
/// * `position` - Center position for removal area
/// * `traffic_definition` - Optional traffic definition to filter which vehicles are removed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficSinkAction {
    #[serde(rename = "@rate", default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<Double>,
    #[serde(rename = "@radius")]
    pub radius: Double,
    #[serde(rename = "Position")]
    pub position: Position,
    #[serde(rename = "TrafficDefinition", default, skip_serializing_if = "Option::is_none")]
    pub traffic_definition: Option<TrafficDefinition>,
}

/// Traffic swarm action for swarm behavior around central object
///
/// This action creates a swarm of vehicles positioned in an elliptical pattern
/// around a central object. The swarm can be used to simulate traffic
/// density and complex interaction scenarios.
///
/// # Fields
///
/// * `inner_radius` - Inner radius of the elliptical swarm area (meters)
/// * `number_of_vehicles` - Number of vehicles to generate
/// * `offset` - Offset from central object (meters)
/// * `semi_major_axis` - Length of semi-major axis (meters)
/// * `semi_minor_axis` - Length of semi-minor axis (meters)
/// * `velocity` - Optional velocity for swarm vehicles
/// * `central_object` - Reference to central object
/// * `traffic_definition` - Optional traffic definition for generated vehicles
///
/// # Example
///
/// ```rust
/// use openscenario_rs::types::actions::{TrafficSwarmAction, CentralSwarmObject};
/// use openscenario_rs::types::basic::{Double, UnsignedInt};
/// use openscenario_rs::types::positions::Position;
///
/// let swarm = TrafficSwarmAction {
///     inner_radius: Double::literal(5.0),
///     number_of_vehicles: UnsignedInt::literal(10),
///     offset: Double::literal(0.0),
///     semi_major_axis: Double::literal(20.0),
///     semi_minor_axis: Double::literal(15.0),
///     velocity: Some(Double::literal(30.0)),
///     central_object: CentralSwarmObject::new("CentralEntity"),
///     traffic_definition: None,
///     traffic_distribution: None,
///     initial_speed_range: None,
///     direction_of_travel_distribution: None,
/// };
/// ```
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
    #[serde(rename = "@velocity", default, skip_serializing_if = "Option::is_none")]
    pub velocity: Option<Double>,
    #[serde(rename = "CentralObject")]
    pub central_object: CentralSwarmObject,
    #[serde(rename = "TrafficDefinition", default, skip_serializing_if = "Option::is_none")]
    pub traffic_definition: Option<TrafficDefinition>,
    #[serde(rename = "TrafficDistribution", default, skip_serializing_if = "Option::is_none")]
    pub traffic_distribution: Option<TrafficDistribution>,
    #[serde(rename = "InitialSpeedRange", default, skip_serializing_if = "Option::is_none")]
    pub initial_speed_range: Option<Range>,
    #[serde(
        rename = "DirectionOfTravelDistribution",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub direction_of_travel_distribution: Option<DirectionOfTravelDistribution>,
}

/// Traffic area action for area-based traffic management
///
/// XSD `TrafficAreaAction` (`:2220-2227`): `xsd:all` of required
/// `TrafficDistribution` and required `TrafficArea`; required attributes
/// `@numberOfEntities` (UnsignedInt) and `@continuous` (Boolean).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficAreaAction {
    #[serde(rename = "@numberOfEntities")]
    pub number_of_entities: UnsignedInt,
    #[serde(rename = "@continuous")]
    pub continuous: Boolean,
    #[serde(rename = "TrafficDistribution")]
    pub traffic_distribution: TrafficDistribution,
    #[serde(rename = "TrafficArea")]
    pub traffic_area: TrafficArea,
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
    #[serde(rename = "@delay", skip_serializing_if = "Option::is_none")]
    pub delay: Option<Double>,
    #[serde(rename = "@reference", skip_serializing_if = "Option::is_none")]
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
    #[serde(
        rename = "TrafficSignalGroupState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
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
///
/// XSD `TrafficStopAction` is an empty complexType with no attributes or children.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficStopAction {}


/// Traffic definition for vehicle category and controller distribution
///
/// Defines the properties of traffic that should be generated, including
/// vehicle types and their probabilities, as well as controller behavior profiles.
///
/// # Fields
///
/// * `vehicle_category_distribution` - Distribution of vehicle categories (cars, trucks, etc.)
/// * `controller_distribution` - Distribution of controller behaviors
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficDefinition {
    #[serde(rename = "@name")]
    pub name: OSString,
    #[serde(rename = "VehicleCategoryDistribution")]
    pub vehicle_category_distribution: VehicleCategoryDistribution,
    #[serde(
        rename = "VehicleRoleDistribution",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vehicle_role_distribution: Option<VehicleRoleDistribution>,
    #[serde(rename = "ControllerDistribution")]
    pub controller_distribution: ControllerDistribution,
}

/// Vehicle role distribution for traffic composition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct VehicleRoleDistribution {
    #[serde(rename = "VehicleRoleDistributionEntry", default)]
    pub entries: Vec<VehicleRoleDistributionEntry>,
}

/// Vehicle role distribution entry
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VehicleRoleDistributionEntry {
    #[serde(rename = "@weight")]
    pub weight: Double,
    #[serde(rename = "@role")]
    pub role: crate::types::enums::Role,
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
    #[serde(rename = "Controller", default, skip_serializing_if = "Option::is_none")]
    pub controller: Option<Controller>,
    #[serde(
        rename = "CatalogReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub catalog_reference: Option<ControllerCatalogReference>,
}

/// Central swarm object specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CentralSwarmObject {
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
}

/// Traffic area definition as a choice of `Polygon` or a set of `RoadRange`s
///
/// XSD `TrafficArea` (`:2214-2219`): choice of `Polygon` or `RoadRange`
/// (`maxOccurs="unbounded"`). Modeled as parallel `Option` fields per the
/// crate's XSD-choice convention.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficArea {
    #[serde(rename = "Polygon", default, skip_serializing_if = "Option::is_none")]
    pub polygon: Option<Polygon>,
    #[serde(rename = "RoadRange", default, skip_serializing_if = "Vec::is_empty")]
    pub road_range: Vec<RoadRange>,
}

/// Closed polygon area defined by at least three positions
///
/// XSD `Polygon` (`:1728-1732`): sequence of `Position`, `minOccurs="3"`,
/// unbounded.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Polygon {
    #[serde(rename = "Position")]
    pub position: Vec<Position>,
}

/// Range along a road defined by at least two road cursors
///
/// XSD `RoadRange` (`:1949-1954`): sequence of `RoadCursor`, `minOccurs="2"`,
/// unbounded; optional attribute `@length` (Double).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoadRange {
    #[serde(rename = "@length", default, skip_serializing_if = "Option::is_none")]
    pub length: Option<Double>,
    #[serde(rename = "RoadCursor")]
    pub road_cursor: Vec<RoadCursor>,
}

/// A position along a road, optionally restricted to a set of lanes
///
/// XSD `RoadCursor` (`:1926-1932`): sequence of `Lane` (minOccurs=0,
/// unbounded); required attribute `@roadId` (String), optional `@s` (Double).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoadCursor {
    #[serde(rename = "@roadId")]
    pub road_id: OSString,
    #[serde(rename = "@s", default, skip_serializing_if = "Option::is_none")]
    pub s: Option<Double>,
    #[serde(rename = "Lane", default, skip_serializing_if = "Vec::is_empty")]
    pub lane: Vec<Lane>,
}

/// Reference to a lane by numeric id
///
/// XSD `Lane` (`:1333-1335`): required attribute `@id` (Int).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Lane {
    #[serde(rename = "@id")]
    pub id: Int,
}

/// Weighted distribution of traffic entities
///
/// XSD `TrafficDistribution` (`:2236-2240`): sequence of
/// `TrafficDistributionEntry`, `maxOccurs="unbounded"`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficDistribution {
    #[serde(rename = "TrafficDistributionEntry")]
    pub traffic_distribution_entry: Vec<TrafficDistributionEntry>,
}

/// Single weighted entry in a `TrafficDistribution`
///
/// XSD `TrafficDistributionEntry` (`:2241-2247`): sequence of required
/// `EntityDistribution` and optional `Properties` (minOccurs=0); required
/// attribute `@weight` (Double).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficDistributionEntry {
    #[serde(rename = "@weight")]
    pub weight: Double,
    #[serde(rename = "EntityDistribution")]
    pub entity_distribution: EntityDistribution,
    #[serde(rename = "Properties", default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Properties>,
}

/// Split between vehicles traveling in the same vs. opposite direction
///
/// XSD `DirectionOfTravelDistribution` (`:1063-1066`): required attributes
/// `@same` (Double) and `@opposite` (Double).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DirectionOfTravelDistribution {
    #[serde(rename = "@same")]
    pub same: Double,
    #[serde(rename = "@opposite")]
    pub opposite: Double,
}


impl Default for TrafficSourceAction {
    fn default() -> Self {
        Self {
            radius: Double::literal(10.0),         // 10 meter radius
            rate: Double::literal(10.0),           // 10 vehicles per minute
            velocity: Some(Double::literal(50.0)), // 50 km/h default velocity
            speed: None,
            position: Position::default(),
            traffic_definition: Some(TrafficDefinition::default()),
            traffic_distribution: None,
        }
    }
}

impl Default for TrafficSinkAction {
    fn default() -> Self {
        Self {
            rate: Some(Double::literal(10.0)), // 10 vehicles per minute
            radius: Double::literal(50.0),     // 50 meter radius
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
            traffic_distribution: None,
            initial_speed_range: None,
            direction_of_travel_distribution: None,
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
        Self {}
    }
}

impl Default for TrafficDefinition {
    fn default() -> Self {
        Self {
            name: OSString::literal("DefaultTrafficDefinition".to_string()),
            vehicle_category_distribution: VehicleCategoryDistribution::default(),
            vehicle_role_distribution: None,
            controller_distribution: ControllerDistribution::default(),
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
                controller: Some(Controller::new(
                    "DefaultTrafficController".to_string(),
                    crate::types::enums::ControllerType::Movement,
                )),
                catalog_reference: None,
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
            polygon: Some(Polygon::default()),
            road_range: Vec::new(),
        }
    }
}

impl Default for Polygon {
    fn default() -> Self {
        Self {
            position: vec![Position::default(), Position::default(), Position::default()],
        }
    }
}

impl Default for RoadRange {
    fn default() -> Self {
        Self {
            length: None,
            road_cursor: vec![RoadCursor::default(), RoadCursor::default()],
        }
    }
}

impl Default for RoadCursor {
    fn default() -> Self {
        Self {
            road_id: OSString::literal("DefaultRoad".to_string()),
            s: None,
            lane: Vec::new(),
        }
    }
}

impl Default for Lane {
    fn default() -> Self {
        Self { id: Int::literal(0) }
    }
}

impl Default for TrafficDistribution {
    fn default() -> Self {
        Self {
            traffic_distribution_entry: vec![TrafficDistributionEntry::default()],
        }
    }
}

impl Default for TrafficDistributionEntry {
    fn default() -> Self {
        Self {
            weight: Double::literal(1.0),
            entity_distribution: EntityDistribution::default(),
            properties: None,
        }
    }
}

impl Default for DirectionOfTravelDistribution {
    fn default() -> Self {
        Self {
            same: Double::literal(1.0),
            opposite: Double::literal(0.0),
        }
    }
}

impl Default for TrafficAreaAction {
    fn default() -> Self {
        Self {
            number_of_entities: UnsignedInt::literal(1),
            continuous: Boolean::literal(false),
            traffic_distribution: TrafficDistribution::default(),
            traffic_area: TrafficArea::default(),
        }
    }
}



impl TrafficSourceAction {
    /// Create traffic source with radius, rate and position
    pub fn new(
        radius: f64,
        rate: f64,
        position: Position,
        traffic_definition: TrafficDefinition,
    ) -> Self {
        Self {
            radius: Double::literal(radius),
            rate: Double::literal(rate),
            velocity: None,
            speed: None,
            position,
            traffic_definition: Some(traffic_definition),
            traffic_distribution: None,
        }
    }

    /// Create traffic source with velocity
    pub fn with_velocity(
        radius: f64,
        rate: f64,
        velocity: f64,
        position: Position,
        traffic_definition: TrafficDefinition,
    ) -> Self {
        Self {
            radius: Double::literal(radius),
            rate: Double::literal(rate),
            velocity: Some(Double::literal(velocity)),
            speed: None,
            position,
            traffic_definition: Some(traffic_definition),
            traffic_distribution: None,
        }
    }

    /// Set the traffic distribution for the source
    pub fn with_traffic_distribution(mut self, traffic_distribution: TrafficDistribution) -> Self {
        self.traffic_distribution = Some(traffic_distribution);
        self
    }
}

impl TrafficSinkAction {
    /// Create traffic sink with rate, radius and position
    pub fn new(rate: f64, radius: f64, position: Position) -> Self {
        Self {
            rate: Some(Double::literal(rate)),
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
            rate: Some(Double::literal(rate)),
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
            traffic_distribution: None,
            initial_speed_range: None,
            direction_of_travel_distribution: None,
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

    /// Set traffic distribution for the swarm
    pub fn with_traffic_distribution(mut self, traffic_distribution: TrafficDistribution) -> Self {
        self.traffic_distribution = Some(traffic_distribution);
        self
    }

    /// Set initial speed range for the swarm
    pub fn with_initial_speed_range(mut self, initial_speed_range: Range) -> Self {
        self.initial_speed_range = Some(initial_speed_range);
        self
    }

    /// Set direction-of-travel distribution for the swarm
    pub fn with_direction_of_travel_distribution(
        mut self,
        direction_of_travel_distribution: DirectionOfTravelDistribution,
    ) -> Self {
        self.direction_of_travel_distribution = Some(direction_of_travel_distribution);
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
    /// Create a traffic area action with the required distribution and area
    pub fn new(
        number_of_entities: u32,
        continuous: bool,
        traffic_distribution: TrafficDistribution,
        traffic_area: TrafficArea,
    ) -> Self {
        Self {
            number_of_entities: UnsignedInt::literal(number_of_entities),
            continuous: Boolean::literal(continuous),
            traffic_distribution,
            traffic_area,
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

impl TrafficDefinition {
    /// Create traffic definition with vehicle categories only
    pub fn with_vehicles(distribution: VehicleCategoryDistribution) -> Self {
        Self {
            name: OSString::literal("DefaultTrafficDefinition".to_string()),
            vehicle_category_distribution: distribution,
            vehicle_role_distribution: None,
            controller_distribution: ControllerDistribution::default(),
        }
    }

    /// Create traffic definition with controllers only
    pub fn with_controllers(distribution: ControllerDistribution) -> Self {
        Self {
            name: OSString::literal("DefaultTrafficDefinition".to_string()),
            vehicle_category_distribution: VehicleCategoryDistribution::default(),
            vehicle_role_distribution: None,
            controller_distribution: distribution,
        }
    }

    /// Create traffic definition with both vehicles and controllers
    pub fn with_both(
        vehicles: VehicleCategoryDistribution,
        controllers: ControllerDistribution,
    ) -> Self {
        Self {
            name: OSString::literal("DefaultTrafficDefinition".to_string()),
            vehicle_category_distribution: vehicles,
            vehicle_role_distribution: None,
            controller_distribution: controllers,
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
                controller: Some(Controller::new(
                    controller,
                    crate::types::enums::ControllerType::Movement,
                )),
                catalog_reference: None,
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

impl Polygon {
    /// Create a rectangular polygon from world-space corners
    pub fn rectangle(x: f64, y: f64, width: f64, height: f64) -> Self {
        use crate::types::positions::WorldPosition;

        let corner = |cx: f64, cy: f64| Position {
            world_position: Some(WorldPosition::new(cx, cy)),
            ..Position::empty()
        };

        Self {
            position: vec![
                corner(x, y),
                corner(x + width, y),
                corner(x + width, y + height),
                corner(x, y + height),
            ],
        }
    }
}

impl TrafficArea {
    /// Create a traffic area bounded by a rectangular polygon
    pub fn rectangle(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            polygon: Some(Polygon::rectangle(x, y, width, height)),
            road_range: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::positions::Position;

    #[test]
    fn test_controller_distribution_entries_deserialize() {
        // XSD `ControllerDistribution` (:990) is a sequence of
        // `ControllerDistributionEntry` (`maxOccurs="unbounded"`) and has
        // neither `@controllerType` nor a `ParameterValueDistribution`
        // child. This previously failed to deserialize because the
        // shadowed `types::controllers::ControllerDistribution` required
        // both.
        let xml = r#"<ControllerDistribution>
            <ControllerDistributionEntry weight="0.6">
                <Controller name="AIController"/>
            </ControllerDistributionEntry>
            <ControllerDistributionEntry weight="0.4">
                <CatalogReference catalogName="ControllerCatalog" entryName="Manual"/>
            </ControllerDistributionEntry>
        </ControllerDistribution>"#;

        let distribution: ControllerDistribution = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(distribution.entries.len(), 2);
        assert_eq!(distribution.entries[0].weight.as_literal(), Some(&0.6));
        assert!(distribution.entries[0].controller.is_some());
        assert_eq!(distribution.entries[1].weight.as_literal(), Some(&0.4));
        assert!(distribution.entries[1].catalog_reference.is_some());
    }

    #[test]
    fn test_traffic_source_action_creation() {
        let source = TrafficSourceAction::new(
            5.0,
            15.0,
            Position::default(),
            TrafficDefinition::default(),
        );

        assert_eq!(source.radius.as_literal(), Some(&5.0));
        assert_eq!(source.rate.as_literal(), Some(&15.0));
        assert!(source.velocity.is_none());
    }

    #[test]
    fn test_traffic_source_with_velocity() {
        let source = TrafficSourceAction::with_velocity(
            5.0,
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

        assert_eq!(sink.rate.as_ref().unwrap().as_literal(), Some(&10.0));
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

        assert_eq!(sink.rate.as_ref().unwrap().as_literal(), Some(&12.0));
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
        let traffic_area = TrafficArea::rectangle(0.0, 0.0, 50.0, 50.0);

        let area = TrafficAreaAction::new(3, true, TrafficDistribution::default(), traffic_area);

        assert_eq!(area.number_of_entities.as_literal(), Some(&3));
        assert_eq!(area.continuous.as_literal(), Some(&true));
        assert_eq!(area.traffic_area.polygon.unwrap().position.len(), 4);
        assert!(area.traffic_area.road_range.is_empty());
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
        // XSD `TrafficStopAction` is an empty complexType.
        let action = TrafficStopAction::default();
        assert_eq!(action, TrafficStopAction {});

        let xml = quick_xml::se::to_string(&action).unwrap();
        let parsed: TrafficStopAction = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(parsed, action);
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

        assert!(!definition.vehicle_category_distribution.entries.is_empty());
        assert!(!definition.controller_distribution.entries.is_empty());
    }

    #[test]
    fn test_traffic_area_shapes() {
        let rect = TrafficArea::rectangle(10.0, 20.0, 30.0, 40.0);
        let polygon = rect.polygon.expect("rectangle should produce a Polygon");
        assert_eq!(polygon.position.len(), 4);

        let corner0 = polygon.position[0]
            .world_position
            .as_ref()
            .expect("expected a WorldPosition");
        assert_eq!(corner0.x.as_literal(), Some(&10.0));
        assert_eq!(corner0.y.as_literal(), Some(&20.0));

        let corner2 = polygon.position[2]
            .world_position
            .as_ref()
            .expect("expected a WorldPosition");
        assert_eq!(corner2.x.as_literal(), Some(&40.0)); // 10 + 30
        assert_eq!(corner2.y.as_literal(), Some(&60.0)); // 20 + 40
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
        assert_eq!(sink.rate.as_ref().unwrap().as_literal(), Some(&10.0));
        assert_eq!(sink.radius.as_literal(), Some(&50.0));

        let swarm = TrafficSwarmAction::default();
        assert_eq!(swarm.number_of_vehicles.as_literal(), Some(&20));
        assert_eq!(swarm.inner_radius.as_literal(), Some(&10.0));
        assert_eq!(swarm.semi_major_axis.as_literal(), Some(&100.0));
        assert_eq!(swarm.semi_minor_axis.as_literal(), Some(&50.0));

        let stop = TrafficStopAction::default();
        assert_eq!(stop, TrafficStopAction {});
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

    #[test]
    fn test_traffic_source_action_speed_round_trip() {
        let xml = r#"<TrafficSourceAction radius="5" rate="10" speed="15.0">
    <Position><WorldPosition x="0" y="0"/></Position>
</TrafficSourceAction>"#;
        let action: TrafficSourceAction = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(action.speed.clone().unwrap().as_literal(), Some(&15.0));

        let serialized = quick_xml::se::to_string(&action).unwrap();
        assert!(serialized.contains(r#"speed="15""#), "serialized: {serialized}");
        let reparsed: TrafficSourceAction = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(action, reparsed);
    }

    // TASK A/B/C: traffic-distribution subtree, TrafficArea, and the three
    // traffic actions that reference it.

    fn sample_entity_distribution() -> EntityDistribution {
        use crate::types::entities::{ScenarioObjectTemplate, Vehicle};

        EntityDistribution {
            entries: vec![crate::types::entities::EntityDistributionEntry::new(
                ScenarioObjectTemplate::new_vehicle(Vehicle::default()),
                1.0,
            )],
        }
    }

    #[test]
    fn test_traffic_distribution_round_trip() {
        let distribution = TrafficDistribution {
            traffic_distribution_entry: vec![
                TrafficDistributionEntry {
                    weight: Double::literal(0.6),
                    entity_distribution: sample_entity_distribution(),
                    properties: None,
                },
                TrafficDistributionEntry {
                    weight: Double::literal(0.4),
                    entity_distribution: sample_entity_distribution(),
                    properties: None,
                },
            ],
        };

        let xml = quick_xml::se::to_string(&distribution).unwrap();
        let reparsed: TrafficDistribution = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(distribution, reparsed);
        assert_eq!(reparsed.traffic_distribution_entry.len(), 2);
        assert_eq!(
            reparsed.traffic_distribution_entry[0].weight.as_literal(),
            Some(&0.6)
        );
    }

    #[test]
    fn test_traffic_area_polygon_round_trip() {
        use crate::types::positions::WorldPosition;

        let traffic_area = TrafficArea {
            polygon: Some(Polygon {
                position: vec![
                    Position {
                        world_position: Some(WorldPosition::new(0.0, 0.0)),
                        ..Position::empty()
                    },
                    Position {
                        world_position: Some(WorldPosition::new(10.0, 0.0)),
                        ..Position::empty()
                    },
                    Position {
                        world_position: Some(WorldPosition::new(10.0, 10.0)),
                        ..Position::empty()
                    },
                ],
            }),
            road_range: Vec::new(),
        };

        let xml = quick_xml::se::to_string(&traffic_area).unwrap();
        let reparsed: TrafficArea = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(traffic_area, reparsed);
        assert_eq!(reparsed.polygon.unwrap().position.len(), 3);
        assert!(reparsed.road_range.is_empty());
    }

    #[test]
    fn test_traffic_area_road_range_round_trip() {
        let traffic_area = TrafficArea {
            polygon: None,
            road_range: vec![RoadRange {
                length: Some(Double::literal(50.0)),
                road_cursor: vec![
                    RoadCursor {
                        road_id: OSString::literal("Road1".to_string()),
                        s: Some(Double::literal(0.0)),
                        lane: vec![Lane { id: Int::literal(-1) }],
                    },
                    RoadCursor {
                        road_id: OSString::literal("Road1".to_string()),
                        s: Some(Double::literal(50.0)),
                        lane: Vec::new(),
                    },
                ],
            }],
        };

        let xml = quick_xml::se::to_string(&traffic_area).unwrap();
        let reparsed: TrafficArea = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(traffic_area, reparsed);
        assert!(reparsed.polygon.is_none());
        assert_eq!(reparsed.road_range.len(), 1);
        assert_eq!(reparsed.road_range[0].road_cursor.len(), 2);
        assert_eq!(reparsed.road_range[0].road_cursor[0].lane[0].id.as_literal(), Some(&-1));
    }

    #[test]
    fn test_traffic_area_action_round_trip() {
        let action = TrafficAreaAction::new(
            5,
            true,
            TrafficDistribution {
                traffic_distribution_entry: vec![TrafficDistributionEntry {
                    weight: Double::literal(1.0),
                    entity_distribution: sample_entity_distribution(),
                    properties: None,
                }],
            },
            TrafficArea::rectangle(0.0, 0.0, 20.0, 20.0),
        );

        let xml = quick_xml::se::to_string(&action).unwrap();
        let reparsed: TrafficAreaAction = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(action, reparsed);
        assert_eq!(reparsed.number_of_entities.as_literal(), Some(&5));
        assert_eq!(reparsed.continuous.as_literal(), Some(&true));
    }

    #[test]
    fn test_traffic_source_action_radius_and_distribution_round_trip() {
        let action = TrafficSourceAction::new(
            8.0,
            12.0,
            Position {
                world_position: Some(crate::types::positions::WorldPosition::new(1.0, 2.0)),
                ..Position::empty()
            },
            TrafficDefinition::default(),
        )
        .with_traffic_distribution(TrafficDistribution {
            traffic_distribution_entry: vec![TrafficDistributionEntry {
                weight: Double::literal(1.0),
                entity_distribution: sample_entity_distribution(),
                properties: None,
            }],
        });

        let xml = quick_xml::se::to_string(&action).unwrap();
        let reparsed: TrafficSourceAction = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(action, reparsed);
        assert_eq!(reparsed.radius.as_literal(), Some(&8.0));
        assert!(reparsed.traffic_distribution.is_some());
    }

    #[test]
    fn test_traffic_swarm_action_speed_range_and_direction_round_trip() {
        let swarm = TrafficSwarmAction::new("Ego", 100.0, 50.0, 10)
            .with_initial_speed_range(Range {
                lower_limit: Double::literal(5.0),
                upper_limit: Double::literal(20.0),
            })
            .with_direction_of_travel_distribution(DirectionOfTravelDistribution {
                same: Double::literal(0.8),
                opposite: Double::literal(0.2),
            });

        let xml = quick_xml::se::to_string(&swarm).unwrap();
        let reparsed: TrafficSwarmAction = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(swarm, reparsed);
        assert_eq!(
            reparsed.initial_speed_range.unwrap().lower_limit.as_literal(),
            Some(&5.0)
        );
        assert_eq!(
            reparsed
                .direction_of_travel_distribution
                .unwrap()
                .same
                .as_literal(),
            Some(&0.8)
        );
    }
}
