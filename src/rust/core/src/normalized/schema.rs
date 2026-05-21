use serde::{Deserialize, Serialize};

/// Normalized schema tree used for parameters and request bodies.
///
/// The shape is intentionally smaller than the source OpenAPI schema model. It captures the parts
/// the generator needs for sample payloads and parameter defaults: references, primitive/object
/// kinds, nested properties, collection items, and composition keywords.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct NormalizedSchema {
    /// Optional schema reference identifier when the value points at a reusable component.
    pub reference: Option<String>,
    /// Schema kinds associated with this node.
    pub types: Vec<NormalizedSchemaType>,
    /// Named object properties in source order.
    pub properties: Vec<NormalizedSchemaProperty>,
    /// Item schema for array-like shapes.
    pub items: Option<Box<NormalizedSchema>>,
    /// Schemas that participate in `allOf` composition.
    pub all_of: Vec<NormalizedSchema>,
    /// Schemas that participate in `oneOf` composition.
    pub one_of: Vec<NormalizedSchema>,
    /// Schemas that participate in `anyOf` composition.
    pub any_of: Vec<NormalizedSchema>,
}

/// Named property within a normalized object schema.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedSchemaProperty {
    /// Property name as exposed in the generated payload.
    pub name: String,
    /// Property schema.
    pub schema: NormalizedSchema,
}

/// Primitive or structural schema kind used by the normalized model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NormalizedSchemaType {
    /// String-like value.
    String,
    /// Integer value.
    Integer,
    /// Floating-point or decimal number value.
    Number,
    /// Boolean value.
    Boolean,
    /// Object value with properties.
    Object,
    /// Array value with item schema.
    Array,
    /// Explicit `null` value.
    Null,
    /// Any other source-specific schema type preserved as-is.
    Other(
        /// Original schema type string carried through normalization.
        String,
    ),
}
