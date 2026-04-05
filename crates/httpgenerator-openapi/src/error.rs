use std::{error::Error, fmt, path::PathBuf};

use reqwest::StatusCode;
use url::Url;

use crate::{OpenApiContentFormat, OpenApiSource, OpenApiSpecificationVersion};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceClassificationError {
    EmptyInput,
    UnsupportedUrlScheme(String),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentFormatDetectionError {
    EmptyContent,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawOpenApiLoadError {
    SourceClassification(SourceClassificationError),
    FileRead {
        path: PathBuf,
        reason: String,
    },
    HttpRequest {
        url: Url,
        reason: String,
    },
    HttpStatus {
        url: Url,
        status: StatusCode,
    },
    HttpBodyRead {
        url: Url,
        reason: String,
    },
    FormatDetection {
        source: OpenApiSource,
        error: ContentFormatDetectionError,
    },
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecificationVersionDetectionError {
    MissingVersionField,
    InvalidVersionFieldType { field: &'static str },
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypedOpenApiParseError {
    VersionDetection {
        source: OpenApiSource,
        error: SpecificationVersionDetectionError,
    },
    UnsupportedVersion {
        source: OpenApiSource,
        version: OpenApiSpecificationVersion,
    },
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
