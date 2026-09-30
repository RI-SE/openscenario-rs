//! Catalog-file forms of the three entity kinds: vehicle, pedestrian, and misc
//! object. Each adds parameter declarations to the scenario type it mirrors.

use crate::types::basic::{Double, MinVec, OSString, Value};
use crate::types::entities::vehicle;
use crate::types::enums::{
    ControllerType, MiscObjectCategory, PedestrianCategory, Role, VehicleCategory,
};
use crate::types::geometry::BoundingBox;
use serde::{Deserialize, Serialize};

/// A catalog entry type: one of the element kinds a `<Catalog>` holds.
///
/// Resolving a reference to an entry is not a method of the entry type. It is done on the XML,
/// by [`crate::parser::resolve`], because section 9.5 of ASAM OpenSCENARIO XML resolves the
/// entry against its own `<ParameterDeclarations>`, and a per-type conversion had to repeat
/// that rule for every field of every entry kind.
pub trait CatalogEntity: Clone + Send + Sync {
    /// Get the name of this catalog entity
    fn entity_name(&self) -> &str;
}

/// Vehicle entity definition for catalogs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogVehicle {
    /// Name of the vehicle in the catalog. XSD `Vehicle` (`:2498`, the same
    /// complex type `CatalogVehicle` mirrors) types `@name` as the schema's
    /// `String`, a union including the parameter member, so a catalog entry
    /// may name itself with `$paramName`.
    #[serde(rename = "@name")]
    pub name: OSString,

    /// Vehicle category
    #[serde(rename = "@vehicleCategory")]
    pub vehicle_category: Value<VehicleCategory>,

    /// Role of the vehicle (e.g. ambulance, police)
    #[serde(rename = "@role", default, skip_serializing_if = "Option::is_none")]
    pub role: Option<Value<Role>>,

    /// Mass of the vehicle in kg (can be parameterized)
    #[serde(rename = "@mass", default, skip_serializing_if = "Option::is_none")]
    pub mass: Option<Double>,

    /// Path to an external 3D model (can be parameterized)
    #[serde(rename = "@model3d", default, skip_serializing_if = "Option::is_none")]
    pub model3d: Option<OSString>,

    /// Bounding box (can have parameterized dimensions)
    #[serde(rename = "BoundingBox")]
    pub bounding_box: BoundingBox,

    /// Performance characteristics (can be parameterized)
    #[serde(rename = "Performance")]
    pub performance: CatalogPerformance,

    /// Axle definitions (can be parameterized)
    #[serde(rename = "Axles")]
    pub axles: CatalogAxles,

    /// Additional properties
    #[serde(rename = "Properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<vehicle::Properties>,

    /// Trailer hitch attachment point
    #[serde(
        rename = "TrailerHitch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailer_hitch: Option<vehicle::TrailerHitch>,

    /// Trailer coupler attachment point
    #[serde(
        rename = "TrailerCoupler",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailer_coupler: Option<vehicle::TrailerCoupler>,

    /// Attached trailer (inline definition or reference)
    #[serde(rename = "Trailer", default, skip_serializing_if = "Option::is_none")]
    pub trailer: Option<vehicle::Trailer>,

    /// Parameter declarations for this catalog vehicle
    #[serde(
        rename = "ParameterDeclarations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<crate::types::basic::ParameterDeclarations>,
}

/// Performance characteristics with parameter support
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogPerformance {
    #[serde(rename = "@maxSpeed")]
    pub max_speed: Double,
    #[serde(rename = "@maxAcceleration")]
    pub max_acceleration: Double,
    #[serde(
        rename = "@maxAccelerationRate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_acceleration_rate: Option<Double>,
    #[serde(rename = "@maxDeceleration")]
    pub max_deceleration: Double,
    #[serde(
        rename = "@maxDecelerationRate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_deceleration_rate: Option<Double>,
}

/// Axles with parameter support
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogAxles {
    #[serde(rename = "FrontAxle", default, skip_serializing_if = "Option::is_none")]
    pub front_axle: Option<CatalogFrontAxle>,
    #[serde(rename = "RearAxle")]
    pub rear_axle: CatalogRearAxle,
    #[serde(
        rename = "AdditionalAxle",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_axles: Vec<CatalogRearAxle>,
}

/// Front axle with parameter support
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogFrontAxle {
    #[serde(rename = "@maxSteering")]
    pub max_steering: Double,
    #[serde(rename = "@wheelDiameter")]
    pub wheel_diameter: Double,
    #[serde(rename = "@trackWidth")]
    pub track_width: Double,
    #[serde(rename = "@positionX")]
    pub position_x: Double,
    #[serde(rename = "@positionZ")]
    pub position_z: Double,
}

/// Rear axle with parameter support
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogRearAxle {
    #[serde(rename = "@maxSteering")]
    pub max_steering: Double,
    #[serde(rename = "@wheelDiameter")]
    pub wheel_diameter: Double,
    #[serde(rename = "@trackWidth")]
    pub track_width: Double,
    #[serde(rename = "@positionX")]
    pub position_x: Double,
    #[serde(rename = "@positionZ")]
    pub position_z: Double,
}

impl CatalogEntity for CatalogVehicle {
    fn entity_name(&self) -> &str {
        self.name
            .as_literal()
            .map(String::as_str)
            .unwrap_or_default()
    }
}

/// Controller entity definition for catalogs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Controller")]
pub struct CatalogController {
    /// Name of the controller in the catalog. XSD `Controller` (`:970`, the
    /// same complex type `CatalogController` mirrors) types `@name` as the
    /// schema's `String`, a union including the parameter member.
    #[serde(rename = "@name")]
    pub name: OSString,

    /// Type of controller
    #[serde(rename = "@controllerType", skip_serializing_if = "Option::is_none")]
    pub controller_type: Option<Value<ControllerType>>,

    /// Parameter declarations for this catalog controller
    #[serde(
        rename = "ParameterDeclarations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<crate::types::basic::ParameterDeclarations>,

    /// Additional properties
    #[serde(rename = "Properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<vehicle::Properties>,
}

impl CatalogEntity for CatalogController {
    fn entity_name(&self) -> &str {
        self.name
            .as_literal()
            .map(String::as_str)
            .unwrap_or_default()
    }
}

/// Pedestrian entity definition for catalogs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogPedestrian {
    /// Name of pedestrian in the catalog. XSD `Pedestrian` (`:1677`, the same
    /// complex type `CatalogPedestrian` mirrors) types `@name` as the schema's
    /// `String`, a union including the parameter member.
    #[serde(rename = "@name")]
    pub name: OSString,

    /// Category of pedestrian
    #[serde(rename = "@pedestrianCategory")]
    pub pedestrian_category: Value<PedestrianCategory>,

    /// Mass in kg (can be parameterized) - REQUIRED by XSD
    #[serde(rename = "@mass")]
    pub mass: OSString,

    /// Role
    #[serde(rename = "@role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Value<Role>>,

    /// 3D model path (can be parameterized)
    #[serde(rename = "@model3d", skip_serializing_if = "Option::is_none")]
    pub model3d: Option<String>,

    /// Bounding box (can have parameterized dimensions)
    #[serde(rename = "BoundingBox")]
    pub bounding_box: BoundingBox,

    /// Additional properties (can be parameterized)
    #[serde(rename = "Properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<vehicle::Properties>,

    /// Parameter declarations for this catalog pedestrian
    #[serde(
        rename = "ParameterDeclarations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<crate::types::basic::ParameterDeclarations>,
}

impl CatalogEntity for CatalogPedestrian {
    fn entity_name(&self) -> &str {
        self.name
            .as_literal()
            .map(String::as_str)
            .unwrap_or_default()
    }
}

/// Placeholder catalog entities for remaining types
/// These provide basic structure for future implementation

/// Miscellaneous object entity definition for catalogs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogMiscObject {
    /// Name of the object in the catalog. XSD `MiscObject` (`:1474`, the same
    /// complex type `CatalogMiscObject` mirrors) types `@name` as the schema's
    /// `String`, a union including the parameter member.
    #[serde(rename = "@name")]
    pub name: OSString,

    /// Mass of the object in kg — XSD attribute `mass`, `use="required"`
    #[serde(rename = "@mass")]
    pub mass: Double,

    /// Category of the object — XSD attribute `miscObjectCategory`, `use="required"`
    #[serde(rename = "@miscObjectCategory")]
    pub misc_object_category: Value<MiscObjectCategory>,

    /// Optional reference to a 3D model — XSD attribute `model3d`
    #[serde(rename = "@model3d", default, skip_serializing_if = "Option::is_none")]
    pub model3d: Option<OSString>,

    #[serde(rename = "BoundingBox")]
    pub bounding_box: BoundingBox,

    /// Optional additional properties — XSD child element `<Properties>`
    #[serde(
        rename = "Properties",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub properties: Option<vehicle::Properties>,

    #[serde(
        rename = "ParameterDeclarations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<crate::types::basic::ParameterDeclarations>,
}

/// Maneuver entity definition for catalogs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogManeuver {
    /// Name of the maneuver in the catalog. XSD `Maneuver` (`:1450`, the same
    /// complex type `CatalogManeuver` mirrors) types `@name` as the schema's
    /// `String`, a union including the parameter member.
    #[serde(rename = "@name")]
    pub name: OSString,
    #[serde(
        rename = "ParameterDeclarations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parameter_declarations: Option<crate::types::basic::ParameterDeclarations>,

    /// Events making up this maneuver. XSD `Maneuver` (`:1450-1456`, the same
    /// complex type `CatalogManeuver` mirrors) declares `<Event>` with
    /// `maxOccurs="unbounded"` and no `minOccurs`, so at least one is required.
    /// The bound is now carried in the type rather than in a doc comment.
    #[serde(rename = "Event")]
    pub events: MinVec<crate::types::scenario::story::Event, 1>,
}

// Placeholder implementations for remaining catalog entities
// These will be expanded when the corresponding entity types are fully implemented

impl CatalogEntity for CatalogMiscObject {
    fn entity_name(&self) -> &str {
        self.name
            .as_literal()
            .map(String::as_str)
            .unwrap_or_default()
    }
}

impl CatalogEntity for CatalogManeuver {
    fn entity_name(&self) -> &str {
        self.name
            .as_literal()
            .map(String::as_str)
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog_vehicle_entity_name() {
        let catalog_vehicle = CatalogVehicle {
            name: OSString::literal("SportsCar".to_string()),
            vehicle_category: Value::Literal(VehicleCategory::Car),
            role: None,
            mass: None,
            model3d: None,
            bounding_box: BoundingBox::new(
                crate::types::geometry::Center::new(0.0, 0.0, 0.0),
                crate::types::geometry::Dimensions::new(2.0, 4.5, 1.5),
            ),
            performance: CatalogPerformance {
                max_speed: Value::Literal(250.0),
                max_acceleration: Value::Literal(15.0),
                max_acceleration_rate: None,
                max_deceleration: Value::Literal(12.0),
                max_deceleration_rate: None,
            },
            axles: CatalogAxles {
                front_axle: Some(CatalogFrontAxle {
                    max_steering: Value::Literal(0.6),
                    wheel_diameter: Value::Literal(0.65),
                    track_width: Value::Literal(1.8),
                    position_x: Value::Literal(3.0),
                    position_z: Value::Literal(0.3),
                }),
                rear_axle: CatalogRearAxle {
                    max_steering: Value::Literal(0.0),
                    wheel_diameter: Value::Literal(0.65),
                    track_width: Value::Literal(1.8),
                    position_x: Value::Literal(0.0),
                    position_z: Value::Literal(0.3),
                },
                additional_axles: vec![],
            },
            properties: None,
            trailer_hitch: None,
            trailer_coupler: None,
            trailer: None,
            parameter_declarations: None,
        };

        assert_eq!(catalog_vehicle.entity_name(), "SportsCar");
    }

    #[test]
    fn test_catalog_controller_entity_name() {
        let catalog_controller = CatalogController {
            name: OSString::literal("AIDriver".to_string()),
            controller_type: Some(Value::Literal(ControllerType::Movement)),
            parameter_declarations: None,
            properties: None,
        };

        assert_eq!(catalog_controller.entity_name(), "AIDriver");
    }

    /// Schema-validity guard: `controllerType` is an XSD enumeration
    /// (`ControllerType`), so an invalid string must be rejected at parse
    /// time rather than silently round-tripped as a plain string.
    #[test]
    fn test_catalog_controller_rejects_invalid_controller_type() {
        let xml = r#"<Controller name="BadController" controllerType="notARealType"/>"#;
        let err = quick_xml::de::from_str::<CatalogController>(xml)
            .expect_err("invalid controllerType value should be rejected");
        assert!(err.to_string().contains("notARealType"), "{err}");
    }

    #[test]
    fn test_catalog_pedestrian_entity_name() {
        let catalog_pedestrian = CatalogPedestrian {
            name: OSString::literal("WalkingPerson".to_string()),
            pedestrian_category: Value::Literal(PedestrianCategory::Pedestrian),
            mass: Value::Literal("75.0".to_string()),
            role: Some(Value::Literal(crate::types::enums::Role::None)),
            model3d: None,
            bounding_box: BoundingBox::new(
                crate::types::geometry::Center::new(0.0, 0.0, 0.0),
                crate::types::geometry::Dimensions::new(2.0, 4.5, 1.5),
            ),
            properties: None,
            parameter_declarations: None,
        };

        assert_eq!(catalog_pedestrian.entity_name(), "WalkingPerson");
    }

    /// Schema-validity guard: `pedestrianCategory` is an XSD enumeration
    /// (`PedestrianCategory`), so an invalid string must be rejected at parse
    /// time rather than silently round-tripped as a plain string.
    #[test]
    fn test_catalog_pedestrian_rejects_invalid_category() {
        let xml = r#"<Pedestrian name="BadPed" pedestrianCategory="notARealCategory" mass="75.0">
    <BoundingBox>
        <Center x="0.0" y="0.0" z="0.5"/>
        <Dimensions width="0.5" length="0.5" height="1.8"/>
    </BoundingBox>
</Pedestrian>"#;
        let err = quick_xml::de::from_str::<CatalogPedestrian>(xml)
            .expect_err("invalid pedestrianCategory value should be rejected");
        assert!(err.to_string().contains("notARealCategory"), "{err}");
    }

    /// The XSD requires a Maneuver to carry at least one Event (`Maneuver` :1453,
    /// `Event` at the default `minOccurs="1"`).
    ///
    /// This used to be checked by a hand-written guard in the typed catalog conversion. The
    /// `events` field is now a `MinVec<Event, 1>`, so an event-less catalog maneuver is
    /// refused where it enters the crate: at parse time.
    #[test]
    fn a_catalog_maneuver_without_events_is_refused_at_parse() {
        let err = quick_xml::de::from_str::<CatalogManeuver>(r#"<Maneuver name="empty"/>"#)
            .expect_err("a Maneuver needs at least one Event");
        assert!(err.to_string().contains("Event"), "{err}");
    }

    /// Regression: `mass`, `miscObjectCategory` and `<Properties>` are part of
    /// the XSD MiscObject type but were absent from the catalog entry struct,
    /// so a real misc object catalog failed to parse at all.
    #[test]
    fn test_catalog_misc_object_round_trip() {
        let xml = r#"<MiscObject miscObjectCategory="obstacle" mass="70" name="obstacle">
    <BoundingBox>
        <Center x="0.5" y="0.0" z="0.5"/>
        <Dimensions width="1.0" length="1.0" height="1.0"/>
    </BoundingBox>
    <Properties/>
</MiscObject>"#;

        let misc_object: CatalogMiscObject = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(misc_object.name.as_literal().unwrap(), "obstacle");
        assert_eq!(misc_object.mass.as_literal().unwrap(), &70.0);
        assert_eq!(
            misc_object.misc_object_category,
            Value::Literal(MiscObjectCategory::Obstacle)
        );
        assert!(misc_object.properties.is_some());

        let serialized = quick_xml::se::to_string(&misc_object).unwrap();
        assert!(
            serialized.contains("mass=\"70\""),
            "mass lost on serialize: {serialized}"
        );
        assert!(
            serialized.contains("miscObjectCategory=\"obstacle\""),
            "category lost on serialize: {serialized}"
        );
    }

    /// Regression: the XSD Maneuver type is ParameterDeclarations? followed by
    /// one or more <Event>. The catalog entry modelled only @name, so every
    /// event in an inline catalog <Maneuver> was silently discarded.
    #[test]
    fn test_catalog_maneuver_parses_events() {
        let xml = r#"<Maneuver name="LogAndSetVariables">
    <ParameterDeclarations>
        <ParameterDeclaration name="collidingEntity" parameterType="string" value="VRU"/>
    </ParameterDeclarations>
    <Event name="AtCollision" priority="parallel" maximumExecutionCount="1">
        <Action name="SetCollisionVariable">
            <GlobalAction>
                <VariableAction variableRef="collisionDetected">
                    <SetAction value="true"/>
                </VariableAction>
            </GlobalAction>
        </Action>
    </Event>
</Maneuver>"#;

        let maneuver: CatalogManeuver = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(maneuver.name.as_literal().unwrap(), "LogAndSetVariables");
        assert_eq!(maneuver.events.len(), 1);

        let event = &maneuver.events[0];
        assert_eq!(event.name.as_literal().unwrap(), "AtCollision");
        assert_eq!(event.actions.len(), 1);

        let serialized = quick_xml::se::to_string(&maneuver).unwrap();
        assert!(
            serialized.contains("<Event"),
            "Event lost on serialize: {serialized}"
        );
        assert!(
            serialized.contains("SetCollisionVariable"),
            "Action lost on serialize: {serialized}"
        );
    }

    // ---------------------------------------------------------------------------
    // Regression tests: ParameterDeclarations XML round-trip
    // ---------------------------------------------------------------------------

    /// Verify that a Vehicle with a <ParameterDeclarations> block deserializes
    /// correctly.  This was previously broken because the declaration type it used
    /// lacked #[serde(rename = "@...")] on its fields, causing quick-xml to look for
    /// child elements instead of XML attributes.
    #[test]
    fn test_catalog_vehicle_with_parameter_declarations_parses() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<OpenSCENARIO>
  <FileHeader revMajor="1" revMinor="3" date="2024-01-01T00:00:00"
              description="regression test" author="test"/>
  <Catalog name="TestCatalog">
    <Vehicle name="TestVehicle" vehicleCategory="car">
      <ParameterDeclarations>
        <ParameterDeclaration name="MaxDeceleration" parameterType="double" value="10.0"/>
        <ParameterDeclaration name="MaxSpeed" parameterType="double" value="50.0"/>
      </ParameterDeclarations>
      <BoundingBox>
        <Center x="0.0" y="0.0" z="0.75"/>
        <Dimensions width="2.0" length="4.5" height="1.5"/>
      </BoundingBox>
      <Performance maxSpeed="$MaxSpeed" maxAcceleration="10.0" maxDeceleration="$MaxDeceleration"/>
      <Axles>
        <FrontAxle maxSteering="0.5" wheelDiameter="0.6" trackWidth="1.7" positionX="2.8" positionZ="0.3"/>
        <RearAxle  maxSteering="0.0" wheelDiameter="0.6" trackWidth="1.7" positionX="0.0" positionZ="0.3"/>
      </Axles>
    </Vehicle>
  </Catalog>
</OpenSCENARIO>"#;

        let catalog = crate::parser::xml::parse_catalog_from_str(xml)
            .expect("catalog with ParameterDeclarations should parse without error");

        assert_eq!(catalog.catalog.vehicles.len(), 1, "expected one vehicle");

        let vehicle = &catalog.catalog.vehicles[0];
        assert_eq!(vehicle.name.as_literal().unwrap(), "TestVehicle");

        let decls = vehicle
            .parameter_declarations
            .as_ref()
            .expect("vehicle should have a ParameterDeclarations block");

        assert_eq!(
            decls.parameter_declarations.len(),
            2,
            "expected two parameter declarations"
        );

        let max_decel = decls
            .parameter_declarations
            .iter()
            .find(|p| p.name.as_literal().map(|n| n == "MaxDeceleration") == Some(true))
            .expect("MaxDeceleration declaration must be present");
        assert_eq!(
            max_decel.parameter_type,
            Value::Literal(crate::types::enums::ParameterType::Double)
        );
        assert_eq!(
            max_decel.value.as_literal().map(String::as_str),
            Some("10.0"),
            "value should map to the 'value' XML attribute"
        );

        let max_speed = decls
            .parameter_declarations
            .iter()
            .find(|p| p.name.as_literal().map(|n| n == "MaxSpeed") == Some(true))
            .expect("MaxSpeed declaration must be present");
        assert_eq!(
            max_speed.parameter_type,
            Value::Literal(crate::types::enums::ParameterType::Double)
        );
        assert_eq!(
            max_speed.value.as_literal().map(String::as_str),
            Some("50.0")
        );
    }

    /// Regression: catalog entities previously used a bespoke
    /// `ParameterDeclarationsBlock` whose declarations had no `<ConstraintGroup>`
    /// field, so constraints were silently dropped on every catalog round-trip.
    #[test]
    fn test_catalog_vehicle_parameter_declarations_constraint_group_round_trip() {
        let xml = r#"<Vehicle name="ConstrainedCar" vehicleCategory="car">
  <ParameterDeclarations>
    <ParameterDeclaration name="x" parameterType="double" value="1.0">
      <ConstraintGroup>
        <ValueConstraint rule="greaterThan" value="0"/>
      </ConstraintGroup>
    </ParameterDeclaration>
  </ParameterDeclarations>
  <BoundingBox>
    <Center x="0.0" y="0.0" z="0.75"/>
    <Dimensions width="2.0" length="4.5" height="1.5"/>
  </BoundingBox>
  <Performance maxSpeed="50.0" maxAcceleration="10.0" maxDeceleration="8.0"/>
  <Axles>
    <FrontAxle maxSteering="0.5" wheelDiameter="0.6" trackWidth="1.7" positionX="2.8" positionZ="0.3"/>
    <RearAxle  maxSteering="0.0" wheelDiameter="0.6" trackWidth="1.7" positionX="0.0" positionZ="0.3"/>
  </Axles>
</Vehicle>"#;

        let check = |vehicle: &CatalogVehicle| {
            let decls = vehicle
                .parameter_declarations
                .as_ref()
                .expect("ParameterDeclarations must be present");
            assert_eq!(decls.parameter_declarations.len(), 1);
            let decl = &decls.parameter_declarations[0];
            assert_eq!(decl.name.as_literal().map(String::as_str), Some("x"));
            assert_eq!(
                decl.parameter_type,
                Value::Literal(crate::types::enums::ParameterType::Double)
            );
            assert_eq!(decl.value.as_literal().map(String::as_str), Some("1.0"));
            assert_eq!(
                decl.constraint_groups.len(),
                1,
                "ConstraintGroup must survive"
            );
            let constraints = &decl.constraint_groups[0].value_constraints;
            assert_eq!(constraints.len(), 1);
            assert_eq!(
                constraints[0].rule,
                Value::Literal(crate::types::enums::Rule::GreaterThan)
            );
            assert_eq!(
                constraints[0].value.as_literal().map(String::as_str),
                Some("0")
            );
        };

        let vehicle: CatalogVehicle = quick_xml::de::from_str(xml).unwrap();
        check(&vehicle);

        let serialized = quick_xml::se::to_string(&vehicle).unwrap();
        assert!(
            serialized.contains("<ConstraintGroup>"),
            "ConstraintGroup lost on serialize: {serialized}"
        );

        let reparsed: CatalogVehicle = quick_xml::de::from_str(&serialized).unwrap();
        check(&reparsed);
        assert_eq!(vehicle, reparsed);
    }

    /// Verify that a Vehicle WITHOUT a <ParameterDeclarations> block still
    /// parses correctly (regression guard: the Option must remain None).
    #[test]
    fn test_catalog_vehicle_without_parameter_declarations_parses() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<OpenSCENARIO>
  <FileHeader revMajor="1" revMinor="3" date="2024-01-01T00:00:00"
              description="regression test" author="test"/>
  <Catalog name="TestCatalog">
    <Vehicle name="SimpleCar" vehicleCategory="car">
      <BoundingBox>
        <Center x="0.0" y="0.0" z="0.75"/>
        <Dimensions width="2.0" length="4.5" height="1.5"/>
      </BoundingBox>
      <Performance maxSpeed="50.0" maxAcceleration="10.0" maxDeceleration="8.0"/>
      <Axles>
        <FrontAxle maxSteering="0.5" wheelDiameter="0.6" trackWidth="1.7" positionX="2.8" positionZ="0.3"/>
        <RearAxle  maxSteering="0.0" wheelDiameter="0.6" trackWidth="1.7" positionX="0.0" positionZ="0.3"/>
      </Axles>
    </Vehicle>
  </Catalog>
</OpenSCENARIO>"#;

        let catalog = crate::parser::xml::parse_catalog_from_str(xml)
            .expect("catalog without ParameterDeclarations should parse without error");

        let vehicle = &catalog.catalog.vehicles[0];
        assert_eq!(vehicle.name.as_literal().unwrap(), "SimpleCar");
        assert!(
            vehicle.parameter_declarations.is_none(),
            "parameter_declarations should be None when element is absent"
        );
    }

    // ------------------------------------------------------------------
    // XSD field additions: round-trip regression tests
    // ------------------------------------------------------------------

    #[test]
    fn test_catalog_vehicle_role_mass_model3d_and_trailer_round_trip() {
        let xml = r#"<Vehicle name="TowCar" vehicleCategory="car" role="police" mass="1500.0" model3d="car.osgb">
    <BoundingBox>
        <Center x="0.0" y="0.0" z="0.75"/>
        <Dimensions width="2.0" length="4.5" height="1.5"/>
    </BoundingBox>
    <Performance maxSpeed="50.0" maxAcceleration="10.0" maxDeceleration="8.0"/>
    <Axles>
        <RearAxle maxSteering="0.0" wheelDiameter="0.6" trackWidth="1.7" positionX="0.0" positionZ="0.3"/>
    </Axles>
    <TrailerHitch dx="1.0"/>
    <TrailerCoupler dx="0.5"/>
</Vehicle>"#;

        let vehicle: CatalogVehicle = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(
            vehicle.role,
            Some(Value::Literal(crate::types::enums::Role::Police))
        );
        assert_eq!(vehicle.mass.clone().unwrap().as_literal(), Some(&1500.0));
        assert_eq!(
            vehicle.model3d.clone().unwrap().as_literal(),
            Some(&"car.osgb".to_string())
        );
        assert!(vehicle.trailer_hitch.is_some());
        assert!(vehicle.trailer_coupler.is_some());

        let serialized = quick_xml::se::to_string(&vehicle).unwrap();
        let reparsed: CatalogVehicle = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(vehicle, reparsed);
    }

    #[test]
    fn test_catalog_performance_rate_fields_round_trip() {
        let xml = r#"<Performance maxSpeed="50" maxAcceleration="5" maxAccelerationRate="2.5" maxDeceleration="6" maxDecelerationRate="3.5"/>"#;
        let performance: CatalogPerformance = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(
            performance
                .max_acceleration_rate
                .clone()
                .unwrap()
                .as_literal(),
            Some(&2.5)
        );
        assert_eq!(
            performance
                .max_deceleration_rate
                .clone()
                .unwrap()
                .as_literal(),
            Some(&3.5)
        );

        let serialized = quick_xml::se::to_string(&performance).unwrap();
        let reparsed: CatalogPerformance = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(performance, reparsed);
    }

    #[test]
    fn test_catalog_axles_additional_axle_round_trip() {
        let xml = r#"<Axles>
    <RearAxle maxSteering="0.0" wheelDiameter="0.6" trackWidth="1.7" positionX="0.0" positionZ="0.3"/>
    <AdditionalAxle maxSteering="0.0" wheelDiameter="0.6" trackWidth="1.7" positionX="-3.0" positionZ="0.3"/>
</Axles>"#;
        let axles: CatalogAxles = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(axles.additional_axles.len(), 1);

        let serialized = quick_xml::se::to_string(&axles).unwrap();
        assert!(
            serialized.contains("<AdditionalAxle"),
            "serialized: {serialized}"
        );
        let reparsed: CatalogAxles = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(axles, reparsed);
    }
}
