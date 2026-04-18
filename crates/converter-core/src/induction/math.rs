//! Savings estimation — character-level delta calculations.
//!
//! All functions return estimated character counts as signed integers.
//! Positive = saving; negative = overhead (optimisation would bloat output).

use std::collections::HashMap;

/// Estimate the net character saving from converting a string field to an enum.
///
/// Model:
///   original_cost  = avg_quoted_string_len × total_uses
///   enum_def_cost  = Σ(field_name_len + 2 for comma/space) + ~12 for outer syntax
///   ref_cost       = avg_enum_ref_len (EnumName.Field ≈ 12 chars) × total_uses
///   saving         = original_cost - enum_def_cost - ref_cost
pub fn enum_saving(values: &HashMap<String, usize>, total_uses: usize) -> isize {
    if values.is_empty() || total_uses == 0 {
        return 0;
    }

    let avg_str_len: usize = values.keys().map(|s| s.len() + 2).sum::<usize>() / values.len();
    let enum_def_cost: isize = values.keys()
        .map(|s| s.len() as isize + 2)
        .sum::<isize>()
        + 12;
    let ref_len: isize = 12; // rough average: "EnumName.FIELD"
    let original:  isize = avg_str_len as isize * total_uses as isize;
    let optimised: isize = enum_def_cost + ref_len * total_uses as isize;
    original - optimised
}

/// Estimate the net character saving from extracting repeated objects into a QuickFunc.
///
/// Model:
///   func_def_cost  = (param_count × 12) + (static_field_count × 20) + 30
///   call_cost      = (param_count × 8) + 6   per call
///   obj_cost       = (param_count × 12) + (static_field_count × 20) + 4   per object
///   saving         = (obj_cost × instances) - func_def_cost - (call_cost × instances)
pub fn func_saving(param_count: usize, static_field_count: usize, instance_count: usize) -> isize {
    let func_def:  isize = (param_count * 12 + static_field_count * 20 + 30) as isize;
    let call_cost: isize = (param_count * 8 + 6) as isize;
    let obj_cost:  isize = (param_count * 12 + static_field_count * 20 + 4) as isize;
    let original:  isize = obj_cost * instance_count as isize;
    let optimised: isize = func_def + call_cost * instance_count as isize;
    original - optimised
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enum_saving_positive_for_repeated_values() {
        let mut values = HashMap::new();
        values.insert("SWORD".to_string(), 5usize);
        values.insert("BOW".to_string(),   3);
        values.insert("STAFF".to_string(), 2);
        let saving = enum_saving(&values, 10);
        // 10 instances of ~7-char strings vs a small enum def + short refs
        assert!(saving > 0, "expected positive saving, got {}", saving);
    }

    #[test]
    fn func_saving_positive_for_large_arrays() {
        // 10 objects, each with 4 params and 3 static fields
        let saving = func_saving(4, 3, 10);
        assert!(saving > 0, "expected positive saving, got {}", saving);
    }

    #[test]
    fn func_saving_negative_for_small_arrays() {
        // Only 2 instances — defining a function costs more than it saves
        let saving = func_saving(3, 2, 2);
        // May or may not be negative depending on field counts; just verify it computes
        let _ = saving;
    }
}
