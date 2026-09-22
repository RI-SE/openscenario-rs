//! Parameter visibility as ASAM OpenSCENARIO XML 1.3 section 9.1 defines it: a parameter's
//! scope is the subtree rooted in the element holding its declaration, and among overlapping
//! scopes the smallest one wins.
//!
//! Each claim is its own test, so one failing claim cannot hide the next. Declarations are
//! parsed from XML wherever the claim is about documents, so the values fed to the scope are
//! values a document can actually produce.

use openscenario_rs::types::basic::{Double, Int, ParameterDeclarations, Value};
use openscenario_rs::types::enums::ParameterType;
use openscenario_rs::types::ParameterScope;
use openscenario_rs::Error;

fn declarations(xml: &str) -> ParameterDeclarations {
    quick_xml::de::from_str(xml).expect("fixture declarations parse")
}

/// A `Double` attribute exactly as the typed deserializer produces it from `raw`.
fn double_attr(raw: &str) -> Double {
    quick_xml::de::from_str::<Holder>(&format!(r#"<Holder value="{raw}"/>"#))
        .expect("fixture attribute parses")
        .value
}

#[derive(serde::Deserialize)]
struct Holder {
    #[serde(rename = "@value")]
    value: Double,
}

fn declare_ok(scope: &mut ParameterScope, name: &str, ty: &str, value: &str) {
    scope
        .declare(name, ty, value)
        .unwrap_or_else(|e| panic!("declaring {name}: {e}"));
}

fn refusal(scope: &mut ParameterScope, name: &str, ty: &str, value: &str) -> String {
    match scope.declare(name, ty, value) {
        Ok(()) => panic!("declaring {name} ({ty}) = {value:?} was accepted"),
        Err(e) => e.to_string(),
    }
}

// --- Section 9.1's own shadowing example --------------------------------------------------

/// "if the Maneuver "Overtake" declares a parameter "ego_speed" and "Overtake" contains a
/// FollowTrajectoryAction that also declares a parameter "ego_speed", then only the second
/// declaration of "ego_speed" is visible in the FollowTrajectoryAction."
#[test]
fn inner_declaration_shadows_outer_inside_its_subtree() {
    let mut scope = ParameterScope::new();
    // Entering the Maneuver "Overtake".
    scope.push_frame();
    scope
        .declare_all(&declarations(
            r#"<ParameterDeclarations>
                 <ParameterDeclaration name="ego_speed" parameterType="double" value="30"/>
               </ParameterDeclarations>"#,
        ))
        .unwrap();
    // Entering the nested element that redeclares it.
    scope.push_frame();
    scope
        .declare_all(&declarations(
            r#"<ParameterDeclarations>
                 <ParameterDeclaration name="ego_speed" parameterType="double" value="10"/>
               </ParameterDeclarations>"#,
        ))
        .unwrap();

    let use_site = double_attr("$ego_speed");
    assert_eq!(scope.resolve(&use_site).unwrap(), 10.0);
}

#[test]
fn outer_declaration_is_visible_again_after_leaving_the_inner_subtree() {
    let mut scope = ParameterScope::new();
    scope.push_frame();
    declare_ok(&mut scope, "ego_speed", "double", "30");
    scope.push_frame();
    declare_ok(&mut scope, "ego_speed", "double", "10");
    scope.pop_frame().unwrap();

    let use_site = double_attr("$ego_speed");
    assert_eq!(scope.resolve(&use_site).unwrap(), 30.0);
}

#[test]
fn an_expression_sees_the_innermost_binding() {
    let mut scope = ParameterScope::new();
    declare_ok(&mut scope, "ego_speed", "double", "30");
    scope.push_frame();
    declare_ok(&mut scope, "ego_speed", "double", "10");

    let use_site = double_attr("${$ego_speed * 2}");
    assert_eq!(scope.resolve(&use_site).unwrap(), 20.0);
}

#[test]
fn visible_values_leaves_shadowed_bindings_out() {
    let mut scope = ParameterScope::new();
    declare_ok(&mut scope, "ego_speed", "double", "30");
    declare_ok(&mut scope, "lane", "int", "1");
    scope.push_frame();
    declare_ok(&mut scope, "ego_speed", "double", "10");

    let visible = scope.visible_values();
    assert_eq!(visible.len(), 2);
    assert_eq!(visible["ego_speed"], "10");
    assert_eq!(visible["lane"], "1");
}

// --- Visibility ----------------------------------------------------------------------------

/// "A parameter is considered global if it is a child of the OpenSCENARIO element": the root
/// frame, visible everywhere below it.
#[test]
fn a_global_is_visible_at_depth() {
    let mut scope = ParameterScope::new();
    declare_ok(&mut scope, "road_friction", "double", "0.8");
    for _ in 0..4 {
        scope.push_frame();
    }
    assert_eq!(scope.depth(), 4);
    let binding = scope.lookup("road_friction").unwrap();
    assert_eq!(binding.value, "0.8");
    assert_eq!(binding.parameter_type, ParameterType::Double);
}

/// "if a parameter "ego_speed" is defined at the root of a Maneuver [...] The parameter shall
/// not be used in other Maneuver instances."
#[test]
fn a_sibling_scopes_parameter_is_not_visible() {
    let mut scope = ParameterScope::new();
    // First Maneuver.
    scope.push_frame();
    declare_ok(&mut scope, "ego_speed", "double", "30");
    scope.pop_frame().unwrap();
    // Its sibling.
    scope.push_frame();

    match scope.lookup("ego_speed") {
        Err(Error::ParameterNotFound { param, .. }) => assert_eq!(param, "ego_speed"),
        other => panic!("expected ParameterNotFound for ego_speed, got {other:?}"),
    }
    assert!(scope.resolve(&double_attr("$ego_speed")).is_err());
}

#[test]
fn an_undeclared_name_is_an_error_naming_it() {
    let mut scope = ParameterScope::new();
    declare_ok(&mut scope, "declared", "int", "1");

    let err = scope
        .resolve(&Value::<i32>::Parameter("undeclared".into()))
        .unwrap_err();
    match &err {
        Error::ParameterNotFound { param, available } => {
            assert_eq!(param, "undeclared");
            assert_eq!(available, &["declared".to_string()]);
        }
        other => panic!("expected ParameterNotFound, got {other:?}"),
    }
    assert!(err.to_string().contains("undeclared"));
}

#[test]
fn an_undeclared_name_inside_an_expression_is_an_error_naming_it() {
    let scope = ParameterScope::new();
    let err = scope
        .resolve(&double_attr("${$undeclared + 1}"))
        .unwrap_err();
    assert!(err.to_string().contains("undeclared"), "{err}");
}

#[test]
fn popping_the_root_frame_is_an_error() {
    let mut scope = ParameterScope::new();
    assert!(scope.pop_frame().is_err());
    assert_eq!(scope.depth(), 0);
}

// --- Type conformance ----------------------------------------------------------------------

/// Section 9.1: the `parameterType` check "is not ensured by the XML validator and therefore
/// must be implemented by the simulator". A declaration's `value` is an XSD `String`, so the
/// document parses; the scope is where the mismatch must surface.
#[test]
fn a_type_mismatched_declaration_parses_but_is_refused_by_the_scope() {
    let parsed = declarations(
        r#"<ParameterDeclarations>
             <ParameterDeclaration name="lane_id" parameterType="int" value="left"/>
           </ParameterDeclarations>"#,
    );
    let err = ParameterScope::new().declare_all(&parsed).unwrap_err();
    match &err {
        Error::ParameterError { param, message } => {
            assert_eq!(param, "lane_id");
            assert!(message.contains("left"), "{message}");
        }
        other => panic!("expected ParameterError naming lane_id, got {other:?}"),
    }
}

#[test]
fn a_mismatched_declaration_is_not_bound() {
    let mut scope = ParameterScope::new();
    let _ = scope.declare("lane_id", "int", "left");
    assert!(scope.lookup("lane_id").is_err());
}

#[test]
fn int_out_of_range_is_refused() {
    refusal(&mut ParameterScope::new(), "p", "int", "2147483648");
}

#[test]
fn deprecated_integer_is_checked_as_int() {
    let mut scope = ParameterScope::new();
    declare_ok(&mut scope, "ok", "integer", "-1");
    refusal(&mut scope, "bad", "integer", "1.5");
}

#[test]
fn unsigned_int_refuses_a_negative() {
    refusal(&mut ParameterScope::new(), "p", "unsignedInt", "-1");
}

#[test]
fn unsigned_short_refuses_above_65535() {
    let mut scope = ParameterScope::new();
    declare_ok(&mut scope, "ok", "unsignedShort", "65535");
    refusal(&mut scope, "bad", "unsignedShort", "65536");
}

/// Section 9.2.3: "ASAM OpenSCENARIO does not use NaN or infinity".
#[test]
fn double_refuses_nan_and_infinity() {
    let mut scope = ParameterScope::new();
    declare_ok(&mut scope, "ok", "double", "-1.5e3");
    refusal(&mut scope, "nan", "double", "NaN");
    refusal(&mut scope, "inf", "double", "INF");
    refusal(&mut scope, "word", "double", "fast");
}

/// xsd:boolean's lexical space, which section 9.2.2 restates: "0, 1, true, and false".
#[test]
fn boolean_accepts_its_four_literals_and_nothing_else() {
    let mut scope = ParameterScope::new();
    for (i, v) in ["true", "false", "1", "0"].into_iter().enumerate() {
        declare_ok(&mut scope, &format!("b{i}"), "boolean", v);
    }
    refusal(&mut scope, "yes", "boolean", "yes");
    refusal(&mut scope, "upper", "boolean", "True");
}

#[test]
fn date_time_accepts_xsd_date_times_with_and_without_a_timezone() {
    let mut scope = ParameterScope::new();
    declare_ok(&mut scope, "utc", "dateTime", "2026-09-22T12:00:00Z");
    declare_ok(
        &mut scope,
        "offset",
        "dateTime",
        "2026-09-22T12:00:00.5+02:00",
    );
    declare_ok(&mut scope, "local", "dateTime", "2026-09-22T12:00:00");
    refusal(&mut scope, "bad", "dateTime", "noon");
}

#[test]
fn string_accepts_anything_literal() {
    let mut scope = ParameterScope::new();
    declare_ok(&mut scope, "s", "string", "any text, 1.5, true");
    declare_ok(&mut scope, "empty", "string", "");
}

#[test]
fn an_unknown_parameter_type_is_refused() {
    let msg = refusal(&mut ParameterScope::new(), "p", "float", "1.0");
    assert!(msg.contains("'p'"), "{msg}");
}

#[test]
fn a_resolved_int_parameter_resolves_into_an_int_field() {
    let mut scope = ParameterScope::new();
    declare_ok(&mut scope, "lane", "int", "-2");
    let field = Int::parameter("lane".into());
    assert_eq!(scope.resolve(&field).unwrap(), -2);
}

// --- Names ---------------------------------------------------------------------------------

/// "The parameter name must match the regular expression `[A-Za-z_][A-Za-z0-9_]*`."
#[test]
fn a_malformed_name_is_refused() {
    let mut scope = ParameterScope::new();
    refusal(&mut scope, "1st", "int", "1");
    refusal(&mut scope, "ego-speed", "int", "1");
    refusal(&mut scope, "", "int", "1");
    declare_ok(&mut scope, "_private", "int", "1");
}

#[test]
fn names_are_case_sensitive() {
    let mut scope = ParameterScope::new();
    declare_ok(&mut scope, "Speed", "double", "1");
    declare_ok(&mut scope, "speed", "double", "2");
    assert_eq!(scope.get("Speed"), Some("1"));
    assert_eq!(scope.get("speed"), Some("2"));
}

/// "Parameter names starting with OSC are reserved [...] the OSC prefix shall not be used."
/// The published page italicises the prefix, which is also how AsciiDoc renders `_OSC_`, so
/// both readings are refused.
#[test]
fn reserved_prefixes_are_refused() {
    let mut scope = ParameterScope::new();
    refusal(&mut scope, "OSC_version", "int", "1");
    refusal(&mut scope, "_OSC_version", "int", "1");
    declare_ok(&mut scope, "osc_lowercase", "int", "1");
    declare_ok(&mut scope, "_OSCversion", "int", "1");
}

#[test]
fn the_same_name_twice_in_one_frame_is_refused() {
    let mut scope = ParameterScope::new();
    declare_ok(&mut scope, "p", "int", "1");
    let msg = refusal(&mut scope, "p", "int", "2");
    assert!(msg.contains("'p'"), "{msg}");
    assert_eq!(scope.get("p"), Some("1"));
}

#[test]
fn the_same_name_in_a_nested_frame_is_accepted() {
    let mut scope = ParameterScope::new();
    declare_ok(&mut scope, "p", "int", "1");
    scope.push_frame();
    declare_ok(&mut scope, "p", "int", "2");
}

// --- References inside a declaration -------------------------------------------------------
//
// Section 9.1 allows a declaration's name, type or value to reference another parameter but
// does not say which declarations the reference may see. The scope reads one
// `<ParameterDeclarations>` in order: a reference sees the earlier declarations of its own
// `<ParameterDeclarations>` and everything the enclosing elements declare.

#[test]
fn a_declaration_value_may_reference_an_earlier_sibling_in_an_expression() {
    let mut scope = ParameterScope::new();
    let parsed = declarations(
        r#"<ParameterDeclarations>
             <ParameterDeclaration name="base" parameterType="double" value="10"/>
             <ParameterDeclaration name="derived" parameterType="double" value="${$base * 2}"/>
           </ParameterDeclarations>"#,
    );
    scope
        .declare_all(&parsed)
        .expect("an earlier sibling is visible");
    assert_eq!(scope.get("derived"), Some("20"));
}

#[test]
fn a_declaration_value_may_be_a_bare_reference_to_an_earlier_sibling() {
    let mut scope = ParameterScope::new();
    let declared = scope
        .declare_sequence(&[("base", "double", "10"), ("copy", "double", "$base")])
        .expect("an earlier sibling is visible");
    assert_eq!(declared[1].0, "copy");
    assert_eq!(declared[1].1.value, "10");
}

#[test]
fn a_declaration_may_reference_an_enclosing_frame() {
    let mut scope = ParameterScope::new();
    declare_ok(&mut scope, "outer", "int", "4");
    scope.push_frame();
    declare_ok(&mut scope, "inner", "int", "${$outer + 1}");
    assert_eq!(scope.get("inner"), Some("5"));
}

#[test]
fn a_declaration_name_may_reference_an_earlier_sibling() {
    let mut scope = ParameterScope::new();
    scope
        .declare_sequence(&[
            ("the_name", "string", "speed"),
            ("$the_name", "double", "3"),
        ])
        .expect("the name resolves");
    assert_eq!(scope.get("speed"), Some("3"));
}

#[test]
fn a_declaration_type_may_reference_an_earlier_sibling() {
    let mut scope = ParameterScope::new();
    scope
        .declare_sequence(&[("the_type", "string", "int"), ("p", "$the_type", "7")])
        .expect("the type resolves");
    assert_eq!(
        scope.lookup("p").unwrap().parameter_type,
        ParameterType::Int
    );
}

#[test]
fn a_resolved_type_still_checks_the_value() {
    let mut scope = ParameterScope::new();
    let err = scope
        .declare_sequence(&[("the_type", "string", "int"), ("p", "$the_type", "fast")])
        .unwrap_err()
        .to_string();
    assert!(
        err.contains("'p'") && err.contains("not a valid int"),
        "{err}"
    );
}

#[test]
fn a_referenced_value_is_checked_against_the_declaring_type() {
    let mut scope = ParameterScope::new();
    let err = scope
        .declare_sequence(&[("half", "double", "2.5"), ("count", "int", "$half")])
        .unwrap_err()
        .to_string();
    assert!(
        err.contains("'count'") && err.contains("not a valid int"),
        "{err}"
    );
}

#[test]
fn a_forward_reference_is_an_error_naming_both_parameters() {
    let mut scope = ParameterScope::new();
    let err = scope
        .declare_sequence(&[("early", "double", "${$late * 2}"), ("late", "double", "1")])
        .unwrap_err()
        .to_string();
    assert!(err.contains("'early'"), "{err}");
    assert!(err.contains("`late`"), "{err}");
    assert!(err.contains("declared after it"), "{err}");
    assert!(scope.lookup("early").is_err());
}

#[test]
fn a_forward_reference_is_an_error_even_when_an_outer_frame_declares_the_name() {
    // The later sibling's scope is the smaller one, so it is the declaration the reference
    // would mean; resolving to the outer one instead would be a silent surprise.
    let mut scope = ParameterScope::new();
    declare_ok(&mut scope, "late", "double", "100");
    scope.push_frame();
    let err = scope
        .declare_sequence(&[("early", "double", "$late"), ("late", "double", "1")])
        .unwrap_err()
        .to_string();
    assert!(err.contains("'early'") && err.contains("`late`"), "{err}");
    assert!(err.contains("declared after it"), "{err}");
}

#[test]
fn a_self_reference_is_an_error_naming_the_parameter() {
    let mut scope = ParameterScope::new();
    let err = refusal(&mut scope, "loop_", "double", "${$loop_ + 1}");
    assert!(err.contains("'loop_'"), "{err}");
    assert!(
        err.contains("references the parameter being declared"),
        "{err}"
    );
}

#[test]
fn a_self_reference_is_an_error_even_when_an_outer_frame_declares_the_name() {
    let mut scope = ParameterScope::new();
    declare_ok(&mut scope, "speed", "double", "10");
    scope.push_frame();
    let err = refusal(&mut scope, "speed", "double", "$speed");
    assert!(
        err.contains("references the parameter being declared"),
        "{err}"
    );
}

#[test]
fn an_undeclared_reference_is_an_error_naming_both_parameters() {
    let err = refusal(&mut ParameterScope::new(), "p", "double", "$nowhere");
    assert!(err.contains("'p'") && err.contains("`nowhere`"), "{err}");
    assert!(err.contains("not declared before it"), "{err}");
}

#[test]
fn a_reference_free_expression_is_evaluated() {
    let mut scope = ParameterScope::new();
    declare_ok(&mut scope, "three", "int", "${1 + 2}");
    assert_eq!(scope.get("three"), Some("3"));
}

#[test]
fn a_boolean_declaration_evaluates_its_expression_as_a_boolean() {
    let mut scope = ParameterScope::new();
    scope
        .declare_sequence(&[
            ("speed", "double", "12"),
            ("fast", "boolean", "${$speed > 10}"),
        ])
        .expect("a comparison is a Boolean expression");
    assert_eq!(scope.get("fast"), Some("true"));
}

#[test]
fn a_failing_declaration_expression_keeps_its_cause() {
    let err = refusal(&mut ParameterScope::new(), "p", "double", "${1 / 0}");
    assert!(err.contains("'p'"), "{err}");
    assert!(err.to_lowercase().contains("division by zero"), "{err}");
}

#[test]
fn a_literal_string_that_only_looks_like_a_reference_is_accepted() {
    // `$1` is not a parameter name, so the XSD `String` deserializer keeps it literal.
    let mut scope = ParameterScope::new();
    declare_ok(&mut scope, "price", "string", "$1");
    assert_eq!(scope.get("price"), Some("$1"));
}
