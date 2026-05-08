use serde::{Deserialize, Serialize};

use super::NormalizedSchema;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NormalizedRequestBody {
    Inline(NormalizedInlineRequestBody),
    Reference { reference: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedInlineRequestBody {
    pub description: Option<String>,
    pub required: bool,
    pub content: Vec<NormalizedMediaType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedMediaType {
    pub content_type: String,
    pub schema: Option<NormalizedSchema>,
}
