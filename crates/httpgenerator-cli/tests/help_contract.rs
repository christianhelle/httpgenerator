use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};

fn run_httpgenerator(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_httpgenerator"))
        .args(args)
        .output()
        .expect("httpgenerator command should run")
}

fn normalize(output: &[u8]) -> String {
    String::from_utf8_lossy(output).replace("\r\n", "\n")
}

fn temp_output_dir(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "httpgenerator-rust-cli-bin-tests-{name}-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ))
}

fn petstore_fixture() -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("test")
        .join("OpenAPI")
        .join("v3.0")
        .join("petstore.json")
        .to_string_lossy()
        .into_owned()
}

fn webhook_fixture() -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("test")
        .join("OpenAPI")
        .join("v3.1")
        .join("webhook-example.json")
        .to_string_lossy()
        .into_owned()
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

#[test]
fn generation_output_includes_support_key_header() {
    let output_dir = temp_output_dir("support-key");
    let petstore = petstore_fixture();
    let output_path = output_dir.to_string_lossy().into_owned();

    let output = run_httpgenerator(&[&petstore, "--output", &output_path]);

    let _ = fs::remove_dir_all(&output_dir);

    assert!(output.status.success());
    let stdout = normalize(&output.stdout);
    assert!(stdout.contains("HTTP File Generator v"));
    assert!(stdout.contains("Support key: "));
    assert!(!stdout.contains("Support key: Unavailable when logging is disabled"));
    assert!(stdout.contains("Validating OpenAPI specification..."));
    assert!(stdout.contains("Validated OpenAPI 3.0.x specification successfully"));
    assert!(stdout.contains("Writing 19 file(s)..."));
    assert!(stdout.contains("Files written successfully:"));
    assert!(stdout.contains("Generation completed successfully!"));
    assert!(stdout.contains("Duration: "));
}

#[test]
fn generation_output_hides_support_key_when_logging_is_disabled() {
    let output_dir = temp_output_dir("support-key-disabled");
    let petstore = petstore_fixture();
    let output_path = output_dir.to_string_lossy().into_owned();

    let output = run_httpgenerator(&[&petstore, "--output", &output_path, "--no-logging"]);

    let _ = fs::remove_dir_all(&output_dir);

    assert!(output.status.success());
    let stdout = normalize(&output.stdout);
    assert!(stdout.contains("HTTP File Generator v"));
    assert!(stdout.contains("Support key: Unavailable when logging is disabled"));
}

#[test]
fn unsupported_v31_validation_failure_suggests_skip_validation() {
    let output_dir = temp_output_dir("v31-guidance");
    let webhook = webhook_fixture();
    let output_path = output_dir.to_string_lossy().into_owned();

    let output = run_httpgenerator(&[&webhook, "--output", &output_path]);

    let _ = fs::remove_dir_all(&output_dir);

    assert!(!output.status.success());
    let stderr = normalize(&output.stderr);
    assert!(stderr.contains("Error: OpenAPI 3.1.x documents are not supported by CLI validation yet; retry with --skip-validation"));
    assert!(stderr.contains("Tips:"));
    assert!(stderr.contains("Consider using the --skip-validation argument."));
    assert!(stderr.contains("Swagger 2.0 and OpenAPI 3.0.x"));
}
