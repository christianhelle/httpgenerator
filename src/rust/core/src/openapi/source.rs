//! OpenAPI source classification for local paths and HTTP(S) URLs.
//!
//! These helpers turn the original CLI input into a structured source value that later stages can
//! use for file reads, HTTP fetches, and extension-based format hints.

use std::{
    fmt,
    path::{Path, PathBuf},
};

use url::Url;

use super::{OpenApiContentFormat, SourceClassificationError};

/// A classified OpenAPI source location.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenApiSource {
    /// A local filesystem path.
    Path(PathBuf),
    /// A remote HTTP or HTTPS URL.
    Url(Url),
}

impl OpenApiSource {
    /// Returns `true` when the source is a local filesystem path.
    pub fn is_local_path(&self) -> bool {
        matches!(self, Self::Path(_))
    }

    /// Returns `true` when the source is a remote URL.
    pub fn is_url(&self) -> bool {
        matches!(self, Self::Url(_))
    }

    /// Returns a best-effort format hint derived from the path or URL extension.
    pub fn format_hint(&self) -> Option<OpenApiContentFormat> {
        match self {
            Self::Path(path) => OpenApiContentFormat::from_path(path),
            Self::Url(url) => OpenApiContentFormat::from_path(Path::new(url.path())),
        }
    }
}

impl fmt::Display for OpenApiSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Path(path) => write!(f, "{}", path.display()),
            Self::Url(url) => write!(f, "{url}"),
        }
    }
}

/// Classifies a CLI-style OpenAPI input as either a local path or an HTTP(S) URL.
///
/// Inputs that contain `://` are only treated as URLs when the prefix is a valid URL scheme, which
/// keeps Windows paths such as `C:\specs\petstore.yaml` on the local-path branch.
///
/// # Examples
///
/// ```
/// use httpgenerator_core::openapi::{OpenApiContentFormat, OpenApiSource, classify_source};
/// use std::path::PathBuf;
///
/// let path_source = classify_source("test/OpenAPI/v3.0/petstore.json").unwrap();
/// assert_eq!(
///     path_source,
///     OpenApiSource::Path(PathBuf::from("test/OpenAPI/v3.0/petstore.json"))
/// );
/// assert_eq!(path_source.format_hint(), Some(OpenApiContentFormat::Json));
///
/// let url_source = classify_source("https://example.com/openapi.yaml").unwrap();
/// assert!(url_source.is_url());
/// ```
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
    use crate::openapi::{OpenApiContentFormat, SourceClassificationError};

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
