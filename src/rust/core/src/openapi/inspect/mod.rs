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

pub fn inspect_document(input: &str) -> Result<OpenApiInspection, OpenApiInspectionError> {
    let raw = load_raw_document(input).map_err(OpenApiInspectionError::Load)?;
    inspect_raw_document(&raw)
}

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
