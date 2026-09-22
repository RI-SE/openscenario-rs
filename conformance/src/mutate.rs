//! Builds schema-invalid variants of corpus files, so a gate can ask the question the other five
//! binaries cannot: **does the crate refuse what the schema refuses?**
//!
//! The corpus binaries all feed the crate valid documents, so none of them can observe the crate
//! accepting an invalid one. That is the direction in which serde fails silently: an unknown
//! element is ignored, a missing choice branch used to parse as all-`None`, and a repeated element
//! below its `minOccurs` produced an empty `Vec`. Each of those defects lived under a green gate.
//!
//! The engine here is deliberately approximate. It reads `Schema/OpenSCENARIO.xsd` to decide
//! *where* a mutation is likely to break a constraint, but it does not have to be right: every
//! mutant is judged by libxml2 against the same schema, and a mutant that stays valid is discarded
//! rather than tested. Thus the XSD reader only needs to be a good guide, while the oracle stays
//! exact.
//!
//! Nothing in this module names an element or a type. Every name comes out of the schema or out of
//! the document being mutated, so the gate keeps working when the schema moves to a later version.

use std::collections::{BTreeMap, BTreeSet};

use quick_xml::escape::escape;
use quick_xml::events::Event;
use quick_xml::Reader;

// --- a minimal XML tree ---------------------------------------------------------------------
//
// Mutation is a tree edit: remove a child, duplicate a child, insert a child, drop or rewrite an
// attribute. `xml_profile` reduces a document to a multiset of keys, which cannot be edited and
// written back, so this module carries its own tree. It keeps attribute order and text nodes, and
// drops comments and processing instructions, none of which affect validity.

/// A node of the parsed document: an element or a run of character data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Element(Element),
    Text(String),
}

/// One XML element: its tag name, its attributes in document order, and its children.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Element {
    pub name: String,
    pub attrs: Vec<(String, String)>,
    pub children: Vec<Node>,
}

impl Element {
    /// Parses a whole document and returns its root element.
    pub fn parse(xml: &str) -> Result<Element, String> {
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(false);
        let mut stack: Vec<Element> = Vec::new();
        let mut root: Option<Element> = None;
        loop {
            match reader.read_event() {
                Ok(Event::Start(e)) => stack.push(start_element(&e)?),
                Ok(Event::Empty(e)) => {
                    let el = start_element(&e)?;
                    close(&mut stack, &mut root, el);
                }
                Ok(Event::End(_)) => {
                    let el = stack
                        .pop()
                        .ok_or_else(|| "unbalanced end tag".to_string())?;
                    close(&mut stack, &mut root, el);
                }
                Ok(Event::Text(t)) => {
                    let text = t.xml_content().map_err(|e| e.to_string())?.into_owned();
                    if let Some(top) = stack.last_mut() {
                        top.children.push(Node::Text(text));
                    }
                }
                // quick-xml reports an entity reference inside character data as its own event
                // rather than folding it into the surrounding text, so it has to be resolved
                // here or `&amp;` in an element's content would be dropped.
                Ok(Event::GeneralRef(r)) => {
                    let text = match r.resolve_char_ref().map_err(|e| e.to_string())? {
                        Some(c) => c.to_string(),
                        None => {
                            let name = r.decode().map_err(|e| e.to_string())?.into_owned();
                            match name.as_str() {
                                "amp" => "&".to_string(),
                                "lt" => "<".to_string(),
                                "gt" => ">".to_string(),
                                "apos" => "'".to_string(),
                                "quot" => "\"".to_string(),
                                other => return Err(format!("unknown entity &{other};")),
                            }
                        }
                    };
                    if let Some(top) = stack.last_mut() {
                        top.children.push(Node::Text(text));
                    }
                }
                Ok(Event::CData(t)) => {
                    let text = String::from_utf8_lossy(t.as_ref()).into_owned();
                    if let Some(top) = stack.last_mut() {
                        top.children.push(Node::Text(text));
                    }
                }
                Ok(Event::Eof) => break,
                Ok(_) => {}
                Err(e) => return Err(e.to_string()),
            }
        }
        root.ok_or_else(|| "no root element".to_string())
    }

    /// Serializes the element as a standalone document, XML declaration included.
    pub fn to_document(&self) -> String {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        self.write(&mut out);
        out.push('\n');
        out
    }

    fn write(&self, out: &mut String) {
        out.push('<');
        out.push_str(&self.name);
        for (key, value) in &self.attrs {
            out.push(' ');
            out.push_str(key);
            out.push_str("=\"");
            out.push_str(&escape(value.as_str()));
            out.push('"');
        }
        if self.children.is_empty() {
            out.push_str("/>");
            return;
        }
        out.push('>');
        for child in &self.children {
            match child {
                Node::Element(e) => e.write(out),
                Node::Text(t) => out.push_str(&escape(t.as_str())),
            }
        }
        out.push_str("</");
        out.push_str(&self.name);
        out.push('>');
    }

    /// The child elements, paired with their index in `children` (which also holds text nodes, so
    /// the index is not the ordinal of the element among its element siblings).
    pub fn child_elements(&self) -> impl Iterator<Item = (usize, &Element)> {
        self.children
            .iter()
            .enumerate()
            .filter_map(|(i, c)| match c {
                Node::Element(e) => Some((i, e)),
                Node::Text(_) => None,
            })
    }

    /// The element reached by following `path`, each step an index into `children`.
    fn at_mut(&mut self, path: &[usize]) -> Option<&mut Element> {
        let mut here = self;
        for step in path {
            here = match here.children.get_mut(*step)? {
                Node::Element(e) => e,
                Node::Text(_) => return None,
            };
        }
        Some(here)
    }
}

fn start_element(e: &quick_xml::events::BytesStart<'_>) -> Result<Element, String> {
    let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
    let mut attrs = Vec::new();
    for attr in e.attributes() {
        let attr = attr.map_err(|e| e.to_string())?;
        let key = String::from_utf8_lossy(attr.key.as_ref()).into_owned();
        let value = attr
            .unescape_value()
            .map_err(|e| e.to_string())?
            .into_owned();
        attrs.push((key, value));
    }
    Ok(Element {
        name,
        attrs,
        children: Vec::new(),
    })
}

fn close(stack: &mut [Element], root: &mut Option<Element>, el: Element) {
    match stack.last_mut() {
        Some(parent) => parent.children.push(Node::Element(el)),
        None => {
            if root.is_none() {
                *root = Some(el);
            }
        }
    }
}

// --- the schema index -----------------------------------------------------------------------

/// How many times a particle may repeat. `None` is `maxOccurs="unbounded"`.
type Max = Option<u32>;

/// One `<xsd:attribute>` declaration.
#[derive(Debug, Clone)]
pub struct AttrDecl {
    pub name: String,
    pub type_name: String,
    pub required: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Compositor {
    Sequence,
    Choice,
    All,
}

#[derive(Debug, Clone)]
enum Item {
    Element {
        name: String,
        type_name: String,
        min: u32,
        max: Max,
    },
    GroupRef {
        name: String,
        min: u32,
    },
    Nested(ModelGroup),
}

#[derive(Debug, Clone)]
struct ModelGroup {
    compositor: Compositor,
    items: Vec<Item>,
}

/// One declared child element of a complex type, flattened out of the model group tree.
#[derive(Debug, Clone)]
pub struct ChildSlot {
    pub name: String,
    pub type_name: String,
    /// `minOccurs`, forced to 0 when an enclosing group or choice makes the particle optional.
    pub min: u32,
    pub max: Max,
    /// Which branch of which choice this element belongs to, if any.
    pub branch: Option<(usize, usize)>,
}

/// An `xsd:choice` in a type's content model, with each branch's element names.
///
/// A branch is not always one element: at the document root the branches are `xsd:group` refs that
/// expand to several sibling elements each, which is why a branch carries a set of names.
#[derive(Debug, Clone)]
pub struct ChoiceInfo {
    /// 0 when the choice may match the empty sequence, 1 otherwise.
    pub min: u32,
    pub branches: Vec<BTreeSet<String>>,
}

/// What a complex type allows: its child elements, its choices, and its attributes.
#[derive(Debug, Clone, Default)]
pub struct ChildModel {
    pub slots: Vec<ChildSlot>,
    pub choices: Vec<ChoiceInfo>,
    pub attrs: Vec<AttrDecl>,
}

impl ChildModel {
    /// The declaration for a child element named `name`.
    pub fn slot(&self, name: &str) -> Option<&ChildSlot> {
        self.slots.iter().find(|s| s.name == name)
    }

    /// The declaration for an attribute named `name`.
    pub fn attr(&self, name: &str) -> Option<&AttrDecl> {
        self.attrs.iter().find(|a| a.name == name)
    }
}

/// The parts of `Schema/OpenSCENARIO.xsd` a mutation needs: content models, attribute uses, and
/// enumeration facets.
#[derive(Debug, Default)]
pub struct Xsd {
    /// Global element declarations: element name to type name. The schema declares exactly one.
    roots: BTreeMap<String, String>,
    /// Flattened content model per complex type name.
    models: BTreeMap<String, ChildModel>,
    /// Simple type name to its enumeration values, for the types that restrict one.
    enums: BTreeMap<String, Vec<String>>,
}

impl Xsd {
    /// Reads and indexes the schema document.
    pub fn parse(xsd: &str) -> Result<Xsd, String> {
        let root = Element::parse(xsd)?;
        let mut types: BTreeMap<String, ModelGroup> = BTreeMap::new();
        let mut attrs: BTreeMap<String, Vec<AttrDecl>> = BTreeMap::new();
        let mut groups: BTreeMap<String, ModelGroup> = BTreeMap::new();
        let mut out = Xsd::default();

        for (_, child) in root.child_elements() {
            let Some(name) = attr_of(child, "name") else {
                continue;
            };
            match local_name(&child.name) {
                "element" => {
                    if let Some(type_name) = attr_of(child, "type") {
                        out.roots.insert(name.to_string(), type_name.to_string());
                    }
                }
                "simpleType" => {
                    let values = enumeration_values(child);
                    if !values.is_empty() {
                        out.enums.insert(name.to_string(), values);
                    }
                }
                "complexType" => {
                    if let Some(group) = first_model_group(child) {
                        types.insert(name.to_string(), group);
                    }
                    attrs.insert(name.to_string(), attribute_decls(child));
                }
                "group" => {
                    if let Some(group) = first_model_group(child) {
                        groups.insert(name.to_string(), group);
                    }
                }
                _ => {}
            }
        }

        if out.roots.is_empty() {
            return Err("the schema declares no global element".to_string());
        }

        for (name, decls) in &attrs {
            let mut model = ChildModel {
                attrs: decls.clone(),
                ..ChildModel::default()
            };
            if let Some(group) = types.get(name) {
                flatten(&groups, group, 1, None, &mut model, 0);
            }
            out.models.insert(name.clone(), model);
        }
        Ok(out)
    }

    /// The type of the document root element, by element name.
    pub fn root_type(&self, element: &str) -> Option<&str> {
        self.roots.get(element).map(String::as_str)
    }

    /// The flattened content model of a complex type.
    pub fn model(&self, type_name: &str) -> Option<&ChildModel> {
        self.models.get(type_name)
    }

    /// The enumeration values of a simple type, for the types that restrict one.
    pub fn enum_values(&self, type_name: &str) -> Option<&[String]> {
        self.enums.get(type_name).map(Vec::as_slice)
    }

    /// How many complex types and enumerated simple types were indexed, for the run header.
    pub fn sizes(&self) -> (usize, usize) {
        (self.models.len(), self.enums.len())
    }
}

fn local_name(qname: &str) -> &str {
    qname.rsplit(':').next().unwrap_or(qname)
}

fn attr_of<'a>(el: &'a Element, name: &str) -> Option<&'a str> {
    el.attrs
        .iter()
        .find(|(k, _)| local_name(k) == name)
        .map(|(_, v)| v.as_str())
}

fn enumeration_values(el: &Element) -> Vec<String> {
    let mut values = Vec::new();
    collect_enumerations(el, &mut values);
    values
}

fn collect_enumerations(el: &Element, out: &mut Vec<String>) {
    for (_, child) in el.child_elements() {
        if local_name(&child.name) == "enumeration" {
            if let Some(value) = attr_of(child, "value") {
                out.push(value.to_string());
            }
        }
        collect_enumerations(child, out);
    }
}

/// Every `<xsd:attribute>` below a complex type. They sit directly under the type, except in the
/// handful of `xsd:simpleContent` types where they hang off the `xsd:extension`.
fn attribute_decls(el: &Element) -> Vec<AttrDecl> {
    let mut out = Vec::new();
    collect_attribute_decls(el, &mut out);
    out
}

fn collect_attribute_decls(el: &Element, out: &mut Vec<AttrDecl>) {
    for (_, child) in el.child_elements() {
        match local_name(&child.name) {
            "attribute" => {
                if let (Some(name), Some(type_name)) =
                    (attr_of(child, "name"), attr_of(child, "type"))
                {
                    out.push(AttrDecl {
                        name: name.to_string(),
                        type_name: type_name.to_string(),
                        required: attr_of(child, "use") == Some("required"),
                    });
                }
            }
            "simpleContent" | "complexContent" | "extension" | "restriction" => {
                collect_attribute_decls(child, out)
            }
            _ => {}
        }
    }
}

fn first_model_group(el: &Element) -> Option<ModelGroup> {
    el.child_elements().find_map(|(_, c)| parse_model_group(c))
}

fn parse_model_group(el: &Element) -> Option<ModelGroup> {
    let compositor = match local_name(&el.name) {
        "sequence" => Compositor::Sequence,
        "choice" => Compositor::Choice,
        "all" => Compositor::All,
        _ => return None,
    };
    let mut items = Vec::new();
    for (_, child) in el.child_elements() {
        match local_name(&child.name) {
            "element" => {
                if let (Some(name), Some(type_name)) =
                    (attr_of(child, "name"), attr_of(child, "type"))
                {
                    items.push(Item::Element {
                        name: name.to_string(),
                        type_name: type_name.to_string(),
                        min: occurs_min(child),
                        max: occurs_max(child),
                    });
                }
            }
            "group" => {
                if let Some(name) = attr_of(child, "ref") {
                    items.push(Item::GroupRef {
                        name: name.to_string(),
                        min: occurs_min(child),
                    });
                }
            }
            "sequence" | "choice" | "all" => {
                if let Some(nested) = parse_model_group(child) {
                    items.push(Item::Nested(nested));
                }
            }
            _ => {}
        }
    }
    Some(ModelGroup { compositor, items })
}

fn occurs_min(el: &Element) -> u32 {
    attr_of(el, "minOccurs")
        .and_then(|v| v.parse().ok())
        .unwrap_or(1)
}

fn occurs_max(el: &Element) -> Max {
    match attr_of(el, "maxOccurs") {
        None => Some(1),
        Some("unbounded") => None,
        Some(v) => v.parse().ok(),
    }
}

/// Walks a model group tree and records one [`ChildSlot`] per declared child element.
///
/// `required` is 1 while the particle chain above is mandatory and 0 once anything on it is
/// optional, which is what makes an element inside `<xsd:group ref="…" minOccurs="0"/>` come out
/// with `min = 0` even though its own declaration says nothing.
///
/// `depth` guards against a group that refers to itself; the schema has no such cycle today, and
/// an unguarded recursion would hang the gate rather than fail it.
fn flatten(
    groups: &BTreeMap<String, ModelGroup>,
    group: &ModelGroup,
    required: u32,
    branch: Option<(usize, usize)>,
    out: &mut ChildModel,
    depth: u32,
) {
    if depth > 16 {
        return;
    }
    // A choice nested inside another choice's branch stays part of that branch: only the outermost
    // choice on a path decides cardinality, and re-registering the inner one would invent a second
    // constraint the schema does not state.
    if group.compositor == Compositor::Choice && branch.is_none() && group.items.len() > 1 {
        let choice = out.choices.len();
        out.choices.push(ChoiceInfo {
            min: required,
            branches: vec![BTreeSet::new(); group.items.len()],
        });
        for (index, item) in group.items.iter().enumerate() {
            flatten_item(groups, item, required, Some((choice, index)), out, depth);
        }
        return;
    }
    let required = if group.compositor == Compositor::Choice {
        0
    } else {
        required
    };
    for item in &group.items {
        flatten_item(groups, item, required, branch, out, depth);
    }
}

fn flatten_item(
    groups: &BTreeMap<String, ModelGroup>,
    item: &Item,
    required: u32,
    branch: Option<(usize, usize)>,
    out: &mut ChildModel,
    depth: u32,
) {
    match item {
        Item::Element {
            name,
            type_name,
            min,
            max,
        } => {
            if let Some((choice, index)) = branch {
                out.choices[choice].branches[index].insert(name.clone());
            }
            out.slots.push(ChildSlot {
                name: name.clone(),
                type_name: type_name.clone(),
                min: min * required,
                max: *max,
                branch,
            });
        }
        Item::GroupRef { name, min } => {
            if let Some(group) = groups.get(name) {
                let required = if *min == 0 { 0 } else { required };
                flatten(groups, group, required, branch, out, depth + 1);
            }
        }
        Item::Nested(group) => flatten(groups, group, required, branch, out, depth + 1),
    }
}

// --- resolving a document against the schema ------------------------------------------------

/// One element of a document, with the complex type the schema gives it.
#[derive(Debug, Clone)]
pub struct Site {
    /// Indices into `children`, from the root, naming the element.
    pub path: Vec<usize>,
    /// Slash-separated element names, for the report.
    pub element_path: String,
    pub type_name: String,
}

/// Every element of `root` whose type the schema can resolve, in document order.
///
/// Resolution is top-down from the global element declaration, so an element name that means
/// different types in different places is read correctly. A child the parent's type does not
/// declare is already a schema violation; its subtree is skipped and counted by the caller.
pub fn resolve(xsd: &Xsd, root: &Element) -> (Vec<Site>, usize) {
    let mut sites = Vec::new();
    let mut unresolved = 0usize;
    let Some(type_name) = xsd.root_type(&root.name) else {
        return (sites, 1);
    };
    let mut stack = vec![(Vec::new(), root.name.clone(), type_name.to_string())];
    // An explicit stack rather than recursion so that a deeply nested document cannot overflow.
    while let Some((path, element_path, type_name)) = stack.pop() {
        let Some(element) = element_at(root, &path) else {
            unresolved += 1;
            continue;
        };
        let Some(model) = xsd.model(&type_name) else {
            unresolved += 1;
            continue;
        };
        sites.push(Site {
            path: path.clone(),
            element_path: element_path.clone(),
            type_name: type_name.clone(),
        });
        for (index, child) in element.child_elements() {
            match model.slot(&child.name) {
                Some(slot) => {
                    let mut child_path = path.clone();
                    child_path.push(index);
                    stack.push((
                        child_path,
                        format!("{element_path}/{}", child.name),
                        slot.type_name.clone(),
                    ));
                }
                None => unresolved += 1,
            }
        }
    }
    sites.sort_by(|a, b| a.path.cmp(&b.path));
    (sites, unresolved)
}

fn element_at<'a>(root: &'a Element, path: &[usize]) -> Option<&'a Element> {
    let mut here = root;
    for step in path {
        here = match here.children.get(*step)? {
            Node::Element(e) => e,
            Node::Text(_) => return None,
        };
    }
    Some(here)
}

/// A valid instance of each complex type, harvested from the corpus, so a branch inserted by the
/// `double-choice` mutation carries content the schema accepts rather than an empty element the
/// schema would reject for a second, unrelated reason.
#[derive(Debug, Default)]
pub struct Donors {
    by_type: BTreeMap<String, Element>,
}

impl Donors {
    /// Records the elements of one schema-valid document, keeping the first instance seen of each
    /// type. First rather than largest, because the corpus is walked in sorted order and the gate
    /// has to produce the same mutants on every run.
    pub fn harvest(&mut self, xsd: &Xsd, root: &Element, sites: &[Site]) {
        for site in sites {
            if self.by_type.contains_key(&site.type_name) {
                continue;
            }
            if xsd.model(&site.type_name).is_none() {
                continue;
            }
            if let Some(element) = element_at(root, &site.path) {
                self.by_type.insert(site.type_name.clone(), element.clone());
            }
        }
    }

    /// An instance of `type_name` renamed to `element`, ready to insert.
    pub fn instance(&self, type_name: &str, element: &str) -> Option<Element> {
        let mut donor = self.by_type.get(type_name)?.clone();
        donor.name = element.to_string();
        Some(donor)
    }

    /// How many distinct types have a donor, for the run header.
    pub fn len(&self) -> usize {
        self.by_type.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_type.is_empty()
    }
}

// --- mutations ------------------------------------------------------------------------------

/// The five ways a document is broken, each aimed at one kind of XSD constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    DropRequired,
    ExceedMax,
    DoubleChoice,
    DropRequiredAttr,
    BadEnum,
}

impl Kind {
    pub const ALL: [Kind; 5] = [
        Kind::DropRequired,
        Kind::ExceedMax,
        Kind::DoubleChoice,
        Kind::DropRequiredAttr,
        Kind::BadEnum,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Kind::DropRequired => "drop-required",
            Kind::ExceedMax => "exceed-max",
            Kind::DoubleChoice => "double-choice",
            Kind::DropRequiredAttr => "drop-required-attr",
            Kind::BadEnum => "bad-enum",
        }
    }

    /// Parses a kind as written in `conformance/expectations.toml`. Not `FromStr`, because an
    /// unknown kind is a manifest error the caller reports with the list of known kinds, not a
    /// parse failure to propagate.
    pub fn parse(s: &str) -> Option<Kind> {
        Kind::ALL.into_iter().find(|k| k.as_str() == s)
    }
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The value `bad-enum` writes. It is outside every enumeration in the schema and is not
/// `$name`/`${expr}` shaped, so it cannot be read as the parameter member of the union either.
const NOT_AN_ENUM_VALUE: &str = "not-a-valid-enumeration-value";

/// One mutated document, and enough about where it was mutated to name a candidate issue.
#[derive(Debug, Clone)]
pub struct Mutant {
    pub kind: Kind,
    /// The XSD complex type of the element that was mutated.
    pub type_name: String,
    pub element_path: String,
    /// What was done, for the hole listing.
    pub detail: String,
    pub xml: String,
}

/// What a mutation does, resolved before any document is cloned.
#[derive(Debug, Clone)]
struct Edit {
    kind: Kind,
    site: usize,
    detail: String,
    action: Action,
}

#[derive(Debug, Clone)]
enum Action {
    RemoveChild(usize),
    DuplicateChild(usize),
    InsertChild(usize, Element),
    RemoveAttr(usize),
    SetAttr(usize, String),
}

/// Generates the mutants for one document: at most `cap` per kind, chosen deterministically.
///
/// The cap is per kind per file rather than per file, so a kind with few candidate sites is not
/// crowded out by one with many. `seed` should be derived from the file path, so the choice varies
/// across the corpus while staying identical between runs.
pub fn mutants(
    xsd: &Xsd,
    donors: &Donors,
    root: &Element,
    sites: &[Site],
    seed: u64,
    cap: usize,
) -> Vec<Mutant> {
    let mut out = Vec::new();
    for kind in Kind::ALL {
        let mut edits = candidates(xsd, donors, root, sites, kind);
        select(&mut edits, seed ^ fnv1a(kind.as_str()), cap);
        for edit in edits {
            let site = &sites[edit.site];
            let mut document = root.clone();
            let Some(target) = document.at_mut(&site.path) else {
                continue;
            };
            match &edit.action {
                Action::RemoveChild(index) => {
                    target.children.remove(*index);
                }
                Action::DuplicateChild(index) => {
                    let copy = target.children[*index].clone();
                    target.children.insert(*index + 1, copy);
                }
                Action::InsertChild(index, element) => {
                    target
                        .children
                        .insert(*index + 1, Node::Element(element.clone()));
                }
                Action::RemoveAttr(index) => {
                    target.attrs.remove(*index);
                }
                Action::SetAttr(index, value) => {
                    target.attrs[*index].1 = value.clone();
                }
            }
            out.push(Mutant {
                kind: edit.kind,
                type_name: site.type_name.clone(),
                element_path: site.element_path.clone(),
                detail: edit.detail,
                xml: document.to_document(),
            });
        }
    }
    out
}

fn candidates(xsd: &Xsd, donors: &Donors, root: &Element, sites: &[Site], kind: Kind) -> Vec<Edit> {
    let mut edits = Vec::new();
    for (index, site) in sites.iter().enumerate() {
        let (Some(element), Some(model)) =
            (element_at(root, &site.path), xsd.model(&site.type_name))
        else {
            continue;
        };
        match kind {
            Kind::DropRequired => drop_required(element, model, index, &mut edits),
            Kind::ExceedMax => exceed_max(element, model, index, &mut edits),
            Kind::DoubleChoice => double_choice(donors, element, model, index, &mut edits),
            Kind::DropRequiredAttr => drop_required_attr(element, model, index, &mut edits),
            Kind::BadEnum => bad_enum(xsd, element, model, index, &mut edits),
        }
    }
    edits
}

/// Counts, per child element name, how many times it occurs, and the index of its first
/// occurrence.
fn child_census(element: &Element) -> BTreeMap<&str, (usize, usize)> {
    let mut census: BTreeMap<&str, (usize, usize)> = BTreeMap::new();
    for (index, child) in element.child_elements() {
        census
            .entry(child.name.as_str())
            .and_modify(|(count, _)| *count += 1)
            .or_insert((1, index));
    }
    census
}

fn drop_required(element: &Element, model: &ChildModel, site: usize, out: &mut Vec<Edit>) {
    let census = child_census(element);
    for (name, (count, first)) in &census {
        let Some(slot) = model.slot(name) else {
            continue;
        };
        // Either the element's own particle is mandatory and removing one drops below minOccurs,
        // or it is the only branch present in a choice that must match something.
        let below_min = slot.min >= 1 && (*count as u32) <= slot.min;
        let empties_choice = slot.branch.is_some_and(|(choice, _)| {
            model.choices[choice].min >= 1 && branch_members(element, model, choice) == 1
        });
        if below_min || empties_choice {
            out.push(Edit {
                kind: Kind::DropRequired,
                site,
                detail: format!("removed <{name}>"),
                action: Action::RemoveChild(*first),
            });
        }
    }
}

/// How many child elements of `element` belong to any branch of choice `choice`.
fn branch_members(element: &Element, model: &ChildModel, choice: usize) -> usize {
    element
        .child_elements()
        .filter(|(_, c)| {
            model
                .slot(&c.name)
                .and_then(|s| s.branch)
                .is_some_and(|(id, _)| id == choice)
        })
        .count()
}

fn exceed_max(element: &Element, model: &ChildModel, site: usize, out: &mut Vec<Edit>) {
    for (name, (_, first)) in &child_census(element) {
        let Some(slot) = model.slot(name) else {
            continue;
        };
        if slot.max == Some(1) {
            out.push(Edit {
                kind: Kind::ExceedMax,
                site,
                detail: format!("duplicated <{name}>"),
                action: Action::DuplicateChild(*first),
            });
        }
    }
}

fn double_choice(
    donors: &Donors,
    element: &Element,
    model: &ChildModel,
    site: usize,
    out: &mut Vec<Edit>,
) {
    for (id, choice) in model.choices.iter().enumerate() {
        // Which branches the document has taken. A branch whose members repeat is still one
        // branch: two <EntityRef> elements are not two branches.
        let mut present: Vec<(usize, usize)> = Vec::new();
        for (index, child) in element.child_elements() {
            if let Some((c, b)) = model.slot(&child.name).and_then(|s| s.branch) {
                if c == id && !present.iter().any(|(taken, _)| *taken == b) {
                    present.push((b, index));
                }
            }
        }
        let [(taken, after)] = present[..] else {
            continue;
        };
        for (other, names) in choice.branches.iter().enumerate() {
            if other == taken {
                continue;
            }
            // Any one element of another branch is enough: the choice has already matched, so a
            // second branch's element has nowhere to go in the content model.
            for name in names {
                let Some(slot) = model.slot(name) else {
                    continue;
                };
                let Some(donor) = donors.instance(&slot.type_name, name) else {
                    continue;
                };
                out.push(Edit {
                    kind: Kind::DoubleChoice,
                    site,
                    detail: format!("added <{name}> beside the branch already taken"),
                    action: Action::InsertChild(after, donor),
                });
                break;
            }
        }
    }
}

fn drop_required_attr(element: &Element, model: &ChildModel, site: usize, out: &mut Vec<Edit>) {
    for (index, (name, _)) in element.attrs.iter().enumerate() {
        if model.attr(name).is_some_and(|a| a.required) {
            out.push(Edit {
                kind: Kind::DropRequiredAttr,
                site,
                detail: format!("removed the required attribute @{name}"),
                action: Action::RemoveAttr(index),
            });
        }
    }
}

fn bad_enum(xsd: &Xsd, element: &Element, model: &ChildModel, site: usize, out: &mut Vec<Edit>) {
    for (index, (name, value)) in element.attrs.iter().enumerate() {
        let Some(decl) = model.attr(name) else {
            continue;
        };
        let Some(values) = xsd.enum_values(&decl.type_name) else {
            continue;
        };
        // Skip an attribute that already holds a parameter reference: its value is not in the
        // enumeration either, and replacing it would test nothing new.
        if value.starts_with('$') || values.iter().all(|v| v != value) {
            continue;
        }
        out.push(Edit {
            kind: Kind::BadEnum,
            site,
            detail: format!("@{name} set outside the {} enumeration", decl.type_name),
            action: Action::SetAttr(index, NOT_AN_ENUM_VALUE.to_string()),
        });
    }
}

// --- deterministic selection ------------------------------------------------------------------

/// Keeps at most `cap` edits, chosen by a seeded shuffle and restored to document order.
///
/// A fixed seed rather than "the first `cap`" because the first candidates in a file cluster in
/// its header, and a gate that only ever mutates `FileHeader` would report a narrow slice of the
/// crate as sound.
fn select(edits: &mut Vec<Edit>, seed: u64, cap: usize) {
    if edits.len() <= cap {
        return;
    }
    let mut order: Vec<usize> = (0..edits.len()).collect();
    let mut state = seed | 1;
    for i in (1..order.len()).rev() {
        state = next(state);
        let j = (state % (i as u64 + 1)) as usize;
        order.swap(i, j);
    }
    order.truncate(cap);
    order.sort_unstable();
    let chosen: Vec<Edit> = order.iter().map(|i| edits[*i].clone()).collect();
    *edits = chosen;
}

/// A 64-bit linear congruential step (the constants are Knuth's MMIX). Written out rather than
/// taken from a crate because the gate needs a generator whose sequence is fixed forever, not one
/// that may change with a dependency bump.
fn next(state: u64) -> u64 {
    state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
}

/// FNV-1a. `std`'s default hasher is seeded per process for `HashMap`, so it cannot be used to
/// derive a seed that must be the same on every run and every machine.
pub fn fnv1a(s: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in s.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINI_XSD: &str = r#"<?xml version="1.0"?>
<xsd:schema xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <xsd:element name="Root" type="Root"/>
  <xsd:simpleType name="Colour">
    <xsd:union>
      <xsd:simpleType><xsd:restriction base="xsd:string">
        <xsd:enumeration value="red"/><xsd:enumeration value="blue"/>
      </xsd:restriction></xsd:simpleType>
    </xsd:union>
  </xsd:simpleType>
  <xsd:complexType name="Root">
    <xsd:sequence>
      <xsd:element name="Head" type="Head"/>
      <xsd:group ref="Body"/>
    </xsd:sequence>
    <xsd:attribute name="name" type="xsd:string" use="required"/>
    <xsd:attribute name="colour" type="Colour"/>
  </xsd:complexType>
  <xsd:group name="Body">
    <xsd:choice>
      <xsd:element name="Left" type="Head"/>
      <xsd:element name="Right" type="Head"/>
    </xsd:choice>
  </xsd:group>
  <xsd:complexType name="Head">
    <xsd:sequence>
      <xsd:element name="Note" type="Head" minOccurs="0" maxOccurs="unbounded"/>
    </xsd:sequence>
  </xsd:complexType>
</xsd:schema>"#;

    fn mini() -> Xsd {
        Xsd::parse(MINI_XSD).expect("mini schema")
    }

    /// A round trip through the tree keeps attributes, children, and escaped text.
    #[test]
    fn document_survives_parse_and_write() {
        let xml = r#"<Root name="a &amp; b"><Head/><Left>text</Left></Root>"#;
        let root = Element::parse(xml).expect("parse");
        assert_eq!(root.name, "Root");
        assert_eq!(root.attrs[0].1, "a & b");
        let written = root.to_document();
        assert!(written.contains(r#"name="a &amp; b""#), "{written}");
        assert!(written.contains("<Left>text</Left>"), "{written}");
        assert_eq!(Element::parse(&written).expect("reparse"), root);
    }

    /// An element inside a referenced group belongs to that group's choice, and the choice's
    /// branches are named by the elements they contain.
    #[test]
    fn group_reference_is_flattened_into_the_referring_type() {
        let xsd = mini();
        let model = xsd.model("Root").expect("Root");
        assert_eq!(model.slot("Head").expect("Head").min, 1);
        let left = model.slot("Left").expect("Left");
        assert_eq!(left.branch, Some((0, 0)));
        assert_eq!(model.choices.len(), 1);
        assert_eq!(model.choices[0].min, 1);
        assert_eq!(model.choices[0].branches[1].iter().next().unwrap(), "Right");
    }

    /// Required attributes are distinguished from optional ones, and enumerations are indexed.
    #[test]
    fn attribute_uses_and_enumerations_are_read() {
        let xsd = mini();
        let model = xsd.model("Root").expect("Root");
        assert!(model.attr("name").expect("name").required);
        assert!(!model.attr("colour").expect("colour").required);
        assert_eq!(xsd.enum_values("Colour").expect("Colour"), ["red", "blue"]);
    }

    fn mutate_all(xml: &str) -> Vec<Mutant> {
        let xsd = mini();
        let root = Element::parse(xml).expect("parse");
        let (sites, unresolved) = resolve(&xsd, &root);
        assert_eq!(unresolved, 0, "every element should resolve");
        let mut donors = Donors::default();
        donors.harvest(&xsd, &root, &sites);
        mutants(&xsd, &donors, &root, &sites, 1, 8)
    }

    /// Each mutation kind fires on the shape it is aimed at, and none fires on a shape it is not.
    #[test]
    fn every_kind_produces_the_mutation_it_describes() {
        let muts = mutate_all(r#"<Root name="r" colour="red"><Head/><Left/></Root>"#);
        let of = |k: Kind| -> Vec<&Mutant> { muts.iter().filter(|m| m.kind == k).collect() };

        let dropped = of(Kind::DropRequired);
        assert_eq!(dropped.len(), 2, "Head and the taken branch Left");
        assert!(dropped.iter().any(|m| m.detail == "removed <Head>"));
        assert!(dropped.iter().any(|m| m.detail == "removed <Left>"));

        assert_eq!(
            of(Kind::ExceedMax).len(),
            2,
            "Head and Left are maxOccurs=1"
        );

        let doubled = of(Kind::DoubleChoice);
        assert_eq!(doubled.len(), 1);
        assert!(doubled[0].xml.contains("<Right/>"), "{}", doubled[0].xml);

        let attrs = of(Kind::DropRequiredAttr);
        assert_eq!(attrs.len(), 1, "only @name is use=required");
        assert!(!attrs[0].xml.contains("name=\"r\""));

        let enums = of(Kind::BadEnum);
        assert_eq!(enums.len(), 1);
        assert!(enums[0].xml.contains(NOT_AN_ENUM_VALUE), "{}", enums[0].xml);
    }

    /// An optional, repeatable element is not a `drop-required` or `exceed-max` candidate: the
    /// oracle would discard such a mutant as still valid, and generating it wastes the cap.
    #[test]
    fn optional_repeated_children_are_not_candidates() {
        let muts = mutate_all(r#"<Root name="r"><Head><Note/></Head><Left/></Root>"#);
        assert!(!muts
            .iter()
            .any(|m| m.detail.contains("<Note>") && m.kind != Kind::DoubleChoice));
    }

    /// A branch that repeats is still one branch, so `double-choice` does not fire on it.
    #[test]
    fn a_repeated_branch_is_one_branch() {
        let xsd = mini();
        let root = Element::parse(r#"<Root name="r"><Head/><Left/><Left/></Root>"#).expect("parse");
        let (sites, _) = resolve(&xsd, &root);
        let mut donors = Donors::default();
        donors.harvest(&xsd, &root, &sites);
        let muts = mutants(&xsd, &donors, &root, &sites, 1, 8);
        let doubled: Vec<&Mutant> = muts
            .iter()
            .filter(|m| m.kind == Kind::DoubleChoice)
            .collect();
        assert_eq!(doubled.len(), 1, "one <Right> added, not one per <Left>");
    }

    /// The selection is a function of the seed alone, so two runs of the gate agree.
    #[test]
    fn selection_is_deterministic_and_respects_the_cap() {
        let xsd = mini();
        let root =
            Element::parse(r#"<Root name="r" colour="red"><Head/><Left/></Root>"#).expect("parse");
        let (sites, _) = resolve(&xsd, &root);
        let mut donors = Donors::default();
        donors.harvest(&xsd, &root, &sites);
        let first = mutants(&xsd, &donors, &root, &sites, 7, 1);
        let second = mutants(&xsd, &donors, &root, &sites, 7, 1);
        let detail = |v: &[Mutant]| -> Vec<String> { v.iter().map(|m| m.detail.clone()).collect() };
        assert_eq!(detail(&first), detail(&second));
        for kind in Kind::ALL {
            assert!(first.iter().filter(|m| m.kind == kind).count() <= 1);
        }
        let other = mutants(&xsd, &donors, &root, &sites, 99, 1);
        assert_eq!(
            other.len(),
            first.len(),
            "the cap does not depend on the seed"
        );
    }
}
