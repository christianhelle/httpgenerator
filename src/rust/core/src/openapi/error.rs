//! Error types for OpenAPI loading, inspection, and normalization.
//!
//! Each enum maps to one stage of the public ingestion pipeline so callers can match on the exact
//! failure boundary they care about without losing source or version context.

use std::{error::Error, fmt};

use crate::NormalizedHttpMethod;

use super::{RawOpenApiLoadError, ReadError, SpecificationVersionDetectionError, TypedOpenApiParseError};

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

/// Errors returned while normalizing a loaded OpenAPI document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenApiNormalizationError {
    /// The raw document tree did not match the structure expected by the normalizer.
    InvalidStructure { path: String, context: String },
    /// A path item used a `$ref` form that the normalizer does not yet support.
    UnsupportedPathItemReference { path: String, reference: String },
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
    /// Reading the document or merging its external references failed.
    Read(Box<ReadError>),
    /// The document does not fit the typed model for its specification version.
    TypedParse(Box<TypedOpenApiParseError>),
    /// Normalization of the loaded document failed.
    Normalize(Box<OpenApiNormalizationError>),
}

impl fmt::Display for OpenApiDocumentNormalizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read(error) => write!(f, "{error}"),
            Self::TypedParse(error) => write!(f, "{error}"),
            Self::Normalize(error) => write!(f, "{error}"),
        }
    }
}

impl Error for OpenApiDocumentNormalizationError {}
