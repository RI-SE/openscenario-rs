//! Comprehensive routing demo
//!
//! This example demonstrates:
//! - Creating routes with multiple waypoints
//! - Different waypoint position types (world, lane, relative)
//! - Route distance calculations and analytics
//! - Route validation and continuity checking
//! - XML serialization and deserialization

use openscenario_rs::types::{Route, RouteRef, RouteStrategy, Waypoint};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚗 OpenSCENARIO-rs Routing Demo");
    println!("{}", "=".repeat(60));

    // Demo 1: Create a simple point-to-point route
    demo_simple_route()?;

    // Demo 2: Create a complex multi-waypoint route
    demo_complex_route()?;

    // Demo 3: Demonstrate route analytics
    demo_route_analytics()?;

    // Demo 4: Show route references
    demo_route_references()?;

    // Demo 5: Demonstrate route validation
    demo_route_validation()?;

    // Demo 6: Show XML serialization
    demo_xml_serialization()?;

    println!("\n✅ All routing demos completed successfully!");
    Ok(())
}

fn demo_simple_route() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📍 Demo 1: Simple Point-to-Point Route");
    println!("{}", "-".repeat(40));

    // Create a simple route from origin to destination
    let route = Route::new(
        "SimpleRoute",
        false,
        vec![
            Waypoint::world_position(0.0, 0.0, 0.0, RouteStrategy::Shortest),
            Waypoint::world_position(1000.0, 500.0, 0.0, RouteStrategy::Fastest),
        ],
    )
    .unwrap();

    println!("Route name: {}", route.name.resolve(&HashMap::new())?);
    println!("Waypoint count: {}", route.waypoint_count());
    println!("Is closed: {}", route.is_closed()?);
    println!("Total distance: {:.2} meters", route.total_distance()?);

    Ok(())
}

fn demo_complex_route() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🗺️  Demo 2: Complex Multi-Waypoint Route");
    println!("{}", "-".repeat(40));

    // Create a complex route with different position types
    let route = Route::new(
        "ComplexRoute",
        true,
        vec![
            Waypoint::world_position(0.0, 0.0, 0.0, RouteStrategy::Shortest),
            Waypoint::lane_position("highway_1", "lane_2", 500.0, RouteStrategy::Fastest),
            Waypoint::relative_world_position(
                "lead_vehicle",
                50.0,
                0.0,
                0.0,
                RouteStrategy::LeastIntersections,
            ),
            Waypoint::world_position(2000.0, 1000.0, 0.0, RouteStrategy::Random),
        ],
    )
    .unwrap();

    println!("Route name: {}", route.name.resolve(&HashMap::new())?);
    println!("Waypoint count: {}", route.waypoint_count());
    println!("Is closed: {}", route.is_closed()?);

    // Show waypoint details
    for (i, waypoint) in route.waypoints.iter().enumerate() {
        println!(
            "  Waypoint {}: {:?} strategy",
            i + 1,
            waypoint.route_strategy
        );
        if waypoint.position.world_position().is_some() {
            println!("    Position type: World");
        } else if waypoint.position.lane_position().is_some() {
            println!("    Position type: Lane");
        } else if waypoint.position.relative_world_position().is_some() {
            println!("    Position type: Relative World");
        }
    }

    Ok(())
}

fn demo_route_analytics() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📊 Demo 3: Route Analytics");
    println!("{}", "-".repeat(40));

    // Create a route for analytics demonstration
    let route = Route::new(
        "AnalyticsRoute",
        false,
        vec![
            Waypoint::world_position(0.0, 0.0, 0.0, RouteStrategy::Shortest),
            Waypoint::world_position(300.0, 400.0, 0.0, RouteStrategy::Fastest),
            Waypoint::world_position(600.0, 0.0, 0.0, RouteStrategy::LeastIntersections),
        ],
    )
    .unwrap();

    println!(
        "Route Analytics for '{}':",
        route.name.resolve(&HashMap::new())?
    );

    // Calculate total distance
    let total_distance = route.total_distance()?;
    println!("  Total distance: {:.2} meters", total_distance);

    // Calculate segment distances
    let segment_distances = route.segment_distances()?;
    println!("  Segment distances:");
    for (i, distance) in segment_distances.iter().enumerate() {
        println!("    Segment {}: {:.2} meters", i + 1, distance);
    }

    Ok(())
}

fn demo_route_references() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔗 Demo 4: Route References");
    println!("{}", "-".repeat(40));

    // Create a route for reference demonstration
    let route = Route::new(
        "ReferenceRoute",
        false,
        vec![
            Waypoint::world_position(0.0, 0.0, 0.0, RouteStrategy::Shortest),
            Waypoint::world_position(1500.0, 800.0, 0.0, RouteStrategy::Fastest),
        ],
    )
    .unwrap();

    // Create direct route reference
    let direct_ref = RouteRef::direct(route.clone());
    println!("Created direct route reference");

    // Create catalog route reference
    let catalog_ref = RouteRef::catalog("RouteCatalog", "HighwayRoute");
    println!("Created catalog route reference");

    // Show route reference types
    match &direct_ref {
        RouteRef::Direct(r) => println!(
            "Direct reference to route: {}",
            r.name.resolve(&HashMap::new())?
        ),
        RouteRef::Catalog(c) => println!(
            "Catalog reference: {}/{}",
            c.catalog_name.resolve(&HashMap::new())?,
            c.entry_name.resolve(&HashMap::new())?
        ),
    }

    match &catalog_ref {
        RouteRef::Direct(r) => println!(
            "Direct reference to route: {}",
            r.name.resolve(&HashMap::new())?
        ),
        RouteRef::Catalog(c) => println!(
            "Catalog reference: {}/{}",
            c.catalog_name.resolve(&HashMap::new())?,
            c.entry_name.resolve(&HashMap::new())?
        ),
    }

    Ok(())
}

fn demo_route_validation() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n✅ Demo 5: Route Validation");
    println!("{}", "-".repeat(40));

    // XSD `Route` declares `Waypoint` with `minOccurs="2"`, and `Route::waypoints` is a
    // `MinVec<Waypoint, 2>`, so the check happens when the route is constructed. There is
    // no separate validation step to call and no way to hold a route that would fail it.
    let valid_route = Route::new(
        "ValidRoute",
        false,
        vec![
            Waypoint::world_position(0.0, 0.0, 0.0, RouteStrategy::Shortest),
            Waypoint::world_position(100.0, 100.0, 0.0, RouteStrategy::Fastest),
        ],
    );
    match valid_route {
        Ok(route) => println!(
            "✅ Two waypoints accepted: {} total",
            route.waypoint_count()
        ),
        Err(e) => println!("❌ Two waypoints rejected: {}", e),
    }

    match Route::new("EmptyRoute", false, vec![]) {
        Ok(_) => println!("❌ Empty route should have been refused"),
        Err(e) => println!("✅ Empty route refused at construction: {}", e),
    }

    match Route::new(
        "SingleRoute",
        false,
        vec![Waypoint::world_position(
            0.0,
            0.0,
            0.0,
            RouteStrategy::Shortest,
        )],
    ) {
        Ok(_) => println!("❌ Single-waypoint route should have been refused"),
        Err(e) => println!("✅ Single-waypoint route refused at construction: {}", e),
    }

    Ok(())
}

fn demo_xml_serialization() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📄 Demo 6: XML Serialization");
    println!("{}", "-".repeat(40));

    // Create a route for serialization
    let route = Route::new(
        "SerializationDemo",
        false,
        vec![
            Waypoint::world_position(0.0, 0.0, 0.0, RouteStrategy::Shortest),
            Waypoint::lane_position("road_1", "lane_1", 250.0, RouteStrategy::Fastest),
            Waypoint::world_position(500.0, 300.0, 0.0, RouteStrategy::LeastIntersections),
        ],
    )
    .unwrap();

    // Serialize to XML
    let xml = quick_xml::se::to_string(&route)?;
    println!("Serialized route to XML:");
    println!("{}", xml);

    // Deserialize back from XML
    let deserialized: Route = quick_xml::de::from_str(&xml)?;
    println!("\nDeserialized route:");
    println!("  Name: {}", deserialized.name.resolve(&HashMap::new())?);
    println!("  Waypoints: {}", deserialized.waypoint_count());
    println!("  Closed: {}", deserialized.is_closed()?);

    // Verify roundtrip integrity
    if route.name.resolve(&HashMap::new())? == deserialized.name.resolve(&HashMap::new())?
        && route.waypoint_count() == deserialized.waypoint_count()
    {
        println!("✅ XML serialization roundtrip successful");
    } else {
        println!("❌ XML serialization roundtrip failed");
    }

    Ok(())
}
