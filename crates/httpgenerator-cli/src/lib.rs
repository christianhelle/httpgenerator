use std::{
    error::Error,
    fmt, fs,
    path::{Path, PathBuf},
    sync::mpsc,
    thread,
    time::Duration,
};

use httpgenerator_core::{GeneratorSettings, HttpFile, generate_http_files};
use httpgenerator_openapi::{
    OpenApiInspection, OpenApiSpecificationVersion, inspect_document, load_and_normalize_document,
};

use crate::args::CliArgs;

pub mod args;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionSummary {
    pub output_folder: PathBuf,
    pub files: Vec<PathBuf>,
    pub validation: Option<OpenApiInspection>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliError {
    MissingInput,
    AzureAuthNotImplemented,
    InspectOpenApi(String),
    LoadOpenApi(String),
    UnsupportedValidationVersion {
        version: OpenApiSpecificationVersion,
    },
    CreateOutputDirectory {
        path: PathBuf,
        reason: String,
    },
    WriteFiles {
        path: PathBuf,
        reason: String,
    },
    WriteTimeout {
        seconds: u64,
    },
    WriteChannelClosed,
}

impl fmt::Display for CliError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingInput => write!(
                formatter,
                "missing OpenAPI input path or URL; run with --help for usage"
            ),
            Self::AzureAuthNotImplemented => write!(
                formatter,
                "Azure Entra ID token acquisition is not implemented in the Rust CLI yet; pass --authorization-header directly for now"
            ),
            Self::InspectOpenApi(reason) => write!(formatter, "{reason}"),
            Self::LoadOpenApi(reason) => write!(formatter, "{reason}"),
            Self::UnsupportedValidationVersion { version } => write!(
                formatter,
                "{version} documents are not supported by CLI validation yet; retry with --skip-validation"
            ),
            Self::CreateOutputDirectory { path, reason } => write!(
                formatter,
                "failed to create output directory '{}': {reason}",
                path.display()
            ),
            Self::WriteFiles { path, reason } => write!(
                formatter,
                "failed to write generated file '{}': {reason}",
                path.display()
            ),
            Self::WriteTimeout { seconds } => write!(
                formatter,
                "timed out after {seconds} second(s) while writing generated files"
            ),
            Self::WriteChannelClosed => write!(
                formatter,
                "file writing worker stopped before reporting a result"
            ),
        }
    }
}

impl Error for CliError {}

pub fn execute(args: CliArgs) -> Result<ExecutionSummary, CliError> {
    let open_api_path = args.open_api_path.clone().ok_or(CliError::MissingInput)?;

    if args.authorization_header.is_none()
        && (args.azure_scope.is_some() || args.azure_tenant_id.is_some())
    {
        return Err(CliError::AzureAuthNotImplemented);
    }

    let validation = validate_openapi_document(&open_api_path, args.skip_validation)?;
    let document = load_and_normalize_document(&open_api_path)
        .map_err(|error| CliError::LoadOpenApi(error.to_string()))?;
    let settings = GeneratorSettings {
        open_api_path: open_api_path.clone(),
        authorization_header: args.authorization_header.clone(),
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
    let output_folder = PathBuf::from(&args.output_folder);
    let files = write_files(&output_folder, result.files, args.timeout)?;

    Ok(ExecutionSummary {
        output_folder,
        files,
        validation,
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

fn write_files(
    output_folder: &Path,
    files: Vec<HttpFile>,
    timeout_seconds: u64,
) -> Result<Vec<PathBuf>, CliError> {
    if !output_folder.exists() {
        fs::create_dir_all(output_folder).map_err(|error| CliError::CreateOutputDirectory {
            path: output_folder.to_path_buf(),
            reason: error.to_string(),
        })?;
    }

    let output_folder = output_folder.to_path_buf();
    let (sender, receiver) = mpsc::channel();

    thread::spawn({
        let output_folder = output_folder.clone();
        move || {
            let result = write_files_worker(&output_folder, files);
            let _ = sender.send(result);
        }
    });

    receiver
        .recv_timeout(Duration::from_secs(timeout_seconds))
        .map_err(|error| match error {
            mpsc::RecvTimeoutError::Timeout => CliError::WriteTimeout {
                seconds: timeout_seconds,
            },
            mpsc::RecvTimeoutError::Disconnected => CliError::WriteChannelClosed,
        })?
}

fn write_files_worker(
    output_folder: &Path,
    files: Vec<HttpFile>,
) -> Result<Vec<PathBuf>, CliError> {
    let mut written_paths = Vec::with_capacity(files.len());

    for file in files {
        let path = output_folder.join(&file.filename);
        fs::write(&path, file.content).map_err(|error| CliError::WriteFiles {
            path: path.clone(),
            reason: error.to_string(),
        })?;
        written_paths.push(path);
    }

    Ok(written_paths)
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    use crate::{CliError, ExecutionSummary, args::CliArgs, execute};

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
            open_api_path: Some(
                PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("..")
                    .join("..")
                    .join("test")
                    .join("OpenAPI")
                    .join("v3.0")
                    .join("petstore.json")
                    .to_string_lossy()
                    .into_owned(),
            ),
            output_folder: output_folder.to_string_lossy().into_owned(),
            ..CliArgs::default()
        }
    }

    fn webhook31_args(output_folder: PathBuf) -> CliArgs {
        CliArgs {
            open_api_path: Some(
                PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("..")
                    .join("..")
                    .join("test")
                    .join("OpenAPI")
                    .join("v3.1")
                    .join("webhook-example.json")
                    .to_string_lossy()
                    .into_owned(),
            ),
            output_folder: output_folder.to_string_lossy().into_owned(),
            ..CliArgs::default()
        }
    }

    impl Default for CliArgs {
        fn default() -> Self {
            Self {
                open_api_path: None,
                output_folder: "./".to_string(),
                no_logging: false,
                skip_validation: false,
                authorization_header: None,
                authorization_header_from_environment_variable: false,
                authorization_header_variable_name: "authorization".to_string(),
                content_type: "application/json".to_string(),
                base_url: None,
                output_type: super::args::OutputTypeArg::OneRequestPerFile,
                azure_scope: None,
                azure_tenant_id: None,
                timeout: 120,
                generate_intellij_tests: false,
                custom_headers: Vec::new(),
                skip_headers: false,
            }
        }
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

    #[test]
    fn execute_respects_one_file_mode_and_custom_headers() {
        let output_folder = temp_output_dir("onefile");
        let summary = execute(CliArgs {
            output_type: super::args::OutputTypeArg::OneFile,
            generate_intellij_tests: true,
            custom_headers: vec!["X-API-Key: test123".to_string()],
            ..petstore_args(output_folder)
        })
        .unwrap();

        assert_eq!(summary.files.len(), 1);
        assert!(summary.validation.is_some());
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
                version: httpgenerator_openapi::OpenApiSpecificationVersion::OpenApi31,
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
        assert!(summary.files.is_empty());

        cleanup(&summary);
    }

    #[test]
    fn execute_rejects_azure_auth_until_ported() {
        let error = execute(CliArgs {
            open_api_path: Some("test/OpenAPI/v3.0/petstore.json".to_string()),
            azure_scope: Some("api://example/.default".to_string()),
            azure_tenant_id: Some("tenant-id".to_string()),
            ..CliArgs::default()
        })
        .unwrap_err();

        assert_eq!(error, CliError::AzureAuthNotImplemented);
    }
}
