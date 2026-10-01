//! What a caller reads when a file-reading entry point fails.
//!
//! `with_context` used to have no arm for `Error::XmlParseError`, so a syntax or type error on
//! the plain (unresolved) parse path lost whatever context the caller wrapped it with. A user
//! calling `parse_from_file` on a malformed file therefore got a message naming the quick-xml
//! failure but never the file it came from. The `_validated` entry points and the size limit
//! fail before any parse, each with its own message.
//!
//! Every row also asserts the path, so a failure that loses it (an I/O error, a structural
//! rejection, the size refusal) cannot pass as one that keeps it.
//!
//! One row per entry point and failure. Every row is checked and every failure reported, so one
//! failing row cannot hide the next.

use openscenario_rs::catalog::loader::CatalogLoader;
use openscenario_rs::parser::xml::{
    parse_catalog_from_file_validated, parse_from_file_validated, serialize_catalog_to_file,
};
use openscenario_rs::{parse_catalog_from_file, parse_from_file, serialize_to_file, Error};
use std::path::Path;

/// The largest file the parser reads, `MAX_FILE_SIZE` in `src/parser/xml.rs`.
const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024;

enum Fixture {
    /// No file is created at all.
    Absent,
    Text(&'static str),
    /// A sparse file one byte over the limit; its content is never read.
    Oversized,
}

type EntryPoint = fn(&Path) -> Result<(), Error>;

const HEADER_ONLY: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<OpenSCENARIO>
  <FileHeader revMajor="1" revMinor="3" date="2024-01-01T00:00:00" author="a" description="d"/>
</OpenSCENARIO>"#;

/// Well-formed XML, but `revMajor` is not the `xsd:unsignedShort` the schema asks for.
const BAD_REV_MAJOR: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<OpenSCENARIO>
  <FileHeader revMajor="not-a-number" revMinor="3" date="2024-01-01T00:00:00" author="a" description="d"/>
  <CatalogLocations/>
  <RoadNetwork/>
  <Entities/>
  <Storyboard><Init><Actions/></Init><StopTrigger/></Storyboard>
</OpenSCENARIO>"#;

fn empty_scenario() -> openscenario_rs::OpenScenario {
    openscenario_rs::parse_from_str(BAD_REV_MAJOR.replace("not-a-number", "1").as_str())
        .expect("fixture parses")
}

fn empty_catalog() -> openscenario_rs::types::catalogs::files::CatalogFile {
    openscenario_rs::parse_catalog_from_str(CATALOG).expect("fixture parses")
}

const CATALOG: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<OpenSCENARIO>
  <FileHeader revMajor="1" revMinor="3" date="2024-01-01T00:00:00" author="a" description="d"/>
  <Catalog name="c"/>
</OpenSCENARIO>"#;

#[test]
fn each_file_entry_point_reports_its_failure() {
    let over = (MAX_FILE_SIZE + 1).to_string();
    let limit = MAX_FILE_SIZE.to_string();
    let too_large = [
        "Value out of range for field 'file_size'",
        over.as_str(),
        limit.as_str(),
        "{path}",
    ];
    // (label, file name, content, entry point, substrings the message must hold; `{path}`
    // stands for the file's path)
    let rows: Vec<(&str, &str, Fixture, EntryPoint, Vec<&str>)> = vec![
        (
            "syntax error through parse_from_file names the file",
            "malformed.xosc",
            Fixture::Text("<not valid xml"),
            |p| parse_from_file(p).map(drop),
            vec!["XML parsing error", "{path}"],
        ),
        (
            "syntax error through parse_catalog_from_file names the file",
            "malformed_catalog.xosc",
            Fixture::Text("<not valid xml"),
            |p| parse_catalog_from_file(p).map(drop),
            vec!["XML parsing error", "{path}"],
        ),
        (
            "type error through parse_from_file names the file",
            "bad_type.xosc",
            Fixture::Text(BAD_REV_MAJOR),
            |p| parse_from_file(p).map(drop),
            vec!["XML parsing error", "{path}"],
        ),
        // `CatalogLoader::load_and_parse_catalog_file` composes its own `with_context` on top
        // of `parse_catalog_from_file`'s; it used to drop its context for the same reason.
        (
            "syntax error through CatalogLoader names the file",
            "malformed_loader.xosc",
            Fixture::Text("<not valid xml"),
            |p| {
                CatalogLoader::new()
                    .load_and_parse_catalog_file(p)
                    .map(drop)
            },
            vec!["XML parsing error", "{path}"],
        ),
        // The structural pass rejects the file before the typed parser sees it; without it,
        // this file would fail as an `XML parsing error` instead.
        (
            "parse_from_file_validated on a document with no OpenSCENARIO root",
            "other_root.xosc",
            Fixture::Text("<Root/>"),
            |p| parse_from_file_validated(p).map(drop),
            vec![
                "Invalid XML structure",
                "Document does not appear to contain OpenSCENARIO root element",
                "{path}",
            ],
        ),
        (
            "parse_catalog_from_file_validated on a document with no Catalog",
            "no_catalog.xosc",
            Fixture::Text(HEADER_ONLY),
            |p| parse_catalog_from_file_validated(p).map(drop),
            vec![
                "Invalid XML structure",
                "Document does not appear to contain Catalog element",
                "{path}",
            ],
        ),
        (
            "parse_from_file refuses a file over the size limit",
            "too_large.xosc",
            Fixture::Oversized,
            |p| parse_from_file(p).map(drop),
            too_large.to_vec(),
        ),
        (
            "parse_catalog_from_file refuses a file over the size limit",
            "too_large_catalog.xosc",
            Fixture::Oversized,
            |p| parse_catalog_from_file(p).map(drop),
            too_large.to_vec(),
        ),
        (
            "parse_from_file on a missing file",
            "missing.xosc",
            Fixture::Absent,
            |p| parse_from_file(p).map(drop),
            vec!["File not found", "{path}"],
        ),
        (
            "parse_catalog_from_file on a missing file",
            "missing_catalog.xosc",
            Fixture::Absent,
            |p| parse_catalog_from_file(p).map(drop),
            vec!["File not found", "{path}"],
        ),
        (
            "serialize_to_file into a missing directory",
            "no_such_dir/out.xosc",
            Fixture::Absent,
            |p| serialize_to_file(&empty_scenario(), p),
            vec!["Cannot write file", "{path}"],
        ),
        (
            "serialize_catalog_to_file into a missing directory",
            "no_such_dir/out_catalog.xosc",
            Fixture::Absent,
            |p| serialize_catalog_to_file(&empty_catalog(), p),
            vec!["Cannot write file", "{path}"],
        ),
    ];

    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("xml_parse_error_context");
    std::fs::create_dir_all(&dir).expect("temporary directory");
    let mut failures = Vec::new();
    for (label, name, fixture, entry, expected) in rows {
        let path = dir.join(name);
        match fixture {
            Fixture::Absent => {
                let _ = std::fs::remove_file(&path);
            }
            Fixture::Text(text) => std::fs::write(&path, text).expect("fixture written"),
            Fixture::Oversized => std::fs::File::create(&path)
                .and_then(|f| f.set_len(MAX_FILE_SIZE + 1))
                .expect("sparse fixture written"),
        }
        let msg = match entry(&path) {
            Ok(()) => {
                failures.push(format!("{label}: parsed without error"));
                continue;
            }
            Err(e) => e.to_string(),
        };
        let path_text = path.display().to_string();
        for want in expected {
            let want = want.replace("{path}", &path_text);
            if !msg.contains(&want) {
                // Capped: a regression that reads the oversized file echoes its content.
                let shown: String = msg.chars().take(400).collect();
                failures.push(format!("{label}: expected `{want}` in: {shown}"));
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
