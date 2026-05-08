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
