use std::sync::Mutex;

use exceptionless::ExceptionlessClient;
use tokio::runtime::Handle;

use super::{TelemetryEvent, TelemetrySink};

#[derive(Debug)]
pub struct ExceptionlessTelemetrySink {
    client: ExceptionlessClient,
    events: Mutex<Vec<TelemetryEvent>>,
}

const EXCEPTIONLESS_API_KEY: &str = "7VSRHLYiJdF7Xp0WaVwmEbJxVmrjqHnTIZNKkrkI";

impl ExceptionlessTelemetrySink {
    pub fn new() -> Self {
        Self {
            client: ExceptionlessClient::with_api_key(EXCEPTIONLESS_API_KEY),
            events: Mutex::new(Vec::new()),
        }
    }

    pub async fn flush(&self) {
        let events = {
            let mut guard = self.events.lock().unwrap();
            std::mem::take(&mut *guard)
        };

        if events.is_empty() {
            return;
        }

        let handle = match Handle::try_current() {
            Ok(h) => h,
            Err(_) => return,
        };

        let client = self.client.clone();
        let result = handle.block_on(async {
            let mut failed = 0;

            for event in events {
                match event {
                    TelemetryEvent::FeatureUsage(event) => {
                        let mut builder = client
                            .feature(&event.feature_name)
                            .user_identity(&event.anonymous_identity);

                        builder = builder.data("supportKey", event.support_key.as_str());

                        if builder.send().await.is_err() {
                            failed += 1;
                        }
                    }
                    TelemetryEvent::Error(event) => {
                        let error = ExceptionlessErrorOwned(
                            event.error_type.clone(),
                            event.message.clone(),
                        );

                        let builder = client
                            .error(&error)
                            .tag("error")
                            .source("httpgenerator")
                            .user_identity(&event.anonymous_identity)
                            .data("supportKey", event.support_key.as_str())
                            .data("commandLine", event.command_line.as_str())
                            .data("settings", serde_json::Value::String(event.settings_json));

                        if builder.send().await.is_err() {
                            failed += 1;
                        }
                    }
                }
            }

            failed
        });

        if result > 0 {
            eprintln!("Warning: {result} telemetry events failed to submit");
        }
    }

    pub fn take_events(&self) -> Vec<TelemetryEvent> {
        let mut guard = self.events.lock().unwrap();
        std::mem::take(&mut *guard)
    }
}

impl TelemetrySink for ExceptionlessTelemetrySink {
    fn emit(&mut self, event: TelemetryEvent) {
        if let Ok(mut guard) = self.events.lock() {
            guard.push(event);
        }
    }
}

#[derive(Debug)]
struct ExceptionlessErrorOwned(String, String);

impl std::fmt::Display for ExceptionlessErrorOwned {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.0, self.1)
    }
}

impl std::error::Error for ExceptionlessErrorOwned {}
