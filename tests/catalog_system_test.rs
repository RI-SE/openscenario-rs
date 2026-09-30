//! Catalog files through the crate's file-level entry points: serializing a `CatalogFile`
//! and parsing it back, and loading one from disk with `CatalogLoader`.

use openscenario_rs::catalog::CatalogLoader;

use openscenario_rs::parser::xml::{parse_catalog_from_str, serialize_catalog_to_string};
use openscenario_rs::types::basic::Value;
use openscenario_rs::types::catalogs::files::CatalogFile;
use std::fs;

use tempfile::TempDir;

#[test]
fn test_catalog_serialization_roundtrip() {
    // Create a catalog programmatically
    let catalog = CatalogFile::new(
        "TestCatalog".to_string(),
        "IntegrationTest".to_string(),
        "Test catalog for serialization".to_string(),
    );

    // Serialize to XML
    let xml = serialize_catalog_to_string(&catalog).unwrap();

    // Verify XML structure
    assert!(xml.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
    assert!(xml.contains("OpenSCENARIO"));
    assert!(xml.contains("TestCatalog"));
    assert!(xml.contains("IntegrationTest"));

    // Parse it back
    let parsed_catalog = parse_catalog_from_str(&xml).unwrap();
    assert_eq!(
        parsed_catalog.catalog_name().as_literal().unwrap(),
        "TestCatalog"
    );
    assert_eq!(
        parsed_catalog.file_header.author.as_literal().unwrap(),
        "IntegrationTest"
    );
}

#[test]
fn test_catalog_loader_with_temporary_files() {
    // Create a temporary directory with a test catalog file
    let temp_dir = TempDir::new().unwrap();
    let catalog_path = temp_dir.path().join("test_catalog.xosc");

    let catalog_xml = r#"<?xml version="1.0"?>
    <OpenSCENARIO>
        <FileHeader author="TempTest" date="2024-01-01T00:00:00" description="Temporary Test Catalog" revMajor="1" revMinor="3"/>
        <Catalog name="TempTestCatalog">
            <Vehicle name="TempVehicle" vehicleCategory="car">
                <BoundingBox>
                    <Center x="1.0" y="0.0" z="0.8"/>
                    <Dimensions width="1.8" length="4.0" height="1.6"/>
                </BoundingBox>
                <Performance maxSpeed="40" maxAcceleration="4" maxDeceleration="6"/>
                <Axles>
                    <FrontAxle maxSteering="0.4" wheelDiameter="0.55" trackWidth="1.6" positionX="2.5" positionZ="0.25"/>
                    <RearAxle maxSteering="0.0" wheelDiameter="0.55" trackWidth="1.6" positionX="0.0" positionZ="0.25"/>
                </Axles>
            </Vehicle>
        </Catalog>
    </OpenSCENARIO>"#;

    // Write the test catalog file
    fs::write(&catalog_path, catalog_xml).unwrap();

    // Test loading with CatalogLoader
    let loader = CatalogLoader::new();
    let catalog = loader.load_and_parse_catalog_file(&catalog_path).unwrap();

    assert_eq!(
        catalog.catalog_name().as_literal().unwrap(),
        "TempTestCatalog"
    );
    assert_eq!(catalog.file_header.author.as_literal().unwrap(), "TempTest");
    assert_eq!(catalog.vehicles().len(), 1);
    let vehicle = &catalog.vehicles()[0];
    assert_eq!(vehicle.name.as_literal().unwrap(), "TempVehicle");
    assert_eq!(
        vehicle.vehicle_category,
        Value::Literal(openscenario_rs::types::enums::VehicleCategory::Car)
    );
}
