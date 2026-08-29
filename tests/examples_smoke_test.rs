//! Runs every example and asserts it exits successfully.
//!
//! Examples are compiled by `cargo test --all-targets` but never executed, so an example can
//! compile and still panic on a missing fixture, a builder chain that omits a required element,
//! or an action built without its mandatory field. Every such rot has happened in this
//! repository at least once and was invisible until someone ran the example by hand.
//!
//! Each case shells out to `cargo run --example <name>`, inheriting `CARGO_TARGET_DIR` so the
//! binaries built by the enclosing `cargo test` are reused rather than rebuilt.

use std::path::{Path, PathBuf};
use std::process::Command;

/// Examples that need no feature flags.
const PLAIN_EXAMPLES: &[&str] = &[
    "action_wrappers_demo",
    "basic_parsing",
    "bounding_box_demo",
    "byvalue_conditions_demo",
    "expression_demo",
    "motion_conditions_demo",
    "routing_demo",
    "simple_spatial_demo",
    "spatial_conditions_demo",
    "vehicle_axles_demo",
    "vehicle_components_demo",
];

/// Examples that take a scenario file as an argument, paired with a fixture to feed them.
const ARGUMENT_EXAMPLES: &[(&str, &str)] = &[("parse", "tests/data/simple_scenario.xosc")];

/// Examples that require `--features builder`.
const BUILDER_EXAMPLES: &[&str] = &[
    "alks_scenario_4_1_1_comprehensive",
    "builder_basic_demo",
    "builder_comprehensive_demo",
    "builder_performance_demo",
    "cut_in_scenario_demo",
    "pedestrian_builder_demo",
];

/// The crate root, so examples that open fixtures by relative path find them.
fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn run_example(name: &str, features: Option<&str>, args: &[&str]) -> Result<(), String> {
    let mut command = Command::new(env!("CARGO"));
    command
        .current_dir(crate_root())
        .args(["run", "--quiet", "--example", name]);

    if let Some(features) = features {
        command.args(["--features", features]);
    }

    if !args.is_empty() {
        command.arg("--");
        command.args(args);
    }

    let output = command
        .output()
        .map_err(|e| format!("could not spawn cargo for example `{name}`: {e}"))?;

    if output.status.success() {
        return Ok(());
    }

    // The example's own stderr is where the panic or returned error lands; cargo's build
    // warnings land there too, so report the tail rather than the whole stream.
    let stderr = String::from_utf8_lossy(&output.stderr);
    let tail: Vec<&str> = stderr.lines().rev().take(12).collect();
    let tail = tail.into_iter().rev().collect::<Vec<_>>().join("\n");

    Err(format!(
        "example `{name}` exited with {}:\n{tail}",
        output.status
    ))
}

fn run_all(names: &[&str], features: Option<&str>) {
    let failures: Vec<String> = names
        .iter()
        .filter_map(|name| run_example(name, features, &[]).err())
        .collect();

    assert!(
        failures.is_empty(),
        "{} of {} examples failed:\n\n{}",
        failures.len(),
        names.len(),
        failures.join("\n\n")
    );
}

#[test]
#[ignore = "spawns cargo per example; run with --ignored or via the examples gate"]
fn plain_examples_run() {
    run_all(PLAIN_EXAMPLES, None);
}

#[test]
#[ignore = "spawns cargo per example; run with --ignored or via the examples gate"]
fn builder_examples_run() {
    run_all(BUILDER_EXAMPLES, Some("builder"));
}

#[test]
#[ignore = "spawns cargo per example; run with --ignored or via the examples gate"]
fn argument_examples_run() {
    let failures: Vec<String> = ARGUMENT_EXAMPLES
        .iter()
        .filter_map(|(name, fixture)| run_example(name, None, &[fixture]).err())
        .collect();

    assert!(
        failures.is_empty(),
        "{} of {} argument-taking examples failed:\n\n{}",
        failures.len(),
        ARGUMENT_EXAMPLES.len(),
        failures.join("\n\n")
    );
}

/// Guards the lists above against drift: every file in `examples/` must appear in one of them.
///
/// This one is cheap, so it is not ignored. Without it a new example could be added and never
/// covered, which is exactly how the existing breakage went unnoticed.
#[test]
fn every_example_is_covered() {
    let examples_dir = crate_root().join("examples");
    let mut uncovered = Vec::new();

    for entry in std::fs::read_dir(&examples_dir).expect("examples/ must be readable") {
        let path = entry.expect("readable dir entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue; // examples/ also holds .xodr fixtures
        }

        let stem = Path::new(&path)
            .file_stem()
            .and_then(|s| s.to_str())
            .expect("example file name must be valid UTF-8")
            .to_string();

        let covered = PLAIN_EXAMPLES.contains(&stem.as_str())
            || BUILDER_EXAMPLES.contains(&stem.as_str())
            || ARGUMENT_EXAMPLES.iter().any(|(name, _)| *name == stem);

        if !covered {
            uncovered.push(stem);
        }
    }

    uncovered.sort();
    assert!(
        uncovered.is_empty(),
        "these examples are not listed in examples_smoke_test.rs and so are never run: {uncovered:?}"
    );
}
