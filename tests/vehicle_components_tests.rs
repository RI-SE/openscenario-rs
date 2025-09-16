//! Integration tests for Week 4 Vehicle Components Implementation
//!
//! Tests the enhanced BoundingBox geometric operations and comprehensive
//! Axle system implementation for various vehicle types.

use openscenario_rs::types::{
    entities::{axles::{Axle, Axles}, Vehicle},
    geometry::{BoundingBox, Center, Dimensions},
};
use std::collections::HashMap;

#[test]
fn test_enhanced_bounding_box_operations() {
    let params = HashMap::new();
    
    let bbox = BoundingBox {
        center: Center::default(),
        dimensions: Dimensions {
            width: openscenario_rs::types::basic::Value::literal(2.0),
            length: openscenario_rs::types::basic::Value::literal(4.0),
            height: openscenario_rs::types::basic::Value::literal(1.5),
        },
    };

    // Test volume calculation
    let volume = bbox.volume(&params).unwrap();
    assert_eq!(volume, 12.0);

    // Test point containment
    assert!(bbox.contains_point(0.0, 0.0, 0.0, &params).unwrap());
    assert!(!bbox.contains_point(3.0, 0.0, 0.0, &params).unwrap());

    // Test distance calculation
    let distance = bbox.distance_to_point(3.0, 0.0, 0.0, &params).unwrap();
    assert!((distance - 1.0).abs() < 0.001);
}

#[test]
fn test_bounding_box_intersection() {
    let params = HashMap::new();
    
    let bbox1 = BoundingBox {
        center: Center::default(),
        dimensions: Dimensions::car(),
    };
    
    let bbox2_overlapping = BoundingBox {
        center: Center {
            x: openscenario_rs::types::basic::Value::literal(1.0),
            y: openscenario_rs::types::basic::Value::literal(0.0),
            z: openscenario_rs::types::basic::Value::literal(0.0),
        },
        dimensions: Dimensions::car(),
    };
    
    let bbox3_separate = BoundingBox {
        center: Center {
            x: openscenario_rs::types::basic::Value::literal(10.0),
            y: openscenario_rs::types::basic::Value::literal(0.0),
            z: openscenario_rs::types::basic::Value::literal(0.0),
        },
        dimensions: Dimensions::car(),
    };

    assert!(bbox1.intersects(&bbox2_overlapping, &params).unwrap());
    assert!(!bbox1.intersects(&bbox3_separate, &params).unwrap());
}

#[test]
fn test_dimensions_presets() {
    let params = HashMap::new();
    
    // Test all preset dimensions
    let car = Dimensions::car();
    let truck = Dimensions::truck();
    let bus = Dimensions::bus();
    let motorcycle = Dimensions::motorcycle();
    let pedestrian = Dimensions::pedestrian();

    // Verify car dimensions
    assert_eq!(car.width.as_literal().unwrap(), &1.8);
    assert_eq!(car.length.as_literal().unwrap(), &4.5);
    assert_eq!(car.height.as_literal().unwrap(), &1.5);

    // Verify truck is larger than car
    assert!(truck.width.resolve(&params).unwrap() > car.width.resolve(&params).unwrap());
    assert!(truck.length.resolve(&params).unwrap() > car.length.resolve(&params).unwrap());
    assert!(truck.height.resolve(&params).unwrap() > car.height.resolve(&params).unwrap());

    // Test footprint area calculations
    let car_area = car.footprint_area(&params).unwrap();
    let truck_area = truck.footprint_area(&params).unwrap();
    assert!(truck_area > car_area);

    // Test scaling
    let scaled_car = car.scale(2.0, &params).unwrap();
    assert_eq!(scaled_car.width.as_literal().unwrap(), &3.6);
}

#[test]
fn test_axle_system_configurations() {
    // Test different axle configurations
    let car_axles = Axles::car();
    let truck_axles = Axles::truck();
    let trailer_axles = Axles::trailer();
    let motorcycle_axles = Axles::motorcycle();

    // Verify axle counts
    assert_eq!(car_axles.axle_count(), 2);
    assert_eq!(truck_axles.axle_count(), 3);
    assert_eq!(trailer_axles.axle_count(), 1);
    assert_eq!(motorcycle_axles.axle_count(), 2);

    // Verify front axle presence
    assert!(car_axles.front_axle.is_some());
    assert!(truck_axles.front_axle.is_some());
    assert!(trailer_axles.front_axle.is_none());
    assert!(motorcycle_axles.front_axle.is_some());

    // Verify additional axles
    assert_eq!(car_axles.additional_axles.len(), 0);
    assert_eq!(truck_axles.additional_axles.len(), 1);
    assert_eq!(trailer_axles.additional_axles.len(), 0);
    assert_eq!(motorcycle_axles.additional_axles.len(), 0);
}

#[test]
fn test_axle_calculations() {
    let params = HashMap::new();
    
    let car_axles = Axles::car();
    let wheelbase = car_axles.wheelbase(&params).unwrap();
    assert!(wheelbase > 0.0);
    
    assert!(car_axles.is_steerable(&params).unwrap());
    
    let trailer_axles = Axles::trailer();
    assert!(!trailer_axles.is_steerable(&params).unwrap());
}

#[test]
fn test_individual_axle_properties() {
    let params = HashMap::new();
    
    let front_axle = Axle::front_car();
    let rear_axle = Axle::rear_car();
    let motorcycle_axle = Axle::front_motorcycle();

    // Test wheel circumference
    let circumference = front_axle.wheel_circumference(&params).unwrap();
    assert!(circumference > 0.0);

    // Test steering angles
    let front_steering = front_axle.max_steering_degrees(&params).unwrap();
    let rear_steering = rear_axle.max_steering_degrees(&params).unwrap();
    assert!(front_steering > 0.0);
    assert_eq!(rear_steering, 0.0);

    // Test turning radius
    let turning_radius = front_axle.turning_radius(2.8, &params).unwrap();
    assert!(turning_radius > 0.0 && turning_radius < f64::INFINITY);
    
    let rear_turning_radius = rear_axle.turning_radius(2.8, &params).unwrap();
    assert_eq!(rear_turning_radius, f64::INFINITY);

    // Test dual wheels
    assert!(front_axle.has_dual_wheels(&params).unwrap());
    assert!(!motorcycle_axle.has_dual_wheels(&params).unwrap());

    // Test wheel positions
    let car_positions = front_axle.wheel_positions(&params).unwrap();
    let motorcycle_positions = motorcycle_axle.wheel_positions(&params).unwrap();
    assert_eq!(car_positions.len(), 2);
    assert_eq!(motorcycle_positions.len(), 1);
}

#[test]
fn test_vehicle_factory_methods() {
    let params = HashMap::new();
    
    let car = Vehicle::new_car("TestCar".to_string());
    let truck = Vehicle::new_truck("TestTruck".to_string());
    let motorcycle = Vehicle::new_motorcycle("TestBike".to_string());

    // Test names
    assert_eq!(car.name.as_literal().unwrap(), "TestCar");
    assert_eq!(truck.name.as_literal().unwrap(), "TestTruck");
    assert_eq!(motorcycle.name.as_literal().unwrap(), "TestBike");

    // Test categories
    assert_eq!(car.vehicle_category, openscenario_rs::types::enums::VehicleCategory::Car);
    assert_eq!(truck.vehicle_category, openscenario_rs::types::enums::VehicleCategory::Truck);
    assert_eq!(motorcycle.vehicle_category, openscenario_rs::types::enums::VehicleCategory::Motorbike);

    // Test axle counts
    assert_eq!(car.axle_count(), 2);
    assert_eq!(truck.axle_count(), 3);
    assert_eq!(motorcycle.axle_count(), 2);

    // Test vehicle methods
    assert!(car.wheelbase(&params).unwrap() > 0.0);
    assert!(car.is_steerable(&params).unwrap());
    assert!(car.footprint_area(&params).unwrap() > 0.0);

    // Truck should have larger footprint than car
    assert!(truck.footprint_area(&params).unwrap() > car.footprint_area(&params).unwrap());
}

#[test]
fn test_axle_serialization() {
    let axle = Axle::front_car();
    let xml = quick_xml::se::to_string(&axle).unwrap();
    
    assert!(xml.contains("maxSteering"));
    assert!(xml.contains("wheelDiameter"));
    assert!(xml.contains("trackWidth"));
    assert!(xml.contains("positionX"));
    assert!(xml.contains("positionZ"));
}

#[test]
fn test_axles_serialization() {
    let axles = Axles::car();
    let xml = quick_xml::se::to_string(&axles).unwrap();
    
    assert!(xml.contains("FrontAxle"));
    assert!(xml.contains("RearAxle"));
    assert!(!xml.contains("AdditionalAxle")); // Should be empty for car
}

#[test]
fn test_vehicle_with_enhanced_components() {
    let params = HashMap::new();
    
    let vehicle = Vehicle::new_truck("IntegrationTestTruck".to_string());
    
    // Test that all components work together
    let wheelbase = vehicle.wheelbase(&params).unwrap();
    let footprint = vehicle.footprint_area(&params).unwrap();
    let volume = vehicle.bounding_box.volume(&params).unwrap();
    
    assert!(wheelbase > 0.0);
    assert!(footprint > 0.0);
    assert!(volume > 0.0);
    
    // Test that truck has appropriate characteristics
    assert_eq!(vehicle.axle_count(), 3);
    assert!(vehicle.is_steerable(&params).unwrap());
    
    // Test bounding box operations
    assert!(vehicle.bounding_box.contains_point(0.0, 0.0, 0.0, &params).unwrap());
    
    // Test serialization of complete vehicle
    let xml = quick_xml::se::to_string(&vehicle).unwrap();
    assert!(xml.contains("name=\"IntegrationTestTruck\""));
    assert!(xml.contains("vehicleCategory=\"truck\""));
    assert!(xml.contains("BoundingBox"));
    assert!(xml.contains("Axles"));
    assert!(xml.contains("Performance"));
}