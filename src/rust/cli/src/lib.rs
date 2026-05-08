pub mod args;
mod auth;
mod error;
mod execution;
mod observer;
pub mod telemetry;
mod writer;

pub use error::CliError;
pub use execution::{execute, execute_with_observer, should_attempt_azure_auth};
pub use observer::{AzureAuthStatus, ExecutionObserver, ExecutionSummary};
pub use telemetry::{NoopTelemetrySink, TelemetryRecorder};

#[cfg(test)]
mod args_tests;
