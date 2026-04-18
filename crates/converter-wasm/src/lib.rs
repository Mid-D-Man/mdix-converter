//! WASM bindings — thin wasm-bindgen wrapper around converter-core.
//!
//! These functions are exposed to JavaScript via the generated JS glue code.
//! The playground calls them directly from Svelte after loading the WASM module.

use wasm_bindgen::prelude::*;
use converter_core::{convert, ConvertOptions, InputFormat};

#[wasm_bindgen(start)]
pub fn init() {
    // Surface Rust panics as readable JS errors in the browser console
    console_error_panic_hook::set_once();
}

/// Convert a JSON string to optimised .mdix.
///
/// @param input     - Raw JSON text
/// @param threshold - Minimum char saving to apply an optimisation (default 50)
/// @returns          Optimised .mdix source, or throws a JS error on failure
#[wasm_bindgen]
pub fn convert_json(input: &str, threshold: usize) -> Result<String, JsValue> {
    let opts = ConvertOptions { threshold, min_occurrences: 3 };
    convert(input, InputFormat::Json, &opts)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Convert a TOML string to optimised .mdix.
///
/// @param input     - Raw TOML text
/// @param threshold - Minimum char saving to apply an optimisation (default 50)
/// @returns          Optimised .mdix source, or throws a JS error on failure
#[wasm_bindgen]
pub fn convert_toml(input: &str, threshold: usize) -> Result<String, JsValue> {
    let opts = ConvertOptions { threshold, min_occurrences: 3 };
    convert(input, InputFormat::Toml, &opts)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Return the converter-core version string.
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
