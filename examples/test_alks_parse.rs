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
                    let total_count = deterministic.single_distributions.len() + deterministic.multi_distributions.len();
                    println!("🔢 Deterministic distributions: {} total", total_count);
                    
                    // Process single parameter distributions
                    for (i, single) in deterministic.single_distributions.iter().enumerate() {
                        println!("  {}. Single Parameter: {}", i+1, single.parameter_name);
                    }
                    
                    // Process multi parameter distributions
                    for (i, multi) in deterministic.multi_distributions.iter().enumerate() {
                        println!("  {}. Multi Parameter Distribution", deterministic.single_distributions.len() + i + 1);
                        match &multi.distribution_type {
                            openscenario_rs::types::distributions::DeterministicMultiParameterDistributionType::ValueSetDistribution(dist) => {
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