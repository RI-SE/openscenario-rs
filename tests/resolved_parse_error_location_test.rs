//! Where a typed-parse error is reported once a document has passed through
//! [`resolve_parameters`], which parses **the resolved text**, not what the caller wrote.
//!
//! Substituting a `$name` reference or an `${expression}` never changes a document's line
//! count, since the replacement text lands inside the same attribute value. Inlining a
//! `<CatalogReference>` does: the catalog entry that replaces it can span more or fewer lines
//! than the reference did, so every later line shifts. A typed-parse error's own position is
//! always against the resolved text; these tests check that it is translated back to a line the
//! caller can actually find -- the source document's own line normally, or the catalog file's
//! line when the failure sits inside an inlined entry.
//!
//! Each claim is its own test, so that one failing claim cannot hide the next.

use openscenario_rs::{parse_file_resolved, parse_str_resolved};
use std::path::{Path, PathBuf};

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/resolved_parse_error_location")
}

// --- No catalog reference: the resolved text has the same line count as the source ---------

/// A `string`-typed parameter substituted into a `Double` attribute leaves the typed parser
/// nothing that fits `xsd:double`, so it fails after resolution -- with no catalog reference in
/// the document, the resolved text is line-for-line identical to what was parsed, and the error
/// should name the very line the caller wrote.
#[test]
fn a_typed_parse_error_after_resolution_names_the_line_the_document_has() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<OpenSCENARIO>
  <FileHeader revMajor="1" revMinor="3" date="2024-01-01T00:00:00" author="a" description="d"/>
  <ParameterDeclarations>
    <ParameterDeclaration name="speed" parameterType="string" value="fast"/>
  </ParameterDeclarations>
  <CatalogLocations/>
  <RoadNetwork/>
  <Entities>
    <ScenarioObject name="Ego">
      <Vehicle name="car" vehicleCategory="car">
        <BoundingBox>
          <Center x="0" y="0" z="0"/>
          <Dimensions width="2" length="4" height="1.5"/>
        </BoundingBox>
        <Performance maxSpeed="$speed" maxAcceleration="5" maxDeceleration="10"/>
        <Axles>
          <FrontAxle maxSteering="0.5" wheelDiameter="0.6" trackWidth="1.8" positionX="1.3" positionZ="0.3"/>
          <RearAxle maxSteering="0" wheelDiameter="0.6" trackWidth="1.8" positionX="-1.3" positionZ="0.3"/>
        </Axles>
      </Vehicle>
    </ScenarioObject>
  </Entities>
  <Storyboard><Init><Actions/></Init><StopTrigger/></Storyboard>
</OpenSCENARIO>"#;
    // Line 16 is `<Performance maxSpeed="$speed" .../>`; `$speed` resolves to the literal
    // text "fast" before the typed parser ever sees it.
    assert_eq!(
        xml.lines().nth(15).unwrap().trim_start(),
        r#"<Performance maxSpeed="$speed" maxAcceleration="5" maxDeceleration="10"/>"#
    );

    let msg = parse_str_resolved(xml).unwrap_err().to_string();
    assert!(msg.contains("fast"), "{msg}");
    assert!(msg.contains("line 16"), "{msg}");
}

// --- A catalog reference before the failure shifts every later line -------------------------

/// `sedan` (`vehicles.xosc:5-15`, eleven lines) replaces the single-line `<CatalogReference/>`
/// at `scenario_shift.xosc:15`, so everything from there on shifts down by ten lines in the
/// resolved text. The document's own `<Performance maxSpeed="$speed" .../>`, which fails after
/// `$speed` resolves to the non-numeric literal "fast", sits at source line 23 but resolved line
/// 33. The reported location must be the line the caller can find in their own file, 23, not 33.
#[test]
fn a_typed_parse_error_after_the_catalog_reference_names_the_document_line_not_the_resolved_one() {
    let msg = parse_file_resolved(fixture_dir().join("scenario_shift.xosc"))
        .unwrap_err()
        .to_string();
    assert!(msg.contains("fast"), "{msg}");
    assert!(msg.contains("line 23"), "{msg}");
    assert!(!msg.contains("line 33"), "{msg}");
}

// --- A failure inside an inlined entry names the catalog file, not the scenario -------------

/// `brokenEntry` declares `topSpeed` as a `string` and uses it as `Performance`'s `maxSpeed`
/// (`vehicles.xosc:24`). Resolution accepts it -- a `string` declaration conforms to any text --
/// so the failure only appears once the typed parser reads the inlined `maxSpeed="fast"`. The
/// caller cannot fix line 24 of their own scenario file: nothing at that line came from them.
/// The location must instead name `vehicles.xosc` and the line inside it.
#[test]
fn a_typed_parse_error_inside_an_inlined_entry_names_the_catalog_file() {
    let msg = parse_file_resolved(fixture_dir().join("scenario_entry.xosc"))
        .unwrap_err()
        .to_string();
    assert!(msg.contains("fast"), "{msg}");
    assert!(msg.contains("catalog file"), "{msg}");
    assert!(msg.contains("vehicles.xosc"), "{msg}");
    assert!(msg.contains("line 24"), "{msg}");
}

// --- `with_context` no longer splices a generic description into a specific one -------------

/// Before the fix, wrapping a `ParameterError` with a file-level context prepended that context
/// into the message field the `Parameter '{param}' error: ` prefix already introduces, so the
/// generic "Failed to parse file: ..." sentence landed between the prefix and the actual
/// reason -- as if the file itself had failed to parse, with the real cause trailing after a
/// second colon. The context now appends instead, so the parameter's own message reads as one
/// unbroken sentence and the file context is a distinguishable trailing clause.
#[test]
fn wrapping_a_parameter_error_with_file_context_does_not_split_its_message() {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("resolved_parse_error_location");
    std::fs::create_dir_all(&dir).expect("temporary directory");
    let path = dir.join("undeclared.xosc");
    std::fs::write(
        &path,
        r#"<?xml version="1.0" encoding="UTF-8"?>
<OpenSCENARIO>
  <FileHeader revMajor="1" revMinor="3" date="2024-01-01T00:00:00" author="a" description="d"/>
  <CatalogLocations/>
  <RoadNetwork/>
  <Entities/>
  <Storyboard><Init><Actions/></Init><StopTrigger name="$undeclared"/></Storyboard>
</OpenSCENARIO>"#,
    )
    .expect("fixture written");

    let msg = parse_file_resolved(&path).unwrap_err().to_string();

    // The parameter's own message is intact and starts the string: nothing was inserted
    // between the prefix `with_context` cannot see and the reason that follows it.
    assert!(
        msg.starts_with("Parameter 'undeclared' error: not declared"),
        "{msg}"
    );
    // The file-level context survives, as a trailing clause, not spliced into the reason.
    assert!(
        msg.ends_with(&format!("(Failed to parse file: {})", path.display())),
        "{msg}"
    );
    // The two are joined by ` (`, so `with_context` never wrote `context: message`.
    assert!(!msg.contains("error: Failed to parse file"), "{msg}");
}
