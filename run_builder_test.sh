#!/bin/bash
echo "Running builder tests with feature enabled..."
cd /home/ashfaqf/playground/opencsd/openscenario-rs
cargo test --features builder builder_basic_test::builder_tests::test_minimal_scenario_creation 2>&1