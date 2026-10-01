//! The 37 enumerations of the OpenSCENARIO specification. `enum_wire_names_test`
//! pins that count, so a new one cannot be added without the test noticing.
//!
//! Each carries the serde renames that map its variants to the wire names the schema
//! uses. Deprecated values are kept and marked, not dropped, since a file that still
//! sets one remains schema-valid.
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Emits an enum from a single variant -> wire-name table: the `#[serde(rename)]`
/// attributes, `Display`, `FromStr`, and an `ALL` slice (used by
/// `tests/enum_wire_names_test.rs` for coverage) all come from one place.
///
/// Before this macro existed, the rename, the `Display` match and the `FromStr`
/// match were three independent hand-written transcriptions of the same table --
/// see `docs/type_system_guide.md:220-231`. Folding them into one macro
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

        #[allow(deprecated)]
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let s = match self {
                    $($name::$variant => $wire,)+
                };
                write!(f, "{}", s)
            }
        }

        #[allow(deprecated)]
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
    /// `TriggeringEntitiesRule` simple type.
    pub enum TriggeringEntitiesRule {
        All => "all",
        Any => "any",
    }
}

osc_enum! {
    /// Priority level for events and actions
    ///
    /// Wire names verified against `Schema/OpenSCENARIO.xsd`'s `Priority` simple
    /// type. The schema marks `overwrite` deprecated in favor of
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
    /// `StoryboardElementState` simple type.
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
    /// `StoryboardElementType` simple type.
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
    /// simple type. The schema marks `integer` deprecated in favor of
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
    /// simple type.
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
    /// simple type.
    pub enum ReferenceContext {
        Relative => "relative",
        Absolute => "absolute",
    }
}

osc_enum! {
    /// Speed target value type
    ///
    /// Wire names verified against `Schema/OpenSCENARIO.xsd`'s
    /// `SpeedTargetValueType` simple type.
    pub enum SpeedTargetValueType {
        Delta => "delta",
        Factor => "factor",
    }
}

osc_enum! {
    /// Dynamics shape enumeration
    ///
    /// Wire names verified against `Schema/OpenSCENARIO.xsd`'s `DynamicsShape`
    /// simple type.
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

// `CloudState`'s own osc_enum! expansion (Debug/Clone/PartialEq/Eq/Serialize/
// Deserialize derives, Display, FromStr) necessarily refers to its variants by
// name, which would otherwise warn on every one of those generated impls.
// Scoped to this one deprecated enum, not the crate, per F8/OST-29.
#[allow(deprecated)]
mod cloud_state {
    use serde::{Deserialize, Serialize};
    use std::fmt;
    use std::str::FromStr;

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
}
#[allow(deprecated)]
pub use cloud_state::CloudState;
