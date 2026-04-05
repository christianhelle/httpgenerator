use clap::{Arg, ArgAction, Command, CommandFactory, Parser, ValueEnum};
use httpgenerator_core::OutputType;
use std::fmt;

const HELP_EXAMPLES: &str = "\
Examples:
  httpgenerator ./openapi.json
  httpgenerator ./openapi.json --output ./
  httpgenerator ./openapi.json --output-type onefile
  httpgenerator https://petstore.swagger.io/v2/swagger.json
  httpgenerator https://petstore3.swagger.io/api/v3/openapi.json --base-url https://petstore3.swagger.io
  httpgenerator ./openapi.json --authorization-header Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9c
  httpgenerator ./openapi.json --azure-scope [Some Application ID URI]/.default
  httpgenerator ./openapi.json --generate-intellij-tests
  httpgenerator ./openapi.json --custom-header X-Custom-Header: Value --custom-header X-Another-Header: AnotherValue";

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Default)]
pub enum OutputTypeArg {
    #[default]
    #[value(name = "OneRequestPerFile")]
    OneRequestPerFile,
    #[value(name = "OneFile")]
    OneFile,
    #[value(name = "OneFilePerTag")]
    OneFilePerTag,
}

impl OutputTypeArg {
    pub const fn as_str(self) -> &'static str {
        match self {
            OutputTypeArg::OneRequestPerFile => "OneRequestPerFile",
            OutputTypeArg::OneFile => "OneFile",
            OutputTypeArg::OneFilePerTag => "OneFilePerTag",
        }
    }
}

impl fmt::Display for OutputTypeArg {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl From<OutputTypeArg> for OutputType {
    fn from(value: OutputTypeArg) -> Self {
        match value {
            OutputTypeArg::OneRequestPerFile => OutputType::OneRequestPerFile,
            OutputTypeArg::OneFile => OutputType::OneFile,
            OutputTypeArg::OneFilePerTag => OutputType::OneFilePerTag,
        }
    }
}

#[derive(Debug, Clone, Parser, PartialEq, Eq)]
#[command(
    name = "httpgenerator",
    bin_name = "httpgenerator",
    version,
    about = "Generate .http files from OpenAPI specifications",
    disable_help_flag = true,
    disable_version_flag = true
)]
pub struct CliArgs {
    #[arg(
        value_name = "URL or input file",
        help = "URL or file path to OpenAPI Specification file"
    )]
    pub open_api_path: Option<String>,

    #[arg(
        short = 'o',
        long = "output",
        value_name = "OUTPUT",
        default_value = "./",
        help = "Output directory"
    )]
    pub output_folder: String,

    #[arg(
        long = "no-logging",
        default_value_t = false,
        help = "Don't log errors or collect telemetry"
    )]
    pub no_logging: bool,

    #[arg(
        long = "skip-validation",
        default_value_t = false,
        help = "Skip validation of OpenAPI Specification file"
    )]
    pub skip_validation: bool,

    #[arg(
        long = "authorization-header",
        value_name = "HEADER",
        help = "Authorization header to use for all requests"
    )]
    pub authorization_header: Option<String>,

    #[arg(
        long = "load-authorization-header-from-environment",
        default_value_t = false,
        help = "Load the authorization header from an environment variable or define it in the .http file. You can use --authorization-header-variable-name to specify the environment variable name."
    )]
    pub authorization_header_from_environment_variable: bool,

    #[arg(
        long = "authorization-header-variable-name",
        value_name = "VARIABLE-NAME",
        default_value = "authorization",
        help = "Name of the environment variable to load the authorization header from"
    )]
    pub authorization_header_variable_name: String,

    #[arg(
        long = "content-type",
        value_name = "CONTENT-TYPE",
        default_value = "application/json",
        help = "Default Content-Type header to use for all requests"
    )]
    pub content_type: String,

    #[arg(
        long = "base-url",
        value_name = "BASE-URL",
        help = "Default Base URL to use for all requests. Use this if the OpenAPI spec doesn't explicitly specify a server URL."
    )]
    pub base_url: Option<String>,

    #[arg(
        long = "output-type",
        value_name = "OUTPUT-TYPE",
        default_value_t = OutputTypeArg::OneRequestPerFile,
        ignore_case = true,
        help = "OneRequestPerFile generates one .http file per request. OneFile generates a single .http file for all requests. OneFilePerTag generates one .http file per first tag associated with each request."
    )]
    pub output_type: OutputTypeArg,

    #[arg(
        long = "azure-scope",
        value_name = "SCOPE",
        help = "Azure Entra ID Scope to use for retrieving Access Token for Authorization header"
    )]
    pub azure_scope: Option<String>,

    #[arg(
        long = "azure-tenant-id",
        value_name = "TENANT-ID",
        help = "Azure Entra ID Tenant ID to use for retrieving Access Token for Authorization header"
    )]
    pub azure_tenant_id: Option<String>,

    #[arg(
        long = "timeout",
        value_name = "SECONDS",
        default_value_t = 120,
        help = "Timeout (in seconds) for writing files to disk"
    )]
    pub timeout: u64,

    #[arg(
        long = "generate-intellij-tests",
        default_value_t = false,
        help = "Generate IntelliJ tests that assert whether the response status code is 200"
    )]
    pub generate_intellij_tests: bool,

    #[arg(
        long = "custom-header",
        value_name = "HEADER",
        help = "Add custom HTTP headers to the generated request"
    )]
    pub custom_headers: Vec<String>,

    #[arg(
        long = "skip-headers",
        default_value_t = false,
        help = "Don't generate header parameters in the files"
    )]
    pub skip_headers: bool,
}

pub fn build_command() -> Command {
    CliArgs::command()
        .override_usage("httpgenerator [URL or input file] [OPTIONS]")
        .after_help(HELP_EXAMPLES)
        .term_width(100)
        .arg(
            Arg::new("help")
                .short('h')
                .long("help")
                .help("Print help information")
                .action(ArgAction::Help),
        )
        .arg(
            Arg::new("version")
                .short('v')
                .long("version")
                .help("Print version information")
                .action(ArgAction::Version),
        )
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::{CliArgs, OutputTypeArg, build_command};

    #[test]
    fn defaults_match_current_cli_surface() {
        let args = CliArgs::parse_from(["httpgenerator", "./openapi.json"]);

        assert_eq!(args.open_api_path.as_deref(), Some("./openapi.json"));
        assert_eq!(args.output_folder, "./");
        assert!(!args.no_logging);
        assert!(!args.skip_validation);
        assert_eq!(args.authorization_header, None);
        assert!(!args.authorization_header_from_environment_variable);
        assert_eq!(args.authorization_header_variable_name, "authorization");
        assert_eq!(args.content_type, "application/json");
        assert_eq!(args.base_url, None);
        assert_eq!(args.output_type, OutputTypeArg::OneRequestPerFile);
        assert_eq!(args.azure_scope, None);
        assert_eq!(args.azure_tenant_id, None);
        assert_eq!(args.timeout, 120);
        assert!(!args.generate_intellij_tests);
        assert!(args.custom_headers.is_empty());
        assert!(!args.skip_headers);
    }

    #[test]
    fn parses_repeated_headers_and_explicit_output_type() {
        let args = CliArgs::parse_from([
            "httpgenerator",
            "./openapi.json",
            "--output-type",
            "OneFilePerTag",
            "--custom-header",
            "X-First: one",
            "--custom-header",
            "X-Second: two",
        ]);

        assert_eq!(args.output_type, OutputTypeArg::OneFilePerTag);
        assert_eq!(
            args.custom_headers,
            vec!["X-First: one".to_string(), "X-Second: two".to_string()]
        );
    }

    #[test]
    fn help_command_uses_httpgenerator_identity_and_examples() {
        let help = build_command().render_long_help().to_string();

        assert!(help.contains("Usage: httpgenerator [URL or input file] [OPTIONS]"));
        assert!(help.contains("Examples:"));
        assert!(help.contains("httpgenerator ./openapi.json --output-type onefile"));
        assert!(help.contains("httpgenerator https://petstore.swagger.io/v2/swagger.json"));
        assert!(!help.contains("httpgenerator-cli"));
    }
}
