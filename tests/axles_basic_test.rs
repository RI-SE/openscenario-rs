//! Basic test for axles module functionality

use std::collections::HashMap;

#[test]
fn test_axles_module_basic() {
    // Test that we can import and use the axles module
    use openscenario_rs::types::{Axle, Axles};

    let params = HashMap::new();

    // Test basic axle creation
    let car_axles = Axles::car();
    assert_eq!(car_axles.axle_count(), 2);

    let truck_axles = Axles::truck();
    assert_eq!(truck_axles.axle_count(), 3);

    // Test individual axle
    let front_axle = Axle::front_car();
    let circumference = front_axle.wheel_circumference(&params).unwrap();
    assert!(circumference > 0.0);
}

#[test]
fn test_vehicle_with_axles() {
    // Test that Vehicle can use the new axles system
    use openscenario_rs::types::Vehicle;

    let vehicle = Vehicle::default();
    assert_eq!(vehicle.axle_count(), 2);
}
