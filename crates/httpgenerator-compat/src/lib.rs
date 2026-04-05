#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompatibilityHarnessStatus {
    Planned,
}

pub fn current_status() -> CompatibilityHarnessStatus {
    CompatibilityHarnessStatus::Planned
}
