mod operations;
mod parameters;
mod request_body;
mod schema;
mod servers;

#[cfg(test)]
mod tests;

use crate::{NormalizedOpenApiDocument, NormalizedSpecificationVersion};

use super::loader::{LoadOptions, load_document};
use super::{
    LoadedOpenApiDocument, OpenApiDocumentNormalizationError, OpenApiNormalizationError,
    OpenApiSpecificationVersion,
};

/// Loads a document from a file path or URL string and immediately normalizes it.
///
/// This is the highest-level ingestion entry point for callers that want a
/// [`NormalizedOpenApiDocument`] and do not need to inspect the intermediate
/// [`LoadedOpenApiDocument`].
///
/// # Examples
///
/// ```no_run
/// use httpgenerator_core::openapi::{LoadOptions, load_and_normalize_document};
///
/// let normalized = load_and_normalize_document(
///     "test/OpenAPI/v3.0/petstore.json",
///     LoadOptions::default(),
/// )
/// .unwrap();
///
/// assert!(!normalized.operations.is_empty());
/// ```
pub fn load_and_normalize_document(
    input: &str,
    options: LoadOptions,
) -> Result<NormalizedOpenApiDocument, OpenApiDocumentNormalizationError> {
    let document =
        load_document(input, options).map_err(OpenApiDocumentNormalizationError::Load)?;
    normalize_loaded_document(&document).map_err(OpenApiDocumentNormalizationError::Normalize)
}

/// Normalizes a previously loaded document into the generator's stable handoff model.
///
/// Use this when a caller wants to inspect loading results first, cache a
/// [`LoadedOpenApiDocument`], or choose tolerant loading before converting to
/// [`NormalizedOpenApiDocument`].
///
/// # Examples
///
/// ```
/// use httpgenerator_core::openapi::{
///     LoadOptions, OpenApiSource, decode_raw_document, load_document_from_raw,
///     normalize_loaded_document,
/// };
/// use std::path::PathBuf;
///
/// let raw = decode_raw_document(
///     OpenApiSource::Path(PathBuf::from("openapi.json")),
///     r#"{
///         "openapi": "3.0.2",
///         "info": { "title": "Example", "version": "1.0.0" },
///         "paths": {
///             "/pets": {
///                 "get": {
///                     "operationId": "listPets",
///                     "responses": { "200": { "description": "ok" } }
///                 }
///             }
///         }
///     }"#,
/// )
/// .unwrap();
///
/// let loaded = load_document_from_raw(raw, LoadOptions::default()).unwrap();
/// let normalized = normalize_loaded_document(&loaded).unwrap();
///
/// assert_eq!(normalized.operations.len(), 1);
/// assert_eq!(normalized.operations[0].operation_id.as_deref(), Some("listPets"));
/// ```
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
