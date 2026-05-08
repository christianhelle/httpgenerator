use serde::{Deserialize, Serialize};

use super::{NormalizedHttpMethod, NormalizedParameter, NormalizedRequestBody};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NormalizedSpecificationVersion {
    Swagger2,
    OpenApi30,
    OpenApi31,
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
