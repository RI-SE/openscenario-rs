use openscenario_rs::types::conditions::entity::EntityCondition;

fn main() {
    // Try to deserialize an EntityCondition containing RelativeDistanceCondition
    let xml = r#"<RelativeDistanceCondition entityRef="CutInVehicle" relativeDistanceType="longitudinal" value="30.0" freespace="true" rule="lessThan" coordinateSystem="entity" />"#;

    match quick_xml::de::from_str::<EntityCondition>(xml) {
        Ok(condition) => {
            println!("✅ Successfully parsed EntityCondition:");
            println!("  condition: {:?}", condition);
        }
        Err(e) => {
            println!("❌ Failed to parse EntityCondition: {}", e);
        }
    }
}
