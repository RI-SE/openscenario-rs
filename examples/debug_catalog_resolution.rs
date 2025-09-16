//! Debug catalog resolution for ALKS scenarios

use openscenario_rs::{
    parser::xml::parse_from_file,
    catalog::{extract_scenario_parameters, resolve_catalog_reference_simple},
    error::Error,
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
    let scenario = parse_from_file(file_path)?;
    println!("Scenario parsed successfully!");

    // Print catalog locations
    if let Some(catalog_locations) = &scenario.catalog_locations {
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
    }

    // Extract parameters from the scenario
    let parameters = extract_scenario_parameters(&scenario.parameter_declarations);
    println!("\n=== Parameters ===");
    for (name, value) in &parameters {
        println!("  {} = {}", name, value);
    }

    // Get base path for catalog resolution
    let base_path = Path::new(file_path).parent().unwrap_or(Path::new("."));

    // Print entities and their catalog references
    println!("\n=== Entities ===");
    for entity in &scenario.entities.scenario_objects {
        println!("Entity: {:?}", entity.name);
        match &entity.catalog_reference {
            Some(catalog_ref) => {
                println!("  Catalog Reference:");
                println!("    Catalog Name: {:?}", catalog_ref.catalog_name);
                println!("    Entry Name: {:?}", catalog_ref.entry_name);
                
                // Use the simple catalog resolution function
                if let Some(catalog_locations) = &scenario.catalog_locations {
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
                } else {
                    println!("    ✗ No catalog locations specified in scenario");
                }
            },
            None => {
                println!("  No catalog reference (inline definition)");
            }
        }
    }

    Ok(())
}