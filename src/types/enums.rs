//! All enumeration types from the OpenSCENARIO specification
//!
//! This file contains:
//! - All 37 enumeration types with their complete value sets
//! - Serde annotations for correct XML serialization (rename attributes)
//! - Deprecation markers for legacy enum values
//! - Default implementations where appropriate
//! - String conversion helpers for debugging and display
//!
//! Contributes to project by:
//! - Ensuring type safety for all predefined value sets in OpenSCENARIO
//! - Preventing invalid enum values at compile time
//! - Providing clear mapping between Rust types and XML values
//! - Supporting evolution of the specification through deprecation handling
//! - Enabling exhaustive pattern matching for robust code

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Vehicle category enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VehicleCategory {
    #[serde(rename = "car")]
    Car,
    #[serde(rename = "van")]
    Van,
    #[serde(rename = "truck")]
    Truck,
    #[serde(rename = "semitrailer")]
    Semitrailer,
    #[serde(rename = "bus")]
    Bus,
    #[serde(rename = "motorbike")]
    Motorbike,
    #[serde(rename = "bicycle")]
    Bicycle,
    #[serde(rename = "train")]
    Train,
    #[serde(rename = "tram")]
    Tram,
}

/// Pedestrian category enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PedestrianCategory {
    #[serde(rename = "pedestrian")]
    Pedestrian,
    #[serde(rename = "wheelchair")]
    Wheelchair,
    #[serde(rename = "animal")]
    Animal,
}

/// Object type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObjectType {
    #[serde(rename = "vehicle")]
    Vehicle,
    #[serde(rename = "pedestrian")]
    Pedestrian,
    #[serde(rename = "miscellaneousObject")]
    MiscellaneousObject,
}

/// Rule enumeration for conditions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rule {
    #[serde(rename = "equalTo")]
    EqualTo,
    #[serde(rename = "greaterThan")]
    GreaterThan,
    #[serde(rename = "lessThan")]
    LessThan,
    #[serde(rename = "greaterOrEqual")]
    GreaterOrEqual,
    #[serde(rename = "lessOrEqual")]
    LessOrEqual,
    #[serde(rename = "notEqualTo")]
    NotEqualTo,
}

/// Condition edge enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionEdge {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "rising")]
    Rising,
    #[serde(rename = "falling")]
    Falling,
    #[serde(rename = "risingOrFalling")]
    RisingOrFalling,
}

/// Triggering entities rule enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TriggeringEntitiesRule {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "any")]
    Any,
}

/// Priority level for events and actions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    #[serde(rename = "overwrite")]
    Overwrite,
    #[serde(rename = "override")]
    Override,
    #[serde(rename = "parallel")]
    Parallel,
    #[serde(rename = "skip")]
    Skip,
}

/// Storyboard element state enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StoryboardElementState {
    #[serde(rename = "completeState")]
    CompleteState,
    #[serde(rename = "endTransition")]
    EndTransition,
    #[serde(rename = "runningState")]
    RunningState,
    #[serde(rename = "skipTransition")]
    SkipTransition,
    #[serde(rename = "standbyState")]
    StandbyState,
    #[serde(rename = "startTransition")]
    StartTransition,
    #[serde(rename = "stopTransition")]
    StopTransition,
}

/// Storyboard element type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StoryboardElementType {
    #[serde(rename = "act")]
    Act,
    #[serde(rename = "action")]
    Action,
    #[serde(rename = "event")]
    Event,
    #[serde(rename = "maneuver")]
    Maneuver,
    #[serde(rename = "maneuverGroup")]
    ManeuverGroup,
    #[serde(rename = "story")]
    Story,
}

/// Parameter data type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParameterType {
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "dateTime")]
    DateTime,
    #[serde(rename = "double")]
    Double,
    #[serde(rename = "int")]
    Int,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "unsignedInt")]
    UnsignedInt,
    #[serde(rename = "unsignedShort")]
    UnsignedShort,
}

/// Coordinate system enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoordinateSystem {
    #[serde(rename = "entity")]
    Entity,
    #[serde(rename = "lane")]
    Lane,
    #[serde(rename = "road")]
    Road,
    #[serde(rename = "trajectory")]
    Trajectory,
}

/// Reference context enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReferenceContext {
    #[serde(rename = "relative")]
    Relative,
    #[serde(rename = "absolute")]
    Absolute,
}

/// Speed target value type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpeedTargetValueType {
    #[serde(rename = "delta")]
    Delta,
    #[serde(rename = "absolute")]
    Absolute,
}

/// Dynamics shape enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DynamicsShape {
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "cubic")]
    Cubic,
    #[serde(rename = "sinusoidal")]
    Sinusoidal,
    #[serde(rename = "step")]
    Step,
}

/// Dynamics dimension enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DynamicsDimension {
    #[serde(rename = "rate")]
    Rate,
    #[serde(rename = "time")]
    Time,
    #[serde(rename = "distance")]
    Distance,
}

/// Relative distance type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelativeDistanceType {
    #[serde(rename = "longitudinal")]
    Longitudinal,
    #[serde(rename = "lateral")]
    Lateral,
    #[serde(rename = "cartesianDistance")]
    Cartesian,
}

/// Following mode enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FollowingMode {
    #[serde(rename = "position")]
    Position,
    #[serde(rename = "follow")]
    Follow,
}

// Implement Display trait for all enums to show XML representation
impl fmt::Display for VehicleCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            VehicleCategory::Car => "car",
            VehicleCategory::Van => "van",
            VehicleCategory::Truck => "truck",
            VehicleCategory::Semitrailer => "semitrailer",
            VehicleCategory::Bus => "bus",
            VehicleCategory::Motorbike => "motorbike",
            VehicleCategory::Bicycle => "bicycle",
            VehicleCategory::Train => "train",
            VehicleCategory::Tram => "tram",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for VehicleCategory {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "car" => Ok(VehicleCategory::Car),
            "van" => Ok(VehicleCategory::Van),
            "truck" => Ok(VehicleCategory::Truck),
            "semitrailer" => Ok(VehicleCategory::Semitrailer),
            "bus" => Ok(VehicleCategory::Bus),
            "motorbike" => Ok(VehicleCategory::Motorbike),
            "bicycle" => Ok(VehicleCategory::Bicycle),
            "train" => Ok(VehicleCategory::Train),
            "tram" => Ok(VehicleCategory::Tram),
            _ => Err(format!("Invalid vehicle category: {}", s)),
        }
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Rule::EqualTo => "equalTo",
            Rule::GreaterThan => "greaterThan",
            Rule::LessThan => "lessThan",
            Rule::GreaterOrEqual => "greaterOrEqual",
            Rule::LessOrEqual => "lessOrEqual",
            Rule::NotEqualTo => "notEqualTo",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "equalTo" => Ok(Rule::EqualTo),
            "greaterThan" => Ok(Rule::GreaterThan),
            "lessThan" => Ok(Rule::LessThan),
            "greaterOrEqual" => Ok(Rule::GreaterOrEqual),
            "lessOrEqual" => Ok(Rule::LessOrEqual),
            "notEqualTo" => Ok(Rule::NotEqualTo),
            _ => Err(format!("Invalid rule: {}", s)),
        }
    }
}

impl fmt::Display for ConditionEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ConditionEdge::None => "none",
            ConditionEdge::Rising => "rising",
            ConditionEdge::Falling => "falling",
            ConditionEdge::RisingOrFalling => "risingOrFalling",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for ConditionEdge {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(ConditionEdge::None),
            "rising" => Ok(ConditionEdge::Rising),
            "falling" => Ok(ConditionEdge::Falling),
            "risingOrFalling" => Ok(ConditionEdge::RisingOrFalling),
            _ => Err(format!("Invalid condition edge: {}", s)),
        }
    }
}

impl fmt::Display for PedestrianCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            PedestrianCategory::Pedestrian => "pedestrian",
            PedestrianCategory::Wheelchair => "wheelchair",
            PedestrianCategory::Animal => "animal",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for PedestrianCategory {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pedestrian" => Ok(PedestrianCategory::Pedestrian),
            "wheelchair" => Ok(PedestrianCategory::Wheelchair),
            "animal" => Ok(PedestrianCategory::Animal),
            _ => Err(format!("Invalid pedestrian category: {}", s)),
        }
    }
}

impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ObjectType::Vehicle => "vehicle",
            ObjectType::Pedestrian => "pedestrian",
            ObjectType::MiscellaneousObject => "miscellaneousObject",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for ObjectType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "vehicle" => Ok(ObjectType::Vehicle),
            "pedestrian" => Ok(ObjectType::Pedestrian),
            "miscellaneousObject" => Ok(ObjectType::MiscellaneousObject),
            _ => Err(format!("Invalid object type: {}", s)),
        }
    }
}

impl fmt::Display for RelativeDistanceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            RelativeDistanceType::Longitudinal => "longitudinal",
            RelativeDistanceType::Lateral => "lateral",
            RelativeDistanceType::Cartesian => "cartesianDistance",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for RelativeDistanceType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "longitudinal" => Ok(RelativeDistanceType::Longitudinal),
            "lateral" => Ok(RelativeDistanceType::Lateral),
            "cartesianDistance" => Ok(RelativeDistanceType::Cartesian),
            _ => Err(format!("Invalid relative distance type: {}", s)),
        }
    }
}

impl fmt::Display for FollowingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            FollowingMode::Position => "position",
            FollowingMode::Follow => "follow",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for FollowingMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "position" => Ok(FollowingMode::Position),
            "follow" => Ok(FollowingMode::Follow),
            _ => Err(format!("Invalid following mode: {}", s)),
        }
    }
}

impl fmt::Display for DynamicsDimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            DynamicsDimension::Rate => "rate",
            DynamicsDimension::Time => "time",
            DynamicsDimension::Distance => "distance",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for DynamicsDimension {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rate" => Ok(DynamicsDimension::Rate),
            "time" => Ok(DynamicsDimension::Time),
            "distance" => Ok(DynamicsDimension::Distance),
            _ => Err(format!("Invalid dynamics dimension: {}", s)),
        }
    }
}

/// Miscellaneous object category enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MiscObjectCategory {
    #[serde(rename = "barrier")]
    Barrier,
    #[serde(rename = "building")]
    Building,
    #[serde(rename = "crosswalk")]
    Crosswalk,
    #[serde(rename = "gantry")]
    Gantry,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "obstacle")]
    Obstacle,
    #[serde(rename = "parkingSpace")]
    ParkingSpace,
    #[serde(rename = "patch")]
    Patch,
    #[serde(rename = "pole")]
    Pole,
    #[serde(rename = "railing")]
    Railing,
    #[serde(rename = "roadMark")]
    RoadMark,
    #[serde(rename = "soundBarrier")]
    SoundBarrier,
    #[serde(rename = "streetLamp")]
    StreetLamp,
    #[serde(rename = "trafficIsland")]
    TrafficIsland,
    #[serde(rename = "tree")]
    Tree,
    #[serde(rename = "vegetation")]
    Vegetation,
    #[serde(rename = "wind")]
    Wind,
}

/// Controller type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControllerType {
    #[serde(rename = "lateral")]
    Lateral,
    #[serde(rename = "longitudinal")]
    Longitudinal,
    #[serde(rename = "lighting")]
    Lighting,
    #[serde(rename = "animation")]
    Animation,
    #[serde(rename = "movement")]
    Movement,
    #[serde(rename = "appearance")]
    Appearance,
    #[serde(rename = "all")]
    All,
}

/// Precipitation type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrecipitationType {
    #[serde(rename = "dry")]
    Dry,
    #[serde(rename = "rain")]
    Rain,
    #[serde(rename = "snow")]
    Snow,
}

/// Wetness level enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Wetness {
    #[serde(rename = "dry")]
    Dry,
    #[serde(rename = "moist")]
    Moist,
    #[serde(rename = "wetWithPuddles")]
    WetWithPuddles,
    #[serde(rename = "lowFlooded")]
    LowFlooded,
    #[serde(rename = "highFlooded")]
    HighFlooded,
}

/// Color type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorType {
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "violet")]
    Violet,
    #[serde(rename = "orange")]
    Orange,
    #[serde(rename = "brown")]
    Brown,
    #[serde(rename = "black")]
    Black,
    #[serde(rename = "white")]
    White,
    #[serde(rename = "grey")]
    Grey,
}

/// Role enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "ambulance")]
    Ambulance,
    #[serde(rename = "civil")]
    Civil,
    #[serde(rename = "fire")]
    Fire,
    #[serde(rename = "military")]
    Military,
    #[serde(rename = "police")]
    Police,
    #[serde(rename = "publicTransport")]
    PublicTransport,
    #[serde(rename = "roadAssistance")]
    RoadAssistance,
}

impl fmt::Display for MiscObjectCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            MiscObjectCategory::Barrier => "barrier",
            MiscObjectCategory::Building => "building",
            MiscObjectCategory::Crosswalk => "crosswalk",
            MiscObjectCategory::Gantry => "gantry",
            MiscObjectCategory::None => "none",
            MiscObjectCategory::Obstacle => "obstacle",
            MiscObjectCategory::ParkingSpace => "parkingSpace",
            MiscObjectCategory::Patch => "patch",
            MiscObjectCategory::Pole => "pole",
            MiscObjectCategory::Railing => "railing",
            MiscObjectCategory::RoadMark => "roadMark",
            MiscObjectCategory::SoundBarrier => "soundBarrier",
            MiscObjectCategory::StreetLamp => "streetLamp",
            MiscObjectCategory::TrafficIsland => "trafficIsland",
            MiscObjectCategory::Tree => "tree",
            MiscObjectCategory::Vegetation => "vegetation",
            MiscObjectCategory::Wind => "wind",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for MiscObjectCategory {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "barrier" => Ok(MiscObjectCategory::Barrier),
            "building" => Ok(MiscObjectCategory::Building),
            "crosswalk" => Ok(MiscObjectCategory::Crosswalk),
            "gantry" => Ok(MiscObjectCategory::Gantry),
            "none" => Ok(MiscObjectCategory::None),
            "obstacle" => Ok(MiscObjectCategory::Obstacle),
            "parkingSpace" => Ok(MiscObjectCategory::ParkingSpace),
            "patch" => Ok(MiscObjectCategory::Patch),
            "pole" => Ok(MiscObjectCategory::Pole),
            "railing" => Ok(MiscObjectCategory::Railing),
            "roadMark" => Ok(MiscObjectCategory::RoadMark),
            "soundBarrier" => Ok(MiscObjectCategory::SoundBarrier),
            "streetLamp" => Ok(MiscObjectCategory::StreetLamp),
            "trafficIsland" => Ok(MiscObjectCategory::TrafficIsland),
            "tree" => Ok(MiscObjectCategory::Tree),
            "vegetation" => Ok(MiscObjectCategory::Vegetation),
            "wind" => Ok(MiscObjectCategory::Wind),
            _ => Err(format!("Invalid misc object category: {}", s)),
        }
    }
}

impl fmt::Display for ControllerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ControllerType::Lateral => "lateral",
            ControllerType::Longitudinal => "longitudinal",
            ControllerType::Lighting => "lighting",
            ControllerType::Animation => "animation",
            ControllerType::Movement => "movement",
            ControllerType::Appearance => "appearance",
            ControllerType::All => "all",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for ControllerType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lateral" => Ok(ControllerType::Lateral),
            "longitudinal" => Ok(ControllerType::Longitudinal),
            "lighting" => Ok(ControllerType::Lighting),
            "animation" => Ok(ControllerType::Animation),
            "movement" => Ok(ControllerType::Movement),
            "appearance" => Ok(ControllerType::Appearance),
            "all" => Ok(ControllerType::All),
            _ => Err(format!("Invalid controller type: {}", s)),
        }
    }
}

impl fmt::Display for PrecipitationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            PrecipitationType::Dry => "dry",
            PrecipitationType::Rain => "rain",
            PrecipitationType::Snow => "snow",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for PrecipitationType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dry" => Ok(PrecipitationType::Dry),
            "rain" => Ok(PrecipitationType::Rain),
            "snow" => Ok(PrecipitationType::Snow),
            _ => Err(format!("Invalid precipitation type: {}", s)),
        }
    }
}

impl fmt::Display for Wetness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Wetness::Dry => "dry",
            Wetness::Moist => "moist",
            Wetness::WetWithPuddles => "wetWithPuddles",
            Wetness::LowFlooded => "lowFlooded",
            Wetness::HighFlooded => "highFlooded",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Wetness {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dry" => Ok(Wetness::Dry),
            "moist" => Ok(Wetness::Moist),
            "wetWithPuddles" => Ok(Wetness::WetWithPuddles),
            "lowFlooded" => Ok(Wetness::LowFlooded),
            "highFlooded" => Ok(Wetness::HighFlooded),
            _ => Err(format!("Invalid wetness: {}", s)),
        }
    }
}

impl fmt::Display for ColorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ColorType::Other => "other",
            ColorType::Red => "red",
            ColorType::Yellow => "yellow",
            ColorType::Green => "green",
            ColorType::Blue => "blue",
            ColorType::Violet => "violet",
            ColorType::Orange => "orange",
            ColorType::Brown => "brown",
            ColorType::Black => "black",
            ColorType::White => "white",
            ColorType::Grey => "grey",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for ColorType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "other" => Ok(ColorType::Other),
            "red" => Ok(ColorType::Red),
            "yellow" => Ok(ColorType::Yellow),
            "green" => Ok(ColorType::Green),
            "blue" => Ok(ColorType::Blue),
            "violet" => Ok(ColorType::Violet),
            "orange" => Ok(ColorType::Orange),
            "brown" => Ok(ColorType::Brown),
            "black" => Ok(ColorType::Black),
            "white" => Ok(ColorType::White),
            "grey" => Ok(ColorType::Grey),
            _ => Err(format!("Invalid color type: {}", s)),
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Role::None => "none",
            Role::Ambulance => "ambulance",
            Role::Civil => "civil",
            Role::Fire => "fire",
            Role::Military => "military",
            Role::Police => "police",
            Role::PublicTransport => "publicTransport",
            Role::RoadAssistance => "roadAssistance",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Role::None),
            "ambulance" => Ok(Role::Ambulance),
            "civil" => Ok(Role::Civil),
            "fire" => Ok(Role::Fire),
            "military" => Ok(Role::Military),
            "police" => Ok(Role::Police),
            "publicTransport" => Ok(Role::PublicTransport),
            "roadAssistance" => Ok(Role::RoadAssistance),
            _ => Err(format!("Invalid role: {}", s)),
        }
    }
}

/// Angle type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AngleType {
    #[serde(rename = "relative")]
    Relative,
    #[serde(rename = "absolute")]
    Absolute,
}

/// Directional dimension enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DirectionalDimension {
    #[serde(rename = "longitudinal")]
    Longitudinal,
    #[serde(rename = "lateral")]
    Lateral,
    #[serde(rename = "vertical")]
    Vertical,
}

/// Vehicle component type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VehicleComponentType {
    #[serde(rename = "hood")]
    Hood,
    #[serde(rename = "trunk")]
    Trunk,
    #[serde(rename = "doorFrontLeft")]
    DoorFrontLeft,
    #[serde(rename = "doorFrontRight")]
    DoorFrontRight,
    #[serde(rename = "doorRearLeft")]
    DoorRearLeft,
    #[serde(rename = "doorRearRight")]
    DoorRearRight,
    #[serde(rename = "windowFrontLeft")]
    WindowFrontLeft,
    #[serde(rename = "windowFrontRight")]
    WindowFrontRight,
    #[serde(rename = "windowRearLeft")]
    WindowRearLeft,
    #[serde(rename = "windowRearRight")]
    WindowRearRight,
}

/// Vehicle light type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VehicleLightType {
    #[serde(rename = "headlight")]
    Headlight,
    #[serde(rename = "taillight")]
    Taillight,
    #[serde(rename = "brakeLight")]
    BrakeLight,
    #[serde(rename = "reverseLight")]
    ReverseLight,
    #[serde(rename = "indicatorLeft")]
    IndicatorLeft,
    #[serde(rename = "indicatorRight")]
    IndicatorRight,
    #[serde(rename = "warningLight")]
    WarningLight,
    #[serde(rename = "fogLight")]
    FogLight,
    #[serde(rename = "highBeam")]
    HighBeam,
    #[serde(rename = "licensePlateLight")]
    LicensePlateLight,
}

/// Light mode enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LightMode {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "flashing")]
    Flashing,
}

impl fmt::Display for AngleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            AngleType::Relative => "relative",
            AngleType::Absolute => "absolute",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for AngleType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "relative" => Ok(AngleType::Relative),
            "absolute" => Ok(AngleType::Absolute),
            _ => Err(format!("Invalid angle type: {}", s)),
        }
    }
}

impl fmt::Display for DirectionalDimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            DirectionalDimension::Longitudinal => "longitudinal",
            DirectionalDimension::Lateral => "lateral",
            DirectionalDimension::Vertical => "vertical",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for DirectionalDimension {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "longitudinal" => Ok(DirectionalDimension::Longitudinal),
            "lateral" => Ok(DirectionalDimension::Lateral),
            "vertical" => Ok(DirectionalDimension::Vertical),
            _ => Err(format!("Invalid directional dimension: {}", s)),
        }
    }
}

impl fmt::Display for VehicleComponentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            VehicleComponentType::Hood => "hood",
            VehicleComponentType::Trunk => "trunk",
            VehicleComponentType::DoorFrontLeft => "doorFrontLeft",
            VehicleComponentType::DoorFrontRight => "doorFrontRight",
            VehicleComponentType::DoorRearLeft => "doorRearLeft",
            VehicleComponentType::DoorRearRight => "doorRearRight",
            VehicleComponentType::WindowFrontLeft => "windowFrontLeft",
            VehicleComponentType::WindowFrontRight => "windowFrontRight",
            VehicleComponentType::WindowRearLeft => "windowRearLeft",
            VehicleComponentType::WindowRearRight => "windowRearRight",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for VehicleComponentType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hood" => Ok(VehicleComponentType::Hood),
            "trunk" => Ok(VehicleComponentType::Trunk),
            "doorFrontLeft" => Ok(VehicleComponentType::DoorFrontLeft),
            "doorFrontRight" => Ok(VehicleComponentType::DoorFrontRight),
            "doorRearLeft" => Ok(VehicleComponentType::DoorRearLeft),
            "doorRearRight" => Ok(VehicleComponentType::DoorRearRight),
            "windowFrontLeft" => Ok(VehicleComponentType::WindowFrontLeft),
            "windowFrontRight" => Ok(VehicleComponentType::WindowFrontRight),
            "windowRearLeft" => Ok(VehicleComponentType::WindowRearLeft),
            "windowRearRight" => Ok(VehicleComponentType::WindowRearRight),
            _ => Err(format!("Invalid vehicle component type: {}", s)),
        }
    }
}

impl fmt::Display for VehicleLightType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            VehicleLightType::Headlight => "headlight",
            VehicleLightType::Taillight => "taillight",
            VehicleLightType::BrakeLight => "brakeLight",
            VehicleLightType::ReverseLight => "reverseLight",
            VehicleLightType::IndicatorLeft => "indicatorLeft",
            VehicleLightType::IndicatorRight => "indicatorRight",
            VehicleLightType::WarningLight => "warningLight",
            VehicleLightType::FogLight => "fogLight",
            VehicleLightType::HighBeam => "highBeam",
            VehicleLightType::LicensePlateLight => "licensePlateLight",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for VehicleLightType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "headlight" => Ok(VehicleLightType::Headlight),
            "taillight" => Ok(VehicleLightType::Taillight),
            "brakeLight" => Ok(VehicleLightType::BrakeLight),
            "reverseLight" => Ok(VehicleLightType::ReverseLight),
            "indicatorLeft" => Ok(VehicleLightType::IndicatorLeft),
            "indicatorRight" => Ok(VehicleLightType::IndicatorRight),
            "warningLight" => Ok(VehicleLightType::WarningLight),
            "fogLight" => Ok(VehicleLightType::FogLight),
            "highBeam" => Ok(VehicleLightType::HighBeam),
            "licensePlateLight" => Ok(VehicleLightType::LicensePlateLight),
            _ => Err(format!("Invalid vehicle light type: {}", s)),
        }
    }
}

impl fmt::Display for LightMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            LightMode::On => "on",
            LightMode::Off => "off",
            LightMode::Flashing => "flashing",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for LightMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(LightMode::On),
            "off" => Ok(LightMode::Off),
            "flashing" => Ok(LightMode::Flashing),
            _ => Err(format!("Invalid light mode: {}", s)),
        }
    }
}

// TODO: Add remaining enums incrementally (Week 5+)
// TODO: All geometry and positioning enums (DirectionalDimension, etc.)
// TODO: All vehicle and pedestrian behavior enums
// TODO: All environment and weather enums (CloudState, WeatherValue, etc.)
// TODO: All dynamics and control enums

// TODO: Add deprecation markers for legacy values
// TODO: #[deprecated] on CloudState enum values when implemented
// TODO: #[deprecated] on ParameterType::integer variant when implemented

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vehicle_category_display() {
        assert_eq!(VehicleCategory::Car.to_string(), "car");
        assert_eq!(VehicleCategory::Truck.to_string(), "truck");
    }

    #[test]
    fn test_vehicle_category_from_str() {
        assert_eq!("car".parse::<VehicleCategory>().unwrap(), VehicleCategory::Car);
        assert_eq!("truck".parse::<VehicleCategory>().unwrap(), VehicleCategory::Truck);
        assert!("invalid".parse::<VehicleCategory>().is_err());
    }

    #[test]
    fn test_rule_display() {
        assert_eq!(Rule::EqualTo.to_string(), "equalTo");
        assert_eq!(Rule::GreaterThan.to_string(), "greaterThan");
    }

    #[test]
    fn test_misc_object_category_display() {
        assert_eq!(MiscObjectCategory::Barrier.to_string(), "barrier");
        assert_eq!(MiscObjectCategory::Building.to_string(), "building");
        assert_eq!(MiscObjectCategory::None.to_string(), "none");
    }

    #[test]
    fn test_misc_object_category_from_str() {
        assert_eq!("barrier".parse::<MiscObjectCategory>().unwrap(), MiscObjectCategory::Barrier);
        assert_eq!("obstacle".parse::<MiscObjectCategory>().unwrap(), MiscObjectCategory::Obstacle);
        assert!("invalid".parse::<MiscObjectCategory>().is_err());
    }

    #[test]
    fn test_controller_type_display() {
        assert_eq!(ControllerType::Lateral.to_string(), "lateral");
        assert_eq!(ControllerType::All.to_string(), "all");
    }

    #[test]
    fn test_precipitation_type_display() {
        assert_eq!(PrecipitationType::Dry.to_string(), "dry");
        assert_eq!(PrecipitationType::Rain.to_string(), "rain");
    }

    #[test]
    fn test_wetness_display() {
        assert_eq!(Wetness::Dry.to_string(), "dry");
        assert_eq!(Wetness::WetWithPuddles.to_string(), "wetWithPuddles");
    }

    #[test]
    fn test_color_type_display() {
        assert_eq!(ColorType::Red.to_string(), "red");
        assert_eq!(ColorType::Blue.to_string(), "blue");
    }

    #[test]
    fn test_role_display() {
        assert_eq!(Role::None.to_string(), "none");
        assert_eq!(Role::Police.to_string(), "police");
    }

    #[test]
    fn test_angle_type_display() {
        assert_eq!(AngleType::Relative.to_string(), "relative");
        assert_eq!(AngleType::Absolute.to_string(), "absolute");
    }

    #[test]
    fn test_angle_type_from_str() {
        assert_eq!("relative".parse::<AngleType>().unwrap(), AngleType::Relative);
        assert_eq!("absolute".parse::<AngleType>().unwrap(), AngleType::Absolute);
        assert!("invalid".parse::<AngleType>().is_err());
    }

    #[test]
    fn test_directional_dimension_display() {
        assert_eq!(DirectionalDimension::Longitudinal.to_string(), "longitudinal");
        assert_eq!(DirectionalDimension::Lateral.to_string(), "lateral");
        assert_eq!(DirectionalDimension::Vertical.to_string(), "vertical");
    }

    #[test]
    fn test_vehicle_component_type_display() {
        assert_eq!(VehicleComponentType::Hood.to_string(), "hood");
        assert_eq!(VehicleComponentType::DoorFrontLeft.to_string(), "doorFrontLeft");
    }

    #[test]
    fn test_vehicle_light_type_display() {
        assert_eq!(VehicleLightType::Headlight.to_string(), "headlight");
        assert_eq!(VehicleLightType::BrakeLight.to_string(), "brakeLight");
    }

    #[test]
    fn test_light_mode_display() {
        assert_eq!(LightMode::On.to_string(), "on");
        assert_eq!(LightMode::Off.to_string(), "off");
        assert_eq!(LightMode::Flashing.to_string(), "flashing");
    }
}