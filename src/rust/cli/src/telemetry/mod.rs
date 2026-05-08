mod events;
mod recorder;
mod redaction;
mod sink;

pub use events::{ErrorEvent, FeatureUsageEvent, TelemetryContext, TelemetryEvent};
pub use recorder::TelemetryRecorder;
pub use sink::{MemoryTelemetrySink, NoopTelemetrySink, TelemetrySink};

#[cfg(test)]
mod tests;
