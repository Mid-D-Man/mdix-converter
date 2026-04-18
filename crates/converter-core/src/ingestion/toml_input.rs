// Auto-generated stub
// crates/converter-core/src/ingestion/toml_input.rs
//
// Parses a TOML string into the format-agnostic IR (Node tree).
// Uses the `toml` crate for parsing, then maps toml::Value → Node.
//
// TOML type mapping:
//   string        → Node::Primitive(Primitive::Str)
//   integer       → Node::Primitive(Primitive::Int)
//   float         → Node::Primitive(Primitive::Float)
//   boolean       → Node::Primitive(Primitive::Bool)
//   datetime      → Node::Primitive(Primitive::Str)   (ISO-8601 string)
//   array         → Node::Array
//   table         → Node::Record  (insertion order preserved)

use toml::Value;
use crate::ConvertError;
use super::ir::{Node, Record, Primitive};

/// Parse a TOML string into the IR.
pub fn parse(input: &str) -> Result<Node, ConvertError> {
    let value: Value = toml::from_str(input)
        .map_err(|e| ConvertError::Parse(format!("TOML parse error: {}", e)))?;
    Ok(value_to_node(value))
}

// ── Recursive mapping ─────────────────────────────────────────────────────────

fn value_to_node(v: Value) -> Node {
    match v {
        Value::String(s)    => Node::Primitive(Primitive::Str(s)),
        Value::Integer(i)   => Node::Primitive(Primitive::Int(i)),
        Value::Float(f)     => Node::Primitive(Primitive::Float(f)),
        Value::Boolean(b)   => Node::Primitive(Primitive::Bool(b)),

        // TOML datetimes have no direct analogue in the IR — preserve as string
        Value::Datetime(dt) => Node::Primitive(Primitive::Str(dt.to_string())),

        Value::Array(arr)   => {
            // TOML arrays may be heterogeneous in the spec but are usually
            // homogeneous in practice.  Map each element independently.
            Node::Array(arr.into_iter().map(value_to_node).collect())
        }

        Value::Table(map) => {
            let mut record = Record::new();
            // toml's IndexMap preserves insertion order.
            for (key, val) in map {
                record.insert(key, value_to_node(val));
            }
            Node::Record(record)
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_flat_table() {
        let toml = r#"
name    = "Alice"
age     = 30
active  = true
"#;
        let node = parse(toml).unwrap();
        let rec  = node.as_record().expect("expected record");
        assert_eq!(rec.fields.len(), 3);
    }

    #[test]
    fn parses_nested_table() {
        let toml = r#"
[server]
host = "localhost"
port = 8080
"#;
        let node   = parse(toml).unwrap();
        let rec    = node.as_record().unwrap();
        let server = rec.get("server").unwrap();
        assert!(server.is_record());
        let srv_rec = server.as_record().unwrap();
        assert!(srv_rec.get("host").is_some());
        assert!(srv_rec.get("port").is_some());
    }

    #[test]
    fn parses_array_of_tables() {
        let toml = r#"
[[weapons]]
name = "Sword"
damage = 25

[[weapons]]
name = "Bow"
damage = 20
"#;
        let node     = parse(toml).unwrap();
        let rec      = node.as_record().unwrap();
        let weapons  = rec.get("weapons").unwrap();
        let arr      = weapons.as_array().expect("expected array");
        assert_eq!(arr.len(), 2);
    }

    #[test]
    fn integer_maps_to_int() {
        let node = parse("x = 42").unwrap();
        let rec  = node.as_record().unwrap();
        match rec.get("x").unwrap() {
            Node::Primitive(Primitive::Int(42)) => {}
            other => panic!("expected Int(42), got {:?}", other),
        }
    }

    #[test]
    fn float_maps_to_float() {
        let node = parse("pi = 3.14").unwrap();
        let rec  = node.as_record().unwrap();
        assert!(matches!(rec.get("pi").unwrap(), Node::Primitive(Primitive::Float(_))));
    }

    #[test]
    fn datetime_maps_to_str() {
        let node = parse("ts = 2024-01-01T00:00:00Z").unwrap();
        let rec  = node.as_record().unwrap();
        assert!(matches!(rec.get("ts").unwrap(), Node::Primitive(Primitive::Str(_))));
    }

    #[test]
    fn invalid_toml_returns_parse_error() {
        let result = parse("[[[[invalid");
        assert!(matches!(result, Err(ConvertError::Parse(_))));
    }

    #[test]
    fn inline_array_of_strings() {
        let node = parse(r#"tags = ["web", "api", "v1"]"#).unwrap();
        let rec  = node.as_record().unwrap();
        let arr  = rec.get("tags").unwrap().as_array().expect("expected array");
        assert_eq!(arr.len(), 3);
    }
      }
