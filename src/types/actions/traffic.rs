//! Traffic actions: sources and sinks that add and remove background traffic, swarms
//! that surround a central entity, area-based density control, and the signal actions
//! that drive intersections. `TrafficDefinition` describes the population each draws
//! from, as vehicle-category and controller distributions.
use crate::types::basic::{Boolean, Double, Int, MinVec, OSString, Range, UnsignedInt, Value};
use crate::types::catalogs::references::ControllerCatalogReference;
use crate::types::controllers::Controller;
use crate::types::entities::{EntityDistribution, Properties};
use crate::types::enums::VehicleCategory;
use crate::types::positions::Position;
use serde::{Deserialize, Serialize};

/// Spawns traffic at a position, at a given rate, within a given radius.
///
/// XSD `TrafficSourceAction`: `radius` and `rate` are `use="required"`; `velocity`,
/// `TrafficDefinition` and `TrafficDistribution` are optional.
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
    #[serde(
        rename = "TrafficDefinition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub traffic_definition: Option<TrafficDefinition>,
    #[serde(
        rename = "TrafficDistribution",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub traffic_distribution: Option<TrafficDistribution>,
}

/// Removes traffic that enters a radius around a position.
///
/// XSD `TrafficSinkAction`: only `radius` is `use="required"`; `rate` and
/// `TrafficDefinition` are optional.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficSinkAction {
    #[serde(rename = "@rate", default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<Double>,
    #[serde(rename = "@radius")]
    pub radius: Double,
    #[serde(rename = "Position")]
    pub position: Position,
    #[serde(
        rename = "TrafficDefinition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub traffic_definition: Option<TrafficDefinition>,
}

/// Maintains a swarm of vehicles in an elliptical band around a central entity.
///
/// The band runs between `inner_radius` and the ellipse given by `semi_major_axis`
/// and `semi_minor_axis`, displaced from the central object by `offset`. XSD
/// `TrafficSwarmAction` marks the five geometry attributes and `CentralObject`
/// required; everything else is optional.
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
    #[serde(
        rename = "TrafficDefinition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub traffic_definition: Option<TrafficDefinition>,
    #[serde(
        rename = "TrafficDistribution",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub traffic_distribution: Option<TrafficDistribution>,
    #[serde(
        rename = "InitialSpeedRange",
        default,
        skip_serializing_if = "Option::is_none"
    )]
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
///
/// XSD `TrafficSignalAction` (:2248-2253) is a choice of
/// `TrafficSignalControllerAction` | `TrafficSignalStateAction`. `$value`
/// reads the branch from the live reader by element name; `flatten` would
/// buffer the children into a map first, which breaks the moment a sequence
/// appears below the choice.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficSignalAction {
    #[serde(rename = "$value")]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TrafficStopAction {}

/// The population a traffic source or swarm draws from: which vehicle categories
/// appear and with what weight, and which controllers drive them.
///
/// XSD `TrafficDefinition` requires `name`, `VehicleCategoryDistribution` and
/// `ControllerDistribution`; `VehicleRoleDistribution` is optional.
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
///
/// XSD `VehicleRoleDistribution` (`:2535-2539`): sequence of
/// `VehicleRoleDistributionEntry`, `maxOccurs="unbounded"`, no `minOccurs`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VehicleRoleDistribution {
    #[serde(rename = "VehicleRoleDistributionEntry")]
    pub entries: MinVec<VehicleRoleDistributionEntry, 1>,
}

/// Vehicle role distribution entry
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VehicleRoleDistributionEntry {
    #[serde(rename = "@weight")]
    pub weight: Double,
    #[serde(rename = "@role")]
    pub role: Value<crate::types::enums::Role>,
}

/// Vehicle category distribution for traffic composition
///
/// XSD `VehicleCategoryDistribution` (`:2520-2524`): sequence of
/// `VehicleCategoryDistributionEntry`, `maxOccurs="unbounded"`, no `minOccurs`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VehicleCategoryDistribution {
    #[serde(rename = "VehicleCategoryDistributionEntry")]
    pub entries: MinVec<VehicleCategoryDistributionEntry, 1>,
}

/// Vehicle category distribution entry
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VehicleCategoryDistributionEntry {
    #[serde(rename = "@category")]
    pub category: Value<VehicleCategory>,
    #[serde(rename = "@weight")]
    pub weight: Double,
}

/// Controller distribution for traffic behavior
///
/// XSD `ControllerDistribution` (`:990-994`): sequence of
/// `ControllerDistributionEntry`, `maxOccurs="unbounded"`, no `minOccurs`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ControllerDistribution {
    #[serde(rename = "ControllerDistributionEntry")]
    pub entries: MinVec<ControllerDistributionEntry, 1>,
}

/// Controller distribution entry: choice of an inline `Controller` or a catalog
/// reference to one, weighted by `@weight`.
///
/// XSD `ControllerDistributionEntry` (`:995-1001`): a bare `xsd:choice` of `Controller` |
/// `CatalogReference`, no occurrence attributes, so exactly one branch is required,
/// alongside the required sibling attribute `@weight`. `$value` reads the branch from
/// the live reader by element name; parallel `Option` fields would let both branches
/// populate at once and both re-serialize, which no schema-valid document can express.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ControllerDistributionEntry {
    #[serde(rename = "@weight")]
    pub weight: Double,
    #[serde(rename = "$value")]
    pub choice: ControllerDistributionEntryChoice,
}

/// The branch selected by a `ControllerDistributionEntry`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ControllerDistributionEntryChoice {
    Controller(Controller),
    CatalogReference(ControllerCatalogReference),
}

/// Central swarm object specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CentralSwarmObject {
    #[serde(rename = "@entityRef")]
    pub entity_ref: OSString,
}

/// Traffic area definition as a choice of `Polygon` or a set of `RoadRange`s
///
/// XSD `TrafficArea` (`:2214-2218`): a bare `xsd:choice` of `Polygon` | `RoadRange`
/// (`maxOccurs="unbounded"` on the `RoadRange` branch), no occurrence attributes on the
/// choice itself, so exactly one branch is required. `$value` reads the branch from the
/// live reader by element name; the `RoadRange` variant holds the whole repeated
/// sequence, since a schema-valid document selecting this branch may carry more than one
/// `<RoadRange>` sibling.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficArea {
    #[serde(rename = "$value")]
    pub choice: TrafficAreaChoice,
}

/// The branch selected by a `TrafficArea`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum TrafficAreaChoice {
    Polygon(Polygon),
    RoadRange(Vec<RoadRange>),
}

/// Closed polygon area defined by at least three positions
///
/// XSD `Polygon` (`:1728-1732`): sequence of `Position`, `minOccurs="3"`,
/// unbounded — the crate's only field with a lower bound above two.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Polygon {
    #[serde(rename = "Position")]
    pub position: MinVec<Position, 3>,
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
    pub road_cursor: MinVec<RoadCursor, 2>,
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
/// `TrafficDistributionEntry`, `maxOccurs="unbounded"`, no `minOccurs`, so the
/// XSD default of 1 applies. An empty document already fails to parse (there
/// is no `#[serde(default)]` here), so the type-level bound closes the
/// serialization side: the crate can no longer construct a distribution
/// naming zero entries and emit it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficDistribution {
    #[serde(rename = "TrafficDistributionEntry")]
    pub traffic_distribution_entry: MinVec<TrafficDistributionEntry, 1>,
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
    #[serde(
        rename = "Properties",
        default,
        skip_serializing_if = "Option::is_none"
    )]
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

// `impl Default` removed for every fabricating type in this
// file: `TrafficSourceAction`, `TrafficSinkAction`, `TrafficSwarmAction`,
// `TrafficSignalAction` (a choice — its old default silently picked the
// `TrafficSignalStateAction` branch), `TrafficSignalStateAction`,
// `TrafficSignalControllerAction`, `TrafficSignalController`, `Phase`,
// `TrafficSignalState`, `TrafficSignalGroupState`, `TrafficDefinition`,
// `VehicleCategoryDistribution`, `ControllerDistribution`,
// `CentralSwarmObject`, `TrafficArea`, `Polygon`, `RoadRange`, `RoadCursor`,
// `Lane`, `TrafficDistribution`, `TrafficDistributionEntry`,
// `DirectionOfTravelDistribution`, `TrafficAreaAction`. Every attribute they
// touched is `use="required"` with no `default="…"` in `Schema/OpenSCENARIO.xsd`
// (checked: `TrafficSourceAction` :2300-2314, `TrafficSwarmAction` :2317-2334,
// `TrafficSignalController`/`Phase`/`TrafficSignalState*` :1714-1723 and
// :2258-2287, `TrafficDefinition` :2228-2235, `Lane` :1333-1335, `RoadCursor`
// :1926-1932, `RoadRange` :1949-1954, `DirectionOfTravelDistribution`
// :1063-1066, `TrafficAreaAction` :2220-2227). Use the named constructors
// below instead.

impl TrafficSourceAction {
    /// Create traffic source with radius, rate and position
    ///
    /// XSD `TrafficSourceAction` (`:2300-2314`): `TrafficDistribution` is
    /// `minOccurs="0"`, so it is optional here; add it with
    /// [`with_traffic_distribution`](Self::with_traffic_distribution).
    pub fn new(radius: f64, rate: f64, position: Position) -> Self {
        Self {
            radius: Double::literal(radius),
            rate: Double::literal(rate),
            velocity: None,
            speed: None,
            position,
            traffic_definition: None,
            traffic_distribution: None,
        }
    }

    /// Set the initial speed of spawned vehicles (`@speed`)
    pub fn with_speed(mut self, speed: f64) -> Self {
        self.speed = Some(Double::literal(speed));
        self
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
    /// Create a traffic definition with the required name, vehicle category
    /// distribution and controller distribution.
    ///
    /// XSD `TrafficDefinition` (`:2228-2235`) requires `@name` and both
    /// `VehicleCategoryDistribution` and `ControllerDistribution` children
    /// (`xsd:all`, no `minOccurs="0"`); `VehicleRoleDistribution` is the only
    /// optional member.
    pub fn new(
        name: impl Into<String>,
        vehicle_category_distribution: VehicleCategoryDistribution,
        controller_distribution: ControllerDistribution,
    ) -> Self {
        Self {
            name: OSString::literal(name.into()),
            vehicle_category_distribution,
            vehicle_role_distribution: None,
            controller_distribution,
        }
    }
}

impl VehicleCategoryDistribution {
    /// Create distribution with single category
    pub fn single_category(category: VehicleCategory, weight: f64) -> Self {
        Self {
            // A one-element vec always satisfies MIN = 1.
            entries: MinVec::new(vec![VehicleCategoryDistributionEntry {
                category: Value::Literal(category),
                weight: Double::literal(weight),
            }])
            .expect("one element"),
        }
    }

    /// Create distribution for mixed traffic (cars, trucks, vans)
    pub fn mixed_traffic() -> Self {
        Self {
            // A three-element literal vec always satisfies MIN = 1.
            entries: MinVec::new(vec![
                VehicleCategoryDistributionEntry {
                    category: Value::Literal(VehicleCategory::Car),
                    weight: Double::literal(0.7), // 70% cars
                },
                VehicleCategoryDistributionEntry {
                    category: Value::Literal(VehicleCategory::Truck),
                    weight: Double::literal(0.2), // 20% trucks
                },
                VehicleCategoryDistributionEntry {
                    category: Value::Literal(VehicleCategory::Van),
                    weight: Double::literal(0.1), // 10% vans
                },
            ])
            .expect("three elements"),
        }
    }

    /// Create distribution for urban traffic (mostly cars)
    pub fn urban_traffic() -> Self {
        Self {
            // A three-element literal vec always satisfies MIN = 1.
            entries: MinVec::new(vec![
                VehicleCategoryDistributionEntry {
                    category: Value::Literal(VehicleCategory::Car),
                    weight: Double::literal(0.85),
                },
                VehicleCategoryDistributionEntry {
                    category: Value::Literal(VehicleCategory::Bus),
                    weight: Double::literal(0.1),
                },
                VehicleCategoryDistributionEntry {
                    category: Value::Literal(VehicleCategory::Van),
                    weight: Double::literal(0.05),
                },
            ])
            .expect("three elements"),
        }
    }
}

impl ControllerDistribution {
    /// Create distribution with single controller
    pub fn single_controller(controller: String, weight: f64) -> Self {
        Self {
            // A one-element vec always satisfies MIN = 1.
            entries: MinVec::new(vec![ControllerDistributionEntry {
                weight: Double::literal(weight),
                choice: ControllerDistributionEntryChoice::Controller(Controller::new(
                    controller,
                    crate::types::enums::ControllerType::Movement,
                )),
            }])
            .expect("one element"),
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

        let corner = |cx: f64, cy: f64| Position::world(WorldPosition::new(cx, cy));

        Self {
            // A four-corner literal vec always satisfies MIN = 3.
            position: MinVec::new(vec![
                corner(x, y),
                corner(x + width, y),
                corner(x + width, y + height),
                corner(x, y + height),
            ])
            .expect("four corners"),
        }
    }
}

impl TrafficArea {
    /// Create a traffic area bounded by a rectangular polygon
    pub fn rectangle(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            choice: TrafficAreaChoice::Polygon(Polygon::rectangle(x, y, width, height)),
        }
    }
}

impl RoadCursor {
    /// Create a road cursor at a given road id, with no lane restriction
    ///
    /// XSD `RoadCursor` (`:1926-1932`): required `@roadId`, optional `@s` and
    /// `Lane` children.
    pub fn new(road_id: impl Into<String>) -> Self {
        Self {
            road_id: OSString::literal(road_id.into()),
            s: None,
            lane: Vec::new(),
        }
    }

    /// Set the `@s` coordinate along the road
    pub fn with_s(mut self, s: f64) -> Self {
        self.s = Some(Double::literal(s));
        self
    }

    /// Restrict the cursor to a set of lanes
    pub fn with_lanes(mut self, lanes: Vec<Lane>) -> Self {
        self.lane = lanes;
        self
    }
}

impl Lane {
    /// Create a lane reference by numeric id
    ///
    /// XSD `Lane` (`:1333-1335`): required `@id` (Int), no declared default.
    pub fn new(id: i32) -> Self {
        Self {
            id: Int::literal(id),
        }
    }
}

impl RoadRange {
    /// Create a road range from at least two road cursors.
    ///
    /// XSD `RoadRange` (`:1949-1954`): sequence of `RoadCursor`,
    /// `minOccurs="2"`; optional `@length`. Fails if fewer than two cursors are given.
    pub fn new(road_cursor: Vec<RoadCursor>) -> crate::error::Result<Self> {
        Ok(Self {
            length: None,
            road_cursor: MinVec::new(road_cursor)?,
        })
    }

    /// Set the `@length` attribute
    pub fn with_length(mut self, length: f64) -> Self {
        self.length = Some(Double::literal(length));
        self
    }
}

impl TrafficDistribution {
    /// Create a traffic distribution from its weighted entries.
    ///
    /// XSD `TrafficDistribution` (`:2236-2240`): sequence of
    /// `TrafficDistributionEntry`, `maxOccurs="unbounded"` with the default
    /// `minOccurs="1"` — at least one entry is required. Fails if `traffic_distribution_entry`
    /// is empty.
    pub fn new(
        traffic_distribution_entry: Vec<TrafficDistributionEntry>,
    ) -> crate::error::Result<Self> {
        Ok(Self {
            traffic_distribution_entry: MinVec::new(traffic_distribution_entry)?,
        })
    }
}

impl TrafficDistributionEntry {
    /// Create a weighted traffic distribution entry
    ///
    /// XSD `TrafficDistributionEntry` (`:2241-2247`): required `@weight` and
    /// `EntityDistribution`; `Properties` is the only optional member.
    pub fn new(weight: f64, entity_distribution: EntityDistribution) -> Self {
        Self {
            weight: Double::literal(weight),
            entity_distribution,
            properties: None,
        }
    }
}

impl DirectionOfTravelDistribution {
    /// Create a same/opposite direction-of-travel split
    ///
    /// XSD `DirectionOfTravelDistribution` (`:1063-1066`): both `@same` and
    /// `@opposite` are `use="required"` with no declared default.
    pub fn new(same: f64, opposite: f64) -> Self {
        Self {
            same: Double::literal(same),
            opposite: Double::literal(opposite),
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
        assert!(matches!(
            distribution.entries[0].choice,
            ControllerDistributionEntryChoice::Controller(_)
        ));
        assert_eq!(distribution.entries[1].weight.as_literal(), Some(&0.4));
        assert!(matches!(
            distribution.entries[1].choice,
            ControllerDistributionEntryChoice::CatalogReference(_)
        ));
    }

    #[test]
    fn test_controller_distribution_entry_zero_branches_rejected() {
        let xml = r#"<ControllerDistributionEntry weight="1"></ControllerDistributionEntry>"#;
        let err = quick_xml::de::from_str::<ControllerDistributionEntry>(xml).unwrap_err();
        assert!(
            err.to_string().contains("missing field `$value`"),
            "got: {err}"
        );
    }

    #[test]
    fn test_controller_distribution_entry_two_branches_rejected() {
        let xml = r#"<ControllerDistributionEntry weight="1"><Controller name="AI"/><CatalogReference catalogName="c" entryName="e"/></ControllerDistributionEntry>"#;
        let err = quick_xml::de::from_str::<ControllerDistributionEntry>(xml).unwrap_err();
        assert!(
            err.to_string().contains("duplicate field `$value`"),
            "got: {err}"
        );
    }

    #[test]
    fn test_controller_distribution_entry_controller_round_trip() {
        let xml = r#"<ControllerDistributionEntry weight="0.6"><Controller name="AIController"/></ControllerDistributionEntry>"#;
        let entry: ControllerDistributionEntry = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(entry.weight.as_literal(), Some(&0.6));
        match &entry.choice {
            ControllerDistributionEntryChoice::Controller(c) => {
                assert_eq!(c.name.as_literal(), Some(&"AIController".to_string()))
            }
            other => panic!("expected Controller, got {other:?}"),
        }
        let serialized = quick_xml::se::to_string(&entry).unwrap();
        assert_eq!(serialized, xml);
        let reparsed: ControllerDistributionEntry = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(entry, reparsed);
    }

    #[test]
    fn test_controller_distribution_entry_catalog_reference_round_trip() {
        let xml = r#"<ControllerDistributionEntry weight="0.4"><CatalogReference catalogName="ControllerCatalog" entryName="Manual"/></ControllerDistributionEntry>"#;
        let entry: ControllerDistributionEntry = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(entry.weight.as_literal(), Some(&0.4));
        assert!(matches!(
            entry.choice,
            ControllerDistributionEntryChoice::CatalogReference(_)
        ));
        let serialized = quick_xml::se::to_string(&entry).unwrap();
        assert_eq!(serialized, xml);
        let reparsed: ControllerDistributionEntry = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(entry, reparsed);
    }

    #[test]
    fn test_traffic_source_action_creation() {
        let source = TrafficSourceAction::new(5.0, 15.0, Position::world_origin());

        assert_eq!(source.radius.as_literal(), Some(&5.0));
        assert_eq!(source.rate.as_literal(), Some(&15.0));
        assert!(source.velocity.is_none());
    }

    #[test]
    fn test_traffic_constructors_write_only_current_forms() {
        let distribution = TrafficDistribution::new(vec![TrafficDistributionEntry {
            weight: Double::literal(1.0),
            entity_distribution: sample_entity_distribution(),
            properties: None,
        }])
        .unwrap();
        let range = Range {
            lower_limit: Double::literal(10.0),
            upper_limit: Double::literal(30.0),
        };
        let origin = || Position::world_origin();

        // (xml, markers that must be present)
        let cases: Vec<(String, &[&str])> = vec![
            (
                quick_xml::se::to_string(
                    &TrafficSourceAction::new(5.0, 15.0, origin())
                        .with_speed(20.0)
                        .with_traffic_distribution(distribution.clone()),
                )
                .unwrap(),
                &["<TrafficDistribution>", r#"speed="20""#],
            ),
            (
                quick_xml::se::to_string(
                    &TrafficSwarmAction::new("Ego", 100.0, 50.0, 5)
                        .with_traffic_distribution(distribution)
                        .with_initial_speed_range(range),
                )
                .unwrap(),
                &["<TrafficDistribution>", "<InitialSpeedRange"],
            ),
            (
                quick_xml::se::to_string(&TrafficSinkAction::new(10.0, 30.0, origin())).unwrap(),
                &["<TrafficSinkAction"],
            ),
        ];
        for (xml, present) in cases {
            for marker in present {
                assert!(xml.contains(marker), "missing {marker} in {xml}");
            }
            for deprecated in ["TrafficDefinition", "velocity"] {
                assert!(!xml.contains(deprecated), "found {deprecated} in {xml}");
            }
        }
    }

    #[test]
    fn test_traffic_sink_action_creation() {
        let sink = TrafficSinkAction::new(10.0, 30.0, Position::world_origin());

        assert_eq!(sink.rate.as_ref().unwrap().as_literal(), Some(&10.0));
        assert_eq!(sink.radius.as_literal(), Some(&30.0));
        assert!(sink.traffic_definition.is_none());
    }

    #[test]
    fn test_traffic_swarm_action_creation() {
        let swarm = TrafficSwarmAction::new("LeadVehicle", 100.0, 50.0, 15).with_inner_radius(5.0);

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
    fn test_vehicle_category_distribution() {
        let mixed = VehicleCategoryDistribution::mixed_traffic();
        assert_eq!(mixed.entries.len(), 3);
        assert!(mixed
            .entries
            .iter()
            .any(|e| matches!(e.category, Value::Literal(VehicleCategory::Car))));

        let urban = VehicleCategoryDistribution::urban_traffic();
        assert_eq!(urban.entries.len(), 3);
        assert_eq!(urban.entries[0].weight.as_literal().unwrap(), &0.85); // Mostly cars
    }

    #[test]
    fn test_traffic_definition_creation() {
        let vehicles = VehicleCategoryDistribution::urban_traffic();
        let controllers = ControllerDistribution::single_controller("AI1".to_string(), 1.0);

        let definition = TrafficDefinition::new("UrbanTraffic", vehicles, controllers);

        assert_eq!(
            definition.name.as_literal(),
            Some(&"UrbanTraffic".to_string())
        );
        assert_eq!(
            definition.vehicle_category_distribution,
            VehicleCategoryDistribution::urban_traffic()
        );
        assert_eq!(
            definition.controller_distribution,
            ControllerDistribution::single_controller("AI1".to_string(), 1.0)
        );
    }

    #[test]
    fn test_traffic_area_shapes() {
        let rect = TrafficArea::rectangle(10.0, 20.0, 30.0, 40.0);
        let polygon = match rect.choice {
            TrafficAreaChoice::Polygon(p) => p,
            other => panic!("expected Polygon, got {other:?}"),
        };
        assert_eq!(polygon.position.len(), 4);

        let corner0 = polygon.position[0]
            .world_position()
            .expect("expected a WorldPosition");
        assert_eq!(corner0.x.as_literal(), Some(&10.0));
        assert_eq!(corner0.y.as_literal(), Some(&20.0));

        let corner2 = polygon.position[2]
            .world_position()
            .expect("expected a WorldPosition");
        assert_eq!(corner2.x.as_literal(), Some(&40.0)); // 10 + 30
        assert_eq!(corner2.y.as_literal(), Some(&60.0)); // 20 + 40
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
    fn test_traffic_signal_construction() {
        // None of these types implement `Default` anymore
        // — every fabricated field (`name`, `duration`, ids, `state`) was
        // `use="required"` in the XSD with no declared default. Exercise the
        // explicit constructors instead.
        let controller = TrafficSignalController::new("TestController");
        assert_eq!(
            controller.name.as_literal(),
            Some(&"TestController".to_string())
        );
        assert!(controller.delay.is_none());
        assert!(controller.reference.is_none());
        assert_eq!(controller.phases.len(), 0);

        let phase = Phase::new("TestPhase", 30.0);
        assert_eq!(phase.name.as_literal(), Some(&"TestPhase".to_string()));
        assert_eq!(phase.duration.as_literal(), Some(&30.0));
        assert_eq!(phase.traffic_signal_states.len(), 0);
        assert!(phase.traffic_signal_group_state.is_none());

        let state = TrafficSignalState::new("signal_1", "green");
        assert_eq!(
            state.traffic_signal_id.as_literal(),
            Some(&"signal_1".to_string())
        );
        assert_eq!(state.state.as_literal(), Some(&"green".to_string()));

        let group_state = TrafficSignalGroupState::new("green");
        assert_eq!(group_state.state.as_literal(), Some(&"green".to_string()));

        let state_action = TrafficSignalStateAction::new("TestSignal", "green");
        assert_eq!(
            state_action.name.as_literal(),
            Some(&"TestSignal".to_string())
        );
        assert_eq!(state_action.state.as_literal(), Some(&"green".to_string()));

        let controller_action = TrafficSignalControllerAction::new("TestController", "Phase1");
        assert_eq!(
            controller_action.traffic_signal_controller_ref.as_literal(),
            Some(&"TestController".to_string())
        );
        assert_eq!(
            controller_action.phase_ref.as_literal(),
            Some(&"Phase1".to_string())
        );
    }

    // TRAFFIC SWARM ACTION TESTS

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
    fn test_traffic_swarm_builder_pattern() {
        let swarm = TrafficSwarmAction::new("BuilderTest", 120.0, 80.0, 10)
            .with_inner_radius(25.0)
            .with_offset(15.0)
            .with_central_swarm_object("NewCentralEntity");

        assert_eq!(swarm.inner_radius.as_literal().unwrap(), &25.0);
        assert_eq!(swarm.offset.as_literal().unwrap(), &15.0);
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
        assert!(
            serialized.contains(r#"speed="15""#),
            "serialized: {serialized}"
        );
        let reparsed: TrafficSourceAction = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(action, reparsed);
    }

    // TASK A/B/C: traffic-distribution subtree, TrafficArea, and the three
    // traffic actions that reference it.

    fn sample_entity_distribution() -> EntityDistribution {
        use crate::types::entities::{ScenarioObjectTemplate, Vehicle};

        EntityDistribution {
            entries: MinVec::new(vec![crate::types::entities::EntityDistributionEntry::new(
                ScenarioObjectTemplate::new_vehicle(Vehicle::new_car("TestVehicle".to_string())),
                1.0,
            )])
            .unwrap(),
        }
    }

    #[test]
    fn test_traffic_distribution_round_trip() {
        let distribution = TrafficDistribution {
            traffic_distribution_entry: MinVec::new(vec![
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
            ])
            .unwrap(),
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
            choice: TrafficAreaChoice::Polygon(Polygon {
                position: MinVec::new(vec![
                    Position::world(WorldPosition::new(0.0, 0.0)),
                    Position::world(WorldPosition::new(10.0, 0.0)),
                    Position::world(WorldPosition::new(10.0, 10.0)),
                ])
                .unwrap(),
            }),
        };

        let xml = quick_xml::se::to_string(&traffic_area).unwrap();
        let reparsed: TrafficArea = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(traffic_area, reparsed);
        match reparsed.choice {
            TrafficAreaChoice::Polygon(p) => assert_eq!(p.position.len(), 3),
            other => panic!("expected Polygon, got {other:?}"),
        }
    }

    #[test]
    fn test_traffic_area_road_range_round_trip() {
        let traffic_area = TrafficArea {
            choice: TrafficAreaChoice::RoadRange(vec![RoadRange {
                length: Some(Double::literal(50.0)),
                road_cursor: MinVec::new(vec![
                    RoadCursor {
                        road_id: OSString::literal("Road1".to_string()),
                        s: Some(Double::literal(0.0)),
                        lane: vec![Lane {
                            id: Int::literal(-1),
                        }],
                    },
                    RoadCursor {
                        road_id: OSString::literal("Road1".to_string()),
                        s: Some(Double::literal(50.0)),
                        lane: Vec::new(),
                    },
                ])
                .unwrap(),
            }]),
        };

        let xml = quick_xml::se::to_string(&traffic_area).unwrap();
        let reparsed: TrafficArea = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(traffic_area, reparsed);
        let road_ranges = match reparsed.choice {
            TrafficAreaChoice::RoadRange(rr) => rr,
            other => panic!("expected RoadRange, got {other:?}"),
        };
        assert_eq!(road_ranges.len(), 1);
        assert_eq!(road_ranges[0].road_cursor.len(), 2);
        assert_eq!(
            road_ranges[0].road_cursor[0].lane[0].id.as_literal(),
            Some(&-1)
        );
    }

    #[test]
    fn test_traffic_area_zero_branches_rejected() {
        let xml = "<TrafficArea></TrafficArea>";
        let err = quick_xml::de::from_str::<TrafficArea>(xml).unwrap_err();
        assert!(
            err.to_string().contains("missing field `$value`"),
            "got: {err}"
        );
    }

    #[test]
    fn test_traffic_area_polygon_and_road_range_together_rejected() {
        // Two *different* branches selected at once — the choice cardinality
        // violation, not to be confused with a single `RoadRange` branch
        // that legitimately repeats.
        let xml = concat!(
            "<TrafficArea>",
            "<Polygon>",
            r#"<Position><WorldPosition x="0" y="0"/></Position>"#,
            r#"<Position><WorldPosition x="1" y="0"/></Position>"#,
            r#"<Position><WorldPosition x="1" y="1"/></Position>"#,
            "</Polygon>",
            r#"<RoadRange><RoadCursor roadId="R1"/><RoadCursor roadId="R2"/></RoadRange>"#,
            "</TrafficArea>"
        );
        let err = quick_xml::de::from_str::<TrafficArea>(xml).unwrap_err();
        assert!(
            err.to_string().contains("duplicate field `$value`"),
            "got: {err}"
        );
    }

    #[test]
    fn test_traffic_area_two_road_ranges_is_a_single_valid_branch() {
        // A `RoadRange` branch that repeats is one branch selected twice,
        // not two branches — the schema's `maxOccurs="unbounded"` on this
        // element, not a second choice member. This must be accepted, and
        // the serialized bytes must equal the source document: a `Vec`
        // directly beneath an externally tagged enum is read from the live
        // parser rather than buffered into a map, so nothing here should
        // behave like the `#[serde(flatten)]` failure this campaign exists
        // to remove.
        let xml = concat!(
            "<TrafficArea>",
            r#"<RoadRange><RoadCursor roadId="R1"/><RoadCursor roadId="R2"/></RoadRange>"#,
            r#"<RoadRange><RoadCursor roadId="R3"/><RoadCursor roadId="R4"/></RoadRange>"#,
            "</TrafficArea>"
        );
        let area: TrafficArea = quick_xml::de::from_str(xml).unwrap();
        let road_ranges = match &area.choice {
            TrafficAreaChoice::RoadRange(rr) => rr,
            other => panic!("expected RoadRange, got {other:?}"),
        };
        assert_eq!(road_ranges.len(), 2);
        assert_eq!(
            road_ranges[0].road_cursor[0].road_id.as_literal(),
            Some(&"R1".to_string())
        );
        assert_eq!(
            road_ranges[1].road_cursor[1].road_id.as_literal(),
            Some(&"R4".to_string())
        );

        let serialized = quick_xml::se::to_string(&area).unwrap();
        assert_eq!(serialized, xml, "byte-exact round trip");
        let reparsed: TrafficArea = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(area, reparsed);
    }

    #[test]
    fn test_traffic_area_action_round_trip() {
        let action = TrafficAreaAction::new(
            5,
            true,
            TrafficDistribution::new(vec![TrafficDistributionEntry::new(
                0.7,
                sample_entity_distribution(),
            )])
            .unwrap(),
            TrafficArea::rectangle(0.0, 0.0, 20.0, 20.0),
        );

        let xml = quick_xml::se::to_string(&action).unwrap();
        let reparsed: TrafficAreaAction = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(action, reparsed);
        assert_eq!(reparsed.number_of_entities.as_literal(), Some(&5));
        assert_eq!(reparsed.continuous.as_literal(), Some(&true));
        let entries = &reparsed.traffic_distribution.traffic_distribution_entry;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].weight.as_literal(), Some(&0.7));
        assert!(entries[0].properties.is_none());

        // XSD `TrafficDistribution` requires at least one entry.
        assert!(TrafficDistribution::new(Vec::new()).is_err());
    }

    #[test]
    fn test_traffic_source_action_radius_and_distribution_round_trip() {
        let action = TrafficSourceAction::new(
            8.0,
            12.0,
            Position::world(crate::types::positions::WorldPosition::new(1.0, 2.0)),
        )
        .with_traffic_distribution(TrafficDistribution {
            traffic_distribution_entry: MinVec::new(vec![TrafficDistributionEntry {
                weight: Double::literal(1.0),
                entity_distribution: sample_entity_distribution(),
                properties: None,
            }])
            .unwrap(),
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
            reparsed
                .initial_speed_range
                .unwrap()
                .lower_limit
                .as_literal(),
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
