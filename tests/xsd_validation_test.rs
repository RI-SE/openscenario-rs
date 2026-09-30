//! Wire-level fixes for two XSD shapes.
//!
//! - `LaneChangeAction/@targetLaneOffset` is optional: `None` omits the attribute, and an
//!   empty `targetLaneOffset=""` parses as absent.
//! - `ObjectController` (`Schema/OpenSCENARIO.xsd:1522-1528`) is a bare choice of
//!   `Controller` or `CatalogReference`: exactly one branch parses, zero or two are rejected.

use openscenario_rs::types::basic::{Double, Value};
use openscenario_rs::types::controllers::ObjectController;

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
}

#[test]
fn test_object_controller_deserialization() {
    // An empty `<ObjectController/>` is schema-invalid (XSD:1522-1528): the
    // pre-conversion `validate()` accepted it "for backward compatibility",
    // but zero of the 212-file conformance corpus contains one, and serde
    // now rejects it directly.
    let empty_xml = r#"<ObjectController />"#;
    let error = quick_xml::de::from_str::<ObjectController>(empty_xml).unwrap_err();
    assert!(
        error.to_string().contains("missing field `$value`"),
        "empty ObjectController is schema-invalid and must be rejected, got: {error}"
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
    let error = quick_xml::de::from_str::<ObjectController>(both_xml).unwrap_err();
    assert!(
        error.to_string().contains("duplicate field `$value`"),
        "ObjectController with both branches must be rejected, got: {error}"
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
