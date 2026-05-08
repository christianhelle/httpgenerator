use std::{ffi::OsString, path::PathBuf};

use httpgenerator_cli::{
    AzureAuthStatus, CliError, ExecutionObserver, ExecutionSummary, NoopTelemetrySink,
    TelemetryRecorder, args, execute, execute_with_observer, should_attempt_azure_auth,
    telemetry,
};

struct ContractObserver;

impl ExecutionObserver for ContractObserver {}

#[test]
fn lib_facade_exposes_the_intentional_public_cli_surface() {
    let _execute: fn(args::CliArgs) -> Result<ExecutionSummary, CliError> = execute;
    let _execute_with_observer:
        fn(args::CliArgs, &mut ContractObserver) -> Result<ExecutionSummary, CliError> =
        execute_with_observer::<ContractObserver>;
    let _should_attempt_azure_auth: fn(&args::CliArgs) -> bool = should_attempt_azure_auth;

    let _error = CliError::MissingInput;
    let summary = ExecutionSummary {
        output_folder: PathBuf::from("out"),
        files: vec![PathBuf::from("request.http")],
        validation: None,
        azure_auth: AzureAuthStatus::NotRequested,
    };

    assert_eq!(summary.azure_auth, AzureAuthStatus::NotRequested);
    assert_eq!(summary.files, vec![PathBuf::from("request.http")]);

    let mut cli_args = args::CliArgs::default();
    cli_args.output_type = args::OutputTypeArg::OneFile;
    assert!(!should_attempt_azure_auth(&cli_args));

    let raw_args = [OsString::from("httpgenerator")];
    let mut root_recorder: TelemetryRecorder<NoopTelemetrySink> =
        TelemetryRecorder::from_cli_args(&raw_args, &cli_args, NoopTelemetrySink);
    root_recorder.record_feature_usage(&cli_args);
    let _: NoopTelemetrySink = root_recorder.into_sink();

    struct ModuleSink;

    impl telemetry::TelemetrySink for ModuleSink {
        fn emit(&mut self, _event: telemetry::TelemetryEvent) {}
    }

    let _module_recorder =
        telemetry::TelemetryRecorder::from_cli_args(&raw_args, &cli_args, ModuleSink);

    let feature_event = telemetry::TelemetryEvent::FeatureUsage(telemetry::FeatureUsageEvent {
        feature_name: "facade-contract".to_string(),
        support_key: "support".to_string(),
        anonymous_identity: "anonymous".to_string(),
    });

    let telemetry::TelemetryEvent::FeatureUsage(feature) = feature_event else {
        unreachable!("feature event should round-trip through the public telemetry module");
    };

    assert_eq!(feature.feature_name, "facade-contract");
}
