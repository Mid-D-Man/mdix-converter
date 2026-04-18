//! Function induction — structural skeleton hashing.
//!
//! Algorithm:
//!   1. Walk arrays in the IR.
//!   2. Hash the structural skeleton (sorted key names) of each object.
//!   3. Group objects with the same skeleton hash.
//!   4. Identify which values are dynamic (arguments) vs static (defaults).
//!   5. If estimated character saving exceeds the threshold, emit a candidate.

use std::collections::HashMap;
use crate::ConvertOptions;
use crate::ingestion::{Node, Record};
use super::math;

#[derive(Debug, Clone)]
pub struct FuncCandidate {
    /// Resolved name (set by naming phase; usually the array's key singularised).
    pub name:         String,
    /// Keys whose values vary across instances — become function parameters.
    pub param_keys:   Vec<String>,
    /// Keys whose values are constant across instances — become defaults inside the body.
    pub defaults:     Vec<(String, Node)>,
    /// All matched object instances (used for @DATA call generation).
    pub instances:    Vec<Node>,
    /// Estimated character saving from this induction.
    pub saving:       isize,
}

pub fn detect(root: &Node, opts: &ConvertOptions) -> Vec<FuncCandidate> {
    let mut candidates = Vec::new();
    find_homogeneous_arrays(root, "", &mut candidates, opts);
    candidates.sort_by(|a, b| b.saving.cmp(&a.saving));
    candidates
}

fn find_homogeneous_arrays(
    node:      &Node,
    array_key: &str,
    out:       &mut Vec<FuncCandidate>,
    opts:      &ConvertOptions,
) {
    match node {
        Node::Array(items) if items.len() >= opts.min_occurrences => {
            if let Some(c) = try_extract_func(array_key, items, opts) {
                out.push(c);
            }
            // Recurse into array items too
            for item in items {
                find_homogeneous_arrays(item, "", out, opts);
            }
        }
        Node::Record(r) => {
            for (key, child) in &r.fields {
                find_homogeneous_arrays(child, key, out, opts);
            }
        }
        _ => {}
    }
}

/// Try to extract a QuickFunc from a homogeneous array of objects.
/// Returns None if the objects are structurally too diverse or savings are insufficient.
fn try_extract_func(
    source_key: &str,
    items:      &[Node],
    opts:       &ConvertOptions,
) -> Option<FuncCandidate> {
    // All items must be records for function induction to apply
    let records: Vec<&Record> = items.iter()
        .filter_map(|n| n.as_record())
        .collect();
    if records.len() < opts.min_occurrences {
        return None;
    }

    // All records must share the same skeleton
    let skeleton = records[0].skeleton();
    if !records.iter().all(|r| r.skeleton() == skeleton) {
        return None;
    }

    // Classify each key: dynamic (varies) or static (constant)
    let mut param_keys: Vec<String>          = Vec::new();
    let mut defaults:   Vec<(String, Node)>  = Vec::new();

    for key in &skeleton {
        let first_val = records[0].get(key)?;
        let is_constant = records.iter().all(|r| {
            r.get(key).map(|v| v == first_val).unwrap_or(false)
        });
        if is_constant {
            defaults.push((key.to_string(), first_val.clone()));
        } else {
            param_keys.push(key.to_string());
        }
    }

    // No point inducing a function with no parameters
    if param_keys.is_empty() {
        return None;
    }

    let saving = math::func_saving(param_keys.len(), defaults.len(), records.len());
    if saving < opts.threshold as isize {
        return None;
    }

    Some(FuncCandidate {
        name:      source_key.to_string(), // naming phase will singularise
        param_keys,
        defaults,
        instances: items.to_vec(),
        saving,
    })
}
