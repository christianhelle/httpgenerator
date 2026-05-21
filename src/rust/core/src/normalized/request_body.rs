use serde::{Deserialize, Serialize};

use super::NormalizedSchema;

/// Normalized request body definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NormalizedRequestBody {
    /// Request body details are available inline.
    Inline(NormalizedInlineRequestBody),
    /// Request body was represented by an unresolved OpenAPI `$ref`.
    Reference { reference: String },
}

/// Inline request body details.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedInlineRequestBody {
    /// Optional request body description.
    pub description: Option<String>,
    /// Indicates whether callers must provide a body.
    pub required: bool,
    /// Media types accepted by the operation.
    pub content: Vec<NormalizedMediaType>,
}

/// Media type entry for a request body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedMediaType {
    /// MIME content type, such as `application/json`.
    pub content_type: String,
    /// Optional body schema for this media type.
    pub schema: Option<NormalizedSchema>,
}
