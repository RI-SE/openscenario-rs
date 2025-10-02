use openscenario_rs::types::conditions::entity::RelativeDistanceCondition;

fn main() {
    // Try to deserialize just the RelativeDistanceCondition element
    let xml = r#"<RelativeDistanceCondition entityRef="CutInVehicle" relativeDistanceType="longitudinal" value="30.0" freespace="true" rule="lessThan" coordinateSystem="entity" />"#;

    match quick_xml::de::from_str::<RelativeDistanceCondition>(xml) {
        Ok(condition) => {
            println!("✅ Successfully parsed RelativeDistanceCondition:");
            println!("  entityRef: {}", condition.entity_ref);
            println!("  value: {}", condition.value);
            println!("  freespace: {}", condition.freespace);
            println!(
                "  relativeDistanceType: {:?}",
                condition.relative_distance_type
            );
            println!("  rule: {:?}", condition.rule);
            println!("  coordinateSystem: {:?}", condition.coordinate_system);
        }
        Err(e) => {
            println!("❌ Failed to parse RelativeDistanceCondition: {}", e);
        }
    }
}
