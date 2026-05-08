use std::path::PathBuf;

use crate::openapi::{OpenApiSource, OpenApiSpecificationVersion, decode_raw_document};

use super::{OpenApiStats, inspect_raw_document};

#[test]
fn inspects_petstore_v30_with_dotnet_parity_counts() {
    let raw = decode_raw_document(
        OpenApiSource::Path(PathBuf::from("test/OpenAPI/v3.0/petstore.json")),
        include_str!("../../../../../../test/OpenAPI/v3.0/petstore.json"),
    )
    .unwrap();

    let inspection = inspect_raw_document(&raw).unwrap();

    assert_eq!(
        inspection.specification_version,
        OpenApiSpecificationVersion::OpenApi30
    );
    assert_eq!(inspection.stats.path_item_count, 13);
    assert_eq!(inspection.stats.operation_count, 19);
    assert_eq!(inspection.stats.parameter_count, 17);
    assert_eq!(inspection.stats.request_body_count, 9);
    assert_eq!(inspection.stats.response_count, 19);
    assert_eq!(inspection.stats.link_count, 0);
    assert_eq!(inspection.stats.callback_count, 0);
    assert_eq!(inspection.stats.schema_count, 73);
}

#[test]
fn inspects_petstore_v20_with_body_parameters_as_request_bodies() {
    let raw = decode_raw_document(
        OpenApiSource::Path(PathBuf::from("test/OpenAPI/v2.0/petstore.json")),
        include_str!("../../../../../../test/OpenAPI/v2.0/petstore.json"),
    )
    .unwrap();

    let inspection = inspect_raw_document(&raw).unwrap();

    assert_eq!(
        inspection.specification_version,
        OpenApiSpecificationVersion::Swagger2
    );
    assert!(inspection.stats.request_body_count > 0);
    assert!(inspection.stats.schema_count > 0);
}

#[test]
fn inspects_callback_examples_with_callbacks() {
    let raw = decode_raw_document(
        OpenApiSource::Path(PathBuf::from("test/OpenAPI/v3.0/callback-example.json")),
        include_str!("../../../../../../test/OpenAPI/v3.0/callback-example.json"),
    )
    .unwrap();

    let inspection = inspect_raw_document(&raw).unwrap();

    assert!(inspection.stats.callback_count > 0);
}

#[test]
fn empty_stats_start_at_zero() {
    let stats = OpenApiStats::default();

    assert_eq!(stats.path_item_count, 0);
    assert_eq!(stats.operation_count, 0);
    assert_eq!(stats.parameter_count, 0);
    assert_eq!(stats.request_body_count, 0);
    assert_eq!(stats.response_count, 0);
    assert_eq!(stats.link_count, 0);
    assert_eq!(stats.callback_count, 0);
    assert_eq!(stats.schema_count, 0);
}
