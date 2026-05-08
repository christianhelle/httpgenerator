use std::path::PathBuf;

use httpgenerator_core::{
    GeneratorSettings, generate_http_files,
    openapi::{
        OpenApiInspection, OpenApiSpecificationVersion, inspect_document,
        load_and_normalize_document_with_options,
    },
};

use crate::{
    CliError,
    args::CliArgs,
    auth::try_get_access_token,
    observer::{AzureAuthStatus, ExecutionObserver, ExecutionSummary, NoopExecutionObserver},
    writer::write_files,
};

pub fn execute(args: CliArgs) -> Result<ExecutionSummary, CliError> {
    let mut observer = NoopExecutionObserver;
    execute_with(args, &mut observer, try_get_access_token)
}

pub fn execute_with_observer<O>(
    args: CliArgs,
    observer: &mut O,
) -> Result<ExecutionSummary, CliError>
where
    O: ExecutionObserver,
{
    execute_with(args, observer, try_get_access_token)
}

pub fn should_attempt_azure_auth(args: &CliArgs) -> bool {
    if args
        .authorization_header
        .as_deref()
        .map(str::trim)
        .is_some_and(|header| !header.is_empty())
    {
        return false;
    }

    args.azure_scope
        .as_deref()
        .map(str::trim)
        .is_some_and(|scope| !scope.is_empty())
        || args
            .azure_tenant_id
            .as_deref()
            .map(str::trim)
            .is_some_and(|tenant_id| !tenant_id.is_empty())
}

fn execute_with<F, O>(
    args: CliArgs,
    observer: &mut O,
    acquire_token: F,
) -> Result<ExecutionSummary, CliError>
where
    F: Fn(Option<&str>, &str) -> Result<Option<String>, String>,
    O: ExecutionObserver,
{
    let open_api_path = args.open_api_path.clone().ok_or(CliError::MissingInput)?;
    if !args.skip_validation {
        observer.validation_started();
    }

    let validation = validate_openapi_document(&open_api_path, args.skip_validation)?;
    if let Some(inspection) = &validation {
        observer.validation_succeeded(inspection);
    }

    let should_attempt_azure_auth = should_attempt_azure_auth(&args);
    if should_attempt_azure_auth {
        observer.azure_auth_started();
    }

    let (authorization_header, azure_auth) = resolve_authorization_header(&args, acquire_token);
    if should_attempt_azure_auth {
        observer.azure_auth_finished(&azure_auth);
    }

    let document = load_and_normalize_document_with_options(&open_api_path, args.skip_validation)
        .map_err(|error| CliError::LoadOpenApi(error.to_string()))?;
    let settings = GeneratorSettings {
        open_api_path: open_api_path.clone(),
        authorization_header,
        authorization_header_from_environment_variable: args
            .authorization_header_from_environment_variable,
        authorization_header_variable_name: args.authorization_header_variable_name.clone(),
        content_type: args.content_type.clone(),
        base_url: args.base_url.clone(),
        output_type: args.output_type.into(),
        timeout: args.timeout,
        generate_intellij_tests: args.generate_intellij_tests,
        custom_headers: args.custom_headers.clone(),
        skip_headers: args.skip_headers,
    };
    let result = generate_http_files(&settings, &document);
    observer.file_writing_started(result.files.len());
    let output_folder = PathBuf::from(&args.output_folder);
    let files = write_files(&output_folder, result.files, args.timeout)?;
    observer.files_written(&files);

    Ok(ExecutionSummary {
        output_folder,
        files,
        validation,
        azure_auth,
    })
}

fn validate_openapi_document(
    open_api_path: &str,
    skip_validation: bool,
) -> Result<Option<OpenApiInspection>, CliError> {
    if skip_validation {
        return Ok(None);
    }

    let inspection = inspect_document(open_api_path)
        .map_err(|error| CliError::InspectOpenApi(error.to_string()))?;

    if inspection.specification_version == OpenApiSpecificationVersion::OpenApi31 {
        return Err(CliError::UnsupportedValidationVersion {
            version: inspection.specification_version,
        });
    }

    Ok(Some(inspection))
}

fn resolve_authorization_header<F>(
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

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    use crate::{
        AzureAuthStatus, CliError, ExecutionObserver, ExecutionSummary,
        args::{CliArgs, OutputTypeArg},
        observer::NoopExecutionObserver,
    };

    use super::{execute, execute_with, should_attempt_azure_auth};

    fn temp_output_dir(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "httpgenerator-rust-cli-tests-{name}-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ))
    }

    fn petstore_args(output_folder: PathBuf) -> CliArgs {
        CliArgs {
            open_api_path: Some(test_fixture_path("v3.0", "petstore.json")),
            output_folder: output_folder.to_string_lossy().into_owned(),
            ..CliArgs::default()
        }
    }

    fn webhook31_args(output_folder: PathBuf) -> CliArgs {
        CliArgs {
            open_api_path: Some(test_fixture_path("v3.1", "webhook-example.json")),
            output_folder: output_folder.to_string_lossy().into_owned(),
            ..CliArgs::default()
        }
    }

    fn non_oauth31_args(output_folder: PathBuf) -> CliArgs {
        CliArgs {
            open_api_path: Some(test_fixture_path("v3.1", "non-oauth-scopes.json")),
            output_folder: output_folder.to_string_lossy().into_owned(),
            ..CliArgs::default()
        }
    }

    fn test_fixture_path(version: &str, file_name: &str) -> String {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("..")
            .join("test")
            .join("OpenAPI")
            .join(version)
            .join(file_name)
            .to_string_lossy()
            .into_owned()
    }

    fn cleanup(summary: &ExecutionSummary) {
        let _ = fs::remove_dir_all(&summary.output_folder);
    }

    #[test]
    fn execute_writes_petstore_files() {
        let output_folder = temp_output_dir("petstore");
        let summary = execute(petstore_args(output_folder)).unwrap();

        assert_eq!(summary.files.len(), 19);
        assert!(
            summary
                .validation
                .as_ref()
                .is_some_and(|inspection| inspection.stats.path_item_count > 0)
        );
        assert_eq!(summary.azure_auth, AzureAuthStatus::NotRequested);
        assert!(
            summary
                .files
                .iter()
                .any(|path| path.ends_with("PutUpdatePet.http"))
        );
        assert!(
            summary
                .files
                .iter()
                .any(|path| path.ends_with("GetLoginUser.http"))
        );

        cleanup(&summary);
    }

    #[derive(Default)]
    struct RecordingObserver {
        events: Vec<String>,
    }

    impl ExecutionObserver for RecordingObserver {
        fn validation_started(&mut self) {
            self.events.push("validation_started".to_string());
        }

        fn validation_succeeded(
            &mut self,
            inspection: &httpgenerator_core::openapi::OpenApiInspection,
        ) {
            self.events.push(format!(
                "validation_succeeded:{}",
                inspection.specification_version
            ));
        }

        fn file_writing_started(&mut self, file_count: usize) {
            self.events
                .push(format!("file_writing_started:{file_count}"));
        }

        fn files_written(&mut self, paths: &[PathBuf]) {
            self.events.push(format!("files_written:{}", paths.len()));
        }
    }

    #[test]
    fn execute_notifies_observer_in_cli_lifecycle_order() {
        let output_folder = temp_output_dir("observer-order");
        let mut observer = RecordingObserver::default();
        let summary = execute_with(
            petstore_args(output_folder),
            &mut observer,
            |_tenant_id, _scope| Ok(None),
        )
        .unwrap();

        assert_eq!(
            observer.events,
            vec![
                "validation_started".to_string(),
                "validation_succeeded:OpenAPI 3.0.x".to_string(),
                "file_writing_started:19".to_string(),
                "files_written:19".to_string(),
            ]
        );

        cleanup(&summary);
    }

    #[test]
    fn execute_respects_one_file_mode_and_custom_headers() {
        let output_folder = temp_output_dir("onefile");
        let summary = execute(CliArgs {
            output_type: OutputTypeArg::OneFile,
            generate_intellij_tests: true,
            custom_headers: vec!["X-API-Key: test123".to_string()],
            ..petstore_args(output_folder)
        })
        .unwrap();

        assert_eq!(summary.files.len(), 1);
        assert!(summary.validation.is_some());
        assert_eq!(summary.azure_auth, AzureAuthStatus::NotRequested);
        let content = fs::read_to_string(&summary.files[0]).unwrap();
        assert!(content.contains("X-API-Key: test123"));
        assert!(content.contains("> {%"));
        assert!(content.contains("### Request: PUT /pet"));

        cleanup(&summary);
    }

    #[test]
    fn execute_rejects_openapi31_without_skip_validation() {
        let output_folder = temp_output_dir("openapi31-validation");
        let error = execute(webhook31_args(output_folder)).unwrap_err();

        assert_eq!(
            error,
            CliError::UnsupportedValidationVersion {
                version: httpgenerator_core::openapi::OpenApiSpecificationVersion::OpenApi31,
            }
        );
    }

    #[test]
    fn execute_allows_openapi31_with_skip_validation() {
        let output_folder = temp_output_dir("openapi31-skip");
        let summary = execute(CliArgs {
            skip_validation: true,
            ..webhook31_args(output_folder)
        })
        .unwrap();

        assert!(summary.validation.is_none());
        assert_eq!(summary.azure_auth, AzureAuthStatus::NotRequested);
        assert!(summary.files.is_empty());

        cleanup(&summary);
    }

    #[test]
    fn execute_allows_invalid_openapi31_with_skip_validation() {
        let output_folder = temp_output_dir("openapi31-invalid-skip");
        let summary = execute(CliArgs {
            skip_validation: true,
            ..non_oauth31_args(output_folder)
        })
        .unwrap();

        assert!(summary.validation.is_none());
        assert_eq!(summary.azure_auth, AzureAuthStatus::NotRequested);
        assert_eq!(summary.files.len(), 1);
        let content = fs::read_to_string(&summary.files[0]).unwrap();
        assert!(content.contains("### Request: GET /users"));

        cleanup(&summary);
    }

    #[test]
    fn execute_uses_acquired_azure_token_as_authorization_header() {
        let output_folder = temp_output_dir("azure-auth");
        let mut observer = NoopExecutionObserver;
        let summary = execute_with(
            CliArgs {
                azure_scope: Some("api://example/.default".to_string()),
                azure_tenant_id: Some("tenant-id".to_string()),
                ..petstore_args(output_folder)
            },
            &mut observer,
            |tenant_id, scope| {
                assert_eq!(tenant_id, Some("tenant-id"));
                assert_eq!(scope, "api://example/.default");
                Ok(Some("test-token".to_string()))
            },
        )
        .unwrap();

        assert_eq!(summary.azure_auth, AzureAuthStatus::Acquired);
        let content = fs::read_to_string(&summary.files[0]).unwrap();
        assert!(content.contains("@authorization = Bearer test-token"));
        assert!(content.contains("Authorization: {{authorization}}"));

        cleanup(&summary);
    }

    #[test]
    fn execute_continues_when_azure_token_lookup_fails() {
        let output_folder = temp_output_dir("azure-auth-failure");
        let mut observer = NoopExecutionObserver;
        let summary = execute_with(
            CliArgs {
                azure_scope: Some("api://example/.default".to_string()),
                azure_tenant_id: Some("tenant-id".to_string()),
                ..petstore_args(output_folder)
            },
            &mut observer,
            |tenant_id, scope| {
                assert_eq!(tenant_id, Some("tenant-id"));
                assert_eq!(scope, "api://example/.default");
                Err("Azure CLI credential failed: not logged in".to_string())
            },
        )
        .unwrap();

        assert_eq!(
            summary.azure_auth,
            AzureAuthStatus::Failed {
                reason: "Azure CLI credential failed: not logged in".to_string(),
            }
        );
        let content = fs::read_to_string(&summary.files[0]).unwrap();
        assert!(!content.contains("@authorization ="));
        assert!(!content.contains("Authorization: {{authorization}}"));

        cleanup(&summary);
    }

    #[test]
    fn execute_continues_when_azure_scope_is_missing() {
        let output_folder = temp_output_dir("azure-auth-missing-scope");
        let mut observer = NoopExecutionObserver;
        let summary = execute_with(
            CliArgs {
                azure_tenant_id: Some("tenant-id".to_string()),
                ..petstore_args(output_folder)
            },
            &mut observer,
            |_tenant_id, _scope| panic!("token provider should not run without a scope"),
        )
        .unwrap();

        assert_eq!(
            summary.azure_auth,
            AzureAuthStatus::Failed {
                reason: "Azure Entra ID scope is required to acquire an authorization header."
                    .to_string(),
            }
        );

        cleanup(&summary);
    }

    #[test]
    fn should_attempt_azure_auth_only_when_scope_or_tenant_is_present_without_header() {
        let mut args = CliArgs {
            open_api_path: None,
            output_folder: "./".to_string(),
            no_logging: false,
            skip_validation: false,
            authorization_header: None,
            authorization_header_from_environment_variable: false,
            authorization_header_variable_name: "authorization".to_string(),
            content_type: "application/json".to_string(),
            base_url: None,
            output_type: OutputTypeArg::OneRequestPerFile,
            azure_scope: None,
            azure_tenant_id: None,
            timeout: 120,
            generate_intellij_tests: false,
            custom_headers: Vec::new(),
            skip_headers: false,
        };

        assert!(!should_attempt_azure_auth(&args));

        args.azure_scope = Some("api://example/.default".to_string());
        assert!(should_attempt_azure_auth(&args));

        args.authorization_header = Some("Bearer token".to_string());
        assert!(!should_attempt_azure_auth(&args));
    }
}
