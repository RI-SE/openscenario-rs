include!(concat!(env!("OUT_DIR"), "/generated_roundtrip.rs"));

/// The generated suite must describe the corpus that is on disk *now*.
///
/// `conformance/build.rs` declares `cargo:rerun-if-changed` on `conformance/corpus/`, but cargo
/// tracks a directory by the directory's own mtime and `mv` preserves mtime. Moving the corpus
/// aside and back therefore leaves cargo believing nothing changed, so the empty file written
/// while the corpus was absent survives: `cargo test` reports `0 passed` and `scripts/gate.sh`
/// goes green having round-tripped nothing.
///
/// Comparing the build-time counts against the corpus at run time cannot go vacuous. An empty
/// suite either matches an empty corpus — a fresh checkout, which must still build and only
/// warn — or fails and says how to recover.
#[test]
fn generated_suite_matches_the_corpus_on_disk() {
    let on_disk = openscenario_roundtrip_harness::corpus_files().len();
    assert_eq!(
        on_disk, CORPUS_FILES_AT_BUILD_TIME,
        "the generated round-trip suite is stale: conformance/build.rs saw \
         {CORPUS_FILES_AT_BUILD_TIME} corpus file(s) and generated {GENERATED_ROUNDTRIP_TESTS} \
         test(s), but conformance/corpus/ holds {on_disk} now. Run `touch conformance/build.rs` \
         and test again."
    );
    assert!(
        on_disk == 0 || GENERATED_ROUNDTRIP_TESTS > 0,
        "{on_disk} corpus file(s) are present but the generated round-trip suite is empty"
    );
}
