use serde_json::{Map, Value};

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
