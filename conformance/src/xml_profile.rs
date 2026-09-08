//! Structural XML comparison and shared reporting helpers.
//!
//! Both sides of a comparison are reduced to a multiset of `path/to/Element` and
//! `path/to/Element@attr` keys, so attribute order and whitespace don't matter. Anything present
//! on the left but not the right is *dropped*; anything on the right but not the left is
//! *invented*.
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

/// Reduces XML to a multiset of `path/to/Element` and `path/to/Element@attr` keys.
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
            _ => {}
        }
        buf.clear();
    }
    Ok(counts)
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
