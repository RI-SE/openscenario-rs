#!/bin/bash
echo "Checking compilation errors..."
cargo check --features builder 2>&1 | head -50