#!/bin/bash
echo "Testing builder compilation..."
cargo check --features builder
echo "Exit code: $?"