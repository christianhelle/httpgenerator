use httpgenerator_cli::{AzureAuthStatus, CliError};
use httpgenerator_core::openapi::{OpenApiInspection, OpenApiSpecificationVersion, OpenApiStats};
use std::{env, path::PathBuf, time::Duration};

use super::format::{
    PresentationMode, format_duration, render_panel, render_rule, style, support_key_line,
    table_row, text_width,
};

pub(super) fn render_header(mode: PresentationMode, no_logging: bool) -> String {
    match mode {
        PresentationMode::Rich => render_rich_header(no_logging),
        PresentationMode::Plain => render_plain_header(no_logging),
    }
}

pub(super) fn render_validation_started(mode: PresentationMode) -> String {
    match mode {
        PresentationMode::Rich => format!(
            "{}\n",
            style("🔍 Validating OpenAPI specification...", &["36"])
        ),
        PresentationMode::Plain => "Validating OpenAPI specification...\n".to_string(),
    }
}

pub(super) fn render_validation_succeeded(
    mode: PresentationMode,
    inspection: &OpenApiInspection,
) -> String {
    match mode {
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

pub(super) fn render_azure_auth_started(mode: PresentationMode) -> String {
    match mode {
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

pub(super) fn render_azure_auth_finished(
    stdout_mode: PresentationMode,
    stderr_mode: PresentationMode,
    status: &AzureAuthStatus,
) -> String {
    match status {
        AzureAuthStatus::NotRequested => String::new(),
        AzureAuthStatus::Acquired => match stdout_mode {
            PresentationMode::Rich => format!(
                "{}\n\n",
                style("✅ Successfully acquired access token", &["32"])
            ),
            PresentationMode::Plain => {
                "Successfully acquired access token from Azure Entra ID\n\n".to_string()
            }
        },
        AzureAuthStatus::Failed { reason } => match stderr_mode {
            PresentationMode::Rich => {
                format!("{} {}\n", style("Error:", &["31"]), style(reason, &["31"]))
            }
            PresentationMode::Plain => format!("Error: {reason}\n"),
        },
    }
}

pub(super) fn render_file_writing_started(mode: PresentationMode, file_count: usize) -> String {
    match mode {
        PresentationMode::Rich => {
            let label = format!("📁 Writing {file_count} file(s)");
            format!("{}\n", render_rule(&label, &["33"]))
        }
        PresentationMode::Plain => format!("Writing {file_count} file(s)...\n"),
    }
}

pub(super) fn render_files_written(mode: PresentationMode, paths: &[PathBuf]) -> String {
    match mode {
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

pub(super) fn render_success(mode: PresentationMode, duration: Duration) -> String {
    match mode {
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

pub(super) fn render_error(mode: PresentationMode, error: &CliError) -> String {
    match (mode, error) {
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

pub(super) fn render_plain_header(no_logging: bool) -> String {
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
        style(
            &format!("🔑 {}", httpgenerator_core::support_key()),
            &["32"],
        )
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
