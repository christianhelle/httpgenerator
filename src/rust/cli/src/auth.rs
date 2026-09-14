//! Azure Entra ID access token acquisition.
//!
//! Tokens are obtained by invoking the Azure CLI (`az`) and the Azure Developer CLI (`azd`)
//! directly, which is what the Azure identity SDKs do for these credential types. Shelling out
//! keeps the dependency surface of this crate to the standard library plus JSON parsing.

use std::{
    io::{self, Read},
    process::{Child, Command, Stdio},
    sync::mpsc::{self, Receiver, RecvTimeoutError},
    thread,
    time::{Duration, Instant},
};

use serde_json::Value;

/// How long `az` or `azd` may take before the attempt is abandoned. A cold start of either CLI
/// can take several seconds, so this is deliberately generous.
const PROCESS_TIMEOUT: Duration = Duration::from_secs(30);
const POLL_INTERVAL: Duration = Duration::from_millis(20);

/// What a finished CLI process reported.
#[derive(Debug, Clone, PartialEq, Eq)]
struct CliResponse {
    success: bool,
    stdout: String,
    stderr: String,
}

pub fn try_get_access_token(
    tenant_id: Option<&str>,
    scope: &str,
) -> Result<Option<String>, String> {
    try_get_access_token_with(tenant_id, scope, run)
}

/// Tries the Azure CLI first and the Azure Developer CLI second, using `run` to invoke each one
/// with a program name, its arguments and a credential name for error messages.
fn try_get_access_token_with(
    tenant_id: Option<&str>,
    scope: &str,
    mut run: impl FnMut(&str, &[&str], &str) -> Result<CliResponse, String>,
) -> Result<Option<String>, String> {
    let tenant_id = tenant_id
        .map(str::trim)
        .filter(|tenant_id| !tenant_id.is_empty());
    let scope = scope.trim();

    if scope.is_empty() {
        return Ok(None);
    }

    let mut errors = Vec::new();

    let azure_cli = run("az", &azure_cli_args(tenant_id, scope), "Azure CLI")
        .and_then(|response| token_from_response(&response, "accessToken", "Azure CLI"));
    match azure_cli {
        Ok(token) => return Ok(Some(token)),
        Err(error) => errors.push(error),
    }

    let azure_developer_cli = run(
        "azd",
        &azure_developer_cli_args(tenant_id, scope),
        "Azure Developer CLI",
    )
    .and_then(|response| token_from_response(&response, "token", "Azure Developer CLI"));
    match azure_developer_cli {
        Ok(token) => return Ok(Some(token)),
        Err(error) => errors.push(error),
    }

    Err(errors.join("\n"))
}

fn azure_cli_args<'a>(tenant_id: Option<&'a str>, scope: &'a str) -> Vec<&'a str> {
    let mut args = vec![
        "account",
        "get-access-token",
        "--scope",
        scope,
        "--output",
        "json",
    ];

    if let Some(tenant_id) = tenant_id {
        args.push("--tenant");
        args.push(tenant_id);
    }

    args
}

fn azure_developer_cli_args<'a>(tenant_id: Option<&'a str>, scope: &'a str) -> Vec<&'a str> {
    let mut args = vec!["auth", "token", "--scope", scope, "--output", "json"];

    if let Some(tenant_id) = tenant_id {
        args.push("--tenant-id");
        args.push(tenant_id);
    }

    args
}

fn run(program: &str, args: &[&str], credential: &str) -> Result<CliResponse, String> {
    // Only Windows routes the call through the command interpreter; elsewhere arguments are
    // passed to the process verbatim, so any characters are fine.
    if cfg!(windows) {
        for argument in args {
            if !is_safe_argument(argument) {
                return Err(format!(
                    "{credential} credential initialization failed: argument {argument:?} contains unsupported characters"
                ));
            }
        }
    }

    let child = build_command(program, args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| spawn_error(program, credential, &error))?;

    match wait_with_timeout(child, PROCESS_TIMEOUT) {
        Ok(Some(response)) => Ok(response),
        Ok(None) => Err(format!(
            "{credential} credential failed: `{program}` did not finish within {} seconds",
            PROCESS_TIMEOUT.as_secs()
        )),
        Err(error) => Err(format!("{credential} credential failed: {error}")),
    }
}

/// Waits for `child` to exit, killing it and returning `None` once `timeout` has passed.
fn wait_with_timeout(mut child: Child, timeout: Duration) -> io::Result<Option<CliResponse>> {
    // The pipes are drained on their own threads so a chatty process cannot block on a full pipe.
    let stdout = read_in_background(child.stdout.take());
    let stderr = read_in_background(child.stderr.take());
    let deadline = Instant::now() + timeout;

    loop {
        if let Some(status) = child.try_wait()? {
            // A descendant can outlive the process and keep the pipes open, so the output is only
            // awaited for whatever remains of the deadline.
            let (Some(stdout), Some(stderr)) = (
                receive_before(&stdout, deadline),
                receive_before(&stderr, deadline),
            ) else {
                return Ok(None);
            };

            return Ok(Some(CliResponse {
                success: status.success(),
                stdout,
                stderr,
            }));
        }

        if Instant::now() >= deadline {
            // The reader threads are left behind: a grandchild process may still hold the pipes.
            let _ = child.kill();
            let _ = child.wait();
            return Ok(None);
        }

        thread::sleep(POLL_INTERVAL);
    }
}

fn read_in_background(pipe: Option<impl Read + Send + 'static>) -> Receiver<String> {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let mut bytes = Vec::new();
        if let Some(mut pipe) = pipe {
            let _ = pipe.read_to_end(&mut bytes);
        }
        let _ = sender.send(String::from_utf8_lossy(&bytes).into_owned());
    });

    receiver
}

/// Returns the collected output, or `None` if the pipe is still open when `deadline` passes.
fn receive_before(output: &Receiver<String>, deadline: Instant) -> Option<String> {
    match output.recv_timeout(deadline.saturating_duration_since(Instant::now())) {
        Ok(output) => Some(output),
        Err(RecvTimeoutError::Timeout) => None,
        Err(RecvTimeoutError::Disconnected) => Some(String::new()),
    }
}

fn spawn_error(program: &str, credential: &str, error: &std::io::Error) -> String {
    if error.kind() == std::io::ErrorKind::NotFound {
        format!(
            "{credential} credential initialization failed: {} ({error})",
            not_found_hint(program)
        )
    } else {
        format!("{credential} credential initialization failed: could not start `{program}` ({error})")
    }
}

#[cfg(windows)]
fn build_command(program: &str, args: &[&str]) -> Command {
    // `az` and `azd` ship as batch scripts on Windows, which `CreateProcess` cannot launch
    // directly, so they are resolved through the command interpreter.
    let mut command = Command::new("cmd");
    command.arg("/C").arg(program).args(args);
    command
}

#[cfg(not(windows))]
fn build_command(program: &str, args: &[&str]) -> Command {
    let mut command = Command::new(program);
    command.args(args);
    command
}

/// Rejects arguments that the Windows command interpreter would treat as syntax rather than data.
fn is_safe_argument(argument: &str) -> bool {
    !argument.is_empty()
        && !argument.chars().any(|character| {
            matches!(character, '&' | '|' | '<' | '>' | '^' | '"' | '%' | '\r' | '\n')
        })
}

fn not_found_hint(program: &str) -> String {
    format!("`{program}` was not found on PATH")
}

fn token_from_response(
    response: &CliResponse,
    field: &str,
    credential: &str,
) -> Result<String, String> {
    if !response.success {
        let details = if response.stderr.trim().is_empty() {
            &response.stdout
        } else {
            &response.stderr
        };

        return Err(format!(
            "{credential} credential failed: {}",
            summarize_error(details)
        ));
    }

    let payload: Value = serde_json::from_str(response.stdout.trim()).map_err(|error| {
        format!("{credential} credential failed: unexpected response ({error})")
    })?;

    payload
        .get(field)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .map(str::to_owned)
        .ok_or_else(|| format!("{credential} credential failed: response did not contain an access token"))
}

fn summarize_error(error: &str) -> String {
    let summary = error
        .split("Traceback")
        .next()
        .unwrap_or(error)
        .split("To troubleshoot")
        .next()
        .unwrap_or(error)
        .replace("Here is the traceback:", "")
        .replace('\r', " ");

    let summary = summary
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ");

    if summary.is_empty() {
        error
            .lines()
            .next()
            .unwrap_or("unknown Azure authentication error")
            .trim()
            .to_string()
    } else {
        summary
    }
}

#[cfg(test)]
mod tests {
    use std::{
        io::{Error, ErrorKind},
        process::{Command, Stdio},
        time::{Duration, Instant},
    };

    use super::{
        CliResponse, build_command, is_safe_argument, spawn_error, summarize_error,
        token_from_response, try_get_access_token, try_get_access_token_with, wait_with_timeout,
    };

    fn succeeded(stdout: &str) -> CliResponse {
        CliResponse {
            success: true,
            stdout: stdout.to_string(),
            stderr: String::new(),
        }
    }

    fn failed(stderr: &str) -> CliResponse {
        CliResponse {
            success: false,
            stdout: String::new(),
            stderr: stderr.to_string(),
        }
    }

    #[test]
    fn reads_the_token_field_of_each_cli_response_shape() {
        let azure_cli = succeeded(r#"{"accessToken":" az-token ","expiresOn":"2026-01-01"}"#);
        let azure_developer_cli = succeeded(r#"{"token":"azd-token","expiresOn":"2026-01-01"}"#);

        assert_eq!(
            token_from_response(&azure_cli, "accessToken", "Azure CLI"),
            Ok("az-token".to_string())
        );
        assert_eq!(
            token_from_response(&azure_developer_cli, "token", "Azure Developer CLI"),
            Ok("azd-token".to_string())
        );
    }

    #[test]
    fn reports_failed_exits_missing_tokens_and_invalid_json() {
        let failure = token_from_response(&failed("ERROR: Please run 'az login'"), "accessToken", "Azure CLI");
        assert_eq!(
            failure,
            Err("Azure CLI credential failed: ERROR: Please run 'az login'".to_string())
        );

        let missing = token_from_response(&succeeded(r#"{"token":""}"#), "token", "Azure Developer CLI");
        assert!(missing.is_err_and(|error| error.contains("did not contain an access token")));

        let invalid = token_from_response(&succeeded("not json"), "accessToken", "Azure CLI");
        assert!(invalid.is_err_and(|error| error.contains("unexpected response")));
    }

    #[test]
    fn falls_back_to_the_azure_developer_cli_with_the_expected_arguments() {
        let mut invocations = Vec::new();

        let token = try_get_access_token_with(Some(" tenant "), " api://app/.default ", |program, args, _| {
            invocations.push(format!("{program} {}", args.join(" ")));
            Ok(match program {
                "az" => failed("ERROR: Please run 'az login'"),
                _ => succeeded(r#"{"token":"azd-token"}"#),
            })
        });

        assert_eq!(token, Ok(Some("azd-token".to_string())));
        assert_eq!(
            invocations,
            [
                "az account get-access-token --scope api://app/.default --output json --tenant tenant",
                "azd auth token --scope api://app/.default --output json --tenant-id tenant",
            ]
        );
    }

    #[test]
    fn stops_at_the_azure_cli_token_and_joins_errors_when_both_fail() {
        let mut calls = 0;
        let token = try_get_access_token_with(None, "scope", |_, _, _| {
            calls += 1;
            Ok(succeeded(r#"{"accessToken":"az-token"}"#))
        });
        assert_eq!((token, calls), (Ok(Some("az-token".to_string())), 1));

        let error = try_get_access_token_with(None, "scope", |_, _, credential| {
            Err(format!("{credential} unavailable"))
        });
        assert_eq!(
            error,
            Err("Azure CLI unavailable\nAzure Developer CLI unavailable".to_string())
        );
    }

    fn spawn_piped(program: &str, args: &[&str]) -> std::process::Child {
        Command::new(program)
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("test process should start")
    }

    #[test]
    fn kills_a_process_that_outlives_the_timeout() {
        let child = if cfg!(windows) {
            spawn_piped("ping", &["-n", "30", "127.0.0.1"])
        } else {
            spawn_piped("sleep", &["30"])
        };
        let started = Instant::now();

        assert_eq!(
            wait_with_timeout(child, Duration::from_millis(200)).expect("waiting should work"),
            None
        );
        assert!(started.elapsed() < Duration::from_secs(10));
    }

    #[test]
    fn times_out_when_a_descendant_keeps_the_output_pipe_open() {
        // The parent exits straight away, but the background process inherits its stdout.
        let child = if cfg!(windows) {
            spawn_piped("cmd", &["/C", "start /b ping -n 6 127.0.0.1 & echo token"])
        } else {
            spawn_piped("sh", &["-c", "sleep 5 & echo token"])
        };
        let started = Instant::now();

        assert_eq!(
            wait_with_timeout(child, Duration::from_millis(500)).expect("waiting should work"),
            None
        );
        assert!(started.elapsed() < Duration::from_secs(4));
    }

    #[test]
    fn collects_the_output_of_a_process_that_finishes_in_time() {
        let child = if cfg!(windows) {
            spawn_piped("cmd", &["/C", "echo token"])
        } else {
            spawn_piped("sh", &["-c", "echo token"])
        };

        let response = wait_with_timeout(child, Duration::from_secs(10))
            .expect("waiting should work")
            .expect("the process should finish in time");

        assert!(response.success);
        assert_eq!(response.stdout.trim(), "token");
    }

    #[cfg(windows)]
    #[test]
    fn runs_the_cli_through_the_command_interpreter_on_windows() {
        let command = build_command("az", &["account", "get-access-token"]);

        assert_eq!(command.get_program(), "cmd");
        assert_eq!(
            command.get_args().collect::<Vec<_>>(),
            ["/C", "az", "account", "get-access-token"]
        );
    }

    #[cfg(not(windows))]
    #[test]
    fn runs_the_cli_directly_outside_windows() {
        let command = build_command("az", &["account", "get-access-token"]);

        assert_eq!(command.get_program(), "az");
        assert_eq!(
            command.get_args().collect::<Vec<_>>(),
            ["account", "get-access-token"]
        );
    }

    #[test]
    fn summarize_error_removes_traceback_noise() {
        let error = "AzureCliCredential authentication failed. ERROR: The command failed with an unexpected error. Here is the traceback:\nTraceback (most recent call last):\n  File ...\nTo troubleshoot, visit https://aka.ms/azsdk/rust/identity/troubleshoot#azure-cli";

        assert_eq!(
            summarize_error(error),
            "AzureCliCredential authentication failed. ERROR: The command failed with an unexpected error."
        );
    }

    #[test]
    fn only_a_missing_executable_is_reported_as_not_found_on_path() {
        let missing = Error::from(ErrorKind::NotFound);
        let denied = Error::from(ErrorKind::PermissionDenied);

        assert!(spawn_error("az", "Azure CLI", &missing).contains("`az` was not found on PATH"));
        let message = spawn_error("az", "Azure CLI", &denied);
        assert!(message.contains("could not start `az`"));
        assert!(!message.contains("PATH"));
    }

    #[test]
    fn blank_scope_does_not_invoke_any_credential() {
        assert_eq!(try_get_access_token(None, "   "), Ok(None));
    }

    #[test]
    fn rejects_arguments_containing_command_syntax() {
        assert!(is_safe_argument("https://management.azure.com/.default"));
        assert!(is_safe_argument("72f988bf-86f1-41af-91ab-2d7cd011db47"));
        assert!(!is_safe_argument("scope & whoami"));
        assert!(!is_safe_argument(""));
    }
}
