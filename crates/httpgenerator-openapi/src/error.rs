use std::{error::Error, fmt};

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
