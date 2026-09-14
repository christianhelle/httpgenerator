//! Azure Entra ID access token acquisition.
//!
//! Tokens are obtained by invoking the Azure CLI (`az`) and the Azure Developer CLI (`azd`)
//! directly, which is what the Azure identity SDKs do for these credential types. Shelling out
//! keeps the dependency surface of this crate to the standard library plus JSON parsing.

use std::process::{Command, Output};

use serde_json::Value;

pub fn try_get_access_token(
    tenant_id: Option<&str>,
    scope: &str,
) -> Result<Option<String>, String> {
    let tenant_id = tenant_id
        .map(str::trim)
        .filter(|tenant_id| !tenant_id.is_empty());
    let scope = scope.trim();

    if scope.is_empty() {
        return Ok(None);
    }

    let mut errors = Vec::new();

    match get_token_with_azure_cli(tenant_id, scope) {
        Ok(token) => return Ok(Some(token)),
        Err(error) => errors.push(error),
    }

    match get_token_with_azure_developer_cli(tenant_id, scope) {
        Ok(token) => return Ok(Some(token)),
        Err(error) => errors.push(error),
    }

    Err(errors.join("\n"))
}

fn get_token_with_azure_cli(tenant_id: Option<&str>, scope: &str) -> Result<String, String> {
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

    let output = run("az", &args, "Azure CLI")?;

    token_from_output(&output, "accessToken", "Azure CLI")
}

fn get_token_with_azure_developer_cli(
    tenant_id: Option<&str>,
    scope: &str,
) -> Result<String, String> {
    let mut args = vec!["auth", "token", "--scope", scope, "--output", "json"];

    if let Some(tenant_id) = tenant_id {
        args.push("--tenant-id");
        args.push(tenant_id);
    }

    let output = run("azd", &args, "Azure Developer CLI")?;

    token_from_output(&output, "token", "Azure Developer CLI")
}

fn run(program: &str, args: &[&str], credential: &str) -> Result<Output, String> {
    for argument in args {
        if !is_safe_argument(argument) {
            return Err(format!(
                "{credential} credential initialization failed: argument {argument:?} contains unsupported characters"
            ));
        }
    }

    build_command(program, args)
        .output()
        .map_err(|error| spawn_error(program, credential, &error))
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

fn token_from_output(output: &Output, field: &str, credential: &str) -> Result<String, String> {
    let stdout = String::from_utf8_lossy(&output.stdout);

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let details = if stderr.trim().is_empty() {
            stdout.as_ref()
        } else {
            stderr.as_ref()
        };

        return Err(format!(
            "{credential} credential failed: {}",
            summarize_error(details)
        ));
    }

    let payload: Value = serde_json::from_str(stdout.trim()).map_err(|error| {
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

    use super::{is_safe_argument, spawn_error, summarize_error, try_get_access_token};

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
