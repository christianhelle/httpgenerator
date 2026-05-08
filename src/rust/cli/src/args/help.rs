use clap::{Arg, ArgAction, Command};

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

pub(super) fn configure(command: Command) -> Command {
    command
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
