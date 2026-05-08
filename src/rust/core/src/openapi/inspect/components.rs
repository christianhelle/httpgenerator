use serde_json::{Map, Value};

use super::super::OpenApiSpecificationVersion;
use super::model::OpenApiStats;
use super::paths::count_callback_entries;
use super::schema::{
    count_header_schema_for_object, count_parameter_schema_for_object, count_request_body_schemas,
    count_response_links_for_object, count_response_value_schemas, count_schema_objects,
    is_reference_object,
};

pub(super) fn collect_component_stats(
    root: &Value,
    specification_version: OpenApiSpecificationVersion,
    stats: &mut OpenApiStats,
) {
    stats.parameter_count += count_component_parameters(root, specification_version);
    stats.schema_count += count_component_schemas(root, specification_version);
    stats.schema_count += count_component_parameter_schemas(root, specification_version);
    stats.request_body_count += count_component_request_bodies(root, specification_version);
    stats.schema_count += count_component_request_body_schemas(root, specification_version);
    stats.schema_count += count_component_header_schemas(root, specification_version);
    stats.schema_count += count_component_response_schemas(root, specification_version);
    stats.link_count += count_component_links(root, specification_version);
    stats.link_count += count_component_response_links(root, specification_version);
    stats.callback_count += count_component_callbacks(root, specification_version);
}

fn component_entries<'a>(
    root: &'a Value,
    specification_version: OpenApiSpecificationVersion,
    component_name: &str,
) -> Option<&'a Map<String, Value>> {
    match specification_version {
        OpenApiSpecificationVersion::Swagger2 => None,
        OpenApiSpecificationVersion::OpenApi30 | OpenApiSpecificationVersion::OpenApi31 => root
            .get("components")
            .and_then(|components| components.get(component_name))
            .and_then(Value::as_object),
    }
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

fn count_component_parameters(
    root: &Value,
    specification_version: OpenApiSpecificationVersion,
) -> usize {
    component_entries(root, specification_version, "parameters")
        .map(|parameters| {
            parameters
                .values()
                .filter(|parameter| {
                    parameter
                        .as_object()
                        .is_some_and(|parameter| !is_reference_object(parameter))
                })
                .count()
        })
        .unwrap_or_default()
}

fn count_component_parameter_schemas(
    root: &Value,
    specification_version: OpenApiSpecificationVersion,
) -> usize {
    component_entries(root, specification_version, "parameters")
        .map(|parameters| {
            parameters
                .values()
                .map(|parameter| {
                    parameter
                        .as_object()
                        .map(count_parameter_schema_for_object)
                        .unwrap_or_default()
                })
                .sum::<usize>()
        })
        .unwrap_or_default()
}

fn count_component_request_bodies(
    root: &Value,
    specification_version: OpenApiSpecificationVersion,
) -> usize {
    component_entries(root, specification_version, "requestBodies")
        .map(|request_bodies| {
            request_bodies
                .values()
                .filter(|request_body| {
                    request_body
                        .as_object()
                        .is_some_and(|request_body| !is_reference_object(request_body))
                })
                .count()
        })
        .unwrap_or_default()
}

fn count_component_request_body_schemas(
    root: &Value,
    specification_version: OpenApiSpecificationVersion,
) -> usize {
    component_entries(root, specification_version, "requestBodies")
        .map(|request_bodies| {
            request_bodies
                .values()
                .map(count_request_body_schemas)
                .sum::<usize>()
        })
        .unwrap_or_default()
}

fn count_component_header_schemas(
    root: &Value,
    specification_version: OpenApiSpecificationVersion,
) -> usize {
    component_entries(root, specification_version, "headers")
        .map(|headers| {
            headers
                .values()
                .map(|header| {
                    header
                        .as_object()
                        .map(count_header_schema_for_object)
                        .unwrap_or_default()
                })
                .sum::<usize>()
        })
        .unwrap_or_default()
}

fn count_component_response_schemas(
    root: &Value,
    specification_version: OpenApiSpecificationVersion,
) -> usize {
    component_entries(root, specification_version, "responses")
        .map(|responses| {
            responses
                .values()
                .filter_map(Value::as_object)
                .map(|response| {
                    let response_value = Value::Object(response.clone());
                    count_response_value_schemas(&response_value)
                        + super::schema::count_response_header_schemas_for_object(response)
                })
                .sum::<usize>()
        })
        .unwrap_or_default()
}

fn count_component_links(
    root: &Value,
    specification_version: OpenApiSpecificationVersion,
) -> usize {
    component_entries(root, specification_version, "links")
        .map(|links| {
            links
                .values()
                .filter(|link| {
                    link.as_object()
                        .is_some_and(|link| !is_reference_object(link))
                })
                .count()
        })
        .unwrap_or_default()
}

fn count_component_response_links(
    root: &Value,
    specification_version: OpenApiSpecificationVersion,
) -> usize {
    component_entries(root, specification_version, "responses")
        .map(|responses| {
            responses
                .values()
                .filter_map(Value::as_object)
                .map(count_response_links_for_object)
                .sum::<usize>()
        })
        .unwrap_or_default()
}

fn count_component_callbacks(
    root: &Value,
    specification_version: OpenApiSpecificationVersion,
) -> usize {
    component_entries(root, specification_version, "callbacks")
        .map(count_callback_entries)
        .unwrap_or_default()
}
