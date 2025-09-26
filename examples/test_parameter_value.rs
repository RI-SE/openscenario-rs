use openscenario_rs::types::basic::Double;

fn main() {
    // Test parameter expression parsing with the exact value from the XML
    let xml = r#"<test value="$CutInVehicle_HeadwayDistanceTrigger_dx0_m" />"#;
    
    #[derive(serde::Deserialize, Debug)]
    struct Test {
        #[serde(rename = "@value")]
        value: Double,
    }

    match quick_xml::de::from_str::<Test>(xml) {
        Ok(test) => {
            println!("✅ Successfully parsed parameter value:");
            println!("  value: {}", test.value);
            match test.value {
                openscenario_rs::types::basic::Value::Parameter(ref param) => {
                    println!("  parsed as parameter: {}", param);
                },
                openscenario_rs::types::basic::Value::Expression(ref expr) => {
                    println!("  parsed as expression: {}", expr);
                },
                openscenario_rs::types::basic::Value::Literal(ref lit) => {
                    println!("  parsed as literal: {}", lit);
                },
            }
        },
        Err(e) => {
            println!("❌ Failed to parse parameter value: {}", e);
        }
    }
}