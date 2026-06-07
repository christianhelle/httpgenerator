use std::ffi::OsString;

use crate::args::{CliArgs, OutputTypeArg};

use super::{
    MemoryTelemetrySink, TelemetryEvent, TelemetryRecorder,
    redaction::{feature_usage_names, redacted_command_line},
    sink::TelemetrySink,
};

#[test]
fn feature_usage_matches_dotnet_rules_for_default_args() {
    assert_eq!(
        feature_usage_names(&CliArgs::default()),
        vec![
            "authorization-header-variable-name",
            "content-type",
            "output-type",
            "timeout",
        ]
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>()
    );
}

#[test]
fn feature_usage_tracks_enabled_and_configured_options() {
    let args = CliArgs {
        skip_validation: true,
        authorization_header: Some("Bearer secret-token".to_string()),
        authorization_header_from_environment_variable: true,
        base_url: Some("https://api.example.com".to_string()),
        output_type: OutputTypeArg::OneFilePerTag,
        azure_scope: Some("api://example/.default".to_string()),
        azure_tenant_id: Some("tenant-id".to_string()),
        generate_intellij_tests: true,
        custom_headers: vec!["X-Test: 1".to_string()],
        skip_headers: true,
        ..CliArgs::default()
    };

    assert_eq!(
        feature_usage_names(&args),
        vec![
            "skip-validation",
            "authorization-header",
            "load-authorization-header-from-environment",
            "authorization-header-variable-name",
            "content-type",
            "base-url",
            "output-type",
            "azure-scope",
            "azure-tenant-id",
            "timeout",
            "generate-intellij-tests",
            "custom-header",
            "skip-headers",
        ]
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>()
    );
}

#[test]
fn redacted_command_line_hides_authorization_headers_and_normalizes_program_name() {
    let command_line = redacted_command_line(&[
        OsString::from(r"C:\tools\httpgenerator.exe"),
        OsString::from("petstore.json"),
        OsString::from("--authorization-header"),
        OsString::from("Bearer secret-token"),
        OsString::from("--output"),
        OsString::from("."),
    ]);

    assert_eq!(
        command_line,
        "httpgenerator petstore.json --authorization-header [REDACTED] --output ."
    );
    assert!(!command_line.contains("secret-token"));
}

#[test]
fn record_error_captures_redacted_settings_and_support_context() {
    let args = CliArgs {
        open_api_path: Some("petstore.json".to_string()),
        authorization_header: Some("Bearer secret-token".to_string()),
        custom_headers: vec![
            "Authorization: Basic secret-value".to_string(),
            "X-Test: 1".to_string(),
        ],
        ..CliArgs::default()
    };
    let raw_args = [
        OsString::from(r"C:\tools\httpgenerator.exe"),
        OsString::from("petstore.json"),
        OsString::from("--authorization-header"),
        OsString::from("Bearer secret-token"),
    ];
    let mut memory_sink = MemoryTelemetrySink::default();
    let mut recorder =
        TelemetryRecorder::from_cli_args(&raw_args, &args, memory_sink.into());

    recorder.record_error(&args, "CliError", "boom");

    let sink = recorder.into_sink();
    assert_eq!(sink.events().len(), 1);

    let TelemetryEvent::Error(event) = &sink.events()[0] else {
        panic!("expected an error event");
    };

    assert_eq!(event.error_type, "CliError");
    assert_eq!(event.message, "boom");
    assert_eq!(event.support_key.len(), 7);
    assert_eq!(event.anonymous_identity.len(), 44);
    assert_eq!(
        event.command_line,
        "httpgenerator petstore.json --authorization-header [REDACTED]"
    );
    assert!(
        event
            .settings_json
            .contains(r#""authorizationHeader":"[REDACTED]""#)
    );
    assert!(
        event
            .settings_json
            .contains(r#""customHeaders":["Authorization: [REDACTED]","X-Test: 1"]"#)
    );
    assert!(!event.settings_json.contains("secret-token"));
    assert!(!event.command_line.contains("secret-token"));
}

#[test]
fn no_logging_disables_feature_and_error_events() {
    let args = CliArgs {
        no_logging: true,
        skip_headers: true,
        ..CliArgs::default()
    };
    let mut recorder = TelemetryRecorder::from_cli_args(
        &[OsString::from("httpgenerator")],
        &args,
        MemoryTelemetrySink::default().into(),
    );

    recorder.record_feature_usage(&args);
    recorder.record_error(&args, "CliError", "boom");

    assert!(recorder.into_sink().events().is_empty());
}

#[test]
fn record_feature_usage_emits_ordered_feature_events() {
    let args = CliArgs {
        skip_validation: true,
        custom_headers: vec!["X-Test: 1".to_string()],
        ..CliArgs::default()
    };
    let mut recorder = TelemetryRecorder::from_cli_args(
        &[OsString::from("httpgenerator")],
        &args,
        MemoryTelemetrySink::default().into(),
    );

    recorder.record_feature_usage(&args);

    let sink = recorder.into_sink();
    let names = sink
        .events()
        .iter()
        .map(|event| match event {
            TelemetryEvent::FeatureUsage(event) => event.feature_name.as_str(),
            TelemetryEvent::Error(_) => panic!("expected feature events"),
        })
        .collect::<Vec<_>>();

    assert_eq!(
        names,
        vec![
            "skip-validation",
            "authorization-header-variable-name",
            "content-type",
            "output-type",
            "timeout",
            "custom-header",
        ]
    );
}
