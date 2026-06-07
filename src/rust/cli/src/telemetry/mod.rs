mod events;
mod exceptionless;
mod recorder;
mod redaction;
mod sink;

pub use events::{ErrorEvent, FeatureUsageEvent, TelemetryContext, TelemetryEvent};
pub use exceptionless::ExceptionlessTelemetrySink;
pub use recorder::TelemetryRecorder;
pub use sink::{MemoryTelemetrySink, NoopTelemetrySink, TelemetrySink};

#[derive(Debug)]
pub enum TelemetrySinkCollection {
    Exceptionless(ExceptionlessTelemetrySink),
    Memory(MemoryTelemetrySink),
    Noop,
}

impl From<ExceptionlessTelemetrySink> for TelemetrySinkCollection {
    fn from(sink: ExceptionlessTelemetrySink) -> Self {
        TelemetrySinkCollection::Exceptionless(sink)
    }
}

impl TelemetrySinkCollection {
    pub fn emit(&mut self, event: TelemetryEvent) {
        match self {
            TelemetrySinkCollection::Exceptionless(sink) => sink.emit(event),
            TelemetrySinkCollection::Memory(sink) => sink.emit(event),
            TelemetrySinkCollection::Noop => {}
        }
    }

    pub async fn flush(&self) {
        if let TelemetrySinkCollection::Exceptionless(sink) = self {
            sink.flush().await;
        }
    }

    pub fn events(&self) -> &[TelemetryEvent] {
        match self {
            TelemetrySinkCollection::Memory(sink) => sink.events(),
            _ => &[],
        }
    }
}

#[cfg(test)]
mod tests;
