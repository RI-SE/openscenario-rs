//! OpenSCENARIO Catalog Resolution Tool
//!
//! This command-line tool parses OpenSCENARIO files and resolves all catalog references
//! into a fully resolved scenario file. It demonstrates the complete catalog system
//! integration for production use cases.
//!
//! ## Features
//! - Command-line interface for scenario file processing
//! - Complete catalog reference resolution with parameter substitution
//! - Automatic catalog discovery and loading
//! - Circular dependency detection and error handling
//! - Clean output generation with resolved entities
//! - Comprehensive logging and progress reporting
//!
//! ## Usage
//! ```bash
//! cargo run --example parse -- path/to/scenario.xosc
//! ```
//!
//! ## Output
//! - Creates an 'output' directory if it doesn't exist
//! - Generates a resolved scenario file: `output/resolved_scenario.xosc`
//! - Logs all resolution activities and statistics

use openscenario_rs::{
    catalog::CatalogLoader,
    expression::evaluate_expression,
    parse_str, serialize_str,
    types::{
        basic::{Directory, Value},
        catalogs::entities::CatalogEntity,
    },
    OpenScenario,
};
use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

/// Main application entry point
fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <scenario_file.xosc>", args[0]);
        eprintln!();
        eprintln!("Example: cargo run --example parse -- xosc/alks_scenario_4_1_1_free_driving_template.xosc");
        std::process::exit(1);
    }

    let input_file = &args[1];

    // Run the catalog resolution process
    match process_scenario(input_file) {
        Ok(output_path) => {
            println!(
                "✅ SUCCESS: Resolved scenario written to: {}",
                output_path.display()
            );
        }
        Err(e) => {
            eprintln!("❌ ERROR: {}", e);
            std::process::exit(1);
        }
    }
}

/// Main processing function that handles the entire catalog resolution pipeline
fn process_scenario(input_file: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    println!("🚀 OpenSCENARIO Catalog Resolution Tool");
    println!("═══════════════════════════════════════");

    // Step 1: Load and parse the input scenario
    println!("\n📁 Loading scenario file: {}", input_file);
    let input_path = Path::new(input_file);
    if !input_path.exists() {
        return Err(format!("Input file does not exist: {}", input_file).into());
    }

    let xml = fs::read_to_string(input_path)?;
    let mut scenario = parse_str(&xml)?;

    println!("✅ Successfully parsed scenario file");
    println!("   📋 Description: {:?}", scenario.file_header.description);
    println!("   👤 Author: {:?}", scenario.file_header.author);
    println!("   📅 Date: {:?}", scenario.file_header.date);
    println!(
        "   🎭 Entities: {}",
        scenario.entities.scenario_objects.len()
    );

    // Step 2: Extract parameters from scenario
    let scenario_parameters = extract_scenario_parameters(&scenario);
    println!("   ⚙️  Parameters: {}", scenario_parameters.len());
    for (name, value) in &scenario_parameters {
        println!("      - {} = {}", name, value);
    }

    // Step 3: Check if scenario uses catalogs
    if let Some(catalog_locations) = scenario.catalog_locations.clone() {
        println!("\n🗂️  Catalog locations found - proceeding with resolution:");

        // Get the base directory for relative catalog paths
        let base_dir = input_path.parent().unwrap_or(Path::new("."));

        // Resolve catalog references
        resolve_catalog_references(
            &mut scenario,
            &catalog_locations,
            &scenario_parameters,
            base_dir,
        )?;
    } else {
        println!("\n💡 No catalog locations found - scenario uses inline entities only");
    }

    // Step 4: Resolve all expressions in the scenario
    println!("\n🧮 Resolving mathematical expressions...");
    resolve_all_expressions(&mut scenario, &scenario_parameters)?;

    // Step 5: Create output directory
    let output_dir = Path::new("output");
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
        println!("\n📁 Created output directory: {}", output_dir.display());
    }

    // Step 5: Generate output filename
    let input_stem = input_path
        .file_stem()
        .unwrap_or_else(|| std::ffi::OsStr::new("scenario"));
    let output_filename = format!("{}_resolved.xosc", input_stem.to_string_lossy());
    let output_path = output_dir.join(output_filename);

    // Step 6: Serialize and write the resolved scenario
    println!("\n💾 Serializing resolved scenario...");
    let resolved_xml = serialize_str(&scenario)?;

    let mut output_file = File::create(&output_path)?;
    output_file.write_all(resolved_xml.as_bytes())?;

    println!("✅ Resolved scenario written to: {}", output_path.display());
    println!("   📊 Output size: {} bytes", resolved_xml.len());

    // Step 7: Report resolution statistics
    print_resolution_summary(&scenario);

    Ok(output_path)
}

/// Extract parameter declarations from the scenario for use in catalog resolution
fn extract_scenario_parameters(scenario: &OpenScenario) -> HashMap<String, String> {
    let mut parameters = HashMap::new();

    if let Some(param_declarations) = &scenario.parameter_declarations {
        for param in &param_declarations.parameter_declarations {
            if let (Some(name), Some(value)) = (param.name.as_literal(), param.value.as_literal()) {
                parameters.insert(name.clone(), value.clone());
            }
        }
    }

    parameters
}

/// Main catalog resolution function that processes all catalog references in the scenario
fn resolve_catalog_references(
    scenario: &mut OpenScenario,
    catalog_locations: &openscenario_rs::types::catalogs::locations::CatalogLocations,
    parameters: &HashMap<String, String>,
    base_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let catalog_loader = CatalogLoader::new();
    let mut resolved_vehicles = 0;
    let mut resolved_controllers = 0;

    // Process each entity in the scenario
    for entity in &mut scenario.entities.scenario_objects {
        let entity_name = entity
            .name
            .as_literal()
            .unwrap_or(&"<unknown>".to_string())
            .clone();
        println!("   🎭 Processing entity: {}", entity_name);

        // Resolve vehicle catalog references
        if let Some(vehicle_ref) = &entity.catalog_reference {
            println!("      🚗 Resolving vehicle reference...");

            if let Some(vehicle_catalog_location) = &catalog_locations.vehicle_catalog {
                match resolve_vehicle_reference(
                    &catalog_loader,
                    vehicle_ref,
                    vehicle_catalog_location,
                    parameters,
                    base_dir,
                ) {
                    Ok(resolved_vehicle) => {
                        println!(
                            "         ✅ Vehicle resolved: {} -> {}",
                            vehicle_ref
                                .entry_name
                                .as_literal()
                                .unwrap_or(&"<unknown>".to_string()),
                            resolved_vehicle
                                .name
                                .as_literal()
                                .unwrap_or(&"<unknown>".to_string())
                        );

                        // Replace catalog reference with resolved inline entity
                        entity.catalog_reference = None;
                        entity.vehicle = Some(resolved_vehicle);
                        resolved_vehicles += 1;
                    }
                    Err(e) => {
                        println!("         ❌ Failed to resolve vehicle: {}", e);
                        return Err(e);
                    }
                }
            }
        }

        // Resolve controller catalog references
        if let Some(object_controller) = &mut entity.object_controller {
            if let Some(controller_ref) = &object_controller.catalog_reference {
                println!("      🎮 Resolving controller reference...");

                if let Some(controller_catalog_location) = &catalog_locations.controller_catalog {
                    match resolve_controller_reference(
                        &catalog_loader,
                        controller_ref,
                        controller_catalog_location,
                        parameters,
                        base_dir,
                    ) {
                        Ok(resolved_controller) => {
                            println!(
                                "         ✅ Controller resolved: {} -> {}",
                                controller_ref
                                    .entry_name
                                    .as_literal()
                                    .unwrap_or(&"<unknown>".to_string()),
                                resolved_controller
                                    .name
                                    .as_literal()
                                    .unwrap_or(&"<unknown>".to_string())
                            );

                            // Replace catalog reference with resolved inline entity
                            object_controller.catalog_reference = None;
                            object_controller.controller = Some(resolved_controller);
                            resolved_controllers += 1;
                        }
                        Err(e) => {
                            println!("         ❌ Failed to resolve controller: {}", e);
                            return Err(e);
                        }
                    }
                }
            }
        }
    }

    println!(
        "   📊 Resolution complete: {} vehicles, {} controllers",
        resolved_vehicles, resolved_controllers
    );

    // Remove catalog locations from the resolved scenario since everything is now inline
    scenario.catalog_locations = None;

    Ok(())
}

/// Resolve a specific vehicle catalog reference to a concrete vehicle entity
fn resolve_vehicle_reference(
    catalog_loader: &CatalogLoader,
    vehicle_ref: &openscenario_rs::types::catalogs::references::VehicleCatalogReference,
    vehicle_location: &openscenario_rs::types::catalogs::locations::VehicleCatalogLocation,
    scenario_parameters: &HashMap<String, String>,
    base_dir: &Path,
) -> Result<openscenario_rs::types::entities::vehicle::Vehicle, Box<dyn std::error::Error>> {
    // Resolve the catalog directory path
    let catalog_dir_path = vehicle_location
        .directory
        .path
        .as_literal()
        .ok_or("Cannot resolve parameterized catalog directory paths")?;

    let resolved_catalog_dir = if Path::new(catalog_dir_path).is_relative() {
        base_dir.join(catalog_dir_path)
    } else {
        PathBuf::from(catalog_dir_path)
    };

    let catalog_dir = Directory {
        path: Value::literal(resolved_catalog_dir.to_string_lossy().to_string()),
    };

    // Load all vehicle catalogs
    let catalog_vehicles = catalog_loader.load_vehicle_catalogs(&catalog_dir)?;

    // Resolve the vehicle entry name (may be parameterized like $LeadVehicle_Model)
    let entry_name = vehicle_ref.entry_name.resolve(scenario_parameters)?;

    let catalog_vehicle = catalog_vehicles
        .iter()
        .find(|v| v.entity_name() == entry_name)
        .ok_or_else(|| format!("Vehicle '{}' not found in catalog", entry_name))?;

    // Collect parameters for resolution (scenario params + reference params)
    let mut resolution_parameters = scenario_parameters.clone();

    if let Some(param_assignments) = &vehicle_ref.parameter_assignments {
        for assignment in param_assignments {
            if let (Some(param_name), Some(param_value)) = (
                assignment.parameter_ref.as_literal(),
                assignment.value.as_literal(),
            ) {
                resolution_parameters.insert(param_name.clone(), param_value.clone());
            }
        }
    }

    // Convert catalog vehicle to scenario vehicle
    catalog_vehicle
        .clone()
        .into_scenario_entity(resolution_parameters)
        .map_err(|e| e.into())
}

/// Resolve a specific controller catalog reference to a concrete controller entity
fn resolve_controller_reference(
    catalog_loader: &CatalogLoader,
    controller_ref: &openscenario_rs::types::catalogs::references::ControllerCatalogReference,
    controller_location: &openscenario_rs::types::catalogs::locations::ControllerCatalogLocation,
    scenario_parameters: &HashMap<String, String>,
    base_dir: &Path,
) -> Result<openscenario_rs::types::controllers::Controller, Box<dyn std::error::Error>> {
    // Resolve the catalog directory path
    let catalog_dir_path = controller_location
        .directory
        .path
        .as_literal()
        .ok_or("Cannot resolve parameterized catalog directory paths")?;

    let resolved_catalog_dir = if Path::new(catalog_dir_path).is_relative() {
        base_dir.join(catalog_dir_path)
    } else {
        PathBuf::from(catalog_dir_path)
    };

    let catalog_dir = Directory {
        path: Value::literal(resolved_catalog_dir.to_string_lossy().to_string()),
    };

    // Load all controller catalogs
    let catalog_controllers = catalog_loader.load_controller_catalogs(&catalog_dir)?;

    // Resolve the controller entry name (may be parameterized)
    let entry_name = controller_ref.entry_name.resolve(scenario_parameters)?;

    let catalog_controller = catalog_controllers
        .iter()
        .find(|c| c.entity_name() == entry_name)
        .ok_or_else(|| format!("Controller '{}' not found in catalog", entry_name))?;

    // Collect parameters for resolution (scenario params + reference params)
    let mut resolution_parameters = scenario_parameters.clone();

    if let Some(param_assignments) = &controller_ref.parameter_assignments {
        for assignment in param_assignments {
            if let (Some(param_name), Some(param_value)) = (
                assignment.parameter_ref.as_literal(),
                assignment.value.as_literal(),
            ) {
                resolution_parameters.insert(param_name.clone(), param_value.clone());
            }
        }
    }

    // Convert catalog controller to scenario controller
    catalog_controller
        .clone()
        .into_scenario_entity(resolution_parameters)
        .map_err(|e| e.into())
}

/// Print a comprehensive summary of the resolution process
fn print_resolution_summary(scenario: &OpenScenario) {
    println!("\n📈 Resolution Summary");
    println!("════════════════════");

    let entity_count = scenario.entities.scenario_objects.len();
    let mut inline_vehicles = 0;
    let mut inline_controllers = 0;
    let mut remaining_catalog_refs = 0;

    for entity in &scenario.entities.scenario_objects {
        if entity.vehicle.is_some() {
            inline_vehicles += 1;
        }
        if let Some(object_controller) = &entity.object_controller {
            if object_controller.controller.is_some() {
                inline_controllers += 1;
            }
        }
        if entity.catalog_reference.is_some()
            || entity
                .object_controller
                .as_ref()
                .map_or(false, |oc| oc.catalog_reference.is_some())
        {
            remaining_catalog_refs += 1;
        }
    }

    println!("   🎭 Total entities: {}", entity_count);
    println!("   🚗 Inline vehicles: {}", inline_vehicles);
    println!("   🎮 Inline controllers: {}", inline_controllers);
    println!(
        "   🔗 Remaining catalog references: {}",
        remaining_catalog_refs
    );

    if remaining_catalog_refs == 0 {
        println!("   ✅ All catalog references successfully resolved!");
    } else {
        println!("   ⚠️  Some catalog references could not be resolved");
    }

    println!(
        "   🗂️  Catalog locations: {}",
        if scenario.catalog_locations.is_some() {
            "Present (will be removed in output)"
        } else {
            "None"
        }
    );
}

/// Resolve all mathematical expressions in the scenario using the parameter context
fn resolve_all_expressions(
    scenario: &mut OpenScenario,
    parameters: &HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut expressions_resolved = 0;
    let mut expressions_failed = 0;

    // Resolve expressions in scenario entities
    for entity in &mut scenario.entities.scenario_objects {
        // Process vehicle properties if present
        if let Some(vehicle) = &mut entity.vehicle {
            expressions_resolved +=
                resolve_vehicle_expressions(vehicle, parameters, &mut expressions_failed)?;
        }
    }

    // Resolve expressions in storyboard
    expressions_resolved += resolve_storyboard_expressions(
        &mut scenario.storyboard,
        parameters,
        &mut expressions_failed,
    )?;

    println!(
        "   ✅ Expressions resolved: {} success, {} failed",
        expressions_resolved, expressions_failed
    );

    Ok(())
}

/// Resolve expressions in vehicle properties
fn resolve_vehicle_expressions(
    vehicle: &mut openscenario_rs::types::entities::vehicle::Vehicle,
    parameters: &HashMap<String, String>,
    expressions_failed: &mut u32,
) -> Result<u32, Box<dyn std::error::Error>> {
    let mut resolved_count = 0;

    // Resolve performance values
    resolved_count += resolve_value_expression(
        &mut vehicle.performance.max_speed,
        parameters,
        expressions_failed,
    )?;
    resolved_count += resolve_value_expression(
        &mut vehicle.performance.max_acceleration,
        parameters,
        expressions_failed,
    )?;
    resolved_count += resolve_value_expression(
        &mut vehicle.performance.max_deceleration,
        parameters,
        expressions_failed,
    )?;

    Ok(resolved_count)
}

/// Resolve expressions in the storyboard
fn resolve_storyboard_expressions(
    storyboard: &mut openscenario_rs::types::scenario::storyboard::Storyboard,
    parameters: &HashMap<String, String>,
    expressions_failed: &mut u32,
) -> Result<u32, Box<dyn std::error::Error>> {
    let mut resolved_count = 0;

    // Resolve expressions in initial actions
    for private in &mut storyboard.init.actions.private_actions {
        for action in &mut private.private_actions {
            resolved_count += resolve_action_expressions(action, parameters, expressions_failed)?;
        }
    }

    // Resolve expressions in stop trigger
    if let Some(stop_trigger) = &mut storyboard.stop_trigger {
        for condition_group in &mut stop_trigger.condition_groups {
            for condition in &mut condition_group.conditions {
                resolved_count +=
                    resolve_condition_expressions(condition, parameters, expressions_failed)?;
            }
        }
    }

    Ok(resolved_count)
}

/// Resolve expressions in private actions
fn resolve_action_expressions(
    action: &mut openscenario_rs::types::scenario::init::PrivateActionWrapper,
    parameters: &HashMap<String, String>,
    expressions_failed: &mut u32,
) -> Result<u32, Box<dyn std::error::Error>> {
    let mut resolved_count = 0;

    if let Some(longitudinal) = &mut action.longitudinal_action {
        if let Some(speed_action) = &mut longitudinal.speed_action {
            resolved_count += resolve_value_expression(
                &mut speed_action.speed_action_dynamics.value,
                parameters,
                expressions_failed,
            )?;
            if let Some(abs_target) = &mut speed_action.speed_action_target.absolute {
                resolved_count += resolve_value_expression(
                    &mut abs_target.value,
                    parameters,
                    expressions_failed,
                )?;
            }
        }
    }

    Ok(resolved_count)
}

/// Resolve expressions in trigger conditions
fn resolve_condition_expressions(
    condition: &mut openscenario_rs::types::scenario::triggers::Condition,
    parameters: &HashMap<String, String>,
    expressions_failed: &mut u32,
) -> Result<u32, Box<dyn std::error::Error>> {
    let mut resolved_count = 0;

    if let Some(by_value) = &mut condition.by_value_condition {
        if let Some(sim_time) = &mut by_value.simulation_time {
            resolved_count +=
                resolve_value_expression(&mut sim_time.value, parameters, expressions_failed)?;
        }
    }

    Ok(resolved_count)
}

/// Resolve a single Value<T> expression to its literal value
fn resolve_value_expression<T>(
    value: &mut Value<T>,
    parameters: &HashMap<String, String>,
    expressions_failed: &mut u32,
) -> Result<u32, Box<dyn std::error::Error>>
where
    T: std::str::FromStr + Clone,
    T::Err: std::fmt::Display,
{
    match value {
        Value::Expression(expr) => {
            let expr_string = expr.clone(); // Clone to avoid borrowing issues
            match evaluate_expression::<f64>(&expr_string, parameters) {
                Ok(result) => {
                    // Convert the result back to the target type
                    let result_str = result.to_string();
                    match result_str.parse::<T>() {
                        Ok(literal_value) => {
                            *value = Value::Literal(literal_value);
                            println!("      ✅ Resolved: {} → {}", expr_string, result_str);
                            Ok(1)
                        }
                        Err(e) => {
                            *expressions_failed += 1;
                            println!(
                                "      ❌ Parse error for {}: {} → {} ({})",
                                expr_string, result, result_str, e
                            );
                            Ok(0)
                        }
                    }
                }
                Err(e) => {
                    *expressions_failed += 1;
                    println!("      ❌ Evaluation error for {}: {}", expr_string, e);
                    Ok(0)
                }
            }
        }
        Value::Parameter(_) | Value::Literal(_) => Ok(0), // No expression to resolve
    }
}
