use serde::{Deserialize, Serialize};

use super::NormalizedSchema;

/// Normalized request body attached to an operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NormalizedRequestBody {
    /// Fully expanded request body definition.
    Inline(NormalizedInlineRequestBody),
    /// Unresolved request-body reference.
    Reference {
        /// Reference string, typically pointing at a reusable request-body component.
        reference: String,
    },
}

/// Inline request-body details used during rendering.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedInlineRequestBody {
    /// Human-readable description carried over from the source document.
    pub description: Option<String>,
    /// Whether the request body is required by the operation contract.
    pub required: bool,
    /// Supported media types in source order.
    ///
    /// The generator selects the first entry matching the configured content type.
    pub content: Vec<NormalizedMediaType>,
}

/// A normalized media type entry for request-body content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedMediaType {
    /// Media type name such as `application/json`.
    pub content_type: String,
    /// Optional schema describing the body payload for this media type.
    pub schema: Option<NormalizedSchema>,
}
