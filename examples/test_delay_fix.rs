//! Test to verify the delay attribute fix for Condition elements

#[cfg(feature = "builder")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use openscenario_rs::builder::conditions::TimeConditionBuilder;
    use quick_xml::se::to_string;
    
    println!("🔧 Testing delay attribute fix for Condition elements...");
    
    // Create a time condition using the builder
    let condition = TimeConditionBuilder::new()
        .at_time(120.0)
        .build()?;
    
    // Verify delay is present in the struct
    if let Some(delay) = &condition.delay {
        println!("✅ Delay attribute is present in struct: {:?}", delay);
    } else {
        println!("❌ Delay attribute is missing from struct!");
        return Err("Delay attribute missing from struct".into());
    }
    
    // Serialize to XML and check if delay appears
    let xml = to_string(&condition)?;
    println!("📄 Generated XML:");
    println!("{}", xml);
    
    // Check if delay="0" appears in the XML
    if xml.contains("delay=\"0\"") {
        println!("✅ Delay attribute appears correctly in XML output");
    } else {
        println!("❌ Delay attribute missing from XML output!");
        println!("Expected to find: delay=\"0\"");
        return Err("Delay attribute missing from XML".into());
    }
    
    println!("🎉 All tests passed! The delay attribute fix is working correctly.");
    Ok(())
}

#[cfg(not(feature = "builder"))]
fn main() {
    println!("Builder feature not enabled. Run with:");
    println!("cargo run --example test_delay_fix --features builder");
}