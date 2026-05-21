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

/// Loads and inspects an OpenAPI document from a path or URL.
///
/// Inspection does not normalize operations for generation. It collects a
/// lightweight version and statistics summary useful for UI previews and
/// validation messages.
///
/// # Example
///
/// ```no_run
/// use httpgenerator_core::openapi::inspect_document;
///
/// let inspection = inspect_document("openapi.json")?;
///
/// println!("{} operations", inspection.stats.operation_count);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn inspect_document(input: &str) -> Result<OpenApiInspection, OpenApiInspectionError> {
    let raw = load_raw_document(input).map_err(OpenApiInspectionError::Load)?;
    inspect_raw_document(&raw)
}

/// Inspects an already decoded raw OpenAPI document.
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
