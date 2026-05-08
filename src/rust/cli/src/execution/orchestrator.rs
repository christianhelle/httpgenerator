use std::path::PathBuf;

use httpgenerator_core::{generate_http_files, openapi::load_and_normalize_document_with_options};

use crate::{
    CliError,
    args::CliArgs,
    auth::try_get_access_token,
    observer::{ExecutionObserver, ExecutionSummary, NoopExecutionObserver},
    writer::write_files,
};

use super::{
    authorization::resolve_authorization_header, settings::build_generator_settings,
    validation::validate_openapi_document,
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

pub(crate) fn execute_with<F, O>(
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
    let settings = build_generator_settings(&args, open_api_path.clone(), authorization_header);
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
