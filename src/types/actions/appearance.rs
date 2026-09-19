//! Actions that change how an entity looks: lights, animations, and visibility.
//!
//! Animations cover pedestrian gesture and motion, vehicle components such as doors
//! and windows, and user-defined animation files. Visibility is separate: it decides
//! which subsystems see the entity, not how it is drawn.
use crate::types::basic::{Boolean, Double, OSString, Value};
use crate::types::entities::vehicle::File;
use crate::types::enums::{
    ColorType, LightMode, PedestrianGestureType, PedestrianMotionType, VehicleComponentType,
    VehicleLightType,
};
use serde::{Deserialize, Serialize};

/// Controls entity visibility in different simulation contexts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VisibilityAction {
    /// Whether entity is visible to graphics/rendering systems
    #[serde(rename = "@graphics")]
    pub graphics: Boolean,

    /// Whether entity is detectable by sensor systems
    #[serde(rename = "@sensors")]
    pub sensors: Boolean,

    /// Whether entity participates in traffic interactions
    #[serde(rename = "@traffic")]
    pub traffic: Boolean,

    /// Optional sensor reference set for selective sensor visibility
    #[serde(rename = "SensorReferenceSet", skip_serializing_if = "Option::is_none")]
    pub sensor_reference_set: Option<SensorReferenceSet>,
}

/// Set of sensor references for selective visibility control
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SensorReferenceSet {
    /// Individual sensor references
    #[serde(rename = "SensorReference")]
    pub sensor_references: Vec<SensorReference>,
}

/// Reference to a specific sensor for visibility control
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SensorReference {
    /// Name of the referenced sensor
    #[serde(rename = "@name")]
    pub name: OSString,
}

/// Appearance actions for visual changes and animations
///
/// XSD `AppearanceAction` (`Schema/OpenSCENARIO.xsd:765-770`) is a bare
/// `xsd:choice` of `LightStateAction` or `AnimationAction`, with no
/// `minOccurs`/`maxOccurs`, so both default to 1 and exactly one branch is
/// required. `$value` states that structurally: serde rejects a document
/// naming no branch with `missing field `$value`` and one naming two with
/// `duplicate field `$value``. The parallel-`Option` shape this replaces
/// described `xsd:all` with optional members instead, and accepted both of
/// those documents, keeping whichever branches it was given.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppearanceAction {
    /// The concrete appearance action carried by this element.
    #[serde(rename = "$value")]
    pub choice: AppearanceActionChoice,
}

/// The two branches of the XSD `AppearanceAction` choice (`:765-770`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum AppearanceActionChoice {
    LightStateAction(LightStateAction),
    AnimationAction(AnimationAction),
}

impl AppearanceAction {
    /// Wraps a branch of the choice in an `AppearanceAction` element.
    pub fn new(choice: AppearanceActionChoice) -> Self {
        Self { choice }
    }
}

/// Light state control action for vehicle lighting systems
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LightStateAction {
    /// Time to transition into the requested light state
    #[serde(
        rename = "@transitionTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transition_time: Option<Double>,

    /// Which light is addressed
    #[serde(rename = "LightType")]
    pub light_type: LightType,

    /// Target state of the addressed light
    #[serde(rename = "LightState")]
    pub light_state: LightState,
}

/// Choice of the light being addressed: a standard vehicle light or a user-defined one
///
/// XSD `LightType` (`Schema/OpenSCENARIO.xsd:1418-1423`) is a bare
/// `xsd:choice`, so exactly one of `VehicleLight` or `UserDefinedLight` is
/// required.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LightType {
    /// The concrete light identification carried by this element.
    #[serde(rename = "$value")]
    pub choice: LightTypeChoice,
}

/// The two branches of the XSD `LightType` choice (`:1418-1423`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum LightTypeChoice {
    VehicleLight(VehicleLight),
    UserDefinedLight(UserDefinedLight),
}

impl LightType {
    /// Wraps a branch of the choice in a `LightType` element.
    pub fn new(choice: LightTypeChoice) -> Self {
        Self { choice }
    }
}

/// Standard vehicle light identified by its type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VehicleLight {
    /// Type of the vehicle light
    #[serde(rename = "@vehicleLightType")]
    pub vehicle_light_type: Value<VehicleLightType>,
}

/// User-defined light identified by a free-form type name
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserDefinedLight {
    /// User-defined light type name
    #[serde(rename = "@userDefinedLightType")]
    pub user_defined_light_type: OSString,
}

/// State of a light, including optional color and flashing behavior
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LightState {
    /// Light mode (on, off, flashing)
    #[serde(rename = "@mode")]
    pub mode: Value<LightMode>,

    /// Luminous intensity in lumen
    #[serde(
        rename = "@luminousIntensity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub luminous_intensity: Option<Double>,

    /// Duration of the "on" phase when flashing
    #[serde(
        rename = "@flashingOnDuration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub flashing_on_duration: Option<Double>,

    /// Duration of the "off" phase when flashing
    #[serde(
        rename = "@flashingOffDuration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub flashing_off_duration: Option<Double>,

    /// Optional color of the light
    #[serde(rename = "Color", skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
}

/// Color definition, either RGB or CMYK, tagged with a coarse color type
///
/// XSD `Color` (`Schema/OpenSCENARIO.xsd:929-935`) is a bare `xsd:choice` of
/// `ColorRgb` or `ColorCmyk` alongside the required `@colorType` attribute,
/// so the sibling attribute coexists with the `$value` choice field on the
/// same struct.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Color {
    /// Coarse color classification
    #[serde(rename = "@colorType")]
    pub color_type: Value<ColorType>,

    /// The concrete color definition carried by this element.
    #[serde(rename = "$value")]
    pub choice: ColorChoice,
}

/// The two branches of the XSD `Color` choice (`:929-935`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ColorChoice {
    ColorRgb(ColorRgb),
    ColorCmyk(ColorCmyk),
}

impl Color {
    /// Wraps a branch of the choice with the required `@colorType`.
    pub fn new(color_type: ColorType, choice: ColorChoice) -> Self {
        Self {
            color_type: Value::Literal(color_type),
            choice,
        }
    }
}

/// RGB color components, each in the range [0..1]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ColorRgb {
    /// Red component
    #[serde(rename = "@red")]
    pub red: Double,

    /// Green component
    #[serde(rename = "@green")]
    pub green: Double,

    /// Blue component
    #[serde(rename = "@blue")]
    pub blue: Double,
}

/// CMYK color components, each in the range [0..1]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ColorCmyk {
    /// Cyan component
    #[serde(rename = "@cyan")]
    pub cyan: Double,

    /// Magenta component
    #[serde(rename = "@magenta")]
    pub magenta: Double,

    /// Yellow component
    #[serde(rename = "@yellow")]
    pub yellow: Double,

    /// Key (black) component
    #[serde(rename = "@key")]
    pub key: Double,
}

/// Animation action for entity movement and component animation
///
/// No `Default`. The derive that used to sit here was justified as "every field is
/// `None`, so it states nothing" — reasoning the schema contradicts.
/// XSD `AnimationAction` (`Schema/OpenSCENARIO.xsd:740-747`) declares `AnimationType`
/// with no `minOccurs="0"`, so the element is required; and `AnimationType` is itself
/// a bare `xsd:choice` (`:757-764`) that must select a branch. The derived default
/// therefore emitted `<AnimationType/>`, which validates against nothing — category 3,
/// schema-invalid empty.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnimationAction {
    /// Whether the animation repeats
    #[serde(rename = "@loop", default, skip_serializing_if = "Option::is_none")]
    pub r#loop: Option<Boolean>,

    /// Duration of the animation
    #[serde(
        rename = "@animationDuration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub animation_duration: Option<Double>,

    /// Which animation is addressed
    #[serde(rename = "AnimationType")]
    pub animation_type: AnimationType,

    /// Optional target state of the animation
    #[serde(rename = "AnimationState", skip_serializing_if = "Option::is_none")]
    pub animation_state: Option<AnimationState>,
}

/// Choice of animation being addressed
///
/// XSD `AnimationType` (`Schema/OpenSCENARIO.xsd:757-763`) is a bare
/// `xsd:choice` with no `minOccurs="0"`, so exactly one of the four branches
/// is required. `$value` enforces that structurally instead of through a
/// hand-written `validate()`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnimationType {
    /// The concrete animation carried by this element.
    #[serde(rename = "$value")]
    pub choice: AnimationTypeChoice,
}

/// The four branches of the XSD `AnimationType` choice (`:757-763`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum AnimationTypeChoice {
    ComponentAnimation(ComponentAnimation),
    PedestrianAnimation(PedestrianAnimation),
    AnimationFile(AnimationFile),
    UserDefinedAnimation(UserDefinedAnimation),
}

/// Choice of component being animated
///
/// XSD `ComponentAnimation` (`Schema/OpenSCENARIO.xsd:947-952`) is a bare
/// `xsd:choice` with no `minOccurs="0"` — same shape as its parent
/// `AnimationType`. Because this struct is itself a `$value` choice wrapper
/// and also serves as a variant payload of `AnimationTypeChoice`, it is the
/// element-wrapper-struct pattern `$value` nesting requires: an externally
/// tagged enum cannot itself be the payload of another externally tagged
/// enum's variant and still serialize, so the payload here is a struct that
/// owns its own `$value` rather than `AnimationTypeChoice` embedding
/// `ComponentAnimationChoice` directly.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ComponentAnimation {
    /// The concrete component identification carried by this element.
    #[serde(rename = "$value")]
    pub choice: ComponentAnimationChoice,
}

/// The two branches of the XSD `ComponentAnimation` choice (`:947-952`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ComponentAnimationChoice {
    VehicleComponent(VehicleComponent),
    UserDefinedComponent(UserDefinedComponent),
}

impl ComponentAnimation {
    /// Wraps a branch of the choice in a `ComponentAnimation` element.
    pub fn new(choice: ComponentAnimationChoice) -> Self {
        Self { choice }
    }
}

/// Standard vehicle component identified by its type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VehicleComponent {
    /// Type of the vehicle component
    #[serde(rename = "@vehicleComponentType")]
    pub vehicle_component_type: Value<VehicleComponentType>,
}

/// User-defined component identified by a free-form type name
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserDefinedComponent {
    /// User-defined component type name
    #[serde(rename = "@userDefinedComponentType")]
    pub user_defined_component_type: OSString,
}

/// Pedestrian animation consisting of a motion and optional gestures
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PedestrianAnimation {
    /// Type of pedestrian motion
    #[serde(rename = "@motion", default, skip_serializing_if = "Option::is_none")]
    pub motion: Option<Value<PedestrianMotionType>>,

    /// User-defined pedestrian animation name
    #[serde(
        rename = "@userDefinedPedestrianAnimation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_defined_pedestrian_animation: Option<OSString>,

    /// Gestures performed by the pedestrian
    #[serde(
        rename = "PedestrianGesture",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub pedestrian_gestures: Vec<PedestrianGesture>,
}

/// Single pedestrian gesture
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PedestrianGesture {
    /// Type of the gesture
    #[serde(rename = "@gesture")]
    pub gesture: Value<PedestrianGestureType>,
}

/// Animation defined by an external file
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnimationFile {
    /// Offset into the animation file
    #[serde(
        rename = "@timeOffset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_offset: Option<Double>,

    /// Referenced animation file
    #[serde(rename = "File")]
    pub file: File,
}

/// User-defined animation identified by a free-form type name
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserDefinedAnimation {
    /// User-defined animation type name
    #[serde(rename = "@userDefinedAnimationType")]
    pub user_defined_animation_type: OSString,
}

/// Target state of an animation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnimationState {
    /// Animation state in the range [0..1]
    #[serde(rename = "@state")]
    pub state: Double,
}

// `VisibilityAction`, `LightStateAction`, `LightState` and
// `AnimationState` no longer implement `Default`: all fabricated a value for
// an XSD `use="required"` attribute (`VisibilityAction` :2550-2557, `LightState`
// :1402-1409, `LightStateAction` :1411-1417, `AnimationState` :754-756) — none
// declares a `default="…"`. Construct them explicitly.
impl AnimationAction {
    /// Create an animation action for the given animation (XSD-required child).
    pub fn new(animation_type: AnimationType) -> Self {
        Self {
            r#loop: None,
            animation_duration: None,
            animation_type,
            animation_state: None,
        }
    }
}

impl AnimationType {
    /// `ComponentAnimation` branch of the choice.
    pub fn component(component_animation: ComponentAnimation) -> Self {
        Self {
            choice: AnimationTypeChoice::ComponentAnimation(component_animation),
        }
    }

    /// `PedestrianAnimation` branch of the choice.
    pub fn pedestrian(pedestrian_animation: PedestrianAnimation) -> Self {
        Self {
            choice: AnimationTypeChoice::PedestrianAnimation(pedestrian_animation),
        }
    }

    /// `AnimationFile` branch of the choice.
    pub fn file(animation_file: AnimationFile) -> Self {
        Self {
            choice: AnimationTypeChoice::AnimationFile(animation_file),
        }
    }

    /// `UserDefinedAnimation` branch of the choice.
    pub fn user_defined(user_defined_animation: UserDefinedAnimation) -> Self {
        Self {
            choice: AnimationTypeChoice::UserDefinedAnimation(user_defined_animation),
        }
    }
}

impl ComponentAnimation {
    /// `VehicleComponent` branch of the choice.
    pub fn vehicle(vehicle_component: VehicleComponent) -> Self {
        Self {
            choice: ComponentAnimationChoice::VehicleComponent(vehicle_component),
        }
    }

    /// `UserDefinedComponent` branch of the choice.
    pub fn user_defined(user_defined_component: UserDefinedComponent) -> Self {
        Self {
            choice: ComponentAnimationChoice::UserDefinedComponent(user_defined_component),
        }
    }
}

impl VisibilityAction {
    /// XSD `VisibilityAction` (:2550-2557): all three attributes required.
    pub fn new(graphics: bool, sensors: bool, traffic: bool) -> Self {
        Self {
            graphics: Boolean::literal(graphics),
            sensors: Boolean::literal(sensors),
            traffic: Boolean::literal(traffic),
            sensor_reference_set: None,
        }
    }
}

impl LightStateAction {
    /// XSD `LightStateAction` (:1411-1417): `LightType` and `LightState`
    /// children are both required.
    pub fn new(light_type: LightType, light_state: LightState) -> Self {
        Self {
            transition_time: None,
            light_type,
            light_state,
        }
    }
}

impl LightState {
    /// XSD `LightState` (:1402-1409): `@mode` is the only required attribute.
    pub fn new(mode: LightMode) -> Self {
        Self {
            mode: Value::Literal(mode),
            luminous_intensity: None,
            flashing_on_duration: None,
            flashing_off_duration: None,
            color: None,
        }
    }
}

impl AnimationState {
    /// XSD `AnimationState` (:754-756): required `@state`.
    pub fn new(state: f64) -> Self {
        Self {
            state: Double::literal(state),
        }
    }
}

impl SensorReference {
    /// XSD `SensorReference` (:2019-2021): required `@name`.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: OSString::literal(name.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visibility_action_new_constructor() {
        let va = VisibilityAction::new(true, true, true);
        assert_eq!(va.graphics.as_literal(), Some(&true));
        assert_eq!(va.sensors.as_literal(), Some(&true));
        assert_eq!(va.traffic.as_literal(), Some(&true));
        assert!(va.sensor_reference_set.is_none());
    }

    #[test]
    fn test_sensor_reference_set_default_empty_vec() {
        let srs = SensorReferenceSet {
            sensor_references: Vec::new(),
        };
        assert!(srs.sensor_references.is_empty());
    }

    #[test]
    fn test_light_state_action_corpus_roundtrip() {
        let xml = r#"<LightStateAction><LightType><VehicleLight vehicleLightType="lowBeam"/></LightType><LightState mode="on"/></LightStateAction>"#;
        let action: LightStateAction = quick_xml::de::from_str(xml).unwrap();
        match &action.light_type.choice {
            LightTypeChoice::VehicleLight(v) => {
                assert_eq!(
                    v.vehicle_light_type,
                    Value::Literal(VehicleLightType::LowBeam)
                );
            }
            other => panic!("expected VehicleLight, got {other:?}"),
        }
        assert_eq!(action.light_state.mode, Value::Literal(LightMode::On));
        assert!(action.transition_time.is_none());

        let serialized = quick_xml::se::to_string(&action).unwrap();
        let reparsed: LightStateAction = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(action, reparsed);
        assert!(serialized.contains("vehicleLightType=\"lowBeam\""));
        assert!(serialized.contains("mode=\"on\""));
    }

    #[test]
    fn test_light_state_with_color_roundtrip() {
        let xml = r#"<LightStateAction transitionTime="$transition"><LightType><UserDefinedLight userDefinedLightType="halo"/></LightType><LightState mode="flashing" luminousIntensity="120.0" flashingOnDuration="0.5" flashingOffDuration="0.5"><Color colorType="red"><ColorRgb red="1.0" green="0.0" blue="0.0"/></Color></LightState></LightStateAction>"#;
        let action: LightStateAction = quick_xml::de::from_str(xml).unwrap();

        // Parameter reference survives in a scalar attribute.
        assert_eq!(
            action.transition_time.as_ref().unwrap(),
            &Double::parameter("transition".to_string())
        );

        let color = action.light_state.color.as_ref().unwrap();
        assert_eq!(color.color_type, Value::Literal(ColorType::Red));
        match &color.choice {
            ColorChoice::ColorRgb(rgb) => assert_eq!(rgb.red.as_literal(), Some(&1.0)),
            other => panic!("expected ColorRgb, got {other:?}"),
        }

        let serialized = quick_xml::se::to_string(&action).unwrap();
        // `$transition` is the schema's `parameter` production; see `Value`'s `Serialize`.
        assert!(serialized.contains("transitionTime=\"$transition\""));
        let reparsed: LightStateAction = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(action, reparsed);
    }

    #[test]
    fn test_light_state_with_cmyk_color_roundtrip() {
        let xml = r#"<LightState mode="off"><Color colorType="black"><ColorCmyk cyan="0.0" magenta="0.0" yellow="0.0" key="1.0"/></Color></LightState>"#;
        let state: LightState = quick_xml::de::from_str(xml).unwrap();
        match &state.color.as_ref().unwrap().choice {
            ColorChoice::ColorCmyk(cmyk) => assert_eq!(cmyk.key.as_literal(), Some(&1.0)),
            other => panic!("expected ColorCmyk, got {other:?}"),
        }
        let serialized = quick_xml::se::to_string(&state).unwrap();
        let reparsed: LightState = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(state, reparsed);
    }

    #[test]
    fn test_animation_action_component_branch_roundtrip() {
        let xml = r#"<AnimationAction loop="true" animationDuration="2.5"><AnimationType><ComponentAnimation><VehicleComponent vehicleComponentType="doorFrontLeft"/></ComponentAnimation></AnimationType><AnimationState state="1.0"/></AnimationAction>"#;
        let action: AnimationAction = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(action.r#loop.as_ref().unwrap().as_literal(), Some(&true));
        match &action.animation_type.choice {
            AnimationTypeChoice::ComponentAnimation(ca) => match &ca.choice {
                ComponentAnimationChoice::VehicleComponent(v) => assert_eq!(
                    v.vehicle_component_type,
                    Value::Literal(VehicleComponentType::DoorFrontLeft)
                ),
                other => panic!("expected VehicleComponent, got {other:?}"),
            },
            other => panic!("expected ComponentAnimation, got {other:?}"),
        }
        assert_eq!(
            action.animation_state.as_ref().unwrap().state.as_literal(),
            Some(&1.0)
        );
        let serialized = quick_xml::se::to_string(&action).unwrap();
        let reparsed: AnimationAction = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(action, reparsed);
    }

    #[test]
    fn test_animation_action_pedestrian_branch_roundtrip() {
        let xml = r#"<AnimationAction><AnimationType><PedestrianAnimation motion="walking" userDefinedPedestrianAnimation="limp"><PedestrianGesture gesture="wavingLeftArm"/><PedestrianGesture gesture="crossArms"/></PedestrianAnimation></AnimationType></AnimationAction>"#;
        let action: AnimationAction = quick_xml::de::from_str(xml).unwrap();
        let ped = match &action.animation_type.choice {
            AnimationTypeChoice::PedestrianAnimation(ped) => ped,
            other => panic!("expected PedestrianAnimation, got {other:?}"),
        };
        assert_eq!(
            ped.motion,
            Some(Value::Literal(PedestrianMotionType::Walking))
        );
        assert_eq!(ped.pedestrian_gestures.len(), 2);
        assert_eq!(
            ped.pedestrian_gestures[0].gesture,
            Value::Literal(PedestrianGestureType::WavingLeftArm)
        );
        let serialized = quick_xml::se::to_string(&action).unwrap();
        let reparsed: AnimationAction = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(action, reparsed);
    }

    #[test]
    fn test_animation_action_file_branch_roundtrip() {
        let xml = r#"<AnimationAction><AnimationType><AnimationFile timeOffset="0.25"><File filepath="anim/wave.fbx"/></AnimationFile></AnimationType></AnimationAction>"#;
        let action: AnimationAction = quick_xml::de::from_str(xml).unwrap();
        let file = match &action.animation_type.choice {
            AnimationTypeChoice::AnimationFile(file) => file,
            other => panic!("expected AnimationFile, got {other:?}"),
        };
        assert_eq!(file.file.filepath, "anim/wave.fbx");
        let serialized = quick_xml::se::to_string(&action).unwrap();
        let reparsed: AnimationAction = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(action, reparsed);
    }

    #[test]
    fn test_animation_action_user_defined_branch_roundtrip() {
        let xml = r#"<AnimationAction><AnimationType><UserDefinedAnimation userDefinedAnimationType="custom"/></AnimationType></AnimationAction>"#;
        let action: AnimationAction = quick_xml::de::from_str(xml).unwrap();
        match &action.animation_type.choice {
            AnimationTypeChoice::UserDefinedAnimation(ud) => assert_eq!(
                ud.user_defined_animation_type
                    .as_literal()
                    .map(|s| s.as_str()),
                Some("custom")
            ),
            other => panic!("expected UserDefinedAnimation, got {other:?}"),
        }
        let serialized = quick_xml::se::to_string(&action).unwrap();
        let reparsed: AnimationAction = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(action, reparsed);
    }

    #[test]
    fn test_visibility_action_xml_roundtrip() {
        let va = VisibilityAction::new(true, true, true);
        let xml = quick_xml::se::to_string(&va).unwrap();
        let deserialized: VisibilityAction = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(va, deserialized);
    }
}
