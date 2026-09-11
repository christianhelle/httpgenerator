//! Exceptionless telemetry sink.
//!
//! Events are submitted directly to the Exceptionless v2 collector API. The payload shape is the
//! documented wire format, so no client SDK is needed beyond an HTTP client and JSON.

use std::{
    sync::Mutex,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use serde_json::{Map, Value, json};

use super::{TelemetryEvent, TelemetrySink};

const EXCEPTIONLESS_API_KEY: &str = "7VSRHLYiJdF7Xp0WaVwmEbJxVmrjqHnTIZNKkrkI";
const EXCEPTIONLESS_EVENTS_URL: &str = "https://collector.exceptionless.io/api/v2/events";
const SUBMIT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug, Default)]
pub struct ExceptionlessTelemetrySink {
    events: Mutex<Vec<TelemetryEvent>>,
}

impl ExceptionlessTelemetrySink {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn flush(&self) {
        let events = self.take_events();

        if events.is_empty() {
            return;
        }

        let failed = events.len();
        let payload = Value::Array(events.iter().map(payload_for).collect());

        if submit(&payload).is_err() {
            eprintln!("Warning: {failed} telemetry events failed to submit");
        }
    }

    pub fn take_events(&self) -> Vec<TelemetryEvent> {
        match self.events.lock() {
            Ok(mut guard) => std::mem::take(&mut *guard),
            Err(_) => Vec::new(),
        }
    }
}

impl TelemetrySink for ExceptionlessTelemetrySink {
    fn emit(&mut self, event: TelemetryEvent) {
        if let Ok(mut guard) = self.events.lock() {
            guard.push(event);
        }
    }
}

fn submit(payload: &Value) -> Result<(), ()> {
    let response = ureq::post(EXCEPTIONLESS_EVENTS_URL)
        .config()
        .timeout_global(Some(SUBMIT_TIMEOUT))
        .http_status_as_error(false)
        .build()
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {EXCEPTIONLESS_API_KEY}"))
        .send(payload.to_string())
        .map_err(|_| ())?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(())
    }
}

/// Builds the Exceptionless wire representation of a telemetry event.
fn payload_for(event: &TelemetryEvent) -> Value {
    let date = rfc3339_utc_now();

    match event {
        TelemetryEvent::FeatureUsage(event) => {
            let mut data = Map::new();
            data.insert("@user".to_owned(), json!(event.anonymous_identity));
            data.insert("supportKey".to_owned(), json!(event.support_key));

            json!({
                "type": "usage",
                "source": event.feature_name,
                "date": date,
                "data": Value::Object(data),
            })
        }
        TelemetryEvent::Error(event) => {
            let mut data = Map::new();
            data.insert(
                "@error".to_owned(),
                json!({
                    "message": format!("[{}] {}", event.error_type, event.message),
                    "type": event.error_type,
                }),
            );
            data.insert("@user".to_owned(), json!(event.anonymous_identity));
            data.insert("supportKey".to_owned(), json!(event.support_key));
            data.insert("commandLine".to_owned(), json!(event.command_line));
            data.insert("settings".to_owned(), json!(event.settings_json));

            json!({
                "type": "error",
                "source": "httpgenerator",
                "date": date,
                "tags": ["error"],
                "data": Value::Object(data),
            })
        }
    }
}

/// Formats the current time as an RFC 3339 UTC timestamp with millisecond precision.
fn rfc3339_utc_now() -> String {
    let since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();

    rfc3339_utc(since_epoch.as_secs(), since_epoch.subsec_millis())
}

fn rfc3339_utc(seconds_since_epoch: u64, milliseconds: u32) -> String {
    let days = (seconds_since_epoch / 86_400) as i64;
    let time_of_day = seconds_since_epoch % 86_400;
    let (year, month, day) = civil_from_days(days);
    let (hour, minute, second) = (
        time_of_day / 3_600,
        (time_of_day % 3_600) / 60,
        time_of_day % 60,
    );

    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}.{milliseconds:03}Z")
}

/// Converts a count of days since the Unix epoch into a proleptic Gregorian calendar date.
///
/// This is Howard Hinnant's `civil_from_days` algorithm.
fn civil_from_days(days_since_epoch: i64) -> (i64, u32, u32) {
    let shifted = days_since_epoch + 719_468;
    let era = if shifted >= 0 {
        shifted
    } else {
        shifted - 146_096
    } / 146_097;
    let day_of_era = (shifted - era * 146_097) as u64;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era as i64 + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_position = (5 * day_of_year + 2) / 153;
    let day = (day_of_year - (153 * month_position + 2) / 5 + 1) as u32;
    let month = if month_position < 10 {
        month_position + 3
    } else {
        month_position - 9
    } as u32;

    (if month <= 2 { year + 1 } else { year }, month, day)
}

#[cfg(test)]
mod tests {
    use super::{payload_for, rfc3339_utc};
    use crate::telemetry::{ErrorEvent, FeatureUsageEvent, TelemetryEvent};
    use serde_json::{Map, json};

    #[test]
    fn formats_timestamps_as_rfc3339_utc() {
        assert_eq!(rfc3339_utc(0, 0), "1970-01-01T00:00:00.000Z");
        assert_eq!(rfc3339_utc(1_700_000_000, 123), "2023-11-14T22:13:20.123Z");
        assert_eq!(rfc3339_utc(951_782_400, 0), "2000-02-29T00:00:00.000Z");
    }

    #[test]
    fn feature_usage_payload_uses_the_usage_event_shape() {
        let payload = payload_for(&TelemetryEvent::FeatureUsage(FeatureUsageEvent {
            feature_name: "generate".to_owned(),
            support_key: "prihjx2".to_owned(),
            anonymous_identity: "prihjx2hffzjfsy4vly5".to_owned(),
        }));

        assert_eq!(payload["type"], json!("usage"));
        assert_eq!(payload["source"], json!("generate"));
        assert_eq!(payload["data"]["@user"], json!("prihjx2hffzjfsy4vly5"));
        assert_eq!(payload["data"]["supportKey"], json!("prihjx2"));
        assert!(
            payload["date"]
                .as_str()
                .is_some_and(|date| date.ends_with('Z'))
        );
    }

    #[test]
    fn error_payload_carries_the_error_details_and_context() {
        let payload = payload_for(&TelemetryEvent::Error(ErrorEvent {
            error_type: "GenerationError".to_owned(),
            message: "boom".to_owned(),
            support_key: "prihjx2".to_owned(),
            anonymous_identity: "prihjx2hffzjfsy4vly5".to_owned(),
            command_line: "httpgenerator spec.json".to_owned(),
            settings_json: "{}".to_owned(),
            settings: Map::new(),
        }));

        assert_eq!(payload["type"], json!("error"));
        assert_eq!(payload["source"], json!("httpgenerator"));
        assert_eq!(payload["tags"], json!(["error"]));
        assert_eq!(
            payload["data"]["@error"]["message"],
            json!("[GenerationError] boom")
        );
        assert_eq!(payload["data"]["@error"]["type"], json!("GenerationError"));
        assert_eq!(
            payload["data"]["commandLine"],
            json!("httpgenerator spec.json")
        );
        assert_eq!(payload["data"]["settings"], json!("{}"));
    }
}
