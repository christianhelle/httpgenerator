use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NormalizedSpecificationVersion {
    Swagger2,
    OpenApi30,
    OpenApi31,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NormalizedHttpMethod {
    Get,
    Put,
    Post,
    Delete,
    Options,
    Head,
    Patch,
    Trace,
}

impl NormalizedHttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Get => "get",
            Self::Put => "put",
            Self::Post => "post",
            Self::Delete => "delete",
            Self::Options => "options",
            Self::Head => "head",
            Self::Patch => "patch",
            Self::Trace => "trace",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NormalizedParameterLocation {
    Path,
    Query,
    Header,
    Cookie,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedOpenApiDocument {
    pub specification_version: NormalizedSpecificationVersion,
    pub servers: Vec<NormalizedServer>,
    pub operations: Vec<NormalizedOperation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedServer {
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedOperation {
    pub path: String,
    pub method: NormalizedHttpMethod,
    pub operation_id: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub parameters: Vec<NormalizedParameter>,
    pub request_body: Option<NormalizedRequestBody>,
}

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

#[cfg(test)]
mod tests {
    use super::NormalizedHttpMethod;

    #[test]
    fn normalized_http_method_strings_match_openapi_keys() {
        assert_eq!(NormalizedHttpMethod::Get.as_str(), "get");
        assert_eq!(NormalizedHttpMethod::Patch.as_str(), "patch");
        assert_eq!(NormalizedHttpMethod::Trace.as_str(), "trace");
    }
}
