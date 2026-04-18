//! Ingestion — parse raw text into the intermediate representation.

mod json;
mod toml_input;
pub mod ir;

pub use ir::{Node, Record, Primitive};
use crate::{ConvertError, InputFormat};

pub fn parse(input: &str, format: InputFormat) -> Result<Node, ConvertError> {
    match format {
        InputFormat::Json => json::parse(input),
        InputFormat::Toml => toml_input::parse(input),
    }
}
