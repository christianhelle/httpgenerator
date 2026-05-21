use std::{error::Error, fmt, path::PathBuf};

use reqwest::StatusCode;
use url::Url;

use crate::NormalizedHttpMethod;

use super::{OpenApiContentFormat, OpenApiSource, OpenApiSpecificationVersion};

/// Error returned when source input cannot be classified as a supported path or URL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceClassificationError {
    /// The supplied source string is empty or whitespace.
    EmptyInput,
    /// The source looked like a URL but used an unsupported scheme.
    UnsupportedUrlScheme(String),
    /// The source looked like an HTTP(S) URL but could not be parsed.
    InvalidUrl {
        /// Original source value.
        value: String,
        /// Parser error text.
        reason: String,
    },
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

/// Error returned when JSON/YAML content format cannot be detected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentFormatDetectionError {
    /// The content is empty after trimming whitespace and a UTF-8 byte-order mark.
    EmptyContent,
    /// The content does not look like JSON or YAML.
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

/// Error returned while loading a raw OpenAPI document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawOpenApiLoadError {
    /// The input path or URL could not be classified.
    SourceClassification(SourceClassificationError),
    /// A local file could not be read.
    FileRead {
        /// File path that failed.
        path: PathBuf,
        /// I/O error text.
        reason: String,
    },
    /// A remote HTTP request could not be completed.
    HttpRequest {
        /// Requested URL.
        url: Url,
        /// Request error text.
        reason: String,
    },
    /// A remote HTTP request completed with a non-success status.
    HttpStatus {
        /// Requested URL.
        url: Url,
        /// Returned HTTP status code.
        status: StatusCode,
    },
    /// The response body could not be read or converted to UTF-8.
    HttpBodyRead {
        /// Requested URL.
        url: Url,
        /// Body read or decoding error text.
        reason: String,
    },
    /// JSON/YAML format detection failed.
    FormatDetection {
        /// Source being decoded.
        source: OpenApiSource,
        /// Format detection failure.
        error: ContentFormatDetectionError,
    },
    /// JSON or YAML content could not be decoded.
    Decode {
        /// Source being decoded.
        source: OpenApiSource,
        /// Format selected for decoding.
        format: OpenApiContentFormat,
        /// Decoder error text.
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

/// Error returned when an OpenAPI specification version cannot be detected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecificationVersionDetectionError {
    /// The document has no top-level `openapi` or `swagger` field.
    MissingVersionField,
    /// The version field exists but is not a non-empty string.
    InvalidVersionFieldType {
        /// Name of the invalid version field.
        field: &'static str,
    },
    /// The version field contains a version family this crate does not support.
    UnsupportedVersion {
        /// Name of the version field.
        field: &'static str,
        /// Unsupported version value.
        value: String,
    },
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

/// Error returned while inspecting an OpenAPI document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenApiInspectionError {
    /// Raw document loading failed.
    Load(RawOpenApiLoadError),
    /// Version detection failed.
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

/// Error returned while parsing a raw document into a typed OpenAPI model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypedOpenApiParseError {
    /// The document's specification version could not be detected.
    VersionDetection {
        /// Source being parsed.
        source: OpenApiSource,
        /// Version detection failure.
        error: SpecificationVersionDetectionError,
    },
    /// The detected version is not supported by the typed parser.
    UnsupportedVersion {
        /// Source being parsed.
        source: OpenApiSource,
        /// Detected unsupported version.
        version: OpenApiSpecificationVersion,
    },
    /// Deserialization into the version-specific Rust model failed.
    Deserialize {
        /// Source being parsed.
        source: OpenApiSource,
        /// Version-specific model selected for parsing.
        version: OpenApiSpecificationVersion,
        /// Deserialization error text.
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

/// Error returned while loading a typed or partially typed OpenAPI document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenApiDocumentLoadError {
    /// Raw document loading failed.
    RawLoad(RawOpenApiLoadError),
    /// Typed parsing failed.
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

/// Error returned while converting a loaded OpenAPI document into the normalized model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenApiNormalizationError {
    /// The source document shape was not compatible with the normalizer.
    InvalidStructure {
        /// JSON pointer-like path to the unexpected value.
        path: String,
        /// Additional context about the expected structure.
        context: String,
    },
    /// A path item `$ref` was encountered where inline path items are required.
    UnsupportedPathItemReference {
        /// OpenAPI path containing the reference.
        path: String,
        /// Referenced path item.
        reference: String,
    },
    /// A parameter `$ref` was encountered where inline parameters are required.
    UnsupportedParameterReference {
        /// OpenAPI path containing the parameter.
        path: String,
        /// Operation method containing the parameter.
        method: NormalizedHttpMethod,
        /// Referenced parameter.
        reference: String,
    },
    /// A request body `$ref` was encountered where inline request bodies are required.
    UnsupportedRequestBodyReference {
        /// OpenAPI path containing the request body.
        path: String,
        /// Operation method containing the request body.
        method: NormalizedHttpMethod,
        /// Referenced request body.
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

/// Error returned while loading and normalizing an OpenAPI document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenApiDocumentNormalizationError {
    /// Loading or typed parsing failed.
    Load(OpenApiDocumentLoadError),
    /// Normalization failed after the document was loaded.
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
