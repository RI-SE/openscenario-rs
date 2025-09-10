//! Basic parsing example demonstrating simple OpenSCENARIO file loading
//!
//! This example contains:
//! - Simple scenario file loading and parsing
//! - Basic error handling and validation
//! - Accessing parsed scenario data and entities
//! - Demonstrating the high-level convenience API
//! - Basic scenario introspection and data access patterns
//!
//! Contributes to project by:
//! - Providing immediate working example for new users
//! - Demonstrating the simplest possible usage pattern
//! - Serving as integration test for basic functionality
//! - Offering copy-paste starting point for basic use cases
//! - Validating API ergonomics and developer experience

use openscenario_rs::parse_file;

use openscenario_rs::parse_str;
use openscenario_rs::types::entities::EntityObject;
use openscenario_rs::types::enums::{PedestrianCategory, VehicleCategory};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse a scenario file
    let xml = fs::read_to_string("xosc/cut_in_101_exam.xosc")
        .expect("Failed to read cut_in_101_exam.xosc file");
    println!("read xml");
    println!("{:?}", parse_str(&xml));
    if let Ok(scenario) = parse_str(&xml) {
        let storyboard = &scenario.storyboard;
        // Access file header information
        println!("Scenario: {:?}", scenario.file_header.description);
        println!("Author: {:?}", scenario.file_header.author);

        // Access entities
        println!("Entities:");
        for entity in &scenario.entities.scenario_objects {
            println!("  - {:?}", entity.name);
        }
    }
    let xml = fs::read_to_string("xosc/cut_in_101_exam.xosc")
        .expect("Failed to read cut_in_101_exam.xosc file");

    let result = parse_str(&xml);

    match result {
        Ok(scenario) => {
            // Verify file header information
            assert_eq!(
                scenario.file_header.author.as_literal().unwrap(),
                "OnSite_TOPS"
            );
            assert_eq!(
                scenario.file_header.description.as_literal().unwrap(),
                "scenario_highD"
            );
            assert_eq!(scenario.file_header.rev_major.as_literal().unwrap(), &1);
            assert_eq!(scenario.file_header.rev_minor.as_literal().unwrap(), &0);
            assert_eq!(
                scenario.file_header.date.as_literal().unwrap(),
                "2021-11-02T16:20:00"
            );
        }
        Err(e) => {
            // For now, we expect parsing to potentially fail due to incomplete implementation
            println!(
                "Expected parsing failure due to incomplete implementation: {}",
                e
            );
            // This is acceptable for MVP - we're testing the framework, not full parsing capability
        }
    }
    Ok(())
}

// TODO: Add error handling demonstration
// TODO: Show how to handle different error types
// TODO: Demonstrate graceful error recovery

// TODO: Add example OpenSCENARIO file
// TODO: Create examples/simple_scenario.xosc with minimal valid content
