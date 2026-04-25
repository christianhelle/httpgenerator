use clap::Parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputType {
    OneRequestPerFile,
    OneFile,
    OneFilePerTag,
}

impl Default for OutputType {
    fn default() -> Self {
        Self::OneRequestPerFile
    }
}

fn parse_output_type(value: &str) -> Result<OutputType, String> {
    match value.to_ascii_lowercase().replace(['-', '_'], "").as_str() {
        "onerequestperfile" => Ok(OutputType::OneRequestPerFile),
        "onefile" => Ok(OutputType::OneFile),
        "onefilepertag" => Ok(OutputType::OneFilePerTag),
        _ => Err(format!(
            "invalid output type '{value}'. Expected OneRequestPerFile, OneFile, or OneFilePerTag"
        )),
    }
}

#[derive(Debug, Parser)]
#[command(
    name = "httpgenerator",
    version,
    about = "Generate .http files from OpenAPI specifications"
)]
pub struct Cli {
    #[arg(
        value_name = "URL or input file",
        help = "URL or file path to OpenAPI Specification file"
    )]
    pub openapi_path: String,

    #[arg(
        short = 'o',
        long = "output",
        value_name = "OUTPUT",
        default_value = "./",
        help = "Output directory"
    )]
    pub output: String,

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
        help = "Load the authorization header from an environment variable or define it in the .http file"
    )]
    pub load_authorization_header_from_environment: bool,

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
        help = "Default Base URL to use for all requests"
    )]
    pub base_url: Option<String>,

    #[arg(
        long = "output-type",
        value_name = "OUTPUT-TYPE",
        default_value = "OneRequestPerFile",
        value_parser = parse_output_type,
        help = "OneRequestPerFile, OneFile, or OneFilePerTag"
    )]
    pub output_type: OutputType,

    #[arg(
        long = "azure-scope",
        value_name = "SCOPE",
        help = "Azure Entra ID Scope to use for retrieving Access Token"
    )]
    pub azure_scope: Option<String>,

    #[arg(
        long = "azure-tenant-id",
        value_name = "TENANT-ID",
        help = "Azure Entra ID Tenant ID to use for retrieving Access Token"
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

    #[arg(long = "custom-header", value_name = "HEADER", action = clap::ArgAction::Append, help = "Add custom HTTP headers to the generated request")]
    pub custom_header: Vec<String>,

    #[arg(
        long = "skip-headers",
        default_value_t = false,
        help = "Don't generate header parameters in the files"
    )]
    pub skip_headers: bool,
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
