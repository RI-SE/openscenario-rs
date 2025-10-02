//! Custom deserialization logic for complex OpenSCENARIO patterns
//!
//! This module provides specialized deserializers for OpenSCENARIO XML patterns that
//! cannot be handled by standard serde deserialization, including parameter expressions,
//! polymorphic content, and mixed attribute/element patterns.
//!
//! # Features
//!
//! - **Parameter expression parsing** - `${parameter}` and `{expression}` syntax
//! - **Polymorphic deserialization** - choice-based XML elements with type resolution
//! - **Mixed content handling** - attributes and elements in the same context
//! - **Custom date/time formats** - OpenSCENARIO-specific timestamp parsing
//! - **Catalog reference resolution** - lazy loading and validation of external references
//! - **Expression evaluation** - mathematical and logical expression support
//!
//! # Parameter Expression Handling
//!
//! OpenSCENARIO supports parameterized values using special syntax.
//! This module provides the infrastructure for parsing these expressions,
//! though the actual parsing functions may be located in other modules.
//!
//! # Custom Date/Time Deserialization
//!
//! OpenSCENARIO uses specific date/time formats that require custom deserialization.
//!
//! # Polymorphic Content Deserialization
//!
//! For XSD choice groups and polymorphic elements, this module provides
//! infrastructure for handling variant types and type resolution.
//!
//! # Mixed Attribute/Element Patterns
//!
//! Some OpenSCENARIO elements mix attributes and child elements.
//! This module provides utilities for handling these mixed content patterns.
//!
//! # Catalog Reference Resolution
//!
//! Catalog references require special handling for lazy loading and validation.
//!
//! # Expression Evaluation
//!
//! Mathematical and logical expressions in parameter values require
//! special evaluation logic handled by the expression module.
//!
//! # Custom Deserializer Implementation
//!
//! This module provides patterns and utilities for implementing custom
//! deserializers for complex OpenSCENARIO-specific patterns.
//!
//! # Performance Considerations
//!
//! - Parameter expression parsing is cached for repeated use
//! - Catalog resolution is lazy and cached to avoid redundant loading
//! - Expression evaluation uses compiled expressions when possible
//! - Use type-specific deserializers for better performance
//!
//! # Error Handling
//!
//! All deserializers provide detailed error context with information
//! about what failed and where in the parsing process.
