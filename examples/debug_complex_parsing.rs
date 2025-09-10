use std::fs;

fn main() {
    let xml = fs::read_to_string("xosc/cut_in_101_exam.xosc")
        .expect("Failed to read cut_in_101_exam.xosc file");
    
    // Try parsing just the root element
    match quick_xml::de::from_str::<openscenario_rs::types::scenario::storyboard::OpenScenario>(&xml) {
        Ok(scenario) => {
            println!("✅ Success: parsed OpenSCENARIO with {} entities", 
                     scenario.entities.scenario_objects.len());
        }
        Err(e) => {
            println!("❌ Error parsing OpenSCENARIO: {:?}", e);
            
            // Try parsing smaller sections to isolate the issue
            println!("\n🔍 Debugging individual sections:");
            
            // Test just the Entities section
            if let Some(entities_start) = xml.find("<Entities>") {
                if let Some(entities_end) = xml.find("</Entities>") {
                    let entities_xml = &xml[entities_start..entities_end + 11];
                    match quick_xml::de::from_str::<openscenario_rs::types::entities::Entities>(entities_xml) {
                        Ok(entities) => println!("✅ Entities parsing: {} objects", entities.scenario_objects.len()),
                        Err(e) => println!("❌ Entities parsing error: {:?}", e),
                    }
                }
            }
        }
    }
}
