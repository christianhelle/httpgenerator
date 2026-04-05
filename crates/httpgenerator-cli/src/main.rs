use clap::FromArgMatches;
use httpgenerator_cli::{
    AzureAuthStatus, CliError,
    args::{CliArgs, build_command},
    execute,
};
use httpgenerator_core::support_key;
use httpgenerator_openapi::OpenApiStats;
use std::ffi::OsString;

fn main() {
    let mut raw_args: Vec<OsString> = std::env::args_os().collect();
    if raw_args.len() == 1 {
        raw_args.push(OsString::from("--help"));
    }

    let matches = build_command().get_matches_from(raw_args);
    let args = CliArgs::from_arg_matches(&matches)
        .expect("clap should only return matches that satisfy CliArgs");
    print_header(args.no_logging);

    match execute(args) {
        Ok(summary) => {
            match &summary.azure_auth {
                AzureAuthStatus::NotRequested => {}
                AzureAuthStatus::Acquired => {
                    println!("Successfully acquired access token from Azure Entra ID");
                }
                AzureAuthStatus::Failed { reason } => {
                    eprintln!("Error: {reason}");
                }
            }

            if let Some(validation) = &summary.validation {
                println!(
                    "Validated {} specification successfully",
                    validation.specification_version
                );
                print_stats(&validation.stats);
            }

            println!(
                "Generated {} file(s) in {}",
                summary.files.len(),
                summary.output_folder.display()
            );
            for path in summary.files {
                println!("{}", path.display());
            }
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
    use super::support_key_line;

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
}
