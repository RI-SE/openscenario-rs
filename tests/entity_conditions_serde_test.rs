//! XML serde tests for entity condition structs.
//!
//! Covers the fix described in docs/issues/issue-entity-condition-serde-renames.md:
//! Priority 3 — add missing `@`-prefixed and PascalCase `#[serde(rename = ...)]`
//! annotations to entity condition structs so that quick-xml 0.38 serialises
//! and deserialises them with the correct attribute / element names.
//!
//! # Test organisation
//!
//! - **round-trip** tests: serialise with `quick_xml::se::to_string`, deserialise with
//!   `quick_xml::de::from_str`, assert equality.  These catch both serialisation and
//!   deserialisation regressions.
//! - **raw XML deserialise** tests: parse a hard-coded XML string and assert field
//!   values.  These prove that the *correct* XSD attribute names are accepted.

use openscenario_rs::types::{
    basic::{Boolean, Double, Int, OSString},
    conditions::{
        AngleCondition, CollisionCondition, CollisionTarget, EndOfRoadCondition, OffroadCondition,
        RelativeAngleCondition, RelativeClearanceCondition, RelativeLaneRange,
        RelativeSpeedCondition, TimeToCollisionCondition, TimeToCollisionTarget,
    },
    enums::{AngleType, CoordinateSystem, DirectionalDimension, ObjectType, Rule},
    positions::Position,
    scenario::triggers::EntityRef,
};

// ─── helpers ─────────────────────────────────────────────────────────────────

fn round_trip<T>(original: &T) -> T
where
    T: serde::Serialize + for<'de> serde::Deserialize<'de> + std::fmt::Debug,
{
    let xml = quick_xml::se::to_string(original)
        .unwrap_or_else(|e| panic!("serialize failed for {}: {e}", std::any::type_name::<T>()));
    quick_xml::de::from_str(&xml)
        .unwrap_or_else(|e| panic!("deserialize failed for {}: {e}\nXML was: {xml}", std::any::type_name::<T>()))
}

// ─── EndOfRoadCondition ───────────────────────────────────────────────────────

#[test]
fn test_end_of_road_condition_xml_round_trip() {
    let original = EndOfRoadCondition::new(3.5);
    let deserialized: EndOfRoadCondition = round_trip(&original);
    assert_eq!(original, deserialized);
}

/// Confirm that the XSD attribute name `duration` is accepted by the deserialiser.
#[test]
fn test_end_of_road_condition_raw_xml_deserialize() {
    let xml = r#"<EndOfRoadCondition duration="3.0"/>"#;
    let condition: EndOfRoadCondition = quick_xml::de::from_str(xml)
        .expect("failed to deserialise EndOfRoadCondition from raw XML");
    assert_eq!(condition.duration, Double::literal(3.0));
}

// ─── OffroadCondition ─────────────────────────────────────────────────────────

#[test]
fn test_offroad_condition_xml_round_trip() {
    let original = OffroadCondition::new(2.0);
    let deserialized: OffroadCondition = round_trip(&original);
    assert_eq!(original, deserialized);
}

/// Confirm that the XSD attribute name `duration` is accepted by the deserialiser.
#[test]
fn test_offroad_condition_raw_xml_deserialize() {
    let xml = r#"<OffroadCondition duration="2.0"/>"#;
    let condition: OffroadCondition = quick_xml::de::from_str(xml)
        .expect("failed to deserialise OffroadCondition from raw XML");
    assert_eq!(condition.duration, Double::literal(2.0));
}

// ─── CollisionCondition ───────────────────────────────────────────────────────

#[test]
fn test_collision_condition_with_entity_ref_xml_round_trip() {
    let original = CollisionCondition::with_target("vehicle1");
    let deserialized: CollisionCondition = round_trip(&original);
    assert_eq!(original, deserialized);
}

#[test]
fn test_collision_condition_with_object_type_xml_round_trip() {
    let original = CollisionCondition::with_type(ObjectType::Vehicle);
    let deserialized: CollisionCondition = round_trip(&original);
    assert_eq!(original, deserialized);
}

#[test]
fn test_collision_condition_any_xml_round_trip() {
    let original = CollisionCondition::any_collision();
    let deserialized: CollisionCondition = round_trip(&original);
    assert_eq!(original, deserialized);
}

#[test]
fn test_collision_condition_serializes_entity_ref_element() {
    let condition = CollisionCondition::with_target("ego");
    let xml = quick_xml::se::to_string(&condition).expect("serialize failed");
    // The child element must be named EntityRef, not "target"
    assert!(
        xml.contains("EntityRef"),
        "Expected <EntityRef ...> in XML, got: {xml}"
    );
    assert!(
        xml.contains("ego"),
        "Expected entity value 'ego' in XML, got: {xml}"
    );
    assert!(
        !xml.contains("<target"),
        "Old name '<target' must not appear in XML, got: {xml}"
    );
}

#[test]
fn test_collision_condition_serializes_by_object_type_element() {
    let condition = CollisionCondition::with_type(ObjectType::Pedestrian);
    let xml = quick_xml::se::to_string(&condition).expect("serialize failed");
    // XSD:926 - the child element is named `ByType` (of XSD type `ByObjectType`)
    assert!(
        xml.contains("ByType"),
        "Expected <ByType ...> in XML, got: {xml}"
    );
    assert!(
        xml.contains("type=\"pedestrian\""),
        "Expected attribute 'type' in XML, got: {xml}"
    );
    assert!(
        !xml.contains("<by_type"),
        "Old name '<by_type' must not appear in XML, got: {xml}"
    );
}

// ─── CollisionTarget ─────────────────────────────────────────────────────────

#[test]
fn test_collision_target_xml_round_trip() {
    let original = CollisionTarget {
        target_type: ObjectType::Vehicle,
    };
    let deserialized: CollisionTarget = round_trip(&original);
    assert_eq!(original, deserialized);
}

#[test]
fn test_collision_target_serializes_object_type_attribute() {
    let target = CollisionTarget {
        target_type: ObjectType::Pedestrian,
    };
    let xml = quick_xml::se::to_string(&target).expect("serialize failed");
    // XSD:832 - complexType ByObjectType has required attribute `type`
    assert!(
        xml.contains("type=\"pedestrian\""),
        "Expected type attribute, got: {xml}"
    );
    assert!(
        !xml.contains("target_type"),
        "Old field name 'target_type' must not appear in XML, got: {xml}"
    );
}

// ─── TimeToCollisionTarget ───────────────────────────────────────────────────

#[test]
fn test_time_to_collision_target_entity_xml_round_trip() {
    let original = TimeToCollisionTarget::entity("lead_vehicle");
    let deserialized: TimeToCollisionTarget = round_trip(&original);
    assert_eq!(original, deserialized);
}

#[test]
fn test_time_to_collision_target_position_xml_round_trip() {
    let original = TimeToCollisionTarget::position(Position::default());
    let deserialized: TimeToCollisionTarget = round_trip(&original);
    assert_eq!(original, deserialized);
}

#[test]
fn test_time_to_collision_target_serializes_entity_ref_element() {
    let target = TimeToCollisionTarget::entity("front_car");
    let xml = quick_xml::se::to_string(&target).expect("serialize failed");
    assert!(
        xml.contains("EntityRef"),
        "Expected <EntityRef ...> element, got: {xml}"
    );
    assert!(
        !xml.contains("<entity_ref"),
        "Old name '<entity_ref' must not appear in XML, got: {xml}"
    );
}

// ─── TimeToCollisionCondition ────────────────────────────────────────────────

#[test]
fn test_time_to_collision_condition_entity_target_xml_round_trip() {
    let original =
        TimeToCollisionCondition::with_entity_target("lead_vehicle", 3.0, Rule::LessThan, true);
    let deserialized: TimeToCollisionCondition = round_trip(&original);
    assert_eq!(original, deserialized);
}

#[test]
fn test_time_to_collision_condition_position_target_xml_round_trip() {
    let original =
        TimeToCollisionCondition::with_position_target(Position::default(), 2.5, Rule::LessThan, false);
    let deserialized: TimeToCollisionCondition = round_trip(&original);
    assert_eq!(original, deserialized);
}

#[test]
fn test_time_to_collision_condition_serializes_correct_target_element() {
    let condition =
        TimeToCollisionCondition::with_entity_target("target_car", 5.0, Rule::LessThan, true);
    let xml = quick_xml::se::to_string(&condition).expect("serialize failed");
    // The wrapper element must be TimeToCollisionConditionTarget, not "target"
    assert!(
        xml.contains("TimeToCollisionConditionTarget"),
        "Expected TimeToCollisionConditionTarget element, got: {xml}"
    );
    assert!(
        !xml.contains("<target"),
        "Old name '<target' must not appear in XML, got: {xml}"
    );
}

// ─── AngleCondition ───────────────────────────────────────────────────────────

#[test]
fn test_angle_condition_xml_round_trip() {
    let original = AngleCondition {
        angle_type: AngleType::Heading,
        angle: Double::literal(1.57),
        angle_tolerance: Double::literal(0.1),
        coordinate_system: None,
    };
    let deserialized: AngleCondition = round_trip(&original);
    assert_eq!(original, deserialized);
}

#[test]
#[allow(clippy::approx_constant)] // 3.14 is a scenario angle value, not an approximation of PI
fn test_angle_condition_with_coordinate_system_xml_round_trip() {
    let original = AngleCondition {
        angle_type: AngleType::Pitch,
        angle: Double::literal(3.14),
        angle_tolerance: Double::literal(0.05),
        coordinate_system: Some(CoordinateSystem::Entity),
    };
    let deserialized: AngleCondition = round_trip(&original);
    assert_eq!(original, deserialized);
}

/// Confirm that the XSD attribute names `angleType`, `angle`, `angleTolerance`
/// are accepted by the deserialiser (not the snake_case field names).
#[test]
fn test_angle_condition_raw_xml_deserialize_attributes() {
    let xml = r#"<AngleCondition angleType="heading" angle="1.57" angleTolerance="0.1"/>"#;
    let condition: AngleCondition = quick_xml::de::from_str(xml)
        .expect("failed to deserialise AngleCondition from raw XML");
    assert_eq!(condition.angle_type, AngleType::Heading);
    assert_eq!(condition.angle, Double::literal(1.57));
    assert_eq!(condition.angle_tolerance, Double::literal(0.1));
    assert_eq!(condition.coordinate_system, None);
}

/// Confirm optional `coordinateSystem` attribute is accepted.
#[test]
#[allow(clippy::approx_constant)] // 3.14 is a scenario angle value, not an approximation of PI
fn test_angle_condition_raw_xml_deserialize_with_coordinate_system() {
    let xml = r#"<AngleCondition angleType="pitch" angle="3.14" angleTolerance="0.05" coordinateSystem="entity"/>"#;
    let condition: AngleCondition = quick_xml::de::from_str(xml)
        .expect("failed to deserialise AngleCondition with coordinateSystem");
    assert_eq!(condition.angle_type, AngleType::Pitch);
    assert_eq!(condition.angle, Double::literal(3.14));
    assert_eq!(condition.angle_tolerance, Double::literal(0.05));
    assert_eq!(condition.coordinate_system, Some(CoordinateSystem::Entity));
}

#[test]
fn test_angle_condition_serializes_correct_attribute_names() {
    let condition = AngleCondition {
        angle_type: AngleType::Heading,
        angle: Double::literal(1.0),
        angle_tolerance: Double::literal(0.2),
        coordinate_system: Some(CoordinateSystem::Road),
    };
    let xml = quick_xml::se::to_string(&condition).expect("serialize failed");
    assert!(xml.contains("angleType="), "Expected angleType attribute, got: {xml}");
    assert!(xml.contains("angleTolerance="), "Expected angleTolerance attribute, got: {xml}");
    assert!(xml.contains("coordinateSystem="), "Expected coordinateSystem attribute, got: {xml}");
    // Old snake_case names must not appear
    assert!(!xml.contains("angle_type="), "Old 'angle_type' must not appear, got: {xml}");
    assert!(!xml.contains("angle_tolerance="), "Old 'angle_tolerance' must not appear, got: {xml}");
    assert!(!xml.contains("coordinate_system="), "Old 'coordinate_system' must not appear, got: {xml}");
}

// ─── RelativeSpeedCondition ───────────────────────────────────────────────────

#[test]
fn test_relative_speed_condition_xml_round_trip() {
    let original = RelativeSpeedCondition {
        entity_ref: OSString::literal("front_vehicle".to_string()),
        rule: Rule::LessThan,
        value: Double::literal(5.0),
        direction: None,
    };
    let deserialized: RelativeSpeedCondition = round_trip(&original);
    assert_eq!(original, deserialized);
}

#[test]
fn test_relative_speed_condition_with_direction_xml_round_trip() {
    let original = RelativeSpeedCondition {
        entity_ref: OSString::literal("car_ahead".to_string()),
        rule: Rule::GreaterThan,
        value: Double::literal(2.0),
        direction: Some(DirectionalDimension::Longitudinal),
    };
    let deserialized: RelativeSpeedCondition = round_trip(&original);
    assert_eq!(original, deserialized);
}

#[test]
fn test_relative_speed_condition_serializes_correct_attribute_names() {
    let condition = RelativeSpeedCondition {
        entity_ref: OSString::literal("target".to_string()),
        rule: Rule::EqualTo,
        value: Double::literal(0.0),
        direction: None,
    };
    let xml = quick_xml::se::to_string(&condition).expect("serialize failed");
    assert!(xml.contains("entityRef="), "Expected entityRef attribute, got: {xml}");
    assert!(xml.contains("rule="), "Expected rule attribute, got: {xml}");
    assert!(xml.contains("value="), "Expected value attribute, got: {xml}");
    // Old snake_case field names must not appear
    assert!(!xml.contains("entity_ref="), "Old 'entity_ref' must not appear, got: {xml}");
}

// ─── RelativeLaneRange ────────────────────────────────────────────────────────

#[test]
fn test_relative_lane_range_xml_round_trip() {
    let original = RelativeLaneRange {
        from: Some(Int::literal(-1)),
        to: Some(Int::literal(2)),
    };
    let deserialized: RelativeLaneRange = round_trip(&original);
    assert_eq!(original, deserialized);
}

#[test]
fn test_relative_lane_range_empty_xml_round_trip() {
    let original = RelativeLaneRange { from: None, to: None };
    let deserialized: RelativeLaneRange = round_trip(&original);
    assert_eq!(original, deserialized);
}

#[test]
fn test_relative_lane_range_serializes_correct_attribute_names() {
    let range = RelativeLaneRange {
        from: Some(Int::literal(-2)),
        to: Some(Int::literal(1)),
    };
    let xml = quick_xml::se::to_string(&range).expect("serialize failed");
    assert!(xml.contains("from=\""), "Expected from attribute, got: {xml}");
    assert!(xml.contains("to=\""), "Expected to attribute, got: {xml}");
}

// ─── RelativeClearanceCondition ───────────────────────────────────────────────

#[test]
fn test_relative_clearance_condition_xml_round_trip() {
    let original = RelativeClearanceCondition {
        relative_lane_ranges: vec![RelativeLaneRange {
            from: Some(Int::literal(-1)),
            to: Some(Int::literal(1)),
        }],
        entity_refs: vec![EntityRef {
            entity_ref: OSString::literal("target_vehicle".to_string()),
        }],
        opposite_lanes: Boolean::literal(false),
        distance_forward: Some(Double::literal(50.0)),
        distance_backward: Some(Double::literal(10.0)),
        free_space: Boolean::literal(true),
    };
    let deserialized: RelativeClearanceCondition = round_trip(&original);
    assert_eq!(original, deserialized);
}

#[test]
fn test_relative_clearance_condition_minimal_xml_round_trip() {
    // Minimal: no lane ranges, no entity refs, no optional distances
    let original = RelativeClearanceCondition {
        relative_lane_ranges: vec![],
        entity_refs: vec![],
        opposite_lanes: Boolean::literal(true),
        distance_forward: None,
        distance_backward: None,
        free_space: Boolean::literal(false),
    };
    let deserialized: RelativeClearanceCondition = round_trip(&original);
    assert_eq!(original, deserialized);
}

/// Confirm the XSD attribute names `oppositeLanes`, `freeSpace`, `distanceForward`,
/// `distanceBackward` and child element names `RelativeLaneRange`, `EntityRef`
/// are all correctly parsed from raw XML.
#[test]
fn test_relative_clearance_condition_raw_xml_deserialize() {
    let xml = r#"<RelativeClearanceCondition oppositeLanes="false" freeSpace="true" distanceForward="50.0" distanceBackward="10.0">
        <RelativeLaneRange from="-1" to="1"/>
        <EntityRef entityRef="target"/>
    </RelativeClearanceCondition>"#;

    let condition: RelativeClearanceCondition = quick_xml::de::from_str(xml)
        .expect("failed to deserialise RelativeClearanceCondition from raw XML");

    assert_eq!(condition.opposite_lanes, Boolean::literal(false));
    assert_eq!(condition.free_space, Boolean::literal(true));
    assert_eq!(condition.distance_forward, Some(Double::literal(50.0)));
    assert_eq!(condition.distance_backward, Some(Double::literal(10.0)));

    assert_eq!(condition.relative_lane_ranges.len(), 1);
    assert_eq!(condition.relative_lane_ranges[0].from, Some(Int::literal(-1)));
    assert_eq!(condition.relative_lane_ranges[0].to, Some(Int::literal(1)));

    assert_eq!(condition.entity_refs.len(), 1);
    assert_eq!(
        condition.entity_refs[0].entity_ref,
        OSString::literal("target".to_string())
    );
}

#[test]
fn test_relative_clearance_condition_serializes_correct_attribute_names() {
    let condition = RelativeClearanceCondition {
        relative_lane_ranges: vec![],
        entity_refs: vec![],
        opposite_lanes: Boolean::literal(false),
        distance_forward: Some(Double::literal(30.0)),
        distance_backward: None,
        free_space: Boolean::literal(true),
    };
    let xml = quick_xml::se::to_string(&condition).expect("serialize failed");
    assert!(xml.contains("oppositeLanes="), "Expected oppositeLanes attribute, got: {xml}");
    assert!(xml.contains("freeSpace="), "Expected freeSpace attribute, got: {xml}");
    assert!(xml.contains("distanceForward="), "Expected distanceForward attribute, got: {xml}");
    // Old snake_case names must not appear
    assert!(!xml.contains("opposite_lanes="), "Old 'opposite_lanes' must not appear, got: {xml}");
    assert!(!xml.contains("free_space="), "Old 'free_space' must not appear, got: {xml}");
    assert!(!xml.contains("distance_forward="), "Old 'distance_forward' must not appear, got: {xml}");
}

// ─── RelativeAngleCondition ───────────────────────────────────────────────────

#[test]
fn test_relative_angle_condition_xml_round_trip() {
    let original = RelativeAngleCondition {
        entity_ref: OSString::literal("lead_vehicle".to_string()),
        angle_type: AngleType::Heading,
        angle: Double::literal(0.5),
        angle_tolerance: Double::literal(0.1),
        coordinate_system: None,
    };
    let deserialized: RelativeAngleCondition = round_trip(&original);
    assert_eq!(original, deserialized);
}

#[test]
fn test_relative_angle_condition_with_coordinate_system_xml_round_trip() {
    let original = RelativeAngleCondition {
        entity_ref: OSString::literal("ref_entity".to_string()),
        angle_type: AngleType::Pitch,
        angle: Double::literal(1.0),
        angle_tolerance: Double::literal(0.2),
        coordinate_system: Some(CoordinateSystem::Lane),
    };
    let deserialized: RelativeAngleCondition = round_trip(&original);
    assert_eq!(original, deserialized);
}

#[test]
fn test_relative_angle_condition_serializes_correct_attribute_names() {
    let condition = RelativeAngleCondition {
        entity_ref: OSString::literal("car".to_string()),
        angle_type: AngleType::Heading,
        angle: Double::literal(0.0),
        angle_tolerance: Double::literal(0.1),
        coordinate_system: None,
    };
    let xml = quick_xml::se::to_string(&condition).expect("serialize failed");
    assert!(xml.contains("entityRef="), "Expected entityRef attribute, got: {xml}");
    assert!(xml.contains("angleType="), "Expected angleType attribute, got: {xml}");
    assert!(xml.contains("angleTolerance="), "Expected angleTolerance attribute, got: {xml}");
    // Old snake_case names must not appear as attribute names
    assert!(!xml.contains("entity_ref="), "Old 'entity_ref=' must not appear, got: {xml}");
    assert!(!xml.contains("angle_type="), "Old 'angle_type=' must not appear, got: {xml}");
    assert!(!xml.contains("angle_tolerance="), "Old 'angle_tolerance=' must not appear, got: {xml}");
}
