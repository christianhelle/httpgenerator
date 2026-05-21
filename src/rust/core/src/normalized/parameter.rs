use serde::{Deserialize, Serialize};

use super::{NormalizedParameterLocation, NormalizedSchema};

/// Normalized operation parameter.
///
/// Most generator-facing parameters are expected to be available inline, but references are kept so
/// normalization can preserve source intent when needed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NormalizedParameter {
    /// Fully expanded parameter definition.
    Inline(NormalizedInlineParameter),
    /// Unresolved parameter reference.
    Reference {
        /// Reference string, typically pointing at a reusable parameter component.
        reference: String,
    },
}

/// Inline parameter details used by the generator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedInlineParameter {
    /// Original parameter name from the source document.
    pub name: String,
    /// Where the parameter is supplied in the request.
    pub location: NormalizedParameterLocation,
    /// Human-readable description for generated comments and tooling.
    pub description: Option<String>,
    /// Whether the parameter is required by the source contract.
    pub required: bool,
    /// Optional schema describing the parameter value.
    pub schema: Option<NormalizedSchema>,
}
