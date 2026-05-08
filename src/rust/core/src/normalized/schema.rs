use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct NormalizedSchema {
    pub reference: Option<String>,
    pub types: Vec<NormalizedSchemaType>,
    pub properties: Vec<NormalizedSchemaProperty>,
    pub items: Option<Box<NormalizedSchema>>,
    pub all_of: Vec<NormalizedSchema>,
    pub one_of: Vec<NormalizedSchema>,
    pub any_of: Vec<NormalizedSchema>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedSchemaProperty {
    pub name: String,
    pub schema: NormalizedSchema,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NormalizedSchemaType {
    String,
    Integer,
    Number,
    Boolean,
    Object,
    Array,
    Null,
    Other(String),
}
