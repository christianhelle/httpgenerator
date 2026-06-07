//! Typed OpenAPI parsing over a previously loaded raw document.
//!
//! This layer converts [`super::RawOpenApiDocument`] values into version-specific Rust models for
//! OpenAPI 3.0 and 3.1. Swagger 2.0 remains intentionally unsupported here, so callers can decide
//! whether to stay on the raw bridge or normalize directly.

use super::{OpenApiSpecificationVersion, RawOpenApiDocument, TypedOpenApiParseError};

/// Version-specific typed OpenAPI models exposed by this crate.
pub enum TypedOpenApiDocument {
    /// A parsed [`openapiv3::OpenAPI`] document.
    OpenApi30(Box<openapiv3::OpenAPI>),
    /// A parsed [`openapiv3_1::OpenApi`] document.
    OpenApi31(Box<openapiv3_1::OpenApi>),
}

impl TypedOpenApiDocument {
    /// Returns the specification version represented by this typed document.
    pub fn specification_version(&self) -> OpenApiSpecificationVersion {
        match self {
            Self::OpenApi30(_) => OpenApiSpecificationVersion::OpenApi30,
            Self::OpenApi31(_) => OpenApiSpecificationVersion::OpenApi31,
        }
    }
}

/// Parses a raw document into the matching typed OpenAPI model.
///
/// Use this as the typed front door when you want the crate to detect the specification version and
/// select the correct parser automatically.
///
/// # Examples
///
/// ```
/// use httpgenerator_core::openapi::{
///     OpenApiSource, TypedOpenApiDocument, decode_raw_document, parse_typed_document,
/// };
/// use std::path::PathBuf;
///
/// let raw = decode_raw_document(
///     OpenApiSource::Path(PathBuf::from("petstore.json")),
///     r#"{
///         "openapi": "3.0.2",
///         "info": { "title": "Example", "version": "1.0.0" },
///         "paths": {}
///     }"#,
/// )
/// .unwrap();
///
/// let typed = parse_typed_document(&raw).unwrap();
///
/// assert!(matches!(typed, TypedOpenApiDocument::OpenApi30(_)));
/// ```
pub fn parse_typed_document(
    document: &RawOpenApiDocument,
) -> Result<TypedOpenApiDocument, TypedOpenApiParseError> {
    match document.specification_version().map_err(|error| {
        TypedOpenApiParseError::VersionDetection {
            source: document.source().clone(),
            error: Box::new(error),
        }
    })? {
        OpenApiSpecificationVersion::Swagger2 => Err(TypedOpenApiParseError::UnsupportedVersion {
            source: document.source().clone(),
            version: OpenApiSpecificationVersion::Swagger2,
        }),
        OpenApiSpecificationVersion::OpenApi30 => {
            let document = parse_openapi30_document(document)?;
            Ok(TypedOpenApiDocument::OpenApi30(Box::new(document)))
        }
        OpenApiSpecificationVersion::OpenApi31 => {
            let document = parse_openapi31_document(document)?;
            Ok(TypedOpenApiDocument::OpenApi31(Box::new(document)))
        }
    }
}

/// Parses a raw document as OpenAPI 3.0.
///
/// Returns [`TypedOpenApiParseError::UnsupportedVersion`] when the raw document is not an OpenAPI
/// 3.0 document.
pub fn parse_openapi30_document(
    document: &RawOpenApiDocument,
) -> Result<openapiv3::OpenAPI, TypedOpenApiParseError> {
    parse_versioned_document(document, OpenApiSpecificationVersion::OpenApi30)
}

/// Parses a raw document as OpenAPI 3.1.
///
/// Returns [`TypedOpenApiParseError::UnsupportedVersion`] when the raw document is not an OpenAPI
/// 3.1 document.
pub fn parse_openapi31_document(
    document: &RawOpenApiDocument,
) -> Result<openapiv3_1::OpenApi, TypedOpenApiParseError> {
    parse_versioned_document(document, OpenApiSpecificationVersion::OpenApi31)
}

fn parse_versioned_document<T>(
    document: &RawOpenApiDocument,
    expected_version: OpenApiSpecificationVersion,
) -> Result<T, TypedOpenApiParseError>
where
    T: serde::de::DeserializeOwned,
{
    let detected_version = document.specification_version().map_err(|error| {
        TypedOpenApiParseError::VersionDetection {
            source: document.source().clone(),
            error: Box::new(error),
        }
    })?;

    if detected_version != expected_version {
        return Err(TypedOpenApiParseError::UnsupportedVersion {
            source: document.source().clone(),
            version: detected_version,
        });
    }

    serde_json::from_value(document.value().clone()).map_err(|error| {
        TypedOpenApiParseError::Deserialize {
            source: document.source().clone(),
            version: expected_version,
            reason: error.to_string(),
        }
    })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::openapi::{
        OpenApiSource, OpenApiSpecificationVersion, TypedOpenApiParseError, decode_raw_document,
    };

    use super::{
        TypedOpenApiDocument, parse_openapi30_document, parse_openapi31_document,
        parse_typed_document,
    };

    #[test]
    fn parses_openapi_thirty_documents_through_the_typed_front_door() {
        let raw = decode_raw_document(
            OpenApiSource::Path(PathBuf::from("petstore.json")),
            r#"{
                "openapi": "3.0.2",
                "info": { "title": "Example", "version": "1.0.0" },
                "paths": {}
            }"#,
        )
        .unwrap();

        let typed = parse_typed_document(&raw).unwrap();

        assert!(matches!(typed, TypedOpenApiDocument::OpenApi30(_)));
        assert_eq!(
            typed.specification_version(),
            OpenApiSpecificationVersion::OpenApi30
        );
    }

    #[test]
    fn parses_openapi_thirty_one_documents_through_the_typed_front_door() {
        let raw = decode_raw_document(
            OpenApiSource::Path(PathBuf::from("petstore.yaml")),
            "openapi: 3.1.0\ninfo:\n  title: Example\n  version: 1.0.0\npaths: {}\n",
        )
        .unwrap();

        let typed = parse_typed_document(&raw).unwrap();

        assert!(matches!(typed, TypedOpenApiDocument::OpenApi31(_)));
        assert_eq!(
            typed.specification_version(),
            OpenApiSpecificationVersion::OpenApi31
        );
    }

    #[test]
    fn rejects_swagger_two_documents_until_the_bridge_exists() {
        let raw = decode_raw_document(
            OpenApiSource::Path(PathBuf::from("swagger.json")),
            r#"{
                "swagger": "2.0",
                "info": { "title": "Example", "version": "1.0.0" },
                "paths": {}
            }"#,
        )
        .unwrap();

        match parse_typed_document(&raw) {
            Err(error) => {
                assert_eq!(
                    error,
                    TypedOpenApiParseError::UnsupportedVersion {
                        source: OpenApiSource::Path(PathBuf::from("swagger.json")),
                        version: OpenApiSpecificationVersion::Swagger2,
                    }
                );
            }
            Ok(_) => panic!("expected Swagger 2 documents to stay unsupported"),
        }
    }

    #[test]
    fn rejects_mismatched_version_specific_parsers() {
        let raw = decode_raw_document(
            OpenApiSource::Path(PathBuf::from("openapi.json")),
            r#"{
                "openapi": "3.1.0",
                "info": { "title": "Example", "version": "1.0.0" },
                "paths": {}
            }"#,
        )
        .unwrap();

        match parse_openapi30_document(&raw) {
            Err(error) => {
                assert_eq!(
                    error,
                    TypedOpenApiParseError::UnsupportedVersion {
                        source: OpenApiSource::Path(PathBuf::from("openapi.json")),
                        version: OpenApiSpecificationVersion::OpenApi31,
                    }
                );
            }
            Ok(_) => panic!("expected the 3.0 parser to reject a 3.1 document"),
        }

        parse_openapi31_document(&raw).unwrap();
    }
}
