//! Two types can parse a catalog document from its root: `CatalogFile`, the narrow
//! reader the conformance harness routes catalog inputs through, and `OpenScenario`,
//! the general document root whose `Catalog` branch (XSD `CatalogDefinition`, `:862`)
//! reads the same content. Both hold the content in the same `CatalogContent` type, so
//! nothing about the crate's shape forces them to agree — they agree only because both
//! read the same `<Catalog>` element the same way.
//!
//! This pins that agreement against every catalog-branch file in the corpus, so a
//! change to either reader that makes them diverge fails here instead of staying
//! invisible to the harness, which never runs `OpenScenario` over these files.

use openscenario_rs::{parse_catalog_from_file, parse_from_file};

/// Every corpus file whose root's first non-`FileHeader` child is `<Catalog>`,
/// found by walking `conformance/corpus` and reading each document's element
/// children in order.
const CATALOG_BRANCH_CORPUS_FILES: &[&str] = &[
    "conformance/corpus/OSC-ALKS-scenarios/logical_scenarios/concrete_scenarios/catalogs/controllers/controller_catalog.xosc",
    "conformance/corpus/OSC-ALKS-scenarios/logical_scenarios/concrete_scenarios/catalogs/misc_objects/misc_object_catalog.xosc",
    "conformance/corpus/OSC-ALKS-scenarios/logical_scenarios/concrete_scenarios/catalogs/pedestrians/pedestrian_catalog.xosc",
    "conformance/corpus/OSC-ALKS-scenarios/logical_scenarios/concrete_scenarios/catalogs/vehicles/vehicle_catalog.xosc",
    "conformance/corpus/OSC-NCAP-scenarios/OpenSCENARIO/NCAP/Catalogs/Environments/Environments.xosc",
    "conformance/corpus/OSC-NCAP-scenarios/OpenSCENARIO/NCAP/Catalogs/Maneuver/ManeuverCatalog.xosc",
    "conformance/corpus/OSC-NCAP-scenarios/OpenSCENARIO/NCAP/Catalogs/Pedestrians/Pedestrians.xosc",
    "conformance/corpus/OSC-NCAP-scenarios/OpenSCENARIO/NCAP/Catalogs/Routes/RouteCatalog.xosc",
    "conformance/corpus/OSC-NCAP-scenarios/OpenSCENARIO/NCAP/Catalogs/Trajectories/TrajectoryCatalog.xosc",
    "conformance/corpus/OSC-NCAP-scenarios/OpenSCENARIO/NCAP/Catalogs/Vehicles/Vehicles.xosc",
    "conformance/corpus/openscenario1-engine/engine/tests/data/Scenarios/Catalogs/Controllers/ControllerCatalog.xosc",
    "conformance/corpus/openscenario1-engine/engine/tests/data/Scenarios/Catalogs/Environment/EnvironmentCatalog.xosc",
    "conformance/corpus/openscenario1-engine/engine/tests/data/Scenarios/Catalogs/MiscObjects/MiscObjectCatalog.xosc",
    "conformance/corpus/openscenario1-engine/engine/tests/data/Scenarios/Catalogs/Pedestrians/PedestrianCatalog.xosc",
    "conformance/corpus/openscenario1-engine/engine/tests/data/Scenarios/Catalogs/Vehicles/VehicleCatalog.xosc",
    "conformance/corpus/openscenario1-engine/engine/tests/data/Scenarios/Catalogs/Vehicles/traffic_area_action_test_vehicle_catalog.xosc",
];

#[test]
fn catalog_file_and_the_root_agree_on_every_catalog_branch_corpus_file() {
    for path in CATALOG_BRANCH_CORPUS_FILES {
        let via_catalog_file = parse_catalog_from_file(path)
            .unwrap_or_else(|e| panic!("{path}: CatalogFile must parse: {e}"));

        let via_root = parse_from_file(path)
            .unwrap_or_else(|e| panic!("{path}: OpenScenario must parse: {e}"));
        assert!(
            via_root.is_catalog(),
            "{path}: the root must classify this file as a catalog document"
        );
        let root_catalog = via_root
            .catalog
            .as_ref()
            .unwrap_or_else(|| panic!("{path}: the root's Catalog branch must be populated"));

        // Same catalog name.
        assert_eq!(
            via_catalog_file.catalog.name, root_catalog.name,
            "{path}: catalog name disagrees between CatalogFile and the root"
        );

        // Same entries by kind and name. `entity_names()` walks every kind in a
        // fixed order, so a mismatch in either count or name inside any one kind
        // shows up here.
        assert_eq!(
            via_catalog_file.catalog.entity_names(),
            root_catalog.entity_names(),
            "{path}: catalog entries disagree between CatalogFile and the root"
        );

        // Full structural agreement: same type (`CatalogContent`) read from the
        // same element by two different root types.
        assert_eq!(
            &via_catalog_file.catalog, root_catalog,
            "{path}: CatalogContent disagrees between CatalogFile and the root"
        );

        // Identical bytes when each re-serializes its catalog content.
        let bytes_via_catalog_file = quick_xml::se::to_string(&via_catalog_file.catalog)
            .unwrap_or_else(|e| panic!("{path}: CatalogFile's catalog must serialize: {e}"));
        let bytes_via_root = quick_xml::se::to_string(root_catalog)
            .unwrap_or_else(|e| panic!("{path}: the root's catalog must serialize: {e}"));
        assert_eq!(
            bytes_via_catalog_file, bytes_via_root,
            "{path}: re-serialized catalog content disagrees between CatalogFile and the root"
        );
    }
}
