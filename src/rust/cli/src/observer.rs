use std::path::PathBuf;

use httpgenerator_core::openapi::{OpenApiSpecificationVersion, OpenApiStats};

/// The specification version and stats reported when validation succeeds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenApiInspection {
    pub specification_version: OpenApiSpecificationVersion,
    pub stats: OpenApiStats,
}

pub trait ExecutionObserver {
    fn validation_started(&mut self) {}

    fn validation_succeeded(&mut self, _inspection: &OpenApiInspection) {}

    /// Called with problems found while merging external references that did not stop the run.
    fn reference_warnings(&mut self, _warnings: &[String]) {}

    fn azure_auth_started(&mut self) {}

    fn azure_auth_finished(&mut self, _status: &AzureAuthStatus) {}

    fn file_writing_started(&mut self, _file_count: usize) {}

    fn files_written(&mut self, _paths: &[PathBuf]) {}
}

#[derive(Default)]
pub(crate) struct NoopExecutionObserver;

impl ExecutionObserver for NoopExecutionObserver {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionSummary {
    pub output_folder: PathBuf,
    pub files: Vec<PathBuf>,
    pub validation: Option<OpenApiInspection>,
    pub azure_auth: AzureAuthStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AzureAuthStatus {
    NotRequested,
    Acquired,
    Failed { reason: String },
}
