//! Generator-ready API model used between OpenAPI ingestion and `.http` rendering.
//!
//! Use this module when you want to construct or inspect the stable handoff format consumed by
//! [`crate::generate_http_files`]. These types flatten Swagger/OpenAPI details into a smaller
//! shape that focuses on request generation: operations, parameters, request bodies, and schemas.
//!
//! The normalized model is plain data. It does not load source documents or render output by
//! itself, but it is the key bridge between the optional [`crate::openapi`] feature and the
//! generator.

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
