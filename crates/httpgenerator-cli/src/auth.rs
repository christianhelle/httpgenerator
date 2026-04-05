use azure_core::credentials::TokenCredential;
use azure_identity::{
    AzureCliCredential, AzureCliCredentialOptions, AzureDeveloperCliCredential,
    AzureDeveloperCliCredentialOptions,
};
use pollster::block_on;

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
    let credential = AzureCliCredential::new(Some(AzureCliCredentialOptions {
        subscription: None,
        tenant_id: tenant_id.map(str::to_owned),
        executor: None,
    }))
    .map_err(|error| {
        format!(
            "Azure CLI credential initialization failed: {}",
            summarize_error(&error.to_string())
        )
    })?;

    block_on(credential.get_token(&[scope], None))
        .map(|token| token.token.secret().to_string())
        .map_err(|error| {
            format!(
                "Azure CLI credential failed: {}",
                summarize_error(&error.to_string())
            )
        })
}

fn get_token_with_azure_developer_cli(
    tenant_id: Option<&str>,
    scope: &str,
) -> Result<String, String> {
    let credential = AzureDeveloperCliCredential::new(Some(AzureDeveloperCliCredentialOptions {
        executor: None,
        tenant_id: tenant_id.map(str::to_owned),
    }))
    .map_err(|error| {
        format!(
            "Azure Developer CLI credential initialization failed: {}",
            summarize_error(&error.to_string())
        )
    })?;

    block_on(credential.get_token(&[scope], None))
        .map(|token| token.token.secret().to_string())
        .map_err(|error| {
            format!(
                "Azure Developer CLI credential failed: {}",
                summarize_error(&error.to_string())
            )
        })
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
    use super::summarize_error;

    #[test]
    fn summarize_error_removes_traceback_noise() {
        let error = "AzureCliCredential authentication failed. ERROR: The command failed with an unexpected error. Here is the traceback:\nTraceback (most recent call last):\n  File ...\nTo troubleshoot, visit https://aka.ms/azsdk/rust/identity/troubleshoot#azure-cli";

        assert_eq!(
            summarize_error(error),
            "AzureCliCredential authentication failed. ERROR: The command failed with an unexpected error."
        );
    }
}
