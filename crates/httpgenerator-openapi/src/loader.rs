use crate::{
    OpenApiContentFormat, OpenApiDocumentLoadError, OpenApiSource, OpenApiSpecificationVersion,
    RawOpenApiDocument, TypedOpenApiDocument, TypedOpenApiParseError, load_raw_document,
    load_raw_document_from_source, parse_typed_document,
};

pub enum LoadedOpenApiDocument {
    OpenApi30 {
        raw: RawOpenApiDocument,
        document: openapiv3::OpenAPI,
    },
    OpenApi31 {
        raw: RawOpenApiDocument,
        document: openapiv3_1::OpenApi,
    },
    OpenApi31WebhookOnly {
        raw: RawOpenApiDocument,
    },
}

impl LoadedOpenApiDocument {
    pub fn raw(&self) -> &RawOpenApiDocument {
        match self {
            Self::OpenApi30 { raw, .. }
            | Self::OpenApi31 { raw, .. }
            | Self::OpenApi31WebhookOnly { raw } => raw,
        }
    }

    pub fn source(&self) -> &OpenApiSource {
        self.raw().source()
    }

    pub fn format(&self) -> OpenApiContentFormat {
        self.raw().format()
    }

    pub fn specification_version(&self) -> OpenApiSpecificationVersion {
        match self {
            Self::OpenApi30 { .. } => OpenApiSpecificationVersion::OpenApi30,
            Self::OpenApi31 { .. } | Self::OpenApi31WebhookOnly { .. } => {
                OpenApiSpecificationVersion::OpenApi31
            }
        }
    }

    pub fn as_openapi30(&self) -> Option<&openapiv3::OpenAPI> {
        match self {
            Self::OpenApi30 { document, .. } => Some(document),
            Self::OpenApi31 { .. } | Self::OpenApi31WebhookOnly { .. } => None,
        }
    }

    pub fn as_openapi31(&self) -> Option<&openapiv3_1::OpenApi> {
        match self {
            Self::OpenApi30 { .. } | Self::OpenApi31WebhookOnly { .. } => None,
            Self::OpenApi31 { document, .. } => Some(document),
        }
    }
}

pub fn load_document(input: &str) -> Result<LoadedOpenApiDocument, OpenApiDocumentLoadError> {
    let raw = load_raw_document(input).map_err(OpenApiDocumentLoadError::RawLoad)?;
    load_document_from_raw(raw)
}

pub fn load_document_from_source(
    source: OpenApiSource,
) -> Result<LoadedOpenApiDocument, OpenApiDocumentLoadError> {
    let raw = load_raw_document_from_source(source).map_err(OpenApiDocumentLoadError::RawLoad)?;
    load_document_from_raw(raw)
}

pub fn load_document_from_raw(
    raw: RawOpenApiDocument,
) -> Result<LoadedOpenApiDocument, OpenApiDocumentLoadError> {
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
        }) if is_webhook_only_openapi31_document(&raw) => {
            Ok(LoadedOpenApiDocument::OpenApi31WebhookOnly { raw })
        }
        Err(error) => Err(OpenApiDocumentLoadError::TypedParse(error)),
    }
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

    use crate::{
        OpenApiSource, OpenApiSpecificationVersion, TypedOpenApiParseError, decode_raw_document,
    };

    use super::{
        LoadedOpenApiDocument, load_document, load_document_from_raw, load_document_from_source,
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

        let loaded = load_document_from_raw(raw).unwrap();

        assert!(matches!(loaded, LoadedOpenApiDocument::OpenApi30 { .. }));
        assert_eq!(
            loaded.specification_version(),
            OpenApiSpecificationVersion::OpenApi30
        );
        assert!(loaded.as_openapi30().is_some());
        assert!(loaded.as_openapi31().is_none());
    }

    #[test]
    fn loads_openapi_thirty_one_documents_from_a_source() {
        let file = TestFile::new(
            "openapi.yaml",
            "openapi: 3.1.0\ninfo:\n  title: Example\n  version: 1.0.0\npaths: {}\n",
        );

        let loaded =
            load_document_from_source(OpenApiSource::Path(file.path().to_path_buf())).unwrap();

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
            include_str!("../../../test/OpenAPI/v3.1/webhook-example.json"),
        )
        .unwrap();

        let loaded = load_document_from_raw(raw).unwrap();

        assert!(matches!(
            loaded,
            LoadedOpenApiDocument::OpenApi31WebhookOnly { .. }
        ));
        assert_eq!(
            loaded.specification_version(),
            OpenApiSpecificationVersion::OpenApi31
        );
        assert!(loaded.as_openapi31().is_none());
    }

    #[test]
    fn wraps_swagger_two_documents_as_typed_parse_errors() {
        let raw = decode_raw_document(
            OpenApiSource::Path(PathBuf::from("swagger.json")),
            r#"{
                "swagger": "2.0",
                "info": { "title": "Example", "version": "1.0.0" },
                "paths": {}
            }"#,
        )
        .unwrap();

        match load_document_from_raw(raw) {
            Err(crate::OpenApiDocumentLoadError::TypedParse(error)) => {
                assert_eq!(
                    error,
                    TypedOpenApiParseError::UnsupportedVersion {
                        source: OpenApiSource::Path(PathBuf::from("swagger.json")),
                        version: OpenApiSpecificationVersion::Swagger2,
                    }
                );
            }
            Err(error) => panic!("unexpected loader error: {error}"),
            Ok(_) => panic!("expected Swagger 2 parsing to remain unsupported"),
        }
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

        let loaded = load_document(file.path().to_str().unwrap()).unwrap();

        assert!(matches!(loaded, LoadedOpenApiDocument::OpenApi30 { .. }));
        assert_eq!(loaded.format(), crate::OpenApiContentFormat::Json);
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
