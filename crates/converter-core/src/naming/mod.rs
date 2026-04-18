//! Naming — assign readable identifiers to induced enums and functions.

pub mod heuristics;

use crate::induction::InductionResult;
use crate::ConvertOptions;

/// InductionResult with all candidates named.
pub struct Named {
    pub result: InductionResult,
}

/// Assign readable names to all induced enums and functions.
///
/// For each candidate whose name is still empty (not pre-seeded by the
/// induction phase), the heuristics module derives a name from available
/// context: the source array key, the candidate's own keys, etc.
pub fn resolve(mut result: InductionResult, _opts: &ConvertOptions) -> Named {
    for candidate in &mut result.enums {
        if candidate.name.is_empty() {
            candidate.name = heuristics::name_enum(&candidate.keys);
        }
    }
    for func in &mut result.funcs {
        if func.name.is_empty() {
            func.name = heuristics::name_func(&func.param_keys);
        }
    }
    Named { result }
}
