//! Wire-name conformance test for `src/types/enums.rs`.
//!
//! `Value<T>` (`src/types/basic.rs`) serializes `T` through `Display`, not through the
//! derived `Serialize` impl and its `#[serde(rename)]` attributes. `enums.rs` builds
//! every enum through the `osc_enum!` macro from a single variant -> wire-name table,
//! which emits the `#[serde(rename)]` attributes, `Display`, `FromStr`, and an `ALL`
//! slice together. This test proves, for every variant of every enum:
//!
//!   - the wire name serde derives is an `xsd:enumeration` value of the `xsd:simpleType`
//!     of the same name in `Schema/OpenSCENARIO.xsd`, and every enumeration value there
//!     has a variant (the XSD is read at test time, not transcribed);
//!   - `variant.to_string() == wire_name`        (Display agrees with serde);
//!   - `wire_name.parse::<E>() == Ok(variant)`    (FromStr agrees with serde);
//!   - a string that is no wire name is refused with `Invalid <Enum>: <text>`.
//!
//! The serde wire name is derived by serializing each variant through `serde_json`,
//! which reads the `#[serde(rename)]` the compiler applied.

use openscenario_rs::types::enums::{
    AngleType, AutomaticGearType, ColorType, ConditionEdge, ControllerType, CoordinateSystem,
    DirectionalDimension, DynamicsDimension, DynamicsShape, FollowingMode, FractionalCloudCover,
    LightMode, MiscObjectCategory, ObjectType, ParameterType, PedestrianCategory,
    PedestrianGestureType, PedestrianMotionType, PrecipitationType, Priority, ReferenceContext,
    RelativeDistanceType, Role, RouteStrategy, RoutingAlgorithm, Rule, SpeedTargetValueType,
    StoryboardElementState, StoryboardElementType, TriggeringEntitiesRule, VehicleCategory,
    VehicleComponentType, VehicleLightType, Wetness,
};
#[allow(deprecated)]
use openscenario_rs::types::enums::{CloudState, LateralDisplacement, LongitudinalDisplacement};

use quick_xml::events::Event;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;
use std::str::FromStr;

/// Derive the wire name serde actually produces for `value`, by serializing a
/// one-field wrapper struct to JSON and reading back the field value. This reads
/// the real `#[serde(rename)]` the compiler applied -- it is not transcribed by
/// hand -- and is equivalent for a plain-string enum to what quick_xml would emit
/// for an XML attribute.
fn wire_name<T: Serialize>(value: &T) -> String {
    #[derive(Serialize)]
    struct W<'a, T> {
        v: &'a T,
    }
    let json = serde_json::to_string(&W { v: value }).expect("serialize wrapper");
    let value: serde_json::Value = serde_json::from_str(&json).expect("parse wrapper json");
    value["v"]
        .as_str()
        .unwrap_or_else(|| panic!("wire value for {json} was not a JSON string"))
        .to_string()
}

/// Assert that every variant's `Display` and `FromStr` agree with the wire name serde
/// derives from its `#[serde(rename)]`, that a non-wire string is refused naming the
/// enum, and return the wire names for comparison with the XSD.
fn check_all<T>(enum_name: &str, all: &[T]) -> BTreeSet<String>
where
    T: Serialize + std::fmt::Display + FromStr<Err = String> + Debug + PartialEq,
{
    let mut wires = BTreeSet::new();
    for variant in all {
        let wire = wire_name(variant);
        assert_eq!(
            variant.to_string(),
            wire,
            "Display disagrees with #[serde(rename)] for {variant:?}"
        );
        let parsed = wire
            .parse::<T>()
            .unwrap_or_else(|e| panic!("FromStr({wire:?}) failed for {variant:?}: {e:?}"));
        assert_eq!(
            &parsed, variant,
            "FromStr({wire:?}) round-trip disagrees with the original variant"
        );
        assert!(wires.insert(wire), "duplicate wire name in {enum_name}");
    }
    assert_eq!(
        "notAWireName".parse::<T>(),
        Err(format!("Invalid {enum_name}: notAWireName")),
        "{enum_name} accepted a string that is not one of its wire names"
    );
    wires
}

/// Every `xsd:enumeration` value in `Schema/OpenSCENARIO.xsd`, keyed by the name of the
/// named `xsd:simpleType` that declares it.
fn xsd_enumerations() -> BTreeMap<String, BTreeSet<String>> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/Schema/OpenSCENARIO.xsd");
    let xsd = std::fs::read_to_string(path).expect("read Schema/OpenSCENARIO.xsd");
    let mut reader = quick_xml::Reader::from_str(&xsd);
    let mut enumerations: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    // The named simple type being read, and how deep inside it the reader is: its
    // enumerations sit in an anonymous `xsd:simpleType` of an `xsd:union`.
    let mut current: Option<String> = None;
    let mut depth = 0usize;
    loop {
        match reader.read_event().expect("the XSD is well-formed XML") {
            Event::Start(e) if e.name().as_ref() == b"xsd:simpleType" => {
                if let Some(name) = e.try_get_attribute("name").expect("attribute") {
                    current = Some(name.unescape_value().expect("name").into_owned());
                    depth = 0;
                }
                depth += 1;
            }
            Event::End(e) if e.name().as_ref() == b"xsd:simpleType" => {
                depth -= 1;
                if depth == 0 {
                    current = None;
                }
            }
            Event::Start(e) | Event::Empty(e) if e.name().as_ref() == b"xsd:enumeration" => {
                let simple_type = current
                    .clone()
                    .expect("every xsd:enumeration sits in a named simpleType");
                let value = e
                    .try_get_attribute("value")
                    .expect("attribute")
                    .expect("xsd:enumeration carries @value");
                enumerations
                    .entry(simple_type)
                    .or_default()
                    .insert(value.unescape_value().expect("value").into_owned());
            }
            Event::Eof => break,
            _ => {}
        }
    }
    enumerations
}

macro_rules! wire_names_of {
    ($($enum:ident),+ $(,)?) => {{
        let mut checked = BTreeMap::new();
        $( checked.insert(stringify!($enum).to_string(), check_all(stringify!($enum), $enum::ALL)); )+
        checked
    }};
}

#[test]
#[allow(deprecated)]
fn enum_wire_names_match_the_xsd_and_display_fromstr_agree_with_serde() {
    let crate_enums = wire_names_of!(
        AngleType,
        AutomaticGearType,
        CloudState,
        ColorType,
        ConditionEdge,
        ControllerType,
        CoordinateSystem,
        DirectionalDimension,
        DynamicsDimension,
        DynamicsShape,
        FollowingMode,
        FractionalCloudCover,
        LateralDisplacement,
        LightMode,
        LongitudinalDisplacement,
        MiscObjectCategory,
        ObjectType,
        ParameterType,
        PedestrianCategory,
        PedestrianGestureType,
        PedestrianMotionType,
        PrecipitationType,
        Priority,
        ReferenceContext,
        RelativeDistanceType,
        Role,
        RouteStrategy,
        RoutingAlgorithm,
        Rule,
        SpeedTargetValueType,
        StoryboardElementState,
        StoryboardElementType,
        TriggeringEntitiesRule,
        VehicleCategory,
        VehicleComponentType,
        VehicleLightType,
        Wetness,
    );
    let xsd = xsd_enumerations();
    // The set of enumerations first: an XSD simple type with no enum here, or an enum
    // with no XSD simple type of its name, fails this assert.
    assert_eq!(
        crate_enums.keys().collect::<Vec<_>>(),
        xsd.keys().collect::<Vec<_>>(),
        "the crate's enums and the XSD's enumeration simple types differ"
    );
    for (name, wires) in &crate_enums {
        assert_eq!(wires, &xsd[name], "{name}: wire names differ from the XSD");
    }
}
