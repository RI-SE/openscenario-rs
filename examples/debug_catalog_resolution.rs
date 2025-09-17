//! Debug catalog resolution for ALKS scenarios

use openscenario_rs::{
    parser::xml::parse_from_file,
    catalog::{extract_scenario_parameters, resolve_catalog_reference_simple},
    error::Error,
    types::OpenScenarioDocumentType,
};
use std::env;
use std::path::Path;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <scenario.xosc>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    println!("Debugging catalog resolution for: {}", file_path);

    // Parse the scenario
    let document = parse_from_file(file_path)?;
    println!("Scenario parsed successfully!");

    match document.document_type() {
        OpenScenarioDocumentType::Scenario => {
            // Print catalog locations
            if let Some(catalog_locations) = &document.catalog_locations {
                println!("\n=== Catalog Locations ===");
                if let Some(vehicle_catalog) = &catalog_locations.vehicle_catalog {
                    println!("Vehicle Catalog: {:?}", vehicle_catalog.directory.path);
                }
                if let Some(pedestrian_catalog) = &catalog_locations.pedestrian_catalog {
                    println!("Pedestrian Catalog: {:?}", pedestrian_catalog.directory.path);
                }
                if let Some(misc_object_catalog) = &catalog_locations.misc_object_catalog {
                    println!("MiscObject Catalog: {:?}", misc_object_catalog.directory.path);
                }
                if let Some(controller_catalog) = &catalog_locations.controller_catalog {
                    println!("Controller Catalog: {:?}", controller_catalog.directory.path);
                }

                // Extract parameters from the scenario
                let parameters = extract_scenario_parameters(&document.parameter_declarations);
                println!("\n=== Parameters ===");
                for (name, value) in &parameters {
                    println!("  {} = {}", name, value);
                }

                // Get base path for catalog resolution
                let base_path = Path::new(file_path).parent().unwrap_or(Path::new("."));

                // Print entities and their catalog references
                if let Some(entities) = &document.entities {
                    println!("\n=== Entities ===");
                    for entity in &entities.scenario_objects {
                        println!("Entity: {:?}", entity.name);
                        match &entity.catalog_reference {
                            Some(catalog_ref) => {
                                println!("  Catalog Reference:");
                                println!("    Catalog Name: {:?}", catalog_ref.catalog_name);
                                println!("    Entry Name: {:?}", catalog_ref.entry_name);
                                
                                // Use the simple catalog resolution function
                                match resolve_catalog_reference_simple(
                                    &catalog_ref.catalog_name,
                                    &catalog_ref.entry_name,
                                    catalog_locations,
                                    &parameters,
                                    base_path,
                                ) {
                                    Ok(found) => {
                                        println!("    ✓ Catalog resolution completed:");
                                        println!("      Entry found: {}", if found { "✓ Yes" } else { "✗ No" });
                                    },
                                    Err(e) => {
                                        println!("    ✗ Catalog resolution failed: {}", e);
                                    }
                                }
                            },
                            None => {
                                println!("  No catalog reference (inline definition)");
                            }
                        }
                    }
                }
            } else {
                println!("\n=== No Catalog Locations ===");
            }
        }
        OpenScenarioDocumentType::ParameterVariation => {
            println!("Parameter variation file - no entities/catalog resolution");
        }
        OpenScenarioDocumentType::Catalog => {
            println!("Catalog file - no entities/catalog resolution");
        }
        OpenScenarioDocumentType::Unknown => {
            println!("Unknown document type - no entities/catalog resolution");
        }
    }

    Ok(())
}