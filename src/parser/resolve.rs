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
//! * A `<CatalogReference>` is replaced by the catalog entry it names, resolved with the
//!   reference's `<ParameterAssignments>`, as the next section describes.
//!
//! # Catalog references
//!
//! Section 9.6 resolves a reference "by locating the catalog by name and the entry within this
//! catalog by its entry name", and section 9.5 gives the parameters of that one use: "any
//! reference to "$x" should be replaced with "0", and any reference to "$y" should be replaced
//! with the default value of "7"". The resolved document says the same thing in XML. Each
//! `<CatalogReference>` is replaced in place by a copy of the entry, whose own references are
//! resolved for that use. Every element that holds a `<CatalogReference>` also accepts the
//! entry element in the same position, so the result is read by the typed parser like a
//! document that never used a catalog.
//!
//! * **Locating the entry.** The element holding the reference fixes the kind of entry: a
//!   `<ManeuverGroup>` takes a `Maneuver` from the `ManeuverCatalog` directory, a
//!   `<ScenarioObject>` a `Vehicle`, `Pedestrian` or `MiscObject` from the matching directories,
//!   and so on. Every `.xosc` file in such a directory whose `<Catalog>` carries the reference's
//!   `catalogName` is searched. A relative directory is taken relative to the base directory
//!   given to [`resolve_parameters`], which for a scenario file is the file's own directory.
//!   No entry, or more than one, is an error.
//! * **The entry's scope.** Section 9.5: "No other parameters may be referenced from within the
//!   catalog." The entry is therefore resolved in a scope of its own, holding nothing but its
//!   own declarations, and a reference to a parameter of the referencing document fails there.
//! * **Assignments.** An assignment's attributes belong to the referencing document, so a
//!   `$name` in its `value` is resolved in the document's scope at the reference. The entry's
//!   declarations are then read in order, and a declaration that an assignment names takes the
//!   assigned value in place of its default. Thus, a later default computed from an assigned
//!   parameter sees the assigned value. The model reference for `ParameterAssignment` defines
//!   `parameterRef` as the "name of the parameter that must be declared in the catalog", so an
//!   assignment naming anything else is an error, as are two assignments to one parameter,
//!   since the specification gives no rule for choosing between them. An assigned value is
//!   checked against the declared `parameterType` like a default.
//! * **The written entry** keeps its `<ParameterDeclarations>`, holding the values that were
//!   used, so the resolved document still says what each instance was given.
//! * **Nested references.** A reference inside an entry, such as a vehicle's trailer, is
//!   resolved in the entry's scope and located through the same catalog directories. A
//!   reference that reaches an entry already being resolved is an error, since resolving it
//!   would never end.
//!
//! Every error names the element path and the source line of the element that failed, since
//! the name of an undeclared parameter alone does not locate it in a long scenario.

use crate::catalog::CatalogResolver;
use crate::error::{Error, Result};
use crate::types::scope::ParameterScope;
use quick_xml::events::attributes::Attribute;
use quick_xml::events::{BytesStart, Event};
use quick_xml::{Reader, Writer};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Resolve every parameter reference and expression in an OpenSCENARIO document against the
/// document's own `<ParameterDeclarations>`, replacing each `<CatalogReference>` by the entry
/// it names, and return the resolved XML.
///
/// `base_dir` is the directory a relative catalog `Directory` path is taken from; for a
/// scenario read from a file, pass the file's directory.
///
/// See the [module documentation](self) for the rules. The output is the input with attribute
/// values replaced and catalog references expanded; comments, whitespace and the unchanged
/// attributes are written back as read.
pub fn resolve_parameters(xml: &str, base_dir: &Path) -> Result<String> {
    resolve_parameters_with_map(xml, base_dir).map(|(xml, _)| xml)
}

/// [`resolve_parameters`], additionally returning a [`LineMap`] from a line of the resolved
/// output back to the line of the document, or inlined catalog file, it came from. A typed
/// parse of the resolved text reports its own position against this text, not against what the
/// caller wrote; the map is how that position is translated back.
pub(crate) fn resolve_parameters_with_map(xml: &str, base_dir: &Path) -> Result<(String, LineMap)> {
    let mut document = read_tree(xml)?;
    let mut catalogs = Catalogs::new(base_dir);
    let mut scope = ParameterScope::new();
    let mut path = Vec::new();
    for node in &mut document {
        if let Node::Element(root) = node {
            resolve_element(root, &mut catalogs, &mut scope, &mut path, Frame::Root)?;
        }
    }
    write_tree_mapped(&document)
}

/// Resolve one catalog entry for a single reference made outside any document, and return the
/// entry's resolved XML with the catalog file it came from.
///
/// `location` names the `CatalogLocations` child whose directory is searched, such as
/// `VehicleCatalog`, and `assignments` are the reference's `(parameterRef, value)` pairs,
/// which must already be literal since no document scope surrounds them. The rules are the
/// ones the [module documentation](self) gives for a reference inside a document.
pub(crate) fn resolve_catalog_entry(
    location: &str,
    directory: &Path,
    catalog_name: &str,
    entry_name: &str,
    assignments: &[(String, String)],
) -> Result<(String, PathBuf)> {
    let mut kinds: Vec<(&'static str, &'static str)> = ENTRY_KINDS
        .iter()
        .flat_map(|(_, kinds)| kinds.iter().copied())
        .filter(|(loc, _)| *loc == location)
        .collect();
    // Several holders share one list of entry kinds, so the same pair can appear more than once.
    kinds.dedup();
    // `kinds` is never empty: every caller passes a `location` that is one of the
    // `CatalogLocations` names in `ENTRY_KINDS` (`src/catalog/mod.rs`'s `resolve_reference`
    // callers pass "VehicleCatalog", "ControllerCatalog", "PedestrianCatalog", all listed there).
    let mut catalogs = Catalogs::new(Path::new("."));
    catalogs
        .locations
        .insert(kinds[0].0, directory.to_path_buf());
    let assignments = assignments
        .iter()
        .map(|(name, value)| Assignment {
            name: name.clone(),
            value: value.clone(),
        })
        .collect();
    let mut path = Vec::new();
    let (entry, file) =
        catalogs.instantiate(&kinds, catalog_name, entry_name, assignments, &mut path)?;
    Ok((write_tree(&[Node::Element(entry)])?, file))
}

/// One node of the document tree. Everything that is not an element is kept as the event it
/// was read as, so it can be written back unchanged.
#[derive(Clone)]
enum Node {
    Element(Element),
    Other(Event<'static>),
}

#[derive(Clone)]
struct Element {
    start: BytesStart<'static>,
    /// Written as `<Name/>` when true, `<Name>...</Name>` otherwise.
    empty: bool,
    children: Vec<Node>,
    /// The 1-based source line of the start tag, in whichever document `catalog_source` names.
    line: usize,
    /// `None` for an element read from the document [`resolve_parameters`] was given; the
    /// catalog file it was read from when it is (or descends from) an inlined catalog entry.
    /// Set once, when a catalog file is first read in [`Catalogs::documents`], and carried
    /// unchanged through cloning and resolution so [`LineMap`] can tell the two apart.
    catalog_source: Option<PathBuf>,
}

impl Element {
    /// Where this element's start line sits, for [`LineMap`].
    fn source(&self) -> Source {
        match &self.catalog_source {
            None => Source::Document(self.line),
            Some(file) => Source::Catalog(file.clone(), self.line),
        }
    }
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
const CATALOG_LOCATIONS: &str = "CatalogLocations";

/// For each element that can hold a `<CatalogReference>` (`Schema/OpenSCENARIO.xsd`), the
/// entries it accepts, as pairs of the `CatalogLocations` child naming the directory and the
/// entry's element name. `Trailer` is the inner element of that name, whose type is
/// `ScenarioObject`.
const ENTRY_KINDS: &[(&str, &[(&str, &str)])] = &[
    ("ScenarioObject", ENTITY_ENTRIES),
    ("ScenarioObjectTemplate", ENTITY_ENTRIES),
    ("Trailer", ENTITY_ENTRIES),
    ("AssignControllerAction", CONTROLLER_ENTRIES),
    ("ControllerDistributionEntry", CONTROLLER_ENTRIES),
    ("ObjectController", CONTROLLER_ENTRIES),
    ("AssignRouteAction", ROUTE_ENTRIES),
    ("RouteRef", ROUTE_ENTRIES),
    ("FollowTrajectoryAction", TRAJECTORY_ENTRIES),
    ("TrajectoryRef", TRAJECTORY_ENTRIES),
    (
        "EnvironmentAction",
        &[("EnvironmentCatalog", "Environment")],
    ),
    ("ManeuverGroup", &[("ManeuverCatalog", "Maneuver")]),
];
const ENTITY_ENTRIES: &[(&str, &str)] = &[
    ("VehicleCatalog", "Vehicle"),
    ("PedestrianCatalog", "Pedestrian"),
    ("MiscObjectCatalog", "MiscObject"),
];
const CONTROLLER_ENTRIES: &[(&str, &str)] = &[("ControllerCatalog", "Controller")];
const ROUTE_ENTRIES: &[(&str, &str)] = &[("RouteCatalog", "Route")];
const TRAJECTORY_ENTRIES: &[(&str, &str)] = &[("TrajectoryCatalog", "Trajectory")];

/// Where an element sits, which decides where its declarations go.
#[derive(Clone, Copy)]
enum Frame<'a> {
    /// The document's root element, whose declarations are the global ones.
    Root,
    /// Any element below the root, which opens a frame of its own.
    Nested,
    /// A catalog entry being instantiated. It is the root of its own scope, and its
    /// declarations take the reference's assignments.
    Entry(&'a [Assignment]),
}

/// One `<ParameterAssignment>` of a reference, with both attributes already resolved in the
/// referencing scope.
#[derive(Clone)]
struct Assignment {
    name: String,
    value: String,
}

/// The catalog directories a document declares, and the catalog files read from them.
struct Catalogs {
    base_dir: PathBuf,
    /// Directory per `CatalogLocations` child, such as `VehicleCatalog`.
    locations: HashMap<&'static str, PathBuf>,
    /// The catalog documents of each directory, read on first use.
    loaded: HashMap<PathBuf, Vec<CatalogDocument>>,
    /// The entries being instantiated, so that a reference reaching one of them again is
    /// refused rather than followed forever.
    resolving: CatalogResolver,
}

/// One catalog file: its path, its `<Catalog name>` and its entries.
struct CatalogDocument {
    path: PathBuf,
    name: String,
    entries: Vec<Element>,
}

impl Catalogs {
    fn new(base_dir: &Path) -> Self {
        Self {
            base_dir: base_dir.to_path_buf(),
            locations: HashMap::new(),
            loaded: HashMap::new(),
            resolving: CatalogResolver::new(),
        }
    }

    /// Record the directories of a resolved `<CatalogLocations>`.
    fn record_locations(&mut self, locations: &Element) -> Result<()> {
        for child in &locations.children {
            let Node::Element(location) = child else {
                continue;
            };
            let name = location.name();
            let Some(key) = ENTRY_KINDS
                .iter()
                .flat_map(|(_, kinds)| kinds.iter())
                .map(|(loc, _)| *loc)
                .find(|loc| *loc == name)
            else {
                continue;
            };
            for grandchild in &location.children {
                if let Node::Element(directory) = grandchild {
                    if directory.name() == "Directory" {
                        if let Some(path) = directory.attribute("path")? {
                            self.locations.insert(key, self.base_dir.join(path));
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// The catalog documents in `directory`, reading them on first use. A file whose root holds
    /// no `<Catalog>` is some other kind of document and is skipped; a file that is not
    /// well-formed XML is an error, since skipping it could hide the entry being looked for.
    fn documents(&mut self, directory: &Path) -> Result<&[CatalogDocument]> {
        if !self.loaded.contains_key(directory) {
            let listing = std::fs::read_dir(directory).map_err(|e| {
                Error::catalog_error(&format!(
                    "cannot read catalog directory {}: {}",
                    directory.display(),
                    e
                ))
            })?;
            let mut files: Vec<PathBuf> = listing
                .filter_map(|entry| entry.ok().map(|e| e.path()))
                .filter(|p| p.is_file() && p.extension().is_some_and(|x| x == "xosc"))
                .collect();
            files.sort();
            let mut documents = Vec::new();
            for file in files {
                let text = std::fs::read_to_string(&file)
                    .map_err(|e| Error::file_read_error(&file.to_string_lossy(), &e.to_string()))?;
                let tree = read_tree(text.trim_start_matches('\u{feff}'))
                    .map_err(|e| e.with_context(&format!("catalog file {}", file.display())))?;
                for node in tree {
                    let Node::Element(root) = node else {
                        continue;
                    };
                    for child in root.children {
                        let Node::Element(catalog) = child else {
                            continue;
                        };
                        if catalog.name() != "Catalog" {
                            continue;
                        }
                        let name = catalog.attribute("name")?.unwrap_or_default();
                        let mut entries: Vec<Element> = catalog
                            .children
                            .into_iter()
                            .filter_map(|n| match n {
                                Node::Element(e) => Some(e),
                                Node::Other(_) => None,
                            })
                            .collect();
                        // Tagged once, here, rather than when an entry is inlined: an entry can
                        // itself be cloned and resolved more than once (one per reference), and
                        // every copy must still trace back to this file.
                        for entry in &mut entries {
                            tag_catalog_source(entry, &file);
                        }
                        documents.push(CatalogDocument {
                            path: file.clone(),
                            name,
                            entries,
                        });
                    }
                }
            }
            self.loaded.insert(directory.to_path_buf(), documents);
        }
        Ok(&self.loaded[directory])
    }

    /// Replace the reference `reference`, held by an element named `holder`, by the entry it
    /// names. `scope` is the referencing scope, in which the reference's own attributes resolve.
    fn expand(
        &mut self,
        reference: &mut Element,
        holder: &str,
        scope: &ParameterScope,
        path: &mut Vec<String>,
    ) -> Result<Element> {
        path.push(reference.segment());
        resolve_subtree_attributes(reference, scope, path)?;
        let catalog_name = reference.attribute("catalogName")?.unwrap_or_default();
        let entry_name = reference.attribute("entryName")?.unwrap_or_default();
        let mut assignments = Vec::new();
        for child in &reference.children {
            let Node::Element(list) = child else { continue };
            for grandchild in &list.children {
                let Node::Element(assignment) = grandchild else {
                    continue;
                };
                assignments.push(Assignment {
                    name: assignment.attribute("parameterRef")?.unwrap_or_default(),
                    value: assignment.attribute("value")?.unwrap_or_default(),
                });
            }
        }
        let kinds = ENTRY_KINDS
            .iter()
            .find(|(name, _)| *name == holder)
            .map(|(_, kinds)| *kinds)
            .ok_or_else(|| {
                reference.xml_error(&format!(
                    "a CatalogReference inside <{}> names no known kind of catalog entry",
                    holder
                ))
            })?;
        let (entry, _) = self
            .instantiate(kinds, &catalog_name, &entry_name, assignments, path)
            .map_err(|e| {
                e.with_context(&format!("/{} (line {})", path.join("/"), reference.line))
            })?;
        path.pop();
        Ok(entry)
    }

    /// Find the entry `entry_name` of the catalog `catalog_name` among `kinds`, and resolve a
    /// copy of it with `assignments` in a scope of its own. Returns the resolved entry and the
    /// catalog file holding it.
    fn instantiate(
        &mut self,
        kinds: &[(&'static str, &'static str)],
        catalog_name: &str,
        entry_name: &str,
        assignments: Vec<Assignment>,
        path: &mut Vec<String>,
    ) -> Result<(Element, PathBuf)> {
        let mut searched = Vec::new();
        let mut found: Vec<(Element, PathBuf)> = Vec::new();
        for (location, element_name) in kinds {
            let Some(directory) = self.locations.get(location).cloned() else {
                continue;
            };
            searched.push(format!("{} {}", location, directory.display()));
            for document in self.documents(&directory)? {
                if document.name != catalog_name {
                    continue;
                }
                for entry in &document.entries {
                    if entry.name() == *element_name
                        && entry.attribute("name")?.as_deref() == Some(entry_name)
                    {
                        found.push((entry.clone(), document.path.clone()));
                    }
                }
            }
        }
        let expected: Vec<&str> = kinds.iter().map(|(location, _)| *location).collect();
        let (mut entry, file) = match found.len() {
            1 => found.pop().expect("one entry"),
            0 if searched.is_empty() => {
                return Err(Error::catalog_error(&format!(
                    "entry `{}` of catalog `{}` cannot be located: CatalogLocations declares \
                     none of {}",
                    entry_name,
                    catalog_name,
                    expected.join(", ")
                )))
            }
            0 => {
                return Err(Error::catalog_error(&format!(
                    "catalog `{}` has no entry `{}` (searched {})",
                    catalog_name,
                    entry_name,
                    searched.join("; ")
                )))
            }
            _ => {
                let files: Vec<String> = found
                    .iter()
                    .map(|(_, file)| file.display().to_string())
                    .collect();
                return Err(Error::catalog_error(&format!(
                    "entry `{}` of catalog `{}` is defined {} times, in {}",
                    entry_name,
                    catalog_name,
                    found.len(),
                    files.join(", ")
                )));
            }
        };

        let key = format!("{}/{}", catalog_name, entry_name);
        if self.resolving.is_resolving(&key) {
            return Err(Error::circular_dependency(&format!(
                "entry `{}` of catalog `{}` references itself through {}",
                entry_name,
                catalog_name,
                path.join("/")
            )));
        }
        self.resolving.begin_resolution(&key)?;
        let mut scope = ParameterScope::new();
        let result = resolve_element(
            &mut entry,
            self,
            &mut scope,
            path,
            Frame::Entry(&assignments),
        );
        self.resolving.end_resolution(&key);
        result.map_err(|e| match e {
            Error::CircularDependency { .. } => e,
            e => e.with_context(&format!(
                "entry `{}` of catalog `{}` ({})",
                entry_name,
                catalog_name,
                file.display()
            )),
        })?;
        Ok((entry, file))
    }
}

fn resolve_element(
    element: &mut Element,
    catalogs: &mut Catalogs,
    scope: &mut ParameterScope,
    path: &mut Vec<String>,
    frame: Frame<'_>,
) -> Result<()> {
    path.push(element.segment());

    let declarations = element
        .children
        .iter()
        .position(|child| matches!(child, Node::Element(e) if e.name() == PARAMETER_DECLARATIONS));
    // The root element's declarations are the document's globals, which live in the scope's
    // root frame; a catalog entry is the root of a scope of its own. Every other declaring
    // element opens a frame.
    let pushed = declarations.is_some() && matches!(frame, Frame::Nested);
    if pushed {
        scope.push_frame();
    }
    let assignments = match frame {
        Frame::Entry(assignments) => assignments,
        Frame::Root | Frame::Nested => &[],
    };
    match declarations {
        Some(index) => {
            if let Node::Element(declarations) = &mut element.children[index] {
                declare(declarations, scope, path, assignments)?;
            }
        }
        None => check_assignments(assignments, &[])?,
    }

    resolve_attributes(element, scope, path)?;
    let holder = element.name();
    for (index, child) in element.children.iter_mut().enumerate() {
        if Some(index) == declarations {
            continue;
        }
        let Node::Element(child) = child else {
            continue;
        };
        if child.name() == CATALOG_REFERENCE {
            let entry = catalogs.expand(child, &holder, scope, path)?;
            *child = entry;
            continue;
        }
        resolve_element(child, catalogs, scope, path, Frame::Nested)?;
        if matches!(frame, Frame::Root) && child.name() == CATALOG_LOCATIONS {
            catalogs.record_locations(child)?;
        }
    }

    if pushed {
        scope.pop_frame()?;
    }
    path.pop();
    Ok(())
}

/// Refuse an assignment that names no parameter in `declared`, and two assignments naming the
/// same parameter.
fn check_assignments(assignments: &[Assignment], declared: &[String]) -> Result<()> {
    for (index, assignment) in assignments.iter().enumerate() {
        if assignments[..index]
            .iter()
            .any(|earlier| earlier.name == assignment.name)
        {
            return Err(Error::parameter_error(
                &assignment.name,
                "assigned more than once by the same CatalogReference; the specification \
                 gives no rule for choosing between the values",
            ));
        }
        if !declared.contains(&assignment.name) {
            let declared = if declared.is_empty() {
                "none".to_string()
            } else {
                declared.join(", ")
            };
            return Err(Error::parameter_error(
                &assignment.name,
                &format!(
                    "assigned by a CatalogReference but not declared by the catalog entry; a \
                     ParameterAssignment must name a parameter the entry declares (declared: {})",
                    declared
                ),
            ));
        }
    }
    Ok(())
}

/// Resolve the attributes of `element` and of every element below it in `scope`. Used for a
/// `<CatalogReference>`, whose subtree declares nothing.
fn resolve_subtree_attributes(
    element: &mut Element,
    scope: &ParameterScope,
    path: &mut Vec<String>,
) -> Result<()> {
    resolve_attributes(element, scope, path)?;
    for child in &mut element.children {
        if let Node::Element(child) = child {
            path.push(child.segment());
            resolve_subtree_attributes(child, scope, path)?;
            path.pop();
        }
    }
    Ok(())
}

/// Declare the entries of one `<ParameterDeclarations>` and write their resolved values back.
/// A declaration named by one of `assignments` takes the assigned value in place of its own.
fn declare(
    declarations: &mut Element,
    scope: &mut ParameterScope,
    path: &mut Vec<String>,
    assignments: &[Assignment],
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
    let mut entries: Vec<(&str, &str, &str)> = raw
        .iter()
        .map(|[n, t, v]| (n.as_str(), t.as_str(), v.as_str()))
        .collect();

    let mut declared = Vec::new();
    let mut index = 0;
    for child in &mut declarations.children {
        let Node::Element(declaration) = child else {
            continue;
        };
        if declaration.name() != PARAMETER_DECLARATION {
            continue;
        }
        path.push(declaration.segment());
        if !assignments.is_empty() {
            // The declarations before this one are in scope, so a name that is itself a
            // reference resolves here to the name an assignment would use. A name that fails
            // to resolve is left for the declaration itself to report.
            let name = match scope.resolve_attribute(entries[index].0) {
                Ok(Some(name)) => name,
                _ => entries[index].0.to_string(),
            };
            if let Some(assignment) = assignments.iter().find(|a| a.name == name) {
                entries[index].2 = assignment.value.as_str();
            }
        }
        let (resolved_name, binding) = scope
            .declare_in_sequence(&entries, index)
            .map_err(|e| locate(e, path, declaration.line, None))?;
        declared.push(resolved_name.clone());
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
    check_assignments(assignments, &declared)
        .map_err(|e| locate(e, path, declarations.line, None))?;

    // A declaration's constraint groups are uses, and like every use in the subtree they see
    // all of the element's declarations.
    for child in &mut declarations.children {
        let Node::Element(declaration) = child else {
            continue;
        };
        path.push(declaration.segment());
        for grandchild in &mut declaration.children {
            if let Node::Element(e) = grandchild {
                // A constraint group holds no catalog reference, so no catalog is needed.
                resolve_element(
                    e,
                    &mut Catalogs::new(Path::new(".")),
                    scope,
                    path,
                    Frame::Nested,
                )?;
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
                    catalog_source: None,
                });
                continue;
            }
            // `open` is never empty here: quick_xml's reader rejects a mismatched end tag
            // itself (as an `Err` from `read_event` above) before it ever yields the matching
            // `Event::End`, so every `Event::End` this loop sees has a pushed `Event::Start`.
            Event::End(_) => Node::Element(
                open.pop()
                    .expect("the XML reader rejects unmatched end tags"),
            ),
            Event::Empty(start) => Node::Element(Element {
                start: start.into_owned(),
                empty: true,
                children: Vec::new(),
                line,
                catalog_source: None,
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

/// Set `catalog_source` on `element` and every element below it, marking the whole subtree as
/// read from `file`. Called once, when a catalog file is loaded, so every copy later cloned out
/// of it for a reference keeps the tag.
fn tag_catalog_source(element: &mut Element, file: &Path) {
    element.catalog_source = Some(file.to_path_buf());
    for child in &mut element.children {
        if let Node::Element(child) = child {
            tag_catalog_source(child, file);
        }
    }
}

/// Where one line of [`resolve_parameters`]'s output came from.
#[derive(Clone)]
enum Source {
    /// Line `usize` of the document passed to [`resolve_parameters`].
    Document(usize),
    /// Line `usize` of the catalog file whose entry was inlined at this point.
    Catalog(PathBuf, usize),
}

impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Source::Document(line) => write!(f, "line {}", line),
            Source::Catalog(file, line) => {
                write!(f, "line {} of catalog file {}", line, file.display())
            }
        }
    }
}

/// Maps a line of [`resolve_parameters`]'s output back to where it came from.
///
/// Substituting a `$name` reference or an `${expression}` never changes how many lines a
/// document has, since the replacement text is written in place of the reference within the
/// same attribute value. Inlining a `<CatalogReference>` does: the entry replacing it can hold
/// more or fewer lines, so every line after it shifts. [`LineMap`] is built once, while writing
/// the resolved document, by recording the source of every element's start line; a line between
/// two such records shares the source of the one before it, offset by how many lines separate
/// them, since [`write_tree_mapped`] writes everything but a substituted attribute value back
/// byte for byte.
pub(crate) struct LineMap {
    /// `(output line, source of that line)`, in increasing order of the first field.
    breaks: Vec<(usize, Source)>,
}

impl LineMap {
    /// Where line `output_line` of the resolved document came from, as text fit for an error
    /// message: `"line N"` for the document itself, `"line N of catalog file ..."` for an
    /// inlined entry.
    pub(crate) fn locate(&self, output_line: usize) -> String {
        match self
            .breaks
            .iter()
            .rev()
            .find(|(line, _)| *line <= output_line)
        {
            Some((break_line, Source::Document(source_line))) => {
                Source::Document(source_line + (output_line - break_line)).to_string()
            }
            Some((break_line, Source::Catalog(file, source_line))) => {
                Source::Catalog(file.clone(), source_line + (output_line - break_line)).to_string()
            }
            // No element was ever written, so nothing was substituted either; the output line
            // is the source line.
            None => format!("line {}", output_line),
        }
    }
}

fn write_tree(nodes: &[Node]) -> Result<String> {
    write_tree_mapped(nodes).map(|(xml, _)| xml)
}

/// [`write_tree`], additionally returning a [`LineMap`] for the document it wrote.
fn write_tree_mapped(nodes: &[Node]) -> Result<(String, LineMap)> {
    fn write(
        writer: &mut Writer<Vec<u8>>,
        nodes: &[Node],
        line: &mut usize,
        breaks: &mut Vec<(usize, Source)>,
    ) -> std::io::Result<()> {
        fn count_lines(writer: &Writer<Vec<u8>>, written_from: usize, line: &mut usize) {
            *line += writer.get_ref()[written_from..]
                .iter()
                .filter(|b| **b == b'\n')
                .count();
        }
        for node in nodes {
            match node {
                Node::Other(event) => {
                    let start = writer.get_ref().len();
                    writer.write_event(event.borrow())?;
                    count_lines(writer, start, line);
                }
                Node::Element(element) if element.empty => {
                    breaks.push((*line, element.source()));
                    let start = writer.get_ref().len();
                    writer.write_event(Event::Empty(element.start.borrow()))?;
                    count_lines(writer, start, line);
                }
                Node::Element(element) => {
                    breaks.push((*line, element.source()));
                    let start = writer.get_ref().len();
                    writer.write_event(Event::Start(element.start.borrow()))?;
                    count_lines(writer, start, line);
                    write(writer, &element.children, line, breaks)?;
                    let start = writer.get_ref().len();
                    writer.write_event(Event::End(element.start.to_end()))?;
                    count_lines(writer, start, line);
                }
            }
        }
        Ok(())
    }
    let mut writer = Writer::new(Vec::new());
    let mut line = 1usize;
    let mut breaks = Vec::new();
    write(&mut writer, nodes, &mut line, &mut breaks).map_err(Error::from)?;
    let xml = String::from_utf8(writer.into_inner())
        .map_err(|e| Error::invalid_xml(&format!("resolved document is not UTF-8: {}", e)))?;
    Ok((xml, LineMap { breaks }))
}
