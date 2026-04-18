// Auto-generated stub
// crates/converter-core/src/ingestion/json.rs
//
// Parses a JSON string into the format-agnostic IR (Node tree).
// Uses serde_json for parsing, then maps serde_json::Value → Node.
//
// JSON type mapping:
//   null          → Node::Null
//   bool          → Node::Primitive(Primitive::Bool)
//   integer       → Node::Primitive(Primitive::Int)
//   float         → Node::Primitive(Primitive::Float)
//   string        → Node::Primitive(Primitive::Str)
//   array         → Node::Array
//   object        → Node::Record  (insertion order preserved)

use serde_json::Value;
use crate::ConvertError;
use super::ir::{Node, Record, Primitive};

/// Parse a JSON string into the IR.
pub fn parse(input: &str) -> Result<Node, ConvertError> {
    let value: Value = serde_json::from_str(input)
        .map_err(|e| ConvertError::Parse(format!("JSON parse error: {}", e)))?;
    Ok(value_to_node(value))
}

// ── Recursive mapping ─────────────────────────────────────────────────────────

fn value_to_node(v: Value) -> Node {
    match v {
        Value::Null        => Node::Null,
        Value::Bool(b)     => Node::Primitive(Primitive::Bool(b)),
        Value::String(s)   => Node::Primitive(Primitive::Str(s)),
        Value::Number(n)   => number_to_node(n),
        Value::Array(arr)  => Node::Array(arr.into_iter().map(value_to_node).collect()),
        Value::Object(map) => {
            let mut record = Record::new();
            // serde_json's Map preserves insertion order when the
            // "preserve_order" feature is active (it is by default).
            for (key, val) in map {
                record.insert(key, value_to_node(val));
            }
            Node::Record(record)
        }
    }
}

/// Map a JSON number to Int or Float.
///
/// Prefer Int when the value is representable as i64 with no loss,
/// otherwise fall back to Float (f64).
fn number_to_node(n: serde_json::Number) -> Node {
    if let Some(i) = n.as_i64() {
        return Node::Primitive(Primitive::Int(i));
    }
    if let Some(f) = n.as_f64() {
        return Node::Primitive(Primitive::Float(f));
    }
    // Fallback: stringify the number as a plain string so no data is lost
    Node::Primitive(Primitive::Str(n.to_string()))
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_flat_object() {
        let node = parse(r#"{"name":"Alice","age":30,"active":true}"#).unwrap();
        let rec  = node.as_record().expect("expected record");
        assert_eq!(rec.fields.len(), 3);
    }

    #[test]
    fn parses_nested_object() {
        let node = parse(r#"{"server":{"host":"localhost","port":8080}}"#).unwrap();
        let rec  = node.as_record().unwrap();
        let server = rec.get("server").unwrap();
        assert!(server.is_record());
    }

    #[test]
    fn parses_array_of_objects() {
        let node = parse(r#"[{"id":1},{"id":2},{"id":3}]"#).unwrap();
        let arr  = node.as_array().expect("expected array");
        assert_eq!(arr.len(), 3);
    }

    #[test]
    fn integer_stays_int() {
        let node = parse("42").unwrap();
        assert!(matches!(node, Node::Primitive(Primitive::Int(42))));
    }

    #[test]
    fn float_stays_float() {
        let node = parse("3.14").unwrap();
        assert!(matches!(node, Node::Primitive(Primitive::Float(_))));
    }

    #[test]
    fn null_maps_to_null() {
        let node = parse("null").unwrap();
        assert!(node.is_null());
    }

    #[test]
    fn invalid_json_returns_parse_error() {
        let result = parse("{broken");
        assert!(matches!(result, Err(ConvertError::Parse(_))));
    }

    #[test]
    fn insertion_order_preserved() {
        let node = parse(r#"{"z":1,"a":2,"m":3}"#).unwrap();
        let rec  = node.as_record().unwrap();
        let keys: Vec<&str> = rec.keys().collect();
        assert_eq!(keys, vec!["z", "a", "m"]);
    }
  }
