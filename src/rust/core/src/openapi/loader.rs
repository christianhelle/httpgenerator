//! Loading helpers that bridge raw input into typed or raw-fallback OpenAPI documents.
//!
//! For doctests and other self-contained examples, prefer [`load_document_from_raw`] with
//! [`super::decode_raw_document`] so the example stays independent from fixture paths or network
//! access. Use [`load_document`] or [`load_document_from_source`] for real file paths and URLs.

use super::{
    OpenApiContentFormat, OpenApiDocumentLoadError, OpenApiSource, OpenApiSpecificationVersion,
    RawOpenApiDocument, TypedOpenApiDocument, TypedOpenApiParseError, load_raw_document,
    load_raw_document_from_source, parse_typed_document,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct LoadOptions {
    pub tolerate_invalid_openapi31: bool,
}

/// Represents a successfully loaded OpenAPI document.
///
/// The loader keeps the original [`RawOpenApiDocument`] for every variant so callers can preserve
/// source and format metadata even when a typed model is unavailable. OpenAPI 3.1 input may land
/// in [`LoadedOpenApiDocument::OpenApi31Raw`] when the document is webhook-only or when tolerant
/// loading is explicitly enabled for invalid-but-still-useful specs.
///
/// # Examples
///
/// ```
/// use httpgenerator_core::openapi::{
///     LoadOptions, LoadedOpenApiDocument, OpenApiSource, OpenApiSpecificationVersion,
///     decode_raw_document, load_document_from_raw,
/// };
/// use std::path::PathBuf;
///
/// let raw = decode_raw_document(
///     OpenApiSource::Path(PathBuf::from("openapi.json")),
///     r#"{
///         "openapi": "3.0.2",
///         "info": { "title": "Example", "version": "1.0.0" },
///         "paths": {}
///     }"#,
/// )
/// .unwrap();
///
/// let loaded = load_document_from_raw(raw, LoadOptions::default()).unwrap();
///
/// assert!(matches!(loaded, LoadedOpenApiDocument::OpenApi30 { .. }));
/// assert_eq!(
///     loaded.specification_version(),
///     OpenApiSpecificationVersion::OpenApi30
/// );
/// ```
pub enum LoadedOpenApiDocument {
    /// A Swagger 2.0 document preserved as raw input until a typed bridge exists.
    Swagger2 {
        /// The decoded source document and its metadata.
        raw: RawOpenApiDocument,
    },
    /// An OpenAPI 3.0 document with both raw and typed representations.
    OpenApi30 {
        /// The decoded source document and its metadata.
        raw: RawOpenApiDocument,
        /// The parsed OpenAPI 3.0 model.
        document: openapiv3::OpenAPI,
    },
    /// An OpenAPI 3.1 document with both raw and typed representations.
    OpenApi31 {
        /// The decoded source document and its metadata.
        raw: RawOpenApiDocument,
        /// The parsed OpenAPI 3.1 model.
        document: openapiv3_1::OpenApi,
    },
    /// An OpenAPI 3.1 document kept as raw input because typed parsing was intentionally skipped.
    OpenApi31Raw {
        /// The decoded source document and its metadata.
        raw: RawOpenApiDocument,
    },
}

impl LoadedOpenApiDocument {
    /// Returns the raw document, regardless of which typed variant was produced.
    pub fn raw(&self) -> &RawOpenApiDocument {
        match self {
            Self::Swagger2 { raw }
            | Self::OpenApi30 { raw, .. }
            | Self::OpenApi31 { raw, .. }
            | Self::OpenApi31Raw { raw } => raw,
        }
    }

    /// Returns the source the document was loaded from.
    pub fn source(&self) -> &OpenApiSource {
        self.raw().source()
    }

    /// Returns the detected content format for the loaded document.
    pub fn format(&self) -> OpenApiContentFormat {
        self.raw().format()
    }

    /// Returns the detected specification version.
    pub fn specification_version(&self) -> OpenApiSpecificationVersion {
        match self {
            Self::Swagger2 { .. } => OpenApiSpecificationVersion::Swagger2,
            Self::OpenApi30 { .. } => OpenApiSpecificationVersion::OpenApi30,
            Self::OpenApi31 { .. } | Self::OpenApi31Raw { .. } => {
                OpenApiSpecificationVersion::OpenApi31
            }
        }
    }
}

/// Loads a document from a file path or URL string.
///
/// This is the most convenient entry point when the caller already has the original string input
/// from the CLI or another host surface.
///
/// # Examples
///
/// ```no_run
/// use httpgenerator_core::openapi::{LoadOptions, LoadedOpenApiDocument, load_document};
///
/// let loaded = load_document("test/OpenAPI/v3.0/petstore.json", LoadOptions::default()).unwrap();
///
/// assert!(matches!(loaded, LoadedOpenApiDocument::OpenApi30 { .. }));
/// ```
pub fn load_document(
    input: &str,
    options: LoadOptions,
) -> Result<LoadedOpenApiDocument, OpenApiDocumentLoadError> {
    let raw = load_raw_document(input).map_err(OpenApiDocumentLoadError::RawLoad)?;
    load_document_from_raw(raw, options)
}

/// Loads a document from a pre-classified [`OpenApiSource`].
///
/// Use this when the calling layer has already decided whether the input is a path or URL.
///
/// # Examples
///
/// ```no_run
/// use httpgenerator_core::openapi::{
///     LoadOptions, LoadedOpenApiDocument, OpenApiSource, load_document_from_source,
/// };
/// use std::path::PathBuf;
///
/// let loaded = load_document_from_source(
///     OpenApiSource::Path(PathBuf::from("test/OpenAPI/v3.1/webhook-example.json")),
///     LoadOptions::default(),
/// )
/// .unwrap();
///
/// assert!(matches!(
///     loaded,
///     LoadedOpenApiDocument::OpenApi31 { .. } | LoadedOpenApiDocument::OpenApi31Raw { .. }
/// ));
/// ```
pub fn load_document_from_source(
    source: OpenApiSource,
    options: LoadOptions,
) -> Result<LoadedOpenApiDocument, OpenApiDocumentLoadError> {
    let raw = load_raw_document_from_source(source).map_err(OpenApiDocumentLoadError::RawLoad)?;
    load_document_from_raw(raw, options)
}

/// Loads a document from a previously decoded raw representation.
///
/// This is the most rustdoc-friendly entry point because the caller can embed the document content
/// directly in code without relying on files or network access.
///
/// # Examples
///
/// ```
/// use httpgenerator_core::openapi::{
///     LoadOptions, LoadedOpenApiDocument, OpenApiSource, decode_raw_document,
///     load_document_from_raw,
/// };
/// use std::path::PathBuf;
///
/// let raw = decode_raw_document(
///     OpenApiSource::Path(PathBuf::from("openapi.yaml")),
///     "openapi: 3.1.0\ninfo:\n  title: Example\n  version: 1.0.0\npaths: {}\n",
/// )
/// .unwrap();
///
/// let loaded = load_document_from_raw(raw, LoadOptions::default()).unwrap();
///
/// assert!(matches!(loaded, LoadedOpenApiDocument::OpenApi31 { .. }));
/// ```
pub fn load_document_from_raw(
    raw: RawOpenApiDocument,
    options: LoadOptions,
) -> Result<LoadedOpenApiDocument, OpenApiDocumentLoadError> {
    if matches!(
        raw.specification_version(),
        Ok(OpenApiSpecificationVersion::Swagger2)
    ) {
        return Ok(LoadedOpenApiDocument::Swagger2 { raw });
    }

    match parse_typed_document(&raw) {
        Ok(TypedOpenApiDocument::OpenApi30(document)) => {
            Ok(LoadedOpenApiDocument::OpenApi30 { raw, document })
        }
        Ok(TypedOpenApiDocument::OpenApi31(document)) => {
            Ok(LoadedOpenApiDocument::OpenApi31 { raw, document })
        }
        Err(TypedOpenApiParseError::Deserialize {
            version: OpenApiSpecificationVersion::OpenApi31,
            ..
        }) if should_fallback_to_raw_openapi31(&raw, options.tolerate_invalid_openapi31) => {
            Ok(LoadedOpenApiDocument::OpenApi31Raw { raw })
        }
        Err(error) => Err(OpenApiDocumentLoadError::TypedParse(error)),
    }
}

fn should_fallback_to_raw_openapi31(
    raw: &RawOpenApiDocument,
    tolerate_invalid_openapi31: bool,
) -> bool {
    matches!(
        raw.specification_version(),
        Ok(OpenApiSpecificationVersion::OpenApi31)
    ) && (is_webhook_only_openapi31_document(raw) || tolerate_invalid_openapi31)
}

fn is_webhook_only_openapi31_document(raw: &RawOpenApiDocument) -> bool {
    matches!(
        raw.specification_version(),
        Ok(OpenApiSpecificationVersion::OpenApi31)
    ) && raw.value().get("paths").is_none()
        && raw
            .value()
            .get("webhooks")
            .and_then(serde_json::Value::as_object)
            .is_some()
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::{Path, PathBuf},
        sync::atomic::{AtomicU64, Ordering},
    };

    use crate::openapi::{
        OpenApiContentFormat, OpenApiSource, OpenApiSpecificationVersion, decode_raw_document,
    };

    use super::{
        LoadOptions, LoadedOpenApiDocument, load_document, load_document_from_raw,
        load_document_from_source,
    };

    static TEST_ARTIFACT_ID: AtomicU64 = AtomicU64::new(0);

    #[test]
    fn loads_openapi_thirty_documents_from_raw_input() {
        let raw = decode_raw_document(
            OpenApiSource::Path(PathBuf::from("openapi.json")),
            r#"{
                "openapi": "3.0.2",
                "info": { "title": "Example", "version": "1.0.0" },
                "paths": {}
            }"#,
        )
        .unwrap();

        let loaded = load_document_from_raw(raw, LoadOptions::default()).unwrap();

        assert!(matches!(loaded, LoadedOpenApiDocument::OpenApi30 { .. }));
        assert_eq!(
            loaded.specification_version(),
            OpenApiSpecificationVersion::OpenApi30
        );
    }

    #[test]
    fn loads_openapi_thirty_one_documents_from_a_source() {
        let file = TestFile::new(
            "openapi.yaml",
            "openapi: 3.1.0\ninfo:\n  title: Example\n  version: 1.0.0\npaths: {}\n",
        );

        let loaded = load_document_from_source(
            OpenApiSource::Path(file.path().to_path_buf()),
            LoadOptions::default(),
        )
        .unwrap();

        assert!(matches!(loaded, LoadedOpenApiDocument::OpenApi31 { .. }));
        assert_eq!(
            loaded.specification_version(),
            OpenApiSpecificationVersion::OpenApi31
        );
        assert_eq!(
            loaded.source(),
            &OpenApiSource::Path(file.path().to_path_buf())
        );
    }

    #[test]
    fn loads_webhook_only_openapi_thirty_one_documents_with_a_raw_fallback() {
        let raw = decode_raw_document(
            OpenApiSource::Path(PathBuf::from("test/OpenAPI/v3.1/webhook-example.json")),
            include_str!("../../../../../test/OpenAPI/v3.1/webhook-example.json"),
        )
        .unwrap();

        let loaded = load_document_from_raw(raw, LoadOptions::default()).unwrap();

        assert!(matches!(loaded, LoadedOpenApiDocument::OpenApi31Raw { .. }));
        assert_eq!(
            loaded.specification_version(),
            OpenApiSpecificationVersion::OpenApi31
        );
    }

    #[test]
    fn tolerant_loader_accepts_invalid_openapi_thirty_one_documents() {
        let raw = decode_raw_document(
            OpenApiSource::Path(PathBuf::from("test/OpenAPI/v3.1/non-oauth-scopes.json")),
            include_str!("../../../../../test/OpenAPI/v3.1/non-oauth-scopes.json"),
        )
        .unwrap();

        let loaded = load_document_from_raw(
            raw,
            LoadOptions {
                tolerate_invalid_openapi31: true,
            },
        )
        .unwrap();

        assert!(matches!(loaded, LoadedOpenApiDocument::OpenApi31Raw { .. }));
        assert_eq!(
            loaded.specification_version(),
            OpenApiSpecificationVersion::OpenApi31
        );
    }

    #[test]
    fn loads_swagger_two_documents_with_a_raw_bridge() {
        let raw = decode_raw_document(
            OpenApiSource::Path(PathBuf::from("swagger.json")),
            r#"{
                "swagger": "2.0",
                "info": { "title": "Example", "version": "1.0.0" },
                "paths": {}
            }"#,
        )
        .unwrap();

        let loaded = load_document_from_raw(raw, LoadOptions::default()).unwrap();

        assert!(matches!(loaded, LoadedOpenApiDocument::Swagger2 { .. }));
        assert_eq!(
            loaded.specification_version(),
            OpenApiSpecificationVersion::Swagger2
        );
    }

    #[test]
    fn load_document_reads_and_parses_local_files() {
        let file = TestFile::new(
            "openapi.json",
            r#"{
                "openapi": "3.0.2",
                "info": { "title": "Example", "version": "1.0.0" },
                "paths": {}
            }"#,
        );

        let loaded = load_document(file.path().to_str().unwrap(), LoadOptions::default()).unwrap();

        assert!(matches!(loaded, LoadedOpenApiDocument::OpenApi30 { .. }));
        assert_eq!(loaded.format(), OpenApiContentFormat::Json);
    }

    fn unique_test_path(file_name: &str) -> PathBuf {
        let artifact_id = TEST_ARTIFACT_ID.fetch_add(1, Ordering::Relaxed);
        let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("test-data");

        fs::create_dir_all(&directory).unwrap();

        directory.join(format!(
            "loader-{}-{}-{file_name}",
            std::process::id(),
            artifact_id
        ))
    }

    struct TestFile {
        path: PathBuf,
    }

    impl TestFile {
        fn new(file_name: &str, content: &str) -> Self {
            let path = unique_test_path(file_name);
            fs::write(&path, content).unwrap();
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TestFile {
        fn drop(&mut self) {
            let _ = fs::remove_file(&self.path);
        }
    }
}
