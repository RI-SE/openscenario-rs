//! Enhanced catalog reference system with type-safe generics
//!
//! This module provides the enhanced catalog reference system that enables
//! type-safe catalog entity resolution with parameter substitution.

use super::entities::CatalogEntity;
use crate::catalog::{CatalogManager, ResolvedCatalog};
use crate::error::Result;
use crate::types::basic::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::marker::PhantomData;

use crate::types::basic::{Boolean, Double, Int, OSString, UnsignedInt, UnsignedShort};

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
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_assignments: Option<Vec<ParameterAssignment>>,

    /// Phantom data to maintain type safety
    #[serde(skip)]
    phantom: PhantomData<T>,
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

/// Trait for types that can resolve catalog references
#[async_trait::async_trait]
pub trait CatalogResolvable<T: CatalogEntity> {
    /// Resolve this catalog reference to the actual entity
    async fn resolve(&self, manager: &CatalogManager) -> Result<ResolvedCatalog<T::ResolvedType>>;
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
            parameter_assignments: Some(parameter_assignments),
            phantom: PhantomData,
        }
    }

    /// Get the catalog name as a resolved string
    pub fn get_catalog_name(&self, context_params: &HashMap<String, String>) -> Result<String> {
        self.catalog_name.resolve(context_params)
    }

    /// Get the entry name as a resolved string
    pub fn get_entry_name(&self, context_params: &HashMap<String, String>) -> Result<String> {
        self.entry_name.resolve(context_params)
    }

    /// Build parameter map from assignments
    pub fn build_parameter_map(
        &self,
        context_params: &HashMap<String, String>,
    ) -> Result<HashMap<String, String>> {
        let mut parameters = HashMap::new();

        if let Some(assignments) = &self.parameter_assignments {
            for assignment in assignments {
                let param_name = assignment.parameter_ref.resolve(context_params)?;
                let param_value = assignment.value.resolve(context_params)?;
                parameters.insert(param_name, param_value);
            }
        }

        Ok(parameters)
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

#[async_trait::async_trait]
impl<T: CatalogEntity + Send + Sync> CatalogResolvable<T> for CatalogReference<T> {
    async fn resolve(&self, _manager: &CatalogManager) -> Result<ResolvedCatalog<T::ResolvedType>> {
        let context_params = HashMap::new();
        let catalog_name = self.get_catalog_name(&context_params)?;
        let entry_name = self.get_entry_name(&context_params)?;
        let _parameters = self.build_parameter_map(&context_params)?;

        // Placeholder - actual implementation will load from catalog files
        Err(crate::error::Error::catalog_error(&format!(
            "Catalog resolution not yet implemented: {}::{}",
            catalog_name, entry_name
        )))
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

impl Default for ParameterAssignment {
    fn default() -> Self {
        Self {
            parameter_ref: Value::Literal("defaultParam".to_string()),
            value: Value::Literal("defaultValue".to_string()),
        }
    }
}

// Type aliases for common catalog reference types
use super::entities::{CatalogController, CatalogPedestrian, CatalogVehicle};

pub type VehicleCatalogReference = CatalogReference<CatalogVehicle>;
pub type ControllerCatalogReference = CatalogReference<CatalogController>;
pub type PedestrianCatalogReference = CatalogReference<CatalogPedestrian>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog_reference_creation() {
        let reference =
            VehicleCatalogReference::new("VehicleCatalog".to_string(), "SportsCar".to_string());

        let params = HashMap::new();
        assert_eq!(
            reference.get_catalog_name(&params).unwrap(),
            "VehicleCatalog"
        );
        assert_eq!(reference.get_entry_name(&params).unwrap(), "SportsCar");
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

        let context_params = HashMap::new();
        let param_map = reference.build_parameter_map(&context_params).unwrap();

        assert_eq!(param_map.get("MaxSpeed").unwrap(), "200.0");
        assert_eq!(param_map.get("Color").unwrap(), "Red");
    }

    #[test]
    fn test_parameter_assignment() {
        let assignment = ParameterAssignment::new("TestParam".to_string(), "TestValue".to_string());

        let params = HashMap::new();
        assert_eq!(
            assignment.parameter_ref.resolve(&params).unwrap(),
            "TestParam"
        );
        assert_eq!(assignment.value.resolve(&params).unwrap(), "TestValue");
    }

    #[test]
    fn test_parameterized_reference() {
        let reference = VehicleCatalogReference {
            catalog_name: Value::Parameter("CatalogNameParam".to_string()),
            entry_name: Value::Literal("DefaultVehicle".to_string()),
            parameter_assignments: None,
            phantom: PhantomData,
        };

        let mut context_params = HashMap::new();
        context_params.insert("CatalogNameParam".to_string(), "DynamicCatalog".to_string());

        assert_eq!(
            reference.get_catalog_name(&context_params).unwrap(),
            "DynamicCatalog"
        );
        assert_eq!(
            reference.get_entry_name(&context_params).unwrap(),
            "DefaultVehicle"
        );
    }

    #[test]
    fn test_controller_catalog_reference() {
        let reference = ControllerCatalogReference::new(
            "ControllerCatalog".to_string(),
            "AIDriver".to_string(),
        );

        let params = HashMap::new();
        assert_eq!(
            reference.get_catalog_name(&params).unwrap(),
            "ControllerCatalog"
        );
        assert_eq!(reference.get_entry_name(&params).unwrap(), "AIDriver");
    }

    #[test]
    fn test_pedestrian_catalog_reference() {
        let reference = PedestrianCatalogReference::new(
            "PedestrianCatalog".to_string(),
            "WalkingPerson".to_string(),
        );

        let params = HashMap::new();
        assert_eq!(
            reference.get_catalog_name(&params).unwrap(),
            "PedestrianCatalog"
        );
        assert_eq!(reference.get_entry_name(&params).unwrap(), "WalkingPerson");
    }

    #[test]
    fn test_catalog_reference_defaults() {
        let vehicle_ref = VehicleCatalogReference::default();
        let controller_ref = ControllerCatalogReference::default();
        let pedestrian_ref = PedestrianCatalogReference::default();

        let params = HashMap::new();

        // All should have valid default values
        assert!(vehicle_ref.get_catalog_name(&params).is_ok());
        assert!(controller_ref.get_catalog_name(&params).is_ok());
        assert!(pedestrian_ref.get_catalog_name(&params).is_ok());
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

    #[test]
    fn test_build_parameter_map_with_parameters() {
        let assignments = vec![
            ParameterAssignment::with_values(
                Value::Literal("Speed".to_string()),
                Value::Parameter("SpeedParam".to_string()),
            ),
            ParameterAssignment::with_values(
                Value::Parameter("ColorParamName".to_string()),
                Value::Literal("Blue".to_string()),
            ),
        ];

        let reference = VehicleCatalogReference::with_parameters(
            "TestCatalog".to_string(),
            "TestVehicle".to_string(),
            assignments,
        );

        let mut context_params = HashMap::new();
        context_params.insert("SpeedParam".to_string(), "150.0".to_string());
        context_params.insert("ColorParamName".to_string(), "Color".to_string());

        let param_map = reference.build_parameter_map(&context_params).unwrap();

        assert_eq!(param_map.get("Speed").unwrap(), "150.0");
        assert_eq!(param_map.get("Color").unwrap(), "Blue");
    }
}
