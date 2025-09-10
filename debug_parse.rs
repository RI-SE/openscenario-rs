use openscenario_rs::parse_file;

fn main() {
    let file_path = "xosc/cut_in_0_exam.xosc";
    match parse_file(file_path) {
        Ok(scenario) => {
            println!("✅ Successfully parsed {}!", file_path);
            println!("Scenario name: {}", scenario.file_header.name.as_literal().unwrap_or(&"Unknown".to_string()));
        }
        Err(e) => {
            println!("❌ Failed to parse {}: {:?}", file_path, e);
        }
    }
}