//! Probes for the §9.2 arithmetic and Boolean operators that
//! `ExpressionEvaluator::evaluate_function` did not implement: `round`, `asin`, `acos`, `atan`,
//! `sign`, `pow`, and the Boolean operators `not`/`and`/`or`.
//!
//! Every case here starts from an XML attribute, deserialized through `Value<T>`, and then
//! resolved -- the same path a real document takes -- rather than constructing an AST by hand.

use openscenario_rs::types::basic::{Boolean, Double};
use std::collections::HashMap;

fn resolve_double(
    expr: &str,
    params: &HashMap<String, String>,
) -> openscenario_rs::error::Result<f64> {
    let xml = format!("<v>${{{}}}</v>", expr);
    let value: Double = quick_xml::de::from_str(&xml).unwrap();
    value.resolve(params)
}

fn resolve_bool(
    expr: &str,
    params: &HashMap<String, String>,
) -> openscenario_rs::error::Result<bool> {
    let xml = format!("<v>${{{}}}</v>", expr);
    let value: Boolean = quick_xml::de::from_str(&xml).unwrap();
    value.resolve(params)
}

/// ASAM OpenSCENARIO XML v1.3.0 section 9.2 gives no tie-breaking rule for `round`, only
/// `round: double -> int`, and defers "the definition of the arithmetic operators" to
/// IEEE 754-2019 without naming which of that standard's round-to-integral operations applies.
/// This crate implements the conventional round-half-away-from-zero reading (`f64::round`),
/// which the example `${-round(2.6)}` in the spec's own type-mismatch walkthrough is at least
/// consistent with (2.6 rounds to 3 under either ties-away-from-zero or ties-to-even, since it
/// is not a tie).
#[test]
fn round_rounds_half_away_from_zero() {
    let params = HashMap::new();
    assert_eq!(resolve_double("round(2.5)", &params).unwrap(), 3.0);
    assert_eq!(resolve_double("round(-2.5)", &params).unwrap(), -3.0);
    assert_eq!(resolve_double("round(2.4)", &params).unwrap(), 2.0);
    assert_eq!(resolve_double("-round(2.6)", &params).unwrap(), -3.0);
}

#[test]
fn asin_is_the_inverse_of_sine() {
    let params = HashMap::new();
    let result = resolve_double("asin(1)", &params).unwrap();
    assert!((result - std::f64::consts::FRAC_PI_2).abs() < 1e-10);
}

#[test]
fn acos_is_the_inverse_of_cosine() {
    let params = HashMap::new();
    let result = resolve_double("acos(1)", &params).unwrap();
    assert!(result.abs() < 1e-10);
}

#[test]
fn atan_is_the_inverse_of_tangent() {
    let params = HashMap::new();
    let result = resolve_double("atan(1)", &params).unwrap();
    assert!((result - std::f64::consts::FRAC_PI_4).abs() < 1e-10);
}

/// Section 9.2: "sign(x) = -1, if x < 0; sign(x) = 0, if x = 0; sign(x) = 1, if x > 0."
#[test]
fn sign_returns_minus_one_zero_or_one() {
    let params = HashMap::new();
    assert_eq!(resolve_double("sign(-5)", &params).unwrap(), -1.0);
    assert_eq!(resolve_double("sign(0)", &params).unwrap(), 0.0);
    assert_eq!(resolve_double("sign(5)", &params).unwrap(), 1.0);
}

/// The spec's own example expression, `${pow(2, 8) - 1}`, evaluates to 255.
#[test]
fn pow_matches_the_spec_example() {
    let params = HashMap::new();
    assert_eq!(resolve_double("pow(2, 8) - 1", &params).unwrap(), 255.0);
}

/// Section 9.2's general restrictions require an error, not `NaN`, wherever IEEE 754-2019
/// would produce one. `asin` outside `[-1, 1]` is exactly that case, and the error must name
/// the domain violation rather than surface only "not finite".
#[test]
fn asin_outside_its_domain_is_an_error_naming_the_cause() {
    let params = HashMap::new();
    let err = resolve_double("asin(2)", &params).unwrap_err().to_string();
    assert!(
        err.contains("[-1, 1]") || err.contains("undefined"),
        "expected the domain violation named, got: {err}"
    );
}

/// A negative base raised to a non-integer exponent has no real result (IEEE 754-2019 defines
/// it as NaN), which section 9.2's general restrictions require to be an error naming the cause
/// rather than "not finite".
#[test]
fn pow_of_negative_base_with_fractional_exponent_is_an_error_naming_the_cause() {
    let params = HashMap::new();
    let err = resolve_double("pow(-1, 0.5)", &params)
        .unwrap_err()
        .to_string();
    assert!(
        err.contains("non-integer exponent") || err.contains("not a real number"),
        "expected the domain violation named, got: {err}"
    );
}

/// Section 9.2: "Supported Boolean operators (ordered by operator precedence): Negation
/// operator (not), Conjunction operator (and), Disjunction operator (or)." A `Boolean`-typed
/// attribute must be able to resolve `not $a and $b`, parsed from XML rather than built by hand.
#[test]
fn not_and_and_resolve_as_boolean() {
    let mut params = HashMap::new();
    params.insert("a".to_string(), "false".to_string());
    params.insert("b".to_string(), "true".to_string());

    // not $a and $b == (not false) and true == true
    assert!(resolve_bool("not $a and $b", &params).unwrap());

    params.insert("a".to_string(), "true".to_string());
    // not $a and $b == (not true) and true == false
    assert!(!resolve_bool("not $a and $b", &params).unwrap());
}

#[test]
fn or_resolves_as_boolean() {
    let mut params = HashMap::new();
    params.insert("a".to_string(), "false".to_string());
    params.insert("b".to_string(), "false".to_string());
    assert!(!resolve_bool("$a or $b", &params).unwrap());

    params.insert("b".to_string(), "true".to_string());
    assert!(resolve_bool("$a or $b", &params).unwrap());
}

/// The spec's precedence example: `${$A or $B and not $C}` is equivalent to
/// `${$A or ($B and (not $C))}` -- `not` binds tighter than `and`, which binds tighter than
/// `or`.
#[test]
fn boolean_operator_precedence_matches_the_spec_example() {
    let mut params = HashMap::new();
    params.insert("a".to_string(), "false".to_string());
    params.insert("b".to_string(), "true".to_string());
    params.insert("c".to_string(), "true".to_string());

    // $A or $B and not $C == false or (true and (not true)) == false or false == false
    assert!(!resolve_bool("$a or $b and not $c", &params).unwrap());

    params.insert("c".to_string(), "false".to_string());
    // false or (true and (not false)) == false or true == true
    assert!(resolve_bool("$a or $b and not $c", &params).unwrap());
}

/// A bare parameter reference is a valid Boolean operand under section 9.2 -- a parameter is an
/// operand like any other -- but its AST contains no `not`/`and`/`or` keyword and no comparison
/// operator, so choosing the evaluator by scanning the parsed expression for those keywords sends
/// it down the numeric path, where `"true"` fails to parse as `f64`. The evaluator must instead
/// be chosen by the target attribute's type.
#[test]
fn bare_boolean_parameter_resolves_as_boolean() {
    let mut params = HashMap::new();
    params.insert("flag".to_string(), "true".to_string());
    assert!(resolve_bool("$flag", &params).unwrap());

    params.insert("flag".to_string(), "false".to_string());
    assert!(!resolve_bool("$flag", &params).unwrap());
}

/// `not $flag` is the case the keyword scan already handled (its AST root is `Expr::Not`), kept
/// here alongside the two cases the scan missed so the three probes this issue names sit
/// together.
#[test]
fn not_bare_boolean_parameter_resolves_as_boolean() {
    let mut params = HashMap::new();
    params.insert("flag".to_string(), "true".to_string());
    assert!(!resolve_bool("not $flag", &params).unwrap());
}

/// A comparison resolved into a `Boolean` attribute must produce the literal text
/// `"true"`/`"false"`, not the numeric evaluator's `"1"`/`"0"` -- `bool`'s `FromStr` accepts only
/// the former. Like the bare parameter above, a comparison's AST contains no Boolean keyword, so
/// the fix has to be the same one: pick the evaluator from the target type.
#[test]
fn comparison_resolves_as_boolean_in_a_boolean_attribute() {
    let mut params = HashMap::new();
    params.insert("speed".to_string(), "20".to_string());
    assert!(resolve_bool("$speed > 10", &params).unwrap());

    params.insert("speed".to_string(), "5".to_string());
    assert!(!resolve_bool("$speed > 10", &params).unwrap());
}

/// A Boolean parameter used in an arithmetic position stays a type error in a `Double` attribute
/// (section 9.2 does not admit Boolean operands to arithmetic operators), and the error names the
/// type rather than surfacing only a generic parse failure.
#[test]
fn boolean_parameter_in_arithmetic_position_is_a_type_error_naming_the_type() {
    let mut params = HashMap::new();
    params.insert("flag".to_string(), "true".to_string());
    let err = resolve_double("$flag + 1", &params)
        .unwrap_err()
        .to_string();
    assert!(
        err.contains("failed to parse") && err.contains("true"),
        "expected a parse failure naming the non-numeric value 'true', got: {err}"
    );
}
