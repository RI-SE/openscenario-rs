//! Cardinality tests for the movement and controller choice groups.
//!
//! Each of these types models an XSD model group whose cardinality the Rust
//! type is now responsible for stating. Two shapes appear here and they are not
//! interchangeable:
//!
//! - A bare `<xsd:choice>` requires exactly one branch. It maps to a required
//!   `$value` field, so serde rejects a document naming no branch with
//!   `missing field $value` and one naming two with `duplicate field $value`.
//! - A group that can match the empty sequence — either
//!   `<xsd:group ref="…" minOccurs="0"/>` or a choice whose every branch carries
//!   `minOccurs="0"` — admits zero or one branch. It maps to
//!   `Option<Enum>` behind `$value` with `default`, so absence is legal and only
//!   the two-branch document is rejected.
//!
//! The zero-branch and two-branch claims are separate tests on purpose. A single
//! test asserting both stops at the first failure, and the second claim is the
//! one that was live: before these conversions every type here parsed a
//! two-branch document into a value holding both branches and re-serialized
//! both, emitting XML no schema-valid tool accepts.

use openscenario_rs::types::actions::control::{
    AssignControllerAction, AssignControllerActionChoice, ControllerAction, ControllerActionChoice,
};
use openscenario_rs::types::actions::movement::{
    AbsoluteSpeed, AbsoluteTargetLaneOffset, AbsoluteTargetSpeed, AssignRouteAction, FinalSpeed,
    FinalSpeedChoice, LaneOffsetAction, LaneOffsetActionDynamics, LaneOffsetTarget,
    LongitudinalAction, LongitudinalActionChoice, LongitudinalDistanceAction,
    RelativeSpeedToMaster, RelativeTargetLaneOffset, RelativeTargetSpeed, SpeedActionTarget,
    SpeedActionTargetChoice, SpeedProfileAction, SpeedProfileEntry, SteadyState, TimeReference,
    TimeReferenceChoice, Timing, TrajectoryRef, TrajectoryRefChoice, TransitionDynamics,
};
use openscenario_rs::types::basic::{Double, Value};
use openscenario_rs::types::enums::{
    DynamicsDimension, DynamicsShape, FollowingMode, ReferenceContext, RouteStrategy,
    SpeedTargetValueType,
};
use openscenario_rs::types::positions::Position;
use openscenario_rs::types::routing::{Route, RouteRef, Waypoint};

fn de<T: serde::de::DeserializeOwned>(xml: &str) -> T {
    quick_xml::de::from_str(xml).unwrap_or_else(|e| panic!("deserialize failed for {xml}: {e}"))
}

fn err<T: serde::de::DeserializeOwned + std::fmt::Debug>(xml: &str) -> String {
    match quick_xml::de::from_str::<T>(xml) {
        Ok(v) => panic!("schema-invalid document was accepted: {xml}\n  parsed as: {v:?}"),
        Err(e) => e.to_string(),
    }
}

fn round_trips<T>(xml: &str)
where
    T: serde::de::DeserializeOwned + serde::Serialize + PartialEq + std::fmt::Debug,
{
    let value: T = de(xml);
    let out = quick_xml::se::to_string(&value).expect("serialize failed");
    assert_eq!(out, xml, "serialized bytes must equal the source document");
    assert_eq!(de::<T>(&out), value, "reparse must equal the parsed value");
}

// ---------------------------------------------------------------------------
// SpeedActionTarget — XSD :2049-2054, bare xsd:choice
// ---------------------------------------------------------------------------

const SPEED_TARGET_ABSOLUTE: &str =
    r#"<SpeedActionTarget><AbsoluteTargetSpeed value="30"/></SpeedActionTarget>"#;

#[test]
fn speed_action_target_absolute_branch_round_trips_byte_exact() {
    round_trips::<SpeedActionTarget>(SPEED_TARGET_ABSOLUTE);
    let target: SpeedActionTarget = de(SPEED_TARGET_ABSOLUTE);
    assert!(matches!(
        target.target,
        SpeedActionTargetChoice::AbsoluteTargetSpeed(_)
    ));
}

#[test]
fn speed_action_target_relative_branch_round_trips_byte_exact() {
    let xml = r#"<SpeedActionTarget><RelativeTargetSpeed value="2" entityRef="Ego" speedTargetValueType="delta" continuous="true"/></SpeedActionTarget>"#;
    round_trips::<SpeedActionTarget>(xml);
}

#[test]
fn speed_action_target_rejects_zero_branches() {
    let message = err::<SpeedActionTarget>("<SpeedActionTarget/>");
    assert!(message.contains("$value"), "got: {message}");
}

#[test]
fn speed_action_target_rejects_two_branches() {
    let message = err::<SpeedActionTarget>(
        r#"<SpeedActionTarget><AbsoluteTargetSpeed value="30"/><RelativeTargetSpeed value="2" entityRef="Ego" speedTargetValueType="delta" continuous="true"/></SpeedActionTarget>"#,
    );
    assert!(message.contains("duplicate field"), "got: {message}");
}

// ---------------------------------------------------------------------------
// TrajectoryRef — XSD :2380-2385, bare xsd:choice
// ---------------------------------------------------------------------------

const TRAJECTORY_BRANCH: &str = r#"<TrajectoryRef><Trajectory name="t" closed="false"><Shape><Polyline><Vertex><Position><WorldPosition x="0" y="0"/></Position></Vertex><Vertex><Position><WorldPosition x="1" y="1"/></Position></Vertex></Polyline></Shape></Trajectory></TrajectoryRef>"#;

#[test]
fn trajectory_ref_inline_trajectory_branch_round_trips_byte_exact() {
    round_trips::<TrajectoryRef>(TRAJECTORY_BRANCH);
    let reference: TrajectoryRef = de(TRAJECTORY_BRANCH);
    assert!(reference.trajectory_ref.as_trajectory().is_some());
}

#[test]
fn trajectory_ref_catalog_reference_branch_round_trips_byte_exact() {
    let xml = r#"<TrajectoryRef><CatalogReference catalogName="TrajectoryCatalog" entryName="Lane"/></TrajectoryRef>"#;
    round_trips::<TrajectoryRef>(xml);
    let reference: TrajectoryRef = de(xml);
    assert!(matches!(
        reference.trajectory_ref,
        TrajectoryRefChoice::CatalogReference(_)
    ));
}

#[test]
fn trajectory_ref_rejects_zero_branches() {
    let message = err::<TrajectoryRef>("<TrajectoryRef/>");
    assert!(message.contains("$value"), "got: {message}");
}

#[test]
fn trajectory_ref_rejects_two_branches() {
    let two = TRAJECTORY_BRANCH.replace(
        "</TrajectoryRef>",
        r#"<CatalogReference catalogName="TrajectoryCatalog" entryName="Lane"/></TrajectoryRef>"#,
    );
    let message = err::<TrajectoryRef>(&two);
    assert!(message.contains("duplicate field"), "got: {message}");
}

// ---------------------------------------------------------------------------
// TimeReference — XSD :2173-2178, bare xsd:choice
// ---------------------------------------------------------------------------

#[test]
fn time_reference_none_branch_round_trips_byte_exact() {
    let xml = r#"<TimeReference><None/></TimeReference>"#;
    round_trips::<TimeReference>(xml);
    let reference: TimeReference = de(xml);
    assert!(matches!(
        reference.time_reference,
        TimeReferenceChoice::NoneElement(_)
    ));
}

#[test]
fn time_reference_timing_branch_round_trips_byte_exact() {
    let xml = r#"<TimeReference><Timing domainAbsoluteRelative="absolute" scale="1" offset="0"/></TimeReference>"#;
    round_trips::<TimeReference>(xml);
    // `Timing::new` is exercised here rather than a struct literal so the
    // constructor itself (not just the derived `Deserialize`) is covered.
    let built = TimeReference::timing(Timing::new(ReferenceContext::Absolute, 1.0, 0.0));
    assert_eq!(de::<TimeReference>(xml), built);
}

#[test]
fn time_reference_rejects_zero_branches() {
    let message = err::<TimeReference>("<TimeReference/>");
    assert!(message.contains("$value"), "got: {message}");
}

#[test]
fn time_reference_rejects_two_branches() {
    let message = err::<TimeReference>(
        r#"<TimeReference><None/><Timing domainAbsoluteRelative="absolute" scale="1" offset="0"/></TimeReference>"#,
    );
    assert!(message.contains("duplicate field"), "got: {message}");
}

// ---------------------------------------------------------------------------
// ControllerAction — XSD :978-984, bare xsd:choice
// ---------------------------------------------------------------------------

#[test]
fn controller_action_override_branch_round_trips_byte_exact() {
    let xml = r#"<ControllerAction><OverrideControllerValueAction><Throttle active="true" value="0.5"/></OverrideControllerValueAction></ControllerAction>"#;
    round_trips::<ControllerAction>(xml);
    let action: ControllerAction = de(xml);
    assert!(matches!(
        action.controller_action,
        ControllerActionChoice::OverrideControllerValueAction(_)
    ));
}

#[test]
fn controller_action_activate_branch_round_trips_byte_exact() {
    let xml = r#"<ControllerAction><ActivateControllerAction lateral="true"/></ControllerAction>"#;
    round_trips::<ControllerAction>(xml);
}

#[test]
fn controller_action_rejects_zero_branches() {
    let message = err::<ControllerAction>("<ControllerAction/>");
    assert!(message.contains("$value"), "got: {message}");
}

#[test]
fn controller_action_rejects_two_branches() {
    let message = err::<ControllerAction>(
        r#"<ControllerAction><OverrideControllerValueAction><Throttle active="true" value="0.5"/></OverrideControllerValueAction><ActivateControllerAction lateral="true"/></ControllerAction>"#,
    );
    assert!(message.contains("duplicate field"), "got: {message}");
}

// ---------------------------------------------------------------------------
// AbsoluteSpeed — XSD :672-677, xsd:sequence holding
// <xsd:group ref="SteadyState" minOccurs="0"/>. Absence is legal.
// ---------------------------------------------------------------------------

#[test]
fn absolute_speed_without_steady_state_is_legal_and_round_trips_byte_exact() {
    let xml = r#"<AbsoluteSpeed value="30"/>"#;
    round_trips::<AbsoluteSpeed>(xml);
    let speed: AbsoluteSpeed = de(xml);
    assert!(
        speed.steady_state.is_none(),
        "the group reference is minOccurs=0, so absence must parse"
    );
}

#[test]
fn absolute_speed_with_one_steady_state_branch_round_trips_byte_exact() {
    let xml = r#"<AbsoluteSpeed value="30"><TargetTimeSteadyState time="2"/></AbsoluteSpeed>"#;
    round_trips::<AbsoluteSpeed>(xml);
    let speed: AbsoluteSpeed = de(xml);
    assert!(matches!(
        speed.steady_state,
        Some(SteadyState::TargetTimeSteadyState(_))
    ));
}

#[test]
fn absolute_speed_rejects_two_steady_state_branches() {
    let message = err::<AbsoluteSpeed>(
        r#"<AbsoluteSpeed value="30"><TargetDistanceSteadyState distance="5"/><TargetTimeSteadyState time="2"/></AbsoluteSpeed>"#,
    );
    assert!(message.contains("duplicate field"), "got: {message}");
}

// ---------------------------------------------------------------------------
// RelativeSpeedToMaster — XSD :1889-1895, same optional group reference.
// ---------------------------------------------------------------------------

#[test]
fn relative_speed_to_master_without_steady_state_is_legal_and_round_trips_byte_exact() {
    let xml = r#"<RelativeSpeedToMaster speedTargetValueType="delta" value="-5"/>"#;
    round_trips::<RelativeSpeedToMaster>(xml);
    let speed: RelativeSpeedToMaster = de(xml);
    assert!(speed.steady_state.is_none());
}

#[test]
fn relative_speed_to_master_with_one_steady_state_branch_round_trips_byte_exact() {
    let xml = r#"<RelativeSpeedToMaster speedTargetValueType="delta" value="-5"><TargetDistanceSteadyState distance="5"/></RelativeSpeedToMaster>"#;
    round_trips::<RelativeSpeedToMaster>(xml);
    let speed: RelativeSpeedToMaster = de(xml);
    assert!(matches!(
        speed.steady_state,
        Some(SteadyState::TargetDistanceSteadyState(_))
    ));
}

#[test]
fn relative_speed_to_master_rejects_two_steady_state_branches() {
    let message = err::<RelativeSpeedToMaster>(
        r#"<RelativeSpeedToMaster speedTargetValueType="delta" value="-5"><TargetDistanceSteadyState distance="5"/><TargetTimeSteadyState time="2"/></RelativeSpeedToMaster>"#,
    );
    assert!(message.contains("duplicate field"), "got: {message}");
}

// ---------------------------------------------------------------------------
// AssignControllerAction — XSD :771-785. This is an xsd:choice, but every one
// of its branches carries minOccurs="0", so the group can match the empty
// sequence and the element legally carries no branch at all. `xmllint` against
// Schema/OpenSCENARIO.xsd agrees: <AssignControllerAction/> validates, and
// <Controller> beside <ObjectController> does not.
// ---------------------------------------------------------------------------

#[test]
fn assign_controller_action_without_a_branch_is_legal_and_round_trips_byte_exact() {
    let xml = r#"<AssignControllerAction/>"#;
    round_trips::<AssignControllerAction>(xml);
    let action: AssignControllerAction = de(xml);
    assert!(
        action.controller.is_none(),
        "every branch is minOccurs=0, so the empty element must parse"
    );
}

#[test]
fn assign_controller_action_with_attributes_but_no_branch_round_trips_byte_exact() {
    let xml = r#"<AssignControllerAction activateLateral="true" activateLongitudinal="false"/>"#;
    round_trips::<AssignControllerAction>(xml);
}

#[test]
fn assign_controller_action_with_one_branch_round_trips_byte_exact() {
    let xml = r#"<AssignControllerAction activateLateral="true"><ObjectController><Controller name="AIController" controllerType="movement"/></ObjectController></AssignControllerAction>"#;
    round_trips::<AssignControllerAction>(xml);
    let action: AssignControllerAction = de(xml);
    assert!(matches!(
        action.controller,
        Some(AssignControllerActionChoice::ObjectController(_))
    ));
}

#[test]
fn assign_controller_action_rejects_two_branches() {
    let message = err::<AssignControllerAction>(
        r#"<AssignControllerAction><Controller name="c" controllerType="movement"/><ObjectController><Controller name="d" controllerType="movement"/></ObjectController></AssignControllerAction>"#,
    );
    assert!(message.contains("duplicate field"), "got: {message}");
}

// ---------------------------------------------------------------------------
// Constructors reachable only through the Rust API, not through Deserialize.
//
// Every test above builds a value by parsing XML, so it exercises the derived
// `Deserialize` impl but never the hand-written `impl` blocks movement.rs
// defines beside each type (`::new`, the choice-branch builders, the
// `with_*` chain setters). Those constructors are the schema-fabrication
// guard this crate relies on instead of `Default` (see the removed-`Default`
// comments throughout movement.rs), so a wrong literal or a swapped field
// inside one would not be caught by any test above. Each row here builds a
// value with the constructor and checks it against an independently
// specified expectation (a parsed document or a field-by-field assertion),
// not against the constructor's own output.
// ---------------------------------------------------------------------------

#[test]
fn speed_action_target_choice_accessors_match_the_selected_branch() {
    let absolute = SpeedActionTargetChoice::AbsoluteTargetSpeed(AbsoluteTargetSpeed::new(30.0));
    assert!(absolute.as_absolute().is_some(), "absolute branch");
    assert!(
        absolute.as_relative().is_none(),
        "absolute branch has no relative view"
    );

    let relative = SpeedActionTargetChoice::RelativeTargetSpeed(RelativeTargetSpeed::new(
        2.0,
        "Ego",
        SpeedTargetValueType::Delta,
        true,
    ));
    assert_eq!(
        relative.as_relative().unwrap().entity_ref,
        "Ego",
        "as_relative must return the branch that was constructed"
    );
    assert!(
        relative.as_absolute().is_none(),
        "relative branch has no absolute view"
    );
}

#[test]
fn speed_action_target_relative_constructor_matches_the_parsed_document() {
    let built = SpeedActionTarget::relative(RelativeTargetSpeed::new(
        2.0,
        "Ego",
        SpeedTargetValueType::Delta,
        true,
    ));
    let xml = r#"<SpeedActionTarget><RelativeTargetSpeed value="2" entityRef="Ego" speedTargetValueType="delta" continuous="true"/></SpeedActionTarget>"#;
    assert_eq!(
        de::<SpeedActionTarget>(xml),
        built,
        "SpeedActionTarget::relative + RelativeTargetSpeed::new must match the XSD document"
    );
}

#[test]
fn trajectory_ref_choice_as_catalog_reference_matches_the_selected_branch() {
    let xml = r#"<TrajectoryRef><CatalogReference catalogName="TrajectoryCatalog" entryName="Lane"/></TrajectoryRef>"#;
    let reference: TrajectoryRef = de(xml);
    let catalog = reference
        .trajectory_ref
        .as_catalog_reference()
        .expect("catalog branch");
    assert_eq!(
        catalog.catalog_name.as_literal(),
        Some(&"TrajectoryCatalog".to_string())
    );
    assert!(
        reference.trajectory_ref.as_trajectory().is_none(),
        "the catalog branch has no inline-trajectory view"
    );
}

#[test]
fn transition_dynamics_with_following_mode_builder_matches_the_parsed_document() {
    let built = TransitionDynamics::new(DynamicsDimension::Time, DynamicsShape::Linear, 2.0)
        .with_following_mode(FollowingMode::Position);
    let xml = r#"<TransitionDynamics dynamicsDimension="time" dynamicsShape="linear" followingMode="position" value="2"/>"#;
    assert_eq!(de::<TransitionDynamics>(xml), built);
}

#[test]
fn assign_route_action_constructors_match_the_expected_branch() {
    let route = Route::new(
        "R1",
        false,
        vec![
            Waypoint::new(Position::world_origin(), RouteStrategy::Fastest),
            Waypoint::new(Position::world_origin(), RouteStrategy::Fastest),
        ],
    )
    .expect("two waypoints satisfy the XSD Route minOccurs=2 bound");

    let direct = AssignRouteAction::direct_route(route.clone());
    assert!(
        matches!(direct.route, RouteRef::Direct(_)),
        "direct_route must select the Direct branch"
    );

    let catalog = AssignRouteAction::catalog_route("RouteCatalog", "Loop");
    assert!(
        matches!(catalog.route, RouteRef::Catalog(_)),
        "catalog_route must select the Catalog branch"
    );

    let via_new = AssignRouteAction::new(RouteRef::Direct(route));
    assert_eq!(
        via_new.route, direct.route,
        "new(RouteRef::Direct(route)) must equal direct_route(route)"
    );
}

#[test]
fn lane_offset_action_with_continuous_builder_overrides_the_constructor_value() {
    let action = LaneOffsetAction::new(
        LaneOffsetActionDynamics::new(DynamicsShape::Linear),
        LaneOffsetTarget::absolute(1.0),
        false,
    )
    .with_continuous(true);
    assert_eq!(
        action.continuous.as_literal(),
        Some(&true),
        "with_continuous must overwrite the constructor's `continuous` argument"
    );
}

#[test]
fn relative_and_absolute_target_lane_offset_constructors_set_the_given_fields() {
    let relative = RelativeTargetLaneOffset::new("Ego", 1.5);
    assert_eq!(relative.entity_ref.as_literal(), Some(&"Ego".to_string()));
    assert_eq!(relative.value.as_literal(), Some(&1.5));

    let absolute = AbsoluteTargetLaneOffset::new(2.0);
    assert_eq!(absolute.value.as_literal(), Some(&2.0));
}

#[test]
fn longitudinal_distance_action_builder_chain_sets_both_optional_fields() {
    let action = LongitudinalDistanceAction::new("Ego", true, false)
        .with_distance(10.0)
        .with_time_gap(1.5);
    assert_eq!(action.distance.unwrap().as_literal(), Some(&10.0));
    assert_eq!(action.time_gap.unwrap().as_literal(), Some(&1.5));
    assert_eq!(action.freespace.as_literal(), Some(&true));
    assert_eq!(action.continuous.as_literal(), Some(&false));
}

#[test]
fn speed_profile_action_builder_chain_and_longitudinal_wrapper() {
    let entry = SpeedProfileEntry::new(10.0).with_time(2.0);
    assert_eq!(entry.time.as_ref().unwrap().as_literal(), Some(&2.0));
    assert_eq!(entry.speed.as_literal(), Some(&10.0));

    let profile = SpeedProfileAction::new(FollowingMode::Follow, vec![entry])
        .expect("one entry satisfies the XSD SpeedProfileEntry minOccurs=1 bound")
        .with_entity_ref("Ego");
    assert_eq!(
        profile.entity_ref.as_ref().unwrap().as_literal(),
        Some(&"Ego".to_string())
    );

    let wrapped = LongitudinalAction::speed_profile(profile);
    assert!(matches!(
        wrapped.longitudinal_action_choice,
        LongitudinalActionChoice::SpeedProfileAction(_)
    ));
}

#[test]
fn final_speed_constructors_match_the_selected_branch() {
    let absolute = FinalSpeed::absolute(AbsoluteSpeed::new(30.0));
    match absolute.speed_choice {
        FinalSpeedChoice::AbsoluteSpeed(speed) => {
            assert_eq!(speed.value.as_literal(), Some(&30.0));
        }
        other => panic!("expected AbsoluteSpeed, got {other:?}"),
    }

    let relative = FinalSpeed::relative(RelativeSpeedToMaster::new(
        SpeedTargetValueType::Delta,
        -5.0,
    ));
    match relative.speed_choice {
        FinalSpeedChoice::RelativeSpeedToMaster(speed) => {
            assert_eq!(speed.value.as_literal(), Some(&-5.0));
            assert_eq!(
                speed.speed_target_value_type,
                Value::Literal(SpeedTargetValueType::Delta)
            );
        }
        other => panic!("expected RelativeSpeedToMaster, got {other:?}"),
    }
}

#[test]
fn lane_change_target_lane_offset_accepts_empty_expression_and_parameter_forms() {
    // `deserialize_optional_double` (movement.rs) is the deserializer behind
    // `LaneChangeAction::target_lane_offset`. It handles four string shapes
    // beyond a plain literal: empty (-> None), `${expr}` (-> expression),
    // `$name` (-> parameter), and anything else unparsable (-> None, for XSD
    // compliance). `test_xml_deserialization` in movement.rs's own `mod
    // tests` already covers the plain-literal branch (`targetLaneOffset="0.5"`).
    fn parse_with_offset(offset_attr: &str) -> Option<Double> {
        let xml = format!(
            r#"<LaneChangeAction{offset_attr}><LaneChangeActionDynamics dynamicsDimension="time" dynamicsShape="linear" value="1"/><LaneChangeTarget><RelativeTargetLane entityRef="Ego" value="-1"/></LaneChangeTarget></LaneChangeAction>"#
        );
        de::<openscenario_rs::types::actions::movement::LaneChangeAction>(&xml).target_lane_offset
    }

    assert_eq!(
        parse_with_offset(r#" targetLaneOffset="""#),
        None,
        "an empty string must parse as absent, not an error"
    );
    assert_eq!(
        parse_with_offset(r#" targetLaneOffset="${1.0 + 1.0}""#),
        Some(Double::expression("1.0 + 1.0".to_string())),
        "a braced value is always the expression production, never a parameter"
    );
    assert_eq!(
        parse_with_offset(r#" targetLaneOffset="$offset""#),
        Some(Double::parameter("offset".to_string())),
        "a `$`-prefixed value is a parameter reference"
    );
    assert_eq!(
        parse_with_offset(r#" targetLaneOffset="not-a-number""#),
        None,
        "an unparsable, non-parameter, non-expression string falls back to None"
    );
}
