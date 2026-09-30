//! Wire form of the optional `<Orientation>` child on `RelativeRoadPosition` and
//! `RelativeLanePosition`, built through their `with_orientation` constructors.
//!
//! Each struct declares its own `Orientation` field rename, so each needs its own proof that
//! the child is emitted under that name and read back. The attribute assertions also pin the
//! constructor's argument order: a swapped `ds`/`dt` still round-trips unchanged.

use openscenario_rs::types::positions::{Orientation, RelativeLanePosition, RelativeRoadPosition};

#[test]
fn test_relative_road_position_with_orientation() {
    let orientation = Orientation::heading(1.57);
    let pos =
        RelativeRoadPosition::with_orientation("EgoVehicle".to_string(), 5.0, 1.5, orientation);

    let xml = quick_xml::se::to_string(&pos).expect("Failed to serialize");

    assert!(
        xml.contains(r#"entityRef="EgoVehicle""#),
        "serialized: {xml}"
    );
    assert!(xml.contains(r#"ds="5""#), "serialized: {xml}");
    assert!(xml.contains(r#"dt="1.5""#), "serialized: {xml}");
    assert!(xml.contains("<Orientation"), "serialized: {xml}");
    assert!(xml.contains("h=\"1.57\""), "serialized: {xml}");

    let deserialized: RelativeRoadPosition =
        quick_xml::de::from_str(&xml).expect("Failed to deserialize");

    assert_eq!(pos, deserialized);
    assert_eq!(
        deserialized
            .orientation
            .unwrap()
            .h
            .unwrap()
            .as_literal()
            .unwrap(),
        &1.57
    );
}

#[test]
fn test_relative_lane_position_with_orientation() {
    let orientation = Orientation::new(1.57, 0.1, -0.1);
    let pos = RelativeLanePosition::with_orientation(
        "EgoVehicle".to_string(),
        1,
        20.0,
        -1.0,
        orientation,
    );

    let xml = quick_xml::se::to_string(&pos).expect("Failed to serialize");

    assert!(
        xml.contains(r#"entityRef="EgoVehicle""#),
        "serialized: {xml}"
    );
    assert!(xml.contains(r#"dLane="1""#), "serialized: {xml}");
    assert!(xml.contains(r#"ds="20""#), "serialized: {xml}");
    assert!(xml.contains(r#"offset="-1""#), "serialized: {xml}");
    assert!(xml.contains("<Orientation"), "serialized: {xml}");
    assert!(xml.contains("h=\"1.57\""), "serialized: {xml}");
    assert!(xml.contains("p=\"0.1\""), "serialized: {xml}");
    assert!(xml.contains("r=\"-0.1\""), "serialized: {xml}");

    let deserialized: RelativeLanePosition =
        quick_xml::de::from_str(&xml).expect("Failed to deserialize");

    assert_eq!(pos, deserialized);
    let orient = deserialized.orientation.unwrap();
    assert_eq!(orient.h.unwrap().as_literal().unwrap(), &1.57);
    assert_eq!(orient.p.unwrap().as_literal().unwrap(), &0.1);
    assert_eq!(orient.r.unwrap().as_literal().unwrap(), &-0.1);
}
