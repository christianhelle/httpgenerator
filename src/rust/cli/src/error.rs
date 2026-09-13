use std::{error::Error, fmt, path::PathBuf};

use httpgenerator_core::openapi::OpenApiSpecificationVersion;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliError {
    MissingInput,
    InspectOpenApi(String),
    LoadOpenApi(String),
    UnsupportedValidationVersion {
        version: OpenApiSpecificationVersion,
    },
    UnresolvedReferences {
        references: Vec<String>,
    },
    CreateOutputDirectory {
        path: PathBuf,
        reason: String,
    },
    WriteFiles {
        path: PathBuf,
        reason: String,
    },
    WriteTimeout {
        seconds: u64,
    },
    WriteChannelClosed,
}

impl fmt::Display for CliError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingInput => write!(
                formatter,
                "missing OpenAPI input path or URL; run with --help for usage"
            ),
            Self::InspectOpenApi(reason) => write!(formatter, "{reason}"),
            Self::LoadOpenApi(reason) => write!(formatter, "{reason}"),
            Self::UnsupportedValidationVersion { version } => write!(
                formatter,
                "{version} documents are not supported by CLI validation yet; retry with --skip-validation"
            ),
            Self::UnresolvedReferences { references } => write!(
                formatter,
                "OpenAPI validation failed because external references could not be resolved; retry with --skip-validation to generate anyway:\n{}",
                references.join("\n")
            ),
            Self::CreateOutputDirectory { path, reason } => write!(
                formatter,
                "failed to create output directory '{}': {reason}",
                path.display()
            ),
            Self::WriteFiles { path, reason } => write!(
                formatter,
                "failed to write generated file '{}': {reason}",
                path.display()
            ),
            Self::WriteTimeout { seconds } => write!(
                formatter,
                "timed out after {seconds} second(s) while writing generated files"
            ),
            Self::WriteChannelClosed => write!(
                formatter,
                "file writing worker stopped before reporting a result"
            ),
        }
    }
}

impl Error for CliError {}

impl CliError {
    pub const fn telemetry_name(&self) -> &'static str {
        match self {
            Self::MissingInput => "MissingInput",
            Self::InspectOpenApi(_) => "InspectOpenApi",
            Self::LoadOpenApi(_) => "LoadOpenApi",
            Self::UnsupportedValidationVersion { .. } => "UnsupportedValidationVersion",
            Self::UnresolvedReferences { .. } => "UnresolvedReferences",
            Self::CreateOutputDirectory { .. } => "CreateOutputDirectory",
            Self::WriteFiles { .. } => "WriteFiles",
            Self::WriteTimeout { .. } => "WriteTimeout",
            Self::WriteChannelClosed => "WriteChannelClosed",
        }
    }
}
