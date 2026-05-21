//! Core generation, normalization, and OpenAPI support for HTTP File Generator.
//!
//! `httpgenerator-core` contains the reusable library pieces behind the
//! `httpgenerator` command-line tool:
//!
//! - [`generate_http_files`] renders normalized operations into `.http` files.
//! - [`GeneratorSettings`] controls output layout, headers, and request
//!   placeholders.
//! - [`normalized`] contains the generator-friendly OpenAPI model accepted by
//!   the renderer.
//! - [`openapi`] loads, inspects, parses, and normalizes OpenAPI documents when
//!   the default `openapi` feature is enabled.
//!
//! Most applications start with [`openapi::load_and_normalize_document`] and
//! pass the resulting [`NormalizedOpenApiDocument`] to [`generate_http_files`].
//! Consumers that already have their own OpenAPI parser can build the
//! normalized model directly.
//!
//! # Example
//!
//! ```
//! use httpgenerator_core::{
//!     generate_http_files, GeneratorSettings, NormalizedHttpMethod,
//!     NormalizedOpenApiDocument, NormalizedOperation, NormalizedServer,
//!     NormalizedSpecificationVersion,
//! };
//!
//! let document = NormalizedOpenApiDocument {
//!     specification_version: NormalizedSpecificationVersion::OpenApi30,
//!     servers: vec![NormalizedServer {
//!         url: "https://api.example.com".to_string(),
//!     }],
//!     operations: vec![NormalizedOperation {
//!         path: "/pets".to_string(),
//!         method: NormalizedHttpMethod::Get,
//!         operation_id: Some("listPets".to_string()),
//!         summary: Some("List pets".to_string()),
//!         description: None,
//!         tags: vec!["Pets".to_string()],
//!         parameters: Vec::new(),
//!         request_body: None,
//!     }],
//! };
//!
//! let settings = GeneratorSettings {
//!     open_api_path: "openapi.json".to_string(),
//!     ..GeneratorSettings::default()
//! };
//!
//! let result = generate_http_files(&settings, &document);
//!
//! assert_eq!(result.files[0].filename, "GetListPets.http");
//! assert!(result.files[0].content.contains("GET {{baseUrl}}/pets"));
//! ```
//!
//! # Feature flags
//!
//! The `openapi` feature is enabled by default and exposes the
//! [`openapi`] module. Disabling default features keeps the renderer,
//! normalized model, and helper APIs available without OpenAPI parser or HTTP
//! client dependencies.
#![warn(missing_docs)]

/// Base URL resolution helpers used when rendering request files.
pub mod base_url;
/// Filename helpers for collision-safe `.http` outputs.
pub mod file_naming;
/// `.http` file rendering from normalized OpenAPI documents.
pub mod generator;
/// Input settings and output model types for generation.
pub mod model;
/// OpenAPI shapes normalized into a renderer-friendly model.
pub mod normalized;
#[cfg(feature = "openapi")]
/// OpenAPI loading, parsing, inspection, and normalization.
pub mod openapi;
/// Operation name generation helpers.
pub mod operation_name;
/// Privacy helpers for redacting sensitive command-line values.
pub mod privacy;
/// String conversion helpers used by compatibility naming rules.
pub mod string_extensions;
/// Anonymous support identifier helpers.
pub mod support_information;

pub use base_url::resolve_base_url;
pub use file_naming::unique_filename;
pub use generator::generate_http_files;
pub use model::{GeneratorResult, GeneratorSettings, HttpFile, OutputType};
pub use normalized::{
    NormalizedHttpMethod, NormalizedInlineParameter, NormalizedInlineRequestBody,
    NormalizedMediaType, NormalizedOpenApiDocument, NormalizedOperation, NormalizedParameter,
    NormalizedParameterLocation, NormalizedRequestBody, NormalizedSchema, NormalizedSchemaProperty,
    NormalizedSchemaType, NormalizedServer, NormalizedSpecificationVersion,
};
pub use operation_name::generate_operation_name;
pub use privacy::redact_authorization_headers;
pub use string_extensions::{
    capitalize_first_character, convert_kebab_case_to_pascal_case, convert_route_to_camel_case,
    convert_spaces_to_pascal_case, prefix, prefix_line_breaks,
};
pub use support_information::{
    anonymous_identity, anonymous_identity_from_parts, support_key,
    support_key_from_anonymous_identity,
};
