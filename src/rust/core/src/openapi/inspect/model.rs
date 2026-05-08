use super::super::OpenApiSpecificationVersion;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OpenApiStats {
    pub parameter_count: usize,
    pub schema_count: usize,
    pub path_item_count: usize,
    pub request_body_count: usize,
    pub response_count: usize,
    pub operation_count: usize,
    pub link_count: usize,
    pub callback_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenApiInspection {
    pub specification_version: OpenApiSpecificationVersion,
    pub stats: OpenApiStats,
}
