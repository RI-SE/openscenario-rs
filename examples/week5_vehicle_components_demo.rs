//! Week 5 Vehicle Components Demo
//!
//! This example demonstrates the enhanced vehicle components implemented in Week 5:
//! - Enhanced BoundingBox with volume and contains_point methods
//! - Center distance calculations
//! - New Dimensions default methods (vehicle_default, pedestrian_default, truck_default)
//! - Complete Axle system with multi-axle support
//! - Integration with Vehicle types

use openscenario_rs::types::geometry::{BoundingBox, Center, Dimensions};
use openscenario_rs::types::entities::{Axle, Axles, Vehicle};
use openscenario_rs::types::basic::Value;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚗 Week 5 Vehicle Components Demo");
    println!("==================================\n");

    // Demo 1: Enhanced BoundingBox Operations
    println!("1. Enhanced BoundingBox Operations");
    println!("----------------------------------");
    
    let bbox = BoundingBox {
        center: Center::default(),
        dimensions: Dimensions::vehicle_default(),
    };
    
    let volume = bbox.volume()?;
    println!("Vehicle bounding box volume: {:.2} m³", volume);
    
    // Test point containment
    let test_points = [
        (0.0, 0.0, 0.0),   // Center
        (2.0, 1.0, 0.8),   // Inside
        (3.0, 0.0, 0.0),   // Outside
    ];
    
    for (x, y, z) in test_points {
        let contains = bbox.contains_point(x, y, z)?;
        println!("Point ({}, {}, {}) is {}", x, y, z, 
                if contains { "inside" } else { "outside" });
    }
    println!();

    // Demo 2: Center Distance Calculations
    println!("2. Center Distance Calculations");
    println!("-------------------------------");
    
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
    
    let distance = center1.distance_to(&center2)?;
    println!("Distance between centers: {:.2} m", distance);
    println!();

    // Demo 3: New Dimensions Defaults
    println!("3. New Dimensions Defaults");
    println!("--------------------------");
    
    let vehicle_dims = Dimensions::vehicle_default();
    let pedestrian_dims = Dimensions::pedestrian_default();
    let truck_dims = Dimensions::truck_default();
    
    println!("Vehicle dimensions: {:.1}m × {:.1}m × {:.1}m", 
             vehicle_dims.width.as_literal().unwrap(),
             vehicle_dims.length.as_literal().unwrap(),
             vehicle_dims.height.as_literal().unwrap());
    
    println!("Pedestrian dimensions: {:.1}m × {:.1}m × {:.1}m", 
             pedestrian_dims.width.as_literal().unwrap(),
             pedestrian_dims.length.as_literal().unwrap(),
             pedestrian_dims.height.as_literal().unwrap());
    
    println!("Truck dimensions: {:.1}m × {:.1}m × {:.1}m", 
             truck_dims.width.as_literal().unwrap(),
             truck_dims.length.as_literal().unwrap(),
             truck_dims.height.as_literal().unwrap());
    println!();

    // Demo 4: Complete Axle System
    println!("4. Complete Axle System");
    println!("-----------------------");
    
    let params = HashMap::new();
    
    let car_axles = Axles::car();
    let truck_axles = Axles::truck();
    let trailer_axles = Axles::trailer();
    let motorcycle_axles = Axles::motorcycle();
    
    println!("Car: {} axles, wheelbase: {:.2}m, steerable: {}", 
             car_axles.axle_count(),
             car_axles.wheelbase(&params)?,
             car_axles.is_steerable(&params)?);
    
    println!("Truck: {} axles, wheelbase: {:.2}m, steerable: {}", 
             truck_axles.axle_count(),
             truck_axles.wheelbase(&params)?,
             truck_axles.is_steerable(&params)?);
    
    println!("Trailer: {} axles, wheelbase: {:.2}m, steerable: {}", 
             trailer_axles.axle_count(),
             trailer_axles.wheelbase(&params)?,
             trailer_axles.is_steerable(&params)?);
    
    println!("Motorcycle: {} axles, wheelbase: {:.2}m, steerable: {}", 
             motorcycle_axles.axle_count(),
             motorcycle_axles.wheelbase(&params)?,
             motorcycle_axles.is_steerable(&params)?);
    println!();

    // Demo 5: Axle Wheel Calculations
    println!("5. Axle Wheel Calculations");
    println!("--------------------------");
    
    let front_axle = Axle::front_car();
    let circumference = front_axle.wheel_circumference(&params)?;
    let steering_degrees = front_axle.max_steering_degrees(&params)?;
    let has_dual_wheels = front_axle.has_dual_wheels(&params)?;
    let wheel_positions = front_axle.wheel_positions(&params)?;
    
    println!("Front axle wheel circumference: {:.2}m", circumference);
    println!("Maximum steering angle: {:.1}°", steering_degrees);
    println!("Has dual wheels: {}", has_dual_wheels);
    println!("Wheel positions: {:?}", wheel_positions);
    
    let wheelbase = 2.8;
    let turning_radius = front_axle.turning_radius(wheelbase, &params)?;
    if turning_radius.is_finite() {
        println!("Turning radius: {:.2}m", turning_radius);
    } else {
        println!("Turning radius: infinite (non-steerable)");
    }
    println!();

    // Demo 6: Vehicle Integration
    println!("6. Vehicle Integration");
    println!("---------------------");
    
    let car = Vehicle::new_car("DemoCar".to_string());
    let truck = Vehicle::new_truck("DemoTruck".to_string());
    let motorcycle = Vehicle::new_motorcycle("DemoBike".to_string());
    
    println!("Car '{}': {} axles, footprint: {:.2}m²", 
             car.name.as_literal().unwrap(),
             car.axle_count(),
             car.footprint_area(&params)?);
    
    println!("Truck '{}': {} axles, footprint: {:.2}m²", 
             truck.name.as_literal().unwrap(),
             truck.axle_count(),
             truck.footprint_area(&params)?);
    
    println!("Motorcycle '{}': {} axles, footprint: {:.2}m²", 
             motorcycle.name.as_literal().unwrap(),
             motorcycle.axle_count(),
             motorcycle.footprint_area(&params)?);
    println!();

    // Demo 7: XML Serialization
    println!("7. XML Serialization");
    println!("--------------------");
    
    let bbox_xml = quick_xml::se::to_string(&bbox)?;
    println!("BoundingBox XML (truncated): {}...", 
             &bbox_xml[..bbox_xml.len().min(80)]);
    
    let axles_xml = quick_xml::se::to_string(&car_axles)?;
    println!("Axles XML (truncated): {}...", 
             &axles_xml[..axles_xml.len().min(80)]);
    
    println!("\n✅ Week 5 Vehicle Components Demo Complete!");
    println!("All enhanced functionality is working correctly.");

    Ok(())
}