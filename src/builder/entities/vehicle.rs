//! Vehicle entity builder with fluent API

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
    vehicle_category: Option<VehicleCategory>,
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
        self.vehicle_data.vehicle_category = Some(VehicleCategory::Car);
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
        self.vehicle_data.performance = Some(Performance::default());

        // Default car axles
        self.vehicle_data.axles = Some(Axles::car());

        self
    }

    /// Set vehicle as truck
    pub fn truck(mut self) -> Self {
        self.vehicle_data.vehicle_category = Some(VehicleCategory::Truck);
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
            max_deceleration: Double::literal(8.0),
        });

        // Default truck axles
        self.vehicle_data.axles = Some(Axles::truck());

        self
    }

    /// Set custom dimensions
    pub fn with_dimensions(mut self, length: f64, width: f64, height: f64) -> Self {
        let existing_bbox = self.vehicle_data.bounding_box.unwrap_or_default();

        self.vehicle_data.bounding_box = Some(BoundingBox {
            center: existing_bbox.center,
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
            max_deceleration: Double::literal(max_deceleration),
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
                .unwrap_or(VehicleCategory::Car),
            bounding_box: self.vehicle_data.bounding_box.unwrap_or_default(),
            performance: self.vehicle_data.performance.unwrap_or_default(),
            axles: self.vehicle_data.axles.unwrap_or_else(|| Axles::car()),
            properties: self.vehicle_data.properties,
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
        self.vehicle_data.vehicle_category = Some(VehicleCategory::Car);
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
        self.vehicle_data.performance = Some(Performance::default());

        // Default car axles
        self.vehicle_data.axles = Some(Axles::car());

        self
    }

    /// Set vehicle as truck
    pub fn truck(mut self) -> Self {
        self.vehicle_data.vehicle_category = Some(VehicleCategory::Truck);
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
            max_deceleration: Double::literal(8.0),
        });

        // Default truck axles
        self.vehicle_data.axles = Some(Axles::truck());

        self
    }

    /// Set custom dimensions
    pub fn with_dimensions(mut self, length: f64, width: f64, height: f64) -> Self {
        let existing_bbox = self.vehicle_data.bounding_box.unwrap_or_default();

        self.vehicle_data.bounding_box = Some(BoundingBox {
            center: existing_bbox.center,
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
            max_deceleration: Double::literal(max_deceleration),
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
                .unwrap_or(VehicleCategory::Car),
            bounding_box: self.vehicle_data.bounding_box.unwrap_or_default(),
            performance: self.vehicle_data.performance.unwrap_or_default(),
            axles: self.vehicle_data.axles.unwrap_or_else(|| Axles::car()),
            properties: self.vehicle_data.properties,
        };

        ScenarioObject::new_vehicle(self.name.clone(), vehicle)
    }
}
