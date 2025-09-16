//! Week 5 Vehicle Components Integration Tests
//!
//! This test file verifies the complete implementation of Week 5 vehicle components
//! including enhanced BoundingBox, Center distance calculations, new Dimensions defaults,
//! and comprehensive Axle system integration.

use openscenario_rs::types::geometry::{BoundingBox, Center, Dimensions};
use openscenario_rs::types::entities::{Axle, Axles, Vehicle};
use openscenario_rs::types::basic::Value;
use std::collections::HashMap;

#[test]
fn test_enhanced_bounding_box_volume() {
    // Test volume calculation without parameters
    let bbox = BoundingBox {
        center: Center::default(),
        dimensions: Dimensions::new(2.0, 4.0, 1.5),
    };

    let volume = bbox.volume().unwrap();
    assert_eq!(volume, 12.0); // 2.0 * 4.0 * 1.5

    // Test volume calculation with parameters
    let bbox_with_params = BoundingBox {
        center: Center::default(),
        dimensions: Dimensions {
            width: Value::parameter("width".to_string()),
            length: Value::parameter("length".to_string()),
            height: Value::literal(2.0),
        },
    };

    let mut params = HashMap::new();
    params.insert("width".to_string(), "3.0".to_string());
    params.insert("length".to_string(), "5.0".to_string());

    let volume_with_params = bbox_with_params.volume_with_params(&params).unwrap();
    assert_eq!(volume_with_params, 30.0); // 3.0 * 5.0 * 2.0
}

#[test]
fn test_enhanced_bounding_box_contains_point() {
    let bbox = BoundingBox {
        center: Center::default(),
        dimensions: Dimensions::new(4.0, 6.0, 2.0),
    };

    // Points inside the bounding box
    assert!(bbox.contains_point(0.0, 0.0, 0.0).unwrap());
    assert!(bbox.contains_point(2.9, 1.9, 0.9).unwrap());
    assert!(bbox.contains_point(-2.9, -1.9, -0.9).unwrap());

    // Points outside the bounding box
    assert!(!bbox.contains_point(3.1, 0.0, 0.0).unwrap());
    assert!(!bbox.contains_point(0.0, 2.1, 0.0).unwrap());
    assert!(!bbox.contains_point(0.0, 0.0, 1.1).unwrap());

    // Edge cases (exactly on boundary)
    assert!(bbox.contains_point(3.0, 0.0, 0.0).unwrap());
    assert!(bbox.contains_point(0.0, 2.0, 0.0).unwrap());
    assert!(bbox.contains_point(0.0, 0.0, 1.0).unwrap());
}

#[test]
fn test_center_distance_calculations() {
    // Test 2D distance (z = 0)
    let center1 = Center {
        x: Value::literal(0.0),
        y: Value::literal(0.0),
        z: Value::literal(0.0),
    };

    let center2 = Center {
        x: Value::literal(3.0),
        y: Value::literal(4.0),
        z: Value::literal(0.0),
    };

    let distance = center1.distance_to(&center2).unwrap();
    assert_eq!(distance, 5.0); // 3-4-5 triangle

    // Test 3D distance
    let center3 = Center {
        x: Value::literal(1.0),
        y: Value::literal(2.0),
        z: Value::literal(2.0),
    };

    let center4 = Center {
        x: Value::literal(4.0),
        y: Value::literal(6.0),
        z: Value::literal(2.0),
    };

    let distance_3d = center3.distance_to(&center4).unwrap();
    assert_eq!(distance_3d, 5.0); // sqrt((4-1)² + (6-2)² + (2-2)²) = sqrt(9 + 16 + 0) = 5

    // Test distance with parameters
    let center_param1 = Center {
        x: Value::parameter("x1".to_string()),
        y: Value::parameter("y1".to_string()),
        z: Value::literal(0.0),
    };

    let center_param2 = Center {
        x: Value::literal(6.0),
        y: Value::literal(8.0),
        z: Value::literal(0.0),
    };

    let mut params = HashMap::new();
    params.insert("x1".to_string(), "0.0".to_string());
    params.insert("y1".to_string(), "0.0".to_string());

    let distance_with_params = center_param1.distance_to_with_params(&center_param2, &params).unwrap();
    assert_eq!(distance_with_params, 10.0); // sqrt(6² + 8²) = 10
}

#[test]
fn test_dimensions_new_default_methods() {
    // Test vehicle_default
    let vehicle_dims = Dimensions::vehicle_default();
    assert_eq!(vehicle_dims.width.as_literal().unwrap(), &2.0);
    assert_eq!(vehicle_dims.length.as_literal().unwrap(), &4.5);
    assert_eq!(vehicle_dims.height.as_literal().unwrap(), &1.8);

    // Test pedestrian_default
    let pedestrian_dims = Dimensions::pedestrian_default();
    assert_eq!(pedestrian_dims.width.as_literal().unwrap(), &0.6);
    assert_eq!(pedestrian_dims.length.as_literal().unwrap(), &0.6);
    assert_eq!(pedestrian_dims.height.as_literal().unwrap(), &1.8);

    // Test truck_default
    let truck_dims = Dimensions::truck_default();
    assert_eq!(truck_dims.width.as_literal().unwrap(), &2.5);
    assert_eq!(truck_dims.length.as_literal().unwrap(), &12.0);
    assert_eq!(truck_dims.height.as_literal().unwrap(), &3.5);

    // Test new() constructor
    let custom_dims = Dimensions::new(1.5, 3.0, 2.2);
    assert_eq!(custom_dims.width.as_literal().unwrap(), &1.5);
    assert_eq!(custom_dims.length.as_literal().unwrap(), &3.0);
    assert_eq!(custom_dims.height.as_literal().unwrap(), &2.2);
}

#[test]
fn test_axle_system_comprehensive() {
    let params = HashMap::new();

    // Test car axle configuration
    let car_axles = Axles::car();
    assert_eq!(car_axles.axle_count(), 2);
    assert!(car_axles.is_steerable(&params).unwrap());
    assert!(car_axles.wheelbase(&params).unwrap() > 0.0);

    // Test truck axle configuration
    let truck_axles = Axles::truck();
    assert_eq!(truck_axles.axle_count(), 3);
    assert!(truck_axles.is_steerable(&params).unwrap());
    assert!(truck_axles.wheelbase(&params).unwrap() > car_axles.wheelbase(&params).unwrap());

    // Test trailer configuration (no front axle)
    let trailer_axles = Axles::trailer();
    assert_eq!(trailer_axles.axle_count(), 1);
    assert!(!trailer_axles.is_steerable(&params).unwrap());
    assert_eq!(trailer_axles.wheelbase(&params).unwrap(), 0.0);

    // Test motorcycle configuration
    let motorcycle_axles = Axles::motorcycle();
    assert_eq!(motorcycle_axles.axle_count(), 2);
    assert!(motorcycle_axles.is_steerable(&params).unwrap());
}

#[test]
fn test_axle_wheel_calculations() {
    let params = HashMap::new();

    // Test car front axle
    let car_front = Axle::front_car();
    let circumference = car_front.wheel_circumference(&params).unwrap();
    let expected_circumference = std::f64::consts::PI * 0.65;
    assert!((circumference - expected_circumference).abs() < 0.001);

    let steering_degrees = car_front.max_steering_degrees(&params).unwrap();
    assert!((steering_degrees - 30.0).abs() < 0.1); // Should be ~30 degrees

    assert!(car_front.has_dual_wheels(&params).unwrap());

    let wheel_positions = car_front.wheel_positions(&params).unwrap();
    assert_eq!(wheel_positions.len(), 2);
    assert_eq!(wheel_positions[0], (1.4, -0.8)); // Left wheel
    assert_eq!(wheel_positions[1], (1.4, 0.8));  // Right wheel

    // Test motorcycle front axle (single wheel)
    let motorcycle_front = Axle::front_motorcycle();
    assert!(!motorcycle_front.has_dual_wheels(&params).unwrap());

    let motorcycle_positions = motorcycle_front.wheel_positions(&params).unwrap();
    assert_eq!(motorcycle_positions.len(), 1);
    assert_eq!(motorcycle_positions[0], (0.8, 0.0));

    // Test turning radius calculation
    let wheelbase = 2.8;
    let turning_radius = car_front.turning_radius(wheelbase, &params).unwrap();
    assert!(turning_radius > 0.0 && turning_radius < f64::INFINITY);

    // Test non-steerable axle
    let car_rear = Axle::rear_car();
    let rear_turning_radius = car_rear.turning_radius(wheelbase, &params).unwrap();
    assert_eq!(rear_turning_radius, f64::INFINITY);
}

#[test]
fn test_vehicle_integration_with_enhanced_components() {
    let params = HashMap::new();

    // Test car creation and integration
    let car = Vehicle::new_car("TestCar".to_string());
    assert_eq!(car.name.as_literal().unwrap(), "TestCar");
    assert_eq!(car.axle_count(), 2);
    assert!(car.is_steerable(&params).unwrap());
    assert!(car.wheelbase(&params).unwrap() > 0.0);
    assert!(car.footprint_area(&params).unwrap() > 0.0);

    // Test truck creation with enhanced dimensions
    let truck = Vehicle::new_truck("TestTruck".to_string());
    assert_eq!(truck.name.as_literal().unwrap(), "TestTruck");
    assert_eq!(truck.axle_count(), 3);
    assert!(truck.is_steerable(&params).unwrap());
    assert!(truck.wheelbase(&params).unwrap() > car.wheelbase(&params).unwrap());
    assert!(truck.footprint_area(&params).unwrap() > car.footprint_area(&params).unwrap());

    // Test motorcycle creation
    let motorcycle = Vehicle::new_motorcycle("TestBike".to_string());
    assert_eq!(motorcycle.name.as_literal().unwrap(), "TestBike");
    assert_eq!(motorcycle.axle_count(), 2);
    assert!(motorcycle.is_steerable(&params).unwrap());
    assert!(motorcycle.footprint_area(&params).unwrap() < car.footprint_area(&params).unwrap());
}

#[test]
fn test_bounding_box_geometric_operations() {
    let bbox1 = BoundingBox {
        center: Center::default(),
        dimensions: Dimensions::new(2.0, 2.0, 2.0),
    };

    let bbox2 = BoundingBox {
        center: Center {
            x: Value::literal(1.0),
            y: Value::literal(0.0),
            z: Value::literal(0.0),
        },
        dimensions: Dimensions::new(2.0, 2.0, 2.0),
    };

    let bbox3 = BoundingBox {
        center: Center {
            x: Value::literal(3.0),
            y: Value::literal(0.0),
            z: Value::literal(0.0),
        },
        dimensions: Dimensions::new(2.0, 2.0, 2.0),
    };

    let params = HashMap::new();

    // Test intersection
    assert!(bbox1.intersects(&bbox2, &params).unwrap());
    assert!(!bbox1.intersects(&bbox3, &params).unwrap());

    // Test distance to point
    let distance_inside = bbox1.distance_to_point(0.0, 0.0, 0.0, &params).unwrap();
    assert!(distance_inside < 0.0); // Inside should be negative

    let distance_outside = bbox1.distance_to_point(2.0, 0.0, 0.0, &params).unwrap();
    assert!(distance_outside > 0.0); // Outside should be positive
    assert!((distance_outside - 1.0).abs() < 0.001); // Should be 1.0 unit away
}

#[test]
fn test_dimensions_calculations() {
    let params = HashMap::new();

    // Test footprint area calculation
    let dims = Dimensions::vehicle_default();
    let area = dims.footprint_area(&params).unwrap();
    assert_eq!(area, 9.0); // 2.0 * 4.5

    // Test scaling
    let scaled = dims.scale(2.0, &params).unwrap();
    assert_eq!(scaled.width.as_literal().unwrap(), &4.0);
    assert_eq!(scaled.length.as_literal().unwrap(), &9.0);
    assert_eq!(scaled.height.as_literal().unwrap(), &3.6);

    let scaled_area = scaled.footprint_area(&params).unwrap();
    assert_eq!(scaled_area, 36.0); // 4.0 * 9.0 (4x the original area)
}

#[test]
fn test_xml_serialization_compatibility() {
    // Test that all enhanced components can be serialized to XML
    let bbox = BoundingBox {
        center: Center::default(),
        dimensions: Dimensions::vehicle_default(),
    };

    let xml = quick_xml::se::to_string(&bbox).unwrap();
    assert!(xml.contains("Center"));
    assert!(xml.contains("Dimensions"));
    assert!(xml.contains("width=\"2\""));
    assert!(xml.contains("length=\"4.5\""));
    assert!(xml.contains("height=\"1.8\""));

    // Test axle serialization
    let axles = Axles::car();
    let axles_xml = quick_xml::se::to_string(&axles).unwrap();
    assert!(axles_xml.contains("FrontAxle"));
    assert!(axles_xml.contains("RearAxle"));
    assert!(axles_xml.contains("maxSteering"));
    assert!(axles_xml.contains("wheelDiameter"));

    // Test vehicle serialization
    let vehicle = Vehicle::new_car("TestCar".to_string());
    let vehicle_xml = quick_xml::se::to_string(&vehicle).unwrap();
    assert!(vehicle_xml.contains("name=\"TestCar\""));
    assert!(vehicle_xml.contains("vehicleCategory=\"car\""));
    assert!(vehicle_xml.contains("BoundingBox"));
    assert!(vehicle_xml.contains("Axles"));
}