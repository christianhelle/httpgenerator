use super::TelemetryEvent;

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
