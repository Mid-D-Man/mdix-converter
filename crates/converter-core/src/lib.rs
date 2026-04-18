//! converter-core — format-agnostic induction and emission pipeline.
//!
//! Pipeline stages:
//!   1. ingestion  — parse raw text into a generic IR (Node tree)
//!   2. induction  — detect enum candidates + structurally-identical objects
//!   3. naming     — assign readable identifiers via heuristics
//!   4. emitter    — write idiomatic .mdix source

pub mod ingestion;
pub mod induction;
pub mod naming;
pub mod emitter;

use std::fmt;

// ── Public entry point ───────────────────────────────────────────────────────

/// Convert raw input text to optimised .mdix.
pub fn convert(input: &str, format: InputFormat, opts: &ConvertOptions) -> Result<String, ConvertError> {
    let ir     = ingestion::parse(input, format)?;
    let induced = induction::analyse(&ir, opts)?;
    let named  = naming::resolve(induced, opts);
    emitter::emit(&named)
}

// ── Types ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputFormat {
    Json,
    Toml,
}

impl InputFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "json"        => Some(Self::Json),
            "toml"        => Some(Self::Toml),
            _             => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConvertOptions {
    /// Minimum character saving required to apply an optimisation.
    pub threshold: usize,
    /// Minimum times a pattern must appear before induction fires.
    pub min_occurrences: usize,
}

impl Default for ConvertOptions {
    fn default() -> Self {
        Self { threshold: 50, min_occurrences: 3 }
    }
}

#[derive(Debug)]
pub enum ConvertError {
    Parse(String),
    Emit(String),
}

impl fmt::Display for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConvertError::Parse(s) => write!(f, "Parse error: {}", s),
            ConvertError::Emit(s)  => write!(f, "Emit error: {}",  s),
        }
    }
}
