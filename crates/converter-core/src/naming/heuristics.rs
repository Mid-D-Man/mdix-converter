//! Heuristic naming — derives readable identifiers from structural context.

/// Derive a PascalCase enum name from the field's key name(s).
///
/// Examples:
///   ["weapon_type"] → "WeaponType"
///   ["rarity"]      → "Rarity"
pub fn name_enum(keys: &[String]) -> String {
    keys.first()
        .map(|k| to_pascal(k))
        .unwrap_or_else(|| "MyEnum".to_string())
}

/// Derive a snake_case function name from its source array key by singularising.
///
/// Examples:
///   "weapons"     → "weapon"
///   "enemies"     → "enemy"
///   "categories"  → "category"
///   "entries"     → "entry"
pub fn name_func(keys: &[String]) -> String {
    keys.first()
        .map(|k| singularize(k))
        .unwrap_or_else(|| "make_entry".to_string())
}

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Convert snake_case or kebab-case to PascalCase.
fn to_pascal(s: &str) -> String {
    s.split(['_', '-', ' '])
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None    => String::new(),
                Some(f) => f.to_uppercase().to_string() + chars.as_str(),
            }
        })
        .collect()
}

/// Strip common English plural suffixes.
fn singularize(s: &str) -> String {
    if s.ends_with("ies") { return format!("{}y",   &s[..s.len() - 3]); }
    if s.ends_with("ves") { return format!("{}f",   &s[..s.len() - 3]); }
    if s.ends_with("ses") { return format!("{}",    &s[..s.len() - 2]); }
    if s.ends_with("xes") { return format!("{}",    &s[..s.len() - 2]); }
    if s.ends_with("zes") { return format!("{}",    &s[..s.len() - 2]); }
    if s.ends_with('s') && s.len() > 2 { return s[..s.len() - 1].to_string(); }
    s.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pascal_converts_snake_case()  { assert_eq!(to_pascal("weapon_type"), "WeaponType"); }
    #[test]
    fn pascal_converts_kebab_case()  { assert_eq!(to_pascal("damage-type"), "DamageType"); }
    #[test]
    fn singular_strips_ies()         { assert_eq!(singularize("enemies"),   "enemy"); }
    #[test]
    fn singular_strips_plain_s()     { assert_eq!(singularize("weapons"),   "weapon"); }
    #[test]
    fn singular_leaves_short_words() { assert_eq!(singularize("is"),        "is"); }
}
