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

pub fn load_and_normalize_document(
    input: &str,
) -> Result<NormalizedOpenApiDocument, OpenApiDocumentNormalizationError> {
    load_and_normalize_document_with_options(input, false)
}

pub fn load_and_normalize_document_with_options(
    input: &str,
    tolerate_invalid_openapi31: bool,
) -> Result<NormalizedOpenApiDocument, OpenApiDocumentNormalizationError> {
    let document = load_document_with_options(input, tolerate_invalid_openapi31)
        .map_err(OpenApiDocumentNormalizationError::Load)?;
    normalize_loaded_document(&document).map_err(OpenApiDocumentNormalizationError::Normalize)
}

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
