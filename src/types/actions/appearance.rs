//! Appearance and animation action types for visual representation
//!
//! This file contains:
//! - Light state actions for vehicle lighting systems
//! - Animation actions for entity movement and component animation
//! - Pedestrian gesture and motion animations
//! - Vehicle component animations (doors, windows, etc.)
//! - Custom user-defined animation support
//! - Visibility actions for entity appearance control
//!
use crate::types::basic::{Boolean, Double, OSString};
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AppearanceAction {
    /// Light state action for lighting control
    #[serde(rename = "LightStateAction", skip_serializing_if = "Option::is_none")]
    pub light_state_action: Option<LightStateAction>,

    /// Animation action for entity animations
    #[serde(rename = "AnimationAction", skip_serializing_if = "Option::is_none")]
    pub animation_action: Option<AnimationAction>,
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LightType {
    /// Standard vehicle light
    #[serde(rename = "VehicleLight", skip_serializing_if = "Option::is_none")]
    pub vehicle_light: Option<VehicleLight>,

    /// User-defined light
    #[serde(rename = "UserDefinedLight", skip_serializing_if = "Option::is_none")]
    pub user_defined_light: Option<UserDefinedLight>,
}

/// Standard vehicle light identified by its type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VehicleLight {
    /// Type of the vehicle light
    #[serde(rename = "@vehicleLightType")]
    pub vehicle_light_type: VehicleLightType,
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
    pub mode: LightMode,

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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Color {
    /// Coarse color classification
    #[serde(rename = "@colorType")]
    pub color_type: ColorType,

    /// RGB definition of the color
    #[serde(rename = "ColorRgb", skip_serializing_if = "Option::is_none")]
    pub color_rgb: Option<ColorRgb>,

    /// CMYK definition of the color
    #[serde(rename = "ColorCmyk", skip_serializing_if = "Option::is_none")]
    pub color_cmyk: Option<ColorCmyk>,
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AnimationType {
    /// Animation of a vehicle or user-defined component
    #[serde(rename = "ComponentAnimation", skip_serializing_if = "Option::is_none")]
    pub component_animation: Option<ComponentAnimation>,

    /// Animation of a pedestrian
    #[serde(
        rename = "PedestrianAnimation",
        skip_serializing_if = "Option::is_none"
    )]
    pub pedestrian_animation: Option<PedestrianAnimation>,

    /// Animation loaded from an external file
    #[serde(rename = "AnimationFile", skip_serializing_if = "Option::is_none")]
    pub animation_file: Option<AnimationFile>,

    /// User-defined animation
    #[serde(
        rename = "UserDefinedAnimation",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_defined_animation: Option<UserDefinedAnimation>,
}

/// Choice of component being animated
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ComponentAnimation {
    /// Standard vehicle component
    #[serde(rename = "VehicleComponent", skip_serializing_if = "Option::is_none")]
    pub vehicle_component: Option<VehicleComponent>,

    /// User-defined component
    #[serde(
        rename = "UserDefinedComponent",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_defined_component: Option<UserDefinedComponent>,
}

/// Standard vehicle component identified by its type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VehicleComponent {
    /// Type of the vehicle component
    #[serde(rename = "@vehicleComponentType")]
    pub vehicle_component_type: VehicleComponentType,
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
    pub motion: Option<PedestrianMotionType>,

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
    pub gesture: PedestrianGestureType,
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

impl Default for VisibilityAction {
    fn default() -> Self {
        Self {
            graphics: Boolean::literal(true),
            sensors: Boolean::literal(true),
            traffic: Boolean::literal(true),
            sensor_reference_set: None,
        }
    }
}

impl Default for LightStateAction {
    fn default() -> Self {
        Self {
            transition_time: None,
            light_type: LightType {
                vehicle_light: Some(VehicleLight {
                    vehicle_light_type: VehicleLightType::LowBeam,
                }),
                user_defined_light: None,
            },
            light_state: LightState::default(),
        }
    }
}

impl Default for LightState {
    fn default() -> Self {
        Self {
            mode: LightMode::On,
            luminous_intensity: None,
            flashing_on_duration: None,
            flashing_off_duration: None,
            color: None,
        }
    }
}

impl Default for AnimationAction {
    fn default() -> Self {
        Self {
            r#loop: None,
            animation_duration: None,
            animation_type: AnimationType::default(),
            animation_state: None,
        }
    }
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            state: Double::literal(0.0),
        }
    }
}

impl Default for SensorReference {
    fn default() -> Self {
        Self {
            name: OSString::literal("DefaultSensor".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visibility_action_default_all_true() {
        let va = VisibilityAction::default();
        assert_eq!(va.graphics.as_literal(), Some(&true));
        assert_eq!(va.sensors.as_literal(), Some(&true));
        assert_eq!(va.traffic.as_literal(), Some(&true));
        assert!(va.sensor_reference_set.is_none());
    }

    #[test]
    fn test_appearance_action_default_is_empty() {
        let aa = AppearanceAction::default();
        assert!(aa.light_state_action.is_none());
        assert!(aa.animation_action.is_none());
    }

    #[test]
    fn test_sensor_reference_set_default_empty_vec() {
        let srs = SensorReferenceSet::default();
        assert!(srs.sensor_references.is_empty());
    }

    #[test]
    fn test_light_state_action_corpus_roundtrip() {
        let xml = r#"<LightStateAction><LightType><VehicleLight vehicleLightType="lowBeam"/></LightType><LightState mode="on"/></LightStateAction>"#;
        let action: LightStateAction = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(
            action
                .light_type
                .vehicle_light
                .as_ref()
                .unwrap()
                .vehicle_light_type,
            VehicleLightType::LowBeam
        );
        assert_eq!(action.light_state.mode, LightMode::On);
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
        assert_eq!(color.color_type, ColorType::Red);
        assert_eq!(
            color.color_rgb.as_ref().unwrap().red.as_literal(),
            Some(&1.0)
        );

        let serialized = quick_xml::se::to_string(&action).unwrap();
        assert!(serialized.contains("transitionTime=\"${transition}\""));
        let reparsed: LightStateAction = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(action, reparsed);
    }

    #[test]
    fn test_light_state_with_cmyk_color_roundtrip() {
        let xml = r#"<LightState mode="off"><Color colorType="black"><ColorCmyk cyan="0.0" magenta="0.0" yellow="0.0" key="1.0"/></Color></LightState>"#;
        let state: LightState = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(
            state
                .color
                .as_ref()
                .unwrap()
                .color_cmyk
                .as_ref()
                .unwrap()
                .key
                .as_literal(),
            Some(&1.0)
        );
        let serialized = quick_xml::se::to_string(&state).unwrap();
        let reparsed: LightState = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(state, reparsed);
    }

    #[test]
    fn test_animation_action_component_branch_roundtrip() {
        let xml = r#"<AnimationAction loop="true" animationDuration="2.5"><AnimationType><ComponentAnimation><VehicleComponent vehicleComponentType="doorFrontLeft"/></ComponentAnimation></AnimationType><AnimationState state="1.0"/></AnimationAction>"#;
        let action: AnimationAction = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(action.r#loop.as_ref().unwrap().as_literal(), Some(&true));
        assert_eq!(
            action
                .animation_type
                .component_animation
                .as_ref()
                .unwrap()
                .vehicle_component
                .as_ref()
                .unwrap()
                .vehicle_component_type,
            VehicleComponentType::DoorFrontLeft
        );
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
        let ped = action.animation_type.pedestrian_animation.as_ref().unwrap();
        assert_eq!(ped.motion, Some(PedestrianMotionType::Walking));
        assert_eq!(ped.pedestrian_gestures.len(), 2);
        assert_eq!(
            ped.pedestrian_gestures[0].gesture,
            PedestrianGestureType::WavingLeftArm
        );
        let serialized = quick_xml::se::to_string(&action).unwrap();
        let reparsed: AnimationAction = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(action, reparsed);
    }

    #[test]
    fn test_animation_action_file_branch_roundtrip() {
        let xml = r#"<AnimationAction><AnimationType><AnimationFile timeOffset="0.25"><File filepath="anim/wave.fbx"/></AnimationFile></AnimationType></AnimationAction>"#;
        let action: AnimationAction = quick_xml::de::from_str(xml).unwrap();
        let file = action.animation_type.animation_file.as_ref().unwrap();
        assert_eq!(file.file.filepath, "anim/wave.fbx");
        let serialized = quick_xml::se::to_string(&action).unwrap();
        let reparsed: AnimationAction = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(action, reparsed);
    }

    #[test]
    fn test_animation_action_user_defined_branch_roundtrip() {
        let xml = r#"<AnimationAction><AnimationType><UserDefinedAnimation userDefinedAnimationType="custom"/></AnimationType></AnimationAction>"#;
        let action: AnimationAction = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(
            action
                .animation_type
                .user_defined_animation
                .as_ref()
                .unwrap()
                .user_defined_animation_type
                .as_literal()
                .map(|s| s.as_str()),
            Some("custom")
        );
        let serialized = quick_xml::se::to_string(&action).unwrap();
        let reparsed: AnimationAction = quick_xml::de::from_str(&serialized).unwrap();
        assert_eq!(action, reparsed);
    }

    #[test]
    fn test_visibility_action_xml_roundtrip() {
        let va = VisibilityAction::default();
        let xml = quick_xml::se::to_string(&va).unwrap();
        let deserialized: VisibilityAction = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(va, deserialized);
    }
}
