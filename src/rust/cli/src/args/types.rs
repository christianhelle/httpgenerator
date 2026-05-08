use clap::{Parser, ValueEnum};
use httpgenerator_core::OutputType;
use std::fmt;

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

impl Default for CliArgs {
    fn default() -> Self {
        Self {
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
        }
    }
}
