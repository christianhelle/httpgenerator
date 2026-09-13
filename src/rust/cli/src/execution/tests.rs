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

use super::orchestrator::execute_with;
use super::{execute, execute_with_observer, should_attempt_azure_auth};

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
        inspection: &crate::OpenApiInspection,
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

    fn reference_warnings(&mut self, warnings: &[String]) {
        for warning in warnings {
            self.events.push(format!("reference_warning:{warning}"));
        }
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
    assert_eq!(summary.files.len(), 1);
    assert!(summary.files[0].ends_with("PostNewPet.http"));
    let content = fs::read_to_string(&summary.files[0]).unwrap();
    assert!(content.contains("### Request: POST /newPet"));
    assert!(content.contains("POST {{baseUrl}}/newPet"));

    cleanup(&summary);
}

#[test]
fn execute_writes_webhook_files_per_tag_for_openapi31_with_skip_validation() {
    let output_folder = temp_output_dir("openapi31-webhook-per-tag");
    let summary = execute(CliArgs {
        skip_validation: true,
        output_type: OutputTypeArg::OneFilePerTag,
        ..webhook31_args(output_folder)
    })
    .unwrap();

    assert!(summary.validation.is_none());
    assert_eq!(summary.azure_auth, AzureAuthStatus::NotRequested);
    assert_eq!(summary.files.len(), 1);
    assert!(summary.files[0].ends_with("Webhooks.http"));
    let content = fs::read_to_string(&summary.files[0]).unwrap();
    assert!(content.contains("### Request: POST /newPet"));
    assert!(content.contains("POST {{baseUrl}}/newPet"));

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

#[test]
fn execute_never_attempts_azure_auth_when_the_specification_cannot_be_read() {
    for skip_validation in [false, true] {
        let output_folder = temp_output_dir("unreadable-specification");
        let token_requested = std::cell::Cell::new(false);
        let mut observer = NoopExecutionObserver;

        let error = execute_with(
            CliArgs {
                open_api_path: Some(test_fixture_path("v3.0", "does-not-exist.json")),
                output_folder: output_folder.to_string_lossy().into_owned(),
                skip_validation,
                azure_scope: Some("api://example/.default".to_string()),
                ..CliArgs::default()
            },
            &mut observer,
            |_, _| {
                token_requested.set(true);
                Ok(Some("test-token".to_string()))
            },
        )
        .unwrap_err();

        if skip_validation {
            assert!(matches!(error, CliError::LoadOpenApi(_)), "{error:?}");
        } else {
            assert!(matches!(error, CliError::InspectOpenApi(_)), "{error:?}");
        }
        assert!(!token_requested.get(), "skip_validation = {skip_validation}");
        assert!(!output_folder.exists());
    }
}

/// The stats the legacy .NET CLI reports for a specification, in display order: path items,
/// operations, parameters, request bodies, responses, links, callbacks and schemas.
fn displayed_stats(summary: &ExecutionSummary) -> [usize; 8] {
    let stats = summary
        .validation
        .as_ref()
        .expect("the specification should be validated")
        .stats;

    [
        stats.path_item_count,
        stats.operation_count,
        stats.parameter_count,
        stats.request_body_count,
        stats.response_count,
        stats.link_count,
        stats.callback_count,
        stats.schema_count,
    ]
}

#[test]
fn execute_reports_the_same_validation_stats_as_the_legacy_cli() {
    let multi_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../test/multi-file/petstore.yaml")
        .to_string_lossy()
        .into_owned();

    for (name, input, expected) in [
        (
            "stats-v30",
            test_fixture_path("v3.0", "petstore.json"),
            [13, 19, 17, 9, 19, 0, 0, 73],
        ),
        (
            "stats-v20",
            test_fixture_path("v2.0", "petstore.json"),
            [14, 20, 14, 9, 20, 0, 0, 67],
        ),
        ("stats-multi-file", multi_file, [13, 19, 17, 9, 19, 0, 0, 64]),
    ] {
        let output_folder = temp_output_dir(name);
        let summary = execute(CliArgs {
            open_api_path: Some(input.clone()),
            output_folder: output_folder.to_string_lossy().into_owned(),
            ..CliArgs::default()
        })
        .unwrap();

        assert_eq!(displayed_stats(&summary), expected, "{input}");

        cleanup(&summary);
    }
}

/// Writes `files` of `(name, content)` into a fresh directory and returns the path of the first.
fn specification_files(name: &str, files: &[(&str, &str)]) -> String {
    let directory = temp_output_dir(name);
    fs::create_dir_all(&directory).unwrap();
    for (file, content) in files {
        fs::write(directory.join(file), content).unwrap();
    }

    directory.join(files[0].0).to_string_lossy().into_owned()
}

const PETSTORE_WITH_MISSING_COMPONENTS: &str = r#"
openapi: 3.0.3
info:
  title: Petstore
  version: 1.0.0
paths:
  /pets:
    post:
      operationId: addPet
      requestBody:
        content:
          application/json:
            schema:
              $ref: 'missing.yaml#/components/schemas/Pet'
      responses:
        '200':
          description: ok
"#;

#[test]
fn execute_fails_validation_for_references_that_cannot_be_resolved() {
    let input = specification_files(
        "unresolved-spec",
        &[("petstore.yaml", PETSTORE_WITH_MISSING_COMPONENTS)],
    );
    let output_folder = temp_output_dir("unresolved-output");

    let error = execute(CliArgs {
        open_api_path: Some(input),
        output_folder: output_folder.to_string_lossy().into_owned(),
        ..CliArgs::default()
    })
    .unwrap_err();

    let CliError::UnresolvedReferences { references } = &error else {
        panic!("expected unresolved references, got {error:?}");
    };
    assert_eq!(references.len(), 1);
    assert!(
        references[0].contains("missing.yaml#/components/schemas/Pet"),
        "{references:?}"
    );
    assert!(error.to_string().contains("--skip-validation"), "{error}");
    assert_eq!(error.telemetry_name(), "UnresolvedReferences");
    assert!(!output_folder.exists());
}

#[test]
fn execute_warns_about_unresolved_references_when_validation_is_skipped() {
    let input = specification_files(
        "unresolved-skipped-spec",
        &[("petstore.yaml", PETSTORE_WITH_MISSING_COMPONENTS)],
    );
    let mut observer = RecordingObserver::default();

    let summary = execute_with_observer(
        CliArgs {
            open_api_path: Some(input),
            output_folder: temp_output_dir("unresolved-skipped-output")
                .to_string_lossy()
                .into_owned(),
            skip_validation: true,
            ..CliArgs::default()
        },
        &mut observer,
    )
    .unwrap();

    assert_eq!(summary.files.len(), 1);
    assert!(
        observer.events.iter().any(|event| event.starts_with("reference_warning:")
            && event.contains("missing.yaml#/components/schemas/Pet")),
        "{:?}",
        observer.events
    );

    cleanup(&summary);
}

#[test]
fn execute_warns_about_references_that_were_left_unchanged() {
    let input = specification_files(
        "circular-spec",
        &[
            (
                "tree.yaml",
                "openapi: 3.0.3\ninfo:\n  title: Tree\n  version: 1.0.0\npaths:\n  /tree:\n    get:\n      responses:\n        '200':\n          description: ok\n          content:\n            application/json:\n              schema:\n                $ref: 'node.yaml'\n",
            ),
            (
                "node.yaml",
                "type: object\nproperties:\n  children:\n    type: array\n    items:\n      $ref: 'node.yaml'\n",
            ),
        ],
    );
    let mut observer = RecordingObserver::default();

    let summary = execute_with_observer(
        CliArgs {
            open_api_path: Some(input),
            output_folder: temp_output_dir("circular-output")
                .to_string_lossy()
                .into_owned(),
            ..CliArgs::default()
        },
        &mut observer,
    )
    .unwrap();

    assert!(
        observer
            .events
            .iter()
            .any(|event| event.starts_with("reference_warning:")
                && event.contains("refers back to itself")),
        "{:?}",
        observer.events
    );

    cleanup(&summary);
}

#[test]
fn execute_generates_when_skipped_validation_leaves_references_unresolved() {
    let input = specification_files(
        "unresolved-locations-spec",
        &[(
            "petstore.yaml",
            r#"
openapi: 3.1.0
info:
  title: Petstore
  version: 1.0.0
paths:
  /pets:
    get:
      operationId: listPets
      parameters:
        - $ref: 'missing.yaml#/components/parameters/Limit'
      responses:
        '200':
          description: ok
    post:
      operationId: addPet
      requestBody:
        $ref: 'missing.yaml#/components/requestBodies/Pet'
      responses:
        '200':
          description: ok
  /owners:
    $ref: 'missing.yaml#/components/pathItems/Owners'
"#,
        )],
    );
    let mut observer = RecordingObserver::default();

    let summary = execute_with_observer(
        CliArgs {
            open_api_path: Some(input),
            output_folder: temp_output_dir("unresolved-locations-output")
                .to_string_lossy()
                .into_owned(),
            skip_validation: true,
            ..CliArgs::default()
        },
        &mut observer,
    )
    .unwrap();

    assert_eq!(summary.files.len(), 2);
    assert_eq!(
        observer
            .events
            .iter()
            .filter(|event| event.starts_with("reference_warning:"))
            .count(),
        3,
        "{:?}",
        observer.events
    );

    cleanup(&summary);
}
