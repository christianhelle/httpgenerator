use std::path::PathBuf;

use httpgenerator_core::openapi::OpenApiInspection;

pub trait ExecutionObserver {
    fn validation_started(&mut self) {}

    fn validation_succeeded(&mut self, _inspection: &OpenApiInspection) {}

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
