use converter_core::{convert, ConvertOptions, InputFormat};

// ── Basic smoke tests ─────────────────────────────────────────────────────────

#[test]
fn converts_simple_flat_json() {
    let input = r#"{"name": "Alice", "age": 30, "active": true}"#;
    let result = convert(input, InputFormat::Json, &ConvertOptions::default());
    assert!(result.is_ok(), "{:?}", result);
    let mdix = result.unwrap();
    assert!(mdix.contains("@DATA"), "missing @DATA\n{}", mdix);
    assert!(mdix.contains("name"),  "missing 'name'\n{}",  mdix);
    assert!(mdix.contains("Alice"), "missing 'Alice'\n{}", mdix);
}

#[test]
fn converts_nested_json_object() {
    let input = r#"{"server": {"host": "localhost", "port": 8080}}"#;
    let result = convert(input, InputFormat::Json, &ConvertOptions::default());
    assert!(result.is_ok(), "{:?}", result);
    let mdix = result.unwrap();
    assert!(mdix.contains("server"),    "missing 'server'\n{}",    mdix);
    assert!(mdix.contains("localhost"), "missing 'localhost'\n{}", mdix);
}

#[test]
fn output_always_contains_config_and_data() {
    let input = r#"{"x": 1}"#;
    let mdix = convert(input, InputFormat::Json, &ConvertOptions::default()).unwrap();
    assert!(mdix.contains("@CONFIG"), "missing @CONFIG");
    assert!(mdix.contains("@DATA"),   "missing @DATA");
}

// ── Enum induction ────────────────────────────────────────────────────────────

#[test]
fn detects_enum_from_repeated_string_field() {
    let input = serde_json::json!([
        {"id": 1, "type": "SWORD",  "rarity": "COMMON"},
        {"id": 2, "type": "BOW",    "rarity": "RARE"},
        {"id": 3, "type": "SWORD",  "rarity": "COMMON"},
        {"id": 4, "type": "STAFF",  "rarity": "EPIC"},
        {"id": 5, "type": "BOW",    "rarity": "RARE"},
    ]).to_string();
    let opts = ConvertOptions { threshold: 0, min_occurrences: 2 };
    let result = convert(&input, InputFormat::Json, &opts);
    assert!(result.is_ok(), "{:?}", result);
    let mdix = result.unwrap();
    assert!(mdix.contains("@ENUMS"), "expected @ENUMS block:\n{}", mdix);
}

#[test]
fn no_enum_when_all_values_unique() {
    let input = serde_json::json!([
        {"id": 1, "name": "Alice"},
        {"id": 2, "name": "Bob"},
        {"id": 3, "name": "Charlie"},
    ]).to_string();
    let opts = ConvertOptions { threshold: 0, min_occurrences: 2 };
    let result = convert(&input, InputFormat::Json, &opts).unwrap();
    // Names are all unique — should NOT produce @ENUMS
    assert!(!result.contains("@ENUMS"), "unexpected @ENUMS:\n{}", result);
}

// ── Determinism ───────────────────────────────────────────────────────────────

#[test]
fn same_input_same_output() {
    let input = r#"{"a": 1, "b": "hello", "c": true}"#;
    let opts  = ConvertOptions::default();
    let out1  = convert(input, InputFormat::Json, &opts).unwrap();
    let out2  = convert(input, InputFormat::Json, &opts).unwrap();
    assert_eq!(out1, out2, "output is not deterministic");
}
