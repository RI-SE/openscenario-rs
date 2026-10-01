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
    parse_file_resolved,
    parser::xml::{parse_from_file, serialize_to_string},
    types::{basic::Value, OpenScenarioDocumentType},
};
use std::{
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
        eprintln!("Example: cargo run --example parse -- tests/data/simple_scenario.xosc");
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
            if !literal_path.starts_with('/') && !(literal_path.chars().nth(1) == Some(':')) {
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
        if !scenario_filepath.starts_with('/') && !(scenario_filepath.chars().nth(1) == Some(':')) {
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

    // `parse_file_resolved` resolves every `$name`/`${expression}` against the document's own
    // `<ParameterDeclarations>` (section 9.1 scoping) and inlines every `<CatalogReference>`
    // with its own `<ParameterAssignments>` (section 9.5/9.6), so the document returned here
    // already carries literal values and resolved entities.
    let mut document = parse_file_resolved(input_path)?;

    println!("✅ Successfully parsed and resolved scenario file");
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

            // Step 2: Report the parameters declared on the document. `parse_file_resolved`
            // already replaced every use with its literal value, following the section 9.1
            // scope; the declarations below still show the values that were used.
            if let Some(param_decls) = &document.parameter_declarations {
                println!(
                    "   ⚙️  Parameters: {}",
                    param_decls.parameter_declarations.len()
                );
                for param in &param_decls.parameter_declarations {
                    println!("      - {} = {}", param.name, param.value);
                }
            } else {
                println!("   ⚙️  Parameters: 0");
            }

            // Step 3: Report catalog usage. `parse_file_resolved` has already located each
            // `<CatalogReference>` by catalog name and entry name, in the scope of its own
            // `<ParameterAssignments>`, and inlined the resolved entry in its place (section
            // 9.5/9.6), so there is nothing left to resolve here.
            let has_catalogs = document
                .catalog_locations
                .as_ref()
                .is_some_and(|locations| {
                    locations.vehicle_catalog.is_some()
                        || locations.pedestrian_catalog.is_some()
                        || locations.misc_object_catalog.is_some()
                        || locations.controller_catalog.is_some()
                });
            if has_catalogs {
                println!("\n🗂️  Catalog locations found - references resolved during parsing");
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
                if let Some(deterministic) = param_dist.as_deterministic() {
                    analyze_deterministic_distributions(deterministic);
                }

                if let Some(stochastic) = param_dist.as_stochastic() {
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

    // Step 5: `parse_file_resolved` evaluated every `${expression}` against the document's own
    // parameters as it parsed, so no separate expression pass is needed here.
    println!("\n🧮 Expression Resolution");
    println!("═══════════════════════");
    println!("💡 Expressions were resolved during parsing");

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
                    if entity.catalog_reference().is_some() {
                        catalog_refs += 1;
                    }
                    for object_controller in &entity.object_controller {
                        if object_controller.catalog_reference().is_some() {
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

                if let Some(deterministic) = param_dist.as_deterministic() {
                    use openscenario_rs::types::distributions::deterministic::DeterministicSingleParameterDistributionType;

                    let single_count = deterministic.single_distributions().count();
                    total_params += single_count;
                    for dist in deterministic.single_distributions() {
                        match &dist.distribution {
                            DeterministicSingleParameterDistributionType::DistributionSet(set) => {
                                total_combinations *= set.elements.len();
                            }
                            DeterministicSingleParameterDistributionType::DistributionRange(
                                range,
                            ) => {
                                if let Some(step) = range.step_width.as_literal() {
                                    if let (Some(lower), Some(upper)) = (
                                        range.range.lower_limit.as_literal(),
                                        range.range.upper_limit.as_literal(),
                                    ) {
                                        let count = ((upper - lower) / step + 1.0) as usize;
                                        total_combinations *= count;
                                    }
                                }
                            }
                            DeterministicSingleParameterDistributionType::UserDefinedDistribution(
                                _,
                            ) => {}
                        }
                    }
                    total_params += deterministic.multi_distributions().count();
                }

                if let Some(stochastic) = param_dist.as_stochastic() {
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

    let resolved_path = if let Some(stripped) = relative_filepath.strip_prefix("./") {
        // Explicit relative path: "./path/file.ext"
        base_dir.join(stripped)
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
    use openscenario_rs::types::distributions::deterministic::DeterministicSingleParameterDistributionType;

    let total_count = deterministic.total_count();
    println!(
        "   🎯 Deterministic distributions: {} parameters",
        total_count
    );

    for dist in deterministic.single_distributions() {
        println!("      📊 Parameter: {}", dist.parameter_name);

        match &dist.distribution {
            DeterministicSingleParameterDistributionType::DistributionSet(set) => {
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
            DeterministicSingleParameterDistributionType::DistributionRange(range) => {
                println!("         📏 Distribution Range:");
                println!(
                    "            Range: {} to {}",
                    range.range.lower_limit, range.range.upper_limit
                );
                println!("            Step: {}", range.step_width);
            }
            DeterministicSingleParameterDistributionType::UserDefinedDistribution(user_def) => {
                println!(
                    "         🔧 User Defined: {} (type: {})",
                    user_def.content, user_def.distribution_type
                );
            }
        }
    }

    let multi_count = deterministic.multi_distributions().count();
    if multi_count > 0 {
        println!(
            "   🎯 Multi-parameter distributions: {} groups",
            multi_count
        );
    }
}
