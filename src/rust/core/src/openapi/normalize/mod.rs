mod operations;
mod parameters;
mod request_body;
mod schema;
mod servers;

#[cfg(test)]
mod tests;

use crate::{NormalizedOpenApiDocument, NormalizedSpecificationVersion};

use super::loader::load_document_with_options;
use super::{
    LoadedOpenApiDocument, OpenApiDocumentNormalizationError, OpenApiNormalizationError,
    OpenApiSpecificationVersion,
};

/// Loads an OpenAPI document and converts it into the normalized renderer model.
///
/// This is the highest-level OpenAPI entry point for callers that want to
/// generate `.http` files from a path or URL.
///
/// # Example
///
/// ```no_run
/// use httpgenerator_core::{generate_http_files, GeneratorSettings};
/// use httpgenerator_core::openapi::load_and_normalize_document;
///
/// let input = "openapi.yaml";
/// let document = load_and_normalize_document(input)?;
/// let settings = GeneratorSettings {
///     open_api_path: input.to_string(),
///     ..GeneratorSettings::default()
/// };
/// let result = generate_http_files(&settings, &document);
///
/// println!("Generated {} file(s)", result.files.len());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn load_and_normalize_document(
    input: &str,
) -> Result<NormalizedOpenApiDocument, OpenApiDocumentNormalizationError> {
    load_and_normalize_document_with_options(input, false)
}

/// Loads and normalizes an OpenAPI document with tolerant OpenAPI 3.1 handling.
///
/// When `tolerate_invalid_openapi31` is `true`, OpenAPI 3.1 documents that
/// fail typed parsing can still be normalized from their raw JSON/YAML value.
pub fn load_and_normalize_document_with_options(
    input: &str,
    tolerate_invalid_openapi31: bool,
) -> Result<NormalizedOpenApiDocument, OpenApiDocumentNormalizationError> {
    let document = load_document_with_options(input, tolerate_invalid_openapi31)
        .map_err(OpenApiDocumentNormalizationError::Load)?;
    normalize_loaded_document(&document).map_err(OpenApiDocumentNormalizationError::Normalize)
}

/// Normalizes an already loaded OpenAPI document.
pub fn normalize_loaded_document(
    document: &LoadedOpenApiDocument,
) -> Result<NormalizedOpenApiDocument, OpenApiNormalizationError> {
    Ok(NormalizedOpenApiDocument {
        specification_version: normalize_specification_version(document),
        servers: servers::normalize_servers(document)?,
        operations: operations::normalize_operations(document.raw().value())?,
    })
}

fn normalize_specification_version(
    document: &LoadedOpenApiDocument,
) -> NormalizedSpecificationVersion {
    match document.specification_version() {
        OpenApiSpecificationVersion::Swagger2 => NormalizedSpecificationVersion::Swagger2,
        OpenApiSpecificationVersion::OpenApi30 => NormalizedSpecificationVersion::OpenApi30,
        OpenApiSpecificationVersion::OpenApi31 => NormalizedSpecificationVersion::OpenApi31,
    }
}
