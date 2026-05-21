use super::super::OpenApiSpecificationVersion;

/// Counts collected from the inspected OpenAPI document tree.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OpenApiStats {
    /// Number of reusable or inline parameters discovered.
    pub parameter_count: usize,
    /// Number of schemas found across component and inline locations.
    pub schema_count: usize,
    /// Number of top-level path items.
    pub path_item_count: usize,
    /// Number of request bodies.
    pub request_body_count: usize,
    /// Number of responses.
    pub response_count: usize,
    /// Number of operations across all path items.
    pub operation_count: usize,
    /// Number of links.
    pub link_count: usize,
    /// Number of callbacks.
    pub callback_count: usize,
}

/// Inspection summary returned by [`super::inspect_document`] and [`super::inspect_raw_document`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenApiInspection {
    /// Detected top-level specification version.
    pub specification_version: OpenApiSpecificationVersion,
    /// Aggregate counts gathered from the raw document tree.
    pub stats: OpenApiStats,
}
