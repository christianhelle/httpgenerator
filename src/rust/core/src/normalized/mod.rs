//! Renderer-friendly OpenAPI model.
//!
//! The normalized model keeps only the information the `.http` renderer needs
//! and presents it in a version-neutral shape. OpenAPI 2.0, 3.0, and 3.1
//! documents can all be converted into these types before generation.
//!
//! You can build this model directly when integrating with another parser, or
//! obtain it from [`crate::openapi::load_and_normalize_document`] when the
//! `openapi` feature is enabled.

mod document;
mod http;
mod parameter;
mod request_body;
mod schema;

#[cfg(test)]
mod tests;

pub use document::{
    NormalizedOpenApiDocument, NormalizedOperation, NormalizedServer,
    NormalizedSpecificationVersion,
};
pub use http::{NormalizedHttpMethod, NormalizedParameterLocation};
pub use parameter::{NormalizedInlineParameter, NormalizedParameter};
pub use request_body::{NormalizedInlineRequestBody, NormalizedMediaType, NormalizedRequestBody};
pub use schema::{NormalizedSchema, NormalizedSchemaProperty, NormalizedSchemaType};
