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

use openscenario_rs::{
    parse_str,
    types::{catalogs::locations, VehicleCatalogLocation},
};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse a scenario file
    let xml = fs::read_to_string(
        "xosc/concrete_scenarios/alks_scenario_4_1_1_free_driving_template.xosc",
    )
    .expect("Failed to read cut_in_101_exam.xosc file");
    if let Ok(document) = parse_str(&xml) {
        // Access file header information
        println!("Scenario: {:?}", document.file_header.description);
        println!("Author: {:?}", document.file_header.author);

        match document.document_type() {
            openscenario_rs::types::OpenScenarioDocumentType::Scenario => {
                if let Some(storyboard) = &document.storyboard {
                    let _storyboard = storyboard;
                }
                
                // Access entities
                if let Some(entities) = &document.entities {
                    println!("Entities:");
                    for entity in &entities.scenario_objects {
                        println!("  - {:?}", entity.name);
                    }
                }
                
                if let Some(catalog_locations) = &document.catalog_locations {
                    println!("Catalog locations: {:#?}", catalog_locations.vehicle_catalog);
                }
            }
            openscenario_rs::types::OpenScenarioDocumentType::ParameterVariation => {
                println!("Parameter variation file - no entities/storyboard");
            }
            openscenario_rs::types::OpenScenarioDocumentType::Catalog => {
                println!("Catalog file - no entities/storyboard");
            }
            openscenario_rs::types::OpenScenarioDocumentType::Unknown => {
                println!("Unknown document type");
            }
        }
    }

    Ok(())
}

// TODO: Add error handling demonstration
// TODO: Show how to handle different error types
// TODO: Demonstrate graceful error recovery

// TODO: Add example OpenSCENARIO file
// TODO: Create examples/simple_scenario.xosc with minimal valid content
