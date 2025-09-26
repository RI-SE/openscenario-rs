use openscenario_rs::parser::xml::parse_from_file;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <xosc_file>", args[0]);
        return;
    }

    let file_path = &args[1];
    println!("🚀 Testing ALKS scenario parsing: {}", file_path);

    match parse_from_file(file_path) {
        Ok(scenario) => {
            println!("✅ SUCCESS: Parsed OpenSCENARIO file!");
            println!("📄 Document type: {:?}", scenario.document_type());
            
            if let Some(param_dist) = &scenario.parameter_value_distribution {
                println!("🎯 Parameter Value Distribution found!");
                
                if let Some(deterministic) = &param_dist.deterministic {
                    println!("🔢 Deterministic distributions: {} total", deterministic.total_count());
                    
                    for (i, dist) in deterministic.all_distributions().enumerate() {
                        match dist {
                            openscenario_rs::types::distributions::DeterministicParameterDistribution::Single(single) => {
                                println!("  {}. Single Parameter: {}", i+1, single.parameter_name);
                            },
                            openscenario_rs::types::distributions::DeterministicParameterDistribution::Multi(multi) => {
                                println!("  {}. Multi Parameter Distribution", i+1);
                                let dist = &multi.distribution_type;
                                println!("     📋 Value sets: {}", dist.parameter_value_sets.len());
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ ERROR: {}", e);
        }
    }
}