use crate::args::{CliArgs, OutputTypeArg};
use httpgenerator_core::{
    anonymous_identity, redact_authorization_headers, support_key_from_anonymous_identity,
};
use serde_json::{Map, Value};
use std::ffi::OsString;

const REDACTED: &str = "[REDACTED]";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelemetryContext {
    pub support_key: String,
    pub anonymous_identity: String,
    pub command_line: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureUsageEvent {
    pub feature_name: String,
    pub support_key: String,
    pub anonymous_identity: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ErrorEvent {
    pub error_type: String,
    pub message: String,
    pub support_key: String,
    pub anonymous_identity: String,
    pub command_line: String,
    pub settings_json: String,
    pub settings: Map<String, Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TelemetryEvent {
    FeatureUsage(FeatureUsageEvent),
    Error(ErrorEvent),
}

pub trait TelemetrySink {
    fn emit(&mut self, event: TelemetryEvent);
}

#[derive(Debug, Default)]
pub struct NoopTelemetrySink;

impl TelemetrySink for NoopTelemetrySink {
    fn emit(&mut self, _event: TelemetryEvent) {}
}

#[derive(Debug, Default)]
pub struct MemoryTelemetrySink {
    events: Vec<TelemetryEvent>,
}

impl MemoryTelemetrySink {
    pub fn events(&self) -> &[TelemetryEvent] {
        &self.events
    }
}

impl TelemetrySink for MemoryTelemetrySink {
    fn emit(&mut self, event: TelemetryEvent) {
        self.events.push(event);
    }
}

pub struct TelemetryRecorder<S> {
    context: Option<TelemetryContext>,
    sink: S,
}

impl<S> TelemetryRecorder<S>
where
    S: TelemetrySink,
{
    pub fn from_cli_args(raw_args: &[OsString], args: &CliArgs, sink: S) -> Self {
        let context = (!args.no_logging).then(|| {
            let anonymous_identity = anonymous_identity();
            let support_key = support_key_from_anonymous_identity(&anonymous_identity);

            TelemetryContext {
                support_key,
                anonymous_identity,
                command_line: redacted_command_line(raw_args),
            }
        });

        Self { context, sink }
    }

    pub fn record_feature_usage(&mut self, args: &CliArgs) {
        let Some(context) = &self.context else {
            return;
        };

        for feature_name in feature_usage_names(args) {
            self.sink
                .emit(TelemetryEvent::FeatureUsage(FeatureUsageEvent {
                    feature_name,
                    support_key: context.support_key.clone(),
                    anonymous_identity: context.anonymous_identity.clone(),
                }));
        }
    }

    pub fn record_error(&mut self, args: &CliArgs, error_type: &str, message: &str) {
        let Some(context) = &self.context else {
            return;
        };

        let settings = redacted_settings(args);
        let settings_json = Value::Object(settings.clone()).to_string();

        self.sink.emit(TelemetryEvent::Error(ErrorEvent {
            error_type: error_type.to_string(),
            message: message.to_string(),
            support_key: context.support_key.clone(),
            anonymous_identity: context.anonymous_identity.clone(),
            command_line: context.command_line.clone(),
            settings_json,
            settings,
        }));
    }

    pub fn into_sink(self) -> S {
        self.sink
    }
}

fn feature_usage_names(args: &CliArgs) -> Vec<String> {
    let mut features = Vec::new();

    if args.skip_validation {
        features.push("skip-validation".to_string());
    }

    if args.authorization_header.is_some() {
        features.push("authorization-header".to_string());
    }

    if args.authorization_header_from_environment_variable {
        features.push("load-authorization-header-from-environment".to_string());
    }

    features.push("authorization-header-variable-name".to_string());
    features.push("content-type".to_string());

    if args.base_url.is_some() {
        features.push("base-url".to_string());
    }

    features.push("output-type".to_string());

    if args.azure_scope.is_some() {
        features.push("azure-scope".to_string());
    }

    if args.azure_tenant_id.is_some() {
        features.push("azure-tenant-id".to_string());
    }

    features.push("timeout".to_string());

    if args.generate_intellij_tests {
        features.push("generate-intellij-tests".to_string());
    }

    if !args.custom_headers.is_empty() {
        features.push("custom-header".to_string());
    }

    if args.skip_headers {
        features.push("skip-headers".to_string());
    }

    features
}

fn redacted_command_line(raw_args: &[OsString]) -> String {
    let mut arguments = raw_args
        .iter()
        .map(|value| value.to_string_lossy().into_owned())
        .collect::<Vec<_>>();

    if let Some(program_name) = arguments.first_mut() {
        *program_name = "httpgenerator".to_string();
    }

    redact_authorization_headers(&arguments.join(" "))
}

fn redacted_settings(args: &CliArgs) -> Map<String, Value> {
    let mut settings = Map::new();

    settings.insert(
        "openApiPath".to_string(),
        option_string_value(args.open_api_path.as_deref()),
    );
    settings.insert(
        "outputFolder".to_string(),
        Value::String(args.output_folder.clone()),
    );
    settings.insert("noLogging".to_string(), Value::Bool(args.no_logging));
    settings.insert(
        "skipValidation".to_string(),
        Value::Bool(args.skip_validation),
    );
    settings.insert(
        "authorizationHeader".to_string(),
        redacted_authorization_value(args.authorization_header.as_deref()),
    );
    settings.insert(
        "authorizationHeaderFromEnvironmentVariable".to_string(),
        Value::Bool(args.authorization_header_from_environment_variable),
    );
    settings.insert(
        "authorizationHeaderVariableName".to_string(),
        Value::String(args.authorization_header_variable_name.clone()),
    );
    settings.insert(
        "contentType".to_string(),
        Value::String(args.content_type.clone()),
    );
    settings.insert(
        "baseUrl".to_string(),
        option_string_value(args.base_url.as_deref()),
    );
    settings.insert(
        "outputType".to_string(),
        Value::from(output_type_ordinal(args.output_type)),
    );
    settings.insert(
        "azureScope".to_string(),
        option_string_value(args.azure_scope.as_deref()),
    );
    settings.insert(
        "azureTenantId".to_string(),
        option_string_value(args.azure_tenant_id.as_deref()),
    );
    settings.insert("timeout".to_string(), Value::from(args.timeout));
    settings.insert(
        "generateIntellijTests".to_string(),
        Value::Bool(args.generate_intellij_tests),
    );
    settings.insert(
        "customHeaders".to_string(),
        Value::Array(
            args.custom_headers
                .iter()
                .map(|value| Value::String(redact_custom_header(value)))
                .collect(),
        ),
    );
    settings.insert("skipHeaders".to_string(), Value::Bool(args.skip_headers));

    settings
}

fn option_string_value(value: Option<&str>) -> Value {
    value
        .map(|value| Value::String(value.to_string()))
        .unwrap_or(Value::Null)
}

fn redacted_authorization_value(value: Option<&str>) -> Value {
    value
        .map(|_| Value::String(REDACTED.to_string()))
        .unwrap_or(Value::Null)
}

fn output_type_ordinal(output_type: OutputTypeArg) -> u8 {
    match output_type {
        OutputTypeArg::OneRequestPerFile => 0,
        OutputTypeArg::OneFile => 1,
        OutputTypeArg::OneFilePerTag => 2,
    }
}

fn redact_custom_header(value: &str) -> String {
    let Some((name, _)) = value.split_once(':') else {
        return value.to_string();
    };

    if name.trim().eq_ignore_ascii_case("authorization") {
        format!("{}: {REDACTED}", name.trim())
    } else {
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        MemoryTelemetrySink, TelemetryEvent, TelemetryRecorder, feature_usage_names,
        redacted_command_line,
    };
    use crate::args::{CliArgs, OutputTypeArg};
    use std::ffi::OsString;

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
        let mut recorder =
            TelemetryRecorder::from_cli_args(&raw_args, &args, MemoryTelemetrySink::default());

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
            MemoryTelemetrySink::default(),
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
            MemoryTelemetrySink::default(),
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
}
