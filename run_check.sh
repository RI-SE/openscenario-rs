#!/bin/bash
echo "Running cargo check with builder feature..."
cd /home/ashfaqf/playground/opencsd/openscenario-rs
cargo check --features builder 2>&1 | head -100