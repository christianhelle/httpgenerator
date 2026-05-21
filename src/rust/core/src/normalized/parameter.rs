use serde::{Deserialize, Serialize};

use super::{NormalizedParameterLocation, NormalizedSchema};

/// Normalized operation parameter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NormalizedParameter {
    /// Parameter details are available inline.
    Inline(NormalizedInlineParameter),
    /// Parameter was represented by an unresolved OpenAPI `$ref`.
    Reference { reference: String },
}

/// Inline parameter details used by request rendering.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedInlineParameter {
    /// Parameter name.
    pub name: String,
    /// Location where the parameter is applied.
    pub location: NormalizedParameterLocation,
    /// Optional parameter description.
    pub description: Option<String>,
    /// Indicates whether callers must provide the parameter.
    pub required: bool,
    /// Optional parameter schema.
    pub schema: Option<NormalizedSchema>,
}
