use openscenario_rs::types::conditions::entity::RelativeDistanceCondition;

fn main() {
    // Try to deserialize RelativeDistanceCondition with parameter expression
    let xml = r#"<RelativeDistanceCondition entityRef="CutInVehicle" relativeDistanceType="longitudinal" value="$CutInVehicle_HeadwayDistanceTrigger_dx0_m" freespace="true" rule="lessThan" coordinateSystem="entity" />"#;

    match quick_xml::de::from_str::<RelativeDistanceCondition>(xml) {
        Ok(condition) => {
            println!("✅ Successfully parsed RelativeDistanceCondition with parameter:");
            println!("  entityRef: {}", condition.entity_ref);
            println!("  value: {}", condition.value);
            println!("  freespace: {}", condition.freespace);
            println!("  relativeDistanceType: {:?}", condition.relative_distance_type);
            println!("  rule: {:?}", condition.rule);
            println!("  coordinateSystem: {:?}", condition.coordinate_system);
        },
        Err(e) => {
            println!("❌ Failed to parse RelativeDistanceCondition with parameter: {}", e);
        }
    }
}