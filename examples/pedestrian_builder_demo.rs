//! Pedestrian Builder Demo
//!
//! This example demonstrates a full XSD-compliant pedestrian builder API
//! for creating pedestrians with different categories and specifications.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use openscenario_rs::types::enums::Role;
    use openscenario_rs::ScenarioBuilder;

    println!("Pedestrian Builder Demo");
    println!("============================\n");

    // Build scenario with multiple pedestrian types
    let scenario = ScenarioBuilder::new()
        .with_header("Pedestrian Crossing Scenario", "Demo Author")
        .with_entities()
        // Standard pedestrian
        .add_pedestrian("pedestrian1", |p| {
            println!("Adding standard pedestrian (75kg, 0.6x0.6x1.8m)");
            p.pedestrian()
                .with_mass(75.0)
                .with_role(Role::Civil)
                .finish()
        })
        // Wheelchair user
        .add_pedestrian("wheelchair1", |p| {
            println!("Adding wheelchair user (85kg, 0.8x1.0x1.4m)");
            p.wheelchair()
                .with_mass(85.0)
                .with_role(Role::Civil)
                .finish()
        })
        // Police officer
        .add_pedestrian("police_officer1", |p| {
            println!("Adding police officer (80kg, with 3D model)");
            p.pedestrian()
                .with_mass(80.0)
                .with_role(Role::Police)
                .with_model3d("./models/police_officer.glb")
                .finish()
        })
        // Dog (animal)
        .add_pedestrian("dog1", |p| {
            println!("Adding animal/dog (50kg, 0.5x0.5x0.8m)");
            p.animal()
                .with_mass(50.0)
                .with_model3d("./models/dog.glb")
                .finish()
        })
        // Custom tall person
        .add_pedestrian("tall_person1", |p| {
            println!("Adding custom tall person (90kg, 0.7x0.7x2.0m)");
            p.pedestrian()
                .with_mass(90.0)
                .with_dimensions(0.7, 0.7, 2.0)
                .with_role(Role::Civil)
                .finish()
        })
        .build()?;

    println!("\nScenario built successfully with 5 pedestrians!");

    // Serialize to XML
    let xml = quick_xml::se::to_string(&scenario)?;
    println!(
        "\nGenerated XML (first 500 chars):\n{}",
        &xml[..500.min(xml.len())]
    );

    // Save to file
    std::fs::write("pedestrian_demo.xosc", xml)?;
    println!("\nSaved to: pedestrian_demo.xosc");

    Ok(())
}
