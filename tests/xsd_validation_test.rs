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
    FollowTrajectoryAction, LongitudinalDistanceAction, SpeedAction, SpeedProfileAction,
    TimeReference, Timing, Trajectory, TrajectoryFollowingMode, TrajectoryRef,
};
use openscenario_rs::types::basic::{Double, OSString};
use openscenario_rs::types::catalogs::references::CatalogReference;
use openscenario_rs::types::controllers::{Controller, ObjectController};
use openscenario_rs::types::enums::FollowingMode;
use openscenario_rs::types::scenario::init::{LongitudinalAction, PrivateAction};

#[test]
fn test_longitudinal_action_all_types() {
    // Test SpeedAction
    let speed_action = LongitudinalAction {
        speed_action: Some(SpeedAction::default()),
        longitudinal_distance_action: None,
        speed_profile_action: None,
    };
    assert!(speed_action.validate().is_ok());
    assert_eq!(speed_action.get_action_type(), Some("SpeedAction"));

    // Test LongitudinalDistanceAction
    let distance_action = LongitudinalAction {
        speed_action: None,
        longitudinal_distance_action: Some(LongitudinalDistanceAction::default()),
        speed_profile_action: None,
    };
    assert!(distance_action.validate().is_ok());
    assert_eq!(
        distance_action.get_action_type(),
        Some("LongitudinalDistanceAction")
    );

    // Test SpeedProfileAction
    let profile_action = LongitudinalAction {
        speed_action: None,
        longitudinal_distance_action: None,
        speed_profile_action: Some(SpeedProfileAction::default()),
    };
    assert!(profile_action.validate().is_ok());
    assert_eq!(profile_action.get_action_type(), Some("SpeedProfileAction"));
}

#[test]
fn test_follow_trajectory_action_complete() {
    // Test with direct trajectory
    let trajectory_action = FollowTrajectoryAction {
        trajectory: Some(Trajectory::default()),
        catalog_reference: None,
        time_reference: TimeReference {
            timing: Timing {
                domain_absolute_relative: OSString::literal("absolute".to_string()),
                scale: Double::literal(1.0),
                offset: Double::literal(0.0),
            },
        },
        trajectory_ref: None,
        trajectory_following_mode: TrajectoryFollowingMode {
            following_mode: FollowingMode::Follow,
        },
        initial_distance_offset: None,
    };
    assert!(trajectory_action.validate().is_ok());

    // Test with time reference
    let time_ref_action = FollowTrajectoryAction {
        trajectory: None,
        catalog_reference: None,
        time_reference: TimeReference {
            timing: Timing {
                domain_absolute_relative: OSString::literal("absolute".to_string()),
                scale: Double::literal(1.0),
                offset: Double::literal(0.0),
            },
        },
        trajectory_ref: None,
        trajectory_following_mode: TrajectoryFollowingMode::default(),
        initial_distance_offset: None,
    };
    assert!(time_ref_action.validate().is_ok());

    // Test with trajectory ref
    let traj_ref_action = FollowTrajectoryAction {
        trajectory: None,
        catalog_reference: None,
        time_reference: TimeReference {
            timing: Timing {
                domain_absolute_relative: OSString::literal("absolute".to_string()),
                scale: Double::literal(1.0),
                offset: Double::literal(0.0),
            },
        },
        trajectory_ref: Some(TrajectoryRef::default()),
        trajectory_following_mode: TrajectoryFollowingMode::default(),
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
            dynamics_dimension: DynamicsDimension::Time,
            dynamics_shape: DynamicsShape::Linear,
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
            dynamics_dimension: DynamicsDimension::Time,
            dynamics_shape: DynamicsShape::Linear,
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
    // Test valid ObjectController with direct controller
    let direct_controller = ObjectController {
        name: None,
        controller: Some(Controller::default()),
        catalog_reference: None,
    };
    assert!(direct_controller.validate().is_ok());

    // Test valid ObjectController with catalog reference
    let catalog_controller = ObjectController {
        name: None,
        controller: None,
        catalog_reference: Some(
            openscenario_rs::types::catalogs::references::ControllerCatalogReference::new(
                "ControllerCatalog".to_string(),
                "DefaultController".to_string(),
            ),
        ),
    };
    assert!(catalog_controller.validate().is_ok());

    // Test empty ObjectController (allowed for backward compatibility)
    let empty_controller = ObjectController {
        name: None,
        controller: None,
        catalog_reference: None,
    };
    assert!(empty_controller.validate().is_ok());
    // But strict validation should fail
    assert!(empty_controller.validate_strict().is_err());

    // Test invalid ObjectController with both
    let both_controller = ObjectController {
        name: None,
        controller: Some(Controller::default()),
        catalog_reference: Some(
            openscenario_rs::types::catalogs::references::ControllerCatalogReference::new(
                "ControllerCatalog".to_string(),
                "DefaultController".to_string(),
            ),
        ),
    };
    assert!(both_controller.validate().is_err());
}

#[test]
fn test_object_controller_deserialization() {
    // Test empty ObjectController (should succeed for backward compatibility)
    let empty_xml = r#"<ObjectController />"#;
    let result: Result<ObjectController, _> = quick_xml::de::from_str(empty_xml);
    assert!(
        result.is_ok(),
        "Empty ObjectController should succeed for backward compatibility"
    );
    let obj_controller = result.unwrap();
    assert!(obj_controller.controller.is_none());
    assert!(obj_controller.catalog_reference.is_none());
    assert!(obj_controller.validate().is_ok());
    assert!(obj_controller.validate_strict().is_err());

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
    assert!(obj_controller.controller.is_some());
    assert!(obj_controller.catalog_reference.is_none());

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
    assert!(obj_controller.controller.is_none());
    assert!(obj_controller.catalog_reference.is_some());

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

#[test]
fn test_private_action_choice_group() {
    // Test valid PrivateAction with exactly one action
    let valid_private = PrivateAction {
        longitudinal_action: Some(LongitudinalAction::default()),
        lateral_action: None,
        teleport_action: None,
        routing_action: None,
        synchronize_action: None,
        activate_controller_action: None,
    };
    assert!(valid_private.validate().is_ok());

    // Test invalid PrivateAction with multiple actions
    let invalid_private = PrivateAction {
        longitudinal_action: Some(LongitudinalAction::default()),
        lateral_action: Some(openscenario_rs::types::actions::movement::LateralAction::default()),
        teleport_action: None,
        routing_action: None,
        synchronize_action: None,
        activate_controller_action: None,
    };
    assert!(invalid_private.validate().is_err());
}

#[test]
fn test_xsd_compliance_serialization() {
    // Test that the new structures serialize correctly to XML
    let longitudinal_action = LongitudinalAction {
        speed_action: None,
        longitudinal_distance_action: Some(LongitudinalDistanceAction::default()),
        speed_profile_action: None,
    };

    let xml = quick_xml::se::to_string(&longitudinal_action).unwrap();
    assert!(xml.contains("LongitudinalDistanceAction"));
    assert!(!xml.contains("SpeedAction"));
    assert!(!xml.contains("SpeedProfileAction"));

    // Test round-trip serialization
    let deserialized: LongitudinalAction = quick_xml::de::from_str(&xml).unwrap();
    assert!(deserialized.longitudinal_distance_action.is_some());
    assert!(deserialized.speed_action.is_none());
    assert!(deserialized.speed_profile_action.is_none());
}

#[test]
fn test_validation_error_messages() {
    // Test that validation error messages are descriptive
    let empty_longitudinal = LongitudinalAction {
        speed_action: None,
        longitudinal_distance_action: None,
        speed_profile_action: None,
    };

    let error = empty_longitudinal.validate().unwrap_err();
    assert!(error.contains("exactly one action type"));
    assert!(error.contains("found none"));

    let multiple_longitudinal = LongitudinalAction {
        speed_action: Some(SpeedAction::default()),
        longitudinal_distance_action: Some(LongitudinalDistanceAction::default()),
        speed_profile_action: None,
    };

    let error = multiple_longitudinal.validate().unwrap_err();
    assert!(error.contains("exactly one action type"));
    assert!(error.contains("found multiple"));
}
