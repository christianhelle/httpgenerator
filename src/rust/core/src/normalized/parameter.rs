use serde::{Deserialize, Serialize};

use super::{NormalizedParameterLocation, NormalizedSchema};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NormalizedParameter {
    Inline(NormalizedInlineParameter),
    Reference { reference: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedInlineParameter {
    pub name: String,
    pub location: NormalizedParameterLocation,
    pub description: Option<String>,
    pub required: bool,
    pub schema: Option<NormalizedSchema>,
}
