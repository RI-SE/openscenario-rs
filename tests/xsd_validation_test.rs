//! XSD Validation Fixes Test
//!
//! This test file validates the XSD validation fixes implemented to achieve 95%+ validation success rate.
//! Tests cover:
//! - LongitudinalAction with all three action types (SpeedAction, LongitudinalDistanceAction, SpeedProfileAction)
//! - FollowTrajectoryAction with complete trajectory source options
//! - Empty attribute handling for Double types
//! - ObjectController choice group validation
//! - Document structure validation

use openscenario_rs::types::actions::movement::{
    FollowTrajectoryAction, LongitudinalDistanceAction, NoneElement, SpeedAction,
    SpeedActionTarget, SpeedProfileAction, SpeedProfileEntry, TimeReference, Timing, Trajectory,
    TrajectoryFollowingMode, TrajectoryRef, TransitionDynamics,
};
use openscenario_rs::types::basic::Value;
use openscenario_rs::types::basic::{Double, MinVec, OSString};
use openscenario_rs::types::catalogs::references::CatalogReference;
use openscenario_rs::types::controllers::{Controller, ObjectController};
use openscenario_rs::types::enums::{
    ControllerType, DynamicsDimension, DynamicsShape, FollowingMode, ReferenceContext,
};
use openscenario_rs::types::geometry::shapes::{Polyline, Shape, Vertex};
use openscenario_rs::types::positions::Position;
use openscenario_rs::types::scenario::init::{LongitudinalAction, PrivateAction};

/// A minimal schema-valid `Shape` for tests that need a concrete trajectory shape but do
/// not exercise its content.
fn minimal_shape() -> Shape {
    // Two vertices: XSD `Polyline` (`Schema/OpenSCENARIO.xsd:1735`) declares `Vertex`
    // with `minOccurs="2"`.
    Shape::polyline(Polyline {
        vertices: MinVec::new(vec![
            Vertex::new(Position::world_origin()),
            Vertex::new(Position::world_origin()),
        ])
        .unwrap(),
    })
}

#[test]
fn test_longitudinal_action_all_types() {
    // Test SpeedAction
    let speed_action = LongitudinalAction::speed(SpeedAction::new(
        TransitionDynamics::new(DynamicsDimension::Time, DynamicsShape::Linear, 1.0),
        SpeedActionTarget::absolute(10.0),
    ));
    assert_eq!(speed_action.action_type(), "SpeedAction");

    // Test LongitudinalDistanceAction
    let distance_action = LongitudinalAction::longitudinal_distance(
        LongitudinalDistanceAction::new("DefaultEntity", true, false).with_distance(10.0),
    );
    assert_eq!(distance_action.action_type(), "LongitudinalDistanceAction");

    // Test SpeedProfileAction
    let profile_action = LongitudinalAction::speed_profile(
        SpeedProfileAction::new(FollowingMode::Follow, vec![SpeedProfileEntry::new(10.0)]).unwrap(),
    );
    assert_eq!(profile_action.action_type(), "SpeedProfileAction");
}

#[test]
fn test_follow_trajectory_action_complete() {
    // Test with direct trajectory
    let trajectory_action = FollowTrajectoryAction {
        trajectory: Some(Trajectory::new("TestTrajectory", false, minimal_shape())),
        catalog_reference: None,
        time_reference: TimeReference::timing(Timing {
            domain_absolute_relative: Value::Literal(ReferenceContext::Absolute),
            scale: Double::literal(1.0),
            offset: Double::literal(0.0),
        }),
        trajectory_ref: None,
        trajectory_following_mode: TrajectoryFollowingMode {
            following_mode: Value::Literal(FollowingMode::Follow),
        },
        initial_distance_offset: None,
    };
    assert!(trajectory_action.validate().is_ok());

    // Test with time reference
    let time_ref_action = FollowTrajectoryAction {
        trajectory: None,
        catalog_reference: None,
        time_reference: TimeReference::timing(Timing {
            domain_absolute_relative: Value::Literal(ReferenceContext::Absolute),
            scale: Double::literal(1.0),
            offset: Double::literal(0.0),
        }),
        trajectory_ref: None,
        trajectory_following_mode: TrajectoryFollowingMode::new(FollowingMode::Follow),
        initial_distance_offset: None,
    };
    assert!(time_ref_action.validate().is_ok());

    // Test with trajectory ref
    let traj_ref_action = FollowTrajectoryAction {
        trajectory: None,
        catalog_reference: None,
        time_reference: TimeReference::timing(Timing {
            domain_absolute_relative: Value::Literal(ReferenceContext::Absolute),
            scale: Double::literal(1.0),
            offset: Double::literal(0.0),
        }),
        trajectory_ref: Some(TrajectoryRef::with_trajectory(Trajectory::new(
            "TestTrajectory",
            false,
            minimal_shape(),
        ))),
        trajectory_following_mode: TrajectoryFollowingMode::new(FollowingMode::Follow),
        initial_distance_offset: None,
    };
    assert!(traj_ref_action.validate().is_ok());
}

#[test]
fn test_empty_attribute_handling() {
    // Test that empty strings for Double attributes are properly handled
    let xml_with_empty_double = r#"<LaneChangeAction targetLaneOffset="">
        <LaneChangeActionDynamics dynamicsDimension="time" dynamicsShape="linear" value="2.0" />
        <LaneChangeTarget>
            <RelativeTargetLane entityRef="Ego" value="-1" />
        </LaneChangeTarget>
    </LaneChangeAction>"#;

    // This should succeed by treating empty targetLaneOffset as None
    let result: Result<openscenario_rs::types::actions::movement::LaneChangeAction, _> =
        quick_xml::de::from_str(xml_with_empty_double);

    // The parsing should succeed with targetLaneOffset as None
    assert!(result.is_ok());
    let action = result.unwrap();
    assert!(action.target_lane_offset.is_none());
}

#[test]
fn test_lane_change_action_serialization_fixes() {
    use openscenario_rs::types::actions::movement::{
        LaneChangeAction, LaneChangeTarget, TransitionDynamics,
    };
    use openscenario_rs::types::enums::{DynamicsDimension, DynamicsShape};

    // Test 1: LaneChangeAction with None for target_lane_offset (should omit attribute completely)
    let lane_change_none = LaneChangeAction {
        target_lane_offset: None,
        lane_change_action_dynamics: TransitionDynamics {
            dynamics_dimension: Value::Literal(DynamicsDimension::Time),
            dynamics_shape: Value::Literal(DynamicsShape::Linear),
            following_mode: None,
            value: Double::literal(2.0),
        },
        lane_change_target: LaneChangeTarget::relative("Ego", -1),
    };

    let xml_none = quick_xml::se::to_string(&lane_change_none).unwrap();
    // Should not contain any targetLaneOffset attribute
    assert!(!xml_none.contains("targetLaneOffset"));
    println!("XML with None offset: {}", xml_none);

    // Test 2: LaneChangeAction with Some value for target_lane_offset (should include attribute)
    let lane_change_some = LaneChangeAction {
        target_lane_offset: Some(Double::literal(0.5)),
        lane_change_action_dynamics: TransitionDynamics {
            dynamics_dimension: Value::Literal(DynamicsDimension::Time),
            dynamics_shape: Value::Literal(DynamicsShape::Linear),
            following_mode: None,
            value: Double::literal(2.0),
        },
        lane_change_target: LaneChangeTarget::relative("Ego", -1),
    };

    let xml_some = quick_xml::se::to_string(&lane_change_some).unwrap();
    // Should contain the correct targetLaneOffset value
    assert!(xml_some.contains("targetLaneOffset=\"0.5\""));
    println!("XML with Some offset: {}", xml_some);

    // Test 3: Round-trip deserialization with empty targetLaneOffset=""
    let xml_empty = r#"<LaneChangeAction targetLaneOffset="">
        <LaneChangeActionDynamics dynamicsDimension="time" dynamicsShape="linear" value="2.0"/>
        <LaneChangeTarget>
            <RelativeTargetLane entityRef="Ego" value="-1"/>
        </LaneChangeTarget>
    </LaneChangeAction>"#;

    let deserialized: LaneChangeAction = quick_xml::de::from_str(xml_empty).unwrap();
    assert!(deserialized.target_lane_offset.is_none());

    // Test 4: Serialize the deserialized action (should not have targetLaneOffset)
    let reserialized = quick_xml::se::to_string(&deserialized).unwrap();
    assert!(!reserialized.contains("targetLaneOffset"));
    println!("Reserialized XML: {}", reserialized);
}

#[test]
fn test_object_controller_choice_group() {
    // XSD:1522-1528 is a bare choice with neither branch `minOccurs="0"`, so
    // exactly one of `Controller` or `CatalogReference` is constructible;
    // serde enforces this structurally, and there is no longer a `validate()`
    // to call. `ObjectControllerChoice` cannot represent zero or both.
    let direct_controller = ObjectController::with_controller(Controller::new(
        "TestController".to_string(),
        ControllerType::Movement,
    ));
    assert!(direct_controller.controller().is_some());

    let catalog_controller = ObjectController::with_catalog_reference(
        openscenario_rs::types::catalogs::references::ControllerCatalogReference::new(
            "ControllerCatalog".to_string(),
            "DefaultController".to_string(),
        ),
    );
    assert!(catalog_controller.catalog_reference().is_some());
}

#[test]
fn test_object_controller_deserialization() {
    // An empty `<ObjectController/>` is schema-invalid (XSD:1522-1528): the
    // pre-conversion `validate()` accepted it "for backward compatibility",
    // but zero of the 212-file conformance corpus contains one, and serde
    // now rejects it directly.
    let empty_xml = r#"<ObjectController />"#;
    let result: Result<ObjectController, _> = quick_xml::de::from_str(empty_xml);
    assert!(
        result.is_err(),
        "empty ObjectController is schema-invalid and must be rejected"
    );

    // Test ObjectController with Controller (should succeed)
    let controller_xml = r#"<ObjectController>
        <Controller name="TestController" />
    </ObjectController>"#;
    let result: Result<ObjectController, _> = quick_xml::de::from_str(controller_xml);
    assert!(
        result.is_ok(),
        "ObjectController with Controller should succeed"
    );
    let obj_controller = result.unwrap();
    assert!(obj_controller.controller().is_some());
    assert!(obj_controller.catalog_reference().is_none());

    // Test ObjectController with CatalogReference (should succeed)
    let catalog_xml = r#"<ObjectController>
        <CatalogReference catalogName="ControllerCatalog" entryName="DefaultController" />
    </ObjectController>"#;
    let result: Result<ObjectController, _> = quick_xml::de::from_str(catalog_xml);
    assert!(
        result.is_ok(),
        "ObjectController with CatalogReference should succeed"
    );
    let obj_controller = result.unwrap();
    assert!(obj_controller.controller().is_none());
    assert!(obj_controller.catalog_reference().is_some());

    // Test ObjectController with both (should fail during deserialization)
    let both_xml = r#"<ObjectController>
        <Controller name="TestController" />
        <CatalogReference catalogName="ControllerCatalog" entryName="DefaultController" />
    </ObjectController>"#;
    let result: Result<ObjectController, _> = quick_xml::de::from_str(both_xml);
    assert!(
        result.is_err(),
        "ObjectController with both should fail deserialization"
    );

    // Test ObjectController with name attribute
    let named_xml = r#"<ObjectController name="MyController">
        <Controller name="TestController" />
    </ObjectController>"#;
    let result: Result<ObjectController, _> = quick_xml::de::from_str(named_xml);
    assert!(result.is_ok(), "ObjectController with name should succeed");
    let obj_controller = result.unwrap();
    assert!(obj_controller.name.is_some());
    assert_eq!(
        obj_controller.name.unwrap().as_literal().unwrap(),
        "MyController"
    );
}

/// XSD `PrivateAction` (:1777-1791) is a bare `xsd:choice`. The valid case selects one
/// branch; the invalid cases are documents rather than values, since a value naming no
/// branch or two branches can no longer be constructed.
#[test]
fn test_private_action_choice_group() {
    let valid_private = PrivateAction::longitudinal(LongitudinalAction::speed(SpeedAction::new(
        TransitionDynamics::new(DynamicsDimension::Time, DynamicsShape::Linear, 1.0),
        SpeedActionTarget::absolute(10.0),
    )));
    assert_eq!(valid_private.action_type(), "LongitudinalAction");

    let lateral = PrivateAction::lateral(
        openscenario_rs::types::actions::movement::LateralAction::lane_change(
            openscenario_rs::types::actions::movement::LaneChangeAction::new(
                TransitionDynamics::new(DynamicsDimension::Time, DynamicsShape::Linear, 1.0),
                openscenario_rs::types::actions::movement::LaneChangeTarget::relative("Ego", -1),
            ),
        ),
    );
    assert_eq!(lateral.action_type(), "LateralAction");
}

#[test]
fn test_xsd_compliance_serialization() {
    // Test that the new structures serialize correctly to XML
    let longitudinal_action = LongitudinalAction::longitudinal_distance(
        LongitudinalDistanceAction::new("DefaultEntity", true, false).with_distance(10.0),
    );

    let xml = quick_xml::se::to_string(&longitudinal_action).unwrap();
    assert!(xml.contains("LongitudinalDistanceAction"));
    assert!(!xml.contains("SpeedProfileAction"));

    // Test round-trip serialization
    let deserialized: LongitudinalAction = quick_xml::de::from_str(&xml).unwrap();
    assert_eq!(deserialized, longitudinal_action);
    assert_eq!(deserialized.action_type(), "LongitudinalDistanceAction");
}

/// The hand-written `validate()` these assertions used to call is gone: serde now reports
/// the same two cases while deserializing, so they are stated as documents. Each case gets
/// its own test, since a single test would stop at the first failure.
#[test]
fn longitudinal_action_naming_no_branch_is_rejected() {
    let error = quick_xml::de::from_str::<LongitudinalAction>("<LongitudinalAction/>").unwrap_err();
    assert!(
        error.to_string().contains("missing field `$value`"),
        "got: {error}"
    );
}

#[test]
fn longitudinal_action_naming_two_branches_is_rejected() {
    let xml = r#"<LongitudinalAction><LongitudinalDistanceAction entityRef="DefaultEntity" distance="10" freespace="true" continuous="false"/><SpeedProfileAction followingMode="follow"><SpeedProfileEntry speed="10"/></SpeedProfileAction></LongitudinalAction>"#;
    let error = quick_xml::de::from_str::<LongitudinalAction>(xml).unwrap_err();
    assert!(
        error.to_string().contains("duplicate field `$value`"),
        "got: {error}"
    );
}
