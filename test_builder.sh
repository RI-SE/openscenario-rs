#!/bin/bash
echo "Testing builder compilation..."
cargo check --features builder
echo "Running builder tests..."
cargo test --features builder builder_complete_scenario_test