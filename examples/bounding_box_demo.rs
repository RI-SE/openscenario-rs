//! Demonstration of enhanced BoundingBox functionality
//!
//! This example shows the new geometric operations for BoundingBox
//! and dimension presets for different vehicle types.

use openscenario_rs::types::geometry::{BoundingBox, Center, Dimensions};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== OpenSCENARIO-rs Enhanced BoundingBox Demo ===\n");

    // Create parameter context for resolution
    let params = HashMap::new();

    // === Enhanced BoundingBox Demo ===
    println!("1. Enhanced BoundingBox Operations:");
    
    let bbox = BoundingBox {
        center: Center {
            x: openscenario_rs::types::basic::Value::literal(0.0),
            y: openscenario_rs::types::basic::Value::literal(0.0),
            z: openscenario_rs::types::basic::Value::literal(0.0),
        },
        dimensions: Dimensions::car(),
    };

    println!("   Car BoundingBox:");
    println!("   - Volume: {:.2} m³", bbox.volume_with_params(&params)?);
    println!("   - Contains point (0,0,0): {}", bbox.contains_point_with_params(0.0, 0.0, 0.0, &params)?);
    println!("   - Distance to point (3,0,0): {:.2} m", bbox.distance_to_point(3.0, 0.0, 0.0, &params)?);

    // Test different vehicle dimension presets
    println!("\n2. Vehicle Dimension Presets:");
    let car_dims = Dimensions::car();
    let truck_dims = Dimensions::truck();
    let motorcycle_dims = Dimensions::motorcycle();
    let pedestrian_dims = Dimensions::pedestrian();
    
    println!("   - Car footprint: {:.1} m²", car_dims.footprint_area(&params)?);
    println!("   - Truck footprint: {:.1} m²", truck_dims.footprint_area(&params)?);
    println!("   - Motorcycle footprint: {:.1} m²", motorcycle_dims.footprint_area(&params)?);
    println!("   - Pedestrian footprint: {:.1} m²", pedestrian_dims.footprint_area(&params)?);

    // Test scaling
    println!("\n3. Dimension Scaling:");
    let scaled_car = car_dims.scale(1.5, &params)?;
    println!("   - Original car footprint: {:.1} m²", car_dims.footprint_area(&params)?);
    println!("   - 1.5x scaled car footprint: {:.1} m²", scaled_car.footprint_area(&params)?);

    // === Collision Detection Demo ===
    println!("\n4. Collision Detection:");
    
    let bbox1 = BoundingBox {
        center: Center::default(),
        dimensions: Dimensions::car(),
    };
    
    let bbox2 = BoundingBox {
        center: Center {
            x: openscenario_rs::types::basic::Value::literal(3.0),
            y: openscenario_rs::types::basic::Value::literal(0.0),
            z: openscenario_rs::types::basic::Value::literal(0.0),
        },
        dimensions: Dimensions::car(),
    };

    println!("   Two cars 3m apart:");
    println!("   - Intersecting: {}", bbox1.intersects(&bbox2, &params)?);
    
    let bbox3 = BoundingBox {
        center: Center {
            x: openscenario_rs::types::basic::Value::literal(1.0),
            y: openscenario_rs::types::basic::Value::literal(0.0),
            z: openscenario_rs::types::basic::Value::literal(0.0),
        },
        dimensions: Dimensions::car(),
    };
    
    println!("   Two cars 1m apart:");
    println!("   - Intersecting: {}", bbox1.intersects(&bbox3, &params)?);

    // === Point Containment Tests ===
    println!("\n5. Point Containment Tests:");
    let test_bbox = BoundingBox {
        center: Center::default(),
        dimensions: Dimensions {
            width: openscenario_rs::types::basic::Value::literal(2.0),
            length: openscenario_rs::types::basic::Value::literal(4.0),
            height: openscenario_rs::types::basic::Value::literal(1.5),
        },
    };

    let test_points = vec![
        (0.0, 0.0, 0.0),   // Center
        (1.9, 0.9, 0.7),   // Inside
        (2.1, 0.0, 0.0),   // Outside X
        (0.0, 1.1, 0.0),   // Outside Y
        (0.0, 0.0, 0.8),   // Outside Z
    ];

    for (x, y, z) in test_points {
        let contains = test_bbox.contains_point_with_params(x, y, z, &params)?;
        let distance = test_bbox.distance_to_point(x, y, z, &params)?;
        println!("   Point ({:.1}, {:.1}, {:.1}): contains={}, distance={:.2}m", 
                 x, y, z, contains, distance);
    }

    println!("\n=== Demo Complete ===");
    Ok(())
}