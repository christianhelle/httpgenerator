mod operations;
mod parameters;
mod references;
mod request_body;
mod schema;
mod servers;

#[cfg(test)]
mod tests;

use crate::{NormalizedOpenApiDocument, NormalizedSpecificationVersion};

use super::{
    OpenApiDocumentNormalizationError, OpenApiNormalizationError, OpenApiSpecificationVersion,
    ReadResult, TypedParseOptions, read,
};

/// Reads a document from a file path or URL string and immediately normalizes it.
///
/// External references to other files or URLs are merged into the document first. Documents that
/// do not fit the typed model for their specification version are rejected, unless they are
/// OpenAPI 3.1 documents and `options` tolerates that.
///
/// # Examples
///
/// ```no_run
/// use httpgenerator_core::openapi::{TypedParseOptions, load_and_normalize_document};
///
/// let normalized = load_and_normalize_document(
///     "test/OpenAPI/v3.0/petstore.json",
///     TypedParseOptions::default(),
/// )
/// .unwrap();
///
/// assert!(!normalized.operations.is_empty());
/// ```
pub fn load_and_normalize_document(
    input: &str,
    options: TypedParseOptions,
) -> Result<NormalizedOpenApiDocument, OpenApiDocumentNormalizationError> {
    let document =
        read(input).map_err(|e| OpenApiDocumentNormalizationError::Read(Box::new(e)))?;
    document
        .typed(options)
        .map_err(|e| OpenApiDocumentNormalizationError::TypedParse(Box::new(e)))?;
    normalize_document(&document)
        .map_err(|e| OpenApiDocumentNormalizationError::Normalize(Box::new(e)))
}

/// Normalizes a document read with [`read`] or [`super::OpenApiReader`] into the generator's
/// stable handoff model.
///
/// # Examples
///
/// ```
/// use httpgenerator_core::openapi::{FetchError, OpenApiReader, OpenApiSource, normalize_document};
///
/// let document = OpenApiReader::new()
///     .with_loader(|source: &OpenApiSource| match source.to_string().as_str() {
///         "openapi.json" => Ok(r#"{
///             "openapi": "3.0.2",
///             "info": { "title": "Example", "version": "1.0.0" },
///             "paths": {
///                 "/pets": {
///                     "get": {
///                         "operationId": "listPets",
///                         "responses": { "200": { "description": "ok" } }
///                     }
///                 }
///             }
///         }"#
///         .to_string()),
///         other => Err(FetchError::FileRead { path: other.into(), reason: "not found".into() }),
///     })
///     .read("openapi.json")
///     .unwrap();
///
/// let normalized = normalize_document(&document).unwrap();
///
/// assert_eq!(normalized.operations.len(), 1);
/// assert_eq!(normalized.operations[0].operation_id.as_deref(), Some("listPets"));
/// ```
pub fn normalize_document(
    document: &ReadResult,
) -> Result<NormalizedOpenApiDocument, OpenApiNormalizationError> {
    Ok(NormalizedOpenApiDocument {
        specification_version: normalize_specification_version(document.specification_version),
        servers: servers::normalize_servers(&document.document, &document.source)?,
        operations: operations::normalize_operations(&document.document)?,
    })
}

fn normalize_specification_version(
    version: OpenApiSpecificationVersion,
) -> NormalizedSpecificationVersion {
    match version {
        OpenApiSpecificationVersion::Swagger2 => NormalizedSpecificationVersion::Swagger2,
        OpenApiSpecificationVersion::OpenApi30 => NormalizedSpecificationVersion::OpenApi30,
        OpenApiSpecificationVersion::OpenApi31 => NormalizedSpecificationVersion::OpenApi31,
    }
}
