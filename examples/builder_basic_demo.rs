//! Basic demonstration of the OpenSCENARIO-rs builder system
//!
//! This example shows the fundamental usage of the builder API for creating
//! simple OpenSCENARIO documents with type-safe construction.
//!
//! Run with: `cargo run --example builder_basic_demo --features builder`

#[cfg(feature = "builder")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use openscenario_rs::ScenarioBuilder;

    println!("🚗 OpenSCENARIO-rs Builder Basic Demo");
    println!("====================================");

    // Create a simple scenario with one vehicle
    println!("\n📋 Building basic scenario...");
    
    let scenario = ScenarioBuilder::new()
        .with_header("Basic Highway Scenario", "Builder Demo")
        .with_entities()
            .add_vehicle("ego")
                .car()
                .finish()
        .build()?;

    println!("✅ Basic scenario built successfully!");
    
    // Show scenario details
    println!("\n🔍 Scenario Details:");
    println!("- Description: {}", scenario.file_header.description.as_literal().unwrap_or("N/A"));
    println!("- Author: {}", scenario.file_header.author.as_literal().unwrap_or("N/A"));
    
    if let Some(entities) = &scenario.entities {
        println!("- Entities: {}", entities.scenario_objects.len());
    }

    // Serialize to XML
    println!("\n💾 Serializing to XML...");
    let xml_output = openscenario_rs::serialize_to_string(&scenario)?;
    
    // Show first few lines
    let lines: Vec<&str> = xml_output.lines().take(5).collect();
    println!("Generated XML (first 5 lines):");
    for (i, line) in lines.iter().enumerate() {
        println!("{:2}: {}", i + 1, line);
    }

    println!("\n🎉 Basic demo completed successfully!");

    Ok(())
}

#[cfg(not(feature = "builder"))]
fn main() {
    println!("This example requires the 'builder' feature.");
    println!("Run with: cargo run --example builder_basic_demo --features builder");
}