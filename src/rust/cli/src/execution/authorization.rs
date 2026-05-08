use crate::{args::CliArgs, observer::AzureAuthStatus};

pub(super) fn resolve_authorization_header<F>(
    args: &CliArgs,
    acquire_token: F,
) -> (Option<String>, AzureAuthStatus)
where
    F: Fn(Option<&str>, &str) -> Result<Option<String>, String>,
{
    if let Some(authorization_header) = args
        .authorization_header
        .as_deref()
        .map(str::trim)
        .filter(|header| !header.is_empty())
    {
        return (
            Some(authorization_header.to_string()),
            AzureAuthStatus::NotRequested,
        );
    }

    let tenant_id = args
        .azure_tenant_id
        .as_deref()
        .map(str::trim)
        .filter(|tenant_id| !tenant_id.is_empty());
    let Some(scope) = args
        .azure_scope
        .as_deref()
        .map(str::trim)
        .filter(|scope| !scope.is_empty())
    else {
        return if tenant_id.is_some() {
            (
                None,
                AzureAuthStatus::Failed {
                    reason: "Azure Entra ID scope is required to acquire an authorization header."
                        .to_string(),
                },
            )
        } else {
            (None, AzureAuthStatus::NotRequested)
        };
    };

    match acquire_token(tenant_id, scope) {
        Ok(Some(token)) if !token.trim().is_empty() => (
            Some(format!("Bearer {}", token.trim())),
            AzureAuthStatus::Acquired,
        ),
        Ok(Some(_)) => (
            None,
            AzureAuthStatus::Failed {
                reason: "Azure Entra ID returned an empty access token.".to_string(),
            },
        ),
        Ok(None) => (
            None,
            AzureAuthStatus::Failed {
                reason: "Azure Entra ID did not return an access token.".to_string(),
            },
        ),
        Err(reason) => (None, AzureAuthStatus::Failed { reason }),
    }
}
