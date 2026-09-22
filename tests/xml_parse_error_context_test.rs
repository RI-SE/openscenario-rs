//! `with_context` used to have no arm for `Error::XmlParseError`, so a syntax or type error on
//! the plain (unresolved) parse path lost whatever context the caller wrapped it with. A user
//! calling `parse_from_file` on a malformed file therefore got a message naming the quick-xml
//! failure but never the file it came from. Each claim below is its own test.

use openscenario_rs::{parse_catalog_from_file, parse_from_file};
use std::path::Path;

/// A syntax error in a file parsed through `parse_from_file` must name the file, not only the
/// underlying quick-xml failure.
#[test]
fn a_syntax_error_from_parse_from_file_names_the_file() {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("xml_parse_error_context");
    std::fs::create_dir_all(&dir).expect("temporary directory");
    let path = dir.join("malformed.xosc");
    std::fs::write(&path, "<not valid xml").expect("fixture written");

    let msg = parse_from_file(&path).unwrap_err().to_string();
    assert!(
        msg.contains(&path.display().to_string()),
        "expected the file path in the error, got: {msg}"
    );
}

/// The same holds for a catalog file parsed through `parse_catalog_from_file`: the loader chain
/// wraps `Error::XmlParseError` with the same `with_context`, so it needs the context field too.
#[test]
fn a_syntax_error_from_parse_catalog_from_file_names_the_file() {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("xml_parse_error_context");
    std::fs::create_dir_all(&dir).expect("temporary directory");
    let path = dir.join("malformed_catalog.xosc");
    std::fs::write(&path, "<not valid xml").expect("fixture written");

    let msg = parse_catalog_from_file(&path).unwrap_err().to_string();
    assert!(
        msg.contains(&path.display().to_string()),
        "expected the file path in the error, got: {msg}"
    );
}

/// A type error (well-formed XML, but a value the schema's type rejects) goes through the same
/// path as a syntax error, and must carry the file too.
#[test]
fn a_type_error_from_parse_from_file_names_the_file() {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("xml_parse_error_context");
    std::fs::create_dir_all(&dir).expect("temporary directory");
    let path = dir.join("bad_type.xosc");
    std::fs::write(
        &path,
        r#"<?xml version="1.0" encoding="UTF-8"?>
<OpenSCENARIO>
  <FileHeader revMajor="not-a-number" revMinor="3" date="2024-01-01T00:00:00" author="a" description="d"/>
  <CatalogLocations/>
  <RoadNetwork/>
  <Entities/>
  <Storyboard><Init><Actions/></Init><StopTrigger/></Storyboard>
</OpenSCENARIO>"#,
    )
    .expect("fixture written");

    let msg = parse_from_file(&path).unwrap_err().to_string();
    assert!(
        msg.contains(&path.display().to_string()),
        "expected the file path in the error, got: {msg}"
    );
}

/// `CatalogLoader::load_and_parse_catalog_file` (`src/catalog/loader.rs`) composes its own
/// `with_context` on top of `parse_catalog_from_file`'s. Before the fix this dropped its context
/// too, for the same reason.
#[test]
fn a_syntax_error_through_catalog_loader_names_the_file() {
    use openscenario_rs::catalog::loader::CatalogLoader;

    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("xml_parse_error_context");
    std::fs::create_dir_all(&dir).expect("temporary directory");
    let path = dir.join("malformed_loader.xosc");
    std::fs::write(&path, "<not valid xml").expect("fixture written");

    let loader = CatalogLoader::new();
    let msg = loader
        .load_and_parse_catalog_file(&path)
        .unwrap_err()
        .to_string();
    assert!(
        msg.contains(&path.display().to_string()),
        "expected the file path in the error, got: {msg}"
    );
}
