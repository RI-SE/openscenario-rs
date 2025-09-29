#!/bin/bash
echo "Checking builder compilation..."
cd /home/ashfaqf/playground/opencsd/openscenario-rs

echo "=== Running cargo check with builder feature ==="
cargo check --features builder 2>&1 | head -50

echo ""
echo "=== Running a simple builder test ==="
cargo test --features builder test_minimal_scenario_creation 2>&1 | head -30