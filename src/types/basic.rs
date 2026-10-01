//! The scalar types every attribute is built from, and [`Value<T>`] – the wrapper that
//! lets an attribute hold a literal, a `$parameter` reference, or a `${expression}`.
//!
//! [`Value<T>`] serializes through `Display`, not through its derived `Serialize`, so
//! a parameter round-trips as the reference it was rather than as the value it
//! resolved to. `OSString`, `Double`, `Boolean` and the rest are aliases over it.
use crate::error::{Error, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

/// `xsd:boolean`'s lexical space accepts `0` and `1` alongside `true` and `false` --
/// section 9.2.2 repeats it explicitly: "Boolean literals... can also be given as 0, 1,
/// true, and false." `bool::from_str` accepts only `true`/`false`, so a schema-valid
/// `someBoolAttr="1"` attribute, a `$param` declared boolean whose raw text is `"0"`, or
/// an inherited-scope lookup of one would otherwise fail to parse even though the
/// document is valid.
///
/// Every call site here is generic over `T`, so there is nowhere to special-case `bool`
/// through the type system without an orphan-rule violation (this crate cannot implement
/// `FromStr` for the standard library's `bool`). Instead this narrows the text itself
/// before parsing, gated by `T`'s name the same way the empty-string check below reads
/// `T` for `Double` and `expression.rs`'s `evaluate_expression` reads it for `target_is_bool`.
pub(crate) fn normalize_xsd_lexical<T>(s: &str) -> Cow<'_, str> {
    if std::any::type_name::<T>() == "bool" {
        match s {
            "1" => return Cow::Borrowed("true"),
            "0" => return Cow::Borrowed("false"),
            _ => {}
        }
    }
    Cow::Borrowed(s)
}

// Value enum that can hold either a literal value, a parameter reference, or an expression
//
// OpenSCENARIO supports parameter references using ${parameterName} syntax and
// mathematical expressions using ${expression} syntax.
// This enum allows us to represent both compile-time literals and runtime parameters/expressions.
//
// # Type Parameters
//
// * `T` - The type of the literal value. Must implement `Clone` for the `resolve()` method to work.
//
// # Note
//
// While the `Value<T>` enum itself doesn't require `T: Clone` at the struct level,
// the `resolve()` method and other utility methods require `T: Clone`. Attempting to
// call `resolve()` on a `Value<T>` where `T` doesn't implement `Clone` will result in
// a compilation error.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value<T> {
    /// A literal value known at parse time
    Literal(T),
    /// A parameter reference that will be resolved at runtime
    Parameter(String),
    /// A mathematical expression that will be evaluated at runtime
    Expression(String),
}

impl<T> Value<T>
where
    T: FromStr + Clone,
    T::Err: std::fmt::Display,
{
    /// Resolve this value using the provided parameter map
    pub fn resolve(&self, params: &HashMap<String, String>) -> Result<T> {
        match self {
            Value::Literal(value) => Ok(value.clone()),
            Value::Parameter(param_name) => {
                let param_value = params
                    .get(param_name)
                    .ok_or_else(|| Error::parameter_error(param_name, "parameter not found"))?;

                normalize_xsd_lexical::<T>(param_value)
                    .parse::<T>()
                    .map_err(|e| {
                        Error::parameter_error(
                            param_name,
                            &format!("failed to parse '{}': {}", param_value, e),
                        )
                    })
            }
            Value::Expression(expr) => {
                // For now, we'll treat expressions as parameters that need to be resolved
                // In a full implementation, we would parse and evaluate the mathematical expression
                let resolved_expr = resolve_expression::<T>(expr, params)?;
                resolved_expr.parse::<T>().map_err(|e| {
                    Error::parameter_error(
                        expr,
                        &format!(
                            "failed to parse expression result '{}': {}",
                            resolved_expr, e
                        ),
                    )
                })
            }
        }
    }

    /// Get the literal value if this is a literal, otherwise None
    #[inline]
    pub fn as_literal(&self) -> Option<&T> {
        match self {
            Value::Literal(value) => Some(value),
            Value::Parameter(_) => None,
            Value::Expression(_) => None,
        }
    }

    /// Get the parameter name if this is a parameter, otherwise None
    #[inline]
    pub fn as_parameter(&self) -> Option<&str> {
        match self {
            Value::Literal(_) => None,
            Value::Parameter(name) => Some(name),
            Value::Expression(_) => None,
        }
    }

    /// Get the expression if this is an expression, otherwise None
    #[inline]
    pub fn as_expression(&self) -> Option<&str> {
        match self {
            Value::Literal(_) => None,
            Value::Parameter(_) => None,
            Value::Expression(expr) => Some(expr),
        }
    }
}

impl<T: Clone> Value<T> {
    /// Create a literal value
    #[inline]
    pub fn literal(value: T) -> Self {
        Value::Literal(value)
    }

    /// Create a parameter reference
    #[inline]
    pub fn parameter(name: String) -> Self {
        Value::Parameter(name)
    }

    /// Create an expression
    #[inline]
    pub fn expression(expr: String) -> Self {
        Value::Expression(expr)
    }
}

// Custom serde implementation to handle ${param} and ${expression} syntax
impl<'de, T> Deserialize<'de> for Value<T>
where
    T: Deserialize<'de> + FromStr,
    T::Err: std::fmt::Display,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        // Handle empty strings for Double type - return error for invalid empty values
        if s.is_empty() && std::any::type_name::<T>().contains("f64") {
            return Err(serde::de::Error::custom(
                "Empty string is not a valid value for Double type",
            ));
        }

        // Check if this is a parameter reference or expression
        if s.starts_with("${") && s.ends_with('}') && s.len() > 3 {
            // ASAM OpenSCENARIO XML section 9.2 gives the braced spelling to the
            // `expression` production alone; a bare parameter reference is always
            // unbraced (`$name`, handled below). Section 9.2's own examples reference
            // a parameter *inside* an expression with its `$` prefix, as in
            // `${$defaultWidth + 12.3}`, so a braced single identifier with no `$`
            // (`${pi}`) is not a parameter reference at all -- it is an expression
            // that happens to be one token, and one the crate defines no constant
            // for. Treating it as `$pi` silently renamed the reference on every
            // round trip, so the braced form is always `Value::Expression` here;
            // resolving it is `resolve_expression`'s job, not the deserializer's.
            let content = &s[2..s.len() - 1];
            Ok(Value::Expression(content.to_string()))
        } else if s.starts_with("$") && s.len() > 1 {
            // Handle $param format (without curly braces)
            let content = &s[1..];
            if is_valid_parameter_name(content) && !content.contains(|c| "+-*/%()".contains(c)) {
                Ok(Value::Parameter(content.to_string()))
            } else {
                // Not a valid parameter, treat as literal
                match normalize_xsd_lexical::<T>(&s).parse::<T>() {
                    Ok(value) => Ok(Value::Literal(value)),
                    Err(e) => Err(serde::de::Error::custom(format!(
                        "Failed to parse '{}': {}",
                        s, e
                    ))),
                }
            }
        } else {
            // Try to parse as literal value
            match normalize_xsd_lexical::<T>(&s).parse::<T>() {
                Ok(value) => Ok(Value::Literal(value)),
                Err(e) => Err(serde::de::Error::custom(format!(
                    "Failed to parse '{}': {}",
                    s, e
                ))),
            }
        }
    }
}

/// Forwards `T`'s default, so a container whose enum-typed field became `Value<E>` keeps
/// exactly the default it had before: `Value::Literal(E::default())`. This states nothing
/// that `E::default()` did not already state -- it is not a new invented value.
impl<T: Default> Default for Value<T> {
    fn default() -> Self {
        Value::Literal(T::default())
    }
}

// The schema's `parameter` production is `[$][A-Za-z_][A-Za-z0-9_]*` -- unbraced
// (`Schema/OpenSCENARIO.xsd:4-8`). The braced spelling is the separate `expression`
// production, and while every *scalar* union lists both members, all 37 *enumeration*
// unions list `parameter` alone. Emitting `${name}` for a parameter reference therefore
// produces schema-invalid XML on any enum-typed attribute, while `$name` is valid on
// every union in the schema. Deserialize stays permissive and accepts either spelling.
//
// `Serialize` delegates to `Display` below instead of formatting the `Parameter`/
// `Expression` branches a second time, so the wire form and the user-visible form
// cannot drift apart the way they did before (the F15 sigil bug was exactly this class
// of error: two independent formatters that only agreed because separate tests happened
// to exercise each one).
impl<T> Serialize for Value<T>
where
    T: Serialize + fmt::Display,
{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

// Implement Display trait for Value<T> to enable use in println! and format! macros
impl<T> fmt::Display for Value<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Literal(value) => write!(f, "{}", value),
            // `$name` is the schema's `parameter` production and is valid on every
            // union; `${...}` is `expression`. See the module-level note on `Serialize`.
            Value::Parameter(name) => write!(f, "${}", name),
            Value::Expression(expr) => write!(f, "${{{}}}", expr),
        }
    }
}

// OpenSCENARIO basic type aliases
pub type OSString = Value<String>;
pub type Double = Value<f64>;
pub type Int = Value<i32>;
pub type UnsignedInt = Value<u32>;
pub type UnsignedShort = Value<u16>;
pub type Boolean = Value<bool>;

pub type DateTime = Value<XsdDateTime>;

/// `xsd:dateTime`'s lexical space makes the timezone offset optional (XSD Part 2, `dateTime`);
/// RFC 3339, which `chrono::DateTime<Utc>::from_str` enforces, does not. A timezone-less
/// dateTime is schema-valid ASAM OpenSCENARIO XML -- `TimeOfDayCondition` (XSD:2169-2172) gives
/// `dateTime` no narrower a type than the XSD base -- so this holds either lexical form as
/// parsed, rather than forcing a timezone-less value into an assumed UTC offset that was never
/// written. Forcing that assumption would also make the offset-bearing and offset-less forms
/// indistinguishable on the way back out, which is what made the round trip lossy in the first
/// place.
///
/// The lexical space also admits written forms the parsed value alone cannot distinguish:
/// `+00:00` and `Z` name the same offset, and `.5`, `.50`, and `.500` name the same fraction.
/// `chrono`'s formatter always picks one canonical spelling, so deriving `Display` from the
/// parsed `chrono` value alone rewrites every other spelling on the way back out -- lexically
/// lossy even though the instant is preserved. This type keeps the text a value was parsed
/// from alongside it and reproduces that text literally; only a value built directly with
/// [`XsdDateTime::aware`] or [`XsdDateTime::naive`], which was never parsed from XML, falls
/// back to the canonical form.
///
/// `PartialEq`, `Eq` and `Hash` compare the parsed `XsdDateTimeValue` only; the recorded
/// `source` text takes no part. Two values that name the same instant through different
/// spellings -- `Z` and `+00:00`, or `.5` and `.500` -- therefore compare equal even though
/// each still serializes as the text it was parsed from. A value with no explicit offset
/// (`Naive`) is never equal to one with an offset (`Aware`), regardless of clock reading,
/// because it names no instant to compare. Use `to_string` (via [`Display`](fmt::Display)) or
/// hold onto the source string directly when the written spelling itself is what matters.
#[derive(Debug, Clone)]
pub struct XsdDateTime {
    value: XsdDateTimeValue,
    source: Option<Box<str>>,
}

impl PartialEq for XsdDateTime {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for XsdDateTime {}

impl std::hash::Hash for XsdDateTime {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum XsdDateTimeValue {
    /// A dateTime with an explicit offset, including a trailing `Z` for UTC.
    Aware(chrono::DateTime<chrono::FixedOffset>),
    /// A dateTime with no timezone.
    Naive(chrono::NaiveDateTime),
}

impl XsdDateTime {
    /// A timezone-aware value in its canonical lexical form. Use this to construct a value
    /// that was not read from XML; a value read from XML keeps the text it was parsed from
    /// (see the type-level docs), so prefer parsing over this constructor when the text
    /// matters.
    pub fn aware(value: chrono::DateTime<chrono::FixedOffset>) -> Self {
        Self {
            value: XsdDateTimeValue::Aware(value),
            source: None,
        }
    }

    /// A timezone-less value in its canonical lexical form. See [`XsdDateTime::aware`].
    pub fn naive(value: chrono::NaiveDateTime) -> Self {
        Self {
            value: XsdDateTimeValue::Naive(value),
            source: None,
        }
    }

    /// The timezone-aware instant, if this value carries one.
    pub fn as_aware(&self) -> Option<chrono::DateTime<chrono::FixedOffset>> {
        match self.value {
            XsdDateTimeValue::Aware(dt) => Some(dt),
            XsdDateTimeValue::Naive(_) => None,
        }
    }

    /// The timezone-less instant, if this value carries one.
    pub fn as_naive(&self) -> Option<chrono::NaiveDateTime> {
        match self.value {
            XsdDateTimeValue::Naive(dt) => Some(dt),
            XsdDateTimeValue::Aware(_) => None,
        }
    }
}

impl FromStr for XsdDateTime {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match chrono::DateTime::parse_from_rfc3339(s) {
            Ok(dt) => Ok(Self {
                value: XsdDateTimeValue::Aware(dt),
                source: Some(Box::from(s)),
            }),
            // `parse_from_rfc3339`'s own error is the one worth keeping: the two formats
            // overlap enough (both are `%Y-%m-%dT%H:%M:%S%.f...`) that its message -- naming
            // the missing offset -- is more useful than the naive parser's "input contains
            // invalid characters" on the same text.
            Err(aware_err) => chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f")
                .map(|dt| Self {
                    value: XsdDateTimeValue::Naive(dt),
                    source: Some(Box::from(s)),
                })
                .map_err(|_| aware_err),
        }
    }
}

impl fmt::Display for XsdDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(source) = &self.source {
            return write!(f, "{}", source);
        }
        match self.value {
            // `use_z: true` writes a UTC offset as `Z` instead of `+00:00`; this is the
            // canonical form used only for a value that was never parsed from text.
            XsdDateTimeValue::Aware(dt) => {
                write!(
                    f,
                    "{}",
                    dt.to_rfc3339_opts(chrono::SecondsFormat::AutoSi, true)
                )
            }
            // `%.f` prints nothing when there are no fractional seconds.
            XsdDateTimeValue::Naive(dt) => write!(f, "{}", dt.format("%Y-%m-%dT%H:%M:%S%.f")),
        }
    }
}

/// Serializes through `Display` (the recorded source text, when there is one), matching
/// `Value<T>`'s own serialization path so a `DateTime` used outside `Value` still round-trips
/// its lexical form rather than a `chrono`-derived one.
impl Serialize for XsdDateTime {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

/// Deserializes through `FromStr` so the recorded source text is always the text that was on
/// the wire, matching `Serialize` above.
impl<'de> Deserialize<'de> for XsdDateTime {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<XsdDateTime>()
            .map_err(|e| serde::de::Error::custom(format!("failed to parse '{}': {}", s, e)))
    }
}

// Enumeration type aliases.
//
// All 37 enumeration `simpleType`s in `Schema/OpenSCENARIO.xsd` are `xsd:union`s whose second
// member is `<xsd:restriction base="parameter"/>`, so every enum-typed attribute may carry a
// parameter reference in place of a literal. These aliases name that union type the same way
// `OSString`/`Double` name the scalar ones.
//
// The struct fields themselves spell `Value<E>` rather than the alias, so that the parameter
// mechanism is visible at the point of declaration and `Option<Value<E>>` reads unambiguously;
// the aliases exist for user-facing signatures.

pub type AngleTypeValue = Value<crate::types::enums::AngleType>;
pub type AutomaticGearTypeValue = Value<crate::types::enums::AutomaticGearType>;
#[allow(deprecated)]
pub type CloudStateValue = Value<crate::types::enums::CloudState>;
pub type ColorTypeValue = Value<crate::types::enums::ColorType>;
pub type ConditionEdgeValue = Value<crate::types::enums::ConditionEdge>;
pub type ControllerTypeValue = Value<crate::types::enums::ControllerType>;
pub type CoordinateSystemValue = Value<crate::types::enums::CoordinateSystem>;
pub type DirectionalDimensionValue = Value<crate::types::enums::DirectionalDimension>;
pub type DynamicsDimensionValue = Value<crate::types::enums::DynamicsDimension>;
pub type DynamicsShapeValue = Value<crate::types::enums::DynamicsShape>;
pub type FollowingModeValue = Value<crate::types::enums::FollowingMode>;
pub type FractionalCloudCoverValue = Value<crate::types::enums::FractionalCloudCover>;
#[allow(deprecated)]
pub type LateralDisplacementValue = Value<crate::types::enums::LateralDisplacement>;
pub type LightModeValue = Value<crate::types::enums::LightMode>;
#[allow(deprecated)]
pub type LongitudinalDisplacementValue = Value<crate::types::enums::LongitudinalDisplacement>;
pub type MiscObjectCategoryValue = Value<crate::types::enums::MiscObjectCategory>;
pub type ObjectTypeValue = Value<crate::types::enums::ObjectType>;
pub type ParameterTypeValue = Value<crate::types::enums::ParameterType>;
pub type PedestrianCategoryValue = Value<crate::types::enums::PedestrianCategory>;
pub type PedestrianGestureTypeValue = Value<crate::types::enums::PedestrianGestureType>;
pub type PedestrianMotionTypeValue = Value<crate::types::enums::PedestrianMotionType>;
pub type PrecipitationTypeValue = Value<crate::types::enums::PrecipitationType>;
pub type PriorityValue = Value<crate::types::enums::Priority>;
pub type ReferenceContextValue = Value<crate::types::enums::ReferenceContext>;
pub type RelativeDistanceTypeValue = Value<crate::types::enums::RelativeDistanceType>;
pub type RoleValue = Value<crate::types::enums::Role>;
pub type RouteStrategyValue = Value<crate::types::enums::RouteStrategy>;
pub type RoutingAlgorithmValue = Value<crate::types::enums::RoutingAlgorithm>;
pub type RuleValue = Value<crate::types::enums::Rule>;
pub type SpeedTargetValueTypeValue = Value<crate::types::enums::SpeedTargetValueType>;
pub type StoryboardElementStateValue = Value<crate::types::enums::StoryboardElementState>;
pub type StoryboardElementTypeValue = Value<crate::types::enums::StoryboardElementType>;
pub type TriggeringEntitiesRuleValue = Value<crate::types::enums::TriggeringEntitiesRule>;
pub type VehicleCategoryValue = Value<crate::types::enums::VehicleCategory>;
pub type VehicleComponentTypeValue = Value<crate::types::enums::VehicleComponentType>;
pub type VehicleLightTypeValue = Value<crate::types::enums::VehicleLightType>;
pub type WetnessValue = Value<crate::types::enums::Wetness>;

/// Check if a string is a valid parameter name, i.e. the part after the `$` sigil.
///
/// This is the XSD `parameter` production verbatim, `Schema/OpenSCENARIO.xsd:6`:
///
/// ```text
/// <xsd:pattern value="[$][A-Za-z_][A-Za-z0-9_]*"/>
/// ```
///
/// The check used to be `char::is_alphanumeric`, which is **Unicode**
/// alphanumeric, so the crate accepted `$café` where the schema does not. The pattern
/// is ASCII-only: first character `[A-Za-z_]`, the rest `[A-Za-z0-9_]`. Note this
/// governs the *parameter* production only — the `expression` production
/// (`Schema/OpenSCENARIO.xsd:11`) has its own, different character class and is not
/// affected.
pub fn is_valid_parameter_name(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        // `[A-Za-z_]`
        Some(first) if first.is_ascii_alphabetic() || first == '_' => {}
        // Empty, a leading digit, or any non-ASCII character.
        _ => return false,
    }
    // `[A-Za-z0-9_]*`
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Resolve a mathematical expression by parsing and evaluating it
///
/// This implementation uses the full expression parser and evaluator
fn resolve_expression<T: FromStr>(expr: &str, params: &HashMap<String, String>) -> Result<String>
where
    T::Err: std::fmt::Display,
{
    // The evaluator is authoritative. An earlier version of this function fell back to a
    // textual `${name}` substitution whenever the evaluator returned an error (021f2fc): that
    // substitution predates the evaluator itself, and the commit kept it running afterward as a
    // safety net rather than as a feature the schema asks for. Falling back on error turns an
    // evaluation failure into a false success — a division by zero or an unresolvable reference
    // parses to `Ok` holding the unevaluated text — which is exactly what ASAM OpenSCENARIO XML
    // section 9.2 requires to be an error. Propagating the evaluator's own error instead keeps
    // the real cause (division by zero, an unknown parameter, a malformed expression) rather
    // than discarding it and failing later, elsewhere, on a parse error that names the symptom
    // instead of the cause.
    //
    // `evaluate_expression_text` picks the numeric or Boolean evaluator by the target type `T`
    // and enforces section 9.2's "no NaN or infinity" restriction itself, so this function no
    // longer hardcodes `f64` or repeats that check: a `Boolean`-typed attribute resolving
    // `${not $a and $b}`, `${$flag}`, or `${$speed > 10}` needs the text `"true"`/`"false"`, not
    // a number. The AST shape alone cannot tell a Boolean parameter reference or a comparison
    // from a numeric one -- only the attribute's declared type can -- so the target type is
    // read off `T` the same way the empty-string check above reads `T` for `Double`.
    let target_is_bool = std::any::type_name::<T>() == "bool";
    crate::expression::evaluate_expression_text(expr, params, target_is_bool)
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml;

    #[test]
    fn test_parameter_name_validation() {
        // XSD `parameter`, Schema/OpenSCENARIO.xsd:6 — `[$][A-Za-z_][A-Za-z0-9_]*`
        // (this function validates the part after the `$`).
        assert!(is_valid_parameter_name("speed"));
        assert!(is_valid_parameter_name("vehicle_speed"));
        assert!(is_valid_parameter_name("speed123"));
        assert!(is_valid_parameter_name("_leading_underscore")); // `_` is in `[A-Za-z_]`
        assert!(is_valid_parameter_name("A")); // single ASCII letter
        assert!(!is_valid_parameter_name("123speed")); // Can't start with digit
        assert!(!is_valid_parameter_name("")); // Can't be empty
        assert!(!is_valid_parameter_name("speed-limit")); // No hyphens

        // Non-ASCII is rejected: the XSD character classes are `[A-Za-z_]` and
        // `[A-Za-z0-9_]`, not Unicode alphanumeric. `char::is_alphanumeric` accepted all
        // of these.
        assert!(!is_valid_parameter_name("café")); // non-ASCII in the tail
        assert!(!is_valid_parameter_name("évitement")); // non-ASCII leading letter
        assert!(!is_valid_parameter_name("速度")); // non-Latin script
        assert!(!is_valid_parameter_name("spe\u{0435}d")); // Cyrillic 'е' homoglyph
        assert!(!is_valid_parameter_name("param\u{00B2}")); // superscript two: Unicode alphanumeric
        assert!(!is_valid_parameter_name("\u{FF41}bc")); // fullwidth 'a'
    }

    /// The `${…}` spelling belongs to the `expression` production
    /// (`Schema/OpenSCENARIO.xsd:11`) alone, whatever its content looks like; the
    /// `parameter` production (`Schema/OpenSCENARIO.xsd:4-8`) is unbraced. Braces
    /// therefore settle the variant on their own, independent of the ASCII rule
    /// tightened elsewhere in this file for the bare-sigil form.
    #[test]
    fn braces_always_select_the_expression_production() {
        // A `${…}` body containing an operator is an expression, as before.
        let expr: Value<f64> = quick_xml::de::from_str(r#"<v>${speed + 10}</v>"#).unwrap();
        assert!(matches!(expr, Value::Expression(ref e) if e == "speed + 10"));

        // A plain ASCII name inside `${…}` is *also* an expression -- braced, so it is
        // the `expression` production, never `parameter`, regardless of whether the body
        // parses as a valid parameter name.
        let bare_name: Value<f64> = quick_xml::de::from_str(r#"<v>${speed}</v>"#).unwrap();
        assert!(matches!(bare_name, Value::Expression(ref e) if e == "speed"));
        assert_eq!(bare_name.to_string(), "${speed}");
        let bare_name: Value<String> = quick_xml::de::from_str(r#"<v>${speed}</v>"#).unwrap();
        assert!(matches!(bare_name, Value::Expression(ref e) if e == "speed"));
        assert_eq!(bare_name.to_string(), "${speed}");

        // A non-ASCII `${…}` body is carried through as an expression rather than being
        // dropped or rejected -- the character class this file tightened applies to the
        // unbraced `parameter` production, not to `${…}`.
        let unicode: Value<f64> = quick_xml::de::from_str(r#"<v>${café}</v>"#).unwrap();
        assert!(matches!(unicode, Value::Expression(ref e) if e == "café"));
        assert_eq!(unicode.to_string(), "${café}");

        // The bare-sigil `$café` spelling matches neither XSD production, so it is not a
        // parameter reference; it falls through to a literal parse, which fails for f64.
        assert!(quick_xml::de::from_str::<Value<f64>>(r#"<v>$café</v>"#).is_err());

        // `$speed` (the schema's `parameter` production, unbraced) still deserializes as
        // a parameter and re-serializes with the bare sigil.
        let bare: Value<f64> = quick_xml::de::from_str(r#"<v>$speed</v>"#).unwrap();
        assert!(matches!(bare, Value::Parameter(ref p) if p == "speed"));
        assert_eq!(bare.to_string(), "$speed");
    }

    #[test]
    fn test_value_resolution() {
        let mut params = HashMap::new();
        params.insert("speed".to_string(), "30.0".to_string());
        params.insert("acceleration".to_string(), "2.5".to_string());

        // Test literal resolution
        let literal = Value::<f64>::literal(10.0);
        assert_eq!(literal.resolve(&params).unwrap(), 10.0);

        // Test parameter resolution
        let parameter = Value::<f64>::parameter("speed".to_string());
        assert_eq!(parameter.resolve(&params).unwrap(), 30.0);

        // Test expression resolution (basic)
        let expression = Value::<String>::expression("$speed".to_string());
        assert_eq!(expression.resolve(&params).unwrap(), "30");
    }

    /// A valid expression still resolves, as both a `Double` and a `String`, once the fallback
    /// that swallowed evaluator errors is gone.
    #[test]
    fn valid_expression_still_resolves() {
        let mut params = HashMap::new();
        params.insert("speed".to_string(), "10".to_string());

        let as_double: Value<f64> = quick_xml::de::from_str(r#"<v>${$speed * 2}</v>"#).unwrap();
        assert_eq!(as_double.resolve(&params).unwrap(), 20.0);

        let as_string: Value<String> = quick_xml::de::from_str(r#"<v>${$speed * 2}</v>"#).unwrap();
        assert_eq!(as_string.resolve(&params).unwrap(), "20");
    }

    /// ASAM OpenSCENARIO XML section 9.2 requires an error on division by zero. Before this
    /// change, resolving as `String` returned `Ok("1 / 0")` -- the unevaluated text -- and
    /// resolving as `Double` failed later with "invalid float literal", discarding the real
    /// cause. Both must now fail, and both must name division by zero.
    #[test]
    fn division_by_zero_is_an_error_not_unevaluated_text() {
        let params = HashMap::new();

        let as_double: Value<f64> = quick_xml::de::from_str(r#"<v>${1 / 0}</v>"#).unwrap();
        let err = as_double.resolve(&params).unwrap_err().to_string();
        assert!(
            err.contains("division by zero"),
            "expected the real cause, got: {err}"
        );

        let as_string: Value<String> = quick_xml::de::from_str(r#"<v>${1 / 0}</v>"#).unwrap();
        let err = as_string.resolve(&params).unwrap_err().to_string();
        assert!(
            err.contains("division by zero"),
            "expected the real cause, got: {err}"
        );
    }

    /// Section 9.2 requires an error on "sqrt of a negative value". Before this change,
    /// resolving as `String` returned the unevaluated `"sqrt(-1)"` text as `Ok`.
    #[test]
    fn sqrt_of_negative_is_an_error_not_unevaluated_text() {
        let params = HashMap::new();

        let as_double: Value<f64> = quick_xml::de::from_str(r#"<v>${sqrt(-1)}</v>"#).unwrap();
        let err = as_double.resolve(&params).unwrap_err().to_string();
        assert!(err.contains("sqrt"), "expected the real cause, got: {err}");

        let as_string: Value<String> = quick_xml::de::from_str(r#"<v>${sqrt(-1)}</v>"#).unwrap();
        let err = as_string.resolve(&params).unwrap_err().to_string();
        assert!(err.contains("sqrt"), "expected the real cause, got: {err}");
    }

    /// A reference to a parameter that is not in scope is "an unresolvable type mismatch" in
    /// section 9.2's terms and must be an error, not a textual copy of the missing name. Before
    /// this change, resolving as `String` returned `Ok("$missing * 2")`.
    #[test]
    fn missing_parameter_in_expression_is_an_error_not_unevaluated_text() {
        let params = HashMap::new();

        let as_double: Value<f64> = quick_xml::de::from_str(r#"<v>${$missing * 2}</v>"#).unwrap();
        let err = as_double.resolve(&params).unwrap_err().to_string();
        assert!(
            err.contains("not found"),
            "expected the real cause, got: {err}"
        );

        let as_string: Value<String> =
            quick_xml::de::from_str(r#"<v>${$missing * 2}</v>"#).unwrap();
        let err = as_string.resolve(&params).unwrap_err().to_string();
        assert!(
            err.contains("not found"),
            "expected the real cause, got: {err}"
        );
    }

    /// Text that is not a valid expression at all -- the evaluator's parser rejects it -- must
    /// fail rather than pass through as if it had been a literal. Before this change, resolving
    /// as `String` returned the input text unchanged as `Ok`.
    #[test]
    fn unparseable_expression_is_an_error_not_unevaluated_text() {
        let params = HashMap::new();

        let as_double: Value<f64> =
            quick_xml::de::from_str(r#"<v>${this is not an expression}</v>"#).unwrap();
        let err = as_double.resolve(&params).unwrap_err().to_string();
        assert!(
            err.contains("unknown identifier 'this'"),
            "expected the parser's cause, got: {err}"
        );

        let as_string: Value<String> =
            quick_xml::de::from_str(r#"<v>${this is not an expression}</v>"#).unwrap();
        let err = as_string.resolve(&params).unwrap_err().to_string();
        assert!(
            err.contains("unknown identifier 'this'"),
            "expected the parser's cause, got: {err}"
        );
    }

    /// `${x}` is a bare identifier: it names no constant, and a parameter is referenced as
    /// `$x`, so resolving it must fail -- as both a `Double` and a `String` -- saying the `$`
    /// is missing rather than reporting a parameter that was never meant.
    #[test]
    fn braced_bare_name_fails_to_resolve() {
        let params = HashMap::new();

        let as_double: Value<f64> = quick_xml::de::from_str(r#"<v>${x}</v>"#).unwrap();
        let err = as_double.resolve(&params).unwrap_err().to_string();
        assert!(
            err.contains("parameters are referenced as $x"),
            "expected the missing-`$` cause, got: {err}"
        );

        let as_string: Value<String> = quick_xml::de::from_str(r#"<v>${x}</v>"#).unwrap();
        let err = as_string.resolve(&params).unwrap_err().to_string();
        assert!(
            err.contains("parameters are referenced as $x"),
            "expected the missing-`$` cause, got: {err}"
        );
    }

    #[test]
    fn test_parameter_declaration_creation() {
        // Test basic creation
        let param = ParameterDeclaration::new(
            "MaxSpeed".to_string(),
            ParameterType::Double,
            "60.0".to_string(),
        );

        assert_eq!(param.name.as_literal().unwrap(), "MaxSpeed");
        assert_eq!(param.parameter_type, Value::Literal(ParameterType::Double));
        assert_eq!(param.value.as_literal().unwrap(), "60.0");
        assert!(!param.has_constraints());
    }

    #[test]
    fn test_parameter_declaration_with_constraints() {
        let constraints = ValueConstraintGroup::new(vec![
            ValueConstraint::greater_than("0.0".to_string()),
            ValueConstraint::less_than("100.0".to_string()),
        ])
        .unwrap();

        let param = ParameterDeclaration::with_constraints(
            "Speed".to_string(),
            ParameterType::Double,
            "30.0".to_string(),
            vec![constraints],
        );

        assert!(param.has_constraints());
        let constraint_group = &param.constraint_groups[0];
        assert_eq!(constraint_group.value_constraints.len(), 2);
        assert_eq!(
            constraint_group.value_constraints[0].rule,
            Value::Literal(Rule::GreaterThan)
        );
        assert_eq!(
            constraint_group.value_constraints[1].rule,
            Value::Literal(Rule::LessThan)
        );
    }

    #[test]
    fn test_parameter_declaration_add_constraint() {
        let mut param =
            ParameterDeclaration::new("Age".to_string(), ParameterType::Int, "25".to_string());

        // Initially no constraints
        assert!(!param.has_constraints());

        // Add first constraint
        param.add_constraint(ValueConstraint::greater_than("0".to_string()));
        assert!(param.has_constraints());

        // Add second constraint
        param.add_constraint(ValueConstraint::less_than("120".to_string()));

        let constraints = &param.constraint_groups[0];
        assert_eq!(constraints.value_constraints.len(), 2);
    }

    #[test]
    fn test_range_creation() {
        let range = Range::new(0.0, 100.0);
        assert_eq!(range.lower_limit.as_literal().unwrap(), &0.0);
        assert_eq!(range.upper_limit.as_literal().unwrap(), &100.0);

        // `try_new` accepts lower <= upper, including equal bounds, in argument order.
        for (lower, upper) in [(-5.0, 10.0), (3.0, 3.0)] {
            let range = Range::try_new(lower, upper)
                .unwrap_or_else(|e| panic!("try_new({lower}, {upper}) rejected: {e}"));
            assert_eq!(range.lower_limit.as_literal(), Some(&lower));
            assert_eq!(range.upper_limit.as_literal(), Some(&upper));
        }

        // ...and rejects an inverted range with a validation error on `Range`.
        match Range::try_new(10.0, 0.0) {
            Err(Error::ValidationError { field, message }) => {
                assert_eq!(field, "Range");
                assert!(message.contains("lower limit"), "message: {message}");
            }
            other => panic!("expected ValidationError for try_new(10.0, 0.0), got {other:?}"),
        }
    }

    #[test]
    fn test_directory_creation() {
        // Test basic creation
        let dir = Directory::new("/path/to/catalogs".to_string());
        assert_eq!(dir.path.as_literal().unwrap(), "/path/to/catalogs");

        // Test parameter creation
        let param_dir = Directory::from_parameter("CatalogPath".to_string());
        assert_eq!(param_dir.path.as_parameter().unwrap(), "CatalogPath");

        // Test construction via ::new (no `Directory::default()`: `@path` is
        // `use="required"` with no schema default — Schema/OpenSCENARIO.xsd:1067-1069)
        let default_dir = Directory::new(String::new());
        assert_eq!(default_dir.path.as_literal().unwrap(), "");
    }

    #[test]
    fn test_directory_path_resolution() {
        let mut params = HashMap::new();
        params.insert("CatalogPath".to_string(), "/catalogs/vehicles".to_string());

        // Test literal path resolution
        let dir = Directory::new("/path/to/catalogs".to_string());
        assert_eq!(dir.resolve_path(&params).unwrap(), "/path/to/catalogs");

        // Test parameter path resolution
        let param_dir = Directory::from_parameter("CatalogPath".to_string());
        assert_eq!(
            param_dir.resolve_path(&params).unwrap(),
            "/catalogs/vehicles"
        );
    }

    #[test]
    fn test_directory_path_validation() {
        // Valid paths
        let valid_dir = Directory::new("/path/to/catalogs".to_string());
        assert!(valid_dir.validate_path());

        let relative_dir = Directory::new("./catalogs".to_string());
        assert!(relative_dir.validate_path());

        // Invalid paths
        let empty_dir = Directory::new("".to_string());
        assert!(!empty_dir.validate_path());

        let null_dir = Directory::new("path\0with\0null".to_string());
        assert!(!null_dir.validate_path());

        // Parameters are considered valid at this stage
        let param_dir = Directory::from_parameter("CatalogPath".to_string());
        assert!(param_dir.validate_path());
    }

    #[test]
    fn test_scientific_notation_parsing() {
        // Real XOSC files use this scientific-notation style (e.g. `9.2884257876425379e-04`);
        // assert the XML-deserialized `Double` matches the value `f64::parse` produces.
        let test_values = [
            "0.0000000000000000e+00",
            "1.5000000000000000e+00",
            "9.2884257876425379e-04",
            "3.7479999999999983e+01",
            "-2.8099999999999987e+00",
        ];

        for val in test_values {
            let expected = val.parse::<f64>().unwrap();
            let xml_str = format!("<test>{}</test>", val);
            let double_val: Double = quick_xml::de::from_str(&xml_str).unwrap();
            assert_eq!(double_val, Double::literal(expected));
        }
    }

    #[test]
    fn test_alks_scenario_constraint_pattern() {
        // Test the exact pattern from ALKS scenario file
        let xml = r#"
        <ParameterDeclaration name="SideVehicle_InitPosition_RelativeLaneId" parameterType="int" value="1">
          <ConstraintGroup>
            <ValueConstraint rule="equalTo" value="1"></ValueConstraint>
          </ConstraintGroup>
          <ConstraintGroup>
            <ValueConstraint rule="equalTo" value="-1"></ValueConstraint>
          </ConstraintGroup>
        </ParameterDeclaration>
        "#;

        // This should now parse without the "duplicate field" error
        let result = quick_xml::de::from_str::<ParameterDeclaration>(xml);
        assert!(
            result.is_ok(),
            "Failed to parse ALKS constraint pattern: {:?}",
            result.err()
        );

        let param = result.unwrap();
        assert_eq!(
            param.name.as_literal().unwrap(),
            "SideVehicle_InitPosition_RelativeLaneId"
        );
        assert_eq!(param.parameter_type, Value::Literal(ParameterType::Int));
        assert_eq!(param.value.as_literal().unwrap(), "1");
        assert!(param.has_constraints());
        // Two groups of one `equalTo` constraint each: the parsed document equals the
        // declaration the constructors build for the same content.
        let equal_to = |value: &str| {
            ValueConstraintGroup::new(vec![ValueConstraint::equal_to(value.to_string())]).unwrap()
        };
        assert_eq!(
            param,
            ParameterDeclaration::with_constraints(
                "SideVehicle_InitPosition_RelativeLaneId".to_string(),
                ParameterType::Int,
                "1".to_string(),
                vec![equal_to("1"), equal_to("-1")],
            )
        );
        assert_eq!(
            param.constraint_groups[1].value_constraints[0].rule,
            Value::Literal(Rule::EqualTo)
        );
    }

    #[test]
    #[allow(clippy::approx_constant)] // 3.14 is a literal test input for Display formatting, not an approximation of PI
    fn test_value_display_trait() {
        // Test Display implementation for Value<T>
        let literal_value = Value::<f64>::literal(42.5);
        assert_eq!(format!("{}", literal_value), "42.5");

        // `$speed`, not `${speed}`: the schema's `parameter` production is unbraced
        // (`Schema/OpenSCENARIO.xsd:4-8`), and it is the only spelling the 37 enumeration
        // unions accept. The braced form belongs to `expression`, tested just below.
        let parameter_value = Value::<String>::parameter("speed".to_string());
        assert_eq!(format!("{}", parameter_value), "$speed");

        let expression_value = Value::<String>::expression("speed * 2".to_string());
        assert_eq!(format!("{}", expression_value), "${speed * 2}");

        // Test with different types
        let bool_literal = Value::<bool>::literal(true);
        assert_eq!(format!("{}", bool_literal), "true");

        let string_literal = Value::<String>::literal("hello".to_string());
        assert_eq!(format!("{}", string_literal), "hello");

        // Test with our type aliases
        let double_value = Double::literal(3.14);
        assert_eq!(format!("{}", double_value), "3.14");

        let os_string_param = OSString::parameter("vehicle_name".to_string());
        assert_eq!(format!("{}", os_string_param), "$vehicle_name");

        let boolean_expr = Boolean::expression("speed > 30".to_string());
        assert_eq!(format!("{}", boolean_expr), "${speed > 30}");
    }

    /// `tests/parameterized_enum_test.rs::parameters_serialize_in_the_schema_s_parameter_form`
    /// pins the serialized `Parameter` spelling (`$cat`) through a full document round trip,
    /// but nothing pinned the serialized `Expression` spelling (`${...}`) -- only its `Display`
    /// form, in `test_value_display_trait` above. `Serialize` now delegates to `Display`
    /// (`serializer.collect_str`), so this asserts the two can no longer disagree: serializing
    /// through serde produces exactly the same text `Display` does, for both branches.
    #[test]
    fn serialize_matches_display_for_parameter_and_expression() {
        let parameter = Value::<f64>::parameter("speed".to_string());
        assert_eq!(
            serde_json::to_string(&parameter).unwrap(),
            format!("\"{}\"", parameter)
        );
        assert_eq!(serde_json::to_string(&parameter).unwrap(), "\"$speed\"");

        let expression = Value::<f64>::expression("speed * 2".to_string());
        assert_eq!(
            serde_json::to_string(&expression).unwrap(),
            format!("\"{}\"", expression)
        );
        assert_eq!(
            serde_json::to_string(&expression).unwrap(),
            "\"${speed * 2}\""
        );
    }
}

// Data Container Types for Scenario Structure

use crate::types::enums::{ParameterType, Rule};

/// Parameter declarations container
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ParameterDeclarations {
    #[serde(rename = "ParameterDeclaration", default)]
    pub parameter_declarations: Vec<ParameterDeclaration>,
}

/// Individual parameter declaration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParameterDeclaration {
    #[serde(rename = "@name")]
    pub name: OSString,
    #[serde(rename = "@parameterType")]
    pub parameter_type: Value<ParameterType>,
    #[serde(rename = "@value")]
    pub value: OSString,
    #[serde(
        rename = "ConstraintGroup",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub constraint_groups: Vec<ValueConstraintGroup>,
}

/// Parameter constraints container
///
/// XSD `ValueConstraintGroup` (`:2446-2450`): sequence of `ValueConstraint`,
/// `maxOccurs="unbounded"`, no `minOccurs`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValueConstraintGroup {
    #[serde(rename = "ValueConstraint")]
    pub value_constraints: MinVec<ValueConstraint, 1>,
}

/// Individual parameter value constraint
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValueConstraint {
    #[serde(rename = "@rule")]
    pub rule: Value<Rule>,
    #[serde(rename = "@value")]
    pub value: OSString,
}

/// Value range specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Range {
    #[serde(rename = "@lowerLimit")]
    pub lower_limit: Double,
    #[serde(rename = "@upperLimit")]
    pub upper_limit: Double,
}

// Helper methods for ParameterDeclaration
impl ParameterDeclaration {
    /// Create a new parameter declaration with the given name, type, and value
    pub fn new(name: String, parameter_type: ParameterType, value: String) -> Self {
        Self {
            name: OSString::literal(name),
            parameter_type: Value::Literal(parameter_type),
            value: OSString::literal(value),
            constraint_groups: Vec::new(),
        }
    }

    /// Create a parameter declaration with constraints
    pub fn with_constraints(
        name: String,
        parameter_type: ParameterType,
        value: String,
        constraints: Vec<ValueConstraintGroup>,
    ) -> Self {
        Self {
            name: OSString::literal(name),
            parameter_type: Value::Literal(parameter_type),
            value: OSString::literal(value),
            constraint_groups: constraints,
        }
    }

    /// Add a constraint to this parameter declaration
    pub fn add_constraint(&mut self, constraint: ValueConstraint) {
        if let Some(group) = self.constraint_groups.last_mut() {
            // Growing an already-non-empty group can never fall below `MinVec`'s
            // minimum, so `push` (infallible) replaces the old rebuild-through-`new`.
            group.value_constraints.push(constraint);
        } else {
            self.constraint_groups.push(ValueConstraintGroup {
                // The array proves the one required element; `MinVec::new` and its
                // `Result` are not needed.
                value_constraints: MinVec::from_min([constraint], vec![]),
            });
        }
    }

    /// Check if the parameter has constraints
    pub fn has_constraints(&self) -> bool {
        !self.constraint_groups.is_empty()
    }
}

/// Directory path reference for catalog files
///
/// This type represents a directory path that contains catalog files.
/// It's used by all catalog location types to specify where to find
/// catalog definitions that can be referenced by scenarios.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Directory {
    /// Path to the directory containing catalog files
    #[serde(rename = "@path")]
    pub path: OSString,
}

// Helper methods for Directory
impl Directory {
    /// Create a new Directory with the given path
    pub fn new(path: String) -> Self {
        Self {
            path: OSString::literal(path),
        }
    }

    /// Create a Directory from a parameter reference
    pub fn from_parameter(param_name: String) -> Self {
        Self {
            path: OSString::parameter(param_name),
        }
    }

    /// Get the resolved path string
    pub fn resolve_path(&self, params: &HashMap<String, String>) -> Result<String> {
        self.path.resolve(params)
    }

    /// Check if the directory path is valid (basic validation)
    pub fn validate_path(&self) -> bool {
        if let Some(literal_path) = self.path.as_literal() {
            !literal_path.is_empty() && !literal_path.contains('\0')
        } else {
            // Parameters and expressions are assumed valid at this stage
            true
        }
    }
}

// Helper methods for ValueConstraintGroup
impl ValueConstraintGroup {
    /// Create a new value constraint group with the given constraints.
    ///
    /// Fails if `constraints` is empty: XSD `ValueConstraintGroup` requires at least one
    /// `ValueConstraint`.
    pub fn new(constraints: Vec<ValueConstraint>) -> Result<Self> {
        Ok(Self {
            value_constraints: MinVec::new(constraints)?,
        })
    }

    /// Add a constraint to the group.
    pub fn add_constraint(&mut self, constraint: ValueConstraint) {
        // Growing an already-non-empty group can never fall below `MinVec`'s minimum,
        // so `push` (infallible) replaces the old rebuild-through-`new`.
        self.value_constraints.push(constraint);
    }
}

// Helper methods for ValueConstraint
impl ValueConstraint {
    /// Create a new value constraint
    pub fn new(rule: Rule, value: String) -> Self {
        Self {
            rule: Value::Literal(rule),
            value: OSString::literal(value),
        }
    }

    /// Create an equality constraint
    pub fn equal_to(value: String) -> Self {
        Self::new(Rule::EqualTo, value)
    }

    /// Create a greater than constraint
    pub fn greater_than(value: String) -> Self {
        Self::new(Rule::GreaterThan, value)
    }

    /// Create a less than constraint
    pub fn less_than(value: String) -> Self {
        Self::new(Rule::LessThan, value)
    }
}

// Helper methods for Range
impl Range {
    /// Create a new range with the given limits
    ///
    /// # Panics
    /// Panics if lower > upper when both are literals
    pub fn new(lower: f64, upper: f64) -> Self {
        debug_assert!(lower <= upper, "Range lower limit must be <= upper limit");
        Self {
            lower_limit: Double::literal(lower),
            upper_limit: Double::literal(upper),
        }
    }

    /// Create a new range with validation
    pub fn try_new(lower: f64, upper: f64) -> Result<Self> {
        if lower > upper {
            return Err(Error::validation_error(
                "Range",
                "Range lower limit must be <= upper limit",
            ));
        }
        Ok(Self {
            lower_limit: Double::literal(lower),
            upper_limit: Double::literal(upper),
        })
    }
}

/// A vector that carries its schema lower bound in its type.
///
/// Several `xsd:sequence` particles in `Schema/OpenSCENARIO.xsd` declare a `minOccurs`
/// above one: `Vertex` and `Waypoint` require two children, `Position` inside `Polygon`
/// requires three. `Vec<T>` cannot state such a bound, and no serde attribute states it
/// either. Removing `#[serde(default)]` only rejects the empty case, so a `<Polyline>`
/// with a single `<Vertex>` parses and is written back out in the same schema-invalid
/// shape.
///
/// `MinVec<T, MIN>` closes both directions at once. `Deserialize` reads a `Vec<T>` and
/// then checks the length, so a short document fails to parse. The inner field is
/// private and there is no `DerefMut`, so a value shorter than `MIN` cannot be
/// constructed at all and the serializer has nothing invalid to write. Hence the bound
/// is a property of the type rather than a check every caller has to remember.
///
/// A mutable slice deref would be sound, since a slice cannot shrink a vector. However
/// `DerefMut` also exposes `Vec`'s own inherent methods through auto-deref, `clear` and
/// `truncate` among them, which is why it is left out. Mutation goes through
/// [`MinVec::new`] on a rebuilt vector.
///
/// There is deliberately no `Default` for `MIN > 0`: the empty vector is exactly the
/// value the type exists to reject.
///
/// ```
/// use openscenario_rs::types::basic::MinVec;
///
/// let two: MinVec<u8, 2> = MinVec::new(vec![1, 2]).unwrap();
/// assert_eq!(two.as_slice(), &[1, 2]);
/// assert!(MinVec::<u8, 2>::new(vec![1]).is_err());
///
/// // A caller that already has MIN guaranteed items avoids the `Result` entirely.
/// let mut three: MinVec<u8, 2> = MinVec::from_min([1, 2], vec![3]);
/// assert_eq!(three.as_slice(), &[1, 2, 3]);
/// three.push(4);
/// assert_eq!(three.as_slice(), &[1, 2, 3, 4]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MinVec<T, const MIN: usize>(Vec<T>);

impl<T, const MIN: usize> MinVec<T, MIN> {
    /// Build a `MinVec` from a vector, rejecting one shorter than `MIN`.
    pub fn new(items: Vec<T>) -> Result<Self> {
        if items.len() < MIN {
            return Err(Error::validation_error(
                "MinVec",
                &format!("expected at least {} items, got {}", MIN, items.len()),
            ));
        }
        Ok(Self(items))
    }

    /// Build a `MinVec` from `MIN` required items plus any further ones, with no
    /// fallible check.
    ///
    /// A caller with one guaranteed non-empty `Vec<T>` still has to go through
    /// [`MinVec::new`] and its `Result`, even though nothing about the call can fail.
    /// That mismatch is what leaves a `.unwrap()` or `?` sitting over an error that
    /// never occurs. `from_min` closes it by moving the bound into the signature: the
    /// array `head` carries exactly `MIN` elements at the type level, so supplying it
    /// is itself the proof, and there is nothing left to check at runtime.
    ///
    /// This is the general form of the shape suggested for `MIN = 1` (a single
    /// guaranteed element plus a `Vec` of the rest). A single element does not prove
    /// the bound for `MIN > 1`, which is why the guaranteed part is an array sized to
    /// `MIN` rather than one value.
    pub fn from_min(head: [T; MIN], rest: Vec<T>) -> Self {
        let mut items: Vec<T> = head.into();
        items.extend(rest);
        Self(items)
    }

    /// Append one further item to an already-non-empty `MinVec`, in place.
    ///
    /// A live `MinVec<T, MIN>` holds at least `MIN` items by construction, so growing
    /// it by one cannot fall below that bound. This replaces the rebuild-through-`new`
    /// pattern (`MinVec::new(vec_with_one_more_item).expect("...")`) for the common
    /// case of extending a value the caller already holds.
    pub fn push(&mut self, item: T) {
        self.0.push(item);
    }

    /// Borrow the contents as a slice.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.0
    }

    /// Consume the wrapper and return the underlying vector.
    ///
    /// The bound is checked on construction, so handing back an owned `Vec` cannot
    /// invalidate a live `MinVec`.
    #[inline]
    pub fn into_inner(self) -> Vec<T> {
        self.0
    }
}

impl<T, const MIN: usize> std::ops::Deref for MinVec<T, MIN> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const MIN: usize> TryFrom<Vec<T>> for MinVec<T, MIN> {
    type Error = Error;

    fn try_from(items: Vec<T>) -> Result<Self> {
        Self::new(items)
    }
}

impl<T, const MIN: usize> IntoIterator for MinVec<T, MIN> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T, const MIN: usize> IntoIterator for &'a MinVec<T, MIN> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<T: Serialize, const MIN: usize> Serialize for MinVec<T, MIN> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de, T, const MIN: usize> Deserialize<'de> for MinVec<T, MIN>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let items = Vec::<T>::deserialize(deserializer)?;
        Self::new(items).map_err(serde::de::Error::custom)
    }
}
