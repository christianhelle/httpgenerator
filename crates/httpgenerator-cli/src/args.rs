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
    version,
    about = "Generate .http files from OpenAPI specifications"
)]
pub struct CliArgs {
    #[arg(value_name = "URL or input file")]
    pub open_api_path: Option<String>,

    #[arg(short = 'o', long = "output", default_value = "./")]
    pub output_folder: String,

    #[arg(long = "no-logging", default_value_t = false)]
    pub no_logging: bool,

    #[arg(long = "skip-validation", default_value_t = false)]
    pub skip_validation: bool,

    #[arg(long = "authorization-header")]
    pub authorization_header: Option<String>,

    #[arg(
        long = "load-authorization-header-from-environment",
        default_value_t = false
    )]
    pub authorization_header_from_environment_variable: bool,

    #[arg(
        long = "authorization-header-variable-name",
        default_value = "authorization"
    )]
    pub authorization_header_variable_name: String,

    #[arg(long = "content-type", default_value = "application/json")]
    pub content_type: String,

    #[arg(long = "base-url")]
    pub base_url: Option<String>,

    #[arg(long = "output-type", default_value_t = OutputTypeArg::OneRequestPerFile, ignore_case = true)]
    pub output_type: OutputTypeArg,

    #[arg(long = "azure-scope")]
    pub azure_scope: Option<String>,

    #[arg(long = "azure-tenant-id")]
    pub azure_tenant_id: Option<String>,

    #[arg(long = "timeout", default_value_t = 120)]
    pub timeout: u64,

    #[arg(long = "generate-intellij-tests", default_value_t = false)]
    pub generate_intellij_tests: bool,

    #[arg(long = "custom-header")]
    pub custom_headers: Vec<String>,

    #[arg(long = "skip-headers", default_value_t = false)]
    pub skip_headers: bool,
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::{CliArgs, OutputTypeArg};

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
}
