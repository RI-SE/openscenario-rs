
use openscenario_rs::parser::xml::parse_from_str;

fn main() {
    let file_path = "xosc/concrete_scenarios/alks_scenario_4_4_1_cut_in_no_collision_template.xosc";
    
    // Read the file content
    let content = match std::fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            println!("❌ Failed to read file: {}", e);
            return;
        }
    };
    
    // Try to parse with detailed error reporting
    println!("🔍 Attempting to parse file: {}", file_path);
    println!("📏 File size: {} bytes", content.len());
    
    match parse_from_str(&content) {
        Ok(scenario) => {
            println!("✅ Successfully parsed scenario");
            println!("📋 Description: {:?}", scenario.file_header.description);
        },
        Err(e) => {
            println!("❌ Failed to parse: {}", e);
            
            // Let's try to parse just up to the problematic part
            println!("\n🔧 Trying to isolate the issue...");
            
            // Find the RelativeDistanceCondition line
            let lines: Vec<&str> = content.lines().collect();
            for (i, line) in lines.iter().enumerate() {
                if line.contains("RelativeDistanceCondition") {
                    println!("🎯 Found RelativeDistanceCondition at line {}: {}", i + 1, line.trim());
                    
                    // Print context around this line
                    let start = if i >= 5 { i - 5 } else { 0 };
                    let end = if i + 5 < lines.len() { i + 5 } else { lines.len() - 1 };
                    
                    println!("\n📄 Context around the problematic line:");
                    for j in start..=end {
                        let marker = if j == i { ">>> " } else { "    " };
                        println!("{}{}: {}", marker, j + 1, lines[j].trim());
                    }
                    break;
                }
            }
        }
    }
}