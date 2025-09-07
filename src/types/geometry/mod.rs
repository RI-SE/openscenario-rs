//! Geometry types for OpenSCENARIO shapes and spatial definitions

pub mod shapes;

// Re-export commonly used geometry types
pub use shapes::{BoundingBox, Center, Dimensions, Shape, Polyline, Vertex};