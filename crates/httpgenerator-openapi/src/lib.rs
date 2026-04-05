#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenApiPortStatus {
    Planned,
}

pub fn current_status() -> OpenApiPortStatus {
    OpenApiPortStatus::Planned
}
