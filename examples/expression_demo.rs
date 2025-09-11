//! Expression system demonstration for OpenSCENARIO-rs
//!
//! This example shows how the enhanced Value<T> system handles:
//! - Literal values
//! - Parameter references (${param})
//! - Mathematical expressions (${expr})
//!
//! Contributes to project by:
//! - Demonstrating the complete expression system functionality
//! - Showing real-world usage patterns for parameter and expression handling
//! - Supporting integration testing of expression resolution workflows
//! - Providing usage examples for developers

use openscenario_rs::types::basic::Double;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== OpenSCENARIO-rs Expression System Demo ===\n");

    // Create a parameter context
    let mut params = std::collections::HashMap::new();
    params.insert("vehicle_speed".to_string(), "30.0".to_string());
    params.insert("target_speed".to_string(), "60.0".to_string());
    params.insert("acceleration".to_string(), "2.5".to_string());

    println!("Parameter Context:");
    for (key, value) in &params {
        println!("  {} = {}", key, value);
    }
    println!();

    // Demonstrate different Value types
    let literal_value = Double::literal(25.0);
    let parameter_value = Double::parameter("vehicle_speed".to_string());
    let expression_value = Double::expression("vehicle_speed + 10".to_string());

    println!("Value Types:");
    println!("  Literal: {:?}", literal_value);
    println!("  Parameter: {:?}", parameter_value);
    println!("  Expression: {:?}", expression_value);
    println!();

    // Resolve values
    println!("Resolved Values:");
    println!("  Literal: {}", literal_value.resolve(&params)?);
    println!("  Parameter: {}", parameter_value.resolve(&params)?);
    // For expression, we'll just show the expression string since full evaluation isn't implemented yet
    if let Some(expr) = expression_value.as_expression() {
        println!("  Expression: {}", expr);
    }
    println!();

    // Show serialization
    println!("Serialized Values:");
    println!("  Literal: {}", serde_json::to_string(&literal_value)?);
    println!("  Parameter: {}", serde_json::to_string(&parameter_value)?);
    println!(
        "  Expression: {}",
        serde_json::to_string(&expression_value)?
    );
    println!();

    println!("✅ Expression system demo completed successfully!");

    Ok(())
}

