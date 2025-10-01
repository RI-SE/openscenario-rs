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
//! OpenSCENARIO supports parameterized values using special syntax:
//!
//! ```rust
//! use openscenario_rs::parser::deserializer::parse_parameter_value;
//! use openscenario_rs::types::basic::Value;
//!
//! // Parse literal value
//! let literal: Value<f64> = parse_parameter_value("42.5")?;
//! assert_eq!(literal.as_literal(), Some(&42.5));
//!
//! // Parse parameter reference
//! let param_ref: Value<f64> = parse_parameter_value("${vehicle_speed}")?;
//! assert_eq!(param_ref.as_parameter(), Some("vehicle_speed"));
//!
//! // Parse expression
//! let expr: Value<f64> = parse_parameter_value("{$vehicle_speed * 1.5}")?;
//! assert!(expr.is_expression());
//! ```
//!
//! # Custom Date/Time Deserialization
//!
//! ```rust
//! use openscenario_rs::parser::deserializer::deserialize_openscenario_datetime;
//!
//! // Supports various OpenSCENARIO timestamp formats
//! let datetime1 = deserialize_openscenario_datetime("2024-01-01T12:00:00")?;
//! let datetime2 = deserialize_openscenario_datetime("2024-01-01T12:00:00.000Z")?;
//! let datetime3 = deserialize_openscenario_datetime("2024-01-01T12:00:00+01:00")?;
//! ```
//!
//! # Polymorphic Content Deserialization
//!
//! For XSD choice groups and polymorphic elements:
//!
//! ```rust
//! use openscenario_rs::parser::deserializer::PolymorphicDeserializer;
//!
//! // Define the variant types
//! #[derive(Deserialize)]
//! #[serde(rename_all = "PascalCase")]
//! enum ActionContent {
//!     TeleportAction(TeleportAction),
//!     SpeedAction(SpeedAction),
//!     LaneChangeAction(LaneChangeAction),
//! }
//!
//! // Use custom deserializer
//! let deserializer = PolymorphicDeserializer::new(&[
//!     "TeleportAction",
//!     "SpeedAction", 
//!     "LaneChangeAction"
//! ]);
//!
//! let action: ActionContent = deserializer.deserialize(xml_element)?;
//! ```
//!
//! # Mixed Attribute/Element Patterns
//!
//! Some OpenSCENARIO elements mix attributes and child elements:
//!
//! ```rust
//! use openscenario_rs::parser::deserializer::MixedContentDeserializer;
//!
//! // For elements like:
//! // <Element attr1="value1" attr2="value2">
//! //   <ChildElement>content</ChildElement>
//! // </Element>
//!
//! #[derive(Deserialize)]
//! struct MixedElement {
//!     #[serde(deserialize_with = "MixedContentDeserializer::deserialize_attributes")]
//!     attributes: HashMap<String, String>,
//!     
//!     #[serde(deserialize_with = "MixedContentDeserializer::deserialize_elements")]
//!     children: Vec<ChildElement>,
//! }
//! ```
//!
//! # Catalog Reference Resolution
//!
//! ```rust
//! use openscenario_rs::parser::deserializer::CatalogReferenceDeserializer;
//! use openscenario_rs::catalog::CatalogResolver;
//!
//! // Set up catalog resolver
//! let mut resolver = CatalogResolver::new();
//! resolver.add_catalog_location("vehicles", "./catalogs/vehicles.xosc");
//!
//! // Deserialize with automatic catalog resolution
//! let deserializer = CatalogReferenceDeserializer::with_resolver(resolver);
//! let vehicle_ref: VehicleCatalogRef = deserializer.deserialize(xml)?;
//!
//! // Reference is automatically resolved to actual vehicle definition
//! let resolved_vehicle = vehicle_ref.resolve()?;
//! ```
//!
//! # Expression Evaluation
//!
//! Mathematical and logical expressions in parameter values:
//!
//! ```rust
//! use openscenario_rs::parser::deserializer::ExpressionEvaluator;
//! use openscenario_rs::types::basic::ParameterContext;
//!
//! let mut context = ParameterContext::new();
//! context.set_parameter("speed", 30.0);
//! context.set_parameter("time", 2.5);
//!
//! let evaluator = ExpressionEvaluator::new();
//!
//! // Evaluate mathematical expressions
//! let result: f64 = evaluator.evaluate("{$speed * $time + 10}", &context)?;
//! assert_eq!(result, 85.0); // 30 * 2.5 + 10
//!
//! // Evaluate conditional expressions
//! let condition: bool = evaluator.evaluate_condition("{$speed > 25}", &context)?;
//! assert_eq!(condition, true);
//! ```
//!
//! # Custom Deserializer Implementation
//!
//! For implementing your own custom deserializers:
//!
//! ```rust
//! use serde::{Deserialize, Deserializer};
//! use openscenario_rs::error::Result;
//!
//! fn custom_deserialize<'de, D>(deserializer: D) -> Result<MyType, D::Error>
//! where
//!     D: Deserializer<'de>,
//! {
//!     // Custom deserialization logic
//!     let raw_value = String::deserialize(deserializer)?;
//!     
//!     // Apply custom parsing logic
//!     MyType::from_string(&raw_value)
//!         .map_err(serde::de::Error::custom)
//! }
//!
//! #[derive(Deserialize)]
//! struct MyStruct {
//!     #[serde(deserialize_with = "custom_deserialize")]
//!     custom_field: MyType,
//! }
//! ```
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
//! All deserializers provide detailed error context:
//!
//! ```rust
//! match parse_parameter_value::<f64>("invalid_expression") {
//!     Ok(value) => {
//!         // Use parsed value
//!     }
//!     Err(e) => {
//!         eprintln!("Deserialization error: {}", e);
//!         // Error includes context about what failed and where
//!     }
//! }
//! ```