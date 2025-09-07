# OpenSCENARIO-rs Coding Guidelines

## Project Objective

This library provides a **complete**, **type-safe** Rust implementation for parsing, validating, and manipulating OpenSCENARIO files. The goal is to support all **347+ datatypes** from the OpenSCENARIO XSD schema with zero-copy parsing, comprehensive validation, and ergonomic APIs for autonomous driving simulation scenarios.

## Build & Test Commands

- `cargo build` - Build the project
- `cargo test` - Run all tests
- `cargo test test_name` - Run a specific test
- `cargo test --features full` - Run tests with all features enabled
- `cargo bench` - Run benchmarks (uses criterion)
- `cargo fmt` - Format code using rustfmt
- `cargo clippy` - Run linting

## Code Style & Conventions

- Use `//!` for module-level docs, `///` for function/struct docs
- Group imports: std first, then external crates, then local (crate::)
- Prefer `pub use` for re-exports in mod.rs files
- Use `thiserror` for error types with structured error variants
- Types use PascalCase, functions/variables use snake_case
- Error handling: Use `Result<T>` alias from error.rs, add context with `with_context()`
- Feature gates: Use `#[cfg(feature = "...")]` for conditional compilation
- Validation: Implement `Validate` trait for types that need validation
- Parameters: Use `Value<T>` for parameterizable fields, implement `Resolve` trait

## Testing

- Integration tests in `tests/` directory using real XOSC files
- Use `include_str!()` for test data files
- Test files in `tests/data/` and `tests/fixtures/`
- Property-based tests with `proptest` for complex validation

