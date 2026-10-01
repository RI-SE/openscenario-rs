//! Wire form of the entity conditions (XSD `EntityCondition`, :1135-1156) and of the
//! `ByEntityCondition` wrapper (:825-830).
//!
//! Every row goes through [`assert_wire`]: the schema-valid XML must parse to exactly the value
//! the public constructor builds, and that value must serialize back to exactly the same XML.
//! A wrong element or attribute name, a constructor that stores the wrong field or enum, or a
//! serializer arm that emits the wrong tag fails the row. Add a row here for a new condition
//! or constructor rather than a new test function.

use openscenario_rs::types::basic::{Boolean, Double};
use openscenario_rs::types::conditions::{
    AccelerationCondition, AngleCondition, ByEntityCondition, CollisionCondition,
    CollisionConditionChoice, CollisionTarget, EndOfRoadCondition, EntityCondition,
    OffroadCondition, ReachPositionCondition, RelativeAngleCondition, RelativeClearanceCondition,
    RelativeLaneRange, RelativeSpeedCondition, SpeedCondition, StandStillCondition,
    TimeHeadwayCondition, TimeToCollisionCondition, TimeToCollisionTarget,
    TraveledDistanceCondition,
};
use openscenario_rs::types::enums::{
    AngleType, CoordinateSystem, DirectionalDimension, ObjectType, RelativeDistanceType,
    RoutingAlgorithm, Rule,
};
use openscenario_rs::types::positions::{Position, WorldPosition};
use openscenario_rs::types::scenario::triggers::{EntityRef, TriggeringEntities};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

fn assert_wire<T>(xml: &str, expected: &T)
where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    let parsed: T =
        quick_xml::de::from_str(xml).unwrap_or_else(|e| panic!("failed to parse {xml}: {e}"));
    assert_eq!(&parsed, expected, "parsed value differs for {xml}");
    let emitted = quick_xml::se::to_string(expected).expect("serialize condition");
    assert_eq!(emitted, xml);
}

const ORIGIN: &str = r#"<Position><WorldPosition x="0" y="0"/></Position>"#;

/// `EntityCondition` has no root tag of its own, so its rows are wrapped in the
/// `ByEntityCondition` that carries it in a document.
fn by_entity_xml(entity_condition: &str) -> String {
    format!(
        r#"<ByEntityCondition><TriggeringEntities triggeringEntitiesRule="any"><EntityRef entityRef="Ego"/></TriggeringEntities><EntityCondition>{entity_condition}</EntityCondition></ByEntityCondition>"#
    )
}

fn ego() -> TriggeringEntities {
    TriggeringEntities::any(vec![EntityRef::new("Ego")]).unwrap()
}

#[test]
fn motion_conditions_match_xsd_wire_form() {
    // AccelerationCondition XSD:687-691
    assert_wire(
        r#"<AccelerationCondition value="5" rule="greaterThan"/>"#,
        &AccelerationCondition::new(5.0, Rule::GreaterThan),
    );
    assert_wire(
        r#"<AccelerationCondition value="2.5" rule="greaterThan"/>"#,
        &AccelerationCondition::greater_than(2.5),
    );
    assert_wire(
        r#"<AccelerationCondition value="1" rule="lessThan"/>"#,
        &AccelerationCondition::less_than(1.0),
    );
    assert_wire(
        r#"<AccelerationCondition value="4" rule="equalTo" direction="longitudinal"/>"#,
        &AccelerationCondition::longitudinal(4.0, Rule::EqualTo),
    );
    assert_wire(
        r#"<AccelerationCondition value="2" rule="greaterOrEqual" direction="lateral"/>"#,
        &AccelerationCondition::lateral(2.0, Rule::GreaterOrEqual),
    );
    assert_wire(
        r#"<AccelerationCondition value="1.5" rule="lessOrEqual" direction="vertical"/>"#,
        &AccelerationCondition::vertical(1.5, Rule::LessOrEqual),
    );
    // StandStillCondition XSD:2072-2074
    assert_wire(
        r#"<StandStillCondition duration="3"/>"#,
        &StandStillCondition::new(3.0),
    );
    assert_wire(
        r#"<StandStillCondition duration="5.5"/>"#,
        &StandStillCondition::with_duration(5.5),
    );
    // SpeedCondition XSD:2055-2059
    assert_wire(
        r#"<SpeedCondition value="10" rule="greaterThan"/>"#,
        &SpeedCondition::new(10.0, Rule::GreaterThan),
    );
    assert_wire(
        r#"<SpeedCondition value="8" rule="lessThan" direction="longitudinal"/>"#,
        &SpeedCondition::new(8.0, Rule::LessThan)
            .with_direction(DirectionalDimension::Longitudinal),
    );
    // TraveledDistanceCondition XSD:2392-2394
    assert_wire(
        r#"<TraveledDistanceCondition value="120"/>"#,
        &TraveledDistanceCondition::new(120.0),
    );
}

#[test]
fn road_and_collision_conditions_match_xsd_wire_form() {
    // EndOfRoadCondition XSD:1119-1121, OffroadCondition XSD:1529-1531
    assert_wire(
        r#"<EndOfRoadCondition duration="1.5"/>"#,
        &EndOfRoadCondition::new(1.5),
    );
    assert_wire(
        r#"<EndOfRoadCondition duration="3"/>"#,
        &EndOfRoadCondition::with_duration(3.0),
    );
    assert_wire(
        r#"<OffroadCondition duration="2.5"/>"#,
        &OffroadCondition::new(2.5),
    );
    assert_wire(
        r#"<OffroadCondition duration="4"/>"#,
        &OffroadCondition::with_duration(4.0),
    );
    // CollisionCondition XSD:923-928; the ByType child is XSD type ByObjectType (:831-833)
    assert_wire(
        r#"<CollisionCondition><EntityRef entityRef="Target"/></CollisionCondition>"#,
        &CollisionCondition::with_target("Target"),
    );
    assert_wire(
        r#"<CollisionCondition><ByType type="pedestrian"/></CollisionCondition>"#,
        &CollisionCondition::with_type(ObjectType::Pedestrian),
    );
    assert_wire(
        r#"<CollisionCondition><ByType type="vehicle"/></CollisionCondition>"#,
        &CollisionCondition {
            choice: CollisionConditionChoice::ByType(CollisionTarget::new(ObjectType::Vehicle)),
        },
    );
}

#[test]
fn headway_and_time_to_collision_conditions_match_xsd_wire_form() {
    // TimeHeadwayCondition XSD:2153-2164
    assert_wire(
        r#"<TimeHeadwayCondition entityRef="Lead" value="2.5" rule="greaterThan" freespace="true"/>"#,
        &TimeHeadwayCondition::new("Lead", 2.5, Rule::GreaterThan, true),
    );
    assert_wire(
        r#"<TimeHeadwayCondition entityRef="Lead" value="1.8" rule="lessThan" freespace="false"/>"#,
        &TimeHeadwayCondition::less_than("Lead", 1.8, false),
    );
    assert_wire(
        r#"<TimeHeadwayCondition entityRef="Lead" value="3" rule="greaterThan" freespace="true" coordinateSystem="road" relativeDistanceType="lateral" routingAlgorithm="fastest"/>"#,
        &TimeHeadwayCondition::greater_than("Lead", 3.0, true)
            .with_coordinate_system(CoordinateSystem::Road)
            .with_distance_type(RelativeDistanceType::Lateral)
            .with_routing_algorithm(RoutingAlgorithm::Fastest),
    );
    // alongRoute is deprecated but still schema-valid (XSD:2155)
    assert_wire(
        r#"<TimeHeadwayCondition entityRef="Lead" value="1.5" rule="lessThan" freespace="true" alongRoute="true"/>"#,
        &TimeHeadwayCondition {
            along_route: Some(Boolean::literal(true)),
            ..TimeHeadwayCondition::new("Lead", 1.5, Rule::LessThan, true)
        },
    );

    // TimeToCollisionCondition XSD:2179-2192, target choice XSD:2193-2198
    assert_wire(
        r#"<TimeToCollisionCondition value="3" rule="lessThan" freespace="true"><TimeToCollisionConditionTarget><EntityRef entityRef="Lead"/></TimeToCollisionConditionTarget></TimeToCollisionCondition>"#,
        &TimeToCollisionCondition::with_entity_target("Lead", 3.0, Rule::LessThan, true),
    );
    assert_wire(
        &format!(
            r#"<TimeToCollisionCondition value="2.5" rule="equalTo" freespace="false"><TimeToCollisionConditionTarget>{ORIGIN}</TimeToCollisionConditionTarget></TimeToCollisionCondition>"#
        ),
        &TimeToCollisionCondition::with_position_target(
            Position::world_origin(),
            2.5,
            Rule::EqualTo,
            false,
        ),
    );
    assert_wire(
        r#"<TimeToCollisionCondition value="2" rule="lessThan" freespace="true" coordinateSystem="lane" relativeDistanceType="longitudinal" routingAlgorithm="shortest"><TimeToCollisionConditionTarget><EntityRef entityRef="Obstacle"/></TimeToCollisionConditionTarget></TimeToCollisionCondition>"#,
        &TimeToCollisionCondition::entity_less_than("Obstacle", 2.0, true)
            .with_coordinate_system(CoordinateSystem::Lane)
            .with_distance_type(RelativeDistanceType::Longitudinal)
            .with_routing_algorithm(RoutingAlgorithm::Shortest),
    );
    assert_wire(
        r#"<TimeToCollisionCondition value="4" rule="greaterThan" freespace="false"><TimeToCollisionConditionTarget><EntityRef entityRef="Pedestrian1"/></TimeToCollisionConditionTarget></TimeToCollisionCondition>"#,
        &TimeToCollisionCondition::entity_greater_than("Pedestrian1", 4.0, false),
    );
    assert_wire(
        &format!(
            r#"<TimeToCollisionCondition value="1.5" rule="lessThan" freespace="true"><TimeToCollisionConditionTarget>{ORIGIN}</TimeToCollisionConditionTarget></TimeToCollisionCondition>"#
        ),
        &TimeToCollisionCondition::position_less_than(Position::world_origin(), 1.5, true),
    );
    assert_wire(
        &format!(
            r#"<TimeToCollisionCondition value="6" rule="greaterThan" freespace="false"><TimeToCollisionConditionTarget>{ORIGIN}</TimeToCollisionConditionTarget></TimeToCollisionCondition>"#
        ),
        &TimeToCollisionCondition::position_greater_than(Position::world_origin(), 6.0, false),
    );
    // alongRoute is deprecated but still schema-valid (XSD:2183)
    assert_wire(
        r#"<TimeToCollisionCondition value="3" rule="lessThan" freespace="true" alongRoute="false"><TimeToCollisionConditionTarget><EntityRef entityRef="Ego"/></TimeToCollisionConditionTarget></TimeToCollisionCondition>"#,
        &TimeToCollisionCondition {
            along_route: Some(Boolean::literal(false)),
            ..TimeToCollisionCondition::with_entity_target("Ego", 3.0, Rule::LessThan, true)
        },
    );
    // The target is never a document root; standalone it serializes under its Rust type name.
    assert_wire(
        r#"<TimeToCollisionTarget><EntityRef entityRef="Lead"/></TimeToCollisionTarget>"#,
        &TimeToCollisionTarget::entity("Lead"),
    );
    assert_wire(
        &format!(r#"<TimeToCollisionTarget>{ORIGIN}</TimeToCollisionTarget>"#),
        &TimeToCollisionTarget::position(Position::world_origin()),
    );
}

#[test]
fn angle_speed_and_clearance_conditions_match_xsd_wire_form() {
    // AngleCondition XSD:734-739
    assert_wire(
        r#"<AngleCondition angleType="heading" angle="1.57" angleTolerance="0.1"/>"#,
        &AngleCondition::new(AngleType::Heading, 1.57, 0.1),
    );
    assert_wire(
        r#"<AngleCondition angleType="pitch" angle="0.3" angleTolerance="0.05" coordinateSystem="entity"/>"#,
        &AngleCondition::new(AngleType::Pitch, 0.3, 0.05)
            .with_coordinate_system(CoordinateSystem::Entity),
    );
    // RelativeAngleCondition XSD:1826-1831
    assert_wire(
        r#"<RelativeAngleCondition entityRef="Lead" angleType="heading" angle="0.5" angleTolerance="0.1"/>"#,
        &RelativeAngleCondition::new("Lead", AngleType::Heading, 0.5, 0.1),
    );
    assert_wire(
        r#"<RelativeAngleCondition entityRef="Lead" angleType="roll" angle="1" angleTolerance="0.2" coordinateSystem="lane"/>"#,
        &RelativeAngleCondition::new("Lead", AngleType::Roll, 1.0, 0.2)
            .with_coordinate_system(CoordinateSystem::Lane),
    );
    // RelativeSpeedCondition XSD:1883-1888
    assert_wire(
        r#"<RelativeSpeedCondition entityRef="Lead" rule="lessThan" value="5"/>"#,
        &RelativeSpeedCondition::new("Lead", Rule::LessThan, 5.0),
    );
    assert_wire(
        r#"<RelativeSpeedCondition entityRef="Lead" rule="greaterThan" value="2" direction="longitudinal"/>"#,
        &RelativeSpeedCondition::new("Lead", Rule::GreaterThan, 2.0)
            .with_direction(DirectionalDimension::Longitudinal),
    );
    // RelativeLaneRange XSD:1862-1865: both attributes optional
    assert_wire(
        r#"<RelativeLaneRange from="-1" to="2"/>"#,
        &RelativeLaneRange::new(Some(-1), Some(2)),
    );
    assert_wire(
        r#"<RelativeLaneRange/>"#,
        &RelativeLaneRange::new(None, None),
    );
    // RelativeClearanceCondition XSD:1833-1842: lane ranges before entity refs
    assert_wire(
        r#"<RelativeClearanceCondition oppositeLanes="false" distanceForward="50" distanceBackward="10" freeSpace="true"><RelativeLaneRange from="-1" to="1"/><EntityRef entityRef="Lead"/></RelativeClearanceCondition>"#,
        &RelativeClearanceCondition {
            relative_lane_ranges: vec![RelativeLaneRange::new(Some(-1), Some(1))],
            entity_refs: vec![EntityRef::new("Lead")],
            distance_forward: Some(Double::literal(50.0)),
            distance_backward: Some(Double::literal(10.0)),
            ..RelativeClearanceCondition::new(false, true)
        },
    );
    assert_wire(
        r#"<RelativeClearanceCondition oppositeLanes="true" freeSpace="false"/>"#,
        &RelativeClearanceCondition::new(true, false),
    );
}

#[test]
fn by_entity_condition_constructors_match_xsd_wire_form() {
    assert_wire(
        &by_entity_xml(r#"<SpeedCondition value="25" rule="greaterThan"/>"#),
        &ByEntityCondition::speed(ego(), 25.0, Rule::GreaterThan, "Ego"),
    );
    assert_wire(
        &by_entity_xml(
            r#"<ReachPositionCondition tolerance="3"><Position><WorldPosition x="100" y="200"/></Position></ReachPositionCondition>"#,
        ),
        // Deprecated since 1.2, still a valid document form: no constructor, but it must read and write.
        &ByEntityCondition::new(
            ego(),
            EntityCondition::ReachPosition(ReachPositionCondition::new(
                Position::world(WorldPosition::new(100.0, 200.0)),
                3.0,
            )),
        ),
    );
    assert_wire(
        &by_entity_xml(&format!(
            r#"<DistanceCondition value="40" freespace="true" rule="lessThan">{ORIGIN}</DistanceCondition>"#
        )),
        &ByEntityCondition::distance(ego(), Position::world_origin(), 40.0, true, Rule::LessThan),
    );
    assert_wire(
        &by_entity_xml(
            r#"<RelativeDistanceCondition entityRef="CutIn" value="15" freespace="false" relativeDistanceType="longitudinal" rule="greaterOrEqual"/>"#,
        ),
        &ByEntityCondition::relative_distance(
            ego(),
            "CutIn",
            15.0,
            false,
            RelativeDistanceType::Longitudinal,
            Rule::GreaterOrEqual,
        ),
    );
    assert_wire(
        &by_entity_xml(r#"<AccelerationCondition value="3" rule="greaterThan"/>"#),
        &ByEntityCondition::acceleration(ego(), 3.0, Rule::GreaterThan),
    );
    assert_wire(
        &by_entity_xml(
            r#"<AccelerationCondition value="2.5" rule="lessThan" direction="lateral"/>"#,
        ),
        &ByEntityCondition::acceleration_with_direction(
            ego(),
            2.5,
            Rule::LessThan,
            DirectionalDimension::Lateral,
        ),
    );
    assert_wire(
        &by_entity_xml(r#"<StandStillCondition duration="4"/>"#),
        &ByEntityCondition::standstill(ego(), 4.0),
    );
    assert_wire(
        &by_entity_xml(
            r#"<CollisionCondition><EntityRef entityRef="Target"/></CollisionCondition>"#,
        ),
        &ByEntityCondition::collision_with_target(ego(), "Target"),
    );
    assert_wire(
        &by_entity_xml(r#"<CollisionCondition><ByType type="pedestrian"/></CollisionCondition>"#),
        &ByEntityCondition::collision_with_type(ego(), ObjectType::Pedestrian),
    );
    assert_wire(
        &by_entity_xml(r#"<EndOfRoadCondition duration="3"/>"#),
        &ByEntityCondition::end_of_road(ego(), 3.0),
    );
    assert_wire(
        &by_entity_xml(
            r#"<TimeHeadwayCondition entityRef="Lead" value="2" rule="lessThan" freespace="true"/>"#,
        ),
        &ByEntityCondition::time_headway(ego(), "Lead", 2.0, Rule::LessThan, true),
    );
    assert_wire(
        &by_entity_xml(
            r#"<TimeToCollisionCondition value="3" rule="greaterThan" freespace="false"><TimeToCollisionConditionTarget><EntityRef entityRef="Obstacle"/></TimeToCollisionConditionTarget></TimeToCollisionCondition>"#,
        ),
        &ByEntityCondition::time_to_collision_entity(
            ego(),
            "Obstacle",
            3.0,
            Rule::GreaterThan,
            false,
        ),
    );
    assert_wire(
        &by_entity_xml(&format!(
            r#"<TimeToCollisionCondition value="4.5" rule="equalTo" freespace="true"><TimeToCollisionConditionTarget>{ORIGIN}</TimeToCollisionConditionTarget></TimeToCollisionCondition>"#
        )),
        &ByEntityCondition::time_to_collision_position(
            ego(),
            Position::world_origin(),
            4.5,
            Rule::EqualTo,
            true,
        ),
    );
    // ByEntityCondition::new — the general constructor the per-condition helpers above build on.
    assert_wire(
        &by_entity_xml(r#"<StandStillCondition duration="6"/>"#),
        &ByEntityCondition::new(
            ego(),
            EntityCondition::StandStill(StandStillCondition::new(6.0)),
        ),
    );
    // `offroad` builds the XSD-compliant OffroadCondition.
    assert_wire(
        &by_entity_xml(r#"<OffroadCondition duration="7"/>"#),
        &ByEntityCondition::offroad(ego(), 7.0),
    );
    assert_wire(
        &by_entity_xml(
            r#"<AngleCondition angleType="heading" angle="0.2" angleTolerance="0.05"/>"#,
        ),
        &ByEntityCondition::angle(ego(), AngleType::Heading, 0.2, 0.05),
    );
    assert_wire(
        &by_entity_xml(r#"<RelativeSpeedCondition entityRef="Lead" rule="lessThan" value="3"/>"#),
        &ByEntityCondition::relative_speed(ego(), "Lead", Rule::LessThan, 3.0),
    );
    assert_wire(
        &by_entity_xml(r#"<TraveledDistanceCondition value="50"/>"#),
        &ByEntityCondition::traveled_distance(ego(), 50.0),
    );
    assert_wire(
        &by_entity_xml(r#"<RelativeClearanceCondition oppositeLanes="true" freeSpace="false"/>"#),
        &ByEntityCondition::relative_clearance(ego(), true, false),
    );
    assert_wire(
        &by_entity_xml(
            r#"<RelativeAngleCondition entityRef="Lead" angleType="heading" angle="0.4" angleTolerance="0.1"/>"#,
        ),
        &ByEntityCondition::relative_angle(ego(), "Lead", AngleType::Heading, 0.4, 0.1),
    );
}

#[test]
fn entity_condition_rejects_two_conditions() {
    let xml = r#"<EntityCondition><SpeedCondition value="25" rule="greaterThan"/><StandStillCondition duration="1"/></EntityCondition>"#;
    let error = quick_xml::de::from_str::<EntityCondition>(xml)
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("EntityCondition can only contain one condition type"),
        "unexpected error: {error}"
    );
}

#[test]
fn entity_condition_rejects_empty_element() {
    // XSD `EntityCondition` (:1135-1156) is an xsd:choice with no "none" branch.
    let xml = r#"<EntityCondition/>"#;
    let error = quick_xml::de::from_str::<EntityCondition>(xml)
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("EntityCondition must contain exactly one condition type"),
        "unexpected error: {error}"
    );
}

#[test]
fn entity_condition_rejects_unknown_condition() {
    let xml = r#"<EntityCondition><UnknownCondition value="25"/></EntityCondition>"#;
    let error = quick_xml::de::from_str::<EntityCondition>(xml)
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("Unknown EntityCondition type: UnknownCondition"),
        "unexpected error: {error}"
    );
}
