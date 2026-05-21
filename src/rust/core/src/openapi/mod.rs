//! OpenAPI source loading, parsing, inspection, and normalization.
//!
//! This module is available when the default `openapi` feature is enabled. It
//! provides layered entry points:
//!
//! - [`classify_source`] distinguishes local paths from `http` and `https`
//!   URLs.
//! - [`load_raw_document`] reads and decodes JSON or YAML into a
//!   [`RawOpenApiDocument`].
//! - [`load_document`] additionally parses supported versions into typed
//!   OpenAPI structures where available.
//! - [`inspect_document`] collects lightweight statistics for UI or telemetry.
//! - [`load_and_normalize_document`] converts an OpenAPI document into the
//!   renderer-friendly [`crate::NormalizedOpenApiDocument`].

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
