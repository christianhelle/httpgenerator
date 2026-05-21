//! Inspection helpers for lightweight OpenAPI metadata and stats.
//!
//! Use this module when you want to identify the specification version and collect counts for
//! paths, operations, schemas, and related components without committing to typed parsing or
//! normalization yet.

mod components;
mod model;
mod paths;
mod schema;

#[cfg(test)]
mod tests;

use super::{
    OpenApiInspectionError, OpenApiSpecificationVersion, RawOpenApiDocument, load_raw_document,
};
pub use model::{OpenApiInspection, OpenApiStats};

/// Loads and inspects an OpenAPI document from a CLI-style path or URL input.
///
/// This is the most convenient inspection entry point when the caller only has the original user
/// input and wants version plus stats in one step.
///
/// # Examples
///
/// ```no_run
/// use httpgenerator_core::openapi::inspect_document;
///
/// let inspection = inspect_document("test/OpenAPI/v3.0/petstore.json").unwrap();
///
/// assert_eq!(inspection.specification_version.to_string(), "OpenAPI 3.0.x");
/// assert!(inspection.stats.operation_count > 0);
/// ```
pub fn inspect_document(input: &str) -> Result<OpenApiInspection, OpenApiInspectionError> {
    let raw = load_raw_document(input).map_err(OpenApiInspectionError::Load)?;
    inspect_raw_document(&raw)
}

/// Inspects a previously decoded raw OpenAPI document.
///
/// Use this when you already have a [`RawOpenApiDocument`] from [`super::decode_raw_document`] or
/// [`super::load_raw_document`] and want to avoid reloading the source.
///
/// # Examples
///
/// ```
/// use httpgenerator_core::openapi::{OpenApiSource, decode_raw_document, inspect_raw_document};
/// use std::path::PathBuf;
///
/// let raw = decode_raw_document(
///     OpenApiSource::Path(PathBuf::from("petstore.yaml")),
///     "openapi: 3.1.0\ninfo:\n  title: Example\n  version: 1.0.0\npaths: {}\n",
/// )
/// .unwrap();
///
/// let inspection = inspect_raw_document(&raw).unwrap();
///
/// assert_eq!(inspection.specification_version.to_string(), "OpenAPI 3.1.x");
/// assert_eq!(inspection.stats.path_item_count, 0);
/// ```
pub fn inspect_raw_document(
    document: &RawOpenApiDocument,
) -> Result<OpenApiInspection, OpenApiInspectionError> {
    let specification_version = document
        .specification_version()
        .map_err(OpenApiInspectionError::VersionDetection)?;

    Ok(OpenApiInspection {
        specification_version,
        stats: collect_stats(document.value(), specification_version),
    })
}

fn collect_stats(
    root: &serde_json::Value,
    specification_version: OpenApiSpecificationVersion,
) -> OpenApiStats {
    let mut stats = paths::collect_path_stats(root);
    components::collect_component_stats(root, specification_version, &mut stats);
    stats
}
