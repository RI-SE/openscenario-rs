//! Structural XML comparison and shared reporting helpers.
//!
//! Both sides of a comparison are reduced to a multiset of `path/to/Element`,
//! `path/to/Element@attr` and `path/to/Element#text` keys, so attribute order and whitespace
//! don't matter. Anything present on the left but not the right is *dropped*; anything on the
//! right but not the left is *invented*.
//!
//! The `#text` keys track character content. Without them `profile()` could not see character
//! content at all, so a document carrying loose text in an element that forbids it parsed,
//! silently lost the text, and was certified *lossless* by the one gate whose entire job is to
//! notice that nothing was dropped. An unobservable difference must not be reported as no
//! difference.
//!
//! `lossy` uses this to compare an original corpus file against the crate's re-serialization of
//! it; the builder harness uses the same comparator to measure how much of a real scenario a
//! hand-written builder program fails to reproduce.

use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::BTreeMap;

/// Attributes that are XML plumbing rather than OpenSCENARIO content. The crate deliberately
/// doesn't model these, so counting them as "dropped" would be noise.
pub fn is_plumbing(attr: &str) -> bool {
    attr.starts_with("xmlns") || attr.starts_with("xsi:")
}

/// Reduces XML to a multiset of `path/to/Element`, `path/to/Element@attr` and
/// `path/to/Element#text` keys.
pub fn profile(xml: &str) -> Result<BTreeMap<String, i64>, String> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut counts: BTreeMap<String, i64> = BTreeMap::new();
    let mut stack: Vec<String> = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => return Err(format!("{e}")),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                push_element(&mut counts, &mut stack, &e, true);
            }
            Ok(Event::Empty(e)) => {
                push_element(&mut counts, &mut stack, &e, false);
            }
            Ok(Event::End(_)) => {
                stack.pop();
            }
            Ok(Event::Text(e)) => note_text(&mut counts, &stack, e.as_ref()),
            Ok(Event::CData(e)) => note_text(&mut counts, &stack, e.as_ref()),
            _ => {}
        }
        buf.clear();
    }
    Ok(counts)
}

/// Records one character-content node under the element currently on the stack, as a
/// `path/to/Element#text` key.
///
/// `trim_text(true)` is set on the reader, but confirm what that actually leaves before relying
/// on it: it drops a text event whose content is *entirely* whitespace, and trims the ends of one
/// that is not, so what reaches here is every text node with at least one non-whitespace
/// character. It does not apply to `CDATA` at all, hence the explicit re-trim below.
///
/// Keyed by path and counted, never keyed by value. The question this gate asks is "did anything
/// disappear", and a per-path count answers it without making the key sensitive to escaping or
/// to internal whitespace, which serialization is free to change.
fn note_text(counts: &mut BTreeMap<String, i64>, stack: &[String], raw: &[u8]) {
    if String::from_utf8_lossy(raw).trim().is_empty() {
        return;
    }
    let mut key = stack.join("/");
    key.push_str("#text");
    *counts.entry(key).or_insert(0) += 1;
}

fn push_element(
    counts: &mut BTreeMap<String, i64>,
    stack: &mut Vec<String>,
    e: &quick_xml::events::BytesStart<'_>,
    keep_on_stack: bool,
) {
    let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
    stack.push(name);
    let path = stack.join("/");
    *counts.entry(path.clone()).or_insert(0) += 1;

    for attr in e.attributes().flatten() {
        let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
        if is_plumbing(&key) {
            continue;
        }
        *counts.entry(format!("{path}@{key}")).or_insert(0) += 1;
    }

    if !keep_on_stack {
        stack.pop();
    }
}

/// Keys present on one side of a comparison but not the other.
pub struct Diff {
    pub dropped: Vec<(String, i64)>,
    pub invented: Vec<(String, i64)>,
}

impl Diff {
    /// Total number of dropped keys, counting multiplicity.
    pub fn dropped_count(&self) -> i64 {
        self.dropped.iter().map(|(_, n)| n).sum()
    }

    /// Total number of invented keys, counting multiplicity.
    pub fn invented_count(&self) -> i64 {
        self.invented.iter().map(|(_, n)| n).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.dropped.is_empty() && self.invented.is_empty()
    }
}

pub fn diff(original: &BTreeMap<String, i64>, produced: &BTreeMap<String, i64>) -> Diff {
    let mut dropped = Vec::new();
    let mut invented = Vec::new();

    for (key, &n) in original {
        let m = produced.get(key).copied().unwrap_or(0);
        if n > m {
            dropped.push((key.clone(), n - m));
        }
    }
    for (key, &m) in produced {
        let n = original.get(key).copied().unwrap_or(0);
        if m > n {
            invented.push((key.clone(), m - n));
        }
    }
    Diff { dropped, invented }
}

/// One element instance's content: its attribute values (sorted by attribute name, plumbing
/// excluded) and its trimmed text nodes, in document order.
///
/// `profile()` answers "is anything missing"; this answers "is what's there the same value". A
/// rewrite (`${pi}` reserialized as `$pi`, `1.0` reserialized as `1`) touches no key `profile()`
/// counts, so it is invisible there by construction.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ElementValue {
    pub attrs: Vec<(String, String)>,
    pub text: Vec<String>,
}

/// Reduces XML to a map from `path/to/Element` to the ordered, per-instance list of
/// [`ElementValue`]s at that path, in document order.
///
/// Keyed the same way as `profile()`, so two same-named siblings under one parent land as two
/// entries in one list — position 0 is the first `<Event>` in the document, position 1 the
/// second — while two differently-named siblings never share a list at all. That is what makes
/// position-by-position comparison sound against reordering: swapping differently-named children
/// of an `xsd:all` group changes no per-path list, but swapping same-named siblings changes what
/// sits at each position, and must.
pub fn values(xml: &str) -> Result<BTreeMap<String, Vec<ElementValue>>, String> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut out: BTreeMap<String, Vec<ElementValue>> = BTreeMap::new();
    let mut name_stack: Vec<String> = Vec::new();
    // Path and index into `out[path]` for the element open at each stack depth, so a text node
    // can find the `ElementValue` it belongs to. Only `Start` elements (not self-closing `Empty`
    // ones, which cannot contain text) are pushed here.
    let mut open_stack: Vec<(String, usize)> = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => return Err(format!("{e}")),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => push_value(&mut out, &mut name_stack, &mut open_stack, &e, true),
            Ok(Event::Empty(e)) => {
                push_value(&mut out, &mut name_stack, &mut open_stack, &e, false)
            }
            Ok(Event::End(_)) => {
                name_stack.pop();
                open_stack.pop();
            }
            Ok(Event::Text(e)) => note_value_text(&mut out, &open_stack, e.as_ref()),
            Ok(Event::CData(e)) => note_value_text(&mut out, &open_stack, e.as_ref()),
            _ => {}
        }
        buf.clear();
    }
    Ok(out)
}

fn push_value(
    out: &mut BTreeMap<String, Vec<ElementValue>>,
    name_stack: &mut Vec<String>,
    open_stack: &mut Vec<(String, usize)>,
    e: &quick_xml::events::BytesStart<'_>,
    keep_on_stack: bool,
) {
    let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
    name_stack.push(name);
    let path = name_stack.join("/");

    let mut attrs: Vec<(String, String)> = e
        .attributes()
        .flatten()
        .filter_map(|attr| {
            let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
            if is_plumbing(&key) {
                return None;
            }
            let value = attr.unescape_value().ok()?.into_owned();
            Some((key, value))
        })
        .collect();
    attrs.sort_by(|a, b| a.0.cmp(&b.0));

    let list = out.entry(path.clone()).or_default();
    list.push(ElementValue {
        attrs,
        text: Vec::new(),
    });
    let idx = list.len() - 1;

    if keep_on_stack {
        open_stack.push((path, idx));
    } else {
        name_stack.pop();
    }
}

fn note_value_text(
    out: &mut BTreeMap<String, Vec<ElementValue>>,
    open_stack: &[(String, usize)],
    raw: &[u8],
) {
    let text = String::from_utf8_lossy(raw).trim().to_string();
    if text.is_empty() {
        return;
    }
    let Some((path, idx)) = open_stack.last() else {
        return;
    };
    if let Some(el) = out.get_mut(path).and_then(|list| list.get_mut(*idx)) {
        el.text.push(text);
    }
}

/// Whether two scalar values (an attribute value or a text node) count as the same value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ValueEq {
    /// String-equal.
    Same,
    /// Not string-equal, but both parse as `f64` and compare equal — `1.0` vs `1`, `1e2` vs
    /// `100`. The XSD types here are numeric, so the value space is what matters, not the
    /// lexical form a float formatter happened to choose.
    NumberFormatOnly,
    /// A genuine rewrite.
    Different,
}

fn compare_scalar(before: &str, after: &str) -> ValueEq {
    if before == after {
        return ValueEq::Same;
    }
    match (before.parse::<f64>(), after.parse::<f64>()) {
        (Ok(a), Ok(b)) if a == b => ValueEq::NumberFormatOnly,
        _ => ValueEq::Different,
    }
}

/// One value that changed between input and output at a given location (`path@attr` or
/// `path#text`).
pub struct ValueDiff {
    pub mismatches: Vec<(String, String, String)>,
    /// Rewrites that differ only in numeric formatting (`1.0` vs `1`). Counted separately because
    /// they do not fail the gate; see [`ValueEq::NumberFormatOnly`].
    pub number_format_only: usize,
}

impl ValueDiff {
    pub fn is_empty(&self) -> bool {
        self.mismatches.is_empty()
    }
}

/// Compares two [`values`] maps position by position within each path's list. A path present in
/// only one map, or a list whose lengths differ, contributes nothing here: `profile()`/`diff()`
/// already reports that as a dropped or invented key, and re-reporting it as a "value change"
/// would double-count the same defect under two names. Likewise an attribute present on only one
/// side of a matched pair is `profile()`'s finding, not this one — only a name common to both
/// sides can have its *value* compared.
pub fn compare_values(
    original: &BTreeMap<String, Vec<ElementValue>>,
    produced: &BTreeMap<String, Vec<ElementValue>>,
) -> ValueDiff {
    let mut mismatches = Vec::new();
    let mut number_format_only = 0usize;

    for (path, before_list) in original {
        let Some(after_list) = produced.get(path) else {
            continue;
        };
        for (before, after) in before_list.iter().zip(after_list.iter()) {
            let before_attrs: BTreeMap<&str, &str> = before
                .attrs
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect();
            let after_attrs: BTreeMap<&str, &str> = after
                .attrs
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect();
            for (name, before_val) in &before_attrs {
                let Some(after_val) = after_attrs.get(name) else {
                    continue;
                };
                match compare_scalar(before_val, after_val) {
                    ValueEq::Same => {}
                    ValueEq::NumberFormatOnly => number_format_only += 1,
                    ValueEq::Different => mismatches.push((
                        format!("{path}@{name}"),
                        before_val.to_string(),
                        after_val.to_string(),
                    )),
                }
            }
            for (before_text, after_text) in before.text.iter().zip(after.text.iter()) {
                match compare_scalar(before_text, after_text) {
                    ValueEq::Same => {}
                    ValueEq::NumberFormatOnly => number_format_only += 1,
                    ValueEq::Different => mismatches.push((
                        format!("{path}#text"),
                        before_text.clone(),
                        after_text.clone(),
                    )),
                }
            }
        }
    }

    ValueDiff {
        mismatches,
        number_format_only,
    }
}

/// Prints a count-ranked table, highest first, ties broken by key. Truncates at `limit` with a
/// footer naming how many entries were elided.
pub fn print_ranked(label: &str, totals: &BTreeMap<String, i64>, limit: usize) {
    if totals.is_empty() {
        return;
    }
    let mut ranked: Vec<_> = totals.iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    println!("\ntop {label}:");
    for (key, n) in ranked.iter().take(limit) {
        println!("  {n:>5}  {key}");
    }
    if ranked.len() > limit {
        println!("  … and {} more distinct entries", ranked.len() - limit);
    }
}

pub fn first_line(message: &str) -> &str {
    message.lines().next().unwrap_or(message)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// What `trim_text(true)` actually leaves, pinned rather than assumed: indentation between
    /// elements produces no key at all, so `#text` keys only ever mean real content.
    #[test]
    fn whitespace_between_elements_is_not_a_key() {
        let counts = profile("<a>\n  <b/>\n</a>").expect("well-formed");
        assert_eq!(
            counts.keys().collect::<Vec<_>>(),
            vec!["a", "a/b"],
            "indentation must not register as character content"
        );
    }

    /// Mixed content is counted per node, under the element the node sits in.
    #[test]
    fn mixed_content_counts_one_key_per_text_node() {
        let counts = profile("<a>one<b/>two</a>").expect("well-formed");
        assert_eq!(counts.get("a#text").copied(), Some(2));
        assert_eq!(counts.get("a/b#text").copied(), None);
    }

    /// `trim_text` does not apply to CDATA, which is why `note_text` re-trims.
    #[test]
    fn cdata_is_character_content_but_whitespace_cdata_is_not() {
        let counts = profile("<a><![CDATA[x]]><b><![CDATA[   ]]></b></a>").expect("well-formed");
        assert_eq!(counts.get("a#text").copied(), Some(1));
        assert_eq!(counts.get("a/b#text").copied(), None);
    }

    /// Text present on the left, absent on the right, and the comparator reports it.
    /// Before this ability was added, such a transformation was certified as lossless.
    #[test]
    fn dropped_text_shows_up_in_the_diff() {
        let before = profile("<a><b/>stray</a>").expect("well-formed");
        let after = profile("<a><b/></a>").expect("well-formed");
        let d = diff(&before, &after);
        assert_eq!(d.dropped, vec![("a#text".to_string(), 1)]);
        assert!(d.invented.is_empty());
    }

    /// The defect this comparator exists to catch: `profile()` cannot see this at all, because
    /// the attribute name is unchanged. `compare_values` must.
    #[test]
    fn a_rewritten_value_is_caught() {
        let before = values(r#"<a v="${pi}"/>"#).expect("well-formed");
        let after = values(r#"<a v="$pi"/>"#).expect("well-formed");
        let d = compare_values(&before, &after);
        assert_eq!(
            d.mismatches,
            vec![("a@v".to_string(), "${pi}".to_string(), "$pi".to_string())]
        );
        assert_eq!(d.number_format_only, 0);
    }

    /// A float re-serialized in a different lexical form is not a failure: the XSD types here are
    /// numeric, so the value space is what matters.
    #[test]
    fn number_format_only_rewrite_is_not_a_mismatch() {
        let before = values(r#"<a v="1.0"/>"#).expect("well-formed");
        let after = values(r#"<a v="1"/>"#).expect("well-formed");
        let d = compare_values(&before, &after);
        assert!(d.mismatches.is_empty());
        assert_eq!(d.number_format_only, 1);
    }

    /// Two `<Event>` siblings that swap position change what sits at position 0 and position 1 of
    /// the shared `a/Event` list, so the comparator must see it: this is exactly the priority- and
    /// order-changing defect `lossy`'s name-only comparison cannot.
    #[test]
    fn swapped_same_name_siblings_are_caught() {
        let before =
            values(r#"<a><Event p="first"/><Event p="second"/></a>"#).expect("well-formed");
        let after = values(r#"<a><Event p="second"/><Event p="first"/></a>"#).expect("well-formed");
        let d = compare_values(&before, &after);
        assert_eq!(
            d.mismatches,
            vec![
                (
                    "a/Event@p".to_string(),
                    "first".to_string(),
                    "second".to_string()
                ),
                (
                    "a/Event@p".to_string(),
                    "second".to_string(),
                    "first".to_string()
                ),
            ]
        );
    }

    /// `xsd:all` lets children appear in any order. Two differently-named children swapping
    /// position never touches either one's per-path list — each keeps its own path — so this must
    /// not be reported. Cross-name reordering inside an `xsd:sequence` is schema-invalid output,
    /// which is `validate`'s job, not this comparator's.
    #[test]
    fn swapped_differently_named_xsd_all_children_are_not_caught() {
        let before = values(r#"<a><B v="1"/><C v="2"/></a>"#).expect("well-formed");
        let after = values(r#"<a><C v="2"/><B v="1"/></a>"#).expect("well-formed");
        let d = compare_values(&before, &after);
        assert!(d.mismatches.is_empty());
        assert_eq!(d.number_format_only, 0);
    }
}
