//! Resolution of a document's parameter references against its own declarations.
//!
//! ASAM OpenSCENARIO XML section 9.1 defines a parameter's scope on the XML tree: "the subtree
//! rooted in the element where the `ParameterDeclaration` is located". This module applies that
//! rule to the XML itself, before any typed deserialization. It reads the document into a tree,
//! walks it with a [`ParameterScope`], and replaces every attribute value that is a `$name`
//! reference or an `${expression}` by the text it resolves to. The resolved XML is then read by
//! the ordinary typed parser.
//!
//! Working on the XML rather than on the typed tree means every attribute of every element is
//! covered by the same code. A type added to the crate later is resolved without any change
//! here, whereas a visitor over the typed tree needs one implementation per type and silently
//! skips a type that lacks one.
//!
//! # What the pass does
//!
//! * An element with a `<ParameterDeclarations>` child opens a scope for its whole subtree,
//!   including its own attributes. The declarations of the root element are the global ones
//!   and go into the scope's root frame.
//! * The declarations are read in document order, as [`ParameterScope::declare_sequence`]
//!   describes, and written back with their references resolved. They are kept in the output
//!   rather than stripped, so the resolved document still says which parameters it declared
//!   and with which values; a caller comparing the resolved document with the source finds the
//!   same elements in both.
//! * Attributes of a `<ParameterDeclaration>` are declarations, not uses, and are resolved only
//!   through the scope. The name attribute of an element referring to a parameter by name, such
//!   as `parameterRef`, carries no `$` and is a literal to the pass.
//! * A `<CatalogReference>` and its whole subtree are passed through unchanged. Its
//!   `<ParameterAssignments>` assign values to the referenced catalog entry's parameters, which
//!   is resolution across two documents and is not done here.
//!
//! Every error names the element path and the source line of the element that failed, since
//! the name of an undeclared parameter alone does not locate it in a long scenario.

use crate::error::{Error, Result};
use crate::types::scope::ParameterScope;
use quick_xml::events::attributes::Attribute;
use quick_xml::events::{BytesStart, Event};
use quick_xml::{Reader, Writer};

/// Resolve every parameter reference and expression in an OpenSCENARIO document against the
/// document's own `<ParameterDeclarations>`, returning the resolved XML.
///
/// See the [module documentation](self) for the rules. The output is the input with attribute
/// values replaced; comments, whitespace and the unchanged attributes are written back as read.
pub fn resolve_parameters(xml: &str) -> Result<String> {
    let mut document = read_tree(xml)?;
    let mut scope = ParameterScope::new();
    let mut path = Vec::new();
    for node in &mut document {
        if let Node::Element(root) = node {
            resolve_element(root, &mut scope, &mut path, true)?;
        }
    }
    write_tree(&document)
}

/// One node of the document tree. Everything that is not an element is kept as the event it
/// was read as, so it can be written back unchanged.
enum Node {
    Element(Element),
    Other(Event<'static>),
}

struct Element {
    start: BytesStart<'static>,
    /// Written as `<Name/>` when true, `<Name>...</Name>` otherwise.
    empty: bool,
    children: Vec<Node>,
    /// The 1-based source line of the start tag.
    line: usize,
}

impl Element {
    fn name(&self) -> String {
        String::from_utf8_lossy(self.start.name().as_ref()).into_owned()
    }

    /// The unescaped value of the attribute `key`, if present.
    fn attribute(&self, key: &str) -> Result<Option<String>> {
        for attribute in self.start.attributes() {
            let attribute = attribute.map_err(|e| self.xml_error(&e.to_string()))?;
            if attribute.key.as_ref() == key.as_bytes() {
                let value = attribute
                    .unescape_value()
                    .map_err(|e| self.xml_error(&e.to_string()))?;
                return Ok(Some(value.into_owned()));
            }
        }
        Ok(None)
    }

    /// A path segment naming this element, with its `name` attribute when it has one, since
    /// that is what a reader searches a scenario for.
    fn segment(&self) -> String {
        match self.attribute("name") {
            Ok(Some(name)) => format!("{}[@name='{}']", self.name(), name),
            _ => self.name(),
        }
    }

    fn xml_error(&self, reason: &str) -> Error {
        Error::invalid_xml(&format!(
            "line {}: <{}>: {}",
            self.line,
            self.name(),
            reason
        ))
    }

    /// Rebuild the start tag with the attribute values in `replacements` substituted, keeping
    /// every other attribute's raw text.
    fn replace_attributes(&mut self, replacements: &[(String, String)]) -> Result<()> {
        if replacements.is_empty() {
            return Ok(());
        }
        let name = self.name();
        let mut rebuilt = BytesStart::new(name);
        for attribute in self.start.attributes() {
            let attribute = attribute.map_err(|e| self.xml_error(&e.to_string()))?;
            match replacements
                .iter()
                .find(|(key, _)| key.as_bytes() == attribute.key.as_ref())
            {
                // The `&str` pair form escapes the value; the resolved text is unescaped.
                Some((key, value)) => rebuilt.push_attribute((key.as_str(), value.as_str())),
                None => rebuilt.push_attribute(Attribute {
                    key: attribute.key,
                    value: attribute.value,
                }),
            }
        }
        self.start = rebuilt.into_owned();
        Ok(())
    }
}

const PARAMETER_DECLARATIONS: &str = "ParameterDeclarations";
const PARAMETER_DECLARATION: &str = "ParameterDeclaration";
const CATALOG_REFERENCE: &str = "CatalogReference";

fn resolve_element(
    element: &mut Element,
    scope: &mut ParameterScope,
    path: &mut Vec<String>,
    is_root: bool,
) -> Result<()> {
    if element.name() == CATALOG_REFERENCE {
        return Ok(());
    }
    path.push(element.segment());

    let declarations = element
        .children
        .iter()
        .position(|child| matches!(child, Node::Element(e) if e.name() == PARAMETER_DECLARATIONS));
    // The root element's declarations are the document's globals, which live in the scope's
    // root frame; every other declaring element opens a frame of its own.
    let pushed = declarations.is_some() && !is_root;
    if pushed {
        scope.push_frame();
    }
    if let Some(index) = declarations {
        if let Node::Element(declarations) = &mut element.children[index] {
            declare(declarations, scope, path)?;
        }
    }

    resolve_attributes(element, scope, path)?;
    for (index, child) in element.children.iter_mut().enumerate() {
        if Some(index) == declarations {
            continue;
        }
        if let Node::Element(child) = child {
            resolve_element(child, scope, path, false)?;
        }
    }

    if pushed {
        scope.pop_frame()?;
    }
    path.pop();
    Ok(())
}

/// Declare the entries of one `<ParameterDeclarations>` and write their resolved values back.
fn declare(
    declarations: &mut Element,
    scope: &mut ParameterScope,
    path: &mut Vec<String>,
) -> Result<()> {
    path.push(declarations.segment());
    let mut raw = Vec::new();
    for child in &declarations.children {
        if let Node::Element(declaration) = child {
            if declaration.name() == PARAMETER_DECLARATION {
                // A missing attribute is left for the typed deserializer to report; the empty
                // string stands in for it and fails the scope's own checks, naming the
                // declaration.
                raw.push([
                    declaration.attribute("name")?.unwrap_or_default(),
                    declaration.attribute("parameterType")?.unwrap_or_default(),
                    declaration.attribute("value")?.unwrap_or_default(),
                ]);
            }
        }
    }
    let entries: Vec<(&str, &str, &str)> = raw
        .iter()
        .map(|[n, t, v]| (n.as_str(), t.as_str(), v.as_str()))
        .collect();

    let mut index = 0;
    for child in &mut declarations.children {
        let Node::Element(declaration) = child else {
            continue;
        };
        if declaration.name() != PARAMETER_DECLARATION {
            continue;
        }
        path.push(declaration.segment());
        let (resolved_name, binding) = scope
            .declare_in_sequence(&entries, index)
            .map_err(|e| locate(e, path, declaration.line, None))?;
        let [name, parameter_type, value] = &raw[index];
        index += 1;

        let mut replacements = Vec::new();
        if resolved_name != *name {
            replacements.push(("name".to_string(), resolved_name));
        }
        if binding.parameter_type.to_string() != *parameter_type {
            replacements.push((
                "parameterType".to_string(),
                binding.parameter_type.to_string(),
            ));
        }
        if binding.value != *value {
            replacements.push(("value".to_string(), binding.value));
        }
        declaration.replace_attributes(&replacements)?;
        path.pop();
    }

    // A declaration's constraint groups are uses, and like every use in the subtree they see
    // all of the element's declarations.
    for child in &mut declarations.children {
        let Node::Element(declaration) = child else {
            continue;
        };
        path.push(declaration.segment());
        for grandchild in &mut declaration.children {
            if let Node::Element(e) = grandchild {
                resolve_element(e, scope, path, false)?;
            }
        }
        path.pop();
    }
    path.pop();
    Ok(())
}

fn resolve_attributes(
    element: &mut Element,
    scope: &ParameterScope,
    path: &[String],
) -> Result<()> {
    let mut replacements = Vec::new();
    for attribute in element.start.attributes() {
        let attribute = attribute.map_err(|e| element.xml_error(&e.to_string()))?;
        let raw = attribute
            .unescape_value()
            .map_err(|e| element.xml_error(&e.to_string()))?;
        let key = String::from_utf8_lossy(attribute.key.as_ref()).into_owned();
        if let Some(resolved) = scope
            .resolve_attribute(&raw)
            .map_err(|e| locate(e, path, element.line, Some((&key, &raw))))?
        {
            replacements.push((key, resolved));
        }
    }
    element.replace_attributes(&replacements)
}

/// Attach the element path, source line and, for a use, the attribute to a resolution error.
fn locate(error: Error, path: &[String], line: usize, attribute: Option<(&str, &str)>) -> Error {
    let location = format!("/{} (line {})", path.join("/"), line);
    match (error, attribute) {
        (Error::ParameterNotFound { param, available }, Some((key, raw))) => {
            let visible = if available.is_empty() {
                "none".to_string()
            } else {
                available.join(", ")
            };
            Error::parameter_error(
                &param,
                &format!(
                    "not declared; referenced by attribute `{}=\"{}\"` at {} (visible here: {})",
                    key, raw, location, visible
                ),
            )
        }
        (error, Some((key, raw))) => Error::parameter_error(
            raw,
            &format!(
                "attribute `{}` at {} could not be resolved: {}",
                key, location, error
            ),
        ),
        (Error::ParameterError { param, message }, None) => {
            Error::parameter_error(&param, &format!("declaration at {}: {}", location, message))
        }
        (error, None) => {
            Error::parameter_error(&format!("declaration at {}", location), &error.to_string())
        }
    }
}

fn read_tree(xml: &str) -> Result<Vec<Node>> {
    let mut reader = Reader::from_str(xml);
    // Everything is kept, so that only the resolved attribute values differ from the input.
    reader.config_mut().trim_text(false);

    // Elements under construction, outermost first; the bottom entry collects top-level nodes.
    let mut open: Vec<Element> = Vec::new();
    let mut top: Vec<Node> = Vec::new();
    let mut line = 1;
    let mut counted = 0;
    loop {
        let position = reader.buffer_position() as usize;
        line += xml.as_bytes()[counted..position]
            .iter()
            .filter(|b| **b == b'\n')
            .count();
        counted = position;

        let event = reader.read_event().map_err(|e| {
            Error::invalid_xml(&format!(
                "line {}: {} (at byte {})",
                line,
                e,
                reader.error_position()
            ))
        })?;
        let node = match event {
            Event::Eof => break,
            Event::Start(start) => {
                open.push(Element {
                    start: start.into_owned(),
                    empty: false,
                    children: Vec::new(),
                    line,
                });
                continue;
            }
            Event::End(_) => match open.pop() {
                Some(element) => Node::Element(element),
                None => {
                    return Err(Error::invalid_xml(&format!(
                        "line {}: unmatched end tag",
                        line
                    )))
                }
            },
            Event::Empty(start) => Node::Element(Element {
                start: start.into_owned(),
                empty: true,
                children: Vec::new(),
                line,
            }),
            other => Node::Other(other.into_owned()),
        };
        match open.last_mut() {
            Some(parent) => parent.children.push(node),
            None => top.push(node),
        }
    }
    if let Some(unclosed) = open.last() {
        return Err(unclosed.xml_error("element is not closed"));
    }
    Ok(top)
}

fn write_tree(nodes: &[Node]) -> Result<String> {
    fn write(writer: &mut Writer<Vec<u8>>, nodes: &[Node]) -> std::io::Result<()> {
        for node in nodes {
            match node {
                Node::Other(event) => writer.write_event(event.borrow())?,
                Node::Element(element) if element.empty => {
                    writer.write_event(Event::Empty(element.start.borrow()))?
                }
                Node::Element(element) => {
                    writer.write_event(Event::Start(element.start.borrow()))?;
                    write(writer, &element.children)?;
                    writer.write_event(Event::End(element.start.to_end()))?;
                }
            }
        }
        Ok(())
    }
    let mut writer = Writer::new(Vec::new());
    write(&mut writer, nodes).map_err(Error::from)?;
    String::from_utf8(writer.into_inner())
        .map_err(|e| Error::invalid_xml(&format!("resolved document is not UTF-8: {}", e)))
}
