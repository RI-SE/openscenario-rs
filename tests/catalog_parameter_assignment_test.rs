//! Catalog references assigning values to the parameters of the entries they name.
//!
//! ASAM OpenSCENARIO XML 1.3 section 9.5 states the rule: a catalog entry declares its
//! parameters with default values, and "the ParameterAssignment element within CatalogReference
//! may be used to override these defaults". For one use of the entry, an assigned parameter takes
//! the assigned value and every other parameter keeps its default. The entry sees nothing else:
//! "No other parameters may be referenced from within the catalog." An assignment's own value
//! "may itself reference a parameter"; since the assignment sits in the referencing document,
//! section 9.1's subtree rule resolves that reference in the document's scope. The model
//! reference for `ParameterAssignment` requires `parameterRef` to name a parameter "that must
//! be declared in the catalog".
//!
//! The first group of tests resolves whole documents through `parse_file_resolved`, which
//! replaces each reference by the entry it names. The second group resolves single references
//! through `CatalogManager`. Each claim is its own test, so that one failing claim cannot hide
//! the next.

use openscenario_rs::catalog::CatalogManager;
use openscenario_rs::parser::resolve::resolve_parameters;
use openscenario_rs::types::basic::{Double, Value};
use openscenario_rs::types::catalogs::locations::{
    ControllerCatalogLocation, PedestrianCatalogLocation, VehicleCatalogLocation,
};
use openscenario_rs::types::catalogs::references::{
    ControllerCatalogReference, ParameterAssignment, PedestrianCatalogReference,
    VehicleCatalogReference,
};
use openscenario_rs::types::entities::vehicle::Vehicle;
use openscenario_rs::types::entities::EntityObjectChoice;
use openscenario_rs::{parse_file, parse_file_resolved, parse_str_resolved, OpenScenario};
use std::path::{Path, PathBuf};

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/catalog_parameter_assignments")
}

fn resolved_fixture() -> OpenScenario {
    parse_file_resolved(fixture_dir().join("scenario.xosc")).expect("fixture resolves")
}

/// Write `xml` as a scenario file of its own and resolve it. The catalog directories are made
/// absolute, so the variant finds the fixture catalogs wherever it is written.
fn resolve_variant(name: &str, xml: &str) -> openscenario_rs::Result<OpenScenario> {
    let catalogs = fixture_dir().join("catalogs");
    let xml = xml.replace(
        r#"path="catalogs/"#,
        &format!(r#"path="{}/"#, catalogs.display()),
    );
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("catalog_parameter_assignment");
    std::fs::create_dir_all(&dir).expect("temporary directory");
    let path = dir.join(format!("{name}.xosc"));
    std::fs::write(&path, xml).expect("variant written");
    parse_file_resolved(&path)
}

/// The fixture with the `Assigned` object's reference replaced by `reference`.
fn with_reference(reference: &str) -> String {
    let fixture = std::fs::read_to_string(fixture_dir().join("scenario.xosc")).unwrap();
    let start = fixture.find(r#"<ScenarioObject name="Assigned">"#).unwrap();
    let end = start + fixture[start..].find("</ScenarioObject>").unwrap();
    format!(
        "{}<ScenarioObject name=\"Assigned\">{}{}",
        &fixture[..start],
        reference,
        &fixture[end..]
    )
}

fn vehicle<'a>(document: &'a OpenScenario, object: &str) -> &'a Vehicle {
    let entities = document.entities.as_ref().expect("entities present");
    let scenario_object = entities
        .scenario_objects
        .iter()
        .find(|o| o.name.as_literal().map(String::as_str) == Some(object))
        .expect("object present");
    match &scenario_object.entity {
        EntityObjectChoice::Vehicle(vehicle) => vehicle,
        other => panic!("{object} is not an inline Vehicle after resolution: {other:?}"),
    }
}

fn literal(value: &Double) -> f64 {
    *value.as_literal().expect("resolved to a literal")
}

// --- Whole documents ---------------------------------------------------------------------

#[test]
fn an_assignment_overrides_the_entry_default() {
    let document = resolved_fixture();
    assert_eq!(
        literal(&vehicle(&document, "Assigned").performance.max_speed),
        55.0
    );
}

#[test]
fn an_unassigned_parameter_takes_the_entry_default() {
    let document = resolved_fixture();
    let defaults = vehicle(&document, "Defaults");
    assert_eq!(literal(&defaults.performance.max_speed), 30.0);
    assert_eq!(
        literal(defaults.mass.as_ref().expect("mass present")),
        1500.0
    );
}

/// `half_speed` defaults to `${$top_speed / 2}`; with `top_speed` assigned 55, the default
/// is computed from the assigned value.
#[test]
fn an_entry_default_referencing_an_assigned_parameter_sees_the_assignment() {
    let document = resolved_fixture();
    assert_eq!(
        literal(&vehicle(&document, "Assigned").performance.max_acceleration),
        27.5
    );
}

/// The document declares `top_speed` as 70 and the entry declares it with the default 30.
/// The assignment `top_speed = $top_speed` sits in the document, so its value is the
/// document's 70.
#[test]
fn an_assignment_value_resolves_in_the_referencing_scope() {
    let document = resolved_fixture();
    assert_eq!(
        literal(&vehicle(&document, "FromDocument").performance.max_speed),
        70.0
    );
}

/// The maneuver reference sits inside a `Story` that redeclares `top_speed` as 12, so the
/// assignment sees the story's declaration, not the global one.
#[test]
fn an_assignment_value_sees_the_innermost_declaration_around_the_reference() {
    let document = resolved_fixture();
    let xml = quick_xml::se::to_string(&document).expect("document serializes");
    let speeds: Vec<&str> = xml
        .split("<AbsoluteTargetSpeed value=\"")
        .skip(1)
        .map(|rest| rest.split('"').next().unwrap())
        .collect();
    assert_eq!(speeds, vec!["12"]);
}

#[test]
fn a_resolved_document_keeps_the_entry_declarations_with_the_values_used() {
    let document = resolved_fixture();
    let declarations = vehicle(&document, "Assigned")
        .parameter_declarations
        .as_ref()
        .expect("the inlined entry keeps its declarations");
    let values: Vec<(String, String)> = declarations
        .parameter_declarations
        .iter()
        .map(|d| (d.name.to_string(), d.value.to_string()))
        .collect();
    assert_eq!(
        values,
        vec![
            ("top_speed".to_string(), "55".to_string()),
            ("mass".to_string(), "1500".to_string()),
            ("half_speed".to_string(), "27.5".to_string()),
        ]
    );
}

#[test]
fn an_assignment_to_an_undeclared_parameter_is_refused() {
    let xml = with_reference(
        r#"<CatalogReference catalogName="Vehicles" entryName="sedan">
             <ParameterAssignments>
               <ParameterAssignment parameterRef="colour" value="red"/>
             </ParameterAssignments>
           </CatalogReference>"#,
    );
    let msg = resolve_variant("undeclared", &xml).unwrap_err().to_string();
    assert!(msg.contains("colour"), "{msg}");
    assert!(msg.contains("sedan"), "{msg}");
    assert!(msg.contains("not declared"), "{msg}");
}

#[test]
fn two_assignments_to_one_parameter_are_refused() {
    let xml = with_reference(
        r#"<CatalogReference catalogName="Vehicles" entryName="sedan">
             <ParameterAssignments>
               <ParameterAssignment parameterRef="top_speed" value="40"/>
               <ParameterAssignment parameterRef="top_speed" value="50"/>
             </ParameterAssignments>
           </CatalogReference>"#,
    );
    let msg = resolve_variant("twice", &xml).unwrap_err().to_string();
    assert!(msg.contains("top_speed"), "{msg}");
    assert!(msg.contains("more than once"), "{msg}");
}

#[test]
fn an_assigned_value_must_fit_the_declared_type() {
    let xml = with_reference(
        r#"<CatalogReference catalogName="Vehicles" entryName="sedan">
             <ParameterAssignments>
               <ParameterAssignment parameterRef="top_speed" value="fast"/>
             </ParameterAssignments>
           </CatalogReference>"#,
    );
    let msg = resolve_variant("mistyped", &xml).unwrap_err().to_string();
    assert!(msg.contains("top_speed"), "{msg}");
    assert!(msg.contains("fast"), "{msg}");
}

/// The `leaky` entry uses `$document_speed` without declaring it. The document declares it,
/// but section 9.5 allows no other parameters inside the catalog.
#[test]
fn an_entry_cannot_see_the_parameters_of_the_referencing_document() {
    let xml = with_reference(r#"<CatalogReference catalogName="Vehicles" entryName="leaky"/>"#)
        .replace(
            r#"<ParameterDeclaration name="catalog" parameterType="string" value="Vehicles"/>"#,
            r#"<ParameterDeclaration name="catalog" parameterType="string" value="Vehicles"/>
               <ParameterDeclaration name="document_speed" parameterType="double" value="25"/>"#,
        );
    let msg = resolve_variant("leaky", &xml).unwrap_err().to_string();
    assert!(msg.contains("document_speed"), "{msg}");
    assert!(msg.contains("leaky"), "{msg}");
}

#[test]
fn a_reference_to_a_missing_entry_is_refused() {
    let xml = with_reference(r#"<CatalogReference catalogName="Vehicles" entryName="truck"/>"#);
    let msg = resolve_variant("missing", &xml).unwrap_err().to_string();
    assert!(msg.contains("truck"), "{msg}");
    assert!(msg.contains("Vehicles"), "{msg}");
}

/// The `towing` entry's trailer is a reference to `towing` itself.
#[test]
fn a_reference_that_reaches_itself_is_refused() {
    let xml = with_reference(r#"<CatalogReference catalogName="Vehicles" entryName="towing"/>"#);
    let msg = resolve_variant("cycle", &xml).unwrap_err().to_string();
    assert!(msg.to_lowercase().contains("circular"), "{msg}");
    assert!(msg.contains("towing"), "{msg}");
}

/// The `tractor` entry's trailer is a reference whose assignment reads `$trailer_speed`. The
/// reference sits inside the entry, so the value is the entry's `trailer_speed`.
#[test]
fn a_nested_reference_resolves_its_assignments_in_the_entry_scope() {
    let xml = with_reference(r#"<CatalogReference catalogName="Vehicles" entryName="tractor"/>"#);
    let document = resolve_variant("nested", &xml).expect("variant resolves");
    let serialized = quick_xml::se::to_string(&document).expect("document serializes");
    let trailer = &serialized[serialized.find("<Trailer name=\"load\">").expect("trailer")..];
    assert!(trailer.contains(r#"maxSpeed="20""#), "{trailer}");
}

/// `more_vehicles.xosc` holds a `sedan` in the catalog `Other`, beside the `sedan` of the
/// catalog `Vehicles`. Section 9.6 locates an entry "by name and the entry within this catalog
/// by its entry name", so the catalog name selects between them.
#[test]
fn a_reference_names_its_catalog_as_well_as_its_entry() {
    let xml = with_reference(r#"<CatalogReference catalogName="Other" entryName="sedan"/>"#);
    let document = resolve_variant("other_catalog", &xml).expect("variant resolves");
    assert_eq!(
        literal(&vehicle(&document, "Assigned").performance.max_speed),
        99.0
    );
    assert_eq!(
        literal(&vehicle(&document, "Defaults").performance.max_speed),
        30.0
    );
}

#[test]
fn an_entry_defined_twice_is_refused() {
    let xml = with_reference(r#"<CatalogReference catalogName="Vehicles" entryName="twin"/>"#);
    let msg = resolve_variant("twin", &xml).unwrap_err().to_string();
    assert!(msg.contains("twin"), "{msg}");
    assert!(msg.contains("2 times"), "{msg}");
}

#[test]
fn an_assignment_to_an_entry_without_declarations_is_refused() {
    let xml = with_reference(
        r#"<CatalogReference catalogName="Vehicles" entryName="plain">
             <ParameterAssignments>
               <ParameterAssignment parameterRef="top_speed" value="40"/>
             </ParameterAssignments>
           </CatalogReference>"#,
    );
    let msg = resolve_variant("plain", &xml).unwrap_err().to_string();
    assert!(msg.contains("top_speed"), "{msg}");
    assert!(msg.contains("declared: none"), "{msg}");
}

/// `parameterRef` is an XSD `String` too, so it may be a reference; it belongs to the
/// referencing document and resolves there.
#[test]
fn an_assignment_parameter_ref_may_itself_be_a_reference() {
    let xml = with_reference(
        r#"<CatalogReference catalogName="Vehicles" entryName="sedan">
             <ParameterAssignments>
               <ParameterAssignment parameterRef="$which" value="44"/>
             </ParameterAssignments>
           </CatalogReference>"#,
    )
    .replace(
        r#"<ParameterDeclaration name="catalog" parameterType="string" value="Vehicles"/>"#,
        r#"<ParameterDeclaration name="catalog" parameterType="string" value="Vehicles"/>
           <ParameterDeclaration name="which" parameterType="string" value="top_speed"/>"#,
    );
    let document = resolve_variant("parameter_ref", &xml).expect("variant resolves");
    assert_eq!(
        literal(&vehicle(&document, "Assigned").performance.max_speed),
        44.0
    );
}

#[test]
fn a_controller_reference_in_an_object_controller_is_resolved() {
    let fixture = std::fs::read_to_string(fixture_dir().join("scenario.xosc")).unwrap();
    let xml = fixture.replace(
        r#"<CatalogReference catalogName="Vehicles" entryName="sedan"/>"#,
        r#"<CatalogReference catalogName="Vehicles" entryName="sedan"/>
           <ObjectController>
             <CatalogReference catalogName="Controllers" entryName="driver">
               <ParameterAssignments>
                 <ParameterAssignment parameterRef="aggressiveness" value="0.9"/>
               </ParameterAssignments>
             </CatalogReference>
           </ObjectController>"#,
    );
    let document = resolve_variant("controller", &xml).expect("variant resolves");
    let serialized = quick_xml::se::to_string(&document).expect("document serializes");
    assert!(
        serialized.contains(r#"<Property name="aggressiveness" value="0.9"/>"#),
        "{serialized}"
    );
}

/// A trajectory entry is written in place of the reference as it stands in the catalog, so a
/// NURBS shape stays a NURBS shape.
#[test]
fn a_trajectory_reference_keeps_the_entry_shape() {
    let fixture = std::fs::read_to_string(fixture_dir().join("scenario.xosc")).unwrap();
    let xml = fixture.replace(
        "<Actions/>",
        r#"<Actions>
             <Private entityRef="Assigned">
               <PrivateAction>
                 <RoutingAction>
                   <FollowTrajectoryAction>
                     <TimeReference>
                       <None/>
                     </TimeReference>
                     <TrajectoryFollowingMode followingMode="position"/>
                     <TrajectoryRef>
                       <CatalogReference catalogName="Trajectories" entryName="curve"/>
                     </TrajectoryRef>
                   </FollowTrajectoryAction>
                 </RoutingAction>
               </PrivateAction>
             </Private>
           </Actions>"#,
    );
    let document = resolve_variant("trajectory", &xml).expect("variant resolves");
    let serialized = quick_xml::se::to_string(&document).expect("document serializes");
    assert!(serialized.contains(r#"<Nurbs order="2">"#), "{serialized}");
    assert!(!serialized.contains("<Polyline"), "{serialized}");
}

#[test]
fn a_reference_whose_kind_has_no_catalog_location_is_refused() {
    let fixture = std::fs::read_to_string(fixture_dir().join("scenario.xosc")).unwrap();
    let xml = fixture.replace(
        r#"<ManeuverCatalog>
      <Directory path="catalogs/maneuvers"/>
    </ManeuverCatalog>"#,
        "",
    );
    let msg = resolve_variant("no_location", &xml)
        .unwrap_err()
        .to_string();
    assert!(msg.contains("accelerate"), "{msg}");
    assert!(msg.contains("ManeuverCatalog"), "{msg}");
}

#[test]
fn a_resolved_document_holds_no_catalog_reference() {
    let dir = fixture_dir();
    let xml = std::fs::read_to_string(dir.join("scenario.xosc")).unwrap();
    let resolved = resolve_parameters(&xml, &dir).expect("fixture resolves");
    assert!(!resolved.contains("<CatalogReference"), "{resolved}");
    assert_eq!(resolved.matches(r#"<Vehicle name="sedan""#).count(), 3);
    assert_eq!(
        resolved.matches(r#"<Maneuver name="accelerate">"#).count(),
        1
    );
}

/// `parse_file` keeps every reference as written; only the resolving entry points expand them.
#[test]
fn parse_file_keeps_the_references() {
    let document = parse_file(fixture_dir().join("scenario.xosc")).expect("fixture parses");
    let entities = document.entities.as_ref().unwrap();
    assert!(entities
        .scenario_objects
        .iter()
        .all(|o| matches!(o.entity, EntityObjectChoice::CatalogReference(_))));
}

/// A string has no location of its own, so `parse_str_resolved` takes a relative catalog
/// directory from the working directory, which for a test is the crate root.
#[test]
fn parse_str_resolved_takes_relative_directories_from_the_working_directory() {
    let xml = std::fs::read_to_string(fixture_dir().join("scenario.xosc"))
        .unwrap()
        .replace(
            r#"path="catalogs/"#,
            r#"path="tests/data/catalog_parameter_assignments/catalogs/"#,
        );
    let document = parse_str_resolved(&xml).expect("resolves from the crate root");
    assert_eq!(
        literal(&vehicle(&document, "Assigned").performance.max_speed),
        55.0
    );
}

// --- Single references through CatalogManager --------------------------------------------

fn sedan_reference(assignments: Vec<ParameterAssignment>) -> VehicleCatalogReference {
    VehicleCatalogReference::with_parameters(
        "Vehicles".to_string(),
        "sedan".to_string(),
        assignments,
    )
}

fn vehicle_location() -> VehicleCatalogLocation {
    VehicleCatalogLocation::from_path(
        fixture_dir()
            .join("catalogs/vehicles")
            .to_string_lossy()
            .into_owned(),
    )
}

#[test]
fn catalog_manager_applies_an_assignment() {
    let reference = sedan_reference(vec![ParameterAssignment::new(
        "top_speed".to_string(),
        "55".to_string(),
    )]);
    let resolved = CatalogManager::new()
        .resolve_vehicle_reference(&reference, &vehicle_location())
        .expect("reference resolves");
    assert_eq!(resolved.entity.performance.max_speed, Value::Literal(55.0));
}

/// `more_vehicles.xosc` sorts before `vehicles.xosc` and holds a `sedan` (max speed 99) in the
/// catalog `Other`; a lookup by entry name alone would return that one instead of this default.
#[test]
fn catalog_manager_uses_the_entry_default_for_an_unassigned_parameter() {
    let resolved = CatalogManager::new()
        .resolve_vehicle_reference(&sedan_reference(Vec::new()), &vehicle_location())
        .expect("reference resolves");
    assert_eq!(resolved.entity.performance.max_speed, Value::Literal(30.0));
    assert_eq!(resolved.entity.mass, Some(Value::Literal(1500.0)));
}

#[test]
fn catalog_manager_refuses_an_assignment_to_an_undeclared_parameter() {
    let reference = sedan_reference(vec![ParameterAssignment::new(
        "colour".to_string(),
        "red".to_string(),
    )]);
    let msg = CatalogManager::new()
        .resolve_vehicle_reference(&reference, &vehicle_location())
        .map(|_| ())
        .unwrap_err()
        .to_string();
    assert!(msg.contains("colour"), "{msg}");
}

fn location(kind: &str) -> String {
    fixture_dir()
        .join("catalogs")
        .join(kind)
        .to_string_lossy()
        .into_owned()
}

#[test]
fn catalog_manager_resolves_a_pedestrian_with_its_default() {
    let reference =
        PedestrianCatalogReference::new("Pedestrians".to_string(), "walker".to_string());
    let resolved = CatalogManager::new()
        .resolve_pedestrian_reference(
            &reference,
            &PedestrianCatalogLocation::from_path(location("pedestrians")),
        )
        .expect("reference resolves");
    assert_eq!(resolved.entity.mass, Value::Literal(80.0));
}

#[test]
fn catalog_manager_resolves_a_controller_with_an_assignment() {
    let reference = ControllerCatalogReference::with_parameters(
        "Controllers".to_string(),
        "driver".to_string(),
        vec![ParameterAssignment::new(
            "aggressiveness".to_string(),
            "0.9".to_string(),
        )],
    );
    let resolved = CatalogManager::new()
        .resolve_controller_reference(
            &reference,
            &ControllerCatalogLocation::from_path(location("controllers")),
        )
        .expect("reference resolves");
    let xml = quick_xml::se::to_string(&resolved.entity).expect("controller serializes");
    assert!(xml.contains(r#"value="0.9""#), "{xml}");
}

#[test]
fn catalog_manager_matches_the_catalog_name() {
    let reference = VehicleCatalogReference::new("Other".to_string(), "sedan".to_string());
    let resolved = CatalogManager::new()
        .resolve_vehicle_reference(&reference, &vehicle_location())
        .expect("reference resolves");
    assert_eq!(resolved.entity.performance.max_speed, Value::Literal(99.0));
}

/// A relative catalog directory is taken from the manager's base path, not from the working
/// directory.
#[test]
fn catalog_manager_takes_a_relative_directory_from_its_base_path() {
    let resolved = CatalogManager::with_base_path(fixture_dir())
        .resolve_vehicle_reference(
            &sedan_reference(Vec::new()),
            &VehicleCatalogLocation::from_path("catalogs/vehicles".to_string()),
        )
        .expect("the relative directory resolves against the base path");
    assert_eq!(resolved.entity.performance.max_speed, Value::Literal(30.0));
}

/// A reference built outside a document has no scope around it, so a `$name` in an assigned
/// value cannot be resolved and is refused with a pointer to the document-level entry point.
#[test]
fn catalog_manager_refuses_a_parameter_reference_in_an_assignment() {
    let reference = sedan_reference(vec![ParameterAssignment::with_values(
        Value::Literal("top_speed".to_string()),
        Value::Parameter("speed".to_string()),
    )]);
    let msg = CatalogManager::new()
        .resolve_vehicle_reference(&reference, &vehicle_location())
        .map(|_| ())
        .unwrap_err()
        .to_string();
    assert!(msg.contains("$speed"), "{msg}");
    assert!(msg.contains("parse_file_resolved"), "{msg}");
}
