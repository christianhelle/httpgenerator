use serde_json::{Map, Value};

use crate::{
    OpenApiInspectionError, OpenApiSpecificationVersion, RawOpenApiDocument, load_raw_document,
};

const SUPPORTED_METHODS: &[&str] = &[
    "get", "put", "post", "delete", "options", "head", "patch", "trace",
];

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

pub fn inspect_document(input: &str) -> Result<OpenApiInspection, OpenApiInspectionError> {
    let raw = load_raw_document(input).map_err(OpenApiInspectionError::Load)?;
    inspect_raw_document(&raw)
}

pub fn inspect_raw_document(
    document: &RawOpenApiDocument,
) -> Result<OpenApiInspection, OpenApiInspectionError> {
    let specification_version = document
        .specification_version()
        .map_err(OpenApiInspectionError::VersionDetection)?;

    Ok(OpenApiInspection {
        specification_version,
        stats: collect_stats(document.value(), specification_version),
    })
}

fn collect_stats(root: &Value, specification_version: OpenApiSpecificationVersion) -> OpenApiStats {
    let mut stats = OpenApiStats::default();

    if let Some(paths) = root.get("paths").and_then(Value::as_object) {
        stats.path_item_count = paths.len();

        for path_item in paths.values() {
            let Some(path_item) = path_item.as_object() else {
                continue;
            };

            if let Some(parameters) = path_item.get("parameters").and_then(Value::as_array) {
                stats.parameter_count += parameters.len();
                stats.request_body_count += count_swagger2_body_parameters(parameters);
                stats.schema_count += count_parameter_schemas(parameters);
            }

            for method in SUPPORTED_METHODS {
                let Some(operation) = path_item.get(*method).and_then(Value::as_object) else {
                    continue;
                };

                stats.operation_count += 1;

                if let Some(parameters) = operation.get("parameters").and_then(Value::as_array) {
                    stats.parameter_count += parameters.len();
                    stats.request_body_count += count_swagger2_body_parameters(parameters);
                    stats.schema_count += count_parameter_schemas(parameters);
                }

                if let Some(request_body) = operation.get("requestBody") {
                    stats.request_body_count += 1;
                    stats.schema_count += count_request_body_schemas(request_body);
                }

                if let Some(responses) = operation.get("responses").and_then(Value::as_object) {
                    stats.response_count += 1;
                    stats.schema_count += count_response_schemas(responses);
                    stats.link_count += count_response_links(responses);
                }

                if let Some(callbacks) = operation.get("callbacks").and_then(Value::as_object) {
                    stats.callback_count += callbacks.len();
                }
            }
        }
    }

    stats.schema_count += count_component_schemas(root, specification_version);
    stats
}

fn count_component_schemas(
    root: &Value,
    specification_version: OpenApiSpecificationVersion,
) -> usize {
    match specification_version {
        OpenApiSpecificationVersion::Swagger2 => root
            .get("definitions")
            .and_then(Value::as_object)
            .map(|definitions| {
                definitions
                    .values()
                    .map(count_schema_objects)
                    .sum::<usize>()
            })
            .unwrap_or_default(),
        OpenApiSpecificationVersion::OpenApi30 | OpenApiSpecificationVersion::OpenApi31 => root
            .get("components")
            .and_then(|components| components.get("schemas"))
            .and_then(Value::as_object)
            .map(|schemas| schemas.values().map(count_schema_objects).sum::<usize>())
            .unwrap_or_default(),
    }
}

fn count_swagger2_body_parameters(parameters: &[Value]) -> usize {
    parameters
        .iter()
        .filter(|parameter| {
            parameter
                .get("in")
                .and_then(Value::as_str)
                .is_some_and(|location| location == "body")
        })
        .count()
}

fn count_parameter_schemas(parameters: &[Value]) -> usize {
    parameters
        .iter()
        .map(|parameter| {
            if let Some(schema) = parameter.get("schema") {
                return count_schema_objects(schema);
            }

            parameter
                .as_object()
                .and_then(synthesized_parameter_schema)
                .map(|schema| count_schema_objects(&schema))
                .unwrap_or_default()
        })
        .sum::<usize>()
}

fn count_request_body_schemas(request_body: &Value) -> usize {
    request_body
        .get("content")
        .and_then(Value::as_object)
        .map(|content| {
            content
                .values()
                .filter_map(|media_type| media_type.get("schema"))
                .map(count_schema_objects)
                .sum::<usize>()
        })
        .unwrap_or_default()
}

fn count_response_schemas(responses: &Map<String, Value>) -> usize {
    responses
        .values()
        .map(|response| {
            response
                .get("schema")
                .map(count_schema_objects)
                .unwrap_or_default()
                + response
                    .get("content")
                    .and_then(Value::as_object)
                    .map(|content| {
                        content
                            .values()
                            .filter_map(|media_type| media_type.get("schema"))
                            .map(count_schema_objects)
                            .sum::<usize>()
                    })
                    .unwrap_or_default()
        })
        .sum::<usize>()
}

fn count_response_links(responses: &Map<String, Value>) -> usize {
    responses
        .values()
        .map(|response| {
            response
                .get("links")
                .and_then(Value::as_object)
                .map(|links| links.len())
                .unwrap_or_default()
        })
        .sum::<usize>()
}

fn synthesized_parameter_schema(parameter: &Map<String, Value>) -> Option<Value> {
    let mut schema = Map::new();

    for field_name in ["type", "items", "allOf", "oneOf", "anyOf", "properties"] {
        if let Some(value) = parameter.get(field_name) {
            schema.insert(field_name.to_string(), value.clone());
        }
    }

    if schema.is_empty() {
        None
    } else {
        Some(Value::Object(schema))
    }
}

fn count_schema_objects(value: &Value) -> usize {
    let Some(schema) = value.as_object() else {
        return 0;
    };

    let mut count = usize::from(is_schema_object(schema));

    if let Some(properties) = schema.get("properties").and_then(Value::as_object) {
        count += properties.values().map(count_schema_objects).sum::<usize>();
    }

    if let Some(items) = schema.get("items") {
        count += count_schema_objects(items);
    }

    if let Some(additional_properties) = schema.get("additionalProperties") {
        count += count_schema_objects(additional_properties);
    }

    for field_name in ["allOf", "oneOf", "anyOf"] {
        if let Some(values) = schema.get(field_name).and_then(Value::as_array) {
            count += values.iter().map(count_schema_objects).sum::<usize>();
        }
    }

    count
}

fn is_schema_object(schema: &Map<String, Value>) -> bool {
    schema.contains_key("$ref")
        || schema.contains_key("type")
        || schema.contains_key("properties")
        || schema.contains_key("items")
        || schema.contains_key("allOf")
        || schema.contains_key("oneOf")
        || schema.contains_key("anyOf")
        || schema.contains_key("additionalProperties")
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{OpenApiSource, OpenApiSpecificationVersion, decode_raw_document};

    use super::{OpenApiStats, inspect_raw_document};

    #[test]
    fn inspects_petstore_v30_with_non_zero_counts() {
        let raw = decode_raw_document(
            OpenApiSource::Path(PathBuf::from("test/OpenAPI/v3.0/petstore.json")),
            include_str!("../../../test/OpenAPI/v3.0/petstore.json"),
        )
        .unwrap();

        let inspection = inspect_raw_document(&raw).unwrap();

        assert_eq!(
            inspection.specification_version,
            OpenApiSpecificationVersion::OpenApi30
        );
        assert!(inspection.stats.path_item_count > 0);
        assert!(inspection.stats.operation_count > 0);
        assert!(inspection.stats.parameter_count > 0);
        assert!(inspection.stats.request_body_count > 0);
        assert!(inspection.stats.response_count > 0);
        assert!(inspection.stats.schema_count > 0);
    }

    #[test]
    fn inspects_petstore_v20_with_body_parameters_as_request_bodies() {
        let raw = decode_raw_document(
            OpenApiSource::Path(PathBuf::from("test/OpenAPI/v2.0/petstore.json")),
            include_str!("../../../test/OpenAPI/v2.0/petstore.json"),
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
            include_str!("../../../test/OpenAPI/v3.0/callback-example.json"),
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
}
