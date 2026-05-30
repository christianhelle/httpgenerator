use httpgenerator_cli::{AzureAuthStatus, CliError, ExecutionObserver};
use httpgenerator_core::openapi::OpenApiInspection;
use std::{
    io::{self, IsTerminal, Write as IoWrite},
    path::PathBuf,
    time::Duration,
};

#[path = "render.rs"]
mod render;

use self::render::{
    render_azure_auth_finished, render_azure_auth_started, render_error,
    render_file_writing_started, render_files_written, render_header, render_success,
    render_validation_started, render_validation_succeeded,
};
use super::format::{mode_from_terminal, PresentationMode};

pub(crate) struct CliPresenter {
    stdout_mode: PresentationMode,
    stderr_mode: PresentationMode,
}

impl CliPresenter {
    pub(crate) fn detect() -> Self {
        Self::new(
            mode_from_terminal(io::stdout().is_terminal()),
            mode_from_terminal(io::stderr().is_terminal()),
        )
    }

    fn new(stdout_mode: PresentationMode, stderr_mode: PresentationMode) -> Self {
        Self {
            stdout_mode,
            stderr_mode,
        }
    }

    pub(crate) fn print_header(&mut self, no_logging: bool) {
        self.write_stdout(&render_header(self.stdout_mode, no_logging));
    }

    pub(crate) fn print_success(&mut self, duration: Duration) {
        self.write_stdout(&render_success(self.stdout_mode, duration));
    }

    pub(crate) fn print_error(&mut self, error: &CliError) {
        self.write_stderr(&render_error(self.stderr_mode, error));
    }

    fn write_stdout(&self, output: &str) {
        if output.is_empty() {
            return;
        }

        print!("{output}");
        let _ = io::stdout().flush();
    }

    fn write_stderr(&self, output: &str) {
        if output.is_empty() {
            return;
        }

        eprint!("{output}");
        let _ = io::stderr().flush();
    }
}

impl ExecutionObserver for CliPresenter {
    fn validation_started(&mut self) {
        self.write_stdout(&render_validation_started(self.stdout_mode));
    }

    fn validation_succeeded(&mut self, inspection: &OpenApiInspection) {
        self.write_stdout(&render_validation_succeeded(self.stdout_mode, inspection));
    }

    fn azure_auth_started(&mut self) {
        self.write_stdout(&render_azure_auth_started(self.stdout_mode));
    }

    fn azure_auth_finished(&mut self, status: &AzureAuthStatus) {
        match status {
            AzureAuthStatus::Failed { .. } => self.write_stderr(&render_azure_auth_finished(
                self.stdout_mode,
                self.stderr_mode,
                status,
            )),
            _ => self.write_stdout(&render_azure_auth_finished(
                self.stdout_mode,
                self.stderr_mode,
                status,
            )),
        }
    }

    fn file_writing_started(&mut self, file_count: usize) {
        self.write_stdout(&render_file_writing_started(self.stdout_mode, file_count));
    }

    fn files_written(&mut self, paths: &[PathBuf]) {
        self.write_stdout(&render_files_written(self.stdout_mode, paths));
    }
}

#[cfg(test)]
mod tests {
    use crate::ui::format::PresentationMode;

    use super::{render::render_plain_header, CliPresenter};

    #[test]
    fn plain_header_stays_semantic_without_rich_markers() {
        let header = render_plain_header(true);

        assert!(header.contains("HTTP File Generator v"));
        assert!(header.contains("Support key: Unavailable when logging is disabled"));
        assert!(!header.contains("╭"));
        assert!(!header.contains("🚀"));
    }

    #[test]
    fn rich_header_uses_a_panel_layout() {
        let presenter = CliPresenter::new(PresentationMode::Rich, PresentationMode::Rich);
        let mut header = String::new();
        header.push_str(&super::render_header(presenter.stdout_mode, true));

        assert!(header.contains("╭"));
        assert!(header.contains("🚀 HTTP File Generator"));
        assert!(header.contains("⚠️  Unavailable when logging is disabled"));
    }
}
