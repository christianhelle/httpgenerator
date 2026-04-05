use std::fs;

use serde_json::Value;

use crate::{
    OpenApiContentFormat, OpenApiSource, OpenApiSpecificationVersion, RawOpenApiLoadError,
    SpecificationVersionDetectionError, classify_source, detect_content_format,
    detect_specification_version,
};

#[derive(Debug, Clone, PartialEq)]
pub struct RawOpenApiDocument {
    source: OpenApiSource,
    format: OpenApiContentFormat,
    content: String,
    value: Value,
}

impl RawOpenApiDocument {
    pub fn source(&self) -> &OpenApiSource {
        &self.source
    }

    pub fn format(&self) -> OpenApiContentFormat {
        self.format
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn into_value(self) -> Value {
        self.value
    }

    pub fn specification_version(
        &self,
    ) -> Result<OpenApiSpecificationVersion, SpecificationVersionDetectionError> {
        detect_specification_version(&self.value)
    }
}

pub fn load_raw_document(input: &str) -> Result<RawOpenApiDocument, RawOpenApiLoadError> {
    let source = classify_source(input).map_err(RawOpenApiLoadError::SourceClassification)?;
    load_raw_document_from_source(source)
}

pub fn load_raw_document_from_source(
    source: OpenApiSource,
) -> Result<RawOpenApiDocument, RawOpenApiLoadError> {
    let content = load_source_content(&source)?;
    decode_raw_document(source, content)
}

pub fn decode_raw_document(
    source: OpenApiSource,
    content: impl Into<String>,
) -> Result<RawOpenApiDocument, RawOpenApiLoadError> {
    let content = content.into();
    let format = detect_content_format(Some(&source), &content).map_err(|error| {
        RawOpenApiLoadError::FormatDetection {
            source: source.clone(),
            error,
        }
    })?;
    let value = decode_content(&source, format, &content)?;

    Ok(RawOpenApiDocument {
        source,
        format,
        content,
        value,
    })
}

fn load_source_content(source: &OpenApiSource) -> Result<String, RawOpenApiLoadError> {
    match source {
        OpenApiSource::Path(path) => {
            fs::read_to_string(path).map_err(|error| RawOpenApiLoadError::FileRead {
                path: path.clone(),
                reason: error.to_string(),
            })
        }
        OpenApiSource::Url(url) => {
            let response = reqwest::blocking::get(url.clone()).map_err(|error| {
                RawOpenApiLoadError::HttpRequest {
                    url: url.clone(),
                    reason: error.to_string(),
                }
            })?;

            let status = response.status();
            if !status.is_success() {
                return Err(RawOpenApiLoadError::HttpStatus {
                    url: url.clone(),
                    status,
                });
            }

            let bytes = response
                .bytes()
                .map_err(|error| RawOpenApiLoadError::HttpBodyRead {
                    url: url.clone(),
                    reason: error.to_string(),
                })?;

            String::from_utf8(bytes.to_vec()).map_err(|error| RawOpenApiLoadError::HttpBodyRead {
                url: url.clone(),
                reason: error.to_string(),
            })
        }
    }
}

fn decode_content(
    source: &OpenApiSource,
    format: OpenApiContentFormat,
    content: &str,
) -> Result<Value, RawOpenApiLoadError> {
    let decode_input = content.strip_prefix('\u{feff}').unwrap_or(content);

    match format {
        OpenApiContentFormat::Json => {
            serde_json::from_str(decode_input).map_err(|error| RawOpenApiLoadError::Decode {
                source: source.clone(),
                format,
                reason: error.to_string(),
            })
        }
        OpenApiContentFormat::Yaml => {
            yaml_serde::from_str(decode_input).map_err(|error| RawOpenApiLoadError::Decode {
                source: source.clone(),
                format,
                reason: error.to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        io::{Read, Write},
        net::{Shutdown, TcpListener},
        path::{Path, PathBuf},
        sync::atomic::{AtomicU64, Ordering},
        thread,
    };

    use serde_json::json;
    use url::Url;

    use super::load_raw_document;
    use crate::{
        ContentFormatDetectionError, OpenApiContentFormat, OpenApiSource, RawOpenApiLoadError,
    };

    static TEST_ARTIFACT_ID: AtomicU64 = AtomicU64::new(0);

    #[test]
    fn loads_local_json_documents() {
        let file = TestFile::new(
            "openapi.json",
            "\u{feff}{\"openapi\":\"3.1.0\",\"info\":{\"title\":\"Example\"}}",
        );
        let document = load_raw_document(file.path().to_str().unwrap()).unwrap();

        assert_eq!(
            document.source(),
            &OpenApiSource::Path(file.path().to_path_buf())
        );
        assert_eq!(document.format(), OpenApiContentFormat::Json);
        assert_eq!(
            document.value(),
            &json!({
                "openapi": "3.1.0",
                "info": { "title": "Example" }
            })
        );
    }

    #[test]
    fn loads_local_yaml_documents_by_sniffing_content() {
        let file = TestFile::new("openapi", "openapi: 3.0.0\ninfo:\n  title: Example\n");
        let document = load_raw_document(file.path().to_str().unwrap()).unwrap();

        assert_eq!(
            document.source(),
            &OpenApiSource::Path(file.path().to_path_buf())
        );
        assert_eq!(document.format(), OpenApiContentFormat::Yaml);
        assert_eq!(
            document.value(),
            &json!({
                "openapi": "3.0.0",
                "info": { "title": "Example" }
            })
        );
    }

    #[test]
    fn loads_http_json_documents() {
        let server = TestHttpServer::respond_once(
            "/openapi.json",
            "200 OK",
            &[("Content-Type", "application/json")],
            "{\"openapi\":\"3.0.0\",\"info\":{\"title\":\"Remote\"}}",
        );
        let url = server.url().clone();
        let document = load_raw_document(url.as_str()).unwrap();

        assert_eq!(document.source(), &OpenApiSource::Url(url));
        assert_eq!(document.format(), OpenApiContentFormat::Json);
        assert_eq!(document.value()["info"]["title"], "Remote");
    }

    #[test]
    fn returns_file_read_errors_for_missing_files() {
        let path = unique_test_path("missing-openapi.json");
        let error = load_raw_document(path.to_str().unwrap()).unwrap_err();

        assert!(matches!(
            error,
            RawOpenApiLoadError::FileRead { path: actual, reason }
                if actual == path && reason.contains("file")
        ));
    }

    #[test]
    fn returns_http_status_errors_for_unsuccessful_responses() {
        let server = TestHttpServer::respond_once(
            "/missing.json",
            "404 Not Found",
            &[("Content-Type", "text/plain")],
            "missing",
        );
        let url = server.url().clone();
        let error = load_raw_document(url.as_str()).unwrap_err();

        assert!(matches!(
            error,
            RawOpenApiLoadError::HttpStatus { url: actual, status } if actual == url && status.as_u16() == 404
        ));
    }

    #[test]
    fn returns_format_detection_errors_for_unknown_content() {
        let file = TestFile::new("openapi", "not a recognized document");
        let source = OpenApiSource::Path(file.path().to_path_buf());
        let error = load_raw_document(file.path().to_str().unwrap()).unwrap_err();

        assert_eq!(
            error,
            RawOpenApiLoadError::FormatDetection {
                source,
                error: ContentFormatDetectionError::UnknownFormat,
            }
        );
    }

    #[test]
    fn returns_decode_errors_for_invalid_json_content() {
        let file = TestFile::new("openapi.json", "{\"openapi\":");
        let source = OpenApiSource::Path(file.path().to_path_buf());
        let error = load_raw_document(file.path().to_str().unwrap()).unwrap_err();

        assert!(matches!(
            error,
            RawOpenApiLoadError::Decode { source: actual, format, .. }
                if actual == source && format == OpenApiContentFormat::Json
        ));
    }

    fn unique_test_path(file_name: &str) -> PathBuf {
        let artifact_id = TEST_ARTIFACT_ID.fetch_add(1, Ordering::Relaxed);
        let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("test-data");

        fs::create_dir_all(&directory).unwrap();

        directory.join(format!(
            "{}-{}-{file_name}",
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

    struct TestHttpServer {
        url: Url,
        handle: Option<thread::JoinHandle<()>>,
    }

    impl TestHttpServer {
        fn respond_once(
            path: &str,
            status_line: &str,
            headers: &[(&str, &str)],
            body: &str,
        ) -> Self {
            let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
            let address = listener.local_addr().unwrap();
            let body = body.to_string();
            let status_line = status_line.to_string();
            let header_block = headers
                .iter()
                .map(|(name, value)| format!("{name}: {value}\r\n"))
                .collect::<String>();
            let handle = thread::spawn(move || {
                let (mut stream, _) = listener.accept().unwrap();
                let mut request = [0_u8; 4096];
                let _ = stream.read(&mut request);
                let response = format!(
                    "HTTP/1.1 {status_line}\r\nContent-Length: {}\r\nConnection: close\r\n{header_block}\r\n{body}",
                    body.as_bytes().len()
                );

                stream.write_all(response.as_bytes()).unwrap();
                stream.flush().unwrap();
                let _ = stream.shutdown(Shutdown::Both);
            });
            let url = Url::parse(&format!("http://{address}{path}")).unwrap();

            Self {
                url,
                handle: Some(handle),
            }
        }

        fn url(&self) -> &Url {
            &self.url
        }
    }

    impl Drop for TestHttpServer {
        fn drop(&mut self) {
            if let Some(handle) = self.handle.take() {
                handle.join().unwrap();
            }
        }
    }
}
