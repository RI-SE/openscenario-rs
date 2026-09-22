//! Parameter visibility as ASAM OpenSCENARIO XML 1.3 section 9.1 defines it.
//!
//! Section 9.1 states the rule on the XML tree:
//!
//! > The scope of a parameter is the subtree rooted in the element where the
//! > `ParameterDeclaration` is located. [...] If there are multiple parameters with the same
//! > name and overlapping scopes in the scenario, only the parameter with the smallest scope
//! > that subsumes the location is accessible.
//!
//! [`ParameterScope`] is that rule as a stack. A document walker pushes a frame when it enters
//! an element holding `<ParameterDeclarations>`, declares each `<ParameterDeclaration>` into
//! it, and pops the frame when it leaves the element. A lookup searches from the innermost
//! frame outwards, so the innermost binding shadows every outer one of the same name, and a
//! binding in a sibling subtree is gone by the time the walker reaches the next sibling.
//!
//! The document's global parameters -- "a child of the OpenSCENARIO element that is the root
//! of any ASAM OpenSCENARIO file" (section 9.1) -- belong in the root frame, which exists from
//! construction and cannot be popped.
//!
//! The API takes attribute values as the strings the XML holds, so a pass over the raw XML can
//! drive it without deserializing any typed element first.
//!
//! # What this module refuses, and why
//!
//! * **A name that breaks section 9.1's naming rules.** A name "must match the regular
//!   expression `[A-Za-z_][A-Za-z0-9_]*`". Names in the prefix section 9.1 reserves for future
//!   versions are refused too; see [`reserved_prefix`] for why two prefixes are checked.
//! * **A value that does not conform to its `parameterType`.** Section 9.1: "There is a type
//!   inference check defined by the standard, which ensures that the `parameterType` matches.
//!   The check is not ensured by the XML validator and therefore must be implemented by the
//!   simulator." A declaration's `value` is an XSD `String`, so nothing upstream of this
//!   module checks it.
//! * **Two declarations of one name in one frame.** "The smallest scope that subsumes the
//!   location" picks between frames; it cannot pick between two declarations of the same
//!   frame, whose scopes are identical.
//! * **A declaration that references a parameter declared after it, or itself.** See below.
//!
//! # References inside a declaration
//!
//! Section 9.1 allows a declaration's `name`, `parameterType` or `value` to be a parameter
//! reference or an expression ("it is also generally allowed to use a reference as, for
//! example, `parameterType` on another `ParameterDeclaration`"; "Using a parameter reference in
//! the name field of another parameter declaration is allowed"), and advises against it ("it
//! is strongly advised not to do such chaining of parameters or mutual referencing, as this can
//! easily lead to deadlocks"). However, it never says which declarations such a reference may
//! see. This module reads the declarations of one `<ParameterDeclarations>` sequentially: a
//! reference sees the declarations before it in the same `<ParameterDeclarations>` and every
//! declaration of the enclosing elements. A reference to a later declaration of the same
//! `<ParameterDeclarations>`, or to the declaration itself, is an error naming both
//! parameters. It is an error even when an enclosing element declares the same name, since the
//! later declaration's scope is the smaller one and a reader would expect it to be the one
//! meant. Thus, a cycle cannot be written, and every document the rule accepts resolves in one
//! pass in document order. An expression that references nothing, such as `${1 + 2}`, is
//! evaluated like any other.
//!
//! The referenced value is substituted as text and the result is then checked against the
//! declaring parameter's own `parameterType`, so a `double` parameter holding `2.5` cannot
//! initialize an `int` one.

use crate::error::{Error, Result};
use crate::expression::{Expr, ExpressionParser, Operator};
use crate::types::basic::{
    is_valid_parameter_name, normalize_xsd_lexical, OSString, ParameterDeclaration,
    ParameterDeclarations, Value,
};
use crate::types::enums::ParameterType;
use serde::de::IntoDeserializer;
use serde::Deserialize;
use std::collections::HashMap;
use std::str::FromStr;

/// One visible parameter: its declared type and its value, already checked against that type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterBinding {
    /// The declared `parameterType`.
    pub parameter_type: ParameterType,
    /// The declared `value`, as written in the document.
    pub value: String,
}

/// The parameters visible at one point of a document, as a stack of declaration frames.
///
/// See the [module documentation](self) for the rules it applies.
#[derive(Debug, Clone)]
pub struct ParameterScope {
    /// `frames[0]` is the root frame holding the global parameters; the last frame is the
    /// innermost scope. Never empty.
    frames: Vec<HashMap<String, ParameterBinding>>,
}

impl Default for ParameterScope {
    fn default() -> Self {
        Self::new()
    }
}

impl ParameterScope {
    /// A scope holding only the (empty) root frame, where the global parameters are declared.
    pub fn new() -> Self {
        Self {
            frames: vec![HashMap::new()],
        }
    }

    /// The number of frames pushed above the root frame.
    pub fn depth(&self) -> usize {
        self.frames.len() - 1
    }

    /// Enter the subtree of an element that holds `<ParameterDeclarations>`.
    pub fn push_frame(&mut self) {
        self.frames.push(HashMap::new());
    }

    /// Leave the subtree entered by the matching [`push_frame`](Self::push_frame), discarding
    /// every declaration made in it.
    ///
    /// Popping the root frame is an error: it would leave the walker's pushes and pops
    /// unbalanced, and the global parameters with nowhere to live.
    pub fn pop_frame(&mut self) -> Result<()> {
        if self.frames.len() == 1 {
            return Err(Error::validation_error(
                "ParameterScope",
                "pop_frame called with no frame pushed; the root frame holds the global \
                 parameters and cannot be popped",
            ));
        }
        self.frames.pop();
        Ok(())
    }

    /// Declare a parameter in the innermost frame from the three attribute strings of a
    /// `<ParameterDeclaration>`, exactly as the XML holds them.
    ///
    /// A reference in any of the three resolves against what is visible now, as the module
    /// documentation describes. This call sees one declaration only, so it cannot tell a
    /// reference to a later sibling from a reference to an undeclared name; use
    /// [`declare_sequence`](Self::declare_sequence) for a whole `<ParameterDeclarations>`.
    ///
    /// Fails, naming the parameter, if the name is malformed or reserved, if the name is
    /// already declared in this frame, if `value` does not conform to `parameter_type`, or if
    /// a reference in the declaration cannot be resolved.
    pub fn declare(&mut self, name: &str, parameter_type: &str, value: &str) -> Result<()> {
        self.declare_one(name, parameter_type, value, &[])
            .map(|_| ())
    }

    /// Declare the entries of one `<ParameterDeclarations>` in the innermost frame, in document
    /// order, stopping at the first that fails. Each entry is `(name, parameterType, value)` as
    /// the XML holds them.
    ///
    /// Returns every declared parameter with its references resolved, in document order, so
    /// that a caller rewriting the document can write the literal values back.
    pub fn declare_sequence(
        &mut self,
        declarations: &[(&str, &str, &str)],
    ) -> Result<Vec<(String, ParameterBinding)>> {
        (0..declarations.len())
            .map(|index| self.declare_in_sequence(declarations, index))
            .collect()
    }

    /// Declare entry `index` of one `<ParameterDeclarations>`, whose entries are all given so
    /// that a reference to a later one is recognized as such. The entries before `index` must
    /// already be declared. [`declare_sequence`](Self::declare_sequence) is this call for every
    /// index in turn; a caller that reports errors per declaration calls it directly.
    pub fn declare_in_sequence(
        &mut self,
        declarations: &[(&str, &str, &str)],
        index: usize,
    ) -> Result<(String, ParameterBinding)> {
        // Only a literal later name can be recognized as a forward target; a later name that is
        // itself a reference is not known until that declaration is reached.
        let later: Vec<String> = declarations[index + 1..]
            .iter()
            .filter_map(|(later_name, _, _)| match classify(later_name) {
                Ok(Value::Literal(n)) => Some(n),
                _ => None,
            })
            .collect();
        let (name, parameter_type, value) = declarations[index];
        self.declare_one(name, parameter_type, value, &later)
    }

    fn declare_one(
        &mut self,
        raw_name: &str,
        raw_type: &str,
        raw_value: &str,
        later: &[String],
    ) -> Result<(String, ParameterBinding)> {
        let name = self.resolve_declaration_attribute(raw_name, raw_name, "name", later, false)?;
        check_name(&name)?;
        let parameter_type =
            self.resolve_declaration_attribute(&name, raw_type, "parameterType", later, false)?;
        let parameter_type = ParameterType::from_str(&parameter_type)
            .map_err(|e| Error::parameter_error(&name, &format!("invalid parameterType: {}", e)))?;
        let value = self.resolve_declaration_attribute(
            &name,
            raw_value,
            "value",
            later,
            parameter_type == ParameterType::Boolean,
        )?;
        check_conformance(&name, &parameter_type, &value)?;

        let frame = self
            .frames
            .last_mut()
            .expect("the root frame is never popped");
        if frame.contains_key(&name) {
            return Err(Error::parameter_error(
                &name,
                "declared twice in the same ParameterDeclarations; both declarations have the \
                 same scope, so neither is the one with the smallest scope",
            ));
        }
        let binding = ParameterBinding {
            parameter_type,
            value,
        };
        frame.insert(name.clone(), binding.clone());
        Ok((name, binding))
    }

    /// Resolve one attribute of the declaration of `parameter` to literal text, refusing a
    /// reference to the declaration itself, to a later sibling, or to anything not visible.
    fn resolve_declaration_attribute(
        &self,
        parameter: &str,
        raw: &str,
        attribute: &str,
        later: &[String],
        boolean_target: bool,
    ) -> Result<String> {
        let parsed = classify(raw).map_err(|e| {
            Error::parameter_error(
                parameter,
                &format!("invalid {} `{}`: {}", attribute, raw, e),
            )
        })?;
        if let Value::Literal(text) = parsed {
            return Ok(text);
        }
        for referenced in referenced_names(&parsed) {
            let problem = if referenced == parameter {
                Some("references the parameter being declared".to_string())
            } else if later.contains(&referenced) {
                Some(format!(
                    "references `{}`, which is declared after it in the same \
                     ParameterDeclarations; a declaration may reference only the declarations \
                     before it and those of enclosing elements",
                    referenced
                ))
            } else if self.lookup(&referenced).is_err() {
                let mut visible: Vec<String> = self.visible_values().into_keys().collect();
                visible.sort();
                Some(format!(
                    "references `{}`, which is not declared before it or in an enclosing \
                     element (visible here: {})",
                    referenced,
                    if visible.is_empty() {
                        "none".to_string()
                    } else {
                        visible.join(", ")
                    }
                ))
            } else {
                None
            };
            if let Some(problem) = problem {
                return Err(Error::parameter_error(
                    parameter,
                    &format!("its {} `{}` {}", attribute, raw, problem),
                ));
            }
        }
        self.evaluate(&parsed, Some(boolean_target)).map_err(|e| {
            Error::parameter_error(
                parameter,
                &format!("its {} `{}` could not be resolved: {}", attribute, raw, e),
            )
        })
    }

    /// Declare one parsed `ParameterDeclaration` in the innermost frame.
    ///
    /// Equivalent to [`declare`](Self::declare) on the attribute strings the declaration
    /// serializes to. Its `ConstraintGroup`s are not evaluated.
    pub fn declare_parsed(&mut self, declaration: &ParameterDeclaration) -> Result<()> {
        self.declare(
            &declaration.name.to_string(),
            &declaration.parameter_type.to_string(),
            &declaration.value.to_string(),
        )
    }

    /// Declare every entry of a parsed `<ParameterDeclarations>` in the innermost frame, in
    /// document order, stopping at the first that fails. The same as
    /// [`declare_sequence`](Self::declare_sequence) on the attribute strings the declarations
    /// serialize to.
    pub fn declare_all(&mut self, declarations: &ParameterDeclarations) -> Result<()> {
        let strings: Vec<[String; 3]> = declarations
            .parameter_declarations
            .iter()
            .map(|d| {
                [
                    d.name.to_string(),
                    d.parameter_type.to_string(),
                    d.value.to_string(),
                ]
            })
            .collect();
        let entries: Vec<(&str, &str, &str)> = strings
            .iter()
            .map(|[n, t, v]| (n.as_str(), t.as_str(), v.as_str()))
            .collect();
        self.declare_sequence(&entries).map(|_| ())
    }

    /// Resolve one attribute value as the XML holds it, at this point of the document.
    ///
    /// Returns `None` when the value is a literal, and the resolved text when it is a `$name`
    /// reference or an `${expression}`. The value is classified by the same rules the typed
    /// deserializer applies, so exactly the values it would store as a reference or expression
    /// are resolved here.
    ///
    /// The attribute's schema type is not known at this level, so an expression is evaluated as
    /// a Boolean when its outermost operation is `not`, `and`, `or` or a comparison, or when it
    /// is a single reference to a parameter declared `boolean`; otherwise it is evaluated as a
    /// number. A result of the wrong kind for the attribute fails later, when the typed
    /// deserializer reads it.
    pub fn resolve_attribute(&self, raw: &str) -> Result<Option<String>> {
        match classify(raw).map_err(|e| Error::parse_error(raw, &e.to_string()))? {
            Value::Literal(_) => Ok(None),
            parsed => self.evaluate(&parsed, None).map(Some),
        }
    }

    /// The text a classified value stands for here. `boolean_target` picks the expression
    /// evaluator; `None` infers it from the expression, as [`resolve_attribute`] describes.
    ///
    /// [`resolve_attribute`]: Self::resolve_attribute
    fn evaluate(&self, value: &Value<String>, boolean_target: Option<bool>) -> Result<String> {
        match value {
            Value::Literal(text) => Ok(text.clone()),
            Value::Parameter(name) => Ok(self.lookup(name)?.value.clone()),
            Value::Expression(expr) => {
                let boolean = boolean_target.unwrap_or_else(|| self.is_boolean_expression(expr));
                crate::expression::evaluate_expression_text(expr, &self.visible_values(), boolean)
            }
        }
    }

    fn is_boolean_expression(&self, expr: &str) -> bool {
        let Ok(ast) = ExpressionParser::new(expr).and_then(|mut p| p.parse()) else {
            // The evaluator reports the parse error itself.
            return false;
        };
        match ast {
            Expr::Not(_) | Expr::And(..) | Expr::Or(..) => true,
            Expr::BinaryOp { operator, .. } => matches!(
                operator,
                Operator::Greater
                    | Operator::Less
                    | Operator::GreaterEqual
                    | Operator::LessEqual
                    | Operator::Equal
                    | Operator::NotEqual
            ),
            Expr::Parameter(name) => self
                .lookup(&name)
                .is_ok_and(|b| b.parameter_type == ParameterType::Boolean),
            _ => false,
        }
    }

    /// The binding of `name` visible here: the one in the innermost frame that declares it.
    ///
    /// An undeclared name is [`Error::ParameterNotFound`], listing the names that are visible.
    pub fn lookup(&self, name: &str) -> Result<&ParameterBinding> {
        self.frames
            .iter()
            .rev()
            .find_map(|frame| frame.get(name))
            .ok_or_else(|| {
                let mut available: Vec<String> = self.visible_values().into_keys().collect();
                available.sort();
                Error::parameter_not_found(name, &available)
            })
    }

    /// The value of `name` visible here, or `None` if nothing declares it.
    pub fn get(&self, name: &str) -> Option<&str> {
        self.lookup(name).ok().map(|b| b.value.as_str())
    }

    /// Resolve a value at this point of the document: a literal as itself, a `$name` against
    /// the innermost binding of `name`, an `${expression}` against every visible binding.
    ///
    /// This is the preferred way to resolve a value against a scope. `Value<T>` has an inherent
    /// `resolve` method that takes a flat `HashMap`, but this method provides proper scoped
    /// resolution according to ASAM OpenSCENARIO section 9.1, with inner scopes shadowing outer
    /// ones and cross-scope visibility correctly handled.
    pub fn resolve<T>(&self, value: &Value<T>) -> Result<T>
    where
        T: FromStr + Clone,
        T::Err: std::fmt::Display,
    {
        match value {
            // Through `lookup`, so an undeclared name lists the names visible here.
            Value::Parameter(name) => {
                let binding = self.lookup(name)?;
                normalize_xsd_lexical::<T>(&binding.value)
                    .parse::<T>()
                    .map_err(|e| {
                        Error::parameter_error(
                            name,
                            &format!("failed to parse '{}': {}", binding.value, e),
                        )
                    })
            }
            _ => value.resolve(&self.visible_values()),
        }
    }

    /// Every visible parameter by name, with shadowed outer bindings left out.
    ///
    /// This is the flat map [`Value::resolve`] and the expression evaluator take.
    pub fn visible_values(&self) -> HashMap<String, String> {
        let mut visible = HashMap::new();
        for frame in &self.frames {
            for (name, binding) in frame {
                visible.insert(name.clone(), binding.value.clone());
            }
        }
        visible
    }
}

/// Classify an attribute value with the same rules the typed deserializer applies to an XSD
/// `String`: a literal, a `$name` reference, or an `${expression}`.
fn classify(raw: &str) -> std::result::Result<OSString, serde::de::value::Error> {
    OSString::deserialize(raw.into_deserializer())
}

/// The parameter names a classified value references. An expression that does not parse
/// references nothing here; evaluating it reports the parse error.
fn referenced_names(value: &Value<String>) -> Vec<String> {
    fn collect(expr: &Expr, names: &mut Vec<String>) {
        match expr {
            Expr::Parameter(name) => names.push(name.clone()),
            Expr::Number(_) | Expr::Constant(_) => {}
            Expr::BinaryOp { left, right, .. } | Expr::And(left, right) | Expr::Or(left, right) => {
                collect(left, names);
                collect(right, names);
            }
            Expr::UnaryMinus(inner) | Expr::Not(inner) => collect(inner, names),
            Expr::FunctionCall { args, .. } => args.iter().for_each(|a| collect(a, names)),
        }
    }
    match value {
        Value::Literal(_) => Vec::new(),
        Value::Parameter(name) => vec![name.clone()],
        Value::Expression(expr) => {
            let mut names = Vec::new();
            if let Ok(ast) = ExpressionParser::new(expr).and_then(|mut p| p.parse()) {
                collect(&ast, &mut names);
            }
            names
        }
    }
}

/// The prefixes section 9.1 reserves.
///
/// The published 1.3 and 1.4 text reads "Parameter names starting with *OSC* are reserved for
/// special use in future versions of ASAM OpenSCENARIO. Generally, the *OSC* prefix shall not
/// be used." -- with the prefix italicised. The specification is written in AsciiDoc, where
/// `_OSC_` is the markup for an italic *OSC*, so the source may equally have meant the literal
/// prefix `_OSC_`. The rendered page cannot tell the two apart, so both are refused.
pub fn reserved_prefix(name: &str) -> Option<&'static str> {
    ["_OSC_", "OSC"]
        .into_iter()
        .find(|prefix| name.starts_with(prefix))
}

fn check_name(name: &str) -> Result<()> {
    if !is_valid_parameter_name(name) {
        return Err(Error::parameter_error(
            name,
            "a parameter name must match [A-Za-z_][A-Za-z0-9_]* (ASAM OpenSCENARIO section 9.1)",
        ));
    }
    if let Some(prefix) = reserved_prefix(name) {
        return Err(Error::parameter_error(
            name,
            &format!(
                "names starting with `{}` are reserved for future versions of ASAM OpenSCENARIO \
                 (section 9.1)",
                prefix
            ),
        ));
    }
    Ok(())
}

/// Check a declared value against the lexical space of its `parameterType`'s XSD type.
fn check_conformance(name: &str, parameter_type: &ParameterType, value: &str) -> Result<()> {
    let conforms = match parameter_type {
        ParameterType::String => true,
        // `integer` is the deprecated spelling (XSD:323-325) that `int` replaced.
        ParameterType::Int | ParameterType::Integer => value.parse::<i32>().is_ok(),
        ParameterType::UnsignedInt => value.parse::<u32>().is_ok(),
        ParameterType::UnsignedShort => value.parse::<u16>().is_ok(),
        // Section 9.2.3: "ASAM OpenSCENARIO does not use NaN or infinity".
        ParameterType::Double => value.parse::<f64>().is_ok_and(f64::is_finite),
        // xsd:boolean's lexical space; section 9.2.2 also allows "Boolean literals to be
        // given as 0, 1, true, and false".
        ParameterType::Boolean => matches!(value, "true" | "false" | "1" | "0"),
        // xsd:dateTime: a timezone is optional.
        ParameterType::DateTime => {
            chrono::DateTime::parse_from_rfc3339(value).is_ok()
                || chrono::NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M:%S%.f").is_ok()
        }
    };
    if conforms {
        Ok(())
    } else {
        Err(Error::parameter_error(
            name,
            &format!(
                "declared {} but its value `{}` is not a valid {}",
                parameter_type, value, parameter_type
            ),
        ))
    }
}
