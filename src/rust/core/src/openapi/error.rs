//! Error types for OpenAPI loading, inspection, and normalization.
//!
//! Each enum maps to one stage of the public ingestion pipeline so callers can match on the exact
//! failure boundary they care about without losing source or version context.

use std::{error::Error, fmt, path::PathBuf};

use reqwest::StatusCode;
use url::Url;

use crate::NormalizedHttpMethod;

use super::{OpenApiContentFormat, OpenApiSource, OpenApiSpecificationVersion};

/// Errors returned while classifying a CLI input as a path or URL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceClassificationError {
    /// The supplied input was empty or whitespace-only.
    EmptyInput,
    /// The input looked like a URL, but the scheme is not supported.
    UnsupportedUrlScheme(String),
    /// The input used an HTTP(S) scheme but did not parse as a valid URL.
    InvalidUrl { value: String, reason: String },
}

impl fmt::Display for SourceClassificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "OpenAPI source input cannot be empty"),
            Self::UnsupportedUrlScheme(scheme) => {
                write!(f, "unsupported OpenAPI source URL scheme '{scheme}'")
            }
            Self::InvalidUrl { value, reason } => {
                write!(f, "invalid OpenAPI source URL '{value}': {reason}")
            }
        }
    }
}

impl Error for SourceClassificationError {}

/// Errors returned while detecting whether raw content is JSON or YAML.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentFormatDetectionError {
    /// The supplied content was empty or whitespace-only.
    EmptyContent,
    /// The content did not look like supported JSON or YAML.
    UnknownFormat,
}

impl fmt::Display for ContentFormatDetectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyContent => write!(f, "OpenAPI content cannot be empty"),
            Self::UnknownFormat => write!(f, "unable to detect OpenAPI content format"),
        }
    }
}

impl Error for ContentFormatDetectionError {}

/// Errors returned while loading or decoding a raw OpenAPI document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawOpenApiLoadError {
    /// Source classification failed before any I/O started.
    SourceClassification(SourceClassificationError),
    /// Reading a local file failed.
    FileRead {
        path: PathBuf,
        reason: String,
    },
    /// The initial HTTP request failed.
    HttpRequest {
        url: Url,
        reason: String,
    },
    /// The remote server returned a non-success HTTP status.
    HttpStatus {
        url: Url,
        status: StatusCode,
    },
    /// Reading or decoding the HTTP response body failed.
    HttpBodyRead {
        url: Url,
        reason: String,
    },
    /// Detecting the raw content format failed.
    FormatDetection {
        source: OpenApiSource,
        error: ContentFormatDetectionError,
    },
    /// Decoding JSON or YAML into a generic value failed.
    Decode {
        source: OpenApiSource,
        format: OpenApiContentFormat,
        reason: String,
    },
}

impl fmt::Display for RawOpenApiLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SourceClassification(error) => {
                write!(f, "failed to classify OpenAPI source: {error}")
            }
            Self::FileRead { path, reason } => {
                write!(
                    f,
                    "failed to read OpenAPI file '{}': {reason}",
                    path.display()
                )
            }
            Self::HttpRequest { url, reason } => {
                write!(f, "failed to fetch OpenAPI URL '{url}': {reason}")
            }
            Self::HttpStatus { url, status } => {
                write!(f, "OpenAPI URL '{url}' returned HTTP {status}")
            }
            Self::HttpBodyRead { url, reason } => {
                write!(
                    f,
                    "failed to read OpenAPI response body from '{url}': {reason}"
                )
            }
            Self::FormatDetection { source, error } => {
                write!(
                    f,
                    "failed to detect OpenAPI content format for '{source}': {error}"
                )
            }
            Self::Decode {
                source,
                format,
                reason,
            } => {
                write!(
                    f,
                    "failed to decode {format} OpenAPI document from '{source}': {reason}"
                )
            }
        }
    }
}

impl Error for RawOpenApiLoadError {}

/// Errors returned while detecting the top-level OpenAPI version field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecificationVersionDetectionError {
    /// Neither `openapi` nor `swagger` was present at the top level.
    MissingVersionField,
    /// The version field existed but was not a non-empty string.
    InvalidVersionFieldType { field: &'static str },
    /// The version field was present but outside the supported families.
    UnsupportedVersion { field: &'static str, value: String },
}

impl fmt::Display for SpecificationVersionDetectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingVersionField => {
                write!(
                    f,
                    "OpenAPI document is missing a top-level 'openapi' or 'swagger' version field"
                )
            }
            Self::InvalidVersionFieldType { field } => {
                write!(f, "OpenAPI document field '{field}' must be a string")
            }
            Self::UnsupportedVersion { field, value } => {
                write!(
                    f,
                    "unsupported OpenAPI version '{value}' in field '{field}'"
                )
            }
        }
    }
}

impl Error for SpecificationVersionDetectionError {}

/// Errors returned by the inspection helpers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenApiInspectionError {
    /// Loading the raw document failed.
    Load(RawOpenApiLoadError),
    /// Detecting the specification version failed.
    VersionDetection(SpecificationVersionDetectionError),
}

impl fmt::Display for OpenApiInspectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Load(error) => write!(f, "{error}"),
            Self::VersionDetection(error) => write!(f, "{error}"),
        }
    }
}

impl Error for OpenApiInspectionError {}

/// Errors returned while converting a raw document into a typed OpenAPI model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypedOpenApiParseError {
    /// Version detection failed before typed parsing could start.
    VersionDetection {
        source: OpenApiSource,
        error: SpecificationVersionDetectionError,
    },
    /// The crate does not expose a typed parser for the detected version.
    UnsupportedVersion {
        source: OpenApiSource,
        version: OpenApiSpecificationVersion,
    },
    /// Deserializing into the version-specific Rust model failed.
    Deserialize {
        source: OpenApiSource,
        version: OpenApiSpecificationVersion,
        reason: String,
    },
}

impl fmt::Display for TypedOpenApiParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VersionDetection { source, error } => {
                write!(
                    f,
                    "failed to detect OpenAPI specification version for '{source}': {error}"
                )
            }
            Self::UnsupportedVersion { source, version } => {
                write!(
                    f,
                    "typed OpenAPI parsing is not implemented for {version} documents from '{source}'"
                )
            }
            Self::Deserialize {
                source,
                version,
                reason,
            } => {
                write!(
                    f,
                    "failed to deserialize {version} document from '{source}': {reason}"
                )
            }
        }
    }
}

impl Error for TypedOpenApiParseError {}

/// Errors returned by the typed document loaders.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenApiDocumentLoadError {
    /// Loading the raw document failed.
    RawLoad(RawOpenApiLoadError),
    /// Parsing the typed document failed.
    TypedParse(TypedOpenApiParseError),
}

impl fmt::Display for OpenApiDocumentLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RawLoad(error) => write!(f, "{error}"),
            Self::TypedParse(error) => write!(f, "{error}"),
        }
    }
}

impl Error for OpenApiDocumentLoadError {}

/// Errors returned while normalizing a loaded OpenAPI document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenApiNormalizationError {
    /// The raw document tree did not match the structure expected by the normalizer.
    InvalidStructure {
        path: String,
        context: String,
    },
    /// A path item used a `$ref` form that the normalizer does not yet support.
    UnsupportedPathItemReference {
        path: String,
        reference: String,
    },
    /// An operation parameter used a `$ref` form that the normalizer does not yet support.
    UnsupportedParameterReference {
        path: String,
        method: NormalizedHttpMethod,
        reference: String,
    },
    /// An operation request body used a `$ref` form that the normalizer does not yet support.
    UnsupportedRequestBodyReference {
        path: String,
        method: NormalizedHttpMethod,
        reference: String,
    },
}

impl fmt::Display for OpenApiNormalizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidStructure { path, context } => {
                write!(
                    f,
                    "OpenAPI document contains an unexpected structure at '{path}' ({context})"
                )
            }
            Self::UnsupportedPathItemReference { path, reference } => {
                write!(
                    f,
                    "path item '{path}' uses unsupported $ref '{reference}' during normalization"
                )
            }
            Self::UnsupportedParameterReference {
                path,
                method,
                reference,
            } => {
                write!(
                    f,
                    "{method:?} operation '{path}' uses unsupported parameter $ref '{reference}' during normalization"
                )
            }
            Self::UnsupportedRequestBodyReference {
                path,
                method,
                reference,
            } => {
                write!(
                    f,
                    "{method:?} operation '{path}' uses unsupported requestBody $ref '{reference}' during normalization"
                )
            }
        }
    }
}

impl Error for OpenApiNormalizationError {}

/// Errors returned by the end-to-end normalize-from-source helpers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenApiDocumentNormalizationError {
    /// Loading or typed parsing failed.
    Load(OpenApiDocumentLoadError),
    /// Normalization of the loaded document failed.
    Normalize(OpenApiNormalizationError),
}

impl fmt::Display for OpenApiDocumentNormalizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Load(error) => write!(f, "{error}"),
            Self::Normalize(error) => write!(f, "{error}"),
        }
    }
}

impl Error for OpenApiDocumentNormalizationError {}
