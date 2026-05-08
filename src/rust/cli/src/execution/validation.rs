use httpgenerator_core::openapi::{
    OpenApiInspection, OpenApiSpecificationVersion, inspect_document,
};

use crate::CliError;

pub(super) fn validate_openapi_document(
    open_api_path: &str,
    skip_validation: bool,
) -> Result<Option<OpenApiInspection>, CliError> {
    if skip_validation {
        return Ok(None);
    }

    let inspection = inspect_document(open_api_path)
        .map_err(|error| CliError::InspectOpenApi(error.to_string()))?;

    if inspection.specification_version == OpenApiSpecificationVersion::OpenApi31 {
        return Err(CliError::UnsupportedValidationVersion {
            version: inspection.specification_version,
        });
    }

    Ok(Some(inspection))
}
