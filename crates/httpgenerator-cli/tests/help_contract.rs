use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};

const RICH_OUTPUT_MARKERS: &[&str] = &[
    "┌", "┐", "└", "┘", "├", "┤", "│", "─", "╭", "╮", "╰", "╯", "┬", "┴", "┼", "🚀", "🔍", "✅",
    "📊", "📝", "⚡", "📤", "📥", "🔗", "📞", "📋", "📁", "📄", "🎉", "⏱", "🔑", "⚠", "❌",
];

fn run_httpgenerator(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_httpgenerator"))
        .args(args)
        .output()
        .expect("httpgenerator command should run")
}

fn normalize(output: &[u8]) -> String {
    String::from_utf8_lossy(output).replace("\r\n", "\n")
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

fn temp_output_dir(name: &str) -> PathBuf {
    repo_root()
        .join("temp_test_out")
        .join("help-contract")
        .join(format!(
            "{name}-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ))
}

fn petstore_fixture() -> String {
    repo_root()
        .join("test")
        .join("OpenAPI")
        .join("v3.0")
        .join("petstore.json")
        .to_string_lossy()
        .into_owned()
}

fn webhook_fixture() -> String {
    repo_root()
        .join("test")
        .join("OpenAPI")
        .join("v3.1")
        .join("webhook-example.json")
        .to_string_lossy()
        .into_owned()
}

fn missing_fixture() -> String {
    repo_root()
        .join("test")
        .join("OpenAPI")
        .join("v3.0")
        .join("does-not-exist.json")
        .to_string_lossy()
        .into_owned()
}

fn assert_plain_redirected_output(output: &str) {
    assert!(
        !output.contains('\u{1b}'),
        "redirected output should not contain ANSI escape sequences\noutput:\n{output}"
    );

    for marker in RICH_OUTPUT_MARKERS {
        assert!(
            !output.contains(marker),
            "redirected output should stay plain and semantic; found rich marker `{marker}`\noutput:\n{output}"
        );
    }
}

fn generated_file_lines(output: &str) -> Vec<&str> {
    output
        .lines()
        .filter(|line| line.trim_end().ends_with(".http"))
        .collect()
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
    assert_plain_redirected_output(&stdout);
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
    assert_plain_redirected_output(&stdout);
    assert!(stdout.contains("HTTP File Generator v"));
    assert!(stdout.contains("Support key: "));
    assert!(!stdout.contains("Support key: Unavailable when logging is disabled"));
    assert!(stdout.contains("Validating OpenAPI specification..."));
    assert!(stdout.contains("Validated OpenAPI 3.0.x specification successfully"));
    assert!(stdout.contains("Path Items: 13"));
    assert!(stdout.contains("Operations: 19"));
    assert!(stdout.contains("Parameters: 17"));
    assert!(stdout.contains("Request Bodies: 9"));
    assert!(stdout.contains("Responses: 19"));
    assert!(stdout.contains("Links: 0"));
    assert!(stdout.contains("Callbacks: 0"));
    assert!(stdout.contains("Schemas: 73"));
    assert!(stdout.contains("Writing 19 file(s)..."));
    assert!(stdout.contains("Files written successfully:"));
    assert!(stdout.contains("Generation completed successfully!"));
    assert!(stdout.contains("Duration: "));
    let generated_files = generated_file_lines(&stdout);
    assert_eq!(
        generated_files.len(),
        19,
        "expected one plain file path per generated request\nstdout:\n{stdout}"
    );
    assert!(
        generated_files
            .iter()
            .any(|line| line.ends_with("PutUpdatePet.http"))
    );
    assert!(
        generated_files
            .iter()
            .any(|line| line.ends_with("GetLoginUser.http"))
    );
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
    assert_plain_redirected_output(&stdout);
    assert!(stdout.contains("HTTP File Generator v"));
    assert!(stdout.contains("Support key: Unavailable when logging is disabled"));
}

#[test]
fn azure_scope_warning_stays_plain_on_stderr_without_failing_generation() {
    let output_dir = temp_output_dir("azure-scope-warning");
    let petstore = petstore_fixture();
    let output_path = output_dir.to_string_lossy().into_owned();

    let output = run_httpgenerator(&[
        &petstore,
        "--output",
        &output_path,
        "--no-logging",
        "--azure-tenant-id",
        "tenant-id",
    ]);

    let _ = fs::remove_dir_all(&output_dir);

    assert!(output.status.success());
    let stdout = normalize(&output.stdout);
    let stderr = normalize(&output.stderr);
    assert_plain_redirected_output(&stdout);
    assert_plain_redirected_output(&stderr);
    assert!(stdout.contains("Generation completed successfully!"));
    assert!(!stdout.contains("Azure Entra ID scope is required"));
    assert_eq!(
        stderr,
        "Error: Azure Entra ID scope is required to acquire an authorization header.\n"
    );
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
    assert_plain_redirected_output(&stderr);
    assert!(stderr.contains("Error: OpenAPI 3.1.x documents are not supported by CLI validation yet; retry with --skip-validation"));
    assert!(stderr.contains("Tips:"));
    assert!(stderr.contains("Consider using the --skip-validation argument."));
    assert!(stderr.contains("Swagger 2.0 and OpenAPI 3.0.x"));
}

#[test]
fn missing_file_failure_is_plain_and_mentions_requested_input() {
    let output_dir = temp_output_dir("missing-file");
    let missing = missing_fixture();
    let output_path = output_dir.to_string_lossy().into_owned();

    let output = run_httpgenerator(&[&missing, "--output", &output_path]);

    let _ = fs::remove_dir_all(&output_dir);

    assert!(!output.status.success());
    let stderr = normalize(&output.stderr);
    assert_plain_redirected_output(&stderr);
    assert!(stderr.contains("Error:"));
    assert!(
        stderr.contains("does-not-exist.json"),
        "expected missing file error to mention the requested input\nstderr:\n{stderr}"
    );
}
