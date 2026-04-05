use std::fmt;

use serde_json::Value;

use crate::SpecificationVersionDetectionError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenApiSpecificationVersion {
    Swagger2,
    OpenApi30,
    OpenApi31,
}

impl fmt::Display for OpenApiSpecificationVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Swagger2 => write!(f, "Swagger 2.0"),
            Self::OpenApi30 => write!(f, "OpenAPI 3.0.x"),
            Self::OpenApi31 => write!(f, "OpenAPI 3.1.x"),
        }
    }
}

pub fn detect_specification_version(
    value: &Value,
) -> Result<OpenApiSpecificationVersion, SpecificationVersionDetectionError> {
    if let Some(openapi_version) = value.get("openapi") {
        return classify_openapi_version(openapi_version);
    }

    if let Some(swagger_version) = value.get("swagger") {
        return classify_swagger_version(swagger_version);
    }

    Err(SpecificationVersionDetectionError::MissingVersionField)
}

fn classify_openapi_version(
    value: &Value,
) -> Result<OpenApiSpecificationVersion, SpecificationVersionDetectionError> {
    let version = version_string(value, "openapi")?;
    let (major, minor) = parse_major_minor(version).ok_or_else(|| {
        SpecificationVersionDetectionError::UnsupportedVersion {
            field: "openapi",
            value: version.to_string(),
        }
    })?;

    match (major, minor) {
        (3, 0) => Ok(OpenApiSpecificationVersion::OpenApi30),
        (3, 1) => Ok(OpenApiSpecificationVersion::OpenApi31),
        _ => Err(SpecificationVersionDetectionError::UnsupportedVersion {
            field: "openapi",
            value: version.to_string(),
        }),
    }
}

fn classify_swagger_version(
    value: &Value,
) -> Result<OpenApiSpecificationVersion, SpecificationVersionDetectionError> {
    let version = version_string(value, "swagger")?;
    let (major, minor) = parse_major_minor(version).ok_or_else(|| {
        SpecificationVersionDetectionError::UnsupportedVersion {
            field: "swagger",
            value: version.to_string(),
        }
    })?;

    match (major, minor) {
        (2, 0) => Ok(OpenApiSpecificationVersion::Swagger2),
        _ => Err(SpecificationVersionDetectionError::UnsupportedVersion {
            field: "swagger",
            value: version.to_string(),
        }),
    }
}

fn version_string<'a>(
    value: &'a Value,
    field: &'static str,
) -> Result<&'a str, SpecificationVersionDetectionError> {
    value
        .as_str()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or(SpecificationVersionDetectionError::InvalidVersionFieldType { field })
}

fn parse_major_minor(version: &str) -> Option<(u64, u64)> {
    let mut parts = version.split('.');
    let major = parse_numeric_prefix(parts.next()?)?;
    let minor = parse_numeric_prefix(parts.next()?)?;
    Some((major, minor))
}

fn parse_numeric_prefix(component: &str) -> Option<u64> {
    let digits = component
        .trim()
        .chars()
        .take_while(|character| character.is_ascii_digit())
        .collect::<String>();

    (!digits.is_empty()).then(|| digits.parse().ok()).flatten()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use serde_json::json;

    use super::{OpenApiSpecificationVersion, detect_specification_version};
    use crate::{OpenApiSource, SpecificationVersionDetectionError, decode_raw_document};

    #[test]
    fn detects_swagger_two_documents() {
        let value = json!({
            "swagger": "2.0",
            "info": { "title": "Example" }
        });

        assert_eq!(
            detect_specification_version(&value).unwrap(),
            OpenApiSpecificationVersion::Swagger2
        );
    }

    #[test]
    fn detects_openapi_thirty_documents() {
        let value = json!({
            "openapi": "3.0.2",
            "info": { "title": "Example" }
        });

        assert_eq!(
            detect_specification_version(&value).unwrap(),
            OpenApiSpecificationVersion::OpenApi30
        );
    }

    #[test]
    fn detects_openapi_thirty_one_documents() {
        let value = json!({
            "openapi": "3.1.0",
            "info": { "title": "Example" }
        });

        assert_eq!(
            detect_specification_version(&value).unwrap(),
            OpenApiSpecificationVersion::OpenApi31
        );
    }

    #[test]
    fn reports_missing_version_fields() {
        let value = json!({
            "info": { "title": "Example" }
        });

        assert_eq!(
            detect_specification_version(&value).unwrap_err(),
            SpecificationVersionDetectionError::MissingVersionField
        );
    }

    #[test]
    fn reports_invalid_version_field_types() {
        let value = json!({
            "openapi": 3.1,
            "info": { "title": "Example" }
        });

        assert_eq!(
            detect_specification_version(&value).unwrap_err(),
            SpecificationVersionDetectionError::InvalidVersionFieldType { field: "openapi" }
        );
    }

    #[test]
    fn reports_unsupported_versions() {
        let value = json!({
            "openapi": "3.2.0",
            "info": { "title": "Example" }
        });

        assert_eq!(
            detect_specification_version(&value).unwrap_err(),
            SpecificationVersionDetectionError::UnsupportedVersion {
                field: "openapi",
                value: "3.2.0".to_string(),
            }
        );
    }

    #[test]
    fn raw_documents_expose_detected_specification_versions() {
        let document = decode_raw_document(
            OpenApiSource::Path(PathBuf::from("openapi.json")),
            r#"{"openapi":"3.0.2","info":{"title":"Example"}}"#,
        )
        .unwrap();

        assert_eq!(
            document.specification_version().unwrap(),
            OpenApiSpecificationVersion::OpenApi30
        );
    }
}
