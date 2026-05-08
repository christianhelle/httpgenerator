use serde_json::{Map, Value};

use super::model::OpenApiStats;
use super::schema::{
    count_parameter_schema_for_object, count_request_body_schemas,
    count_response_header_schemas_for_object, count_response_links_for_object,
    count_response_value_schemas, is_reference_object,
};

const SUPPORTED_METHODS: &[&str] = &[
    "get", "put", "post", "delete", "options", "head", "patch", "trace",
];

pub(super) fn collect_path_stats(root: &Value) -> OpenApiStats {
    let mut stats = OpenApiStats::default();

    if let Some(paths) = root.get("paths").and_then(Value::as_object) {
        stats.path_item_count = paths.len();

        for path_item in paths.values() {
            let Some(path_item) = path_item.as_object() else {
                continue;
            };

            if let Some(parameters) = path_item.get("parameters").and_then(Value::as_array) {
                stats.parameter_count += count_parameter_entries(parameters);
                stats.request_body_count += count_swagger2_body_parameters(parameters);
                stats.schema_count += count_parameter_schemas(parameters);
            }

            for method in SUPPORTED_METHODS {
                let Some(operation) = path_item.get(*method).and_then(Value::as_object) else {
                    continue;
                };

                stats.operation_count += 1;

                if let Some(parameters) = operation.get("parameters").and_then(Value::as_array) {
                    stats.parameter_count += count_parameter_entries(parameters);
                    stats.request_body_count += count_swagger2_body_parameters(parameters);
                    stats.schema_count += count_parameter_schemas(parameters);
                }

                if let Some(request_body) = operation.get("requestBody") {
                    stats.request_body_count += count_request_body_entries(request_body);
                    stats.schema_count += count_request_body_schemas(request_body);
                }

                if let Some(responses) = operation.get("responses").and_then(Value::as_object) {
                    stats.response_count += 1;
                    stats.schema_count += count_response_schemas(responses);
                    stats.schema_count += count_response_header_schemas(responses);
                    stats.link_count += count_response_links(responses);
                }

                if let Some(callbacks) = operation.get("callbacks").and_then(Value::as_object) {
                    stats.callback_count += count_callback_entries(callbacks);
                }
            }
        }
    }

    stats
}

pub(super) fn count_callback_entries(callbacks: &Map<String, Value>) -> usize {
    callbacks
        .values()
        .filter(|callback| {
            callback
                .as_object()
                .is_some_and(|callback| !is_reference_object(callback))
        })
        .count()
}

fn count_parameter_entries(parameters: &[Value]) -> usize {
    parameters
        .iter()
        .filter(|parameter| {
            parameter
                .as_object()
                .is_some_and(|parameter| !is_reference_object(parameter))
        })
        .count()
}

fn count_request_body_entries(request_body: &Value) -> usize {
    request_body
        .as_object()
        .filter(|request_body| !is_reference_object(request_body))
        .map(|_| 1)
        .unwrap_or_default()
}

fn count_swagger2_body_parameters(parameters: &[Value]) -> usize {
    parameters
        .iter()
        .filter(|parameter| {
            parameter
                .as_object()
                .is_some_and(|parameter| !is_reference_object(parameter))
                && parameter
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
            parameter
                .as_object()
                .map(count_parameter_schema_for_object)
                .unwrap_or_default()
        })
        .sum::<usize>()
}

fn count_response_schemas(responses: &Map<String, Value>) -> usize {
    responses
        .values()
        .map(count_response_value_schemas)
        .sum::<usize>()
}

fn count_response_header_schemas(responses: &Map<String, Value>) -> usize {
    responses
        .values()
        .filter_map(Value::as_object)
        .map(count_response_header_schemas_for_object)
        .sum::<usize>()
}

fn count_response_links(responses: &Map<String, Value>) -> usize {
    responses
        .values()
        .filter_map(Value::as_object)
        .map(count_response_links_for_object)
        .sum::<usize>()
}
