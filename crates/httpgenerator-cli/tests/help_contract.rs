use std::process::{Command, Output};

fn run_httpgenerator(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_httpgenerator"))
        .args(args)
        .output()
        .expect("httpgenerator command should run")
}

fn normalize(output: &[u8]) -> String {
    String::from_utf8_lossy(output).replace("\r\n", "\n")
}

#[test]
fn no_args_matches_help_output_and_contract() {
    let no_args = run_httpgenerator(&[]);
    let help = run_httpgenerator(&["--help"]);

    assert!(no_args.status.success());
    assert!(help.status.success());
    assert_eq!(normalize(&no_args.stdout), normalize(&help.stdout));
    assert_eq!(normalize(&no_args.stderr), "");
    assert_eq!(normalize(&help.stderr), "");

    let stdout = normalize(&help.stdout);
    for expected in [
        "Usage: httpgenerator [URL or input file] [OPTIONS]",
        "Examples:",
        "httpgenerator ./openapi.json",
        "httpgenerator ./openapi.json --output ./",
        "httpgenerator ./openapi.json --output-type onefile",
        "httpgenerator https://petstore.swagger.io/v2/swagger.json",
        "httpgenerator https://petstore3.swagger.io/api/v3/openapi.json --base-url https://petstore3.swagger.io",
        "httpgenerator ./openapi.json --authorization-header Bearer",
        "httpgenerator ./openapi.json --azure-scope [Some Application ID URI]/.default",
        "httpgenerator ./openapi.json --generate-intellij-tests",
        "httpgenerator ./openapi.json --custom-header X-Custom-Header: Value --custom-header X-Another-Header: AnotherValue",
        "URL or file path to OpenAPI Specification file",
        "Output directory",
        "Default Content-Type header to use for all requests",
        "Generate IntelliJ tests that assert whether the response status code is 200",
        "Don't generate header parameters in the files",
        "--authorization-header-variable-name <VARIABLE-NAME>",
        "[default: ./]",
        "[default: authorization]",
        "[default: application/json]",
        "[default: OneRequestPerFile]",
        "[default: 120]",
    ] {
        assert!(
            stdout.contains(expected),
            "expected help output to contain `{expected}`\nstdout:\n{stdout}"
        );
    }

    assert!(
        !stdout.contains("httpgenerator-cli"),
        "help output should use the public command identity\nstdout:\n{stdout}"
    );
}

#[test]
fn short_and_long_version_flags_match_public_identity() {
    let short = run_httpgenerator(&["-v"]);
    let long = run_httpgenerator(&["--version"]);
    let expected = format!("httpgenerator {}\n", env!("CARGO_PKG_VERSION"));

    assert!(short.status.success());
    assert!(long.status.success());
    assert_eq!(normalize(&short.stdout), expected);
    assert_eq!(normalize(&long.stdout), expected);
    assert_eq!(normalize(&short.stderr), "");
    assert_eq!(normalize(&long.stderr), "");
}
