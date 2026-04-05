use clap::FromArgMatches;
use httpgenerator_cli::{
    AzureAuthStatus, CliError,
    args::{CliArgs, build_command},
    execute,
};
use httpgenerator_core::support_key;
use httpgenerator_openapi::OpenApiStats;
use std::{
    ffi::OsString,
    time::{Duration, Instant},
};

fn main() {
    let mut raw_args: Vec<OsString> = std::env::args_os().collect();
    if raw_args.len() == 1 {
        raw_args.push(OsString::from("--help"));
    }

    let matches = build_command().get_matches_from(raw_args);
    let args = CliArgs::from_arg_matches(&matches)
        .expect("clap should only return matches that satisfy CliArgs");
    let should_validate = !args.skip_validation;
    let should_attempt_azure_auth = should_attempt_azure_auth(&args);
    let started_at = Instant::now();
    print_header(args.no_logging);

    if should_validate {
        println!("Validating OpenAPI specification...");
    }

    match execute(args) {
        Ok(summary) => {
            if let Some(validation) = &summary.validation {
                println!(
                    "Validated {} specification successfully",
                    validation.specification_version
                );
                print_stats(&validation.stats);
                println!();
            }

            if should_attempt_azure_auth {
                println!("Acquiring authorization header from Azure Entra ID...");
            }

            match &summary.azure_auth {
                AzureAuthStatus::NotRequested => {}
                AzureAuthStatus::Acquired => {
                    println!("Successfully acquired access token from Azure Entra ID");
                }
                AzureAuthStatus::Failed { reason } => {
                    eprintln!("Error: {reason}");
                }
            }

            println!("Writing {} file(s)...", summary.files.len());
            println!("Files written successfully:");
            for path in &summary.files {
                println!("{}", path.display());
            }
            println!();
            println!("Generation completed successfully!");
            println!("Duration: {}", format_duration(started_at.elapsed()));
        }
        Err(error) => {
            if let CliError::UnsupportedValidationVersion { version } = &error {
                eprintln!("Error: {error}");
                eprintln!();
                eprintln!("Tips:");
                eprintln!("Consider using the --skip-validation argument.");
                eprintln!(
                    "In some cases, the features that are specific to unsupported OpenAPI versions aren't used."
                );
                eprintln!(
                    "The Rust validation path currently supports Swagger 2.0 and OpenAPI 3.0.x; {version} generation still works when validation is skipped."
                );
                std::process::exit(1);
            }

            eprintln!("Error: {error}");
            std::process::exit(1);
        }
    }
}

fn print_header(no_logging: bool) {
    println!("HTTP File Generator v{}", env!("CARGO_PKG_VERSION"));
    println!("Support key: {}", support_key_line(no_logging));
    println!();
}

fn support_key_line(no_logging: bool) -> String {
    if no_logging {
        "Unavailable when logging is disabled".to_string()
    } else {
        support_key()
    }
}

fn should_attempt_azure_auth(args: &CliArgs) -> bool {
    if args
        .authorization_header
        .as_deref()
        .map(str::trim)
        .is_some_and(|header| !header.is_empty())
    {
        return false;
    }

    args.azure_scope
        .as_deref()
        .map(str::trim)
        .is_some_and(|scope| !scope.is_empty())
        || args
            .azure_tenant_id
            .as_deref()
            .map(str::trim)
            .is_some_and(|tenant_id| !tenant_id.is_empty())
}

fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    let milliseconds = duration.subsec_millis();

    format!("{minutes:02}:{seconds:02}.{milliseconds:03}")
}

fn print_stats(stats: &OpenApiStats) {
    println!("Path Items: {}", stats.path_item_count);
    println!("Operations: {}", stats.operation_count);
    println!("Parameters: {}", stats.parameter_count);
    println!("Request Bodies: {}", stats.request_body_count);
    println!("Responses: {}", stats.response_count);
    println!("Links: {}", stats.link_count);
    println!("Callbacks: {}", stats.callback_count);
    println!("Schemas: {}", stats.schema_count);
}

#[cfg(test)]
mod tests {
    use super::{format_duration, should_attempt_azure_auth, support_key_line};
    use httpgenerator_cli::args::{CliArgs, OutputTypeArg};
    use std::time::Duration;

    #[test]
    fn support_key_line_uses_runtime_support_key_when_logging_is_enabled() {
        let support_key = support_key_line(false);

        assert_eq!(support_key.len(), 7);
        assert_ne!(support_key, "Unavailable when logging is disabled");
    }

    #[test]
    fn support_key_line_hides_support_key_when_logging_is_disabled() {
        assert_eq!(
            support_key_line(true),
            "Unavailable when logging is disabled"
        );
    }

    #[test]
    fn should_attempt_azure_auth_only_when_scope_or_tenant_is_present_without_header() {
        let mut args = CliArgs {
            open_api_path: None,
            output_folder: "./".to_string(),
            no_logging: false,
            skip_validation: false,
            authorization_header: None,
            authorization_header_from_environment_variable: false,
            authorization_header_variable_name: "authorization".to_string(),
            content_type: "application/json".to_string(),
            base_url: None,
            output_type: OutputTypeArg::OneRequestPerFile,
            azure_scope: None,
            azure_tenant_id: None,
            timeout: 120,
            generate_intellij_tests: false,
            custom_headers: Vec::new(),
            skip_headers: false,
        };

        assert!(!should_attempt_azure_auth(&args));

        args.azure_scope = Some("api://example/.default".to_string());
        assert!(should_attempt_azure_auth(&args));

        args.authorization_header = Some("Bearer token".to_string());
        assert!(!should_attempt_azure_auth(&args));
    }

    #[test]
    fn format_duration_matches_runtime_display_shape() {
        assert_eq!(format_duration(Duration::from_millis(8_123)), "00:08.123");
        assert_eq!(format_duration(Duration::from_millis(83_456)), "01:23.456");
    }
}
