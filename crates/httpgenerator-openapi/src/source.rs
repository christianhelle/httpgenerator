use std::path::{Path, PathBuf};

use url::Url;

use crate::{OpenApiContentFormat, SourceClassificationError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenApiSource {
    Path(PathBuf),
    Url(Url),
}

impl OpenApiSource {
    pub fn is_local_path(&self) -> bool {
        matches!(self, Self::Path(_))
    }

    pub fn is_url(&self) -> bool {
        matches!(self, Self::Url(_))
    }

    pub fn format_hint(&self) -> Option<OpenApiContentFormat> {
        match self {
            Self::Path(path) => OpenApiContentFormat::from_path(path),
            Self::Url(url) => OpenApiContentFormat::from_path(Path::new(url.path())),
        }
    }
}

pub fn classify_source(input: &str) -> Result<OpenApiSource, SourceClassificationError> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Err(SourceClassificationError::EmptyInput);
    }

    if let Some(scheme) = candidate_url_scheme(trimmed) {
        let normalized_scheme = scheme.to_ascii_lowercase();

        if normalized_scheme != "http" && normalized_scheme != "https" {
            return Err(SourceClassificationError::UnsupportedUrlScheme(
                normalized_scheme,
            ));
        }

        let url = Url::parse(trimmed).map_err(|error| SourceClassificationError::InvalidUrl {
            value: trimmed.to_string(),
            reason: error.to_string(),
        })?;

        return Ok(OpenApiSource::Url(url));
    }

    Ok(OpenApiSource::Path(PathBuf::from(trimmed)))
}

fn candidate_url_scheme(input: &str) -> Option<&str> {
    let (scheme, _) = input.split_once("://")?;
    let first = scheme.chars().next()?;

    if !first.is_ascii_alphabetic() {
        return None;
    }

    scheme
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || matches!(character, '+' | '-' | '.'))
        .then_some(scheme)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{OpenApiSource, classify_source};
    use crate::{OpenApiContentFormat, SourceClassificationError};

    #[test]
    fn classifies_relative_file_paths_as_local_paths() {
        let source = classify_source("test\\OpenAPI\\v3.0\\petstore.json").unwrap();

        assert_eq!(
            source,
            OpenApiSource::Path(PathBuf::from("test\\OpenAPI\\v3.0\\petstore.json"))
        );
        assert!(source.is_local_path());
        assert_eq!(source.format_hint(), Some(OpenApiContentFormat::Json));
    }

    #[test]
    fn classifies_windows_absolute_paths_as_local_paths() {
        let source = classify_source("C:\\specs\\petstore.yaml").unwrap();

        assert_eq!(
            source,
            OpenApiSource::Path(PathBuf::from("C:\\specs\\petstore.yaml"))
        );
        assert!(source.is_local_path());
        assert_eq!(source.format_hint(), Some(OpenApiContentFormat::Yaml));
    }

    #[test]
    fn classifies_https_urls() {
        let source = classify_source("https://example.com/specs/petstore.yaml?download=1").unwrap();

        assert!(source.is_url());
        assert_eq!(source.format_hint(), Some(OpenApiContentFormat::Yaml));
        assert!(
            matches!(source, OpenApiSource::Url(url) if url.as_str() == "https://example.com/specs/petstore.yaml?download=1")
        );
    }

    #[test]
    fn rejects_unsupported_url_schemes() {
        let error = classify_source("ftp://example.com/openapi.json").unwrap_err();

        assert_eq!(
            error,
            SourceClassificationError::UnsupportedUrlScheme("ftp".to_string())
        );
    }

    #[test]
    fn rejects_invalid_http_urls() {
        let error = classify_source("https://").unwrap_err();

        assert!(matches!(
            error,
            SourceClassificationError::InvalidUrl { value, .. } if value == "https://"
        ));
    }

    #[test]
    fn rejects_empty_input() {
        let error = classify_source("   ").unwrap_err();

        assert_eq!(error, SourceClassificationError::EmptyInput);
    }
}
