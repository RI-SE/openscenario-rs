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
    AbsoluteSpeed, RelativeSpeedToMaster, SpeedActionTarget, SpeedActionTargetChoice, SteadyState,
    TimeReference, TimeReferenceChoice, Timing, TrajectoryRef, TrajectoryRefChoice,
};
use openscenario_rs::types::basic::{Double, Value};
use openscenario_rs::types::enums::ReferenceContext;

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
    let built = TimeReference::timing(Timing {
        domain_absolute_relative: Value::Literal(ReferenceContext::Absolute),
        scale: Double::literal(1.0),
        offset: Double::literal(0.0),
    });
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
