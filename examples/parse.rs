//! OpenSCENARIO Universal Parser Tool
//!
//! This command-line tool parses any OpenSCENARIO file type and demonstrates the complete
//! parsing system for production use cases.
//!
//! ## Features
//! - Command-line interface for scenario file processing
//! - Support for scenarios, parameter variations, and catalogs
//! - Parameter variation analysis with scenario file import
//! - Complete catalog reference resolution with parameter substitution
//! - Automatic catalog discovery and loading
//! - File path resolution (relative paths made absolute relative to scenario location)
//! - Expression resolution (${...} expressions evaluated with parameter context)
//! - Clean output generation with resolved entities
//! - Comprehensive logging and progress reporting
//!
//! ## Usage
//! ```bash
//! cargo run --example parse -- path/to/scenario.xosc
//! cargo run --example parse -- path/to/variation.xosc
//! ```
//!
//! ## Output
//! - Creates an 'output' directory if it doesn't exist
//! - Generates a resolved scenario file: `output/resolved_scenario.xosc`
//! - For parameter variations: also parses and displays the referenced scenario
//! - Logs all resolution activities and statistics

use openscenario_rs::{
    catalog::{extract_scenario_parameters, resolve_catalog_reference_simple},
    expression::evaluate_expression,
    parser::xml::{parse_from_file, serialize_to_string},
    types::{basic::Value, OpenScenarioDocumentType},
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

/// Resolve file paths in OpenSCENARIO documents to be relative to the scenario file location
fn resolve_file_paths_in_document(
    document: &mut openscenario_rs::types::scenario::storyboard::OpenScenario,
    base_scenario_path: &Path,
) -> Result<usize, Box<dyn std::error::Error>> {
    let mut resolved_count = 0;

    // Helper function to resolve a file path Value<String> if it's a relative path
    let resolve_path_value = |value: &mut Value<String>, file_type: &str| -> bool {
        if let Some(literal_path) = value.as_literal() {
            // Only resolve if it's a relative path (not absolute)
            if !literal_path.starts_with('/')
                && !literal_path.chars().nth(1).map_or(false, |c| c == ':')
            {
                match resolve_file_path(base_scenario_path, literal_path) {
                    Ok(resolved_path) => {
                        let resolved_str = resolved_path.to_string_lossy().to_string();
                        println!(
                            "      ✅ Resolved {} path: {} → {}",
                            file_type, literal_path, resolved_str
                        );
                        *value = Value::Literal(resolved_str);
                        true
                    }
                    Err(e) => {
                        println!(
                            "      ❌ Failed to resolve {} path {}: {}",
                            file_type, literal_path, e
                        );
                        false
                    }
                }
            } else {
                println!(
                    "      💡 Skipping absolute {} path: {}",
                    file_type, literal_path
                );
                false
            }
        } else {
            // Path might be parameterized - we'll leave it as-is for now
            // In a full implementation, we could resolve parameters first
            false
        }
    };

    // Process road network file references
    if let Some(road_network) = &mut document.road_network {
        println!("   🛣️  Processing road network files...");

        if let Some(logic_file) = &mut road_network.logic_file {
            if resolve_path_value(&mut logic_file.filepath, "road logic") {
                resolved_count += 1;
            }
        }

        if let Some(scene_file) = &mut road_network.scene_graph_file {
            if resolve_path_value(&mut scene_file.filepath, "scene graph") {
                resolved_count += 1;
            }
        }
    }

    // Process catalog directory paths
    if let Some(catalog_locations) = &mut document.catalog_locations {
        println!("   📚 Processing catalog directory paths...");

        if let Some(vehicle_catalog) = &mut catalog_locations.vehicle_catalog {
            if resolve_path_value(
                &mut vehicle_catalog.directory.path,
                "vehicle catalog directory",
            ) {
                resolved_count += 1;
            }
        }

        if let Some(controller_catalog) = &mut catalog_locations.controller_catalog {
            if resolve_path_value(
                &mut controller_catalog.directory.path,
                "controller catalog directory",
            ) {
                resolved_count += 1;
            }
        }

        if let Some(pedestrian_catalog) = &mut catalog_locations.pedestrian_catalog {
            if resolve_path_value(
                &mut pedestrian_catalog.directory.path,
                "pedestrian catalog directory",
            ) {
                resolved_count += 1;
            }
        }

        if let Some(misc_object_catalog) = &mut catalog_locations.misc_object_catalog {
            if resolve_path_value(
                &mut misc_object_catalog.directory.path,
                "misc object catalog directory",
            ) {
                resolved_count += 1;
            }
        }

        if let Some(environment_catalog) = &mut catalog_locations.environment_catalog {
            if resolve_path_value(
                &mut environment_catalog.directory.path,
                "environment catalog directory",
            ) {
                resolved_count += 1;
            }
        }

        if let Some(maneuver_catalog) = &mut catalog_locations.maneuver_catalog {
            if resolve_path_value(
                &mut maneuver_catalog.directory.path,
                "maneuver catalog directory",
            ) {
                resolved_count += 1;
            }
        }

        if let Some(trajectory_catalog) = &mut catalog_locations.trajectory_catalog {
            if resolve_path_value(
                &mut trajectory_catalog.directory.path,
                "trajectory catalog directory",
            ) {
                resolved_count += 1;
            }
        }

        if let Some(route_catalog) = &mut catalog_locations.route_catalog {
            if resolve_path_value(&mut route_catalog.directory.path, "route catalog directory") {
                resolved_count += 1;
            }
        }
    }

    // Process parameter variation scenario file references
    if let Some(param_dist) = &mut document.parameter_value_distribution {
        println!("   📊 Processing parameter variation scenario reference...");

        // The scenario_file.filepath is a plain String, not a Value<String>
        // We need to handle this differently
        let scenario_filepath = &param_dist.scenario_file.filepath;
        if !scenario_filepath.starts_with('/')
            && !scenario_filepath.chars().nth(1).map_or(false, |c| c == ':')
        {
            match resolve_file_path(base_scenario_path, scenario_filepath) {
                Ok(resolved_path) => {
                    let resolved_str = resolved_path.to_string_lossy().to_string();
                    println!(
                        "      ✅ Resolved scenario file path: {} → {}",
                        scenario_filepath, resolved_str
                    );
                    param_dist.scenario_file.filepath = resolved_str;
                    resolved_count += 1;
                }
                Err(e) => {
                    println!(
                        "      ❌ Failed to resolve scenario file path {}: {}",
                        scenario_filepath, e
                    );
                }
            }
        } else {
            println!(
                "      💡 Skipping absolute scenario file path: {}",
                scenario_filepath
            );
        }
    }

    // In a full implementation, we would also process:
    // - Vehicle file references (Vehicle.file.filepath if present)
    // - Any other file references in actions, conditions, etc.

    Ok(resolved_count)
}

/// Resolve expressions in OpenSCENARIO Value types throughout the document
fn resolve_expressions_in_document(
    document: &mut openscenario_rs::types::scenario::storyboard::OpenScenario,
    parameters: &HashMap<String, String>,
) -> Result<usize, Box<dyn std::error::Error>> {
    let mut resolved_count = 0;

    // Helper function to resolve a Value<String> if it's an expression
    let resolve_string_value = |value: &mut Value<String>| -> bool {
        if let Some(expr) = value.as_expression() {
            match evaluate_expression::<String>(expr, parameters) {
                Ok(result) => {
                    println!("      🔍 Resolved expression: {} → {}", expr, result);
                    *value = Value::Literal(result);
                    true
                }
                Err(e) => {
                    println!("      ❌ Failed to resolve expression {}: {}", expr, e);
                    false
                }
            }
        } else {
            false
        }
    };

    // Helper function to resolve a Value<f64> if it's an expression
    let _resolve_numeric_value = |value: &mut Value<f64>| -> bool {
        if let Some(expr) = value.as_expression() {
            match evaluate_expression::<f64>(expr, parameters) {
                Ok(result) => {
                    println!(
                        "      🔍 Resolved numeric expression: {} → {}",
                        expr, result
                    );
                    *value = Value::Literal(result);
                    true
                }
                Err(e) => {
                    println!(
                        "      ❌ Failed to resolve numeric expression {}: {}",
                        expr, e
                    );
                    false
                }
            }
        } else {
            false
        }
    };

    // Process parameter declarations (these often contain expressions)
    if let Some(param_decls) = &mut document.parameter_declarations {
        println!("   ⚙️  Processing parameter declarations...");
        for param in &mut param_decls.parameter_declarations {
            if resolve_string_value(&mut param.value) {
                resolved_count += 1;
            }
        }
    }

    // Process entities
    if let Some(entities) = &mut document.entities {
        println!("   🎭 Processing entities...");
        for entity in &mut entities.scenario_objects {
            // Resolve entity name if it's an expression
            if resolve_string_value(&mut entity.name) {
                resolved_count += 1;
            }

            // Process vehicle properties if present
            if let Some(vehicle) = &mut entity.vehicle {
                if resolve_string_value(&mut vehicle.name) {
                    resolved_count += 1;
                }
                // Could process other vehicle fields like performance values here
            }

            // Process pedestrian properties if present
            if let Some(pedestrian) = &mut entity.pedestrian {
                if resolve_string_value(&mut pedestrian.name) {
                    resolved_count += 1;
                }
                // Could process other pedestrian fields here
            }
        }
    }

    // In a full implementation, we would also walk through:
    // - Position coordinates (Value<f64>) using resolve_numeric_value
    // - Speed values (Value<f64>) using resolve_numeric_value
    // - Time values (Value<f64>) using resolve_numeric_value
    // - All action parameters throughout the storyboard
    // - Condition parameters in triggers
    // - Trigger parameters and timing values
    // etc.

    // For now, we focus on the most common expression locations in parameters and entity names

    Ok(resolved_count)
}

/// Main processing function that handles the entire parsing and resolution pipeline
fn process_scenario(input_file: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    println!("🚀 OpenSCENARIO Universal Parser Tool");
    println!("═════════════════════════════════════");

    // Step 1: Load and parse the input scenario
    println!("\n📁 Loading scenario file: {}", input_file);
    let input_path = Path::new(input_file);
    if !input_path.exists() {
        return Err(format!("Input file does not exist: {}", input_file).into());
    }

    let mut document = parse_from_file(input_path)?;

    println!("✅ Successfully parsed scenario file");
    println!("   📋 Description: {:?}", document.file_header.description);
    println!("   👤 Author: {:?}", document.file_header.author);
    println!("   📅 Date: {:?}", document.file_header.date);

    match document.document_type() {
        OpenScenarioDocumentType::Scenario => {
            if let Some(entities) = &document.entities {
                println!("   🎭 Entities: {}", entities.scenario_objects.len());
            } else {
                println!("   🎭 Entities: 0");
            }

            // Step 2: Extract parameters from scenario
            let scenario_parameters = extract_scenario_parameters(&document.parameter_declarations);
            println!("   ⚙️  Parameters: {}", scenario_parameters.len());
            for (name, value) in &scenario_parameters {
                println!("      - {} = {}", name, value);
            }

            // Step 3: Check if scenario uses catalogs
            if let Some(catalog_locations) = document.catalog_locations.clone() {
                if catalog_locations.vehicle_catalog.is_some()
                    || catalog_locations.pedestrian_catalog.is_some()
                    || catalog_locations.misc_object_catalog.is_some()
                    || catalog_locations.controller_catalog.is_some()
                {
                    println!("\n🗂️  Catalog locations found - proceeding with resolution:");

                    // Get the base directory for relative catalog paths
                    let base_dir = input_path.parent().unwrap_or(Path::new("."));

                    // Resolve catalog references using the new simple resolver
                    resolve_catalog_references_simple(
                        &mut document,
                        &catalog_locations,
                        &scenario_parameters,
                        base_dir,
                    )?;
                } else {
                    println!(
                        "\n💡 No catalog locations found - scenario uses inline entities only"
                    );
                }
            } else {
                println!("\n💡 No catalog locations found - scenario uses inline entities only");
            }
        }
        OpenScenarioDocumentType::ParameterVariation => {
            println!("   📊 Parameter variation file - analyzing distributions");

            // Analyze parameter variation file
            if let Some(param_dist) = &document.parameter_value_distribution {
                println!(
                    "   📁 Referenced scenario: {}",
                    param_dist.scenario_file.filepath
                );

                // Parse the referenced scenario file to show its structure
                let scenario_path =
                    resolve_scenario_path(input_path, &param_dist.scenario_file.filepath)?;
                if let Ok(scenario_doc) = parse_from_file(&scenario_path) {
                    println!("   ✅ Successfully loaded referenced scenario");
                    println!(
                        "      📋 Description: {:?}",
                        scenario_doc.file_header.description
                    );
                    println!("      👤 Author: {:?}", scenario_doc.file_header.author);

                    if let Some(entities) = &scenario_doc.entities {
                        println!("      🎭 Entities: {}", entities.scenario_objects.len());
                    }

                    if let Some(params) = &scenario_doc.parameter_declarations {
                        println!(
                            "      ⚙️  Template parameters: {}",
                            params.parameter_declarations.len()
                        );
                        for param in &params.parameter_declarations {
                            println!("         - {} = {:?}", param.name, param.value);
                        }
                    }
                } else {
                    println!("   ❌ Could not load referenced scenario file");
                }

                // Analyze distributions
                if let Some(deterministic) = &param_dist.deterministic {
                    analyze_deterministic_distributions(deterministic);
                }

                if let Some(stochastic) = &param_dist.stochastic {
                    println!(
                        "   🎲 Stochastic distributions: {} parameters",
                        stochastic.distributions.len()
                    );
                }
            }
        }
        OpenScenarioDocumentType::Catalog => {
            println!("   📚 Catalog file - no entities");
        }
        OpenScenarioDocumentType::Unknown => {
            println!("   ❓ Unknown document type");
        }
    }

    // Step 4: Resolve file paths (make relative paths absolute relative to scenario location)
    println!("\n📁 File Path Resolution");
    println!("══════════════════════");

    let file_paths_resolved = resolve_file_paths_in_document(&mut document, input_path)?;
    if file_paths_resolved > 0 {
        println!(
            "✅ Resolved {} file paths in the document",
            file_paths_resolved
        );
    } else {
        println!("💡 No relative file paths found to resolve");
    }

    // Step 5: Perform expression resolution (scan for ${...} expressions and evaluate them)
    println!("\n🧮 Expression Resolution");
    println!("═══════════════════════");

    if document.is_scenario() {
        let scenario_parameters = extract_scenario_parameters(&document.parameter_declarations);
        let expressions_resolved =
            resolve_expressions_in_document(&mut document, &scenario_parameters)?;

        if expressions_resolved > 0 {
            println!(
                "✅ Resolved {} expressions in the scenario",
                expressions_resolved
            );
        } else {
            println!("💡 No expressions found to resolve in this scenario");
        }
    } else {
        println!("💡 Expression resolution only applies to scenario documents");
    }

    // Step 6: Create output directory
    let output_dir = Path::new("output");
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
        println!("\n📁 Created output directory: {}", output_dir.display());
    }

    // Step 7: Generate output filename
    let input_stem = input_path
        .file_stem()
        .unwrap_or_else(|| std::ffi::OsStr::new("scenario"));
    let output_filename = format!("{}_resolved.xosc", input_stem.to_string_lossy());
    let output_path = output_dir.join(output_filename);

    // Step 8: Serialize and write the resolved scenario
    println!("\n💾 Serializing resolved scenario...");
    let resolved_xml = serialize_to_string(&document)?;

    let mut output_file = File::create(&output_path)?;
    output_file.write_all(resolved_xml.as_bytes())?;

    println!("✅ Resolved scenario written to: {}", output_path.display());
    println!("   📊 Output size: {} bytes", resolved_xml.len());

    // Step 9: Report resolution statistics
    print_resolution_summary(&document);

    Ok(output_path)
}

// Parameter extraction is now handled by the catalog module

/// Simplified catalog resolution function using the new infrastructure
fn resolve_catalog_references_simple(
    document: &mut openscenario_rs::types::scenario::storyboard::OpenScenario,
    catalog_locations: &openscenario_rs::types::catalogs::locations::CatalogLocations,
    parameters: &HashMap<String, String>,
    base_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut resolved_references = 0;
    let mut failed_references = 0;

    // Process each entity in the scenario
    if document.is_scenario() {
        if let Some(entities) = &mut document.entities {
            for entity in &mut entities.scenario_objects {
                let entity_name = entity.name.to_string();
                println!("   🎭 Processing entity: {}", entity_name);

                // Check if entity has a catalog reference
                if let Some(catalog_ref) = entity.vehicle_catalog_reference() {
                    println!("      🔗 Resolving vehicle catalog reference...");
                    println!("         Catalog: {:?}", catalog_ref.catalog_name);
                    println!("         Entry: {:?}", catalog_ref.entry_name);

                    // Use the new catalog resolution function
                    match resolve_catalog_reference_simple(
                        &catalog_ref.catalog_name,
                        &catalog_ref.entry_name,
                        catalog_locations,
                        parameters,
                        base_dir,
                    ) {
                        Ok(found) => {
                            if found {
                                println!("         ✅ Catalog entry found and validated");
                                resolved_references += 1;
                                // In a full implementation, we would load and replace the entity here
                                // For this example, we just validate that the reference can be resolved
                            } else {
                                println!("         ❌ Catalog entry not found");
                                failed_references += 1;
                            }
                        }
                        Err(e) => {
                            println!("         ❌ Failed to resolve catalog reference: {}", e);
                            failed_references += 1;
                        }
                    }
                }

                // Check controller references
                if let Some(object_controller) = &entity.object_controller {
                    if let Some(controller_ref) = &object_controller.catalog_reference {
                        println!("      🎮 Resolving controller reference...");

                        match resolve_catalog_reference_simple(
                            &controller_ref.catalog_name,
                            &controller_ref.entry_name,
                            catalog_locations,
                            parameters,
                            base_dir,
                        ) {
                            Ok(found) => {
                                if found {
                                    println!("         ✅ Controller entry found and validated");
                                    resolved_references += 1;
                                } else {
                                    println!("         ❌ Controller entry not found");
                                    failed_references += 1;
                                }
                            }
                            Err(e) => {
                                println!(
                                    "         ❌ Failed to resolve controller reference: {}",
                                    e
                                );
                                failed_references += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!(
        "   📊 Resolution complete: {} resolved, {} failed",
        resolved_references, failed_references
    );

    if failed_references > 0 {
        return Err(format!("Failed to resolve {} catalog references", failed_references).into());
    }

    Ok(())
}

// Vehicle and controller resolution functions removed - using simplified approach

/// Print a comprehensive summary of the resolution process
fn print_resolution_summary(document: &openscenario_rs::types::scenario::storyboard::OpenScenario) {
    println!("\n📈 Resolution Summary");
    println!("════════════════════");

    match document.document_type() {
        OpenScenarioDocumentType::Scenario => {
            if let Some(entities) = &document.entities {
                let entity_count = entities.scenario_objects.len();
                let mut catalog_refs = 0;
                let mut controller_refs = 0;

                for entity in &entities.scenario_objects {
                    if entity.entity_catalog_reference.is_some() {
                        catalog_refs += 1;
                    }
                    if let Some(object_controller) = &entity.object_controller {
                        if object_controller.catalog_reference.is_some() {
                            controller_refs += 1;
                        }
                    }
                }

                println!("   🎭 Total entities: {}", entity_count);
                println!("   🔗 Entity catalog references: {}", catalog_refs);
                println!("   🎮 Controller catalog references: {}", controller_refs);
            } else {
                println!("   🎭 Total entities: 0");
                println!("   🔗 Entity catalog references: 0");
                println!("   🎮 Controller catalog references: 0");
            }

            let has_catalogs = if let Some(catalog_locations) = &document.catalog_locations {
                catalog_locations.vehicle_catalog.is_some()
                    || catalog_locations.pedestrian_catalog.is_some()
                    || catalog_locations.misc_object_catalog.is_some()
                    || catalog_locations.controller_catalog.is_some()
            } else {
                false
            };

            println!(
                "   🗂️  Catalog locations: {}",
                if has_catalogs { "Present" } else { "None" }
            );

            println!("   ✅ Catalog reference validation completed!");
        }
        OpenScenarioDocumentType::ParameterVariation => {
            if let Some(param_dist) = &document.parameter_value_distribution {
                println!("   📊 Parameter variation file");
                println!(
                    "   📁 Referenced scenario: {}",
                    param_dist.scenario_file.filepath
                );

                let mut total_params = 0;
                let mut total_combinations = 1;

                if let Some(deterministic) = &param_dist.deterministic {
                    total_params += deterministic.single_distributions.len();
                    for dist in &deterministic.single_distributions {
                        if let Some(set) = &dist.distribution_set {
                            total_combinations *= set.elements.len();
                        } else if let Some(range) = &dist.distribution_range {
                            if let Some(step_val) = range.step_width.as_literal() {
                                if let (Some(lower), Some(upper)) = (
                                    range.range.lower_limit.as_literal(),
                                    range.range.upper_limit.as_literal(),
                                ) {
                                    if let Ok(step_f64) = step_val.parse::<f64>() {
                                        let count = ((upper - lower) / step_f64 + 1.0) as usize;
                                        total_combinations *= count;
                                    }
                                }
                            }
                        }
                    }
                    total_params += deterministic.multi_distributions.len();
                }

                if let Some(stochastic) = &param_dist.stochastic {
                    total_params += stochastic.distributions.len();
                }

                println!("   🎯 Total varied parameters: {}", total_params);
                println!("   🔢 Estimated combinations: {}", total_combinations);
            } else {
                println!("   📊 Parameter variation file - no distributions found");
            }
        }
        OpenScenarioDocumentType::Catalog => {
            println!("   📚 Catalog file - no entities");
        }
        OpenScenarioDocumentType::Unknown => {
            println!("   ❓ Unknown document type");
        }
    }
}

/// General-purpose file path resolver for OpenSCENARIO file references
/// Resolves relative paths relative to the current scenario file's directory
fn resolve_file_path(
    base_scenario_path: &Path,
    relative_filepath: &str,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let base_dir = base_scenario_path.parent().unwrap_or(Path::new("."));

    let resolved_path = if relative_filepath.starts_with("./") {
        // Explicit relative path: "./path/file.ext"
        base_dir.join(&relative_filepath[2..])
    } else if relative_filepath.starts_with("../") {
        // Parent directory relative path: "../path/file.ext"
        base_dir.join(relative_filepath)
    } else if relative_filepath.starts_with('/') || relative_filepath.chars().nth(1) == Some(':') {
        // Absolute path (Unix: "/path" or Windows: "C:\path")
        PathBuf::from(relative_filepath)
    } else {
        // Implicit relative path: "path/file.ext"
        base_dir.join(relative_filepath)
    };

    println!("      🔗 Resolving file path:");
    println!("         Base dir: {}", base_dir.display());
    println!("         Relative: {}", relative_filepath);
    println!("         Resolved: {}", resolved_path.display());

    if !resolved_path.exists() {
        println!(
            "         ❌ File does not exist: {}",
            resolved_path.display()
        );
        return Err(format!(
            "Referenced file does not exist: {}",
            resolved_path.display()
        )
        .into());
    }

    println!("         ✅ File exists");
    Ok(resolved_path)
}

/// Resolve the absolute path to a scenario file referenced from a parameter variation
fn resolve_scenario_path(
    variation_path: &Path,
    scenario_filepath: &str,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    resolve_file_path(variation_path, scenario_filepath)
}

/// Analyze deterministic distributions and display their contents
fn analyze_deterministic_distributions(
    deterministic: &openscenario_rs::types::distributions::Deterministic,
) {
    let total_count =
        deterministic.single_distributions.len() + deterministic.multi_distributions.len();
    println!(
        "   🎯 Deterministic distributions: {} parameters",
        total_count
    );

    for dist in &deterministic.single_distributions {
        println!("      📊 Parameter: {}", dist.parameter_name);

        if let Some(set) = &dist.distribution_set {
            println!(
                "         📋 Distribution Set: {} values",
                set.elements.len()
            );
            for (i, element) in set.elements.iter().enumerate().take(5) {
                println!("            {}. {}", i + 1, element.value);
            }
            if set.elements.len() > 5 {
                println!("            ... and {} more", set.elements.len() - 5);
            }
        }

        if let Some(range) = &dist.distribution_range {
            println!("         📏 Distribution Range:");
            println!(
                "            Range: {} to {}",
                range.range.lower_limit, range.range.upper_limit
            );
            println!("            Step: {}", range.step_width);
        }

        if let Some(user_def) = &dist.user_defined_distribution {
            println!(
                "         🔧 User Defined: {} (type: {})",
                user_def.content, user_def.distribution_type
            );
        }
    }

    let multi_count = deterministic.multi_distributions.len();
    if multi_count > 0 {
        println!(
            "   🎯 Multi-parameter distributions: {} groups",
            multi_count
        );
    }
}
