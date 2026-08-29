//! Basic parsing example demonstrating simple OpenSCENARIO file loading
//!
//! This example contains:
//! - Simple scenario file loading and parsing
//! - Basic error handling and validation
//! - Accessing parsed scenario data and entities
//! - Demonstrating the high-level convenience API
//! - Basic scenario introspection and data access patterns
//!
//! Run with: `cargo run --example basic_parsing`

use openscenario_rs::{parse_file, types::OpenScenarioDocumentType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse a scenario file. `parse_file` reports the path in its error, so a missing
    // file surfaces as a returned error rather than a panic.
    let document = parse_file("tests/data/cut_in_101_exam.xosc")?;

    // Access file header information
    println!("Scenario: {}", document.file_header.description);
    println!("Author: {}", document.file_header.author);

    match document.document_type() {
        OpenScenarioDocumentType::Scenario => {
            if let Some(entities) = &document.entities {
                println!("Entities:");
                for entity in &entities.scenario_objects {
                    println!("  - {}", entity.name);
                }
            }

            if let Some(storyboard) = &document.storyboard {
                println!("Stories: {}", storyboard.stories.len());
            }

            if let Some(catalog_locations) = &document.catalog_locations {
                if let Some(vehicle_catalog) = &catalog_locations.vehicle_catalog {
                    println!("Vehicle catalog: {}", vehicle_catalog.directory.path);
                }
            }
        }
        OpenScenarioDocumentType::ParameterVariation => {
            println!("Parameter variation file - no entities/storyboard");
        }
        OpenScenarioDocumentType::Catalog => {
            println!("Catalog file - no entities/storyboard");
        }
        OpenScenarioDocumentType::Unknown => {
            println!("Unknown document type");
        }
    }

    Ok(())
}
