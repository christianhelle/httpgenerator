use httpgenerator_core::{anonymous_identity, support_key_from_anonymous_identity};
use std::ffi::OsString;

use crate::args::CliArgs;

use super::{
    ErrorEvent, FeatureUsageEvent, TelemetryContext, TelemetryEvent,
    redaction::{feature_usage_names, redacted_command_line, redacted_settings},
    sink::TelemetrySink,
};

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
        let settings_json = serde_json::Value::Object(settings.clone()).to_string();

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
