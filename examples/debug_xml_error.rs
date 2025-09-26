use openscenario_rs::parser::xml::parse_from_file;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <xosc_file>", args[0]);
        return;
    }

    let file_path = &args[1];
    println!("🚀 Debug parsing XML file: {}", file_path);

    // First, let's read the raw XML and see what we're dealing with
    match fs::read_to_string(file_path) {
        Ok(content) => {
            println!("✅ File read successfully, {} bytes", content.len());
            
            // Look for ParameterValueDistribution section
            if content.contains("ParameterValueDistribution") {
                println!("🎯 Found ParameterValueDistribution in XML");
                
                // Extract just the relevant section for debugging
                if let Some(start_pos) = content.find("<ParameterValueDistribution") {
                    if let Some(end_pos) = content.find("</ParameterValueDistribution>") {
                        let section = &content[start_pos..end_pos + "</ParameterValueDistribution>".len()];
                        println!("📋 ParameterValueDistribution section:");
                        println!("{}", section);
                    }
                }
            } else {
                println!("ℹ️  No ParameterValueDistribution found in file");
            }
        }
        Err(e) => {
            println!("❌ Failed to read file: {}", e);
            return;
        }
    }

    // Now try parsing with detailed error information
    match parse_from_file(file_path) {
        Ok(scenario) => {
            println!("✅ SUCCESS: Parsed OpenSCENARIO file!");
            println!("📄 Document type: {:?}", scenario.document_type());
        }
        Err(e) => {
            println!("❌ PARSING ERROR: {}", e);
            println!("📝 Error details: {:?}", e);
            
            // Try to give more context about where the error occurred
            let error_string = format!("{:?}", e);
            if error_string.contains("ParameterValueDistribution") {
                println!("🎯 Error appears to be in ParameterValueDistribution parsing");
            }
            if error_string.contains("Deterministic") {
                println!("🎯 Error appears to be in Deterministic distribution parsing");
            }
        }
    }
}