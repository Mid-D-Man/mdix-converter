//! Emitter — converts a Named InductionResult into .mdix source text.

mod formatter;

use crate::naming::Named;
use crate::induction::{EnumCandidate, FuncCandidate};
use crate::ingestion::{Node, Primitive};
use crate::ConvertError;

/// Emit the full .mdix source from the named induction result.
pub fn emit(named: &Named) -> Result<String, ConvertError> {
    let mut out = String::new();

    out.push_str("@CONFIG(\n  version  -> \"1.0.0\"\n  features -> \"data\"\n)\n\n");

    if !named.result.enums.is_empty() {
        out.push_str(&emit_enums(&named.result.enums));
    }

    if !named.result.funcs.is_empty() {
        out.push_str(&emit_quickfuncs(&named.result.funcs));
    }

    out.push_str(&emit_data(&named.result.root, &named.result.enums, &named.result.funcs));

    Ok(formatter::format(&out))
}

// ── Section emitters ──────────────────────────────────────────────────────────

fn emit_enums(enums: &[EnumCandidate]) -> String {
    let mut s = String::from("@ENUMS(\n");
    for e in enums {
        let fields = e.values.iter()
            .enumerate()
            .map(|(i, v)| format!("  {} = {}", to_screaming_snake(v), i))
            .collect::<Vec<_>>()
            .join(", ");
        s.push_str(&format!("  {} {{ {} }}\n", e.name, fields));
    }
    s.push_str(")\n\n");
    s
}

fn emit_quickfuncs(funcs: &[FuncCandidate]) -> String {
    let mut s = String::from("@QUICKFUNCS(\n");
    for f in funcs {
        let params = f.param_keys.join(", ");
        s.push_str(&format!("  ~{}<object>({}) {{\n    return {{\n", f.name, params));
        for key in &f.param_keys {
            s.push_str(&format!("      {} = {}\n", key, key));
        }
        for (key, val) in &f.defaults {
            s.push_str(&format!("      {} = {}\n", key, emit_inline_value(val)));
        }
        s.push_str("    }\n  }\n");
    }
    s.push_str(")\n\n");
    s
}

fn emit_data(
    root:  &Node,
    enums: &[EnumCandidate],
    funcs: &[FuncCandidate],
) -> String {
    let mut s = String::from("@DATA(\n");
    s.push_str(&emit_node_body(root, 1, enums, funcs));
    s.push_str(")\n");
    s
}

// ── Node serialisation ────────────────────────────────────────────────────────

fn emit_node_body(
    node:  &Node,
    depth: usize,
    enums: &[EnumCandidate],
    funcs: &[FuncCandidate],
) -> String {
    let pad = "  ".repeat(depth);
    match node {
        Node::Record(r) => {
            let mut out = String::new();
            for (key, value) in &r.fields {
                match value {
                    Node::Array(_) | Node::Record(_) => {
                        out.push_str(&format!("{}{}::\n", pad, key));
                        out.push_str(&emit_node_body(value, depth + 1, enums, funcs));
                    }
                    _ => {
                        let v = emit_value_with_enums(value, key, enums);
                        out.push_str(&format!("{}{} = {}\n", pad, key, v));
                    }
                }
            }
            out
        }
        Node::Array(items) => {
            items.iter()
                .map(|item| format!("{}{}\n", pad, emit_inline_value(item)))
                .collect()
        }
        other => emit_inline_value(other) + "\n",
    }
}

fn emit_inline_value(node: &Node) -> String {
    match node {
        Node::Primitive(p) => match p {
            Primitive::Str(s)  => format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\"")),
            Primitive::Int(i)  => i.to_string(),
            Primitive::Float(f)=> format!("{}", f),
            Primitive::Bool(b) => b.to_string(),
        },
        Node::Null           => "null".to_string(),
        Node::Record(r) => {
            let pairs: Vec<String> = r.fields.iter()
                .map(|(k, v)| format!("{} = {}", k, emit_inline_value(v)))
                .collect();
            format!("{{ {} }}", pairs.join(", "))
        }
        Node::Array(items) => {
            let vals: Vec<String> = items.iter().map(emit_inline_value).collect();
            format!("[ {} ]", vals.join(", "))
        }
    }
}

fn emit_value_with_enums(node: &Node, key: &str, enums: &[EnumCandidate]) -> String {
    if let Node::Primitive(Primitive::Str(s)) = node {
        // Check if this string value belongs to a known enum
        for e in enums {
            if e.keys.iter().any(|k| k == key) && e.values.contains(s) {
                return format!("{}.{}", e.name, to_screaming_snake(s));
            }
        }
    }
    emit_inline_value(node)
}

fn to_screaming_snake(s: &str) -> String {
    s.replace([' ', '-'], "_").to_uppercase()
}
