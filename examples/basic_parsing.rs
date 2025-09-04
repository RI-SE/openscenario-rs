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

// TODO: Implement basic parsing example (Week 3)
// TODO: use openscenario::{parse_file, Result};

fn main() -> Result<()> {
    // Parse a scenario file
    let scenario = parse_file("examples/simple_scenario.xosc")?;

    // Access file header information
    println!("Scenario: {}", scenario.file_header.description);
    println!("Author: {}", scenario.file_header.author);

    // Access entities
    println!("Entities:");
    for entity in &scenario.entities.scenario_objects {
        println!("  - {}", entity.name);
    }

    Ok(())
}

// TODO: Add error handling demonstration
// TODO: Show how to handle different error types
// TODO: Demonstrate graceful error recovery

// TODO: Add example OpenSCENARIO file
// TODO: Create examples/simple_scenario.xosc with minimal valid content

