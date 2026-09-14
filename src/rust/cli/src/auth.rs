//! Azure Entra ID access token acquisition.
//!
//! Tokens are obtained by invoking the Azure CLI (`az`) and the Azure Developer CLI (`azd`)
//! directly, which is what the Azure identity SDKs do for these credential types. Shelling out
//! keeps the dependency surface of this crate to the standard library plus JSON parsing.

use std::process::Command;

use serde_json::Value;

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

    let output = build_command(program, args)
        .output()
        .map_err(|error| spawn_error(program, credential, &error))?;

    Ok(CliResponse {
        success: output.status.success(),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    })
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
    use std::io::{Error, ErrorKind};

    use super::{
        CliResponse, build_command, is_safe_argument, spawn_error, summarize_error,
        token_from_response, try_get_access_token, try_get_access_token_with,
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
