//! Trajectory catalog types for OpenSCENARIO reusable trajectory definitions
//!
//! This module contains catalog-specific trajectory types that enable reuse of
//! trajectory definitions across multiple scenarios with parameter substitution.

use crate::types::basic::{Boolean, Double, Int, OSString, ParameterDeclarations, Value};
use crate::types::positions::Position;
use serde::{Deserialize, Serialize};

/// Trajectory definition within a catalog
///
/// Extends the base Trajectory type with catalog-specific functionality
/// including parameter declarations and reusable shape definitions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trajectory")]
pub struct CatalogTrajectory {
    /// Unique name for this trajectory in the catalog
    #[serde(rename = "@name")]
    pub name: String,

    /// Whether the trajectory is closed (forms a loop) — required per XSD
    #[serde(rename = "@closed")]
    pub closed: Boolean,

    /// Parameter declarations for this trajectory
    #[serde(
        rename = "ParameterDeclarations",
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<ParameterDeclarations>,

    /// Shape definition of the trajectory
    #[serde(rename = "Shape")]
    pub shape: CatalogShape,
}

impl Default for CatalogTrajectory {
    fn default() -> Self {
        Self {
            name: "DefaultCatalogTrajectory".to_string(),
            closed: Value::Literal(false),
            parameter_declarations: None,
            shape: CatalogShape::new(CatalogTrajectoryShape::Polyline(CatalogPolyline {
                vertices: Vec::new(),
            })),
        }
    }
}

/// Wrapper for the `<Shape>` element of a catalog trajectory.
///
/// The XSD models `<Shape>` as a container holding exactly one of
/// `<Polyline>`, `<Clothoid>` or `<Nurbs>`.  quick-xml maps an externally
/// tagged enum onto the *field's* element name, so the enum needs this
/// `$value` wrapper to be nested inside `<Shape>` rather than replacing it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Shape")]
pub struct CatalogShape {
    /// The concrete shape variant carried by this `<Shape>` element
    #[serde(rename = "$value")]
    pub shape: CatalogTrajectoryShape,
}

impl CatalogShape {
    /// Wraps a shape variant in a `<Shape>` container
    pub fn new(shape: CatalogTrajectoryShape) -> Self {
        Self { shape }
    }
}

impl From<CatalogTrajectoryShape> for CatalogShape {
    fn from(shape: CatalogTrajectoryShape) -> Self {
        Self { shape }
    }
}

/// Shape of a catalog trajectory (polyline, clothoid, NURBS, etc.)
///
/// Extends the base TrajectoryShape with parameterization capabilities
/// and catalog-specific features.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum CatalogTrajectoryShape {
    /// Polyline trajectory with parameterizable vertices
    Polyline(CatalogPolyline),
    /// Clothoid-based trajectory with parameterizable curvature
    Clothoid(CatalogClothoid),
    /// Spline of chained clothoid segments
    ClothoidSpline(crate::types::geometry::shapes::ClothoidSpline),
    /// NURBS-based trajectory with control points
    Nurbs(CatalogNurbs),
}

/// Polyline trajectory with parameterizable vertices
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Polyline")]
pub struct CatalogPolyline {
    /// Vertices defining the polyline (can be parameterized)
    #[serde(rename = "Vertex")]
    pub vertices: Vec<CatalogVertex>,
}

/// Vertex in a catalog trajectory with parameterizable properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Vertex")]
pub struct CatalogVertex {
    /// Time at this vertex (optional, can be parameterized)
    #[serde(rename = "@time", skip_serializing_if = "Option::is_none")]
    pub time: Option<Double>,

    /// Position at this vertex (can be parameterized)
    #[serde(rename = "Position")]
    pub position: Position,
}

/// Clothoid trajectory segment with parameterizable properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Clothoid")]
pub struct CatalogClothoid {
    /// Curvature at start (can be parameterized)
    #[serde(rename = "@curvature")]
    pub curvature: Double,

    /// Curvature derivative - clothoid parameter (can be parameterized) — deprecated per XSD
    #[serde(rename = "@curvatureDot", default, skip_serializing_if = "Option::is_none")]
    pub curvature_dot: Option<Double>,

    /// Curvature derivative (current replacement for `curvatureDot`, can be parameterized)
    #[serde(
        rename = "@curvaturePrime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub curvature_prime: Option<Double>,

    /// Length of the clothoid (can be parameterized)
    #[serde(rename = "@length")]
    pub length: Double,

    /// Start time (can be parameterized)
    #[serde(rename = "@startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Double>,

    /// Stop time (can be parameterized)
    #[serde(rename = "@stopTime", default, skip_serializing_if = "Option::is_none")]
    pub stop_time: Option<Double>,

    /// Start position (required per XSD)
    #[serde(rename = "Position")]
    pub start_position: Position,
}

/// NURBS (Non-Uniform Rational B-Spline) trajectory definition
///
/// Provides precise mathematical curve representation for complex trajectories.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Nurbs")]
pub struct CatalogNurbs {
    /// Order of the NURBS curve (degree + 1)
    #[serde(rename = "@order")]
    pub order: Int,

    /// Control points defining the NURBS curve
    #[serde(rename = "ControlPoint")]
    pub control_points: Vec<NurbsControlPoint>,

    /// Knot vector for the NURBS curve
    #[serde(rename = "Knot")]
    pub knots: Vec<NurbsKnot>,
}

/// Control point for NURBS trajectory
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "ControlPoint")]
pub struct NurbsControlPoint {
    /// Position of the control point
    #[serde(rename = "Position")]
    pub position: Position,

    /// Time at the control point (optional, can be parameterized)
    #[serde(rename = "@time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<Double>,

    /// Weight of the control point (for rational NURBS)
    #[serde(rename = "@weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<Double>,
}

/// Knot value for NURBS trajectory
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Knot")]
pub struct NurbsKnot {
    /// Knot value
    #[serde(rename = "@value")]
    pub value: Double,
}

// Implementation methods for catalog trajectories

impl CatalogTrajectory {
    /// Creates a new catalog trajectory with the specified name
    pub fn new(name: String, shape: CatalogTrajectoryShape) -> Self {
        Self {
            name,
            closed: Value::Literal(false),
            parameter_declarations: None,
            shape: shape.into(),
        }
    }

    /// Creates a catalog trajectory with parameter declarations
    pub fn with_parameters(
        name: String,
        shape: CatalogTrajectoryShape,
        parameters: ParameterDeclarations,
    ) -> Self {
        Self {
            name,
            closed: Value::Literal(false),
            parameter_declarations: Some(parameters),
            shape: shape.into(),
        }
    }

    /// Creates a closed trajectory (forms a loop)
    pub fn with_closed(name: String, shape: CatalogTrajectoryShape, closed: bool) -> Self {
        Self {
            name,
            closed: Value::Literal(closed),
            parameter_declarations: None,
            shape: shape.into(),
        }
    }

    /// Converts this catalog trajectory to a scenario trajectory, resolving
    /// parameterized values against `parameters`.
    ///
    /// Every catalog shape variant — polyline, clothoid, clothoid spline and
    /// NURBS — maps onto the corresponding `geometry::shapes::Shape` variant.
    pub fn resolve_trajectory(
        &self,
        parameters: &std::collections::HashMap<String, String>,
    ) -> crate::error::Result<crate::types::actions::movement::Trajectory> {
        use crate::types::geometry::shapes::{
            ControlPoint, Knot, Nurbs, Polyline, Shape, Vertex,
        };

        let empty_shape = Shape {
            polyline: None,
            clothoid: None,
            clothoid_spline: None,
            nurbs: None,
        };

        let shape = match &self.shape.shape {
            CatalogTrajectoryShape::Polyline(polyline) => Shape {
                polyline: Some(Polyline {
                    vertices: polyline
                        .vertices
                        .iter()
                        .map(|v| Vertex {
                            time: v.time.clone(),
                            position: v.position.clone(),
                        })
                        .collect(),
                }),
                ..empty_shape
            },
            CatalogTrajectoryShape::Clothoid(clothoid) => Shape {
                clothoid: Some(crate::types::positions::trajectory::Clothoid {
                    curvature: Double::literal(clothoid.curvature.resolve(parameters)?),
                    curvature_dot: clothoid.curvature_dot.clone(),
                    curvature_prime: clothoid
                        .curvature_prime
                        .as_ref()
                        .map(|v| v.resolve(parameters).map(Double::literal))
                        .transpose()?,
                    length: Double::literal(clothoid.length.resolve(parameters)?),
                    start_time: clothoid
                        .start_time
                        .as_ref()
                        .map(|v| v.resolve(parameters).map(Double::literal))
                        .transpose()?,
                    stop_time: clothoid
                        .stop_time
                        .as_ref()
                        .map(|v| v.resolve(parameters).map(Double::literal))
                        .transpose()?,
                    start_position: clothoid.start_position.clone(),
                }),
                ..empty_shape
            },
            CatalogTrajectoryShape::ClothoidSpline(spline) => Shape {
                clothoid_spline: Some(spline.clone()),
                ..empty_shape
            },
            CatalogTrajectoryShape::Nurbs(nurbs) => {
                let order = nurbs.order.resolve(parameters)?;
                let order: u32 = u32::try_from(order).map_err(|_| {
                    crate::error::Error::invalid_value(
                        "Nurbs.order",
                        &order.to_string(),
                        "NURBS order must be a non-negative integer",
                    )
                })?;

                Shape {
                    nurbs: Some(Nurbs {
                        order: Value::Literal(order),
                        control_points: nurbs
                            .control_points
                            .iter()
                            .map(|cp| -> crate::error::Result<ControlPoint> {
                                Ok(ControlPoint {
                                    position: cp.position.clone(),
                                    time: cp
                                        .time
                                        .as_ref()
                                        .map(|v| v.resolve(parameters).map(Double::literal))
                                        .transpose()?,
                                    weight: cp.weight.clone(),
                                })
                            })
                            .collect::<crate::error::Result<Vec<_>>>()?,
                        knots: nurbs
                            .knots
                            .iter()
                            .map(|k| Knot {
                                value: k.value.clone(),
                            })
                            .collect(),
                    }),
                    ..empty_shape
                }
            }
        };

        Ok(crate::types::actions::movement::Trajectory {
            name: OSString::literal(crate::types::catalogs::entities::resolve_parameter(
                &self.name, parameters,
            )?),
            closed: Boolean::literal(self.closed.resolve(parameters)?),
            parameter_declarations: self.parameter_declarations.clone(),
            shape,
        })
    }
}

impl CatalogPolyline {
    /// Creates a polyline from a list of positions
    pub fn from_positions(positions: Vec<Position>) -> Self {
        let vertices = positions
            .into_iter()
            .map(|pos| CatalogVertex {
                time: None,
                position: pos,
            })
            .collect();

        Self { vertices }
    }

    /// Adds a vertex to this polyline
    pub fn add_vertex(&mut self, position: Position, time: Option<Double>) {
        self.vertices.push(CatalogVertex { time, position });
    }
}

impl CatalogClothoid {
    /// Creates a new clothoid with the specified parameters
    pub fn new(curvature: Double, curvature_dot: Double, length: Double) -> Self {
        Self {
            curvature,
            curvature_dot: Some(curvature_dot),
            curvature_prime: None,
            length,
            start_time: None,
            stop_time: None,
            start_position: Position::default(),
        }
    }

    /// Creates a clothoid with a start position
    pub fn with_start_position(
        curvature: Double,
        curvature_dot: Double,
        length: Double,
        start_position: Position,
    ) -> Self {
        Self {
            curvature,
            curvature_dot: Some(curvature_dot),
            curvature_prime: None,
            length,
            start_time: None,
            stop_time: None,
            start_position,
        }
    }
}

impl CatalogNurbs {
    /// Creates a new NURBS curve with the specified order
    pub fn new(order: Int) -> Self {
        Self {
            order,
            control_points: Vec::new(),
            knots: Vec::new(),
        }
    }

    /// Adds a control point to this NURBS curve
    pub fn add_control_point(&mut self, position: Position, weight: Option<Double>) {
        self.control_points.push(NurbsControlPoint {
            position,
            time: None,
            weight,
        });
    }

    /// Adds a knot to this NURBS curve
    pub fn add_knot(&mut self, value: Double) {
        self.knots.push(NurbsKnot { value });
    }
}

impl NurbsControlPoint {
    /// Creates a new control point with the specified position
    pub fn new(position: Position) -> Self {
        Self {
            position,
            time: None,
            weight: None,
        }
    }

    /// Creates a weighted control point
    pub fn with_weight(position: Position, weight: Double) -> Self {
        Self {
            position,
            time: None,
            weight: Some(weight),
        }
    }
}

impl NurbsKnot {
    /// Creates a new knot with the specified value
    pub fn new(value: Double) -> Self {
        Self { value }
    }
}

/// Catalog entity integration so `CatalogTrajectory` can be used as the entry type
/// in `CatalogContent` and behind a `CatalogReference`.
impl crate::types::catalogs::entities::CatalogEntity for CatalogTrajectory {
    type ResolvedType = crate::types::actions::movement::Trajectory;

    fn into_scenario_entity(
        self,
        parameters: std::collections::HashMap<String, String>,
    ) -> crate::error::Result<Self::ResolvedType> {
        self.resolve_trajectory(&parameters)
    }

    fn parameter_schema() -> Vec<crate::types::catalogs::entities::ParameterDefinition> {
        vec![
            crate::types::catalogs::entities::ParameterDefinition {
                name: "StartTime".to_string(),
                parameter_type: "Double".to_string(),
                default_value: Some("0.0".to_string()),
                description: Some("Start time of the trajectory in seconds".to_string()),
            },
            crate::types::catalogs::entities::ParameterDefinition {
                name: "Duration".to_string(),
                parameter_type: "Double".to_string(),
                default_value: Some("60.0".to_string()),
                description: Some("Duration of the trajectory in seconds".to_string()),
            },
            crate::types::catalogs::entities::ParameterDefinition {
                name: "Closed".to_string(),
                parameter_type: "Boolean".to_string(),
                default_value: Some("false".to_string()),
                description: Some("Whether the trajectory is closed (loops back to start)".to_string()),
            },
        ]
    }

    fn entity_name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::basic::ParameterDeclaration;
    use crate::types::enums::ParameterType;

    /// Regression: `<Trajectory><Shape><Polyline>` must deserialize.  The shape
    /// enum is externally tagged, so quick-xml matched the variant against the
    /// `Shape` field element itself and rejected the nested `<Polyline>`.
    #[test]
    fn test_catalog_trajectory_shape_polyline_round_trip() {
        let xml = r#"<Trajectory closed="false" name="VRU_CPx">
    <Shape>
        <Polyline>
            <Vertex>
                <Position>
                    <WorldPosition x="1" y="2" z="3"/>
                </Position>
            </Vertex>
        </Polyline>
    </Shape>
</Trajectory>"#;

        let trajectory: CatalogTrajectory = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(trajectory.name, "VRU_CPx");
        match &trajectory.shape.shape {
            CatalogTrajectoryShape::Polyline(polyline) => {
                assert_eq!(polyline.vertices.len(), 1);
            }
            other => panic!("expected Polyline, got {other:?}"),
        }

        let serialized = quick_xml::se::to_string(&trajectory).unwrap();
        assert!(
            serialized.contains("<Shape><Polyline>"),
            "Shape nesting lost on serialize: {serialized}"
        );
    }

    #[test]
    fn test_catalog_trajectory_creation() {
        let shape = CatalogTrajectoryShape::Polyline(CatalogPolyline {
            vertices: Vec::new(),
        });
        let trajectory = CatalogTrajectory::new("TestTrajectory".to_string(), shape);

        assert_eq!(trajectory.name, "TestTrajectory");
        assert_eq!(trajectory.closed.as_literal(), Some(&false));
        assert!(trajectory.parameter_declarations.is_none());
    }

    #[test]
    fn test_catalog_polyline() {
        let pos1 = Position::default();
        let pos2 = Position::default();

        let mut polyline = CatalogPolyline::from_positions(vec![pos1, pos2]);

        assert_eq!(polyline.vertices.len(), 2);

        let pos3 = Position::default();
        polyline.add_vertex(pos3, Some(Value::Literal(10.0)));

        assert_eq!(polyline.vertices.len(), 3);
        assert!(polyline.vertices[2].time.is_some());
    }

    #[test]
    fn test_catalog_clothoid() {
        let clothoid = CatalogClothoid::new(
            Value::Literal(0.1),
            Value::Parameter("curvature_rate".to_string()),
            Value::Literal(50.0),
        );

        assert_eq!(clothoid.curvature.as_literal().unwrap(), &0.1);
        assert!(matches!(
            clothoid.curvature_dot,
            Some(Value::Parameter(_))
        ));
        assert_eq!(clothoid.length.as_literal().unwrap(), &50.0);
    }

    #[test]
    fn test_catalog_nurbs() {
        let mut nurbs = CatalogNurbs::new(Value::Literal(3));

        let pos1 = Position::default();
        let pos2 = Position::default();

        nurbs.add_control_point(pos1, Some(Value::Literal(1.0)));
        nurbs.add_control_point(pos2, None);

        nurbs.add_knot(Value::Literal(0.0));
        nurbs.add_knot(Value::Literal(1.0));

        assert_eq!(nurbs.order.as_literal().unwrap(), &3);
        assert_eq!(nurbs.control_points.len(), 2);
        assert_eq!(nurbs.knots.len(), 2);
        assert!(nurbs.control_points[0].weight.is_some());
        assert!(nurbs.control_points[1].weight.is_none());
    }

    #[test]
    fn test_trajectory_with_parameters() {
        let param_decl = ParameterDeclarations {
            parameter_declarations: vec![ParameterDeclaration {
                name: OSString::literal("length".to_string()),
                parameter_type: ParameterType::Double,
                value: OSString::literal("100.0".to_string()),
                constraint_groups: Vec::new(),
            }],
        };

        let shape = CatalogTrajectoryShape::Clothoid(CatalogClothoid::new(
            Value::Literal(0.0),
            Value::Literal(0.01),
            Value::Parameter("length".to_string()),
        ));

        let trajectory = CatalogTrajectory::with_parameters(
            "ParameterizedTrajectory".to_string(),
            shape,
            param_decl,
        );

        assert_eq!(trajectory.name, "ParameterizedTrajectory");
        assert!(trajectory.parameter_declarations.is_some());

        match &trajectory.shape.shape {
            CatalogTrajectoryShape::Clothoid(clothoid) => {
                assert!(matches!(clothoid.length, Value::Parameter(_)));
            }
            _ => panic!("Expected clothoid shape"),
        }
    }

    #[test]
    fn test_closed_trajectory() {
        let shape = CatalogTrajectoryShape::Polyline(CatalogPolyline {
            vertices: Vec::new(),
        });
        let trajectory =
            CatalogTrajectory::with_closed("ClosedTrajectory".to_string(), shape, true);

        assert_eq!(
            trajectory.closed.as_literal().unwrap(),
            &true
        );
    }

    #[test]
    fn test_resolve_trajectory_polyline() {
        let shape = CatalogTrajectoryShape::Polyline(CatalogPolyline {
            vertices: vec![
                CatalogVertex {
                    time: Some(Value::Literal(0.0)),
                    position: Position::default(),
                },
                CatalogVertex {
                    time: Some(Value::Literal(5.0)),
                    position: Position::default(),
                },
            ],
        });

        let catalog_trajectory = CatalogTrajectory::new("TestTrajectory".to_string(), shape);
        let scenario_trajectory = catalog_trajectory
            .resolve_trajectory(&std::collections::HashMap::new())
            .unwrap();

        assert_eq!(
            scenario_trajectory.name.as_literal().unwrap(),
            "TestTrajectory"
        );
        assert_eq!(scenario_trajectory.closed.as_literal(), Some(&false));

        let polyline = scenario_trajectory
            .shape
            .polyline
            .as_ref()
            .expect("expected polyline shape");
        assert_eq!(polyline.vertices.len(), 2);
        assert_eq!(polyline.vertices[1].time, Some(Value::Literal(5.0)));
    }

    /// NURBS and clothoid-spline trajectories used to collapse into an empty
    /// polyline; they now map onto their real `Shape` variants.
    #[test]
    fn test_resolve_trajectory_nurbs_and_clothoid() {
        use crate::types::catalogs::entities::CatalogEntity;

        let mut nurbs = CatalogNurbs::new(Value::Literal(3));
        nurbs.add_control_point(Position::default(), Some(Value::Literal(1.0)));
        nurbs.add_control_point(Position::default(), None);
        nurbs.add_knot(Value::Literal(0.0));
        nurbs.add_knot(Value::Literal(1.0));

        let trajectory = CatalogTrajectory::new(
            "NurbsPath".to_string(),
            CatalogTrajectoryShape::Nurbs(nurbs),
        )
        .into_scenario_entity(std::collections::HashMap::new())
        .unwrap();

        let resolved_nurbs = trajectory.shape.nurbs.as_ref().expect("expected NURBS shape");
        assert_eq!(resolved_nurbs.order.as_literal(), Some(&3u32));
        assert_eq!(resolved_nurbs.control_points.len(), 2);
        assert_eq!(resolved_nurbs.knots.len(), 2);
        assert!(trajectory.shape.polyline.is_none());

        let clothoid_trajectory = CatalogTrajectory::new(
            "ClothoidPath".to_string(),
            CatalogTrajectoryShape::Clothoid(CatalogClothoid::new(
                Value::Literal(0.1),
                Value::Literal(0.01),
                Value::Parameter("segmentLength".to_string()),
            )),
        );

        let mut parameters = std::collections::HashMap::new();
        parameters.insert("segmentLength".to_string(), "42.0".to_string());

        let resolved = clothoid_trajectory
            .into_scenario_entity(parameters)
            .unwrap();
        let clothoid = resolved.shape.clothoid.as_ref().expect("expected clothoid");
        assert_eq!(clothoid.length.as_literal(), Some(&42.0));
        assert_eq!(clothoid.curvature.as_literal(), Some(&0.1));
    }

    #[test]
    fn test_defaults() {
        let trajectory = CatalogTrajectory::default();
        let polyline = CatalogPolyline {
            vertices: Vec::new(),
        };
        let clothoid = CatalogClothoid::new(
            Value::Literal(0.0),
            Value::Literal(0.0),
            Value::Literal(1.0),
        );
        let nurbs = CatalogNurbs::new(Value::Literal(2));

        assert_eq!(trajectory.name, "DefaultCatalogTrajectory");
        assert!(polyline.vertices.is_empty());
        assert_eq!(clothoid.curvature.as_literal().unwrap(), &0.0);
        assert_eq!(nurbs.order.as_literal().unwrap(), &2);
    }

    // ------------------------------------------------------------------
    // XSD field additions: round-trip regression tests
    // ------------------------------------------------------------------

    #[test]
    fn test_catalog_clothoid_start_stop_time_and_curvature_prime_round_trip() {
        let xml = r#"<Clothoid curvature="0.1" curvaturePrime="0.02" length="50" startTime="1.0" stopTime="5.0">
    <Position><WorldPosition x="0" y="0"/></Position>
</Clothoid>"#;
        let clothoid: CatalogClothoid = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(clothoid.curvature_prime.as_ref().unwrap().as_literal(), Some(&0.02));
        assert_eq!(clothoid.start_time.as_ref().unwrap().as_literal(), Some(&1.0));
        assert_eq!(clothoid.stop_time.as_ref().unwrap().as_literal(), Some(&5.0));

        let serialized = quick_xml::se::to_string(&clothoid).unwrap();
        let reparsed: CatalogClothoid = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(clothoid, reparsed);
    }

    #[test]
    fn test_nurbs_control_point_time_round_trip() {
        let xml = r#"<ControlPoint time="2.5" weight="1.0">
    <Position><WorldPosition x="0" y="0"/></Position>
</ControlPoint>"#;
        let cp: NurbsControlPoint = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(cp.time.as_ref().unwrap().as_literal(), Some(&2.5));

        let serialized = quick_xml::se::to_string(&cp).unwrap();
        assert!(serialized.contains(r#"time="2.5""#), "serialized: {serialized}");
        let reparsed: NurbsControlPoint = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(cp, reparsed);
    }
}
