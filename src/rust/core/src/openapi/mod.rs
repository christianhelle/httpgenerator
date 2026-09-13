//! OpenAPI ingestion helpers for the crate's **load -> normalize -> generate** workflow.
//!
//! This module is available when the crate's `openapi` feature is enabled. The feature is enabled
//! by default, and docs.rs shows the gate explicitly so downstream users can tell when the surface
//! disappears in smaller `default-features = false` integrations.
//!
//! Reading, decoding, version detection, typed models and merging of external references come from
//! the [`oasreader`](https://docs.rs/oasreader) crate, whose public API is re-exported here.
//! Specifications split across multiple files or URLs are merged into a single document before
//! normalization.
//!
//! # Which function should I call?
//!
//! - **I have a CLI string that might be a path or URL** -> [`load_and_normalize_document`], or
//!   [`read`] followed by [`normalize_document`]
//! - **I need to customize how files and URLs are loaded** -> [`OpenApiReader`] followed by
//!   [`normalize_document`]
//! - **I only need format/version/source inspection** -> [`inspect_raw_document`] or
//!   [`inspect_document`]
//!
//! # Examples
//!
//! ```no_run
//! use httpgenerator_core::openapi::{TypedParseOptions, load_and_normalize_document, read};
//!
//! let document = read("test/OpenAPI/v3.0/petstore.json").unwrap();
//! assert!(!document.contained_external_references);
//!
//! let normalized = load_and_normalize_document(
//!     "test/OpenAPI/v3.0/petstore.json",
//!     TypedParseOptions::default(),
//! )
//! .unwrap();
//! assert!(!normalized.operations.is_empty());
//! ```

mod error;
mod inspect;
mod normalize;

pub use error::{
    OpenApiDocumentNormalizationError, OpenApiInspectionError, OpenApiNormalizationError,
};
pub use inspect::{OpenApiInspection, OpenApiStats, inspect_document, inspect_raw_document};
pub use normalize::{load_and_normalize_document, normalize_document};
pub use oasreader::*;
