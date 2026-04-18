//! Intermediate Representation — format-agnostic value tree.
//!
//! Every supported format (JSON, TOML) is first parsed into this tree.
//! Subsequent pipeline stages work exclusively on this representation,
//! making the induction and emission stages format-independent.

/// A value in the IR.
#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Record(Record),
    Array(Vec<Node>),
    Primitive(Primitive),
    Null,
}

impl Node {
    pub fn is_record(&self) -> bool  { matches!(self, Node::Record(_)) }
    pub fn is_array(&self)  -> bool  { matches!(self, Node::Array(_)) }
    pub fn is_null(&self)   -> bool  { matches!(self, Node::Null) }

    pub fn as_record(&self) -> Option<&Record> {
        if let Node::Record(r) = self { Some(r) } else { None }
    }

    pub fn as_array(&self) -> Option<&[Node]> {
        if let Node::Array(a) = self { Some(a) } else { None }
    }
}

/// An ordered map of string keys to child nodes.
/// Order is preserved to keep output deterministic.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Record {
    pub fields: Vec<(String, Node)>,
}

impl Record {
    pub fn new() -> Self { Self::default() }

    pub fn insert(&mut self, key: impl Into<String>, value: Node) {
        self.fields.push((key.into(), value));
    }

    pub fn get(&self, key: &str) -> Option<&Node> {
        self.fields.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    /// Return all unique keys across this record.
    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.fields.iter().map(|(k, _)| k.as_str())
    }

    /// Structural skeleton: sorted key names only, ignoring values.
    /// Two records with the same skeleton are candidates for a QuickFunc.
    pub fn skeleton(&self) -> Vec<&str> {
        let mut keys: Vec<&str> = self.fields.iter().map(|(k, _)| k.as_str()).collect();
        keys.sort_unstable();
        keys
    }
}

/// Scalar primitive values.
#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

impl Primitive {
    pub fn as_str(&self)  -> Option<&str>  { if let Self::Str(s)  = self { Some(s) } else { None } }
    pub fn as_int(&self)  -> Option<i64>   { if let Self::Int(i)  = self { Some(*i) } else { None } }
    pub fn as_bool(&self) -> Option<bool>  { if let Self::Bool(b) = self { Some(*b) } else { None } }
}
