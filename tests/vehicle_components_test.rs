//! Geometry behaviour of the vehicle bounding box that the unit tests in
//! `src/types/geometry/shapes.rs` do not reach: containment on the box faces, the bus
//! and motorcycle presets, and the parameter-resolving variants of containment and
//! centre distance.

use openscenario_rs::types::basic::Value;
use openscenario_rs::types::geometry::{BoundingBox, Center, Dimensions};
use std::collections::HashMap;

#[test]
fn bounding_box_contains_point_includes_faces_and_resolves_parameters() {
    // length 6 (x), width 4 (y), height 2 (z), centred on the origin.
    let bbox = BoundingBox {
        center: Center::new(0.0, 0.0, 0.0),
        dimensions: Dimensions::new(4.0, 6.0, 2.0),
    };

    // (point, inside): the faces themselves count as inside.
    let cases = [
        ((0.0, 0.0, 0.0), true),
        ((2.9, 1.9, 0.9), true),
        ((-2.9, -1.9, -0.9), true),
        ((3.0, 0.0, 0.0), true),
        ((0.0, 2.0, 0.0), true),
        ((0.0, 0.0, 1.0), true),
        ((3.1, 0.0, 0.0), false),
        ((0.0, 2.1, 0.0), false),
        ((0.0, 0.0, 1.1), false),
    ];
    for ((x, y, z), inside) in cases {
        assert_eq!(
            bbox.contains_point(x, y, z).unwrap(),
            inside,
            "({x}, {y}, {z})"
        );
    }

    // The same box, shifted by a `$param` centre, answers through the parameter.
    let shifted = BoundingBox {
        center: Center {
            x: Value::parameter("cx".to_string()),
            y: Value::literal(0.0),
            z: Value::literal(0.0),
        },
        dimensions: Dimensions::new(4.0, 6.0, 2.0),
    };
    let params = HashMap::from([("cx".to_string(), "10.0".to_string())]);
    assert!(shifted
        .contains_point_with_params(13.0, 0.0, 0.0, &params)
        .unwrap());
    assert!(!shifted
        .contains_point_with_params(0.0, 0.0, 0.0, &params)
        .unwrap());
}

/// The named presets that `shapes.rs`'s own preset test leaves out.
#[test]
fn bus_and_motorcycle_dimension_presets() {
    // (preset, width, length, height)
    for (name, dims, w, l, h) in [
        ("bus", Dimensions::bus(), 2.5, 12.0, 3.2),
        ("motorcycle", Dimensions::motorcycle(), 0.8, 2.2, 1.3),
    ] {
        assert_eq!(dims.width.as_literal(), Some(&w), "{name}: width");
        assert_eq!(dims.length.as_literal(), Some(&l), "{name}: length");
        assert_eq!(dims.height.as_literal(), Some(&h), "{name}: height");
    }
}

#[test]
fn center_distance_resolves_parameter_coordinates() {
    let from = Center {
        x: Value::parameter("x1".to_string()),
        y: Value::parameter("y1".to_string()),
        z: Value::literal(0.0),
    };
    let to = Center::new(6.0, 8.0, 0.0);

    let params = HashMap::from([
        ("x1".to_string(), "0.0".to_string()),
        ("y1".to_string(), "0.0".to_string()),
    ]);
    assert_eq!(from.distance_to_with_params(&to, &params).unwrap(), 10.0);
}
