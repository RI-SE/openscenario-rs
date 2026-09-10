//! Vehicle entity builder with fluent API

use crate::types::basic::Value;
use crate::types::{
    basic::{Double, OSString},
    entities::axles::Axles,
    entities::vehicle::{Performance, Properties},
    entities::{ScenarioObject, Vehicle},
    enums::VehicleCategory,
    geometry::{BoundingBox, Center, Dimensions},
};

/// Builder for vehicle entities with position integration
pub struct VehicleBuilder<'parent> {
    parent: &'parent mut crate::builder::scenario::ScenarioBuilder<
        crate::builder::scenario::HasEntities,
    >,
    name: String,
    vehicle_data: PartialVehicleData,
}

/// Detached vehicle builder that doesn't hold references
pub struct DetachedVehicleBuilder {
    name: String,
    vehicle_data: PartialVehicleData,
}

#[derive(Debug, Default)]
struct PartialVehicleData {
    name: Option<String>,
    vehicle_category: Option<Value<VehicleCategory>>,
    properties: Option<Properties>,
    bounding_box: Option<BoundingBox>,
    performance: Option<Performance>,
    axles: Option<Axles>,
}

impl<'parent> VehicleBuilder<'parent> {
    pub fn new(
        parent: &'parent mut crate::builder::scenario::ScenarioBuilder<
            crate::builder::scenario::HasEntities,
        >,
        name: &str,
    ) -> Self {
        Self {
            parent,
            name: name.to_string(),
            vehicle_data: PartialVehicleData::default(),
        }
    }

    /// Set vehicle as passenger car
    pub fn car(mut self) -> Self {
        self.vehicle_data.vehicle_category = Some(Value::Literal(VehicleCategory::Car));
        self.vehicle_data.name = Some("PassengerCar".to_string());

        // Default car dimensions
        self.vehicle_data.bounding_box = Some(BoundingBox {
            center: Center {
                x: Double::literal(1.4),
                y: Double::literal(0.0),
                z: Double::literal(0.9),
            },
            dimensions: Dimensions {
                width: Double::literal(1.8),
                length: Double::literal(4.5),
                height: Double::literal(1.4),
            },
        });

        // Default car performance
        self.vehicle_data.performance = Some(Performance {
            max_speed: Double::literal(200.0),
            max_acceleration: Double::literal(10.0),
            max_acceleration_rate: None,
            max_deceleration: Double::literal(10.0),
            max_deceleration_rate: None,
        });

        // Default car axles
        self.vehicle_data.axles = Some(Axles::car());

        self
    }

    /// Set the vehicle category to a literal enum value.
    ///
    /// The literal case keeps the bare enum, so existing call sites are unchanged; see
    /// [`Self::with_category_param`] for the `$name` parameter form the schema also allows
    /// on this attribute.
    pub fn with_category(mut self, category: VehicleCategory) -> Self {
        self.vehicle_data.vehicle_category = Some(Value::Literal(category));
        self
    }

    /// Set the vehicle category to a parameter reference.
    ///
    /// `@vehicleCategory` is declared with an `xsd:union` whose second member is
    /// `<xsd:restriction base="parameter"/>`, so `vehicleCategory="$cat"` is schema-valid;
    /// `name` is the bare parameter name, without the `$`.
    pub fn with_category_param(mut self, name: &str) -> Self {
        self.vehicle_data.vehicle_category = Some(Value::Parameter(name.to_string()));
        self
    }

    /// Set vehicle as truck
    pub fn truck(mut self) -> Self {
        self.vehicle_data.vehicle_category = Some(Value::Literal(VehicleCategory::Truck));
        self.vehicle_data.name = Some("Truck".to_string());

        // Default truck dimensions
        self.vehicle_data.bounding_box = Some(BoundingBox {
            center: Center {
                x: Double::literal(4.0),
                y: Double::literal(0.0),
                z: Double::literal(1.5),
            },
            dimensions: Dimensions {
                width: Double::literal(2.5),
                length: Double::literal(8.0),
                height: Double::literal(3.0),
            },
        });

        // Default truck performance
        self.vehicle_data.performance = Some(Performance {
            max_speed: Double::literal(120.0),
            max_acceleration: Double::literal(3.0),
            max_acceleration_rate: None,
            max_deceleration: Double::literal(8.0),
            max_deceleration_rate: None,
        });

        // Default truck axles
        self.vehicle_data.axles = Some(Axles::truck());

        self
    }

    /// Set custom dimensions
    pub fn with_dimensions(mut self, length: f64, width: f64, height: f64) -> Self {
        // No `BoundingBox::default()`: XSD `Center` and `Dimensions` are both
        // `xsd:all` of required attributes with no schema default
        // (`Schema/OpenSCENARIO.xsd:886-890,1058-1062`). If a center was already set
        // via a category preset, keep it; otherwise the origin is the explicit
        // fallback, not an invented one.
        let existing_center = self
            .vehicle_data
            .bounding_box
            .as_ref()
            .map(|bbox| bbox.center.clone())
            .unwrap_or_else(|| Center::new(0.0, 0.0, 0.0));

        self.vehicle_data.bounding_box = Some(BoundingBox {
            center: existing_center,
            dimensions: Dimensions {
                width: Double::literal(width),
                length: Double::literal(length),
                height: Double::literal(height),
            },
        });

        self
    }

    /// Set custom performance characteristics
    pub fn with_performance(
        mut self,
        max_speed: f64,
        max_acceleration: f64,
        max_deceleration: f64,
    ) -> Self {
        self.vehicle_data.performance = Some(Performance {
            max_speed: Double::literal(max_speed),
            max_acceleration: Double::literal(max_acceleration),
            max_acceleration_rate: None,
            max_deceleration: Double::literal(max_deceleration),
            max_deceleration_rate: None,
        });
        self
    }

    /// Finish vehicle and add to scenario
    pub fn finish(
        self,
    ) -> &'parent mut crate::builder::scenario::ScenarioBuilder<crate::builder::scenario::HasEntities>
    {
        let vehicle = Vehicle {
            name: OSString::literal(
                self.vehicle_data
                    .name
                    .unwrap_or_else(|| "DefaultVehicle".to_string()),
            ),
            vehicle_category: self
                .vehicle_data
                .vehicle_category
                .unwrap_or(Value::Literal(VehicleCategory::Car)),
            role: None,
            mass: None,
            model3d: None,
            parameter_declarations: None,
            bounding_box: self
                .vehicle_data
                .bounding_box
                .unwrap_or_else(|| BoundingBox {
                    center: Center::new(0.0, 0.0, 0.0),
                    dimensions: Dimensions {
                        width: Double::literal(2.0),
                        length: Double::literal(4.5),
                        height: Double::literal(1.5),
                    },
                }),
            performance: self
                .vehicle_data
                .performance
                .unwrap_or_else(|| Performance {
                    max_speed: Double::literal(200.0),
                    max_acceleration: Double::literal(10.0),
                    max_acceleration_rate: None,
                    max_deceleration: Double::literal(10.0),
                    max_deceleration_rate: None,
                }),
            axles: self.vehicle_data.axles.unwrap_or_else(|| Axles::car()),
            properties: self.vehicle_data.properties,
            trailer_hitch: None,
            trailer_coupler: None,
            trailer: None,
        };

        let scenario_object = ScenarioObject::new_vehicle(self.name.clone(), vehicle);

        // Add to parent's entities
        if let Some(ref mut entities) = self.parent.data.entities {
            entities.add_object(scenario_object);
        }

        self.parent
    }

    /// Convert to detached builder for closure-based configuration
    pub fn detached(self) -> DetachedVehicleBuilder {
        DetachedVehicleBuilder {
            name: self.name,
            vehicle_data: self.vehicle_data,
        }
    }
}

impl DetachedVehicleBuilder {
    /// Create a new detached vehicle builder
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            vehicle_data: PartialVehicleData::default(),
        }
    }

    /// Set vehicle as passenger car
    pub fn car(mut self) -> Self {
        self.vehicle_data.vehicle_category = Some(Value::Literal(VehicleCategory::Car));
        self.vehicle_data.name = Some("PassengerCar".to_string());

        // Default car dimensions
        self.vehicle_data.bounding_box = Some(BoundingBox {
            center: Center {
                x: Double::literal(1.4),
                y: Double::literal(0.0),
                z: Double::literal(0.9),
            },
            dimensions: Dimensions {
                width: Double::literal(1.8),
                length: Double::literal(4.5),
                height: Double::literal(1.4),
            },
        });

        // Default car performance
        self.vehicle_data.performance = Some(Performance {
            max_speed: Double::literal(200.0),
            max_acceleration: Double::literal(10.0),
            max_acceleration_rate: None,
            max_deceleration: Double::literal(10.0),
            max_deceleration_rate: None,
        });

        // Default car axles
        self.vehicle_data.axles = Some(Axles::car());

        self
    }

    /// Set the vehicle category to a literal enum value.
    ///
    /// The literal case keeps the bare enum, so existing call sites are unchanged; see
    /// [`Self::with_category_param`] for the `$name` parameter form the schema also allows
    /// on this attribute.
    pub fn with_category(mut self, category: VehicleCategory) -> Self {
        self.vehicle_data.vehicle_category = Some(Value::Literal(category));
        self
    }

    /// Set the vehicle category to a parameter reference.
    ///
    /// `@vehicleCategory` is declared with an `xsd:union` whose second member is
    /// `<xsd:restriction base="parameter"/>`, so `vehicleCategory="$cat"` is schema-valid;
    /// `name` is the bare parameter name, without the `$`.
    pub fn with_category_param(mut self, name: &str) -> Self {
        self.vehicle_data.vehicle_category = Some(Value::Parameter(name.to_string()));
        self
    }

    /// Set vehicle as truck
    pub fn truck(mut self) -> Self {
        self.vehicle_data.vehicle_category = Some(Value::Literal(VehicleCategory::Truck));
        self.vehicle_data.name = Some("Truck".to_string());

        // Default truck dimensions
        self.vehicle_data.bounding_box = Some(BoundingBox {
            center: Center {
                x: Double::literal(4.0),
                y: Double::literal(0.0),
                z: Double::literal(1.5),
            },
            dimensions: Dimensions {
                width: Double::literal(2.5),
                length: Double::literal(8.0),
                height: Double::literal(3.0),
            },
        });

        // Default truck performance
        self.vehicle_data.performance = Some(Performance {
            max_speed: Double::literal(120.0),
            max_acceleration: Double::literal(3.0),
            max_acceleration_rate: None,
            max_deceleration: Double::literal(8.0),
            max_deceleration_rate: None,
        });

        // Default truck axles
        self.vehicle_data.axles = Some(Axles::truck());

        self
    }

    /// Set custom dimensions
    pub fn with_dimensions(mut self, length: f64, width: f64, height: f64) -> Self {
        // No `BoundingBox::default()`: XSD `Center` and `Dimensions` are both
        // `xsd:all` of required attributes with no schema default
        // (`Schema/OpenSCENARIO.xsd:886-890,1058-1062`). If a center was already set
        // via a category preset, keep it; otherwise the origin is the explicit
        // fallback, not an invented one.
        let existing_center = self
            .vehicle_data
            .bounding_box
            .as_ref()
            .map(|bbox| bbox.center.clone())
            .unwrap_or_else(|| Center::new(0.0, 0.0, 0.0));

        self.vehicle_data.bounding_box = Some(BoundingBox {
            center: existing_center,
            dimensions: Dimensions {
                width: Double::literal(width),
                length: Double::literal(length),
                height: Double::literal(height),
            },
        });

        self
    }

    /// Set custom performance characteristics
    pub fn with_performance(
        mut self,
        max_speed: f64,
        max_acceleration: f64,
        max_deceleration: f64,
    ) -> Self {
        self.vehicle_data.performance = Some(Performance {
            max_speed: Double::literal(max_speed),
            max_acceleration: Double::literal(max_acceleration),
            max_acceleration_rate: None,
            max_deceleration: Double::literal(max_deceleration),
            max_deceleration_rate: None,
        });
        self
    }

    /// Build the vehicle object
    pub fn build(self) -> ScenarioObject {
        let vehicle = Vehicle {
            name: OSString::literal(
                self.vehicle_data
                    .name
                    .unwrap_or_else(|| "DefaultVehicle".to_string()),
            ),
            vehicle_category: self
                .vehicle_data
                .vehicle_category
                .unwrap_or(Value::Literal(VehicleCategory::Car)),
            role: None,
            mass: None,
            model3d: None,
            parameter_declarations: None,
            bounding_box: self
                .vehicle_data
                .bounding_box
                .unwrap_or_else(|| BoundingBox {
                    center: Center::new(0.0, 0.0, 0.0),
                    dimensions: Dimensions {
                        width: Double::literal(2.0),
                        length: Double::literal(4.5),
                        height: Double::literal(1.5),
                    },
                }),
            performance: self
                .vehicle_data
                .performance
                .unwrap_or_else(|| Performance {
                    max_speed: Double::literal(200.0),
                    max_acceleration: Double::literal(10.0),
                    max_acceleration_rate: None,
                    max_deceleration: Double::literal(10.0),
                    max_deceleration_rate: None,
                }),
            axles: self.vehicle_data.axles.unwrap_or_else(|| Axles::car()),
            properties: self.vehicle_data.properties,
            trailer_hitch: None,
            trailer_coupler: None,
            trailer: None,
        };

        ScenarioObject::new_vehicle(self.name.clone(), vehicle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detached_builder_defaults_when_no_preset_called() {
        let obj = DetachedVehicleBuilder::new("ego").build();
        let v = obj.vehicle.as_ref().unwrap();
        assert_eq!(v.name.as_literal(), Some(&"DefaultVehicle".to_string()));
        assert_eq!(v.vehicle_category, Value::Literal(VehicleCategory::Car));
    }

    #[test]
    fn test_car_preset_sets_category_and_dimensions() {
        let obj = DetachedVehicleBuilder::new("ego").car().build();
        let v = obj.vehicle.as_ref().unwrap();
        assert_eq!(v.vehicle_category, Value::Literal(VehicleCategory::Car));
        assert_eq!(v.name.as_literal(), Some(&"PassengerCar".to_string()));
        assert_eq!(v.bounding_box.dimensions.length.as_literal(), Some(&4.5));
    }

    #[test]
    fn category_setters_cover_both_the_literal_and_the_parameter_case() {
        // OSR-06: the literal setter keeps the bare enum, so no existing call site
        // changed; the parallel `_param` setter reaches the `$name` form that the
        // attribute's `xsd:union` also admits.
        let literal = DetachedVehicleBuilder::new("ego")
            .car()
            .with_category(VehicleCategory::Van)
            .build();
        assert_eq!(
            literal.vehicle.as_ref().unwrap().vehicle_category,
            Value::Literal(VehicleCategory::Van)
        );

        let parameterized = DetachedVehicleBuilder::new("ego")
            .car()
            .with_category_param("cat")
            .build();
        assert_eq!(
            parameterized.vehicle.as_ref().unwrap().vehicle_category,
            Value::Parameter("cat".to_string())
        );
    }

    #[test]
    fn test_truck_preset_overrides_car_preset() {
        let obj = DetachedVehicleBuilder::new("ego").car().truck().build();
        let v = obj.vehicle.as_ref().unwrap();
        assert_eq!(v.vehicle_category, Value::Literal(VehicleCategory::Truck));
        assert_eq!(v.bounding_box.dimensions.length.as_literal(), Some(&8.0));
        assert_eq!(v.performance.max_speed.as_literal(), Some(&120.0));
    }

    #[test]
    fn test_with_dimensions_overrides_preset_but_preserves_center() {
        let obj = DetachedVehicleBuilder::new("ego")
            .car()
            .with_dimensions(5.0, 2.0, 1.6)
            .build();
        let v = obj.vehicle.as_ref().unwrap();
        assert_eq!(v.bounding_box.dimensions.length.as_literal(), Some(&5.0));
        assert_eq!(v.bounding_box.dimensions.width.as_literal(), Some(&2.0));
        assert_eq!(v.bounding_box.dimensions.height.as_literal(), Some(&1.6));
        // Center preserved from car preset
        assert_eq!(v.bounding_box.center.x.as_literal(), Some(&1.4));
    }

    #[test]
    fn test_with_performance_overrides_preset() {
        let obj = DetachedVehicleBuilder::new("ego")
            .truck()
            .with_performance(200.0, 5.0, 10.0)
            .build();
        let v = obj.vehicle.as_ref().unwrap();
        assert_eq!(v.performance.max_speed.as_literal(), Some(&200.0));
        assert_eq!(v.performance.max_acceleration.as_literal(), Some(&5.0));
        assert_eq!(v.performance.max_deceleration.as_literal(), Some(&10.0));
    }
}
