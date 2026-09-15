//! Wire-name conformance test for `src/types/enums.rs`.
//!
//! `Value<T>` (`src/types/basic.rs`) serializes `T` through `Display`, not through the
//! derived `Serialize` impl and its `#[serde(rename)]` attributes. `enums.rs` builds
//! every enum through the `osc_enum!` macro from a single variant -> wire-name table,
//! which emits the `#[serde(rename)]` attributes, `Display`, `FromStr`, and an `ALL`
//! slice together -- but this test is what actually proves all three agree, for every
//! variant of all 37 enums:
//!
//!   - `variant.to_string() == wire_name`        (Display agrees with serde)
//!   - `wire_name.parse::<E>() == Ok(variant)`    (FromStr agrees with serde)
//!
//! The expected wire name is never hand-transcribed from the `#[serde(rename)]`
//! attribute -- that would just be a copy of the same table the macro already
//! generates from, and could drift the same way the earlier hand-written
//! Display/FromStr/rename triplets did. Instead it is derived by round-tripping each
//! variant through `serde_json`, which reads the actual `#[serde(rename)]` the
//! compiler applied.
//!
//! ## Provenance
//!
//! This test was first written and run against the *unmodified* tree, before any
//! `Display`/`FromStr` impl was added or the `osc_enum!` macro existed. At that
//! point 28 of the 37 enums had hand-written `Display`/`FromStr`; the other 9
//! (`TriggeringEntitiesRule`, `Priority`, `StoryboardElementState`,
//! `StoryboardElementType`, `ParameterType`, `CoordinateSystem`, `ReferenceContext`,
//! `SpeedTargetValueType`, `DynamicsShape`) did not implement either trait, so they
//! could not be referenced here at all -- not "test them and see them fail", but a
//! hard compile error, which is why that first run covered only 181 variants across
//! 28 enums. That run found **zero mismatches**: every existing hand-written
//! `Display`/`FromStr` pair already agreed with its `#[serde(rename)]`. The 9 missing
//! enums, and the `osc_enum!` macro, were added afterward; this is the resulting
//! full-coverage version, now covering all 37 enums / 221 variants via each enum's
//! `ALL` slice.

use openscenario_rs::types::basic::Value;
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

use serde::Serialize;
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

/// Assert that `variant`'s `Display` and `FromStr` impls agree with the wire name
/// serde derives from its `#[serde(rename)]` attribute.
fn check<T>(variant: &T)
where
    T: Serialize + std::fmt::Display + FromStr + Debug + PartialEq,
    T::Err: Debug,
{
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
}

/// Check every variant in `E::ALL` and return how many were checked, so the caller
/// can assert total coverage.
fn check_all<T>(all: &[T]) -> usize
where
    T: Serialize + std::fmt::Display + FromStr + Debug + PartialEq,
    T::Err: Debug,
{
    for variant in all {
        check(variant);
    }
    all.len()
}

#[test]
fn enum_display_and_fromstr_agree_with_serde_rename() {
    let mut n = 0usize;

    n += check_all(VehicleCategory::ALL);
    n += check_all(PedestrianCategory::ALL);
    n += check_all(ObjectType::ALL);
    n += check_all(Rule::ALL);
    n += check_all(ConditionEdge::ALL);
    n += check_all(TriggeringEntitiesRule::ALL);
    n += check_all(Priority::ALL);
    n += check_all(StoryboardElementState::ALL);
    n += check_all(StoryboardElementType::ALL);
    n += check_all(ParameterType::ALL);
    n += check_all(CoordinateSystem::ALL);
    n += check_all(ReferenceContext::ALL);
    n += check_all(SpeedTargetValueType::ALL);
    n += check_all(DynamicsShape::ALL);
    n += check_all(DynamicsDimension::ALL);
    n += check_all(RelativeDistanceType::ALL);
    n += check_all(FollowingMode::ALL);
    n += check_all(MiscObjectCategory::ALL);
    n += check_all(ControllerType::ALL);
    n += check_all(PrecipitationType::ALL);
    n += check_all(Wetness::ALL);
    n += check_all(ColorType::ALL);
    n += check_all(Role::ALL);
    n += check_all(AngleType::ALL);
    n += check_all(DirectionalDimension::ALL);
    n += check_all(VehicleComponentType::ALL);
    n += check_all(VehicleLightType::ALL);
    n += check_all(LightMode::ALL);
    n += check_all(AutomaticGearType::ALL);
    n += check_all(FractionalCloudCover::ALL);
    n += check_all(PedestrianMotionType::ALL);
    n += check_all(PedestrianGestureType::ALL);
    n += check_all(RouteStrategy::ALL);
    n += check_all(RoutingAlgorithm::ALL);

    #[allow(deprecated)]
    {
        n += check_all(LateralDisplacement::ALL);
        n += check_all(LongitudinalDisplacement::ALL);
        n += check_all(CloudState::ALL);
    }

    // 37 enums checked above. If a 38th enum is added to enums.rs without a line
    // here, this count silently under-reports rather than the missing enum being
    // silently skipped -- the count is the coverage assertion.
    const ENUM_COUNT: usize = 37;
    const TOTAL_VARIANTS: usize = 221;
    assert_eq!(
        n, TOTAL_VARIANTS,
        "total variant count changed across the {ENUM_COUNT} enums checked above -- \
         either a variant was added/removed, or a `check_all` call for one of the 37 \
         enums is missing; update TOTAL_VARIANTS only after confirming which"
    );
}

/// `Value<VehicleCategory>` (a required, `@`-renamed attribute type, once a field is
/// wrapped in it) must deserialize from both a plain literal wire value and a parameter
/// reference, and serialize back byte-identically. `VehicleCategory` was chosen because
/// it is the example used throughout the design discussion; any enum with verified
/// `Display`/`FromStr` behaves the same way since `Value<T>` in
/// `src/types/basic.rs` is generic over `T: FromStr + Display`.
///
/// This is committed (not run-and-discarded) so that wrapping the enum-typed fields in
/// `Value<E>` does not have to rediscover that this works first.
#[test]
fn value_wraps_vehicle_category_literal_and_parameter() {
    #[derive(Serialize, serde::Deserialize, Debug, PartialEq)]
    struct Wrapper {
        #[serde(rename = "@category")]
        category: Value<VehicleCategory>,
    }

    // Literal wire value on a required, `@`-renamed attribute.
    let literal_xml = r#"{"@category":"car"}"#;
    let parsed: Wrapper = serde_json::from_str(literal_xml).unwrap();
    assert_eq!(parsed.category, Value::Literal(VehicleCategory::Car));
    let reserialized = serde_json::to_string(&parsed).unwrap();
    assert_eq!(
        reserialized, literal_xml,
        "literal round-trip not byte-identical"
    );

    // Parameter reference on the same field.
    //
    // The spelling this asserts changed, and the schema is the reason.
    // `Schema/OpenSCENARIO.xsd:4-13` defines two productions:
    //
    //     parameter   [$][A-Za-z_][A-Za-z0-9_]*
    //     expression  [$][{][ A-Za-z0-9_\+\-\*/%$\(\)\.,]*[\}]
    //
    // The scalar unions (`Double`, `Int`, `Boolean`, ...) list `expression parameter ...`,
    // so both spellings validate there -- which is why the braced form went unchallenged
    // for so long. All 37 *enumeration* unions list `parameter` alone. Emitting `${cat}`
    // on `@vehicleCategory` therefore produces schema-invalid XML, so `Value::Parameter`
    // now serializes as `$cat`, which every union in the schema accepts.
    let param_xml = r#"{"@category":"$cat"}"#;
    let parsed: Wrapper = serde_json::from_str(param_xml).unwrap();
    assert_eq!(parsed.category, Value::Parameter("cat".to_string()));
    let reserialized = serde_json::to_string(&parsed).unwrap();
    assert_eq!(
        reserialized, param_xml,
        "parameter round-trip not byte-identical"
    );

    // Deserialization stays permissive about the two spellings: a document written with
    // the braced form still parses, and normalizes to the schema-valid one on output.
    let braced: Wrapper = serde_json::from_str(r#"{"@category":"${cat}"}"#).unwrap();
    assert_eq!(braced.category, Value::Parameter("cat".to_string()));
    assert_eq!(serde_json::to_string(&braced).unwrap(), param_xml);
}
