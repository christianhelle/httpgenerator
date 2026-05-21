use serde::{Deserialize, Serialize};

use super::{NormalizedHttpMethod, NormalizedParameter, NormalizedRequestBody};

/// OpenAPI specification version represented by a normalized document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NormalizedSpecificationVersion {
    /// Swagger/OpenAPI 2.0.
    Swagger2,
    /// OpenAPI 3.0.x.
    OpenApi30,
    /// OpenAPI 3.1.x.
    OpenApi31,
}

/// Version-neutral OpenAPI document used by the renderer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedOpenApiDocument {
    /// Detected source specification version.
    pub specification_version: NormalizedSpecificationVersion,
    /// Server definitions used to resolve `@baseUrl`.
    ///
    /// The renderer currently uses the first server, if any.
    pub servers: Vec<NormalizedServer>,
    /// Operations rendered into `.http` requests.
    pub operations: Vec<NormalizedOperation>,
}

/// A normalized server entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedServer {
    /// Server URL exactly as normalized from the source document.
    ///
    /// Values may be absolute URLs, relative paths, or template placeholders.
    pub url: String,
}

/// A normalized HTTP operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedOperation {
    /// OpenAPI path template, such as `/pets/{petId}`.
    pub path: String,
    /// HTTP method for the operation.
    pub method: NormalizedHttpMethod,
    /// Optional OpenAPI `operationId`.
    ///
    /// When absent, the renderer derives a name from the method and path.
    pub operation_id: Option<String>,
    /// Optional short summary rendered as request metadata.
    pub summary: Option<String>,
    /// Optional long description rendered as request metadata.
    pub description: Option<String>,
    /// Tags associated with the operation.
    ///
    /// [`crate::OutputType::OneFilePerTag`] uses the first tag for grouping.
    pub tags: Vec<String>,
    /// Parameters declared directly on or inherited by the operation.
    pub parameters: Vec<NormalizedParameter>,
    /// Optional request body definition.
    pub request_body: Option<NormalizedRequestBody>,
}
