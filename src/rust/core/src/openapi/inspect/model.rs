use super::super::OpenApiSpecificationVersion;

/// Counts of notable structures in an OpenAPI document.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OpenApiStats {
    /// Number of parameters found in paths, operations, and components.
    pub parameter_count: usize,
    /// Number of schemas found in components and inline request/response shapes.
    pub schema_count: usize,
    /// Number of path items in the `paths` object.
    pub path_item_count: usize,
    /// Number of request body definitions found inline or in components.
    pub request_body_count: usize,
    /// Number of response definitions found inline or in components.
    pub response_count: usize,
    /// Number of operations under paths.
    pub operation_count: usize,
    /// Number of links found in responses or components.
    pub link_count: usize,
    /// Number of callbacks found in operations or components.
    pub callback_count: usize,
}

/// Lightweight inspection result for an OpenAPI document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenApiInspection {
    /// Detected specification family.
    pub specification_version: OpenApiSpecificationVersion,
    /// Collected document statistics.
    pub stats: OpenApiStats,
}
