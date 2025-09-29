#!/bin/bash
echo "Testing detached builder pattern implementation..."
cd /home/ashfaqf/playground/opencsd/openscenario-rs

echo "=== Checking compilation with builder feature ==="
cargo check --features builder --bin test_detached_compilation 2>&1 | head -20

echo ""
echo "=== Running detached builder compilation test ==="
cargo run --features builder --bin test_detached_compilation 2>&1 | head -10

echo ""
echo "=== Running detached builder tests ==="
cargo test --features builder detached_builder_tests 2>&1 | head -20