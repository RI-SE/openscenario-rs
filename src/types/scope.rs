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
//! * **A declaration whose `name`, `parameterType` or `value` is itself a parameter reference
//!   or an expression.** Section 9.1 allows it ("it is also generally allowed to use a
//!   reference as, for example, `parameterType` on another `ParameterDeclaration`"; "Using a
//!   parameter reference in the name field of another parameter declaration is allowed") and
//!   advises against it ("it is strongly advised not to do such chaining of parameters or
//!   mutual referencing, as this can easily lead to deadlocks"), but it never says which
//!   declarations such a reference may see: only the earlier ones in the same
//!   `<ParameterDeclarations>`, any of them, or only those of enclosing scopes. Each answer
//!   resolves some documents differently from the others, so rather than pick one this module
//!   reports the reference as unresolvable. The caller can resolve the value under whichever
//!   rule it adopts and declare the literal result.

use crate::error::{Error, Result};
use crate::types::basic::{
    is_valid_parameter_name, OSString, ParameterDeclaration, ParameterDeclarations, Value,
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
    /// Fails, naming the parameter, if the name is malformed or reserved, if the name is
    /// already declared in this frame, if `value` does not conform to `parameter_type`, or if
    /// any of the three is a parameter reference or expression (see the module documentation
    /// for why that last case is refused).
    pub fn declare(&mut self, name: &str, parameter_type: &str, value: &str) -> Result<()> {
        let name = literal_attribute(name, name, "name")?;
        check_name(&name)?;
        let parameter_type = literal_attribute(&name, parameter_type, "parameterType")?;
        let parameter_type = ParameterType::from_str(&parameter_type)
            .map_err(|e| Error::parameter_error(&name, &format!("invalid parameterType: {}", e)))?;
        let value = literal_attribute(&name, value, "value")?;
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
        frame.insert(
            name,
            ParameterBinding {
                parameter_type,
                value,
            },
        );
        Ok(())
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
    /// document order, stopping at the first that fails.
    pub fn declare_all(&mut self, declarations: &ParameterDeclarations) -> Result<()> {
        declarations
            .parameter_declarations
            .iter()
            .try_for_each(|d| self.declare_parsed(d))
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
    /// The same operation as [`Resolve::resolve`](crate::types::Resolve::resolve), reachable
    /// without naming the trait: `Value<T>` has an inherent `resolve` taking a flat map, which
    /// method-call syntax would pick instead.
    pub fn resolve<T>(&self, value: &Value<T>) -> Result<T>
    where
        T: FromStr + Clone,
        T::Err: std::fmt::Display,
    {
        match value {
            // Through `lookup`, so an undeclared name lists the names visible here.
            Value::Parameter(name) => {
                let binding = self.lookup(name)?;
                binding.value.parse::<T>().map_err(|e| {
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

/// Parse one attribute of a declaration with the same rules the typed deserializer applies to
/// an XSD `String`, and insist on a literal.
fn literal_attribute(parameter: &str, raw: &str, attribute: &str) -> Result<String> {
    let parsed: std::result::Result<OSString, serde::de::value::Error> =
        OSString::deserialize(raw.into_deserializer());
    match parsed {
        Ok(Value::Literal(s)) => Ok(s),
        Ok(Value::Parameter(_) | Value::Expression(_)) => Err(Error::parameter_error(
            parameter,
            &format!(
                "its {} `{}` is a parameter reference or an expression; ASAM OpenSCENARIO \
                 section 9.1 allows one in a declaration but does not say which declarations \
                 it may see, so it is not resolved here",
                attribute, raw
            ),
        )),
        Err(e) => Err(Error::parameter_error(
            parameter,
            &format!("invalid {} `{}`: {}", attribute, raw, e),
        )),
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
