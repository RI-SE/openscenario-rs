//! Pedestrian entity builder with fluent API

use crate::types::{
    basic::{Double, OSString},
    entities::ScenarioObject,
    enums::{PedestrianCategory, Role},
    geometry::{BoundingBox, Center, Dimensions},
};

/// Builder for pedestrian entities with position integration
pub struct PedestrianBuilder<'parent> {
    parent: &'parent mut crate::builder::scenario::ScenarioBuilder<
        crate::builder::scenario::HasEntities,
    >,
    name: String,
    pedestrian_data: PartialPedestrianData,
}

/// Detached pedestrian builder that doesn't hold references
pub struct DetachedPedestrianBuilder {
    name: String,
    pedestrian_data: PartialPedestrianData,
}

#[derive(Debug, Default)]
struct PartialPedestrianData {
    name: Option<String>,
    pedestrian_category: Option<PedestrianCategory>,
    mass: Option<Double>,
    role: Option<Role>,
    model3d: Option<String>,
    bounding_box: Option<BoundingBox>,
    properties: Option<crate::types::entities::vehicle::Properties>,
}

impl<'parent> PedestrianBuilder<'parent> {
    pub fn new(
        parent: &'parent mut crate::builder::scenario::ScenarioBuilder<
            crate::builder::scenario::HasEntities,
        >,
        name: &str,
    ) -> Self {
        Self {
            parent,
            name: name.to_string(),
            pedestrian_data: PartialPedestrianData::default(),
        }
    }

    /// Set pedestrian as standard pedestrian
    pub fn pedestrian(mut self) -> Self {
        self.pedestrian_data.pedestrian_category = Some(PedestrianCategory::Pedestrian);
        self.pedestrian_data.name = Some("StandardPedestrian".to_string());

        // Default pedestrian dimensions
        self.pedestrian_data.bounding_box = Some(BoundingBox {
            center: Center {
                x: Double::literal(0.0),
                y: Double::literal(0.0),
                z: Double::literal(0.9),
            },
            dimensions: Dimensions {
                width: Double::literal(0.6),
                length: Double::literal(0.6),
                height: Double::literal(1.8),
            },
        });

        self
    }

    /// Set pedestrian as wheelchair user
    pub fn wheelchair(mut self) -> Self {
        self.pedestrian_data.pedestrian_category = Some(PedestrianCategory::Wheelchair);
        self.pedestrian_data.name = Some("Wheelchair".to_string());

        // Default wheelchair dimensions
        self.pedestrian_data.bounding_box = Some(BoundingBox {
            center: Center {
                x: Double::literal(0.0),
                y: Double::literal(0.0),
                z: Double::literal(0.7),
            },
            dimensions: Dimensions {
                width: Double::literal(0.8),
                length: Double::literal(1.0),
                height: Double::literal(1.4),
            },
        });

        self
    }

    /// Set pedestrian as animal
    pub fn animal(mut self) -> Self {
        self.pedestrian_data.pedestrian_category = Some(PedestrianCategory::Animal);
        self.pedestrian_data.name = Some("Animal".to_string());

        // Default animal dimensions
        self.pedestrian_data.bounding_box = Some(BoundingBox {
            center: Center {
                x: Double::literal(0.0),
                y: Double::literal(0.0),
                z: Double::literal(0.4),
            },
            dimensions: Dimensions {
                width: Double::literal(0.5),
                length: Double::literal(0.5),
                height: Double::literal(0.8),
            },
        });

        self
    }

    /// Set custom mass (XSD required field)
    pub fn with_mass(mut self, mass: f64) -> Self {
        self.pedestrian_data.mass = Some(Double::literal(mass));
        self
    }

    /// Set custom dimensions
    pub fn with_dimensions(mut self, length: f64, width: f64, height: f64) -> Self {
        let existing_bbox = self.pedestrian_data.bounding_box.unwrap_or_default();

        self.pedestrian_data.bounding_box = Some(BoundingBox {
            center: existing_bbox.center,
            dimensions: Dimensions {
                width: Double::literal(width),
                length: Double::literal(length),
                height: Double::literal(height),
            },
        });

        self
    }

    /// Set role (civil, police, ambulance, etc.)
    pub fn with_role(mut self, role: Role) -> Self {
        self.pedestrian_data.role = Some(role);
        self
    }

    /// Set 3D model path
    pub fn with_model3d(mut self, model: &str) -> Self {
        self.pedestrian_data.model3d = Some(model.to_string());
        self
    }

    /// Finish pedestrian and add to scenario
    pub fn finish(
        self,
    ) -> &'parent mut crate::builder::scenario::ScenarioBuilder<crate::builder::scenario::HasEntities>
    {
        let pedestrian = crate::types::entities::Pedestrian {
            name: OSString::literal(
                self.pedestrian_data
                    .name
                    .unwrap_or_else(|| "DefaultPedestrian".to_string()),
            ),
            pedestrian_category: self
                .pedestrian_data
                .pedestrian_category
                .unwrap_or(PedestrianCategory::Pedestrian),
            mass: self
                .pedestrian_data
                .mass
                .unwrap_or_else(|| Double::literal(75.0)),
            role: self.pedestrian_data.role,
            model3d: self.pedestrian_data.model3d,
            bounding_box: self.pedestrian_data.bounding_box.unwrap_or_default(),
            properties: self.pedestrian_data.properties,
            parameter_declarations: None,
        };

        let scenario_object = ScenarioObject::new_pedestrian(self.name.clone(), pedestrian);

        // Add to parent's entities
        if let Some(ref mut entities) = self.parent.data.entities {
            entities.add_object(scenario_object);
        }

        self.parent
    }
}

impl DetachedPedestrianBuilder {
    /// Create a new detached pedestrian builder
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            pedestrian_data: PartialPedestrianData::default(),
        }
    }

    /// Set pedestrian as standard pedestrian
    pub fn pedestrian(mut self) -> Self {
        self.pedestrian_data.pedestrian_category = Some(PedestrianCategory::Pedestrian);
        self.pedestrian_data.name = Some("StandardPedestrian".to_string());

        self.pedestrian_data.bounding_box = Some(BoundingBox {
            center: Center {
                x: Double::literal(0.0),
                y: Double::literal(0.0),
                z: Double::literal(0.9),
            },
            dimensions: Dimensions {
                width: Double::literal(0.6),
                length: Double::literal(0.6),
                height: Double::literal(1.8),
            },
        });

        self
    }

    /// Set pedestrian as wheelchair user
    pub fn wheelchair(mut self) -> Self {
        self.pedestrian_data.pedestrian_category = Some(PedestrianCategory::Wheelchair);
        self.pedestrian_data.name = Some("Wheelchair".to_string());

        self.pedestrian_data.bounding_box = Some(BoundingBox {
            center: Center {
                x: Double::literal(0.0),
                y: Double::literal(0.0),
                z: Double::literal(0.7),
            },
            dimensions: Dimensions {
                width: Double::literal(0.8),
                length: Double::literal(1.0),
                height: Double::literal(1.4),
            },
        });

        self
    }

    /// Set pedestrian as animal
    pub fn animal(mut self) -> Self {
        self.pedestrian_data.pedestrian_category = Some(PedestrianCategory::Animal);
        self.pedestrian_data.name = Some("Animal".to_string());

        self.pedestrian_data.bounding_box = Some(BoundingBox {
            center: Center {
                x: Double::literal(0.0),
                y: Double::literal(0.0),
                z: Double::literal(0.4),
            },
            dimensions: Dimensions {
                width: Double::literal(0.5),
                length: Double::literal(0.5),
                height: Double::literal(0.8),
            },
        });

        self
    }

    /// Set custom mass
    pub fn with_mass(mut self, mass: f64) -> Self {
        self.pedestrian_data.mass = Some(Double::literal(mass));
        self
    }

    /// Set custom dimensions
    pub fn with_dimensions(mut self, length: f64, width: f64, height: f64) -> Self {
        let existing_bbox = self.pedestrian_data.bounding_box.unwrap_or_default();

        self.pedestrian_data.bounding_box = Some(BoundingBox {
            center: existing_bbox.center,
            dimensions: Dimensions {
                width: Double::literal(width),
                length: Double::literal(length),
                height: Double::literal(height),
            },
        });

        self
    }

    /// Set role
    pub fn with_role(mut self, role: Role) -> Self {
        self.pedestrian_data.role = Some(role);
        self
    }

    /// Set 3D model path
    pub fn with_model3d(mut self, model: &str) -> Self {
        self.pedestrian_data.model3d = Some(model.to_string());
        self
    }

    /// Build the pedestrian object (returns ScenarioObject)
    pub fn build(self) -> ScenarioObject {
        let pedestrian = crate::types::entities::Pedestrian {
            name: OSString::literal(
                self.pedestrian_data
                    .name
                    .unwrap_or_else(|| "DefaultPedestrian".to_string()),
            ),
            pedestrian_category: self
                .pedestrian_data
                .pedestrian_category
                .unwrap_or(PedestrianCategory::Pedestrian),
            mass: self
                .pedestrian_data
                .mass
                .unwrap_or_else(|| Double::literal(75.0)),
            role: self.pedestrian_data.role,
            model3d: self.pedestrian_data.model3d,
            bounding_box: self.pedestrian_data.bounding_box.unwrap_or_default(),
            properties: self.pedestrian_data.properties,
            parameter_declarations: None,
        };

        ScenarioObject::new_pedestrian(self.name.clone(), pedestrian)
    }
}
