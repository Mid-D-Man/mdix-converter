//! Induction — detect optimisation opportunities in the IR.
//!
//! Phase 2a: Enum induction via string-value frequency analysis.
//! Phase 2b: Function induction via structural skeleton hashing.

pub mod enums;
pub mod functions;
pub mod math;

pub use enums::EnumCandidate;
pub use functions::FuncCandidate;
use crate::{ConvertError, ConvertOptions};
use super::ingestion::Node;

/// Output from the induction phase.
#[derive(Debug)]
pub struct InductionResult {
    /// Original IR root, untouched.
    pub root:  Node,
    /// Enum definitions ready to be emitted in @ENUMS.
    pub enums: Vec<EnumCandidate>,
    /// Function definitions ready to be emitted in @QUICKFUNCS.
    pub funcs: Vec<FuncCandidate>,
}

pub fn analyse(root: &Node, opts: &ConvertOptions) -> Result<InductionResult, ConvertError> {
    let enums = enums::detect(root, opts);
    let funcs = functions::detect(root, opts);
    Ok(InductionResult { root: root.clone(), enums, funcs })
}
