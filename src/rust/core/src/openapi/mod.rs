//! OpenAPI ingestion helpers for the crate's **load -> normalize -> generate** workflow.
//!
//! This module is available when the crate's `openapi` feature is enabled. The feature is enabled
//! by default, and docs.rs shows the gate explicitly so downstream users can tell when the surface
//! disappears in smaller `default-features = false` integrations.
//!
//! The public pipeline has three layers:
//!
//! 1. **Source and raw decoding** with [`classify_source`], [`load_raw_document`],
//!    [`load_raw_document_from_source`], or [`decode_raw_document`]
//! 2. **Typed loading** with [`load_document`], [`load_document_from_source`], or
//!    [`load_document_from_raw`]
//! 3. **Generator-ready normalization** with [`load_and_normalize_document`] or
//!    [`normalize_loaded_document`]
//!
//! `LoadedOpenApiDocument` preserves the original [`RawOpenApiDocument`] in every variant so you
//! can inspect the source, content format, and version even when typed parsing is unavailable.
//!
//! # Which function should I call?
//!
//! - **I have a CLI string that might be a path or URL** -> [`load_document`] or
//!   [`load_and_normalize_document`]
//! - **I already know whether the input is a path or URL** -> [`load_document_from_source`] or
//!   [`load_raw_document_from_source`]
//! - **I need rustdoc-friendly or in-memory examples/tests** -> [`decode_raw_document`] followed by
//!   [`load_document_from_raw`]
//! - **I already have a [`LoadedOpenApiDocument`]** -> [`normalize_loaded_document`]
//! - **I only need format/version/source inspection** -> [`inspect_raw_document`] or
//!   [`inspect_document`]
//!
//! # Current behavior and fallbacks
//!
//! - Swagger 2.0 documents stay in a raw bridge variant until a typed Swagger model is introduced.
//! - OpenAPI 3.0 documents produce typed [`openapiv3::OpenAPI`] values.
//! - OpenAPI 3.1 documents usually produce typed [`openapiv3_1::OpenApi`] values, but webhook-only
//!   or tolerant-invalid inputs can intentionally fall back to [`LoadedOpenApiDocument::OpenApi31Raw`].
//! - Normalization reads from the preserved raw JSON tree so the generator can still work when a
//!   typed 3.1 document is intentionally unavailable.
//!
//! # Examples
//!
//! ```no_run
//! use httpgenerator_core::openapi::{
//!     LoadedOpenApiDocument, OpenApiSource, load_and_normalize_document, load_document_from_source,
//! };
//! use std::path::PathBuf;
//!
//! let loaded = load_document_from_source(OpenApiSource::Path(PathBuf::from(
//!     "test/OpenAPI/v3.0/petstore.json",
//! )))
//! .unwrap();
//!
//! assert!(matches!(loaded, LoadedOpenApiDocument::OpenApi30 { .. }));
//!
//! let normalized = load_and_normalize_document("test/OpenAPI/v3.0/petstore.json").unwrap();
//! assert!(!normalized.operations.is_empty());
//! ```

mod error;
mod format;
mod inspect;
mod loader;
mod normalize;
mod raw;
mod source;
mod typed;
mod version;

pub use error::{
    ContentFormatDetectionError, OpenApiDocumentLoadError, OpenApiDocumentNormalizationError,
    OpenApiInspectionError, OpenApiNormalizationError, RawOpenApiLoadError,
    SourceClassificationError, SpecificationVersionDetectionError, TypedOpenApiParseError,
};
pub use format::{OpenApiContentFormat, detect_content_format, sniff_content_format};
pub use inspect::{OpenApiInspection, OpenApiStats, inspect_document, inspect_raw_document};
pub use loader::{
    LoadedOpenApiDocument, load_document, load_document_from_raw, load_document_from_source,
};
pub use normalize::{
    load_and_normalize_document, load_and_normalize_document_with_options,
    normalize_loaded_document,
};
pub use raw::{
    RawOpenApiDocument, decode_raw_document, load_raw_document, load_raw_document_from_source,
};
pub use source::{OpenApiSource, classify_source};
pub use typed::{
    TypedOpenApiDocument, parse_openapi30_document, parse_openapi31_document, parse_typed_document,
};
pub use version::{OpenApiSpecificationVersion, detect_specification_version};
