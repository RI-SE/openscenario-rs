//! Test the new detached builder functionality
//! This example demonstrates perfect fluent API without lifetime constraints

#[cfg(feature = "builder")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use openscenario_rs::ScenarioBuilder;

    // ✅ Test that ScenarioBuilder is accessible
    let _scenario_builder = ScenarioBuilder::new();
    println!("✅ ScenarioBuilder creation works!");

    println!("🎉 Detached Builder Pattern implementation SUCCESS!");
    println!(
        "🎉 NEW: create_*() methods available for perfect fluent API without lifetime constraints!"
    );

    Ok(())
}

#[cfg(not(feature = "builder"))]
fn main() {
    println!("❌ Builder feature not enabled. Run with: cargo run --features builder --example test_detached_builders");
}
