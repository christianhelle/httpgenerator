use httpgenerator_cli::{AzureAuthStatus, CliError, ExecutionObserver};
use httpgenerator_core::support_key;
use httpgenerator_openapi::{OpenApiInspection, OpenApiSpecificationVersion, OpenApiStats};
use std::{
    env,
    io::{self, IsTerminal, Write as IoWrite},
    time::Duration,
};
use unicode_width::UnicodeWidthStr;

const DEFAULT_RULE_WIDTH: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PresentationMode {
    Rich,
    Plain,
}

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
        self.write_stdout(&self.render_header(no_logging));
    }

    pub(crate) fn print_success(&mut self, duration: Duration) {
        self.write_stdout(&self.render_success(duration));
    }

    pub(crate) fn print_error(&mut self, error: &CliError) {
        self.write_stderr(&self.render_error(error));
    }

    fn render_header(&self, no_logging: bool) -> String {
        match self.stdout_mode {
            PresentationMode::Rich => render_rich_header(no_logging),
            PresentationMode::Plain => render_plain_header(no_logging),
        }
    }

    fn render_validation_started(&self) -> String {
        match self.stdout_mode {
            PresentationMode::Rich => format!(
                "{}\n",
                style("🔍 Validating OpenAPI specification...", &["36"])
            ),
            PresentationMode::Plain => "Validating OpenAPI specification...\n".to_string(),
        }
    }

    fn render_validation_succeeded(&self, inspection: &OpenApiInspection) -> String {
        match self.stdout_mode {
            PresentationMode::Rich => {
                let mut output = String::new();
                output.push_str(&format!(
                    "{}\n",
                    style("✅ OpenAPI specification validated successfully", &["32"])
                ));
                output.push_str(&render_stats_table(&inspection.stats));
                output.push_str("\n\n");
                output
            }
            PresentationMode::Plain => {
                let mut output = String::new();
                output.push_str(&format!(
                    "Validated {} specification successfully\n",
                    inspection.specification_version
                ));
                output.push_str(&render_plain_stats(&inspection.stats));
                output.push('\n');
                output.push('\n');
                output
            }
        }
    }

    fn render_azure_auth_started(&self) -> String {
        match self.stdout_mode {
            PresentationMode::Rich => format!(
                "{}\n",
                style(
                    "🔐 Acquiring authorization header from Azure Entra ID...",
                    &["36"]
                )
            ),
            PresentationMode::Plain => {
                "Acquiring authorization header from Azure Entra ID...\n".to_string()
            }
        }
    }

    fn render_azure_auth_finished(&self, status: &AzureAuthStatus) -> String {
        match status {
            AzureAuthStatus::NotRequested => String::new(),
            AzureAuthStatus::Acquired => match self.stdout_mode {
                PresentationMode::Rich => format!(
                    "{}\n\n",
                    style("✅ Successfully acquired access token", &["32"])
                ),
                PresentationMode::Plain => {
                    "Successfully acquired access token from Azure Entra ID\n\n".to_string()
                }
            },
            AzureAuthStatus::Failed { reason } => match self.stderr_mode {
                PresentationMode::Rich => {
                    format!("{} {}\n", style("Error:", &["31"]), style(reason, &["31"]))
                }
                PresentationMode::Plain => format!("Error: {reason}\n"),
            },
        }
    }

    fn render_file_writing_started(&self, file_count: usize) -> String {
        match self.stdout_mode {
            PresentationMode::Rich => {
                let label = format!("📁 Writing {file_count} file(s)");
                format!("{}\n", render_rule(&label, &["33"]))
            }
            PresentationMode::Plain => format!("Writing {file_count} file(s)...\n"),
        }
    }

    fn render_files_written(&self, paths: &[std::path::PathBuf]) -> String {
        match self.stdout_mode {
            PresentationMode::Rich => {
                let mut output = format!("{}\n", style("✅ Files written successfully:", &["32"]));
                for path in paths {
                    let display_path = path.display().to_string();
                    output.push_str(&format!(
                        "   {} {}\n",
                        style("📄", &["2"]),
                        style(&display_path, &["4"])
                    ));
                }
                output.push('\n');
                output
            }
            PresentationMode::Plain => {
                let mut output = String::from("Files written successfully:\n");
                for path in paths {
                    output.push_str(&format!("{}\n", path.display()));
                }
                output.push('\n');
                output
            }
        }
    }

    fn render_success(&self, duration: Duration) -> String {
        match self.stdout_mode {
            PresentationMode::Rich => {
                let success_plain = "🎉 Generation completed successfully!";
                let success_styled = style("🎉 Generation completed successfully!", &["1", "32"]);

                format!(
                    "{}\n{}\n\n",
                    render_panel(success_plain, &success_styled, &["32"]),
                    style(
                        &format!("⏱️  Duration: {}", format_duration(duration)),
                        &["2"]
                    )
                )
            }
            PresentationMode::Plain => format!(
                "Generation completed successfully!\nDuration: {}\n",
                format_duration(duration)
            ),
        }
    }

    fn render_error(&self, error: &CliError) -> String {
        match (self.stderr_mode, error) {
            (PresentationMode::Rich, CliError::UnsupportedValidationVersion { version }) => {
                render_rich_unsupported_validation_error(error, version)
            }
            (PresentationMode::Plain, CliError::UnsupportedValidationVersion { version }) => {
                render_plain_unsupported_validation_error(error, version)
            }
            (PresentationMode::Rich, _) => {
                format!(
                    "{}\n{}\n",
                    style("Error:", &["31"]),
                    style(&error.to_string(), &["31"])
                )
            }
            (PresentationMode::Plain, _) => format!("Error: {error}\n"),
        }
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
        self.write_stdout(&self.render_validation_started());
    }

    fn validation_succeeded(&mut self, inspection: &OpenApiInspection) {
        self.write_stdout(&self.render_validation_succeeded(inspection));
    }

    fn azure_auth_started(&mut self) {
        self.write_stdout(&self.render_azure_auth_started());
    }

    fn azure_auth_finished(&mut self, status: &AzureAuthStatus) {
        match status {
            AzureAuthStatus::Failed { .. } => {
                self.write_stderr(&self.render_azure_auth_finished(status))
            }
            _ => self.write_stdout(&self.render_azure_auth_finished(status)),
        }
    }

    fn file_writing_started(&mut self, file_count: usize) {
        self.write_stdout(&self.render_file_writing_started(file_count));
    }

    fn files_written(&mut self, paths: &[std::path::PathBuf]) {
        self.write_stdout(&self.render_files_written(paths));
    }
}

fn mode_from_terminal(is_terminal: bool) -> PresentationMode {
    if is_terminal {
        PresentationMode::Rich
    } else {
        PresentationMode::Plain
    }
}

fn render_plain_header(no_logging: bool) -> String {
    format!(
        "HTTP File Generator v{}\nSupport key: {}\n\n",
        env!("CARGO_PKG_VERSION"),
        support_key_line(no_logging)
    )
}

fn render_rich_header(no_logging: bool) -> String {
    let version = format!("v{}", env!("CARGO_PKG_VERSION"));
    let title_plain = format!("🚀 HTTP File Generator {version}");
    let title_styled = format!(
        "{} {}",
        style("🚀 HTTP File Generator", &["1", "34"]),
        style(&version, &["2"])
    );
    let support_value = if no_logging {
        style("⚠️  Unavailable when logging is disabled", &["33"])
    } else {
        style(&format!("🔑 {}", support_key()), &["32"])
    };

    format!(
        "{}\nSupport key: {}\n\n",
        render_panel(&title_plain, &title_styled, &["34"]),
        support_value
    )
}

fn render_plain_stats(stats: &OpenApiStats) -> String {
    format!(
        "Path Items: {}\nOperations: {}\nParameters: {}\nRequest Bodies: {}\nResponses: {}\nLinks: {}\nCallbacks: {}\nSchemas: {}",
        stats.path_item_count,
        stats.operation_count,
        stats.parameter_count,
        stats.request_body_count,
        stats.response_count,
        stats.link_count,
        stats.callback_count,
        stats.schema_count
    )
}

fn render_stats_table(stats: &OpenApiStats) -> String {
    let rows = [
        ("📝 Path Items", stats.path_item_count.to_string()),
        ("⚡ Operations", stats.operation_count.to_string()),
        ("📝 Parameters", stats.parameter_count.to_string()),
        ("📤 Request Bodies", stats.request_body_count.to_string()),
        ("📥 Responses", stats.response_count.to_string()),
        ("🔗 Links", stats.link_count.to_string()),
        ("📞 Callbacks", stats.callback_count.to_string()),
        ("📋 Schemas", stats.schema_count.to_string()),
    ];

    let left_header = "📊 OpenAPI Statistics";
    let right_header = "Count";
    let left_width = rows
        .iter()
        .map(|(label, _)| text_width(label))
        .chain(std::iter::once(text_width(left_header)))
        .max()
        .unwrap_or_default();
    let right_width = rows
        .iter()
        .map(|(_, count)| text_width(count))
        .chain(std::iter::once(text_width(right_header)))
        .max()
        .unwrap_or_default();

    let mut output = String::new();
    output.push_str(&format!(
        "{}\n",
        style(
            &format!(
                "╭{}┬{}╮",
                "─".repeat(left_width + 2),
                "─".repeat(right_width + 2)
            ),
            &["32"]
        )
    ));
    output.push_str(&table_row(
        left_header,
        &style(left_header, &["1"]),
        right_header,
        &style(right_header, &["1"]),
        left_width,
        right_width,
    ));
    output.push_str(&format!(
        "{}\n",
        style(
            &format!(
                "├{}┼{}┤",
                "─".repeat(left_width + 2),
                "─".repeat(right_width + 2)
            ),
            &["32"]
        )
    ));
    for (label, count) in rows {
        output.push_str(&table_row(
            label,
            label,
            &count,
            &style(&count, &["36"]),
            left_width,
            right_width,
        ));
    }
    output.push_str(&style(
        &format!(
            "╰{}┴{}╯",
            "─".repeat(left_width + 2),
            "─".repeat(right_width + 2)
        ),
        &["32"],
    ));
    output
}

fn table_row(
    left_plain: &str,
    left_styled: &str,
    right_plain: &str,
    right_styled: &str,
    left_width: usize,
    right_width: usize,
) -> String {
    format!(
        "{} {}{} {} {}{} {}\n",
        style("│", &["32"]),
        left_styled,
        " ".repeat(left_width.saturating_sub(text_width(left_plain))),
        style("│", &["32"]),
        right_styled,
        " ".repeat(right_width.saturating_sub(text_width(right_plain))),
        style("│", &["32"])
    )
}

fn render_rule(label: &str, border_codes: &[&str]) -> String {
    let label_width = text_width(label);
    let total_width = rule_width().max(label_width + 4);
    let fill_width = total_width.saturating_sub(label_width + 2);
    let left_fill = fill_width / 2;
    let right_fill = fill_width.saturating_sub(left_fill);

    format!(
        "{} {} {}",
        style(&"─".repeat(left_fill), border_codes),
        style(label, border_codes),
        style(&"─".repeat(right_fill), border_codes)
    )
}

fn render_panel(content_plain: &str, content_styled: &str, border_codes: &[&str]) -> String {
    let inner_width = text_width(content_plain);
    let top = style(&format!("╭{}╮", "─".repeat(inner_width + 2)), border_codes);
    let middle = format!(
        "{} {}{} {}",
        style("│", border_codes),
        content_styled,
        " ".repeat(inner_width.saturating_sub(text_width(content_plain))),
        style("│", border_codes)
    );
    let bottom = style(&format!("╰{}╯", "─".repeat(inner_width + 2)), border_codes);

    format!("{top}\n{middle}\n{bottom}")
}

fn render_plain_unsupported_validation_error(
    error: &CliError,
    version: &OpenApiSpecificationVersion,
) -> String {
    format!(
        "Error: {error}\n\nTips:\nConsider using the --skip-validation argument.\nIn some cases, the features that are specific to unsupported OpenAPI versions aren't used.\nThe Rust validation path currently supports Swagger 2.0 and OpenAPI 3.0.x; {version} generation still works when validation is skipped.\n"
    )
}

fn render_rich_unsupported_validation_error(
    error: &CliError,
    version: &OpenApiSpecificationVersion,
) -> String {
    format!(
        "{}\n{}\n\n{}\n{}\n{}\n{}\n",
        style("Error:", &["31"]),
        style(&error.to_string(), &["31"]),
        style("Tips:", &["33"]),
        style("Consider using the --skip-validation argument.", &["33"]),
        style(
            "In some cases, the features that are specific to unsupported OpenAPI versions aren't used.",
            &["33"]
        ),
        style(
            &format!(
                "The Rust validation path currently supports Swagger 2.0 and OpenAPI 3.0.x; {version} generation still works when validation is skipped."
            ),
            &["33"]
        )
    )
}

fn support_key_line(no_logging: bool) -> String {
    if no_logging {
        "Unavailable when logging is disabled".to_string()
    } else {
        support_key()
    }
}

fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    let milliseconds = duration.subsec_millis();

    format!("{minutes:02}:{seconds:02}.{milliseconds:03}")
}

fn style(text: &str, codes: &[&str]) -> String {
    format!("\u{1b}[{}m{text}\u{1b}[0m", codes.join(";"))
}

fn rule_width() -> usize {
    env::var("COLUMNS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|width| *width >= 20)
        .unwrap_or(DEFAULT_RULE_WIDTH)
}

fn text_width(text: &str) -> usize {
    UnicodeWidthStr::width(text)
}

#[cfg(test)]
mod tests {
    use super::{
        CliPresenter, PresentationMode, format_duration, render_plain_header, support_key_line,
    };
    use std::time::Duration;

    #[test]
    fn support_key_line_uses_runtime_support_key_when_logging_is_enabled() {
        let runtime_support_key = support_key_line(false);

        assert_eq!(runtime_support_key.len(), 7);
        assert_ne!(runtime_support_key, "Unavailable when logging is disabled");
    }

    #[test]
    fn support_key_line_hides_support_key_when_logging_is_disabled() {
        assert_eq!(
            support_key_line(true),
            "Unavailable when logging is disabled"
        );
    }

    #[test]
    fn format_duration_matches_runtime_display_shape() {
        assert_eq!(format_duration(Duration::from_millis(8_123)), "00:08.123");
        assert_eq!(format_duration(Duration::from_millis(83_456)), "01:23.456");
    }

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
        let header = presenter.render_header(true);

        assert!(header.contains("╭"));
        assert!(header.contains("🚀 HTTP File Generator"));
        assert!(header.contains("⚠️  Unavailable when logging is disabled"));
    }
}
