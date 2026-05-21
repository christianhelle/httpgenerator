use serde::{Deserialize, Serialize};

/// Normalized schema information used for request samples and placeholders.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct NormalizedSchema {
    /// Optional unresolved OpenAPI `$ref`.
    pub reference: Option<String>,
    /// Primitive or composite schema types.
    pub types: Vec<NormalizedSchemaType>,
    /// Object properties.
    pub properties: Vec<NormalizedSchemaProperty>,
    /// Array item schema.
    pub items: Option<Box<NormalizedSchema>>,
    /// Schemas combined with `allOf`.
    pub all_of: Vec<NormalizedSchema>,
    /// Schemas combined with `oneOf`.
    pub one_of: Vec<NormalizedSchema>,
    /// Schemas combined with `anyOf`.
    pub any_of: Vec<NormalizedSchema>,
}

/// Named property in a normalized object schema.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedSchemaProperty {
    /// Property name.
    pub name: String,
    /// Property schema.
    pub schema: NormalizedSchema,
}

/// Schema type keywords recognized by the normalized model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NormalizedSchemaType {
    /// JSON string value.
    String,
    /// JSON integer value.
    Integer,
    /// JSON number value.
    Number,
    /// JSON boolean value.
    Boolean,
    /// JSON object value.
    Object,
    /// JSON array value.
    Array,
    /// JSON null value.
    Null,
    /// Schema type not represented by a built-in variant.
    Other(String),
}
