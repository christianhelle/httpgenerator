//! Core library for building HTTP client request files from normalized API descriptions.
//!
//! `httpgenerator-core` is the reusable library behind the HTTP File Generator CLI and host
//! integrations. It exposes:
//!
//! - normalized API model types such as [`NormalizedOpenApiDocument`]
//! - generation settings and results such as [`GeneratorSettings`] and [`GeneratorResult`]
//! - `.http` rendering through [`generate_http_files`]
//! - small helper utilities for naming, URL resolution, redaction, and support identifiers
//!
//! By default, the crate enables the `openapi` feature, which adds document loading, inspection,
//! and normalization helpers under [`openapi`]. Disable default features if you only need the
//! normalized model, generation pipeline, or helper APIs in a smaller integration.
//!
//! # Workflow
//!
//! The main library flow is **load -> normalize -> generate**:
//!
//! 1. Use [`openapi`] to load or normalize Swagger/OpenAPI input when the feature is enabled.
//! 2. Use [`normalized`] when you already have a generator-ready [`NormalizedOpenApiDocument`].
//! 3. Use [`model`] to describe generator settings and collect generated files.
//! 4. Use [`generator`] or [`generate_http_files`] to render one or more `.http` files.
//!
//! # Generate a single `.http` file
//!
//! ```
//! use httpgenerator_core::{
//!     generate_http_files, GeneratorSettings, NormalizedHttpMethod, NormalizedOpenApiDocument,
//!     NormalizedOperation, NormalizedServer, NormalizedSpecificationVersion, OutputType,
//! };
//!
//! let settings = GeneratorSettings {
//!     open_api_path: "https://api.example.com/openapi.json".into(),
//!     output_type: OutputType::OneFile,
//!     ..Default::default()
//! };
//!
//! let document = NormalizedOpenApiDocument {
//!     specification_version: NormalizedSpecificationVersion::OpenApi30,
//!     servers: vec![NormalizedServer {
//!         url: "https://api.example.com".into(),
//!     }],
//!     operations: vec![NormalizedOperation {
//!         path: "/pets".into(),
//!         method: NormalizedHttpMethod::Get,
//!         operation_id: Some("listPets".into()),
//!         summary: Some("List pets".into()),
//!         description: None,
//!         tags: vec!["pets".into()],
//!         parameters: vec![],
//!         request_body: None,
//!     }],
//! };
//!
//! let result = generate_http_files(&settings, &document);
//!
//! assert_eq!(result.files.len(), 1);
//! assert_eq!(result.files[0].filename, "Requests.http");
//! assert!(result.files[0].content.contains("GET {{baseUrl}}/pets"));
//! ```
//!
//! # Load an OpenAPI document from disk
//!
//! ```no_run
//! # #[cfg(feature = "openapi")] {
//! use httpgenerator_core::openapi::{load_document, LoadedOpenApiDocument};
//!
//! let loaded = load_document("test/OpenAPI/v3.0/petstore.json").unwrap();
//!
//! assert!(matches!(loaded, LoadedOpenApiDocument::OpenApi30 { .. }));
//! # }
//! ```

pub mod base_url;
pub mod file_naming;
pub mod generator;
pub mod model;
pub mod normalized;
#[cfg(feature = "openapi")]
pub mod openapi;
pub mod operation_name;
pub mod privacy;
pub mod string_extensions;
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
