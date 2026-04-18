//! Enum induction — frequency analysis on string field values.
//!
//! Algorithm:
//!   1. Walk the IR collecting every (key → string value) occurrence.
//!   2. For keys where the set of unique values is small (≤ MAX_ENUM_VALUES)
//!      and the total occurrence count meets min_occurrences, compute savings.
//!   3. If estimated character saving exceeds the threshold, emit a candidate.

use std::collections::HashMap;
use crate::ConvertOptions;
use crate::ingestion::{Node, Primitive};
use super::math;

/// Maximum number of unique values a string field may have to be considered
/// an enum candidate. Beyond this it is likely a free-form string, not an enum.
const MAX_ENUM_VALUES: usize = 16;

#[derive(Debug, Clone)]
pub struct EnumCandidate {
    /// Resolved name (set by naming phase).
    pub name:    String,
    /// Sorted list of unique values that become enum fields.
    pub values:  Vec<String>,
    /// Keys in the IR that map to this enum type.
    pub keys:    Vec<String>,
    /// Estimated character saving from this induction.
    pub saving:  isize,
}

pub fn detect(root: &Node, opts: &ConvertOptions) -> Vec<EnumCandidate> {
    // key -> (value -> count)
    let mut freq: HashMap<String, HashMap<String, usize>> = HashMap::new();
    collect_string_freqs(root, &mut freq);

    let mut candidates: Vec<EnumCandidate> = freq
        .into_iter()
        .filter_map(|(key, values)| {
            let total: usize = values.values().copied().sum();
            if values.len() > MAX_ENUM_VALUES || total < opts.min_occurrences {
                return None;
            }
            let saving = math::enum_saving(&values, total);
            if saving < opts.threshold as isize {
                return None;
            }
            let mut vals: Vec<String> = values.into_keys().collect();
            vals.sort();
            Some(EnumCandidate {
                name:   String::new(), // filled by naming phase
                values: vals,
                keys:   vec![key],
                saving,
            })
        })
        .collect();

    // Sort by saving descending so the most impactful enums appear first
    candidates.sort_by(|a, b| b.saving.cmp(&a.saving));
    candidates
}

fn collect_string_freqs(node: &Node, freq: &mut HashMap<String, HashMap<String, usize>>) {
    match node {
        Node::Record(r) => {
            for (key, value) in &r.fields {
                match value {
                    Node::Primitive(Primitive::Str(s)) => {
                        *freq
                            .entry(key.clone())
                            .or_default()
                            .entry(s.clone())
                            .or_insert(0) += 1;
                    }
                    _ => collect_string_freqs(value, freq),
                }
            }
        }
        Node::Array(items) => {
            for item in items {
                collect_string_freqs(item, freq);
            }
        }
        _ => {}
    }
}
