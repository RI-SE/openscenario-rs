//! `CatalogReference`: the catalog name, the entry name, and the parameter
//! assignments applied on resolution. Generic over the entry type, so a reference
//! resolves to the type it names and not to a common enum.

use super::entities::CatalogEntity;
use crate::types::basic::Value;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

use crate::types::basic::OSString;

/// Enhanced catalog reference with generic type parameter for type safety
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogReference<T: CatalogEntity> {
    /// Name of the catalog file
    #[serde(rename = "@catalogName")]
    pub catalog_name: OSString,

    /// Name of the entity within the catalog
    #[serde(rename = "@entryName")]
    pub entry_name: OSString,

    /// Parameter assignments for this reference
    #[serde(
        rename = "ParameterAssignments",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_assignments: Option<ParameterAssignments>,

    /// Phantom data to maintain type safety
    #[serde(skip)]
    phantom: PhantomData<T>,
}

/// Container for a catalog reference's parameter assignments.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ParameterAssignments {
    #[serde(rename = "ParameterAssignment", default)]
    pub assignments: Vec<ParameterAssignment>,
}

/// Parameter assignment for catalog reference resolution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterAssignment {
    /// Reference to the parameter name
    #[serde(rename = "@parameterRef")]
    pub parameter_ref: OSString,

    /// Value to assign to the parameter
    #[serde(rename = "@value")]
    pub value: OSString,
}

impl<T: CatalogEntity> CatalogReference<T> {
    /// Create a new catalog reference
    pub fn new(catalog_name: String, entry_name: String) -> Self {
        Self {
            catalog_name: Value::Literal(catalog_name),
            entry_name: Value::Literal(entry_name),
            parameter_assignments: None,
            phantom: PhantomData,
        }
    }

    /// Create a catalog reference with parameter assignments
    pub fn with_parameters(
        catalog_name: String,
        entry_name: String,
        parameter_assignments: Vec<ParameterAssignment>,
    ) -> Self {
        Self {
            catalog_name: Value::Literal(catalog_name),
            entry_name: Value::Literal(entry_name),
            parameter_assignments: Some(ParameterAssignments {
                assignments: parameter_assignments,
            }),
            phantom: PhantomData,
        }
    }
}

impl<T: CatalogEntity> Default for CatalogReference<T> {
    fn default() -> Self {
        Self {
            catalog_name: Value::Literal("DefaultCatalog".to_string()),
            entry_name: Value::Literal("DefaultEntry".to_string()),
            parameter_assignments: None,
            phantom: PhantomData,
        }
    }
}

impl ParameterAssignment {
    /// Create a new parameter assignment
    pub fn new(parameter_ref: String, value: String) -> Self {
        Self {
            parameter_ref: Value::Literal(parameter_ref),
            value: Value::Literal(value),
        }
    }

    /// Create a parameter assignment with parameterized values
    pub fn with_values(parameter_ref: OSString, value: OSString) -> Self {
        Self {
            parameter_ref,
            value,
        }
    }
}

// Type aliases for common catalog reference types
use super::entities::{CatalogController, CatalogManeuver, CatalogPedestrian, CatalogVehicle};
use super::routes::CatalogRoute;

pub type VehicleCatalogReference = CatalogReference<CatalogVehicle>;
pub type ControllerCatalogReference = CatalogReference<CatalogController>;
pub type PedestrianCatalogReference = CatalogReference<CatalogPedestrian>;
pub type RouteCatalogReference = CatalogReference<CatalogRoute>;
pub type ManeuverCatalogReference = CatalogReference<CatalogManeuver>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_catalog_reference_creation() {
        let reference =
            VehicleCatalogReference::new("VehicleCatalog".to_string(), "SportsCar".to_string());

        assert_eq!(
            reference.catalog_name.as_literal().unwrap(),
            "VehicleCatalog"
        );
        assert_eq!(reference.entry_name.as_literal().unwrap(), "SportsCar");
        assert!(reference.parameter_assignments.is_none());
    }

    #[test]
    fn test_catalog_reference_with_parameters() {
        let assignments = vec![
            ParameterAssignment::new("MaxSpeed".to_string(), "200.0".to_string()),
            ParameterAssignment::new("Color".to_string(), "Red".to_string()),
        ];

        let reference = VehicleCatalogReference::with_parameters(
            "VehicleCatalog".to_string(),
            "CustomVehicle".to_string(),
            assignments,
        );

        let assignments = &reference.parameter_assignments.unwrap().assignments;
        assert_eq!(assignments.len(), 2);
        assert_eq!(
            assignments[0].parameter_ref.as_literal().unwrap(),
            "MaxSpeed"
        );
        assert_eq!(assignments[0].value.as_literal().unwrap(), "200.0");
        assert_eq!(assignments[1].parameter_ref.as_literal().unwrap(), "Color");
        assert_eq!(assignments[1].value.as_literal().unwrap(), "Red");
    }

    #[test]
    fn test_catalog_reference_defaults() {
        // `CatalogReference<T>` is generic, so one entry type stands for all of them.
        let reference = VehicleCatalogReference::default();
        assert_eq!(
            reference.catalog_name.as_literal().unwrap(),
            "DefaultCatalog"
        );
        assert_eq!(reference.entry_name.as_literal().unwrap(), "DefaultEntry");
        assert!(reference.parameter_assignments.is_none());
    }

    #[test]
    fn test_parameter_assignment_with_parameters() {
        let assignment = ParameterAssignment::with_values(
            Value::Parameter("ParamNameRef".to_string()),
            Value::Parameter("ParamValueRef".to_string()),
        );

        let mut context_params = HashMap::new();
        context_params.insert("ParamNameRef".to_string(), "DynamicParam".to_string());
        context_params.insert("ParamValueRef".to_string(), "DynamicValue".to_string());

        assert_eq!(
            assignment.parameter_ref.resolve(&context_params).unwrap(),
            "DynamicParam"
        );
        assert_eq!(
            assignment.value.resolve(&context_params).unwrap(),
            "DynamicValue"
        );
    }
}
