//! All enumeration types from the OpenSCENARIO specification
//!
//! This file contains:
//! - All 37 enumeration types with their complete value sets
//! - Serde annotations for correct XML serialization (rename attributes)
//! - Deprecation markers for legacy enum values
//! - Default implementations where appropriate
//! - String conversion helpers for debugging and display
//!
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Emits an enum from a single variant -> wire-name table: the `#[serde(rename)]`
/// attributes, `Display`, `FromStr`, and an `ALL` slice (used by
/// `tests/enum_wire_names_test.rs` for coverage) all come from one place.
///
/// Before this macro existed, the rename, the `Display` match and the `FromStr`
/// match were three independent hand-written transcriptions of the same table --
/// see OSR-05 / `docs/type_system_guide.md:220-231`. Folding them into one macro
/// invocation per enum makes drift between them structurally impossible: there is
/// only one place to edit a wire name, and every representation changes together.
///
/// `Value<T>` (`src/types/basic.rs`) serializes `T` through `Display`, not through
/// the derived `Serialize`, which is exactly the hazard this macro closes.
macro_rules! osc_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $($variant:ident => $wire:literal),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        pub enum $name {
            $(
                #[serde(rename = $wire)]
                $variant,
            )+
        }

        impl $name {
            /// Every variant of this enum, in declaration order.
            #[allow(deprecated)]
            pub const ALL: &'static [$name] = &[ $($name::$variant),+ ];
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let s = match self {
                    $($name::$variant => $wire,)+
                };
                write!(f, "{}", s)
            }
        }

        impl FromStr for $name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($wire => Ok($name::$variant),)+
                    _ => Err(format!(concat!("Invalid ", stringify!($name), ": {}"), s)),
                }
            }
        }
    };
}

osc_enum! {
    /// Vehicle category enumeration
    pub enum VehicleCategory {
        Car => "car",
        Van => "van",
        Truck => "truck",
        Semitrailer => "semitrailer",
        Bus => "bus",
        Motorbike => "motorbike",
        Bicycle => "bicycle",
        Train => "train",
        Tram => "tram",
        Trailer => "trailer",
    }
}

osc_enum! {
    /// Pedestrian category enumeration
    pub enum PedestrianCategory {
        Pedestrian => "pedestrian",
        Wheelchair => "wheelchair",
        Animal => "animal",
    }
}

osc_enum! {
    /// Object type enumeration
    pub enum ObjectType {
        Vehicle => "vehicle",
        Pedestrian => "pedestrian",
        MiscellaneousObject => "miscellaneous",
        External => "external",
    }
}

osc_enum! {
    /// Rule enumeration for conditions
    pub enum Rule {
        EqualTo => "equalTo",
        GreaterThan => "greaterThan",
        LessThan => "lessThan",
        GreaterOrEqual => "greaterOrEqual",
        LessOrEqual => "lessOrEqual",
        NotEqualTo => "notEqualTo",
    }
}

osc_enum! {
    /// Condition edge enumeration
    pub enum ConditionEdge {
        None => "none",
        Rising => "rising",
        Falling => "falling",
        RisingOrFalling => "risingOrFalling",
    }
}

osc_enum! {
    /// Triggering entities rule enumeration
    ///
    /// Wire names verified against `Schema/OpenSCENARIO.xsd`'s
    /// `TriggeringEntitiesRule` simple type (OSR-05).
    pub enum TriggeringEntitiesRule {
        All => "all",
        Any => "any",
    }
}

osc_enum! {
    /// Priority level for events and actions
    ///
    /// Wire names verified against `Schema/OpenSCENARIO.xsd`'s `Priority` simple
    /// type (OSR-05). The schema marks `overwrite` deprecated in favor of
    /// `override`; both remain valid wire values and are kept here.
    pub enum Priority {
        Overwrite => "overwrite",
        Override => "override",
        Parallel => "parallel",
        Skip => "skip",
    }
}

osc_enum! {
    /// Storyboard element state enumeration
    ///
    /// Wire names verified against `Schema/OpenSCENARIO.xsd`'s
    /// `StoryboardElementState` simple type (OSR-05).
    pub enum StoryboardElementState {
        CompleteState => "completeState",
        EndTransition => "endTransition",
        RunningState => "runningState",
        SkipTransition => "skipTransition",
        StandbyState => "standbyState",
        StartTransition => "startTransition",
        StopTransition => "stopTransition",
    }
}

osc_enum! {
    /// Storyboard element type enumeration
    ///
    /// Wire names verified against `Schema/OpenSCENARIO.xsd`'s
    /// `StoryboardElementType` simple type (OSR-05).
    pub enum StoryboardElementType {
        Act => "act",
        Action => "action",
        Event => "event",
        Maneuver => "maneuver",
        ManeuverGroup => "maneuverGroup",
        Story => "story",
    }
}

osc_enum! {
    /// Parameter data type enumeration
    ///
    /// Wire names verified against `Schema/OpenSCENARIO.xsd`'s `ParameterType`
    /// simple type (OSR-05). The schema marks `integer` deprecated in favor of
    /// `int`; both remain valid wire values and are kept here.
    pub enum ParameterType {
        Boolean => "boolean",
        DateTime => "dateTime",
        Double => "double",
        Int => "int",
        Integer => "integer",
        String => "string",
        UnsignedInt => "unsignedInt",
        UnsignedShort => "unsignedShort",
    }
}

osc_enum! {
    /// Coordinate system enumeration
    ///
    /// Wire names verified against `Schema/OpenSCENARIO.xsd`'s `CoordinateSystem`
    /// simple type (OSR-05).
    pub enum CoordinateSystem {
        Entity => "entity",
        Lane => "lane",
        Road => "road",
        Trajectory => "trajectory",
        World => "world",
    }
}

osc_enum! {
    /// Reference context enumeration
    ///
    /// Wire names verified against `Schema/OpenSCENARIO.xsd`'s `ReferenceContext`
    /// simple type (OSR-05).
    pub enum ReferenceContext {
        Relative => "relative",
        Absolute => "absolute",
    }
}

osc_enum! {
    /// Speed target value type
    ///
    /// Wire names verified against `Schema/OpenSCENARIO.xsd`'s
    /// `SpeedTargetValueType` simple type (OSR-05).
    pub enum SpeedTargetValueType {
        Delta => "delta",
        Factor => "factor",
    }
}

osc_enum! {
    /// Dynamics shape enumeration
    ///
    /// Wire names verified against `Schema/OpenSCENARIO.xsd`'s `DynamicsShape`
    /// simple type (OSR-05).
    pub enum DynamicsShape {
        Linear => "linear",
        Cubic => "cubic",
        Sinusoidal => "sinusoidal",
        Step => "step",
    }
}

osc_enum! {
    /// Dynamics dimension enumeration
    pub enum DynamicsDimension {
        Rate => "rate",
        Time => "time",
        Distance => "distance",
    }
}

osc_enum! {
    /// Relative distance type enumeration
    pub enum RelativeDistanceType {
        Longitudinal => "longitudinal",
        Lateral => "lateral",
        Cartesian => "cartesianDistance",
        Euclidian => "euclidianDistance",
    }
}

osc_enum! {
    /// Following mode enumeration
    pub enum FollowingMode {
        Position => "position",
        Follow => "follow",
    }
}

osc_enum! {
    /// Miscellaneous object category enumeration
    pub enum MiscObjectCategory {
        Barrier => "barrier",
        Building => "building",
        Crosswalk => "crosswalk",
        Gantry => "gantry",
        None => "none",
        Obstacle => "obstacle",
        ParkingSpace => "parkingSpace",
        Patch => "patch",
        Pole => "pole",
        Railing => "railing",
        RoadMark => "roadMark",
        SoundBarrier => "soundBarrier",
        StreetLamp => "streetLamp",
        TrafficIsland => "trafficIsland",
        Tree => "tree",
        Vegetation => "vegetation",
        Wind => "wind",
    }
}

osc_enum! {
    /// Controller type enumeration
    pub enum ControllerType {
        Lateral => "lateral",
        Longitudinal => "longitudinal",
        Lighting => "lighting",
        Animation => "animation",
        Movement => "movement",
        Appearance => "appearance",
        All => "all",
    }
}

osc_enum! {
    /// Precipitation type enumeration
    pub enum PrecipitationType {
        Dry => "dry",
        Rain => "rain",
        Snow => "snow",
    }
}

osc_enum! {
    /// Wetness level enumeration
    pub enum Wetness {
        Dry => "dry",
        Moist => "moist",
        WetWithPuddles => "wetWithPuddles",
        LowFlooded => "lowFlooded",
        HighFlooded => "highFlooded",
    }
}

osc_enum! {
    /// Color type enumeration
    pub enum ColorType {
        Other => "other",
        Red => "red",
        Yellow => "yellow",
        Green => "green",
        Blue => "blue",
        Violet => "violet",
        Orange => "orange",
        Brown => "brown",
        Black => "black",
        White => "white",
        Grey => "grey",
    }
}

osc_enum! {
    /// Role enumeration
    pub enum Role {
        None => "none",
        Ambulance => "ambulance",
        Civil => "civil",
        Fire => "fire",
        Military => "military",
        Police => "police",
        PublicTransport => "publicTransport",
        RoadAssistance => "roadAssistance",
    }
}

osc_enum! {
    /// Angle type enumeration
    pub enum AngleType {
        Heading => "heading",
        Pitch => "pitch",
        Roll => "roll",
    }
}

osc_enum! {
    /// Directional dimension enumeration
    pub enum DirectionalDimension {
        Longitudinal => "longitudinal",
        Lateral => "lateral",
        Vertical => "vertical",
    }
}

osc_enum! {
    /// Vehicle component type enumeration
    pub enum VehicleComponentType {
        Hood => "hood",
        Trunk => "trunk",
        DoorFrontLeft => "doorFrontLeft",
        DoorFrontRight => "doorFrontRight",
        DoorRearLeft => "doorRearLeft",
        DoorRearRight => "doorRearRight",
        WindowFrontLeft => "windowFrontLeft",
        WindowFrontRight => "windowFrontRight",
        WindowRearLeft => "windowRearLeft",
        WindowRearRight => "windowRearRight",
        SideMirrors => "sideMirrors",
        SideMirrorRight => "sideMirrorRight",
        SideMirrorLeft => "sideMirrorLeft",
    }
}

osc_enum! {
    /// Vehicle light type enumeration
    pub enum VehicleLightType {
        DaytimeRunningLights => "daytimeRunningLights",
        LowBeam => "lowBeam",
        HighBeam => "highBeam",
        FogLights => "fogLights",
        FogLightsFront => "fogLightsFront",
        FogLightsRear => "fogLightsRear",
        BrakeLights => "brakeLights",
        WarningLights => "warningLights",
        IndicatorLeft => "indicatorLeft",
        IndicatorRight => "indicatorRight",
        ReversingLights => "reversingLights",
        LicensePlateIllumination => "licensePlateIllumination",
        SpecialPurposeLights => "specialPurposeLights",
    }
}

osc_enum! {
    /// Light mode enumeration
    pub enum LightMode {
        On => "on",
        Off => "off",
        Flashing => "flashing",
    }
}

osc_enum! {
    /// Automatic gear type enumeration
    pub enum AutomaticGearType {
        Neutral => "n",
        Park => "p",
        Reverse => "r",
        Drive => "d",
    }
}

osc_enum! {
    /// Fractional cloud cover enumeration (in oktas)
    pub enum FractionalCloudCover {
        ZeroOktas => "zeroOktas",
        OneOktas => "oneOktas",
        TwoOktas => "twoOktas",
        ThreeOktas => "threeOktas",
        FourOktas => "fourOktas",
        FiveOktas => "fiveOktas",
        SixOktas => "sixOktas",
        SevenOktas => "sevenOktas",
        EightOktas => "eightOktas",
        NineOktas => "nineOktas",
    }
}

osc_enum! {
    /// Pedestrian motion type enumeration
    pub enum PedestrianMotionType {
        Standing => "standing",
        Sitting => "sitting",
        Lying => "lying",
        Squatting => "squatting",
        Walking => "walking",
        Running => "running",
        Reeling => "reeling",
        Crawling => "crawling",
        Cycling => "cycling",
        Jumping => "jumping",
        Ducking => "ducking",
        BendingDown => "bendingDown",
    }
}

osc_enum! {
    /// Pedestrian gesture type enumeration
    pub enum PedestrianGestureType {
        PhoneCallRightHand => "phoneCallRightHand",
        PhoneCallLeftHand => "phoneCallLeftHand",
        PhoneTextRightHand => "phoneTextRightHand",
        PhoneTextLeftHand => "phoneTextLeftHand",
        WavingRightArm => "wavingRightArm",
        WavingLeftArm => "wavingLeftArm",
        UmbrellaRightHand => "umbrellaRightHand",
        UmbrellaLeftHand => "umbrellaLeftHand",
        CrossArms => "crossArms",
        CoffeeRightHand => "coffeeRightHand",
        CoffeeLeftHand => "coffeeLeftHand",
        SandwichRightHand => "sandwichRightHand",
        SandwichLeftHand => "sandwichLeftHand",
    }
}

osc_enum! {
    /// Route strategy enumeration
    pub enum RouteStrategy {
        Fastest => "fastest",
        LeastIntersections => "leastIntersections",
        Random => "random",
        Shortest => "shortest",
    }
}

osc_enum! {
    /// Routing algorithm enumeration
    pub enum RoutingAlgorithm {
        AssignedRoute => "assignedRoute",
        Fastest => "fastest",
        LeastIntersections => "leastIntersections",
        Shortest => "shortest",
        Undefined => "undefined",
    }
}

osc_enum! {
    /// Lateral displacement enumeration
    pub enum LateralDisplacement {
        Any => "any",
        LeftToReferencedEntity => "leftToReferencedEntity",
        RightToReferencedEntity => "rightToReferencedEntity",
    }
}

osc_enum! {
    /// Longitudinal displacement enumeration
    pub enum LongitudinalDisplacement {
        Any => "any",
        TrailingReferencedEntity => "trailingReferencedEntity",
        LeadingReferencedEntity => "leadingReferencedEntity",
    }
}

osc_enum! {
    /// Cloud state enumeration (deprecated)
    #[deprecated(note = "CloudState is deprecated, use FractionalCloudCover instead")]
    pub enum CloudState {
        Cloudy => "cloudy",
        Free => "free",
        Overcast => "overcast",
        Rainy => "rainy",
        SkyOff => "skyOff",
    }
}

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
        assert_eq!(
            "car".parse::<VehicleCategory>().unwrap(),
            VehicleCategory::Car
        );
        assert_eq!(
            "truck".parse::<VehicleCategory>().unwrap(),
            VehicleCategory::Truck
        );
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
        assert_eq!(
            "barrier".parse::<MiscObjectCategory>().unwrap(),
            MiscObjectCategory::Barrier
        );
        assert_eq!(
            "obstacle".parse::<MiscObjectCategory>().unwrap(),
            MiscObjectCategory::Obstacle
        );
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
        assert_eq!(AngleType::Heading.to_string(), "heading");
        assert_eq!(AngleType::Pitch.to_string(), "pitch");
        assert_eq!(AngleType::Roll.to_string(), "roll");
    }

    #[test]
    fn test_angle_type_from_str() {
        assert_eq!("heading".parse::<AngleType>().unwrap(), AngleType::Heading);
        assert_eq!("pitch".parse::<AngleType>().unwrap(), AngleType::Pitch);
        assert_eq!("roll".parse::<AngleType>().unwrap(), AngleType::Roll);
        assert!("invalid".parse::<AngleType>().is_err());
    }

    #[test]
    fn test_directional_dimension_display() {
        assert_eq!(
            DirectionalDimension::Longitudinal.to_string(),
            "longitudinal"
        );
        assert_eq!(DirectionalDimension::Lateral.to_string(), "lateral");
        assert_eq!(DirectionalDimension::Vertical.to_string(), "vertical");
    }

    #[test]
    fn test_vehicle_component_type_display() {
        assert_eq!(VehicleComponentType::Hood.to_string(), "hood");
        assert_eq!(
            VehicleComponentType::DoorFrontLeft.to_string(),
            "doorFrontLeft"
        );
    }

    #[test]
    fn test_vehicle_light_type_display() {
        assert_eq!(
            VehicleLightType::DaytimeRunningLights.to_string(),
            "daytimeRunningLights"
        );
        assert_eq!(VehicleLightType::BrakeLights.to_string(), "brakeLights");
    }

    #[test]
    fn test_light_mode_display() {
        assert_eq!(LightMode::On.to_string(), "on");
        assert_eq!(LightMode::Off.to_string(), "off");
        assert_eq!(LightMode::Flashing.to_string(), "flashing");
    }

    #[test]
    fn test_automatic_gear_type_display() {
        assert_eq!(AutomaticGearType::Neutral.to_string(), "n");
        assert_eq!(AutomaticGearType::Park.to_string(), "p");
        assert_eq!(AutomaticGearType::Reverse.to_string(), "r");
        assert_eq!(AutomaticGearType::Drive.to_string(), "d");
    }

    #[test]
    fn test_automatic_gear_type_from_str() {
        assert_eq!(
            "n".parse::<AutomaticGearType>().unwrap(),
            AutomaticGearType::Neutral
        );
        assert_eq!(
            "p".parse::<AutomaticGearType>().unwrap(),
            AutomaticGearType::Park
        );
        assert_eq!(
            "r".parse::<AutomaticGearType>().unwrap(),
            AutomaticGearType::Reverse
        );
        assert_eq!(
            "d".parse::<AutomaticGearType>().unwrap(),
            AutomaticGearType::Drive
        );
        assert!("invalid".parse::<AutomaticGearType>().is_err());
    }

    #[test]
    fn test_fractional_cloud_cover_display() {
        assert_eq!(FractionalCloudCover::ZeroOktas.to_string(), "zeroOktas");
        assert_eq!(FractionalCloudCover::FiveOktas.to_string(), "fiveOktas");
        assert_eq!(FractionalCloudCover::NineOktas.to_string(), "nineOktas");
    }

    #[test]
    fn test_fractional_cloud_cover_from_str() {
        assert_eq!(
            "zeroOktas".parse::<FractionalCloudCover>().unwrap(),
            FractionalCloudCover::ZeroOktas
        );
        assert_eq!(
            "fiveOktas".parse::<FractionalCloudCover>().unwrap(),
            FractionalCloudCover::FiveOktas
        );
        assert_eq!(
            "nineOktas".parse::<FractionalCloudCover>().unwrap(),
            FractionalCloudCover::NineOktas
        );
        assert!("invalid".parse::<FractionalCloudCover>().is_err());
    }

    #[test]
    fn test_pedestrian_motion_type_display() {
        assert_eq!(PedestrianMotionType::Standing.to_string(), "standing");
        assert_eq!(PedestrianMotionType::Walking.to_string(), "walking");
        assert_eq!(PedestrianMotionType::Running.to_string(), "running");
        assert_eq!(PedestrianMotionType::BendingDown.to_string(), "bendingDown");
    }

    #[test]
    fn test_pedestrian_motion_type_from_str() {
        assert_eq!(
            "standing".parse::<PedestrianMotionType>().unwrap(),
            PedestrianMotionType::Standing
        );
        assert_eq!(
            "walking".parse::<PedestrianMotionType>().unwrap(),
            PedestrianMotionType::Walking
        );
        assert_eq!(
            "running".parse::<PedestrianMotionType>().unwrap(),
            PedestrianMotionType::Running
        );
        assert_eq!(
            "bendingDown".parse::<PedestrianMotionType>().unwrap(),
            PedestrianMotionType::BendingDown
        );
        assert!("invalid".parse::<PedestrianMotionType>().is_err());
    }

    #[test]
    fn test_pedestrian_gesture_type_display() {
        assert_eq!(
            PedestrianGestureType::PhoneCallRightHand.to_string(),
            "phoneCallRightHand"
        );
        assert_eq!(
            PedestrianGestureType::WavingLeftArm.to_string(),
            "wavingLeftArm"
        );
        assert_eq!(
            PedestrianGestureType::CoffeeRightHand.to_string(),
            "coffeeRightHand"
        );
        assert_eq!(
            PedestrianGestureType::SandwichLeftHand.to_string(),
            "sandwichLeftHand"
        );
    }

    #[test]
    fn test_pedestrian_gesture_type_from_str() {
        assert_eq!(
            "phoneCallRightHand"
                .parse::<PedestrianGestureType>()
                .unwrap(),
            PedestrianGestureType::PhoneCallRightHand
        );
        assert_eq!(
            "wavingLeftArm".parse::<PedestrianGestureType>().unwrap(),
            PedestrianGestureType::WavingLeftArm
        );
        assert_eq!(
            "coffeeRightHand".parse::<PedestrianGestureType>().unwrap(),
            PedestrianGestureType::CoffeeRightHand
        );
        assert_eq!(
            "sandwichLeftHand".parse::<PedestrianGestureType>().unwrap(),
            PedestrianGestureType::SandwichLeftHand
        );
        assert!("invalid".parse::<PedestrianGestureType>().is_err());
    }

    #[test]
    fn test_route_strategy_display() {
        assert_eq!(RouteStrategy::Fastest.to_string(), "fastest");
        assert_eq!(
            RouteStrategy::LeastIntersections.to_string(),
            "leastIntersections"
        );
        assert_eq!(RouteStrategy::Random.to_string(), "random");
        assert_eq!(RouteStrategy::Shortest.to_string(), "shortest");
    }

    #[test]
    fn test_route_strategy_from_str() {
        assert_eq!(
            "fastest".parse::<RouteStrategy>().unwrap(),
            RouteStrategy::Fastest
        );
        assert_eq!(
            "leastIntersections".parse::<RouteStrategy>().unwrap(),
            RouteStrategy::LeastIntersections
        );
        assert_eq!(
            "random".parse::<RouteStrategy>().unwrap(),
            RouteStrategy::Random
        );
        assert_eq!(
            "shortest".parse::<RouteStrategy>().unwrap(),
            RouteStrategy::Shortest
        );
        assert!("invalid".parse::<RouteStrategy>().is_err());
    }

    #[test]
    fn test_routing_algorithm_display() {
        assert_eq!(RoutingAlgorithm::AssignedRoute.to_string(), "assignedRoute");
        assert_eq!(RoutingAlgorithm::Fastest.to_string(), "fastest");
        assert_eq!(
            RoutingAlgorithm::LeastIntersections.to_string(),
            "leastIntersections"
        );
        assert_eq!(RoutingAlgorithm::Shortest.to_string(), "shortest");
        assert_eq!(RoutingAlgorithm::Undefined.to_string(), "undefined");
    }

    #[test]
    fn test_routing_algorithm_from_str() {
        assert_eq!(
            "assignedRoute".parse::<RoutingAlgorithm>().unwrap(),
            RoutingAlgorithm::AssignedRoute
        );
        assert_eq!(
            "fastest".parse::<RoutingAlgorithm>().unwrap(),
            RoutingAlgorithm::Fastest
        );
        assert_eq!(
            "leastIntersections".parse::<RoutingAlgorithm>().unwrap(),
            RoutingAlgorithm::LeastIntersections
        );
        assert_eq!(
            "shortest".parse::<RoutingAlgorithm>().unwrap(),
            RoutingAlgorithm::Shortest
        );
        assert_eq!(
            "undefined".parse::<RoutingAlgorithm>().unwrap(),
            RoutingAlgorithm::Undefined
        );
        assert!("invalid".parse::<RoutingAlgorithm>().is_err());
    }

    #[test]
    fn test_lateral_displacement_display() {
        assert_eq!(LateralDisplacement::Any.to_string(), "any");
        assert_eq!(
            LateralDisplacement::LeftToReferencedEntity.to_string(),
            "leftToReferencedEntity"
        );
        assert_eq!(
            LateralDisplacement::RightToReferencedEntity.to_string(),
            "rightToReferencedEntity"
        );
    }

    #[test]
    fn test_lateral_displacement_from_str() {
        assert_eq!(
            "any".parse::<LateralDisplacement>().unwrap(),
            LateralDisplacement::Any
        );
        assert_eq!(
            "leftToReferencedEntity"
                .parse::<LateralDisplacement>()
                .unwrap(),
            LateralDisplacement::LeftToReferencedEntity
        );
        assert_eq!(
            "rightToReferencedEntity"
                .parse::<LateralDisplacement>()
                .unwrap(),
            LateralDisplacement::RightToReferencedEntity
        );
        assert!("invalid".parse::<LateralDisplacement>().is_err());
    }

    #[test]
    fn test_longitudinal_displacement_display() {
        assert_eq!(LongitudinalDisplacement::Any.to_string(), "any");
        assert_eq!(
            LongitudinalDisplacement::TrailingReferencedEntity.to_string(),
            "trailingReferencedEntity"
        );
        assert_eq!(
            LongitudinalDisplacement::LeadingReferencedEntity.to_string(),
            "leadingReferencedEntity"
        );
    }

    #[test]
    fn test_longitudinal_displacement_from_str() {
        assert_eq!(
            "any".parse::<LongitudinalDisplacement>().unwrap(),
            LongitudinalDisplacement::Any
        );
        assert_eq!(
            "trailingReferencedEntity"
                .parse::<LongitudinalDisplacement>()
                .unwrap(),
            LongitudinalDisplacement::TrailingReferencedEntity
        );
        assert_eq!(
            "leadingReferencedEntity"
                .parse::<LongitudinalDisplacement>()
                .unwrap(),
            LongitudinalDisplacement::LeadingReferencedEntity
        );
        assert!("invalid".parse::<LongitudinalDisplacement>().is_err());
    }

    #[test]
    fn test_cloud_state_display() {
        assert_eq!(CloudState::Cloudy.to_string(), "cloudy");
        assert_eq!(CloudState::Free.to_string(), "free");
        assert_eq!(CloudState::Overcast.to_string(), "overcast");
        assert_eq!(CloudState::Rainy.to_string(), "rainy");
        assert_eq!(CloudState::SkyOff.to_string(), "skyOff");
    }

    #[test]
    fn test_cloud_state_from_str() {
        assert_eq!("cloudy".parse::<CloudState>().unwrap(), CloudState::Cloudy);
        assert_eq!("free".parse::<CloudState>().unwrap(), CloudState::Free);
        assert_eq!(
            "overcast".parse::<CloudState>().unwrap(),
            CloudState::Overcast
        );
        assert_eq!("rainy".parse::<CloudState>().unwrap(), CloudState::Rainy);
        assert_eq!("skyOff".parse::<CloudState>().unwrap(), CloudState::SkyOff);
        assert!("invalid".parse::<CloudState>().is_err());
    }

    #[test]
    fn test_cloud_state_deprecation_warning() {
        // This test documents that CloudState is deprecated
        // The deprecation warning should be shown when using these types
        let _state = CloudState::Free;
        // If CloudState is used in real code, developers should migrate to FractionalCloudCover
    }
}
