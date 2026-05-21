use serde::{Deserialize, Serialize};

use super::{NormalizedHttpMethod, NormalizedParameter, NormalizedRequestBody};

/// Normalized specification family for a source document.
///
/// This preserves which top-level dialect produced the normalized model while keeping generation
/// logic independent from the original parser types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NormalizedSpecificationVersion {
    /// A Swagger 2.0 document.
    Swagger2,
    /// An OpenAPI 3.0.x document.
    OpenApi30,
    /// An OpenAPI 3.1.x document.
    OpenApi31,
}

/// Generator-ready view of an API document.
///
/// Use this type as the main handoff into [`crate::generate_http_files`]. It contains the
/// normalized servers and operations that the generator needs, without exposing parser-specific
/// OpenAPI data structures.
///
/// # Examples
///
/// ```
/// use httpgenerator_core::{
///     NormalizedHttpMethod, NormalizedOpenApiDocument, NormalizedOperation, NormalizedServer,
///     NormalizedSpecificationVersion,
/// };
///
/// let document = NormalizedOpenApiDocument {
///     specification_version: NormalizedSpecificationVersion::OpenApi30,
///     servers: vec![NormalizedServer {
///         url: "https://api.example.com".into(),
///     }],
///     operations: vec![NormalizedOperation {
///         path: "/pets".into(),
///         method: NormalizedHttpMethod::Get,
///         operation_id: Some("listPets".into()),
///         summary: Some("List pets".into()),
///         description: None,
///         tags: vec!["pets".into()],
///         parameters: vec![],
///         request_body: None,
///     }],
/// };
///
/// assert_eq!(document.operations.len(), 1);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedOpenApiDocument {
    /// Source specification version that produced this normalized document.
    pub specification_version: NormalizedSpecificationVersion,
    /// Normalized server candidates in source order.
    ///
    /// The generator currently uses the first server when resolving the default `@baseUrl`.
    pub servers: Vec<NormalizedServer>,
    /// Operations available for rendering into `.http` requests.
    pub operations: Vec<NormalizedOperation>,
}

/// Server entry carried forward into the normalized document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedServer {
    /// Server URL exactly as normalized from the source document.
    pub url: String,
}

/// Generator-facing representation of a single API operation.
///
/// This combines the HTTP method, route, optional metadata, parameters, and request body into the
/// shape consumed by the renderer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedOperation {
    /// Path template used when rendering the request URL, such as `/pets/{petId}`.
    pub path: String,
    /// HTTP method for the rendered request.
    pub method: NormalizedHttpMethod,
    /// Preferred stable identifier when the source document defines one.
    pub operation_id: Option<String>,
    /// Short summary shown in generated request comments when present.
    pub summary: Option<String>,
    /// Longer operation description shown in generated request comments when present.
    pub description: Option<String>,
    /// Tags associated with the operation in source order.
    ///
    /// When generating [`crate::OutputType::OneFilePerTag`], the first tag is used as the file
    /// grouping key. Operations without tags fall back to the default group.
    pub tags: Vec<String>,
    /// Parameters attached to the operation after normalization.
    pub parameters: Vec<NormalizedParameter>,
    /// Optional normalized request body for methods that accept one.
    pub request_body: Option<NormalizedRequestBody>,
}
