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
        Ok(()) => {
            println!("✅ SUCCESS: Resolved scenario written to: ");
        }
        Err(e) => {
            eprintln!("❌ ERROR: {}", e);
            std::process::exit(1);
        }
    }
}
fn process_scenario(input_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 OpenSCENARIO Universal Parser Tool");
    println!("═════════════════════════════════════");

    // Step 1: Load and parse the input scenario
    println!("\n📁 Loading scenario file: {}", input_file);
    let input_path = Path::new(input_file);
    if !input_path.exists() {
        return Err(format!("Input file does not exist: {}", input_file).into());
    }
    let output_dir = Path::new("output");
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
        println!("\n📁 Created output directory: {}", output_dir.display());
    }

    let mut document = parse_from_file(input_path)?;
    let resolved = serialize_to_string(&document)?;
    let input_stem = input_path
        .file_stem()
        .unwrap_or_else(|| std::ffi::OsStr::new("scenario"));
    let output_filename = format!("{}_resolved_ss.xosc", input_stem.to_string_lossy());
    let output_path = output_dir.join(output_filename);

    let mut output_file = File::create(&output_path)?;
    output_file.write_all(resolved.as_bytes())?;

    Ok(())
}
